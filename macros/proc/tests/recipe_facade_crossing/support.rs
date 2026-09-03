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

use crate::scratch::{
    cargo, command_refusal, manifest_path, observed_in_scratch_for, repository_root,
};
use std::path::Path;

#[derive(Clone, Copy)]
enum AdopterUnsafePosture {
    Forbidden,
    CallerOwned,
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
    let facade = manifest_path(repository_root()?)?;
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

/// Observe one recipe-facade journey inside its own exclusively owned scratch root.
pub(super) fn observed_in_scratch(
    observe: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<(), String> {
    observed_in_scratch_for("recipe_facade", observe)
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
