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

#[repr(u32)]
pub(crate) enum FuelGaugeBufferProp {
    ManufacturerName = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_MANUFACTURER_NAME,
    DeviceName = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_DEVICE_NAME,
    DeviceChemistry = crate::raw::fuel_gauge_prop_type_FUEL_GAUGE_DEVICE_CHEMISTRY,
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

    /// Private helper function to get a fuel gauge buffer prop value.
    pub(crate) fn get_buffer_prop(&self, prop: FuelGaugeBufferProp, buffer: &mut [u8]) -> crate::error::Result<()> {
        crate::error::to_result_void(
            // SAFETY: - `self.device` lives for the entire duration of `self`.
            //         - `prop` is a copy owned by this function.
            //         - `buffer.as_mut_ptr()` is a valid pointer to a writable memory region
            //           of at least `buffer.len()` bytes that lives for the duration of this call.
            //         - The caller is responsible for passing a buffer of the correct size
            //           for the requested property.
            unsafe {
                crate::raw::fuel_gauge_get_buffer_prop(
                    self.device,
                    prop as u16,
                    buffer.as_mut_ptr() as *mut core::ffi::c_void,
                    buffer.len(),
                )
            }
        )
    }

    // u_TODO: Will probably want to make these function comments more descriptive in the future.
    //         Zephyr has docs for `enum fuel_gauge_prop_type`, where each of the enums has a comment
    //         about the units being returned + any extra info.

    /// Reads the gauge's `manufacturer_name` into the provided buffer.
    /// According to Zephyr, manufacturer name is 1 byte of string length + 20 bytes of data (21 bytes total).
    pub fn manufacturer_name(&self, buffer: &mut [u8]) -> crate::error::Result<()> {
        self.get_buffer_prop(FuelGaugeBufferProp::ManufacturerName, buffer)
    }

    /// Reads the gauge's `device_name` into the provided buffer.
    /// According to Zephyr, device name is 1 byte of string length + 20 bytes of data (21 bytes total).
    pub fn device_name(&self, buffer: &mut [u8]) -> crate::error::Result<()> {
        self.get_buffer_prop(FuelGaugeBufferProp::DeviceName, buffer)
    }

    /// Reads the gauge's `device_chemistry` into the provided buffer.
    /// According to Zephyr, device chemistry is 1 byte of string length + 4 bytes of data (5 bytes total).
    pub fn device_chemistry(&self, buffer: &mut [u8]) -> crate::error::Result<()> {
        self.get_buffer_prop(FuelGaugeBufferProp::DeviceChemistry, buffer)
    }

