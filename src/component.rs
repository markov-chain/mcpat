use raw;

/// Statistics about the power consumption of a component.
#[derive(Clone, Copy, Debug)]
pub struct Power {
    /// The dynamic power.
    pub dynamic: f64,
    /// The leakage power.
    pub leakage: f64,
}

/// A component.
pub trait Component {
    fn area(&self) -> f64;
    fn power(&self) -> Power;
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
