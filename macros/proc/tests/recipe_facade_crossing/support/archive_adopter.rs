//! A standalone Cargo consumer executes delivered callers and native retention without a checkout dependency or a copied host implementation.

use super::delivery::run;
use crate::scratch::{cargo_with_target, command_refusal, manifest_path};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

pub(super) fn observe(delivered: &Path, target: &Path) -> Result<(), String> {
    let adopter = prepare(delivered)?;
    run(&adopter, target, &["update", "--workspace", "--offline"])?;
    let graph = run(
        &adopter,
        target,
        &[
            "tree",
            "--locked",
            "--offline",
            "--edges",
            "normal",
            "--prefix",
            "none",
            "--format",
            "{p}",
            "--no-dedupe",
        ],
    )?;
    super::delivery::check_graph(delivered, &graph)?;
    run(
        &adopter,
        target,
        &["build", "--bins", "--locked", "--offline"],
    )?;
    let job = run(
        &adopter,
        target,
        &["run", "--bin", "delivered-job", "--locked", "--offline"],
    )?;
    for expected in [
        "all: Ok(EveryTrialConcluded { selected: 2, denominator: 2 })",
        "codec: Ok(EveryTrialConcluded { selected: 1, denominator: 2 })",
        "required-lane control: Err(\"required declared trials were not selected\")",
        "absent: Some(Unknown { position: 0 })",
        "benchmark: Ok(())",
        "same-work control: Some(PlantedWorseNotDistinguished)",
    ] {
        assert!(job.lines().any(|line| line == expected), "{job}");
    }
    retained(&adopter, target)?;
    skill(&adopter, delivered, target)
}

fn prepare(delivered: &Path) -> Result<PathBuf, String> {
    let adopter = delivered
        .parent()
        .ok_or("the delivered source has no scratch parent")?
        .join("native-adopter");
    std::fs::create_dir(&adopter).map_err(debug)?;
    let manifest = format!(
        r#"[package]
name = "delivered-neutral-adopter"
version = "0.0.0"
edition = "2024"
publish = false
build = false

[[bin]]
name = "delivered-job"
path = "{}"

[[bin]]
name = "delivered-count"
path = "{}"

[[bin]]
name = "delivered-skill"
path = "skill.rs"

[features]
full = ["macroonz/full"]

[dependencies]
macroonz = {{ version = "={}", default-features = false, features = ["native-tooling"] }}
serde_json = "1"

[lints.rust]
warnings = "deny"
unsafe_code = "forbid"

[workspace]
{}"#,
        manifest_path(&delivered.join("examples/job_workflow/main.rs"))?,
        manifest_path(&delivered.join("examples/retained_workflow/main.rs"))?,
        env!("CARGO_PKG_VERSION"),
        super::delivery::patches(delivered)?,
    );
    std::fs::write(adopter.join("Cargo.toml"), manifest).map_err(debug)?;
    std::fs::copy(delivered.join("Cargo.lock"), adopter.join("Cargo.lock")).map_err(debug)?;
    super::skill::prepare(delivered, &adopter.join("skill.rs"))?;
    Ok(adopter)
}

fn skill(adopter: &Path, delivered: &Path, target: &Path) -> Result<(), String> {
    let arguments = [
        "run",
        "--bin",
        "delivered-skill",
        "--features",
        "full",
        "--locked",
        "--offline",
    ];
    run(adopter, target, &arguments)?;
    let [changed, amended] = super::skill::extension(delivered)?;
    std::fs::write(adopter.join("skill.rs"), changed).map_err(debug)?;
    let disagreed = cargo_with_target(adopter, target, &arguments)?;
    refusal(&disagreed, "assertion `left == right` failed")?;
    std::fs::write(adopter.join("skill.rs"), amended).map_err(debug)?;
    run(adopter, target, &arguments)?;
    super::skill::prepare(delivered, &adopter.join("skill.rs"))?;
    run(adopter, target, &arguments)?;
    Ok(())
}

fn retained(adopter: &Path, target: &Path) -> Result<(), String> {
    let storage = adopter.join("storage");
    std::fs::create_dir(&storage).map_err(debug)?;
    let executable = target
        .join("debug")
        .join(format!("delivered-count{}", std::env::consts::EXE_SUFFIX));
    for (action, expected) in [
        ("retain", "retained original [7, 1, 9]; reached witness [1]"),
        (
            "inspect",
            "loaded historical records; decodes=0 checks=0 probes=0",
        ),
        ("replay", "DefectReproduced; decodes=1 checks=1 probes=0"),
        (
            "replay-fixed",
            "FixedOnWitness; decodes=1 checks=1 probes=0",
        ),
    ] {
        let output = invoke(&executable, &storage, action)?;
        let observed = success(action, &output)?;
        assert_eq!(observed.lines().last(), Some(expected));
    }
    let archived = storage.join("item-count/item-input");
    let original = std::fs::read(&archived).map_err(debug)?;
    let collided = invoke(&executable, &storage, "retain")?;
    refusal(&collided, "fresh batch")?;
    assert_eq!(std::fs::read(&archived).map_err(debug)?, original);
    std::fs::write(&archived, b"not an input archive").map_err(debug)?;
    refusal(
        &invoke(&executable, &storage, "inspect")?,
        "original input convention",
    )?;
    std::fs::write(&archived, original).map_err(debug)?;
    success(
        "restored inspection",
        &invoke(&executable, &storage, "inspect")?,
    )?;
    Ok(())
}

fn invoke(executable: &Path, storage: &Path, action: &str) -> Result<Output, String> {
    let input = storage.join("configuration.json");
    let configuration = format!(
        "{{\"action\":\"{action}\",\"storage\":\"{}\",\"batch\":\"count\"}}",
        manifest_path(storage)?,
    );
    std::fs::write(&input, configuration).map_err(debug)?;
    Command::new(executable)
        .current_dir(storage)
        .stdin(Stdio::from(std::fs::File::open(input).map_err(debug)?))
        .output()
        .map_err(debug)
}

fn success(label: &str, output: &Output) -> Result<String, String> {
    if !output.status.success() {
        return Err(command_refusal(label, output));
    }
    let mut log = std::io::stdout().lock();
    writeln!(log, "archive adopter: {label}").map_err(debug)?;
    log.write_all(&output.stdout).map_err(debug)?;
    log.write_all(&output.stderr).map_err(debug)?;
    String::from_utf8(output.stdout.clone()).map_err(debug)
}

fn refusal(output: &Output, expected: &str) -> Result<(), String> {
    if output.status.success()
        || !String::from_utf8_lossy(&output.stderr).contains(expected)
        || !output.stdout.is_empty()
    {
        return Err(command_refusal("expected adopter refusal", output));
    }
    writeln!(
        std::io::stdout().lock(),
        "archive adopter refused: {expected}"
    )
    .map_err(debug)
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