    /// Returns the gauge's `avg_current` reading.
    /// 
    /// Zephyr notes: Provide a 1 minute average of the current on the battery. Does not check for flags or whether those values are bad readings. See driver instance header for details on implementation and how the average is calculated. Units in uA negative=discharging 
    pub fn avg_current(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::AvgCurrent).map(|val| unsafe { *val.avg_current.as_ref() })
    }

    /// Returns the gauge's `cutoff` reading (for charge cutoff).
    /// 
    /// Zephyr notes: Whether the battery underlying the fuel-gauge is cut off from charge. 
    pub fn cutoff(&self) -> crate::error::Result<bool> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::Cutoff).map(|val| unsafe { *val.cutoff.as_ref() })
    }

    /// Returns the gauge's `current` reading.
    /// 
    /// Zephyr notes: Battery current (uA); negative=discharging. 
    pub fn current(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::Current).map(|val| unsafe { *val.current.as_ref() })
    }

    /// Returns the gauge's `cycle_count` reading.
    /// 
    /// Zephyr notes: Cycle count in 1/100ths (number of charge/discharge cycles). 
    pub fn cycle_count(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::CycleCount).map(|val| unsafe { *val.cycle_count.as_ref() })
    }

    /// Returns the gauge's `connect_state` reading.
    /// 
    /// Zephyr notes: Connect state of battery. 
    pub fn connect_state(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::ConnectState).map(|val| unsafe { *val.connect_state.as_ref() })
    }

    /// Returns the gauge's `flags` reading.
    /// 
    /// Zephyr notes: General Error/Runtime Flags. 
    pub fn flags(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::Flags).map(|val| unsafe { *val.flags.as_ref() })
    } // u_TODO: may want to have a Rust enum for these flags in the future

    /// Returns the gauge's `full_charge_capacity` reading.
    /// 
    /// Zephyr notes: Full Charge Capacity in uAh (might change in some implementations to determine wear).
    pub fn full_charge_capacity(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::FullChargeCapacity).map(|val| unsafe { *val.full_charge_capacity.as_ref() })
    }

    /// Returns the gauge's `present_state` reading.
    /// 
    /// Zephyr notes: Is the battery physically present. 
    pub fn present_state(&self) -> crate::error::Result<bool> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::PresentState).map(|val| unsafe { *val.present_state.as_ref() })
    }

    /// Returns the gauge's `remaining_capacity` reading.
    /// 
    /// Zephyr notes: Remaining capacity in uAh.
    pub fn remaining_capacity(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::RemainingCapacity).map(|val| unsafe { *val.remaining_capacity.as_ref() })
    }

    /// Returns the gauge's `runtime_to_empty` reading.
    /// 
    /// Zephyr notes: Remaining battery life time in minutes.
    pub fn runtime_to_empty(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::RuntimeToEmpty).map(|val| unsafe { *val.runtime_to_empty.as_ref() })
    }

    /// Returns the gauge's `runtime_to_full` reading.
    /// 
    /// Zephyr notes: Remaining time in minutes until battery reaches full charge.
    pub fn runtime_to_full(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::RuntimeToFull).map(|val| unsafe { *val.runtime_to_full.as_ref() })
    }

    /// Returns the gauge's `sbs_mfr_access_word` reading.
    /// 
    /// Zephyr notes: Retrieve word from SBS1.1 ManufacturerAccess.
    pub fn sbs_mfr_access_word(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsMfrAccessWord).map(|val| unsafe { *val.sbs_mfr_access_word.as_ref() })
    }

    /// Returns the gauge's `absolute_state_of_charge` reading.
    /// 
    /// Zephyr notes: Absolute state of charge (percent, 0-100) - expressed as % of design capacity.
    pub fn absolute_state_of_charge(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::AbsoluteStateOfCharge).map(|val| unsafe { *val.absolute_state_of_charge.as_ref() })
    }

    /// Returns the gauge's `relative_state_of_charge` reading.
    /// 
    /// Zephyr notes: Relative state of charge (percent, 0-100) - expressed as % of full charge capacity.
    pub fn relative_state_of_charge(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::RelativeStateOfCharge).map(|val| unsafe { *val.relative_state_of_charge.as_ref() })
    }

    /// Returns the gauge's `temperature` reading.
    /// 
    /// Zephyr notes: Temperature in 0.1 K.
    pub fn temperature(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::Temperature).map(|val| unsafe { *val.temperature.as_ref() })
    }

    /// Returns the gauge's `voltage` reading.
    /// 
    /// Zephyr notes: Battery voltage (uV).
    pub fn voltage(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::Voltage).map(|val| unsafe { *val.voltage.as_ref() })
    }

    /// Returns the gauge's `sbs_mode` reading.
    /// 
    /// Zephyr notes: Battery Mode (flags).
    pub fn sbs_mode(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsMode).map(|val| unsafe { *val.sbs_mode.as_ref() })
    }

    /// Sets the gauge's `sbs_mode` setting.
    /// 
    /// Zephyr notes: Battery Mode (flags).
    pub fn set_sbs_mode(&self, value: u16) -> crate::error::Result<()> {
        // SAFETY: All union fields are primitive types, so zeroed memory is valid.
        let mut val: crate::raw::fuel_gauge_prop_val = unsafe { core::mem::zeroed() };
        
        // SAFETY: Writing to the correct union field for this property.
        unsafe { *val.sbs_mode.as_mut() = value; }
        
        self.set_prop(FuelGaugeProp::SbsMode, val)
    }

    /// Returns the gauge's `chg_current` (charge current) reading.
    /// 
    /// Zephyr notes: Battery desired Max Charging Current (uA).
    pub fn chg_current(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::ChgCurrent).map(|val| unsafe { *val.chg_current.as_ref() })
    }

    /// Returns the gauge's `chg_voltage` (charge voltage) reading.
    /// 
    /// Zephyr notes: Battery desired Max Charging Voltage (uV).
    pub fn chg_voltage(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::ChgVoltage).map(|val| unsafe { *val.chg_voltage.as_ref() })
    }

    /// Returns the gauge's `fg_status` reading.
    /// 
    /// Zephyr notes: Alarm, Status and Error codes (flags).
    pub fn fg_status(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::FgStatus).map(|val| unsafe { *val.fg_status.as_ref() })
    }

    /// Returns the gauge's `design_cap` (design capacity) reading.
    /// 
    /// Zephyr notes: Design Capacity (mAh or 10mWh).
    pub fn design_cap(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::DesignCap).map(|val| unsafe { *val.design_cap.as_ref() })
    }

    /// Returns the gauge's `design_volt` (design voltage) reading.
    /// 
    /// Zephyr notes: Design Voltage (mV).
    pub fn design_volt(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::DesignVolt).map(|val| unsafe { *val.design_volt.as_ref() })
    }

    /// Returns the gauge's `sbs_at_rate` reading.
    /// 
    /// Zephyr notes: AtRate (mA or 10 mW).
    pub fn sbs_at_rate(&self) -> crate::error::Result<i16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsAtRate).map(|val| unsafe { *val.sbs_at_rate.as_ref() })
    }

    /// Sets the gauge's `at_rate` setting.
    /// 
    /// Zephyr notes: AtRate (mA or 10 mW).
    pub fn set_sbs_at_rate(&self, value: i16) -> crate::error::Result<()> {
        // SAFETY: All union fields are primitive types, so zeroed memory is valid.
        let mut val: crate::raw::fuel_gauge_prop_val = unsafe { core::mem::zeroed() };
        
        // SAFETY: Writing to the correct union field for this property.
        unsafe { *val.sbs_at_rate.as_mut() = value; }
        
        self.set_prop(FuelGaugeProp::SbsAtRate, val)
    }

    /// Returns the gauge's `sbs_at_rate_time_to_full` reading.
    /// 
    /// Zephyr notes: AtRateTimeToFull (minutes).
    pub fn sbs_at_rate_time_to_full(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsAtRateTimeToFull).map(|val| unsafe { *val.sbs_at_rate_time_to_full.as_ref() })
    }

    /// Returns the gauge's `sbs_at_rate_time_to_empty` reading.
    /// 
    /// Zephyr notes: AtRateTimeToEmpty (minutes).
    pub fn sbs_at_rate_time_to_empty(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsAtRateTimeToEmpty).map(|val| unsafe { *val.sbs_at_rate_time_to_empty.as_ref() })
    }

    /// Returns the gauge's `sbs_at_rate_ok` reading.
    /// 
    /// Zephyr notes: AtRateOK (boolean).
    pub fn sbs_at_rate_ok(&self) -> crate::error::Result<bool> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsAtRateOk).map(|val| unsafe { *val.sbs_at_rate_ok.as_ref() })
    }

    /// Returns the gauge's `sbs_remaining_capacity_alarm` reading.
    /// 
    /// Zephyr notes: Remaining Capacity Alarm (mAh or 10mWh).
    pub fn sbs_remaining_capacity_alarm(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsRemainingCapacityAlarm).map(|val| unsafe { *val.sbs_remaining_capacity_alarm.as_ref() })
    }

    /// Sets the gauge's `sbs_remaining_capacity_alarm` setting.
    /// 
    /// Zephyr notes: Remaining Capacity Alarm (mAh or 10mWh).
    pub fn set_sbs_remaining_capacity_alarm(&self, value: u16) -> crate::error::Result<()> {
        // SAFETY: All union fields are primitive types, so zeroed memory is valid.
        let mut val: crate::raw::fuel_gauge_prop_val = unsafe { core::mem::zeroed() };
        
        // SAFETY: Writing to the correct union field for this property.
        unsafe { *val.sbs_remaining_capacity_alarm.as_mut() = value; }
        
        self.set_prop(FuelGaugeProp::SbsRemainingCapacityAlarm, val)
    }

    /// Returns the gauge's `sbs_remaining_time_alarm` reading.
    /// 
    /// Zephyr notes: Remaining Time Alarm (minutes).
    pub fn sbs_remaining_time_alarm(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::SbsRemainingTimeAlarm).map(|val| unsafe { *val.sbs_remaining_time_alarm.as_ref() })
    }

    /// Sets the gauge's `sbs_remaining_time_alarm` setting.
    /// 
    /// Zephyr notes: Remaining Time Alarm (minutes).
    pub fn set_sbs_remaining_time_alarm(&self, value: u16) -> crate::error::Result<()> {
        // SAFETY: All union fields are primitive types, so zeroed memory is valid.
        let mut val: crate::raw::fuel_gauge_prop_val = unsafe { core::mem::zeroed() };
        
        // SAFETY: Writing to the correct union field for this property.
        unsafe { *val.sbs_remaining_time_alarm.as_mut() = value; }
        
        self.set_prop(FuelGaugeProp::SbsRemainingTimeAlarm, val)
    }

    /// Returns the gauge's `current_direction` reading.
    /// 
    /// Zephyr notes: Battery current direction (flags).
    pub fn current_direction(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::CurrentDirection).map(|val| unsafe { *val.current_direction.as_ref() })
    }

    /// Returns the gauge's `state_of_charge_alarm` reading.
    /// 
    /// Zephyr notes: Remaining state of charge alarm (percent, 0-100).
    pub fn state_of_charge_alarm(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::StateOfChargeAlarm).map(|val| unsafe { *val.state_of_charge_alarm.as_ref() })
    }

    /// Returns the gauge's `low_voltage_alarm` reading.
    /// 
    /// Zephyr notes: Low Cell Voltage Alarm (uV).
    pub fn low_voltage_alarm(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::LowVoltageAlarm).map(|val| unsafe { *val.low_voltage_alarm.as_ref() })
    }

    /// Returns the gauge's `high_voltage_alarm` reading.
    /// 
    /// Zephyr notes: High Cell Voltage Alarm (uV).
    pub fn high_voltage_alarm(&self) -> crate::error::Result<u32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::HighVoltageAlarm).map(|val| unsafe { *val.high_voltage_alarm.as_ref() })
    }

    /// Returns the gauge's `low_current_alarm` reading.
    /// 
    /// Zephyr notes: Low Cell Current Alarm (uA).
    pub fn low_current_alarm(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::LowCurrentAlarm).map(|val| unsafe { *val.low_current_alarm.as_ref() })
    }

    /// Returns the gauge's `high_current_alarm` reading.
    /// 
    /// Zephyr notes: High Cell Current Alarm (uA).
    pub fn high_current_alarm(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::HighCurrentAlarm).map(|val| unsafe { *val.high_current_alarm.as_ref() })
    }

    /// Returns the gauge's `low_temperature_alarm` reading.
    /// 
    /// Zephyr notes: Low Cell Temperature Alarm (dK).
    pub fn low_temperature_alarm(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::LowTemperatureAlarm).map(|val| unsafe { *val.low_temperature_alarm.as_ref() })
    }

    /// Returns the gauge's `high_temperature_alarm` reading.
    /// 
    /// Zephyr notes: High Cell Temperature Alarm (dK).
    pub fn high_temperature_alarm(&self) -> crate::error::Result<u16> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::HighTemperatureAlarm).map(|val| unsafe { *val.high_temperature_alarm.as_ref() })
    }

    /// Returns the gauge's `gpio_voltage` reading.
    /// 
    /// Zephyr notes: GPIO Voltage (uV).
    pub fn gpio_voltage(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::GpioVoltage).map(|val| unsafe { *val.gpio_voltage.as_ref() })
    }

    /// Returns the gauge's `low_gpio_alarm` reading.
    /// 
    /// Zephyr notes: Low GPIO Voltage Alarm (uV).
    pub fn low_gpio_alarm(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::LowGpioAlarm).map(|val| unsafe { *val.low_gpio_alarm.as_ref() })
    }

    /// Returns the gauge's `high_gpio_alarm` reading.
    /// 
    /// Zephyr notes: High GPIO Voltage Alarm (uV).
    pub fn high_gpio_alarm(&self) -> crate::error::Result<i32> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::HighGpioAlarm).map(|val| unsafe { *val.high_gpio_alarm.as_ref() })
    }

    /// Returns the gauge's `adc_mode` reading.
    /// 
    /// Zephyr notes: ADC Mode (flags).
    pub fn adc_mode(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::AdcMode).map(|val| unsafe { *val.adc_mode.as_ref() })
    }

    /// Returns the gauge's `cc_config` (coulomb counter config) reading.
    /// 
    /// Zephyr notes: Coulomb Counter Config (flags).
    pub fn cc_config(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::CcConfig).map(|val| unsafe { *val.cc_config.as_ref() })
    }

    /// Returns the gauge's `state_of_health` reading.
    ///
    /// Zephyr notes: State of Health (SoH) (percent, 0-100).
    pub fn state_of_health(&self) -> crate::error::Result<u8> {
        // SAFETY: Per the Zephyr API contract, the driver will have populated the correct field of the union.
        self.get_prop(FuelGaugeProp::StateOfHealth).map(|val| unsafe { *val.state_of_health.as_ref() })
    }
}

