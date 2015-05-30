extern crate mcpat;

use mcpat::System;
use std::path::PathBuf;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn workflow() {
    let path = PathBuf::from("tests/fixtures/Xeon.xml");
    let _system = ok!(System::new(&path));
}
