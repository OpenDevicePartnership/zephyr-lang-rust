//! Device wrapper for a PWM fan. See docs for struct `PwmFan` for usage information.

/// A PWM-controlled fan.
/// (This is a wrapper around the `struct device` in Zephyr that represents a PWM controller.)
/// 
/// # Using This Struct From The Devicetree:
/// 
/// In the devicetree, use compatible `"pwm-fan"` with properties:
/// - `pwms`: PWM controller reference with channel, period, flags
/// - `max-rpm`: The maximum RPM the fan is capable of running at.
/// - `min-rpm`: The minimum RPM the fan is capable of running at.
/// - `min-start-rpm`: The minimum RPM needed for the fan to begin running from a dead stop.
/// - `tachometer` (optional): Phandle to a tachometer (RPM sensor) device, if the fan has one.
///   Any Zephyr tachometer works (e.g. `zephyr,tach-gpio` or a vendor peripheral), since they all
///   report on the `SENSOR_CHAN_RPM` channel.
///
/// Example:
/// 
/// 1. Configure a `pwm-fan` in the devicetree:
/// ```dts
/// my_tach: tachometer {
///     compatible = "zephyr,tach-gpio";
///     gpios = <&gpio0 15 GPIO_ACTIVE_HIGH>;
///     pulses-per-round = <2>;
/// };
///
/// fan0: my-fan {
///     compatible = "pwm-fan";
///     pwms = <&pwm0 0 25000 0>;
///     max-rpm = <5000>;
///     min-rpm = <1000>;
///     min-start-rpm = <1500>;
///     tachometer = <&my_tach>;  // optional
/// };
/// ```
/// 2. Then, retrieve an instance of the object in your Rust code:
/// ```rust
/// zephyr::devicetree::my_fan::get_instance().unwrap()
/// ```
/// 
pub struct PwmFan {
    // The PWM controller this fan is driven by, plus the output selection from the `pwms` phandle.
    device: *const crate::raw::device,
    channel: u32,
    period: u32,
    flags: u16,

    // Required devicetree properties
    max_rpm: u16,
    min_rpm: u16,
    min_start_rpm: u16,

    // Optional devicetree properties
    tachometer: Option<crate::device::tachometer::Tachometer>,

    // Internal data (not relavent to the devicetree)
    last_set_rpm: u16,
}

impl PwmFan {
    pub(crate) unsafe fn new(
        unique: &crate::device::Unique,
        _static: &crate::device::NoStatic,
        device: *const crate::raw::device,
        _device_static: &'static crate::device::NoStatic,

        channel: u32,
        period: u32,
        flags: u32,

        max_rpm: u32,
        min_rpm: u32,
        min_start_rpm: u32,
        tachometer: Option<crate::device::tachometer::Tachometer>,
    ) -> Option<PwmFan> {
        // Make sure this instance doesn't already exist.
        if !unique.once() { return None; }

        // Cast flags and max_rpm to u16, or return None if they don't fit.
        use core::convert::TryInto;
        let flags: u16 = flags.try_into().ok()?;
        let max_rpm: u16 = max_rpm.try_into().ok()?;
        let min_rpm: u16 = min_rpm.try_into().ok()?;
        let min_start_rpm: u16 = min_start_rpm.try_into().ok()?;

        // Make sure PWM Controller is ready
        let spec = crate::raw::pwm_dt_spec { dev: device, channel, period, flags };
        if !crate::raw::pwm_is_ready_dt(&spec) { return None; }

        Some(PwmFan {device, channel, period, flags, max_rpm, min_rpm, min_start_rpm, tachometer, last_set_rpm: 0})
    }
}

// u_Note: eventually should probably decide if the embedded_xxx implementations should live directly in the Zephyr driver wrapper layer (i.e., here) or if they should only live in the app-layer code
impl embedded_fans_async::Fan for PwmFan {
    // Returns the maximum RPM a fan is capable of running at.
    fn max_rpm(&self) -> u16 {
        return self.max_rpm;
    }

