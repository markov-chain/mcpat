extern crate mcpat;

use mcpat::{Component, System};
use std::path::PathBuf;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

macro_rules! round(
    ($value:expr, $precision:expr) => ({
        let scale = 10f64.powi($precision);
        ($value * scale).round() / scale
    });
);

// https://github.com/copies/mcpat/blob/master/ExampleResults/Xeon
#[test]
fn workflow() {
    initialize();

    let path = PathBuf::from("tests/fixtures/Xeon.xml");
    let system = ok!(System::open(&path));
    let processor = ok!(system.processor());

    {
        let mut cores = processor.cores();
        assert_eq!(cores.len(), 1);

        let core = cores.next().unwrap();
        let power = core.power();
        assert_eq!(round!(power.dynamic, 4), 39.2989);
        assert_eq!(round!(power.leakage, 3), round!(12.0565 + 0.74513, 3));
    }

    {
        let mut l3s = processor.l3s();
        assert_eq!(l3s.len(), 1);

        let l3 = l3s.next().unwrap();
        let power = l3.power();
        assert_eq!(round!(power.dynamic, 5), 6.70159);
        assert_eq!(round!(power.leakage, 3), round!(10.9824 + 0.165767, 3));
    }

    deinitialize();
}

#[cfg(not(feature = "caching"))]
fn initialize() {
    mcpat::set_optimzed_for_clock_rate(true);
}

#[cfg(not(feature = "caching"))]
fn deinitialize() {
}

#[cfg(feature = "caching")]
fn initialize() {
    mcpat::set_optimzed_for_clock_rate(true);
    ok!(mcpat::caching::activate("127.0.0.1", 6379));
}

#[cfg(feature = "caching")]
fn deinitialize() {
    mcpat::caching::deactivate();
}
