//! Rust wrapper for Zephyr UART Interupt-driven driver.

use core::cell::UnsafeCell;

// Static info for a `Uart` instance.
// Each instance of `Uart` gets their own unique `UartStatic`
// u_Note: This whole area is modelled after what's currently in gpio.rs.
const RX_RINGBUFFER_SIZE: usize = 256;
const TX_RINGBUFFER_SIZE: usize = 256;
/// Staging buffer size for one `uart_fifo_fill()` call; covers the deepest FIFO we expect to
/// meet in one go (ns16750's 64), larger FIFOs just take an extra interrupt.
const TX_CHUNK_SIZE: usize = 64;
const UART_RX_TIMEOUT: i32 = 1000;
pub(crate) struct UartStatic {
    // RX Stuff
    rx_ringbuffer: UnsafeCell<heapless::spsc::Queue<u8, RX_RINGBUFFER_SIZE>>,
    rx_waker: embassy_sync::waitqueue::AtomicWaker,

    // TX Stuff
    tx_ringbuffer: UnsafeCell<heapless::spsc::Queue<u8, TX_RINGBUFFER_SIZE>>,
    tx_waker: embassy_sync::waitqueue::AtomicWaker,
}
unsafe impl Sync for UartStatic {}

impl UartStatic {
    pub(crate) const fn new() -> Self {
        Self {
            rx_ringbuffer: UnsafeCell::new(heapless::spsc::Queue::new()),
            rx_waker: embassy_sync::waitqueue::AtomicWaker::new(),

            tx_ringbuffer: UnsafeCell::new(heapless::spsc::Queue::new()),
            tx_waker: embassy_sync::waitqueue::AtomicWaker::new(),
        }
    }
}

