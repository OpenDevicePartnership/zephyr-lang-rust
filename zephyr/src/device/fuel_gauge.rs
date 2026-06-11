//! Device wrapper for a fuel gauge.

/// A fuel gauge device.
/// u_Note: make a good comment here eventually
pub struct FuelGauge {
    device: *const crate::raw::device,
}

#[repr(u32)]    
pub(crate) enum FuelGaugeProp {
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
    /// Constructor, used by the devicetree generated code.
    pub(crate) unsafe fn new(
        unique: &crate::device::Unique,
        _static: &crate::device::NoStatic,
        device: *const crate::raw::device,
    ) -> Option<FuelGauge> {
        if !unique.once() { return None; }
        Some(FuelGauge { device })
    }

    /// Private helper function to get a fuel gauge prop value.
    pub(crate) fn get_prop(&self, prop: FuelGaugeProp) -> crate::error::Result<crate::raw::fuel_gauge_prop_val> {
        let mut buffer = core::mem::MaybeUninit::<crate::raw::fuel_gauge_prop_val>::uninit();

        crate::error::to_result_void(
            // SAFETY: - `self.device` lives for the entire duration of `self`.
            //         -  `prop` is a copy owned by this function.
            //         - `buffer.as_mut_ptr()` is a valid pointer to a memory area of size `crate::raw::fuel_gauge_prop_val` on the stack.
            //            This memory area is local to get_prop(), so it will live as long as fuel_gauge_get_prop() is using it.
            //            In other words, by the time `buffer` goes out of scope, fuel_gauge_get_prop() will have already returned.
            unsafe { crate::raw::fuel_gauge_get_prop(self.device, prop as u16, buffer.as_mut_ptr()) }
        )?;

        // SAFETY: If we get here, `buffer` is gaurunteed to be successfully initialized.
        Ok(unsafe { buffer.assume_init() })
    }

    /// Private helper function to set a fuel gauge prop value.
    pub(crate) fn set_prop(&self, prop: FuelGaugeProp, val: crate::raw::fuel_gauge_prop_val) -> crate::error::Result<()> {
        crate::error::to_result_void(
            // SAFETY: - `self.device` lives for the entire duration of `self`.
            //         - `prop` is a copy owned by this function.
            //         - `val` is a copy owned by this function.
            unsafe { crate::raw::fuel_gauge_set_prop(self.device, prop as u16, val) }
        )
    }

    // u_TODO: Will probably want to make these function comments more descriptive in the future.
    //         Zephyr has docs for `enum fuel_gauge_prop_type`, where each of the enums has a comment
    //         about the units being returned + any extra info.