use embedded_batteries_async::smart_battery::{BatteryModeFields, ManufactureDate, Cycles, BatteryStatusFields, CapacityModeValue, SpecificationInfoFields, CapacityModeSignedValue, MilliVolts, Minutes, DeciKelvin, MilliAmps, MilliAmpsSigned, Percent};
impl embedded_batteries_async::smart_battery::SmartBattery for FuelGauge {

    async fn battery_mode(&mut self) -> Result<BatteryModeFields, Self::Error> {
        Ok(BatteryModeFields::from_bits(self.sbs_mode()?))
    }

    async fn remaining_capacity_alarm(&mut self) -> Result<CapacityModeValue, Self::Error> {
        let value = self.sbs_remaining_capacity_alarm()?; // Returned in either mAh or 10mWh depending on the capacity_mode, according to Zephyr docs.
        let capacity_mode: bool = self.battery_mode().await?.capacity_mode();

        // When true, the capacity information should be reported in 10mW or 10mWh as appropriate.
        // When false, the capacity information should be reported in mA or mAh as appropriate.
        match capacity_mode {
            true => Ok(CapacityModeValue::CentiWattUnsigned(value)), // centiwatt == 10mW
            false => Ok(CapacityModeValue::MilliAmpUnsigned(value)),
        }
    }

