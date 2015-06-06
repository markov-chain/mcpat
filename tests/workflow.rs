extern crate mcpat;

use mcpat::{Component, System};
use std::path::PathBuf;

#[macro_use]
mod support;

// https://github.com/copies/mcpat/blob/master/ExampleResults/Xeon
#[test]
fn workflow() {
    support::initialize();

    let path = PathBuf::from("tests/fixtures/Xeon.xml");
    let system = ok!(System::open(&path));
    let processor = ok!(system.processor());

    {
        let mut cores = processor.cores();
        assert_eq!(cores.len(), 1);

        let core = cores.next().unwrap();
        let power = core.power();
        assert_eq!(round!(power.dynamic, 4), 55.7891);
        assert_eq!(round!(power.leakage, 3), round!(5.15028 + 0.74513, 3));
    }

    {
        let mut l3s = processor.l3s();
        assert_eq!(l3s.len(), 1);

        let l3 = l3s.next().unwrap();
        let power = l3.power();
        assert_eq!(round!(power.dynamic, 5), 4.32382);
        assert_eq!(round!(power.leakage, 3), round!(6.06659 + 0.165767, 3));
    }

    support::deinitialize();
}
