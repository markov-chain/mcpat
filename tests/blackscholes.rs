extern crate fixture;
extern crate mcpat;

use mcpat::{Component, Power, Spec};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

#[macro_use]
mod support;

#[test]
fn blackscholes() {
    support::initialize();

    for xml in find("xml").iter() {
        println!("Processing {:?}...", ok!(xml.file_name()));

        let spec = ok!(Spec::open(xml));
        let processor = ok!(spec.processor());
        let cores = processor.cores().map(|c| (c.area() * 1e-6, c.power())).collect::<Vec<_>>();
        let l3s = processor.l3s().map(|c| (c.area() * 1e-6, c.power())).collect::<Vec<_>>();

        let txt = &PathBuf::from(xml).with_extension("txt");
        assert!(fixture::exists(txt));
        let (expected_cores, expected_l3s) = parse(txt);

        compare(&cores, &expected_cores);
        compare(&l3s, &expected_l3s);
    }

    support::deinitialize();
}

fn find(extension: &str) -> Vec<PathBuf> {
    let path = PathBuf::from("tests/fixtures/sniper-parsec-blackscholes");
    fixture::find::all_with_extension(&path, extension)
}

fn parse(path: &Path) -> (Vec<(f64, Power)>, Vec<(f64, Power)>) {
    let mut file = ok!(File::open(path));
    let reader = BufReader::new(&mut file);
    let mut lines = reader.lines();

    let mut cores = Vec::new();
    let mut l3s = Vec::new();

    loop {
        let line = match lines.next() {
            Some(line) => ok!(line),
            _ => break,
        };
        match line.trim() {
            "Core:" => cores.push(read(&mut lines)),
            "L3" => l3s.push(read(&mut lines)),
            _ => {},
        }
    }

    (cores, l3s)
}

fn read<B: BufRead>(lines: &mut Lines<B>) -> (f64, Power) {
    let (mut area, mut dynamic, mut subthreshold, mut gate) = (None, None, None, None);

    fn extract_value(line: &str) -> &str {
        let line = line.trim();
        let k = ok!(line.find('='));
        &line[(k + 2)..line.rfind(' ').unwrap()]
    }

    while dynamic.is_none() || subthreshold.is_none() || gate.is_none() {
        let line = &ok!(ok!(lines.next()));
        if line.contains("Area") {
            area = Some(ok!(extract_value(line).parse::<f64>()));
        } else if line.contains("Runtime Dynamic") {
            dynamic = Some(ok!(extract_value(line).parse::<f64>()));
        } else if line.contains("Subthreshold Leakage with power gating") {
            subthreshold = Some(ok!(extract_value(line).parse::<f64>()));
        } else if line.contains("Gate Leakage") {
            gate = Some(ok!(extract_value(line).parse::<f64>()));
        }
    }

    (ok!(area), Power { dynamic: ok!(dynamic), leakage: ok!(subthreshold) + ok!(gate) })
}

fn compare(actual: &[(f64, Power)], expected: &[(f64, Power)]) {
    assert_eq!(actual.len(), expected.len());

    for (actual, expected) in actual.iter().zip(expected) {
        equal(actual.0, expected.0);
        equal(actual.1.dynamic, expected.1.dynamic);
        equal(actual.1.leakage, expected.1.leakage);
    }
}

fn equal(actual: f64, expected: f64) {
    let string = format!("{}", expected);
    let precision = match string.find('.') {
        Some(k) => string.len() - k - 1,
        _ => 1,
    };
    let scale = 10f64.powi(precision as i32);
    assert_eq!((actual * scale).round() / scale, (expected * scale).round() / scale);
}
