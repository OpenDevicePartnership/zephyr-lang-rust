//! Device wrapper for a RTC clock.

/// A RTC peripheral.
/// u_Note: make a good comment here eventually
pub struct Rtc {
    device: *const crate::raw::device,
    resolution_hz: u32,
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

impl Rtc {
    /// Constructor, used by the devicetree generated code.
    #[allow(dead_code)]
    pub(crate) unsafe fn new(
        unique: &crate::device::Unique,
        _static: &crate::device::NoStatic,
        device: *const crate::raw::device,
        resolution_hz: u32,
    ) -> Option<Rtc> {
        // Make sure this instance doesn't already exist.
        if !unique.once() { return None; }
        Some(Rtc { device, resolution_hz })
    }
}
