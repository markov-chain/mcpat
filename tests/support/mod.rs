use mcpat;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

macro_rules! round(
    ($value:expr, $precision:expr) => ({
        let scale = 10f64.powi($precision);
        ($value * scale).round() / scale
    });
);

#[cfg(not(feature = "caching"))]
pub fn initialize() {
    mcpat::set_optimzed_for_clock_rate(true);
}

#[cfg(not(feature = "caching"))]
pub fn deinitialize() {
}

#[cfg(feature = "caching")]
pub fn initialize() {
    mcpat::set_optimzed_for_clock_rate(true);
    ok!(mcpat::caching::activate("127.0.0.1", 6379));
}

#[cfg(feature = "caching")]
pub fn deinitialize() {
    mcpat::caching::deactivate();
}
