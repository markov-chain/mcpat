extern crate mcpat;

use mcpat::{System, Processor};
use std::path::PathBuf;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn workflow() {
    let path = PathBuf::from("tests/fixtures/Xeon.xml");
    let system = ok!(System::new(&path));
    let _processor = ok!(Processor::new(&system));
}
