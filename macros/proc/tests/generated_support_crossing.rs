//! A generated mutation carrier invoked from a real downstream crate.
//!
//! The producer library and its integration test must be separate crates because an exported carrier is intentionally consumed downstream, after the producer has defined it.
//! This observer builds that two-crate boundary under Rust 1.98 with warnings denied and safe Rust required.

#[path = "support/scratch.rs"]
mod scratch;

use scratch::{cargo, command_refusal, manifest_path, observed_in_scratch_for, repository_root};
use std::path::Path;

/// Write the fixed producer library and downstream consumer.
fn write_specimen(scratch: &Path) -> Result<(), String> {
    let repository = repository_root()?;
    let proc_package = manifest_path(&repository.join("macros/proc"))?;
    let harness_package = manifest_path(&repository.join("harness"))?;
    std::fs::create_dir(scratch.join("src")).map_err(|error| error.to_string())?;
    std::fs::create_dir(scratch.join("tests")).map_err(|error| error.to_string())?;
    let manifest = format!(
        r#"[package]
name = "macroonz-generated-support-observer"
version = "0.0.0"
edition = "2024"
rust-version = "1.98.0"
publish = false
autobins = false
autoexamples = false
autotests = false
autobenches = false
build = false

[lib]
path = "src/lib.rs"

[[test]]
name = "crossing"
path = "tests/crossing.rs"

[dependencies]
macroonz-macros = {{ path = "{proc_package}" }}

[dev-dependencies]
macroonz-harness = {{ path = "{harness_package}" }}

[lints.rust]
warnings = "deny"
unsafe_code = "forbid"

[workspace]
"#
    );
    std::fs::write(scratch.join("Cargo.toml"), manifest).map_err(|error| error.to_string())?;
    std::fs::write(scratch.join("src/lib.rs"), PRODUCER).map_err(|error| error.to_string())?;
    std::fs::write(scratch.join("tests/crossing.rs"), CONSUMER).map_err(|error| error.to_string())
}

/// TOML path spelling preserves host separators and escapes syntax and control characters without lossy substitution.
#[test]
fn manifest_paths_are_exact_toml_basic_string_bodies() {
    let spelling = "plain\\slash\"quote\nline\tcell\u{0001}\u{007f}";
    assert_eq!(
        manifest_path(Path::new(spelling)),
        Ok(r#"plain\\slash\"quote\nline\tcell\u0001\u007F"#.to_owned())
    );
}

/// The proc package's scratch, Cargo, escaping, and refusal mechanics have exactly one support owner, while each lane keeps its own specimen writer.
#[test]
fn named_proc_test_support_debt_does_not_expand() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let owner = ["support/scratch.rs"];
    assert_source_occurrences(&root, concat!("fn ", "scratch_root("), &owner)?;
    assert_source_occurrences(&root, concat!("fn ", "observed_in_scratch_for("), &owner)?;
    assert_source_occurrences(&root, concat!("fn ", "cargo("), &owner)?;
    assert_source_occurrences(&root, concat!("fn ", "manifest_path("), &owner)?;
    assert_source_occurrences(&root, concat!("fn ", "command_refusal("), &owner)?;
    assert_source_occurrences(&root, concat!("fn ", "repository_root("), &owner)?;
    assert_source_occurrences(&root, concat!("static ", "SCRATCH_ORDINAL"), &owner)?;
    assert_source_occurrences(&root, concat!("fn ", "command_reading("), &[])?;
    assert_source_occurrences(&root, concat!(".replace('\\\\', ", "\"\\\\\\\\\")"), &[])?;
    assert_source_occurrences(&root, concat!("std::env::temp_", "dir()"), &[])?;
    assert_source_occurrences(
        &root,
        concat!("fn ", "write_specimen("),
        &[
            "advanced_rust_facade_crossing.rs",
            "generated_support_crossing.rs",
            "recipe_facade_crossing/support.rs",
        ],
    )
}

/// Assert the exact current file roster that defines one proc-test support operation.
fn assert_source_occurrences(root: &Path, needle: &str, expected: &[&str]) -> Result<(), String> {
    let mut observed = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).map_err(|error| error.to_string())? {
            let path = entry.map_err(|error| error.to_string())?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let source = std::fs::read_to_string(&path).map_err(|error| error.to_string())?;
                let relative = path
                    .strip_prefix(root)
                    .map_err(|error| error.to_string())?
                    .to_string_lossy()
                    .replace('\\', "/");
                observed.extend(
                    source
                        .match_indices(needle)
                        .map(|_occurrence| relative.clone()),
                );
            }
        }
    }
    observed.sort();
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(observed, expected, "{needle}");
    Ok(())
}

/// A downstream crate invokes the real carrier and compiles every generated mutation road under the strict wall.
#[test]
fn a_downstream_crate_invokes_the_proc_emitted_mutation_carrier() -> Result<(), String> {
    observed_in_scratch_for("generated_support", observe_crossing)
}

/// Build and execute the downstream crossing inside one exclusively owned scratch root.
fn observe_crossing(scratch: &Path) -> Result<(), String> {
    write_specimen(scratch)?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("scratch lock generation", &locked));
    }
    let tested = cargo(scratch, &["test", "--locked", "--offline"])?;
    if !tested.status.success() {
        return Err(command_refusal("downstream carrier qualification", &tested));
    }
    Ok(())
}

/// The producer library owns the declaration and exports its deferred carrier.
const PRODUCER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

/// The declared order this producer exposes to a downstream mutation harness.
#[macroonz_macros::mutations(
    module = pressed,
    refusal = PressRefusal,
    support = press_support,
    family = named("observer", "refusals"),
    point = named("observer", "press-point"),
    fact = named("observer", "cause-order"),
    map named("observer", "cause-order") = named("observer", "order-held"),
    permit named("observer", "order-held") = ["declared-order-permutation"],
)]
pub enum Cause<const N: usize = { 1 + 1 }> {
    /// The first cause.
    First,
    /// A raw-identifier cause whose field makes the const-generic parameter semantic.
    r#Second([u8; N]),
    /// The third cause.
    Third,
}
"#;

/// The consumer invokes the exported carrier, exercises its generated roads, and checks the complete candidate roster.
const CONSUMER: &str = r#"#![forbid(unsafe_code)]
#![deny(warnings)]

use macroonz_generated_support_observer::press_support;

press_support! {
    harness: macroonz_harness,
}

#[test]
fn the_generated_module_is_complete_and_callable() {
    assert_eq!(pressed::production(&()), ["First", "Second", "Third"]);
    assert_eq!(
        pressed::candidate_orders(),
        [["Second", "First", "Third"], ["First", "Third", "Second"]]
    );
    assert!(pressed::lowering().is_ok());
    let observed = pressed::evaluation(
        &(),
        macroonz_harness::muterprater::EvaluationDirective::no_mutation(),
    );
    assert!(observed.is_ok_and(|reading| {
        reading.meaning() == &["First", "Second", "Third"] && reading.firings() == 0
    }));
}
"#;
