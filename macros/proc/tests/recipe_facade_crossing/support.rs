//! Scratch orchestration over exact package-shaped producer and consumer specimens.

#[path = "support/harness_refusal.rs"]
mod harness_refusal;
#[path = "support/generic_recipe.rs"]
mod generic_recipe;
#[path = "support/effect_execution.rs"]
mod effect_execution;
#[path = "support/historical_subjects.rs"]
mod historical_subjects;
#[path = "support/negative_space.rs"]
mod negative_space;
#[path = "support/no_harness.rs"]
mod no_harness;
#[path = "support/renamed_facade.rs"]
mod renamed_facade;

use effect_execution::{EFFECT_CONSUMER, EFFECT_PRODUCER};
use generic_recipe::{GENERIC_CONSUMER, GENERIC_PRODUCER, GENERIC_REFUSALS};
use harness_refusal::{EMPTY_CONSUMER, HARNESS_REFUSAL_PRODUCER};
use historical_subjects::{SUBJECT_JOURNEYS_CONSUMER, SUBJECT_JOURNEYS_PRODUCER};
use negative_space::{NEGATIVE_SPACE_CONSUMER, NEGATIVE_SPACE_PRODUCER};
use no_harness::{NO_HARNESS_CONSUMER, NO_HARNESS_PRODUCER};
use renamed_facade::{CONSUMER, PRODUCER};

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

static SCRATCH_ORDINAL: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy)]
enum AdopterUnsafePosture {
    Forbidden,
    CallerOwned,
}

fn scratch_root() -> Result<PathBuf, String> {
    let parent = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    for _attempt in 0u16..1_024u16 {
        let ordinal = SCRATCH_ORDINAL.fetch_add(1, Ordering::SeqCst);
        let candidate = parent.join(format!(
            "macroonz_recipe_facade_{}_{ordinal}",
            std::process::id()
        ));
        match std::fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    Err("no unoccupied recipe-facade scratch seat remained".to_owned())
}

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

fn push_toml_unicode_escape(character: char, into: &mut String) -> Result<(), String> {
    let code = u32::from(character);
    into.push_str("\\u");
    for shift in [12_u32, 8_u32, 4_u32, 0_u32] {
        into.push(hexadecimal_digit((code >> shift) & 0x0f)?);
    }
    Ok(())
}

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

fn command_refusal(label: &str, output: &Output) -> String {
    format!(
        "{label} refused with status {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn write_specimen(
    scratch: &Path,
    facade_features: &str,
    producer: &str,
    consumer: &str,
) -> Result<(), String> {
    write_specimen_for_edition(
        scratch,
        "2024",
        facade_features,
        producer,
        consumer,
        AdopterUnsafePosture::Forbidden,
    )
}

fn write_specimen_with_unsafe_posture(
    scratch: &Path,
    facade_features: &str,
    producer: &str,
    consumer: &str,
    unsafe_posture: AdopterUnsafePosture,
) -> Result<(), String> {
    write_specimen_for_edition(
        scratch,
        "2024",
        facade_features,
        producer,
        consumer,
        unsafe_posture,
    )
}

fn write_specimen_for_edition(
    scratch: &Path,
    edition: &str,
    facade_features: &str,
    producer: &str,
    consumer: &str,
    unsafe_posture: AdopterUnsafePosture,
) -> Result<(), String> {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "the proc package is not below the repository root".to_owned())?;
    let facade = manifest_path(repository)?;
    std::fs::create_dir(scratch.join("src")).map_err(|error| error.to_string())?;
    std::fs::create_dir(scratch.join("tests")).map_err(|error| error.to_string())?;
    let unsafe_lint = match unsafe_posture {
        AdopterUnsafePosture::Forbidden => "unsafe_code = \"forbid\"",
        AdopterUnsafePosture::CallerOwned => "",
    };
    let manifest = format!(
        r#"[package]
name = "renamed-recipe-adopter"
version = "0.0.0"
edition = "{edition}"
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
name = "recipe"
path = "tests/recipe.rs"

[dependencies]
bakery = {{ package = "macroonz", path = "{facade}", default-features = false{facade_features} }}

[lints.rust]
warnings = "deny"
{unsafe_lint}

[workspace]
"#
    );
    std::fs::write(scratch.join("Cargo.toml"), manifest).map_err(|error| error.to_string())?;
    std::fs::write(scratch.join("src/lib.rs"), producer).map_err(|error| error.to_string())?;
    std::fs::write(scratch.join("tests/recipe.rs"), consumer).map_err(|error| error.to_string())
}

pub(super) fn observe_effect_execution(scratch: &Path) -> Result<(), String> {
    write_specimen_with_unsafe_posture(
        scratch,
        "",
        EFFECT_PRODUCER,
        EFFECT_CONSUMER,
        AdopterUnsafePosture::CallerOwned,
    )?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("effect-execution lock generation", &locked));
    }
    let tested = cargo(scratch, &["test", "--locked", "--offline"])?;
    if !tested.status.success() {
        return Err(command_refusal("effect-execution qualification", &tested));
    }
    let wasm = cargo(
        scratch,
        &[
            "check",
            "--lib",
            "--locked",
            "--offline",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    if !wasm.status.success() {
        return Err(command_refusal("effect-execution Wasm posture", &wasm));
    }
    Ok(())
}

pub(super) fn observed_in_scratch(
    observe: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<(), String> {
    let scratch = scratch_root()?;
    let observed = observe(&scratch);
    let removed = std::fs::remove_dir_all(&scratch).map_err(|error| error.to_string());
    match (observed, removed) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(refusal), Ok(())) => Err(refusal),
        (Ok(()), Err(cleanup)) => Err(format!(
            "recipe-facade qualification passed but scratch cleanup refused at {}: {cleanup}",
            scratch.display()
        )),
        (Err(refusal), Err(cleanup)) => Err(format!(
            "{refusal}\nrecipe-facade scratch cleanup also refused at {}: {cleanup}",
            scratch.display()
        )),
    }
}

