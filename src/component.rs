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
    fn power(&self) -> Power;
}
