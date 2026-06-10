//! Device wrapper for a RTC clock.

use log::error;

/// Super thin wrapper providing a 1:1 translation of Zephyr's RTC API to
/// Rust, without any extra abstractions on top.
pub struct RtcRaw {
    device: *const crate::raw::device,
}

impl RtcRaw {
    pub(crate) unsafe fn new(
        unique: &crate::device::Unique,
        _static: &crate::device::NoStatic,
        device: *const crate::raw::device,
    ) -> Option<RtcRaw> {
        if !unique.once() { return None; }
        Some(RtcRaw { device })
    }

    /// Helper wrapper method for rtc_get_time()
    /// Returns the same as rtc_get_time():
    /// - ENODATA if RTC time has not been set
    /// - Generic Zephyr errno code if failure
    fn get_time(&self) -> crate::error::Result<crate::raw::rtc_time> {
        let mut buffer = core::mem::MaybeUninit::<crate::raw::rtc_time>::uninit();

        crate::error::to_result_void(
            // SAFETY: - `self.device` lives for the entire duration of `self`.
            //         - `buffer.as_mut_ptr()` is a valid pointer to a memory area of size `crate::raw::rtc_time` on the stack.
            //            This memory area is local to get_time(), so it will live as long as rtc_get_time() is using it.
            //            In other words, by the time `buffer` goes out of scope, rtc_get_time() will have already returned.
            unsafe { crate::raw::rtc_get_time(self.device, buffer.as_mut_ptr()) }
        )?;

        // SAFETY: If we get here, `buffer` is gaurunteed to be successfully initialized.
        Ok(unsafe { buffer.assume_init() })
    }

    /// Helper wrapper method for rtc_set_time()
    /// Returns the same as rtc_set_time():
    /// - EINVAL if RTC time is invalid or exceeds hardware capabilities
    /// - Generic Zephyr errno code if failure
    fn set_time(&self, rtc_time: &crate::raw::rtc_time) -> crate::error::Result<()> {
        crate::error::to_result_void(
            // SAFETY: - `self.device` lives for the entire duration of `self`.
            //         - `rtc_time` is a valid reference that lives for this call.
            unsafe { crate::raw::rtc_set_time(self.device, rtc_time as *const _) }
        )
    }
}

// Helper to convert zephyr::raw::rtc_time to embedded_mcu_hal::time::Datetime
fn to_datetime(rtc_time: &crate::raw::rtc_time) -> Result<embedded_mcu_hal::time::Datetime, embedded_mcu_hal::time::DatetimeError> {
    use embedded_mcu_hal::time::Datetime;
    use embedded_mcu_hal::time::DatetimeFields;
    use embedded_mcu_hal::time::DatetimeError;
    use embedded_mcu_hal::time::Month;

    Datetime::new(DatetimeFields {
        day:        u8::try_from(rtc_time.tm_mday).map_err(|_| DatetimeError::Day)?,
        hour:       u8::try_from(rtc_time.tm_hour).map_err(|_| DatetimeError::Hour)?,
        minute:     u8::try_from(rtc_time.tm_min).map_err(|_| DatetimeError::Minute)?,
        second:     u8::try_from(rtc_time.tm_sec).map_err(|_| DatetimeError::Second)?,
        nanosecond: u32::try_from(rtc_time.tm_nsec).map_err(|_| DatetimeError::Nanosecond)?,

        // For year, we have to add 1900 since rtc_time represents tm_year as `Year - 1900`.
        year:       u16::try_from(rtc_time.tm_year + 1900).map_err(|_| DatetimeError::Year)?,

        // month is kind of annoying, since we first have to convert it from i32 to u8, and then to `Month`
        month:      Month::try_from(
                        // When converting to u8, we add `1` to the value since rtc_time represents months as 0-11 while Datetime represents them as 1-12.
                        u8::try_from(rtc_time.tm_mon + 1).map_err(|_| DatetimeError::Month)?
                    ).map_err(|_| DatetimeError::Month)?,
    })
}