    #[allow(non_snake_case)]
    async fn set_remaining_capacity_alarm(&mut self, capacity: CapacityModeValue) -> Result<(), Self::Error> {
        let capacity_mode: bool = self.battery_mode().await?.capacity_mode();

        // When true, the capacity information should be reported in 10mW or 10mWh as appropriate.
        // When false, the capacity information should be reported in mA or mAh as appropriate.
        let raw: u16 = match (capacity_mode, capacity) {

            // Good cases where the provided `capacity` is consistent with `capacity_mode`
            (true, CapacityModeValue::CentiWattUnsigned(value)) => value, // centiwatt == 10mW
            (false, CapacityModeValue::MilliAmpUnsigned(value)) => value,

            // Mismatch: `capacity_mode` expects mAh, but `capacity` is in 10mWh (cWh).
            (false, CapacityModeValue::CentiWattUnsigned(_)) => {
                log::error!("In set_remaining_capacity_alarm: `capacity_mode` expected a value in mAh, but caller provided a value in cWh to the `capacity` parameter. Invalid.");
                return Err(crate::error::Error(crate::raw::EINVAL));
            },

            // Mismatch: `capacity_mode` expects 10mWh (cWh), but `capacity` is in mAh.
            (true, CapacityModeValue::MilliAmpUnsigned(_)) => {
                log::error!("In set_remaining_capacity_alarm: `capacity_mode` expected a value in cWh, but caller provided a value in mAh to the `capacity` parameter. Invalid.");
                return Err(crate::error::Error(crate::raw::EINVAL));
            },
        };

        self.set_sbs_remaining_capacity_alarm(raw)
    }

