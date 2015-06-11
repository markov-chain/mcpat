extern crate fixture;
extern crate mcpat;

use mcpat::{Component, System};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

#[macro_use]
mod support;

#[test]
fn blackscholes() {
    support::initialize();

    macro_rules! collect(
        ($components:expr) => ({
            let mut data = vec![];
            for component in $components {
                data.push(1e6 * component.area());
                data.push(component.dynamic_power());
                data.push(component.leakage_power());
            }
            data
        });
    );

    for xml in find("xml").iter() {
        println!("Processing {:?}...", ok!(xml.file_name()));

        let system = ok!(System::open(xml));
        let processor = ok!(system.compute());

        let cores = collect!(processor.cores());
        let l3s = collect!(processor.l3s());
        let (expected_cores, expected_l3s) = parse(&PathBuf::from(xml).with_extension("txt"));

        compare(&cores, &expected_cores);
        compare(&l3s, &expected_l3s);
    }

    support::deinitialize();
}

fn find(extension: &str) -> Vec<PathBuf> {
    let path = PathBuf::from("tests/fixtures/sniper-parsec-blackscholes");
    fixture::find::all_with_extension(&path, extension)
}

fn parse(path: &Path) -> (Vec<f64>, Vec<f64>) {
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
            "Core:" => for quantity in read(&mut lines) {
                cores.push(quantity);
            },
            "L3" => for quantity in read(&mut lines) {
                l3s.push(quantity);
            },
            _ => {},
        }
    }

    (cores, l3s)
}

fn read<B: BufRead>(lines: &mut Lines<B>) -> Vec<f64> {
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

    vec![ok!(area), ok!(dynamic), ok!(subthreshold) + ok!(gate)]
}

fn compare(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected) {
        equal(actual, expected);
    }
}

fn equal(actual: f64, expected: f64) {
    let mut scale = 1.0;
    for i in 0..16 {
        scale = 10f64.powi(i);
        if (expected - (expected * scale).round() / scale).abs() < 1e-8 {
            break;
        }
    }
    assert_eq!((actual * scale).round() / scale, (expected * scale).round() / scale);
}