pub(super) fn observe_crossing(scratch: &Path) -> Result<(), String> {
    write_specimen(scratch, ", features = [\"harness\"]", PRODUCER, CONSUMER)?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("scratch lock generation", &locked));
    }
    let tested = cargo(scratch, &["test", "--locked", "--offline"])?;
    if !tested.status.success() {
        return Err(command_refusal("renamed recipe qualification", &tested));
    }
    let wasm = cargo(
        scratch,
        &[
            "check",
            "--lib",
            "--locked",
            "--offline",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    if !wasm.status.success() {
        return Err(command_refusal("renamed recipe Wasm posture", &wasm));
    }
    Ok(())
}

pub(super) fn observe_subject_journeys(scratch: &Path) -> Result<(), String> {
    write_specimen(
        scratch,
        ", features = [\"harness\"]",
        SUBJECT_JOURNEYS_PRODUCER,
        SUBJECT_JOURNEYS_CONSUMER,
    )?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("subject-journey lock generation", &locked));
    }
    let tested = cargo(scratch, &["test", "--locked", "--offline"])?;
    if !tested.status.success() {
        return Err(command_refusal("subject-journey qualification", &tested));
    }
    let wasm = cargo(
        scratch,
        &[
            "check",
            "--lib",
            "--locked",
            "--offline",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    if !wasm.status.success() {
        return Err(command_refusal("subject-journey Wasm posture", &wasm));
    }
    Ok(())
}

pub(super) fn observe_generic_crossing(scratch: &Path) -> Result<(), String> {
    write_specimen(scratch, "", GENERIC_PRODUCER, GENERIC_CONSUMER)?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("generic-recipe lock generation", &locked));
    }
    let tested = cargo(scratch, &["test", "--locked", "--offline"])?;
    if !tested.status.success() {
        return Err(command_refusal("generic-recipe qualification", &tested));
    }
    let wasm = cargo(
        scratch,
        &[
            "check",
            "--lib",
            "--locked",
            "--offline",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    if !wasm.status.success() {
        return Err(command_refusal("generic-recipe Wasm posture", &wasm));
    }
    Ok(())
}

pub(super) fn observe_negative_space(scratch: &Path, edition: &str) -> Result<(), String> {
    write_specimen_for_edition(
        scratch,
        edition,
        "",
        NEGATIVE_SPACE_PRODUCER,
        NEGATIVE_SPACE_CONSUMER,
        AdopterUnsafePosture::Forbidden,
    )?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("negative-space lock generation", &locked));
    }
    let tested = cargo(scratch, &["test", "--locked", "--offline"])?;
    if !tested.status.success() {
        return Err(command_refusal(
            "negative-space edition qualification",
            &tested,
        ));
    }
    Ok(())
}

pub(super) fn observe_generic_refusals(scratch: &Path) -> Result<(), String> {
    let Some((_, first, _)) = GENERIC_REFUSALS.first() else {
        return Err("the generic-refusal denominator is empty".to_owned());
    };
    write_specimen(scratch, "", first, "")?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("generic-refusal lock generation", &locked));
    }
    for (label, producer, expected) in GENERIC_REFUSALS {
        std::fs::write(scratch.join("src/lib.rs"), producer).map_err(|error| error.to_string())?;
        let checked = cargo(scratch, &["check", "--lib", "--locked", "--offline"])?;
        if checked.status.success() {
            return Err(format!("generic refusal `{label}` compiled successfully"));
        }
        let stderr = String::from_utf8_lossy(&checked.stderr);
        if !stderr.contains(expected) {
            return Err(command_refusal(label, &checked));
        }
    }
    Ok(())
}

pub(super) fn observe_without_harness(scratch: &Path) -> Result<(), String> {
    write_specimen(scratch, "", NO_HARNESS_PRODUCER, NO_HARNESS_CONSUMER)?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("no-harness lock generation", &locked));
    }
    let tested = cargo(scratch, &["test", "--locked", "--offline"])?;
    if !tested.status.success() {
        return Err(command_refusal("no-harness recipe qualification", &tested));
    }
    Ok(())
}

pub(super) fn observe_harness_refusal(scratch: &Path) -> Result<(), String> {
    write_specimen(scratch, "", HARNESS_REFUSAL_PRODUCER, EMPTY_CONSUMER)?;
    let locked = cargo(scratch, &["generate-lockfile", "--offline"])?;
    if !locked.status.success() {
        return Err(command_refusal("harness-refusal lock generation", &locked));
    }
    let checked = cargo(scratch, &["check", "--lib", "--locked", "--offline"])?;
    if checked.status.success() {
        return Err("a harness-owned bake compiled without the facade harness feature".to_owned());
    }
    let stderr = String::from_utf8_lossy(&checked.stderr);
    if !stderr
        .contains("projection `trials` requires the facade harness feature, which is unavailable")
    {
        return Err(command_refusal(
            "harness-owned projection produced the wrong refusal",
            &checked,
        ));
    }
    Ok(())
}
