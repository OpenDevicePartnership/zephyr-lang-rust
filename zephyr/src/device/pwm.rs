//! Device wrapper for a PWM controller.
//!
//! This is a thin wrapper around the Zephyr `struct device` that represents a PWM controller (for
//! example an `nxp,sctimer-pwm` peripheral).  It exists mainly so that devicetree nodes describing
//! a PWM controller expose the raw device accessors (`get_instance_raw`/`get_static_raw`).  Those
//! accessors are referenced by other devices that point at a PWM controller through a `pwms`
//! phandle, such as [`crate::device::pwm_fan::PwmFan`].

use super::{NoStatic, Unique};
use crate::raw;

/// A PWM controller device.
pub struct Pwm {
    #[allow(dead_code)]
    device: *const raw::device,
}

impl Pwm {
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
}
