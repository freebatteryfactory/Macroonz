//! A generated mutation carrier invoked from a real downstream crate.
//!
//! The producer library and its integration test must be separate crates because an exported carrier is intentionally consumed downstream, after the producer has defined it.
//! This observer builds that two-crate boundary under Rust 1.98 with warnings denied and safe Rust required.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

static SCRATCH_ORDINAL: AtomicU32 = AtomicU32::new(0);

/// Atomically claim one empty scratch root owned by this process.
fn scratch_root() -> Result<PathBuf, String> {
    let parent = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    for _attempt in 0u16..1_024u16 {
        let ordinal = SCRATCH_ORDINAL.fetch_add(1, Ordering::SeqCst);
        let candidate = parent.join(format!(
            "macroonz_generated_support_{}_{ordinal}",
            std::process::id()
        ));
        match std::fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    Err("no unoccupied generated-support scratch seat remained".to_owned())
}

/// One exact UTF-8 path spelling escaped as the body of a TOML basic string.
fn manifest_path(path: &Path) -> Result<String, String> {
    let spelling = path
        .to_str()
        .ok_or_else(|| format!("package path is not UTF-8: {}", path.display()))?;
    let mut escaped = String::new();
    for character in spelling.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\u{0008}' => escaped.push_str("\\b"),
            '\t' => escaped.push_str("\\t"),
            '\n' => escaped.push_str("\\n"),
            '\u{000c}' => escaped.push_str("\\f"),
            '\r' => escaped.push_str("\\r"),
            '\u{0000}'..='\u{001f}' | '\u{007f}' => {
                push_toml_unicode_escape(character, &mut escaped)?;
            }
            _ => escaped.push(character),
        }
    }
    Ok(escaped)
}

/// Append one four-digit TOML Unicode escape for a control character.
fn push_toml_unicode_escape(character: char, into: &mut String) -> Result<(), String> {
    let code = u32::from(character);
    into.push_str("\\u");
    for shift in [12_u32, 8_u32, 4_u32, 0_u32] {
        into.push(hexadecimal_digit((code >> shift) & 0x0f)?);
    }
    Ok(())
}

/// Render one four-bit value without a fallible formatting road.
fn hexadecimal_digit(value: u32) -> Result<char, String> {
    match value {
        0 => Ok('0'),
        1 => Ok('1'),
        2 => Ok('2'),
        3 => Ok('3'),
        4 => Ok('4'),
        5 => Ok('5'),
        6 => Ok('6'),
        7 => Ok('7'),
        8 => Ok('8'),
        9 => Ok('9'),
        10 => Ok('A'),
        11 => Ok('B'),
        12 => Ok('C'),
        13 => Ok('D'),
        14 => Ok('E'),
        15 => Ok('F'),
        _ => Err(format!("{value} is not a four-bit value")),
    }
}

/// Run one Cargo command against the scratch package.
fn cargo(scratch: &Path, arguments: &[&str]) -> Result<Output, String> {
    Command::new("cargo")
        .arg("+1.98.0")
        .args(arguments)
        .arg("--manifest-path")
        .arg(scratch.join("Cargo.toml"))
        .current_dir(scratch)
        .env("CARGO_TARGET_DIR", scratch.join("target"))
        .output()
        .map_err(|error| error.to_string())
}

/// Render one unsuccessful subprocess as an actionable test refusal.
fn command_refusal(label: &str, output: &Output) -> String {
    format!(
        "{label} refused with status {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// Write the fixed producer library and downstream consumer.
fn write_specimen(scratch: &Path) -> Result<(), String> {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "the proc package is not below the repository root".to_owned())?;
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

/// The proc package's named support debt stays at its exact current denominator until one support owner replaces it.
#[test]
fn named_proc_test_support_debt_does_not_expand() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    assert_source_occurrences(
        &root,
        concat!("fn ", "scratch_root("),
        &[
            "advanced_rust_facade_crossing.rs",
            "generated_support_crossing.rs",
            "recipe_facade_crossing/support.rs",
        ],
    )?;
    assert_source_occurrences(
        &root,
        concat!("fn ", "cargo("),
        &[
            "advanced_rust_facade_crossing.rs",
            "generated_support_crossing.rs",
            "recipe_facade_crossing/support.rs",
        ],
    )?;
    assert_source_occurrences(
        &root,
        concat!("fn ", "write_specimen("),
        &[
            "advanced_rust_facade_crossing.rs",
            "generated_support_crossing.rs",
            "recipe_facade_crossing/support.rs",
        ],
    )?;
    assert_source_occurrences(
        &root,
        concat!("fn ", "manifest_path("),
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
    let scratch = scratch_root()?;
    let observed = observe_crossing(&scratch);
    let removed = std::fs::remove_dir_all(&scratch).map_err(|error| error.to_string());
    match (observed, removed) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(refusal), Ok(())) => Err(refusal),
        (Ok(()), Err(cleanup)) => Err(format!(
            "generated-support qualification passed but scratch cleanup refused at {}: {cleanup}",
            scratch.display()
        )),
        (Err(refusal), Err(cleanup)) => Err(format!(
            "{refusal}\ngenerated-support scratch cleanup also refused at {}: {cleanup}",
            scratch.display()
        )),
    }
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