    async fn remaining_time_alarm(&mut self) -> Result<Minutes, Self::Error> {
        Ok(self.sbs_remaining_time_alarm()? as Minutes)
    }

    async fn set_remaining_time_alarm(&mut self, time: Minutes) -> Result<(), Self::Error> {
        Ok(self.set_sbs_remaining_time_alarm(time as u16)?)
    }

    async fn set_battery_mode(&mut self, flags: BatteryModeFields) -> Result<(), Self::Error> {
        Ok(self.set_sbs_mode(BatteryModeFields::into_bits(flags))?)
    }

    async fn at_rate(&mut self) -> Result<CapacityModeSignedValue, Self::Error> {
        let value: i16 = self.sbs_at_rate()?; // Returned in either mA or 10mW depending on the capacity_mode, according to Zephyr docs.
        let capacity_mode: bool = self.battery_mode().await?.capacity_mode();

        // When true, the capacity information should be reported in 10mW or 10mWh as appropriate.
        // When false, the capacity information should be reported in mA or mAh as appropriate.
        match capacity_mode {
            true => Ok(CapacityModeSignedValue::CentiWattSigned(value)), // centiwatt == 10mW
            false => Ok(CapacityModeSignedValue::MilliAmpSigned(value)),
        }
    }

    async fn set_at_rate(&mut self, rate: CapacityModeSignedValue) -> Result<(), Self::Error> {
        let capacity_mode: bool = self.battery_mode().await?.capacity_mode();

        // When true, the rate information should be reported in 10mW or 10mWh as appropriate.
        // When false, the rate information should be reported in mA or mAh as appropriate.
        let raw: i16 = match (capacity_mode, rate) {

            // Good cases where the provided `rate` is consistent with `capacity_mode`
            (true, CapacityModeSignedValue::CentiWattSigned(value)) => value, // centiwatt == 10mW
            (false, CapacityModeSignedValue::MilliAmpSigned(value)) => value,

            // Mismatch: `capacity_mode` expects mAh, but `rate` is in 10mWh (cWh).
            (false, CapacityModeSignedValue::CentiWattSigned(_)) => {
                log::error!("In set_at_rate: `capacity_mode` expected a value in mAh, but caller provided a value in cWh to the `rate` parameter. Invalid.");
                return Err(crate::error::Error(crate::raw::EINVAL));
            },

            // Mismatch: `capacity_mode` expects 10mWh (cWh), but `rate` is in mAh.
            (true, CapacityModeSignedValue::MilliAmpSigned(_)) => {
                log::error!("In set_at_rate: `capacity_mode` expected a value in cWh, but caller provided a value in mAh to the `rate` parameter. Invalid.");
                return Err(crate::error::Error(crate::raw::EINVAL));
            },
        };

        self.set_sbs_at_rate(raw)
    }

