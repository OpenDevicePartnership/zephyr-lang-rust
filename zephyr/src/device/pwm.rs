//! Device wrapper for a PWM controller.
//!

use super::{NoStatic, Unique};
use crate::raw;

/// A PWM controller device.
pub struct Pwm {
    device: *const raw::device,
}

impl Pwm {
    /// Constructor, used by the devicetree generated code.
    pub(crate) unsafe fn new(
        unique: &Unique,
        _static: &NoStatic,
        device: *const raw::device,
    ) -> Option<Pwm> {
        if !unique.once() {
            return None;
        }

        Some(Pwm { device })
    }

    /// Validate that the PWM device is ready.
    pub fn is_ready(&self, channel: u32, period: u32, flags: u16) -> bool {
        let spec = raw::pwm_dt_spec {
            dev: self.device,
            channel,
            period,
            flags,
        };
        unsafe { raw::pwm_is_ready_dt(&spec) }
    }

    /// Set the period and pulse width for a single PWM output.
    pub fn set_cycles(
        &self,
        channel: u32,
        period: u32,
        pulse: u32,
        flags: u16,
    ) -> crate::error::Result<()> {
        let ret = unsafe { raw::pwm_set_cycles(self.device, channel, period, pulse, flags) };
        crate::error::to_result_void(ret)
    }
}
