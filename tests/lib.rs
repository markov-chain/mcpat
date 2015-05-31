extern crate mcpat;

use mcpat::System;
use std::path::PathBuf;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn workflow() {
    let path = PathBuf::from("tests/fixtures/Xeon.xml");
    let system = ok!(System::new(&path));
    let processor = ok!(system.processor());

    let mut cores = processor.cores();
    assert_eq!(cores.len(), 1);

    let _core = cores.next().unwrap();
}