    async fn at_rate_time_to_full(&mut self) -> Result<Minutes, Self::Error> {
        Ok(self.sbs_at_rate_time_to_full()? as Minutes)
    }

    async fn at_rate_time_to_empty(&mut self) -> Result<Minutes, Self::Error> {
        Ok(self.sbs_at_rate_time_to_empty()? as Minutes)
    }

    async fn at_rate_ok(&mut self) -> Result<bool, Self::Error> {
        Ok(self.sbs_at_rate_ok()?)
    }

    async fn temperature(&mut self) -> Result<DeciKelvin, Self::Error> {
        Ok(self.temperature()? as DeciKelvin)
    }

    async fn voltage(&mut self) -> Result<MilliVolts, Self::Error> {
        let microvolts: i32 = FuelGauge::voltage(self)?; // FuelGauge::voltage(self) returns in uV

        // We need to convert to mV according to the trait method requirement.
        let millivolts: i32 = microvolts / 1000;

        // We also need to convert from `i32` to `u16`. In case we read a negative voltage or we run into overflow, print an error.
        let result: u16 = u16::try_from(millivolts).map_err(|_| {
            log::error!("Voltage out of u16 range: {}mV! Returning an error.", millivolts);
            crate::error::Error(crate::raw::EINVAL)
        })?;

        Ok(result as MilliVolts)
    }

    async fn current(&mut self) -> Result<MilliAmpsSigned, Self::Error> {
        let microamps: i32 = FuelGauge::current(self)?; // FuelGauge::current(self) returns in uA

        // We need to convert to mA according to the trait method requirement.
        let milliamps: i32 = microamps / 1000;

        // We also need to convert from `i32` to `i16`. In case we run into overflow, print an error.
        let result: i16 = i16::try_from(milliamps).map_err(|_| {
            log::error!("Current out of i16 range: {}mA! Returning an error.", milliamps);
            crate::error::Error(crate::raw::EINVAL)
        })?;

        Ok(result as MilliAmpsSigned)
    }

    async fn average_current(&mut self) -> Result<MilliAmpsSigned, Self::Error> {
        let microamps: i32 = self.avg_current()?; // self.avg_current returns in uA

        // We need to convert to mA according to the trait method requirement.
        let milliamps: i32 = microamps / 1000;

        // We also need to convert from `i32` to `i16`. In case we run into overflow, print an error.
        let result: i16 = i16::try_from(milliamps).map_err(|_| {
            log::error!("Current out of i16 range: {}mA! Returning an error.", milliamps);
            crate::error::Error(crate::raw::EINVAL)
        })?;

        Ok(result as MilliAmpsSigned)
    }

    async fn max_error(&mut self) -> Result<Percent, Self::Error> {
        Err(crate::error::Error(crate::raw::ENOTSUP)) // u_TODO: Zephyr API doesn't give us this as far as I can tell. I think this should be fairly easy to add upstream on Zephyr though since this is part of the SBS spec and there seems to be a `SBS_GAUGE_CMD_MAX_ERROR` defined in sbs_gauge.h, it just isn't exposed aynwhere in the public API 
    }

