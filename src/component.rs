use raw;

/// A component of a system on a chip.
pub trait Component {
    /// Return the area (m<sup>2</sup>).
    fn area(&self) -> f64;
    /// Return the dynamic power (W).
    fn dynamic_power(&self) -> f64;
    /// Return the leakage power (W).
    fn leakage_power(&self) -> f64;
}

pub unsafe fn leakage(system: *mut raw::root_system,
                      components: *mut raw::powerComponents) -> f64 {

    let long_channel = (*system).longer_channel_device > 0;
    let power_gating = (*system).power_gating > 0;

    let subthreshold = if power_gating {
        if long_channel {
            raw::powerComponents_power_gated_with_long_channel_leakage(components)
        } else {
            raw::powerComponents_power_gated_leakage(components)
        }
    } else {
        if long_channel {
            raw::powerComponents_longer_channel_leakage(components)
        } else {
            raw::powerComponents_leakage(components)
        }
    };

    subthreshold + raw::powerComponents_gate_leakage(components)
}
