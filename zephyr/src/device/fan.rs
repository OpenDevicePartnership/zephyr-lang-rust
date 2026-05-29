//! Device wrapper for a PWM fan.

use crate::device::{NoStatic, Unique};
use crate::raw;
use log::info;

/// A PWM-controlled fan.
/// (This is a wrapper around the `struct device` in Zephyr that represents a PWM controller.)
/// 
/// # Devicetree
/// 
/// In the devicetree, use compatible `"pwm-fan"` with properties:
/// - `pwms`: PWM controller reference with channel, period, flags
/// - `max-rpm`: The maximum RPM the fan is capable of running at.
/// - `min-rpm`: The minimum RPM the fan is capable of running at.
/// - `min-start-rpm`: The minimum RPM needed for the fan to begin running from a dead stop.
///
/// Example:
/// 
/// 1. Configure a `pwm-fan` in the devicetree
/// ```dts
/// fan0: my-fan {
///     compatible = "pwm-fan";
///     pwms = <&pwm0 0 25000 0>;
///     max-rpm = <5000>;
///     min-rpm = <1000>;
///     min-start-rpm = <1500>;
/// };
/// ```
/// 2. Then, retrieve an instance of the object in your Rust code:
/// ```rust
/// zephyr::devicetree::my_fan::get_instance().unwrap()
/// ```
/// 
pub struct Fan {
    device: *const crate::raw::device,
    channel: u32,
    period: u32,
    flags: u16,
    max_rpm: u16,
    min_rpm: u16,
    min_start_rpm: u16,
}

impl Fan {
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
    ) -> Option<Fan> {
        // Make sure this instance doesn't already exist.
        if !unique.once() { return None; }

        // Cast flags and max_rpm to u16, or return None if they don't fit.
        use core::convert::TryInto;
        let flags: u16 = flags.try_into().ok()?;
        let max_rpm: u16 = max_rpm.try_into().ok()?;
        let min_rpm: u16 = min_rpm.try_into().ok()?;
        let min_start_rpm: u16 = min_start_rpm.try_into().ok()?;

        Some(Fan {device, channel, period, flags, max_rpm, min_rpm})
    }
}

// u_Note: eventually should probably decide if the embedded_xxx implementations should live directly in the Zephyr driver wrapper layer (i.e., here) or if they should only live in the app-layer code
impl embedded_fans_async::Fan for Fan {
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
        let duty_cycle: f32 = (rpm as f32 / self.max_rpm as f32);
        let duty_cycle: f32 = duty_cycle.clamp(0.0, 1.0); // u_Note: maybe we should print a warning or something if the duty cycle gets clamped?

        // Convert duty cycle to pulse
        let pulse: u32 = (duty_cycle * self.period as f32) as u32;

        let ret: i32 = unsafe {
            crate::raw::pwm_set_cycles(
                self.device,
                self.channel,
                self.period,
                pulse,
                self.flags,
            )
        };

        // Check for errors.
        if let Err(e) = crate::error::to_result_void(ret) {
            log::error!("pwm_set_cycles failed: {}", e);
            return Err(e);
        }

        // If pwm_set_cycles() was successful, calculate the actual RPM sent via the pulse
        let actual_duty_cycle: f32 = (pulse as f32) / (self.period as f32);
        let actual_rpm: u16 = (actual_duty_cycle * self.max_rpm as f32) as u16;

        Ok(actual_rpm)
    }
}