// Uart callback. This function signiature has been taken from the uart_callback_set() docs.
unsafe extern "C" fn uart_callback(
    device: *const crate::raw::device,
    user_data: *mut core::ffi::c_void,
) {
    use crate::error::{to_result, Error};

    let state = &*(user_data as *const UartStatic);
    if handle_interrupt(device, state).is_err() {
        return;
    }

    fn handle_interrupt(device: *const crate::raw::device, state: &UartStatic) -> Result<(), ()> {
        /// Helper to check if an IRQ is pending
        fn is_irq_pending(device: *const crate::raw::device) -> Result<bool, ()> {
            match to_result(
                // SAFETY: `device` is a valid UART device pointer for the duration of this call.
                unsafe { crate::raw::uart_irq_is_pending(device) },
            ) {
                Ok(1) => Ok(true),  // An IRQ is pending.
                Ok(0) => Ok(false), // In IRQ is not pending.
                Ok(value) => {
                    log::error!("uart_irq_is_pending() succeeded, but returned a non-boolean value: {}. This should not be possible, and indicates that Zephyr's interrupt-driven UART API may have changed? So, treating this as an error.", value);
                    Err(())
                }
                Err(Error(crate::raw::ENOSYS)) => {
                    log::error!("uart_irq_is_pending() failed with ENOSYS: This function is not implemented.");
                    Err(())
                }
                Err(Error(crate::raw::ENOTSUP)) => {
                    log::error!(
                        "uart_irq_is_pending() failed with ENOTSUP: This API is not enabled."
                    );
                    Err(())
                }
                Err(e) => {
                    log::error!("uart_irq_is_pending() failed with Zephyr errno {}.", e);
                    Err(())
                }
            }
        }

        /// Helper to check if RX is ready
        fn is_rx_ready(device: *const crate::raw::device) -> Result<bool, ()> {
            match to_result(
                // SAFETY: `device` is a valid UART device pointer for the duration of this call.
                unsafe { crate::raw::uart_irq_rx_ready(device) },
            ) {
                Ok(1) => Ok(true),  // A received char is ready.
                Ok(0) => Ok(false), // A received char is not ready.
                Ok(value) => {
                    log::error!("uart_irq_rx_ready() succeeded, but returned a non-boolean value: {}. This should not be possible, and indicates that Zephyr's interrupt-driven UART API may have changed? So, treating this as an error.", value);
                    Err(())
                }
                Err(Error(crate::raw::ENOSYS)) => {
                    log::error!(
                        "uart_irq_rx_ready() failed with ENOSYS: This function is not implemented."
                    );
                    Err(())
                }
                Err(Error(crate::raw::ENOTSUP)) => {
                    log::error!(
                        "uart_irq_rx_ready() failed with ENOTSUP: This API is not enabled."
                    );
                    Err(())
                }
                Err(e) => {
                    log::error!("uart_irq_rx_ready() failed with Zephyr errno {}.", e);
                    Err(())
                }
            }
        }

        /// Helper to read from the FIFO. Reads back a single character (returned here as a Option<u8>).
        fn fifo_read(device: *const crate::raw::device) -> Result<Option<u8>, ()> {
            let mut character: u8 = 0;
            match to_result(
                // SAFETY:
                // `character` is large enough to hold one byte. If `uart_fifo_read()` wrutes nire than that
                // into `charater` (which it shouldn't per its contract), then we print out an error and exit the callback.
                unsafe { crate::raw::uart_fifo_read(device, &mut character, 1) },
            ) {
                Ok(1) => Ok(Some(character)),
                Ok(0) => Ok(None),
                Ok(_) => {
                    log::error!("uart_fifo_read() failed: `uart_fifo_read()` somehow returned more bytes than we requested. This is probably not good, and might have resulted in a memory overwrite.");
                    Err(())
                }
                Err(Error(crate::raw::ENOSYS)) => {
                    log::error!(
                        "uart_fifo_read() failed with -ENOSYS: This function is not implemented."
                    );
                    Err(())
                }
                Err(Error(crate::raw::ENOTSUP)) => {
                    log::error!("uart_fifo_read() failed with -ENOTSUP: This API is not enabled.");
                    Err(())
                }
                Err(e) => {
                    log::error!("uart_fifo_read() failed with Zephyr errno {}.", e);
                    Err(())
                }
            }
        }

        /// Helper to check if TX is ready. Returns `None` if the device isn't ready to write a new byte. Otherwise, returns a `usize` indicating the minimum number of bytes that can be written in a single call to uart_fifo_fill if the device is ready.
        fn is_tx_ready(device: *const crate::raw::device) -> Result<Option<usize>, ()> {
            match to_result(
                // SAFETY: `device` is a valid UART device pointer for the duration of this call.
                unsafe { crate::raw::uart_irq_tx_ready(device) },
            ) {
                Ok(0) => Ok(None),
                Ok(n) => Ok(Some(n as usize)),
                Err(Error(crate::raw::ENOSYS)) => {
                    log::error!("uart_irq_tx_ready() failed with -ENOSYS: This function is not implemented.");
                    Err(())
                }
                Err(Error(crate::raw::ENOTSUP)) => {
                    log::error!(
                        "uart_irq_tx_ready() failed with -ENOTSUP: This API is not enabled."
                    );
                    Err(())
                }
                Err(e) => {
                    log::error!("uart_irq_tx_ready() failed with Zephyr errno {}.", e);
                    Err(())
                }
            }
        }

        /// Helper to write bytes into the TX FIFO. Returns how many of them the driver accepted.
        ///
        /// Must be called at most ONCE per TX-ready event. `uart_fifo_fill()` is not obliged to
        /// flow-control itself -- ns16550 blindly writes `min(size, fifo_size)` bytes to THR
        /// without ever consulting LSR -- so calling it per byte in a loop silently overruns the
        /// hardware FIFO and corrupts bytes still queued for transmission.
        fn fifo_fill(device: *const crate::raw::device, bytes: &[u8]) -> Result<usize, ()> {
            match to_result(
                // SAFETY: `device` is a valid UART device pointer, and `bytes` is valid for
                // `bytes.len()` reads, for the duration of this call.
                unsafe {
                    crate::raw::uart_fifo_fill(
                        device,
                        bytes.as_ptr(),
                        bytes.len() as core::ffi::c_int,
                    )
                },
            ) {
                Ok(n) if n as usize <= bytes.len() => Ok(n as usize),
                Ok(n) => {
                    log::error!("uart_fifo_fill() reported {} bytes written but was only offered {}. That should not be possible, and may mean it read past the end of our buffer.", n, bytes.len());
                    Err(())
                }
                Err(Error(crate::raw::ENOSYS)) => {
                    log::error!(
                        "uart_fifo_fill() failed with -ENOSYS: This function is not implemented."
                    );
                    Err(())
                }
                Err(Error(crate::raw::ENOTSUP)) => {
                    log::error!("uart_fifo_fill() failed with -ENOTSUP: This API is not enabled.");
                    Err(())
                }
                Err(e) => {
                    log::error!("uart_fifo_fill() failed with Zephyr errno {}.", e);
                    Err(())
                }
            }
        }

        // Loop until there's no pending IRQs left to process
        loop {
            // Must be re-run every iteration: drivers whose IIR auto-acks on read (ns16550 and
            // friends) answer the is_*() queries below from a snapshot that only this call refreshes,
            // so hoisting it out of the loop makes them report stale state forever.
            // SAFETY: `device` is a valid UART device pointer for the duration of this call.
            unsafe {
                crate::raw::uart_irq_update(device);
            }

            // If no IRQ is pending we can exit out of the callback
            if !is_irq_pending(device)? {
                return Ok(());
            }

            // Handle RX
            while is_rx_ready(device)? {
                match fifo_read(device)? {
                    Some(byte) => {
                        // Enqueue the byte into the ringbuffer
                        // SAFETY: `state` is a valid UartState pointer for the duration of this call.
                        let ringbuffer = unsafe { &mut *state.rx_ringbuffer.get() };
                        if ringbuffer.enqueue(byte).is_err() {
                            log::warn!("Uart RX ringbuffer is full! Dropped a byte.");
                        }
                    }
                    None => break, // The FIFO is drained at this point so we're done and can break out of the while loop.
                }
            }
            state.rx_waker.wake();

            // Handle TX
            if is_tx_ready(device)?.is_some() {
                // The ringbuffer isn't contiguous, so stage a run of it to hand to the driver as a
                // single slice. Anything that doesn't fit rides the next TX interrupt.
                let mut chunk = [0u8; TX_CHUNK_SIZE];
                let mut chunk_len = 0usize;
                {
                    // SAFETY: `state` is a valid UartStatic pointer for the duration of this borrow.
                    let ringbuffer = unsafe { &*state.tx_ringbuffer.get() };
                    for (slot, &byte) in chunk.iter_mut().zip(ringbuffer.iter()) {
                        *slot = byte;
                        chunk_len += 1;
                    }
                }

                if chunk_len > 0 {
                    // One call only -- see `fifo_fill`. Drop exactly what the driver took.
                    let sent = fifo_fill(device, &chunk[..chunk_len])?;
                    // SAFETY: `state` is a valid UartStatic pointer for the duration of this call.
                    let ringbuffer = unsafe { &mut *state.tx_ringbuffer.get() };
                    for _ in 0..sent {
                        if ringbuffer.dequeue().is_none() {
                            log::error!("ringbuffer.dequeue() failed: Returned `None` while dropping bytes the FIFO had already accepted. This shouldn't be possible at this point?");
                            break;
                        }
                    }
                }

                // Once we've drained everything, stop the TX IRQ (or it will keep firing forever)
                // Important: we need to re-enable the TX IRQ in the write() function after we enqueue stuff to the ringbuffer
                // SAFETY: `state` is a valid UartStatic pointer for the duration of this borrow.
                if unsafe { &*state.tx_ringbuffer.get() }.is_empty() {
                    // SAFETY: `device` is a valid UART device pointer for the duration of this call.
                    unsafe {
                        crate::raw::uart_irq_tx_disable(device);
                    }
                }
                state.tx_waker.wake();
            }
        }
    }
}