    async fn relative_state_of_charge(&mut self) -> Result<Percent, Self::Error> {
        Ok(FuelGauge::relative_state_of_charge(self)? as Percent)
    }

    async fn absolute_state_of_charge(&mut self) -> Result<Percent, Self::Error> {
        Ok(FuelGauge::absolute_state_of_charge(self)? as Percent)
    }

    async fn remaining_capacity(&mut self) -> Result<CapacityModeValue, Self::Error> {
        let uAh: u32 = FuelGauge::remaining_capacity(self)?; // Returned in uAh. Apparently, it's always uAh regardless of the CAPACITY_MODE bit, according to the Zephyr docs.
        let capacity_mode: bool = self.battery_mode().await?.capacity_mode();

        // We need to convert to mAh (from uAh) according to the trait method requirement.
        let mAh: u32 = uAh / 1000;

        // We also need to convert from `u32` to `u16`. In case we run into overflow, print an error.
        let mAh: u16 = u16::try_from(mAh).map_err(|_| {
            log::error!("mAh out of u16 range: {}mAh! Returning an error.", mAh);
            crate::error::Error(crate::raw::EINVAL)
        })?;

        // When true, the capacity information should be reported in 10mW or 10mWh as appropriate. (centiwatt == 10mW)
        // When false, the capacity information should be reported in mA or mAh as appropriate.
        match capacity_mode {
            // If false, the returned value is expected to be in mAh, so we don't need to do any conversions.
            false => Ok(CapacityModeValue::MilliAmpUnsigned(mAh)),

            // If true, the returned value is expected to be in cWh, so we need to convert from mAh to cWh.
            true => {
                // cWh = (mAh × uV) / 10_000_000
                let uV: i32 = FuelGauge::voltage(self)?; // Returned in uV 
                let cWh = (mAh as i64 * uV as i64) / 10_000_000;
                let cWh: u16 = u16::try_from(cWh).map_err(|_| {
                    log::error!("cWh out of u16 range: {}cWh! Returning an error.", cWh);
                    crate::error::Error(crate::raw::EINVAL)
                })?;

                Ok(CapacityModeValue::CentiWattUnsigned(cWh))
            },
        }
    }

    async fn full_charge_capacity(&mut self) -> Result<CapacityModeValue, Self::Error> {
        let uAh: u32 = FuelGauge::full_charge_capacity(self)?; // Returned in uAh. Apparently, it's always uAh regardless of the CAPACITY_MODE bit, according to the Zephyr docs.
        let capacity_mode: bool = self.battery_mode().await?.capacity_mode();

        // We need to convert to mAh (from uAh) according to the trait method requirement.
        let mAh: u32 = uAh / 1000;

        // We also need to convert from `u32` to `u16`. In case we run into overflow, print an error.
        let mAh: u16 = u16::try_from(mAh).map_err(|_| {
            log::error!("mAh out of u16 range: {}mAh! Returning an error.", mAh);
            crate::error::Error(crate::raw::EINVAL)
        })?;

        // When true, the capacity information should be reported in 10mW or 10mWh as appropriate. (centiwatt == 10mW)
        // When false, the capacity information should be reported in mA or mAh as appropriate.
        match capacity_mode {
            // If false, the returned value is expected to be in mAh, so we don't need to do any conversions.
            false => Ok(CapacityModeValue::MilliAmpUnsigned(mAh)),

            // If true, the returned value is expected to be in cWh, so we need to convert from mAh to cWh.
            true => {
                // cWh = (mAh × uV) / 10_000_000
                let uV: i32 = FuelGauge::voltage(self)?; // Returned in uV 
                let cWh = (mAh as i64 * uV as i64) / 10_000_000;
                let cWh: u16 = u16::try_from(cWh).map_err(|_| {
                    log::error!("cWh out of u16 range: {}cWh! Returning an error.", cWh);
                    crate::error::Error(crate::raw::EINVAL)
                })?;

                Ok(CapacityModeValue::CentiWattUnsigned(cWh))
            },
        }
    }

    async fn run_time_to_empty(&mut self) -> Result<Minutes, Self::Error> {
        let minutes: u32 = self.runtime_to_empty()?;
        let minutes: u16 = u16::try_from(minutes).map_err(|_| {
            log::error!("minutes out of u16 range: {} minutes! Returning an error.", minutes);
            crate::error::Error(crate::raw::EINVAL)
        })?;
        Ok(minutes as Minutes)
    }

