//! Device wrapper for a fuel gauge.

/// A fuel gauge device.
/// u_Note: make a good comment here eventually
pub struct FuelGauge {
    device: *const crate::raw::device,
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
