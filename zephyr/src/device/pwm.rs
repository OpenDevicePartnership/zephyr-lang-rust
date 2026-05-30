//! Device wrapper for a PWM controller. Only meant for use within this crate!
//!
//! This is a thin wrapper around the Zephyr `struct device` that represents a PWM controller (for
//! example an `nxp,sctimer-pwm` peripheral).  It exists mainly so that devicetree nodes describing
//! a PWM controller expose the raw device accessors (`get_instance_raw`/`get_static_raw`).  Those
//! accessors are referenced by other devices that point at a PWM controller through a `pwms`
//! phandle, such as [`crate::device::pwm_fan::PwmFan`].

use super::{NoStatic, Unique};

/// A PWM controller device.
pub(crate) struct Pwm {
    device: *const crate::raw::device,
}

impl Pwm {
    pub(crate) unsafe fn new(
        unique: &Unique,
        _static: &NoStatic,
        device: *const crate::raw::device,
    ) -> Option<Pwm> {
        if !unique.once() {
            return None;
        }
        Some(Pwm { device })
    }

    // Wrap a raw PWM controller device handle.  Used by consumers (e.g. a `pwm-fan`) that obtain
    // the controller handle through a `pwms` phandle and manage their own channel/period/flags.
    pub(crate) unsafe fn from_raw(device: *const crate::raw::device) -> Pwm {
        Pwm { device }
    }

    // Validate that the PWM device is ready.
    pub(crate) fn is_ready(&self, channel: u32, period: u32, flags: u16) -> bool {
        let spec = crate::raw::pwm_dt_spec {
            dev: self.device,
            channel,
            period,
            flags,
        };
        unsafe { crate::raw::pwm_is_ready_dt(&spec) }
    }

    // Set the period and pulse width for a single PWM output.
    pub(crate) fn set_cycles(
        &self,
        channel: u32,
        period: u32,
        pulse: u32,
        flags: u16,
    ) -> crate::error::Result<()> {
        let ret = unsafe { crate::raw::pwm_set_cycles(self.device, channel, period, pulse, flags) };
        crate::error::to_result_void(ret)
    }
}