/// A UART peripheral, using Zephyr's interrupt-driven UART API.
/// (This is a wrapper around the `struct device` in Zephyr that represents a UART controller. This driver utilizes Zephyr's interrupt-driven UART API.)
///
/// # Using This Struct From The Devicetree:
///
/// Unlike externally-wired devices (e.g. a sensor), a UART is usually
/// an on-chip peripheral that is already declared in the board's `.dts` file with the
/// vendor-specific compatible (`nxp,lpc-usart`, `nordic,nrf-uarte`, `raspberrypi,pico-uart`,
/// etc.). So, you normally don't add a new devicetree node yourself to use this driver. You
/// should just be able to reference the one your board already provides by its label (e.g. `uart0`, `flexcomm0`, `usart1`). If
/// you have multiple targets in your project, you might want to define a geneirc `app_uart` (or similar) label across all of your devicetree
/// files so you don't have to change you Rust code between targets.
///
/// For example, if my chip had a UART peripheral called `app_uart` in the devicetree, I'd retrieve an instance of it like:
/// ```rust
/// let mut uart = zephyr::devicetree::labels::app_uart::get_instance().unwrap();
/// ```
///
/// You'll also want to enable serial and the Zephyr UART interrupt-driven API in your prj.conf:
/// ```kconfig
/// CONFIG_SERIAL=y
/// CONFIG_UART_INTERRUPT_DRIVEN=y
/// ```
///
/// To prevent collisions with other UART APIs, it may also be a good idea to explicitly disable the async API:
/// ```kconfig
/// CONFIG_UART_ASYNC_API=n
/// ```
///
///
/// If needed, you can also disable Zephyr's UART logging if those messages would collide with your traffic:
/// ```kconfig
/// CONFIG_UART_CONSOLE=n
/// CONFIG_LOG_BACKEND_UART=n
/// ```
///
pub struct Uart {
    device: *const crate::raw::device, // The underlying device itself.
    pub(crate) data: &'static UartStatic, // Our associated data, used for callbacks.
}

