extern crate fixture;
extern crate mcpat;

use mcpat::{Component, Power, System};
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

        let system = ok!(System::open(xml));
        let processor = ok!(system.processor());
        let (cores, l3s) = (processor.cores().map(|c| c.power()).collect::<Vec<_>>(),
                            processor.l3s().map(|c| c.power()).collect::<Vec<_>>());

        let txt = &PathBuf::from(xml).with_extension("txt");
        assert!(fixture::exists(txt));
        let (expected_cores, expected_l3s) = parse_results(txt);

        compare(&cores, &expected_cores);
        compare(&l3s, &expected_l3s);
    }

    support::deinitialize();
}

fn find(extension: &str) -> Vec<PathBuf> {
    let path = PathBuf::from("tests/fixtures/sniper-parsec-blackscholes");
    fixture::find::all_with_extension(&path, extension)
}

fn parse_results(path: &Path) -> (Vec<Power>, Vec<Power>) {
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
            "Core:" => cores.push(read_power(&mut lines)),
            "L3" => l3s.push(read_power(&mut lines)),
            _ => {},
        }
    }

    (cores, l3s)
}

fn read_power<B: BufRead>(lines: &mut Lines<B>) -> Power {
    let (mut dynamic, mut subthreshold, mut gate) = (None, None, None);

    fn extract_value(line: &str) -> &str {
        let line = line.trim();
        let k = ok!(line.find('='));
        &line[(k + 2)..(line.len() - 2)]
    }

    while dynamic.is_none() || subthreshold.is_none() || gate.is_none() {
        let line = &ok!(ok!(lines.next()));
        if line.contains("Runtime Dynamic") {
            dynamic = Some(ok!(extract_value(line).parse::<f64>()));
        } else if line.contains("Subthreshold Leakage with power gating") {
            subthreshold = Some(ok!(extract_value(line).parse::<f64>()));
        } else if line.contains("Gate Leakage") {
            gate = Some(ok!(extract_value(line).parse::<f64>()));
        }
    }

    Power { dynamic: ok!(dynamic), leakage: ok!(subthreshold) + ok!(gate) }
}

fn compare(actual: &[Power], expected: &[Power]) {
    assert_eq!(actual.len(), expected.len());

    for (actual, expected) in actual.iter().zip(expected) {
        equal(actual.dynamic, expected.dynamic);
        equal(actual.leakage, expected.leakage);
    }
}

fn equal(actual: f64, expected: f64) {
    let string = format!("{}", expected);
    let precision = string.len() - ok!(string.find('.')) - 1;
    let scale = 10f64.powi(precision as i32);
    assert_eq!((actual * scale).round() / scale, (expected * scale).round() / scale);
}