    async fn average_time_to_empty(&mut self) -> Result<Minutes, Self::Error> {
        Err(crate::error::Error(crate::raw::ENOTSUP)) // u_TODO: Zephyr API doesn't give us this as far as I can tell.
    }

    async fn charging_current(&mut self) -> Result<MilliAmps, Self::Error> {
        let uA: u32 = self.chg_current()?;
        let mA: u32 = uA / 1000;
        let mA: u16 = u16::try_from(mA).map_err(|_| {
            log::error!("mA out of u16 range: {} mA! Returning an error.", mA);
            crate::error::Error(crate::raw::EINVAL)
        })?;
        Ok(mA as MilliAmps)
    }

    async fn charging_voltage(&mut self) -> Result<MilliVolts, Self::Error> {
        let uV: u32 = self.chg_voltage()?;
        let mV: u32 = uV / 1000;
        let mV: u16 = u16::try_from(mV).map_err(|_| {
            log::error!("mV out of u16 range: {} mV! Returning an error.", mV);
            crate::error::Error(crate::raw::EINVAL)
        })?;
        Ok(mV as MilliVolts)
    }

    async fn battery_status(&mut self) -> Result<BatteryStatusField, Self::Error> {
        Ok(BatteryStatusFields::from_bits(self.fg_status()?))
    }

    async fn cycle_count(&mut self) -> Result<Cycles, Self::Error> {
        let cycles: u32 = FuelGauge::cycle_count(self)?;
        let cycles: u16 = u16::try_from(cycles).map_err(|_| {
            log::error!("cycles out of u16 range: {} cycles! Returning an error.", cycles);
            crate::error::Error(crate::raw::EINVAL)
        })?;
        Ok(cycles as Cycles)
    }

    async fn design_capacity(&mut self) -> Result<CapacityModeValue, Self::Error> {
        let value: u16 = self.design_cap()?; // Returned in either mAh or 10mWh depending on the capacity_mode, according to Zephyr docs.
        let capacity_mode: bool = self.battery_mode().await?.capacity_mode();

        // When true, the capacity information should be reported in 10mW or 10mWh as appropriate.
        // When false, the capacity information should be reported in mA or mAh as appropriate.
        match capacity_mode {
            true => Ok(CapacityModeValue::CentiWattUnsigned(value)), // centiwatt == 10mW
            false => Ok(CapacityModeValue::MilliAmpUnsigned(value)),
        }
    }

    async fn design_voltage(&mut self) -> Result<MilliVolts, Self::Error> {
        let mV: u16 = self.design_volt()?; // Returned in mV if you can believe it
        Ok(mV as MilliVolts)
    }

    async fn specification_info(&mut self) -> Result<SpecificationInfoFields, Self::Error> {
        Err(crate::error::Error(crate::raw::ENOTSUP)) // u_TODO: Zephyr API doesn't give us this as far as I can tell.
    }

    async fn manufacture_date(&mut self) -> Result<ManufactureDate, Self::Error> {
        Err(crate::error::Error(crate::raw::ENOTSUP)) // u_TODO: Zephyr API doesn't give us this as far as I can tell.
    }

    async fn serial_number(&mut self) -> Result<u16, Self::Error> {
        Err(crate::error::Error(crate::raw::ENOTSUP)) // u_TODO: Zephyr API doesn't give us this as far as I can tell.
    }

    async fn manufacturer_name(&mut self, name: &mut [u8]) -> Result<(), Self::Error> {
        FuelGauge::manufacturer_name(self, name)
    }

    async fn device_name(&mut self, name: &mut [u8]) -> Result<(), Self::Error> {
        FuelGauge::device_name(self, name)
    }

    async fn device_chemistry(&mut self, name: &mut [u8]) -> Result<(), Self::Error> {
        FuelGauge::device_chemistry(self, name)
    }
}

impl embedded_batteries_async::smart_battery::ErrorType for FuelGauge {
    type Error = crate::error::Error;
}

impl embedded_batteries_async::smart_battery::Error for crate::error::Error {
    // u_Note: eventually add better mapping to this
    fn kind(&self) -> embedded_batteries_async::smart_battery::ErrorKind {
        embedded_batteries_async::smart_battery::ErrorKind::Other
    }
}