impl Uart {
    /// Constructor, used by the devicetree generated code.
    pub(crate) unsafe fn new(
        unique: &crate::device::Unique,
        data: &'static UartStatic,
        device: *const crate::raw::device,
    ) -> Option<Uart> {
        // Make sure this instance doesn't already exist.
        if !unique.once() {
            return None;
        }

        // Register the UART callback via uart_irq_callback_user_data_set()
        if let Err(e) = crate::error::to_result_void(crate::raw::uart_irq_callback_user_data_set(
            device,
            Some(uart_callback),
            data as *const UartStatic as *mut core::ffi::c_void,
        )) {
            match e.0 {
                crate::raw::ENOSYS => log::error!("uart_irq_callback_user_data_set() returned -ENOSYS: not supported by the device."),
                crate::raw::ENOTSUP => log::error!("uart_irq_callback_user_data_set() returned -ENOTSUP: API not enabled."),
                _ => log::error!("uart_irq_callback_user_data_set() failed with Zephyr errno {}", e),
            }
            return None;
        }

        // Enable RX interrupts (not TX yet though)
        crate::raw::uart_irq_rx_enable(device);

        Some(Uart { device, data })
    }
}

impl embedded_io_async::ErrorType for Uart {
    type Error = embedded_io_async::ErrorKind;
}

