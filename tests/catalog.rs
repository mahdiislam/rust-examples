//! Checks every slide of the deck against what the slide says about it:
//! code that should run runs, code that should panic panics, and code that
//! should not compile is rejected by rustc with exactly the right error.

use std::collections::BTreeSet;
use std::panic;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Once;

use rust_examples::{Example, Kind, SECTIONS};

/// Every slide, with its "07.3"-style number.
fn slides() -> impl Iterator<Item = (String, &'static Example)> {
    SECTIONS.iter().flat_map(|section| {
        section
            .examples
            .iter()
            .enumerate()
            .map(move |(i, example)| (section.slide_id(i), example))
    })
}

/// Expected panics would otherwise fill the test output with messages.
fn quiet_panics() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| panic::set_hook(Box::new(|_| {})));
}

#[test]
fn the_catalog_matches_the_deck() {
    assert_eq!(SECTIONS.len(), 21);
    assert_eq!(slides().count(), 146);
    for (i, section) in SECTIONS.iter().enumerate() {
        assert_eq!(section.number, i + 1, "sections are numbered in order");
        assert!(!section.examples.is_empty());
    }

    let count = |f: fn(&Kind) -> bool| slides().filter(|(_, e)| f(&e.kind)).count();
    assert_eq!(count(|k| matches!(k, Kind::Runs(_))), 122);
    assert_eq!(count(|k| matches!(k, Kind::Panics(_))), 5);
    assert_eq!(count(|k| matches!(k, Kind::DoesNotCompile { .. })), 17);
    assert_eq!(count(|k| matches!(k, Kind::Notes(_))), 2);
}

#[test]
fn every_runnable_slide_runs_and_its_checks_pass() {
    quiet_panics();
    let failures: Vec<String> = slides()
        .filter_map(|(id, example)| match example.kind {
            Kind::Runs(run) => panic::catch_unwind(run)
                .is_err()
                .then(|| format!("{id} {}", example.title)),
            _ => None,
        })
        .collect();
    assert!(failures.is_empty(), "these slides panicked: {failures:#?}");
}

#[test]
fn every_panicking_slide_panics() {
    quiet_panics();
    let failures: Vec<String> = slides()
        .filter_map(|(id, example)| match example.kind {
            Kind::Panics(run) => panic::catch_unwind(run)
                .is_ok()
                .then(|| format!("{id} {}", example.title)),
            _ => None,
        })
        .collect();
    assert!(
        failures.is_empty(),
        "these slides did not panic: {failures:#?}"
    );
}

#[test]
fn every_non_compiling_slide_fails_with_its_error_code() {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let out_dir = std::env::temp_dir().join(format!("rust-examples-{}", std::process::id()));
    std::fs::create_dir_all(&out_dir).unwrap();

    let mut failures = Vec::new();
    for (id, example) in slides() {
        let Kind::DoesNotCompile { file, error, .. } = example.kind else {
            continue;
        };
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(file);
        let output = Command::new(&rustc)
            .args([
                "--edition",
                "2024",
                "--crate-type",
                "bin",
                "--emit=metadata",
            ])
            .arg("--out-dir")
            .arg(&out_dir)
            .arg(&path)
            .output()
            .unwrap_or_else(|e| panic!("could not run {rustc}: {e}"));

        let stderr = String::from_utf8_lossy(&output.stderr);
        let codes: BTreeSet<&str> = stderr
            .match_indices("error[E")
            .map(|(at, _)| &stderr[at + 6..at + 11])
            .collect();
        let expected = BTreeSet::from([error]);

        if output.status.success() {
            failures.push(format!("{id} {file}: compiled, but should not"));
        } else if codes != expected {
            failures.push(format!(
                "{id} {file}: expected {error}, rustc reported {codes:?}"
            ));
        }
    }
    let _ = std::fs::remove_dir_all(&out_dir);
    assert!(failures.is_empty(), "{failures:#?}");
}