    // Returns the minimum RPM a fan is capable of running at.
    fn min_rpm(&self) -> u16 {
        return self.min_rpm;
    }

    // Returns the minimum RPM a fan is capable of running at.
    fn min_start_rpm(&self) -> u16 {
        return self.min_start_rpm;
    }

    // Sets the fan's speed in terms of absolute RPM. Returns the actual RPM set on success.
    async fn set_speed_rpm(&mut self, rpm: u16) -> Result<u16, Self::Error> {
        // Calculate Duty Cycle:
        // Duty Cycle (%) = (Desired RPM/Maximum RPM);
        let duty_cycle: f32 = (rpm as f32) / (self.max_rpm as f32);
        let duty_cycle: f32 = duty_cycle.clamp(0.0, 1.0); // u_Note: maybe we should print a warning or something if the duty cycle gets clamped?

        // Convert duty cycle to pulse
        let pulse: u32 = (duty_cycle * self.period as f32) as u32;

        // Check for errors.
        let ret = unsafe {
            crate::raw::pwm_set_cycles(self.device, self.channel, self.period, pulse, self.flags)
        };
        if let Err(e) = crate::error::to_result_void(ret) {
            match e.0 {
                crate::raw::EINVAL => {
                    log::error!("pwm_set_cycles() returned with Zephyr error status {} (EINVAL), meaning pulse > period.",e);
                    return Err(embedded_fans_async::ErrorKind::InvalidSpeed);
                }
                _ => {
                    log::error!("pwm_set_cycles() returned with Zephyr error status {}.", e);
                    return Err(embedded_fans_async::ErrorKind::Other);
                }
            }
        }

        // If pwm_set_cycles() was successful, calculate the actual RPM sent via the pulse
        let actual_duty_cycle: f32 = (pulse as f32) / (self.period as f32);
        let actual_rpm: u16 = (actual_duty_cycle * self.max_rpm as f32) as u16;

        self.last_set_rpm = actual_rpm;
        log::info!("Set RPM speed. (Requested RPM={}, RPM That Actually Got Set={}, Calculated Pulse={}, Calculated Duty Cycle={}, Actual Duty Cycle={})", rpm, actual_rpm, pulse, duty_cycle, actual_duty_cycle);

        {
            // u_Note: this is temporary for debugging, remove eventually
            use embedded_fans_async::RpmSense;
            if let Ok(tach_rpm) = self.rpm().await { log::info!("Tachometer reads {} RPM", tach_rpm); }
        }
        Ok(actual_rpm)
    }
}

impl embedded_fans_async::RpmSense for PwmFan {
    // Returns the fan's current RPM. 
    // If a tachometer device is configured, this method reads the actual RPM from it.
    // If no tachometer is configured, this method returns the last manually-set RPM.
    async fn rpm(&mut self) -> Result<u16, Self::Error> {
        match &self.tachometer {
            Some(tachometer) => {
                // Read the actual RPM from the tachometer sensor (SENSOR_CHAN_RPM).
                match tachometer.read_rpm() {
                    Ok(rpm) => {
                        let rpm_clamped: i32 = rpm.clamp(0, u16::MAX as i32);
                        log::info!("RPM read from tachometer (rpm={}, rpm_clamped={})", rpm, rpm_clamped);
                        return Ok(rpm.clamp(0, u16::MAX as i32) as u16);
                    }
                    Err(e) => {
                        log::error!("Failed to read tachometer RPM, Zephyr error status {}.", e);
                        return Err(embedded_fans_async::ErrorKind::Other);
                    }
                }
            }
            None => Ok(self.last_set_rpm),
        }
    }
}

// Error type.
impl embedded_fans_async::ErrorType for PwmFan {
    type Error = embedded_fans_async::ErrorKind;
}

// Mark this driver as compatible with thermal_service_interface
impl thermal_service_interface::fan::Driver for PwmFan {}