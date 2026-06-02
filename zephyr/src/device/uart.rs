use core::cell::UnsafeCell;

/// Number of stop bits.
/// (Note: This is just the Rust version of `uart_config_stop_bits` from `#include <zephyr/drivers/uart.h>`)
#[repr(u8)]
pub enum UartConfigStopBits {
    UART_CFG_STOP_BITS_0_5 = 0, // 0.5 stop bit
    UART_CFG_STOP_BITS_1,       // 1 stop bit
    UART_CFG_STOP_BITS_1_5,     // 1.5 stop bits
    UART_CFG_STOP_BITS_2,       // 2 stop bits
}

/// Parity modes.
/// (Note: This is just the Rust equivalent of `uart_config_parity` from `#include <zephyr/drivers/uart.h>`)
#[repr(u8)]
pub enum UartConfigParity {
    UART_CFG_PARITY_NONE = 0,  // No parity.
    UART_CFG_PARITY_ODD,       // Odd parity.
    UART_CFG_PARITY_EVEN,      // Even parity.
    UART_CFG_PARITY_MARK,      // Mark parity.
    UART_CFG_PARITY_SPACE,     // Space parity.
}

/// Number of data bits.
/// (This is just the Rust equivalent of `uart_config_data_bits` from `#include <zephyr/drivers/uart.h>`)
#[repr(u8)]
pub enum UartConfigDataBits {
    UART_CFG_DATA_BITS_5 = 0, // 5 data bits
    UART_CFG_DATA_BITS_6,     // 6 data bits
    UART_CFG_DATA_BITS_7,     // 7 data bits
    UART_CFG_DATA_BITS_8,     // 8 data bits
    UART_CFG_DATA_BITS_9,     // 9 data bits
}

/// Hardware flow control options.
///
/// With flow control set to none, any operations related to flow control signals can be managed by user with uart_line_ctrl functions. In other cases, flow control is managed by hardware/driver.  
///
/// (Note: This is just the Rust equivalent of `uart_config_flow_control` from `#include <zephyr/drivers/uart.h>`)
#[repr(u8)]
pub enum UartConfigFlowControl {
    UART_CFG_FLOW_CTRL_NONE = 0, // No flow control.
    UART_CFG_FLOW_CTRL_RTS_CTS,  // RTS/CTS flow control.
    UART_CFG_FLOW_CTRL_DTR_DSR,  // DTR/DSR flow control.
    UART_CFG_FLOW_CTRL_RS485,    // RS485 flow control.
}

// Guy used to hold the memory areas and waker for `Uart`
// Each instance of `Uart` gets their own unique `UartStatic`
// u_Note: This whole area is modelled after what's currently in gpio.rs.
const RX_DMA_BUFFER_SIZE: usize = 32;
const TX_DMA_BUFFER_SIZE: usize = 32;
const RX_RINGBUFFER_SIZE: usize = 256;
const TX_RINGBUFFER_SIZE: usize = 256;
struct UartStatic {
    // RX Stuff
    rx_dma_buffer: [UnsafeCell<[u8; RX_DMA_BUFFER_SIZE]>; 2], // 2 for double buffering
    rx_ringbuffer: UnsafeCell<heapless::spsc::Queue<u8, RX_RINGBUFFER_SIZE>>,
    rx_waker: embassy_sync::waitqueue::AtomicWaker,

    // TX Stuff
    tx_dma_buffer: [UnsafeCell<[u8; TX_DMA_BUFFER_SIZE]>; 2], // 2 for double buffering
    tx_ringbuffer: UnsafeCell<heapless::spsc::Queue<u8, TX_RINGBUFFER_SIZE>>,
    tx_waker: embassy_sync::waitqueue::AtomicWaker,
}
unsafe impl Sync for UartStatic {}

impl UartStatic {
    pub(crate) const fn new() -> Self {
        Self {
            rx_dma_buffer:   [const { UnsafeCell::new([0; RX_DMA_BUFFER_SIZE]) }; 2],
            rx_ringbuffer:  UnsafeCell::new(heapless::spsc::Queue::new()),
            rx_waker: embassy_sync::waitqueue::AtomicWaker::new(),

            tx_dma_buffer:   [const { UnsafeCell::new([0; TX_DMA_BUFFER_SIZE]) }; 2],
            tx_ringbuffer:  UnsafeCell::new(heapless::spsc::Queue::new()),
            tx_waker: embassy_sync::waitqueue::AtomicWaker::new(),
        }
    }
}