// Helper to convert embedded_mcu_hal::time::Datetime to zephyr::raw::rtc_time
fn to_rtctime(datetime: &embedded_mcu_hal::time::Datetime) -> crate::raw::rtc_time {
    // We don't have to handle any potential errors here, since `Datetime` is already the validated form.
    // Basically, it's impossible for any of these casts to fail.
    crate::raw::rtc_time {
        tm_sec: datetime.second() as i32,
        tm_min: datetime.minute() as i32,
        tm_hour: datetime.hour() as i32,
        tm_mday: datetime.day() as i32,
        tm_nsec: datetime.nanoseconds() as i32,

        // tm_mon is kind of annoying, since we first have to convert it from `Month` to u8, and then to i32
        tm_mon: (u8::from(datetime.month()) - 1) as i32,

        // For year, we have to subtract 1900 since rtc_time represents tm_year as `Year - 1900` (and Datetime represents them normally)
        tm_year: datetime.year() as i32 - 1900,

        // The rest of these don't exist in `Datetime`, so we set them to -1 (which means "Unknown" in the context of the `rtc_time` struct)
        tm_wday: -1,
        tm_yday: -1,
        tm_isdst: -1,
    }
}

/// A RTC peripheral.
/// u_Note: make a good comment here eventually
pub struct Rtc {
    device: RtcRaw,
    resolution_hz: u32,
}

impl Rtc {
    /// Constructor, used by the devicetree generated code.
    #[allow(dead_code)]
    pub(crate) unsafe fn new(
        unique: &crate::device::Unique,
        _static: &crate::device::NoStatic,
        device: RtcRaw,
        resolution_hz: u32,
    ) -> Option<Rtc> {
        // Make sure this instance doesn't already exist.
        if !unique.once() { return None; }
        Some(Rtc { device, resolution_hz })
    }
}

impl embedded_mcu_hal::time::DatetimeClock for Rtc {
    // Reads the current date and time from the hardware clock.
    fn now(&self) -> Result<embedded_mcu_hal::time::Datetime, embedded_mcu_hal::time::DatetimeClockError> {
        use embedded_mcu_hal::time::DatetimeClockError;

        // Get the current RTC time from Zephyr.
        let rtc_time: crate::raw::rtc_time = match self.device.get_time() {
            Ok(rtc_time) => rtc_time,
            Err(e) if e.0 == crate::raw::ENODATA => { error!("Rtc::now(): RTC time has not been set");             return Err(DatetimeClockError::Unknown); }
            Err(e) =>                               { error!("Rtc::now(): rtc_get_time() returned errno {}", e.0); return Err(DatetimeClockError::Unknown); }
        };
        
        // Convert this RTC data into the embedded_mcu_hal::Datetime format.
        let datetime: embedded_mcu_hal::time::Datetime = match to_datetime(&rtc_time) {
            Ok(datetime) => datetime,
            Err(err) => {
                error!("Failure in Rtc::now(): to_datetime() returned with error {:?}. There may have been an issue converting Zephyr's RTC data into the embedded_mcu_hal::Datetime format.", err);
                return Err(DatetimeClockError::Unknown);
            }
        };

        Ok(datetime)
    }

    // Sets the hardware clock to the given date and time.
    fn set(&mut self, datetime: embedded_mcu_hal::time::Datetime) -> Result<(), embedded_mcu_hal::time::DatetimeClockError> {
        use embedded_mcu_hal::time::DatetimeClockError;

        // Convert the provided Datetime into an rtc_time for Zephyr.
        let rtc_time: crate::raw::rtc_time = to_rtctime(&datetime);

        // Set the RTC time.
        match self.device.set_time(&rtc_time) {
            Ok(()) => Ok(()),
            Err(e) if e.0 == crate::raw::EINVAL => { error!("Rtc::set(): RTC time is invalid or exceeds hardware capabilities"); Err(DatetimeClockError::Unknown) }
            Err(e) =>                              { error!("Rtc::set(): rtc_set_time() returned errno {}", e.0);                Err(DatetimeClockError::Unknown) }
        }
    }

    // Returns the resolution of this RTC is Hz.
    fn resolution_hz(&self) -> u32 {
        return self.resolution_hz;
    }
}
