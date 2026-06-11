//! Device wrapper for a fuel gauge.

/// A fuel gauge device.
/// u_Note: make a good comment here eventually
pub struct FuelGauge {
    device: *const crate::raw::device,
}

#[repr(u32)]
enum FuelGaugeProp {
    AvgCurrent = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_AVG_CURRENT,
    Cutoff = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_CHARGE_CUTOFF,
    Current = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_CURRENT,
    CycleCount = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_CYCLE_COUNT,
    ConnectState = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_CONNECT_STATE,
    Flags = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_FLAGS,
    FullChargeCapacity = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_FULL_CHARGE_CAPACITY,
    PresentState = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_PRESENT_STATE,
    RemainingCapacity = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_REMAINING_CAPACITY,
    RuntimeToEmpty = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_RUNTIME_TO_EMPTY,
    RuntimeToFull = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_RUNTIME_TO_FULL,
    SbsMfrAccessWord = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_SBS_MFR_ACCESS,
    AbsoluteStateOfCharge = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_ABSOLUTE_STATE_OF_CHARGE,
    RelativeStateOfCharge = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_RELATIVE_STATE_OF_CHARGE,
    Temperature = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_TEMPERATURE,
    Voltage = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_VOLTAGE,
    SbsMode = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_SBS_MODE,
    ChgCurrent = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_CHARGE_CURRENT,
    ChgVoltage = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_CHARGE_VOLTAGE,
    FgStatus = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_STATUS,
    DesignCap = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_DESIGN_CAPACITY,
    DesignVolt = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_DESIGN_VOLTAGE,
    SbsAtRate = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_SBS_ATRATE,
    SbsAtRateTimeToFull = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_SBS_ATRATE_TIME_TO_FULL,
    SbsAtRateTimeToEmpty = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_SBS_ATRATE_TIME_TO_EMPTY,
    SbsAtRateOk = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_SBS_ATRATE_OK,
    SbsRemainingCapacityAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_SBS_REMAINING_CAPACITY_ALARM,
    SbsRemainingTimeAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_SBS_REMAINING_TIME_ALARM,
    CurrentDirection = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_CURRENT_DIRECTION,
    StateOfChargeAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_STATE_OF_CHARGE_ALARM,
    LowVoltageAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_LOW_VOLTAGE_ALARM,
    HighVoltageAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_HIGH_VOLTAGE_ALARM,
    LowCurrentAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_LOW_CURRENT_ALARM,
    HighCurrentAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_HIGH_CURRENT_ALARM,
    LowTemperatureAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_LOW_TEMPERATURE_ALARM,
    HighTemperatureAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_HIGH_TEMPERATURE_ALARM,
    GpioVoltage = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_GPIO_VOLTAGE,
    LowGpioAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_LOW_GPIO_ALARM,
    HighGpioAlarm = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_HIGH_GPIO_ALARM,
    AdcMode = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_ADC_MODE,
    CcConfig = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_CC_CONFIG,
    StateOfHealth = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_STATE_OF_HEALTH,
}



impl FuelGauge {
    pub(crate) unsafe fn new(
        unique: &crate::device::Unique,
        _static: &crate::device::NoStatic,
        device: *const crate::raw::device,
    ) -> Option<FuelGauge> {
        if !unique.once() { return None; }
        Some(FuelGauge { device })
    }
}