/// UART Peripheral
/// u_Note: eventually add more info to this comment (like devicetree config example(s) once that stuff is figured out)
pub struct Uart {
    device: *const crate::raw::device,    // The underlying device itself.
    pub(crate) data: &'static UartStatic, // Our associated data, used for callbacks.
}

impl Uart {
    /// Constructor, used by the devicetree generated code.
    pub(crate) unsafe fn new(
        unique: &crate::device::Unique,
        data: &'static UartStatic,
        device: *const crate::raw::device,
        _device_static: &'static crate::device::NoStatic,
    ) -> Option<Uart> {
        // Make sure this instance doesn't already exist.
        if !unique.once() { return None; }

        // u_Note: next stuff to do:
        // `uart_callback_set` to register the callback (don't know how specifically to do that). callback should drain the DMA buffer into the rx ringbuffer, then hand it back to `uart_rx_buf_rsp`, and then call rx_waker
        // after registering the callback, do something like `uart_rx_enable(device, data.rx_dma_buffer[0].get() as *mut u8, RX_DMA_BUFFER_SIZE, timeout)
        Some(Uart { device, data })
    }

    /// Lets you get the current uart config.
    pub(crate) fn config_get(&self) -> crate::error::Result<crate::raw::uart_config> {
        let mut config = crate::raw::uart_config::default();

        // Call uart_config_get()
        // SAFETY: `self.device` is valid for the lifetime of the object, and `&mut config` is a valid exclusive pointer of the type uart_config_get() expects.
        unsafe {
            if let Err(e) = crate::error::to_result_void(
                crate::raw::uart_config_get(self.device, &mut config),
            ) {
                match e.0 {
                    crate::raw::ENOSYS => log::error!("uart_confi_get() returned -ENOSYS: driver does not support getting current configuration."),
                    crate::raw::ENOTSUP => log::error!("uart_config_get() returned -ENOTSUP: API is not enabled."),
                    _ => log::error!("uart_config_get() failed with Zephyr errno {}", e),
                }
                return Err(e);
            }
        }

        Ok(config)
    }

    /// Reconfigure the UART at runtime.
    ///
    /// Most apps don't need to call this — the underlying Zephyr driver
    /// configures the hardware from devicetree (`current-speed`, etc.) at boot,
    /// defaulting to 8N1 with no flow control. Call this only if you need
    /// settings that differ from those devicetree defaults.
    pub fn configure(
        &self,
        baudrate: u32,
        parity: UartConfigParity,
        stop_bits: UartConfigStopBits,
        data_bits: UartConfigDataBits,
        flow_ctrl: UartConfigFlowControl,
    ) -> crate::error::Result<()> {
        let config = crate::raw::uart_config {
            baudrate,
            parity: parity as u8,
            stop_bits: stop_bits as u8,
            data_bits: data_bits as u8,
            flow_ctrl: flow_ctrl as u8,
        };

        // SAFETY: `self.device` is valid for the lifetime of `self`, and `&config` is a valid read-only pointer to an initialized `uart_config` that the C side only reads from.
        if let Err(e) = crate::error::to_result_void(unsafe {
            crate::raw::uart_configure(self.device, &config)
        }) {
            match e.0 {
                crate::raw::ENOSYS => log::error!("uart_configure() returned -ENOSYS: configuration is not supported by device or driver does not support setting configuration in runtime."),
                crate::raw::ENOTSUP => log::error!("uart_configure() returned -ENOTSUP: API is likely not enabled"),
                _ => log::error!("uart_configure() failed with Zephyr errno {}", e),
            }
            return Err(e);
        }

        Ok(())
    }
}

impl embedded_io_async::ErrorType for Uart {
    type Error = embedded_io_async::ErrorKind;
}

impl embedded_io_async::Read for Uart {
    async fn read(&mut self, _buf: &mut [u8]) -> Result<usize, Self::Error> {
        // u_Note: next stuff to do:
        // use core::future::pol_fn to drain bytes out of self.data.rx_ringbuffer into the `buf` param,
        // then if any bytes were copied, return `Poll::Ready(Ok(n))`
        // Otherwise, register `cx.waker()` on `self.data.rx_waker`, recheck the ring once, and return `Poll::Pending` if still empty
        Ok(0)
    }
}