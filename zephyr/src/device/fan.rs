//! Device wrapper for a PWM fan.

use crate::device::{NoStatic, Unique};
use crate::raw;
use log::info;

/// A Fan object.
/// 
/// This is a wrapper around the `struct device` in Zephyr that represents a PWM controller.
pub struct Fan {
    device: *const raw::device,
}

// impl embedded_fans_async::Fan for Fan {
//     // Returns the maximum RPM a fan is capable of running at.
//     fn max_rpm(&self) -> u16 {

//     }
// }