impl embedded_io_async::Read for Uart {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        core::future::poll_fn(|cx| {
            // SAFETY: exclusive access on the consumer side, producer only touches it from the UART callback via enqueue().
            let ringbuffer = unsafe { &mut *self.data.rx_ringbuffer.get() };

            // Drain the ringbuffer.
            // Basically just continuously move bytes from our ringbuffer into the user's `buf` until
            // either our ringbuffer is completely empty or the user's `buf` is completely full.
            let mut n = 0;
            for byte in buf.iter_mut() {
                if let Some(b) = ringbuffer.dequeue() {
                    *byte = b;
                    n += 1;
                } else {
                    break;
                }
            }
            if n > 0 {
                return core::task::Poll::Ready(Ok(n));
            } // Return success, indicating how many bytes we drained. If we didn't drain any bytes (meaning the ringbuffer was empty), fall through to the below case.

            // If we get here, we our ringbuffer was empty so we didn't drain anything.
            // So, register the waker so that whenever there ARE bytes to drain, this task gets woken up to finish its job.
            self.data.rx_waker.register(cx.waker()); // We have to register this waker before the final check (i.e., before we know if we will return core::task::Poll::Pending or not) because the ISR might place a byte in the empty buffer WHILE we are doing the check.
            if let Some(byte) = ringbuffer.dequeue() {
                buf[0] = byte;
                core::task::Poll::Ready(Ok(1))
            } else {
                core::task::Poll::Pending
            }
        })
        .await
    }
}

impl embedded_io_async::Write for Uart {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }

        let n = core::future::poll_fn(|cx| {
            // SAFETY: exclusive access on the producer side
            let ringbuffer = unsafe { &mut *self.data.tx_ringbuffer.get() };

            // Enqueue as many bytes from `buf` as will fit.
            // Basically just continuously move bytes from the user's `buf` into our ringbuffer until either our ringbuffer is completely full or the user's `buf` has been completely consumed.
            let mut n = 0;
            for &byte in buf.iter() {
                if ringbuffer.enqueue(byte).is_err() {
                    break;
                }
                n += 1;
            }
            if n > 0 {
                return core::task::Poll::Ready(n);
            } // Return success, indicating how many bytes we enqueued. However, if we didn't enqueue any bytes (meaning the ringbuffer was full), fall through to the below case.

            // If we get here, our ringbuffer was full so we couldn't enqueue anything. So, register the waker so that whenever there IS space to enqueue, this task gets woken up to finish its job.
            self.data.tx_waker.register(cx.waker()); // We have to register this waker before the final check (i.e., before we know if we will return core::task::Poll::Pending or not) because the ISR might drain a byte from the full buffer WHILE we are doing the check.
            if ringbuffer.enqueue(buf[0]).is_ok() {
                core::task::Poll::Ready(1)
            } else {
                core::task::Poll::Pending
            }
        })
        .await;

        // Now that there are bytes in the TX ringbuffer, (re-)enable the TX IRQ so the callback drains them into the FIFO.
        // The callback disables the TX IRQ once the ringbuffer is empty, so we have to re-enable it here every time we enqueue.
        // SAFETY: `self.device` is a valid UART device pointer for the lifetime of `self`.
        unsafe {
            crate::raw::uart_irq_tx_enable(self.device);
        }

        Ok(n)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        core::future::poll_fn(|cx| {
            // SAFETY: exclusive consumer-side access to the ringbuffer's "is empty" view via &mut self.
            // The callback, who's the producer of "empty" as we understand it, only changes len() downward (never upward).
            let ringbuffer = unsafe { &*self.data.tx_ringbuffer.get() };

            // If nothing's left in the ringbuffer, then everything we buffered has been handed to the FIFO and we're flushed.
            if ringbuffer.is_empty() {
                return core::task::Poll::Ready(Ok(()));
            }

            // We need to re-check before returning. So, do the exact same thing we do above, but
            // register the waker first just in case a wake fires as we are actively doing the check.
            self.data.tx_waker.register(cx.waker());
            if ringbuffer.len() == 0 {
                core::task::Poll::Ready(Ok(()))
            } else {
                core::task::Poll::Pending
            }
        })
        .await
    }
}
