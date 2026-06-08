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
fn to_rtctime(datetime: &embedded_mcu_hal::time::Datetime) -> Result<crate::raw::rtc_time, embedded_mcu_hal::time::DatetimeError> {
    use embedded_mcu_hal::time::DatetimeError;
    use crate::raw::rtc_time;

    Ok(rtc_time {
        tm_sec: i32::try_from(datetime.second()).map_err(|_| DatetimeError::Second)?,
        tm_min: i32::try_from(datetime.minute()).map_err(|_| DatetimeError::Minute)?,
        tm_hour: i32::try_from(datetime.hour()).map_err(|_| DatetimeError::Hour)?,
        tm_mday: i32::try_from(datetime.day()).map_err(|_| DatetimeError::Day)?,
        tm_nsec: i32::try_from(datetime.nanoseconds()).map_err(|_| DatetimeError::Nanosecond)?,

        // tm_mon is kind of annoying, since we first have to convert it from `Month` to u8, and then to i32
        tm_mon: i32::try_from(
            // When converting to u8, we subtract `1` from the value since rtc_time represents months as 0-11 while Datetime represents them as 1-12.
            (u8::try_from(datetime.month()).map_err(|_| DatetimeError::Month)?) - 1
        ).map_err(|_| DatetimeError::Month)?,

        // For year, we have to subtract 1900 since rtc_time represents tm_year as `Year - 1900` (and Datetime represents them normally)
        tm_year: i32::try_from(datetime.year() - 1900).map_err(|_| DatetimeError::Year)?,

        // The rest of these don't exist in `Datetime`, so we set them to -1 (which means "Unknown" in the context of the `rtc_time` struct)
        tm_wday: -1,
        tm_yday: -1,
        tm_isdst: -1,
    })
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