    /// Returns the gauge's `avg_current` reading.
    pub fn avg_current(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::AvgCurrent).map(|val| unsafe { *val.avg_current.as_ref() })
    }

    /// Returns the gauge's `cutoff` reading (for charge cutoff).
    pub fn cutoff(&self) -> crate::error::Result<bool> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::Cutoff).map(|val| unsafe { *val.cutoff.as_ref() })
    }

    /// Returns the gauge's `current` reading.
    pub fn current(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::Current).map(|val| unsafe { *val.current.as_ref() })
    }

    /// Returns the gauge's `cycle_count` reading.
    pub fn cycle_count(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::CycleCount).map(|val| unsafe { *val.cycle_count.as_ref() })
    }

    /// Returns the gauge's `connect_state` reading.
    pub fn connect_state(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::ConnectState).map(|val| unsafe { *val.connect_state.as_ref() })
    }

    /// Returns the gauge's `flags` reading.
    pub fn flags(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::Flags).map(|val| unsafe { *val.flags.as_ref() })
    } // u_TODO: may want to have a Rust enum for these flags in the future

    /// Returns the gauge's `full_charge_capacity` reading.
    pub fn full_charge_capacity(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::FullChargeCapacity).map(|val| unsafe { *val.full_charge_capacity.as_ref() })
    }

    /// Returns the gauge's `present_state` reading.
    pub fn present_state(&self) -> crate::error::Result<bool> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::PresentState).map(|val| unsafe { *val.present_state.as_ref() })
    }

    /// Returns the gauge's `remaining_capacity` reading.
    pub fn remaining_capacity(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::RemainingCapacity).map(|val| unsafe { *val.remaining_capacity.as_ref() })
    }

    /// Returns the gauge's `runtime_to_empty` reading.
    pub fn runtime_to_empty(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::RuntimeToEmpty).map(|val| unsafe { *val.runtime_to_empty.as_ref() })
    }

    /// Returns the gauge's `runtime_to_full` reading.
    pub fn runtime_to_full(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::RuntimeToFull).map(|val| unsafe { *val.runtime_to_full.as_ref() })
    }

    /// Returns the gauge's `sbs_mfr_access_word` reading.
    pub fn sbs_mfr_access_word(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsMfrAccessWord).map(|val| unsafe { *val.sbs_mfr_access_word.as_ref() })
    }

    /// Returns the gauge's `absolute_state_of_charge` reading.
    pub fn absolute_state_of_charge(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::AbsoluteStateOfCharge).map(|val| unsafe { *val.absolute_state_of_charge.as_ref() })
    }

    /// Returns the gauge's `relative_state_of_charge` reading.
    pub fn relative_state_of_charge(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::RelativeStateOfCharge).map(|val| unsafe { *val.relative_state_of_charge.as_ref() })
    }

    /// Returns the gauge's `temperature` reading.
    pub fn temperature(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::Temperature).map(|val| unsafe { *val.temperature.as_ref() })
    }

    /// Returns the gauge's `voltage` reading.
    pub fn voltage(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::Voltage).map(|val| unsafe { *val.voltage.as_ref() })
    }

    /// Returns the gauge's `sbs_mode` reading.
    pub fn sbs_mode(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsMode).map(|val| unsafe { *val.sbs_mode.as_ref() })
    }

    /// Sets the gauge's `sbs_mode` setting.
    pub fn set_sbs_mode(&self, value: u16) -> crate::error::Result<()> {
        // SAFETY: All union fields are primitive types, so zeroed memory is valid.
        let mut val: crate::raw::fuel_gauge_prop_val = unsafe { core::mem::zeroed() };
        
        // SAFETY: Writing to the correct union field for this property.
        unsafe { *val.sbs_mode.as_mut() = value; }
        
        self.set_prop(FuelGaugeProp::SbsMode, val)
    }

    /// Returns the gauge's `chg_current` (charge current) reading.
    pub fn chg_current(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::ChgCurrent).map(|val| unsafe { *val.chg_current.as_ref() })
    }

    /// Returns the gauge's `chg_voltage` (charge voltage) reading.
    pub fn chg_voltage(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::ChgVoltage).map(|val| unsafe { *val.chg_voltage.as_ref() })
    }

    /// Returns the gauge's `fg_status` reading.
    pub fn fg_status(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::FgStatus).map(|val| unsafe { *val.fg_status.as_ref() })
    }

    /// Returns the gauge's `design_cap` (design capacity) reading.
    pub fn design_cap(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::DesignCap).map(|val| unsafe { *val.design_cap.as_ref() })
    }

    /// Returns the gauge's `design_volt` (design voltage) reading.
    pub fn design_volt(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::DesignVolt).map(|val| unsafe { *val.design_volt.as_ref() })
    }

    /// Returns the gauge's `sbs_at_rate` reading.
    pub fn sbs_at_rate(&self) -> crate::error::Result<i16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsAtRate).map(|val| unsafe { *val.sbs_at_rate.as_ref() })
    }

    /// Sets the gauge's `at_rate` setting.
    pub fn set_sbs_at_rate(&self, value: i16) -> crate::error::Result<()> {
        // SAFETY: All union fields are primitive types, so zeroed memory is valid.
        let mut val: crate::raw::fuel_gauge_prop_val = unsafe { core::mem::zeroed() };
        
        // SAFETY: Writing to the correct union field for this property.
        unsafe { *val.sbs_at_rate.as_mut() = value; }
        
        self.set_prop(FuelGaugeProp::SbsAtRate, val)
    }

    /// Returns the gauge's `sbs_at_rate_time_to_full` reading.
    pub fn sbs_at_rate_time_to_full(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsAtRateTimeToFull).map(|val| unsafe { *val.sbs_at_rate_time_to_full.as_ref() })
    }

    /// Returns the gauge's `sbs_at_rate_time_to_empty` reading.
    pub fn sbs_at_rate_time_to_empty(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsAtRateTimeToEmpty).map(|val| unsafe { *val.sbs_at_rate_time_to_empty.as_ref() })
    }

    /// Returns the gauge's `sbs_at_rate_ok` reading.
    pub fn sbs_at_rate_ok(&self) -> crate::error::Result<bool> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsAtRateOk).map(|val| unsafe { *val.sbs_at_rate_ok.as_ref() })
    }

    /// Returns the gauge's `sbs_remaining_capacity_alarm` reading.
    pub fn sbs_remaining_capacity_alarm(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsRemainingCapacityAlarm).map(|val| unsafe { *val.sbs_remaining_capacity_alarm.as_ref() })
    }

    /// Sets the gauge's `sbs_remaining_capacity_alarm` setting.
    pub fn set_sbs_remaining_capacity_alarm(&self, value: u16) -> crate::error::Result<()> {
        // SAFETY: All union fields are primitive types, so zeroed memory is valid.
        let mut val: crate::raw::fuel_gauge_prop_val = unsafe { core::mem::zeroed() };
        
        // SAFETY: Writing to the correct union field for this property.
        unsafe { *val.sbs_remaining_capacity_alarm.as_mut() = value; }
        
        self.set_prop(FuelGaugeProp::SbsRemainingCapacityAlarm, val)
    }

    /// Returns the gauge's `sbs_remaining_time_alarm` reading.
    pub fn sbs_remaining_time_alarm(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsRemainingTimeAlarm).map(|val| unsafe { *val.sbs_remaining_time_alarm.as_ref() })
    }

    /// Sets the gauge's `sbs_remaining_time_alarm` setting.
    pub fn set_sbs_remaining_time_alarm(&self, value: u16) -> crate::error::Result<()> {
        // SAFETY: All union fields are primitive types, so zeroed memory is valid.
        let mut val: crate::raw::fuel_gauge_prop_val = unsafe { core::mem::zeroed() };
        
        // SAFETY: Writing to the correct union field for this property.
        unsafe { *val.sbs_remaining_time_alarm.as_mut() = value; }
        
        self.set_prop(FuelGaugeProp::SbsRemainingTimeAlarm, val)
    }

    /// Returns the gauge's `current_direction` reading.
    pub fn current_direction(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::CurrentDirection).map(|val| unsafe { *val.current_direction.as_ref() })
    }

    /// Returns the gauge's `state_of_charge_alarm` reading.
    pub fn state_of_charge_alarm(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::StateOfChargeAlarm).map(|val| unsafe { *val.state_of_charge_alarm.as_ref() })
    }

    /// Returns the gauge's `low_voltage_alarm` reading.
    pub fn low_voltage_alarm(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::LowVoltageAlarm).map(|val| unsafe { *val.low_voltage_alarm.as_ref() })
    }

    /// Returns the gauge's `high_voltage_alarm` reading.
    pub fn high_voltage_alarm(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::HighVoltageAlarm).map(|val| unsafe { *val.high_voltage_alarm.as_ref() })
    }

    /// Returns the gauge's `low_current_alarm` reading.
    pub fn low_current_alarm(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::LowCurrentAlarm).map(|val| unsafe { *val.low_current_alarm.as_ref() })
    }

    /// Returns the gauge's `high_current_alarm` reading.
    pub fn high_current_alarm(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::HighCurrentAlarm).map(|val| unsafe { *val.high_current_alarm.as_ref() })
    }

    /// Returns the gauge's `low_temperature_alarm` reading.
    pub fn low_temperature_alarm(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::LowTemperatureAlarm).map(|val| unsafe { *val.low_temperature_alarm.as_ref() })
    }

    /// Returns the gauge's `high_temperature_alarm` reading.
    pub fn high_temperature_alarm(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::HighTemperatureAlarm).map(|val| unsafe { *val.high_temperature_alarm.as_ref() })
    }

    /// Returns the gauge's `gpio_voltage` reading.
    pub fn gpio_voltage(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::GpioVoltage).map(|val| unsafe { *val.gpio_voltage.as_ref() })
    }

    /// Returns the gauge's `low_gpio_alarm` reading.
    pub fn low_gpio_alarm(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::LowGpioAlarm).map(|val| unsafe { *val.low_gpio_alarm.as_ref() })
    }

    /// Returns the gauge's `high_gpio_alarm` reading.
    pub fn high_gpio_alarm(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::HighGpioAlarm).map(|val| unsafe { *val.high_gpio_alarm.as_ref() })
    }

    /// Returns the gauge's `adc_mode` reading.
    pub fn adc_mode(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::AdcMode).map(|val| unsafe { *val.adc_mode.as_ref() })
    }

    /// Returns the gauge's `cc_config` (coulomb counter config) reading.
    pub fn cc_config(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::CcConfig).map(|val| unsafe { *val.cc_config.as_ref() })
    }

    /// Returns the gauge's `state_of_health` reading.
    pub fn state_of_health(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::StateOfHealth).map(|val| unsafe { *val.state_of_health.as_ref() })
    }
}
