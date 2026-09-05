//! Package extraction and locked execution for the archive-delivery claim, using the existing proc scratch and Cargo owners.

use crate::scratch::{cargo_with_target, command_refusal, manifest_path, repository_root};
use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Component, Path};
use std::process::Command;

const PACKAGES: [(&str, &str); 4] = [
    ("macroonz", ""),
    ("macroonz-compiler", "macros/compiler"),
    ("macroonz-harness", "harness"),
    ("macroonz-macros", "macros/proc"),
];

pub(super) fn observe(scratch: &Path) -> Result<(), String> {
    let source = repository_root()?;
    let packaged = scratch.join("packaged");
    run(
        source,
        &packaged,
        &[
            "package",
            "--workspace",
            "--all-features",
            "--locked",
            "--offline",
            "--no-verify",
            "--allow-dirty",
        ],
    )?;
    let root = scratch.join("delivered");
    for (name, relative) in PACKAGES {
        let identity = format!("{name}-{}", env!("CARGO_PKG_VERSION"));
        let archive = packaged.join("package").join(format!("{identity}.crate"));
        extract(&identity, &archive, &root.join(relative))?;
    }
    join(&root, source)?;
    super::skill::prepare(&root)?;
    let target = scratch.join("build");
    let graph = run(
        &root,
        &target,
        &[
            "tree",
            "--workspace",
            "--all-features",
            "--locked",
            "--offline",
            "--prefix",
            "none",
            "--format",
            "{p}",
            "--no-dedupe",
        ],
    )?;
    check_graph(&root, &graph)?;
    execute(&root, &target)?;
    let source_lock =
        std::fs::read(source.join("Cargo.lock")).map_err(|error| error.to_string())?;
    let delivered_lock =
        std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())?;
    if source_lock != delivered_lock {
        return Err("archive execution changed the committed dependency lock".to_owned());
    }
    Ok(())
}

fn extract(identity: &str, archive: &Path, destination: &Path) -> Result<(), String> {
    let listed = Command::new("tar")
        .arg("-tf")
        .arg(archive)
        .output()
        .map_err(|error| error.to_string())?;
    if !listed.status.success() {
        return Err(command_refusal("archive roster", &listed));
    }
    check_roster(
        identity,
        std::str::from_utf8(&listed.stdout).map_err(|error| error.to_string())?,
    )?;
    std::fs::create_dir_all(destination).map_err(|error| error.to_string())?;
    let extracted = Command::new("tar")
        .arg("-xf")
        .arg(archive)
        .arg("--strip-components=1")
        .arg("-C")
        .arg(destination)
        .output()
        .map_err(|error| error.to_string())?;
    if !extracted.status.success() {
        return Err(command_refusal("archive extraction", &extracted));
    }
    let stamp = std::fs::read(destination.join(".cargo_vcs_info.json"))
        .map_err(|error| error.to_string())?;
    let mut log = std::io::stdout().lock();
    writeln!(log, "archive source record: {identity}").map_err(|error| error.to_string())?;
    log.write_all(&stamp).map_err(|error| error.to_string())?;
    writeln!(log).map_err(|error| error.to_string())?;
    Ok(())
}

pub(super) fn check_roster(identity: &str, roster: &str) -> Result<(), String> {
    let prefix = format!("{identity}/");
    let manifest = format!("{prefix}Cargo.toml");
    let mut seen = BTreeSet::new();
    for entry in roster.lines() {
        if !entry.starts_with(&prefix)
            || entry.contains('\\')
            || Path::new(entry)
                .components()
                .any(|part| !matches!(part, Component::Normal(_)))
            || !seen.insert(entry)
        {
            return Err(format!(
                "archive entry is repeated or outside its package: {entry}"
            ));
        }
    }
    if !seen.contains(manifest.as_str()) {
        return Err("archive roster has no normalized manifest".to_owned());
    }
    Ok(())
}

fn join(root: &Path, source: &Path) -> Result<(), String> {
    let mut manifest = std::fs::OpenOptions::new()
        .append(true)
        .open(root.join("Cargo.toml"))
        .map_err(|error| error.to_string())?;
    manifest
        .write_all(b"\n[workspace]\nresolver = \"3\"\nmembers = [\n")
        .map_err(|error| error.to_string())?;
    for (_, relative) in PACKAGES {
        if !relative.is_empty() {
            writeln!(manifest, "\"{}\",", manifest_path(Path::new(relative))?)
                .map_err(|error| error.to_string())?;
        }
    }
    manifest
        .write_all(b"]\n\n[patch.crates-io]\n")
        .map_err(|error| error.to_string())?;
    for (name, relative) in PACKAGES {
        let path = if relative.is_empty() { "." } else { relative };
        writeln!(
            manifest,
            "{name} = {{ path = \"{}\" }}",
            manifest_path(Path::new(path))?
        )
        .map_err(|error| error.to_string())?;
    }
    std::fs::copy(source.join("Cargo.lock"), root.join("Cargo.lock"))
        .map_err(|error| error.to_string())?;
    Ok(())
}

pub(super) fn expected_graph(root: &Path) -> Vec<String> {
    PACKAGES
        .iter()
        .map(|(name, relative)| {
            let kind = if *name == "macroonz-macros" {
                " (proc-macro)"
            } else {
                ""
            };
            let directory = if relative.is_empty() {
                root.to_path_buf()
            } else {
                root.join(relative)
            };
            format!(
                "{name} v{}{kind} ({})",
                env!("CARGO_PKG_VERSION"),
                directory.display()
            )
            .replace(std::path::MAIN_SEPARATOR, "/")
        })
        .collect()
}

pub(super) fn check_graph(root: &Path, graph: &str) -> Result<(), String> {
    let expected: BTreeSet<_> = expected_graph(root).into_iter().collect();
    let observed: BTreeSet<_> = graph
        .lines()
        .filter(|line| line.starts_with("macroonz"))
        .map(|line| line.replace(std::path::MAIN_SEPARATOR, "/"))
        .collect();
    if observed != expected {
        return Err(format!(
            "archive resolution escaped its extracted graph: expected {expected:?}, observed {observed:?}"
        ));
    }
    Ok(())
}

fn run(root: &Path, target: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = cargo_with_target(root, target, arguments)?;
    let mut log = std::io::stdout().lock();
    writeln!(log, "archive Cargo observation: {arguments:?}").map_err(|error| error.to_string())?;
    if arguments.first() != Some(&"tree") {
        log.write_all(&output.stdout)
            .map_err(|error| error.to_string())?;
    }
    log.write_all(&output.stderr)
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(command_refusal("extracted package execution", &output));
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn execute(root: &Path, target: &Path) -> Result<(), String> {
    facade_controls(root, target)?;
    examples(root, target)?;
    run(
        root,
        target,
        &[
            "build",
            "-p",
            "macroonz",
            "--lib",
            "--all-features",
            "--locked",
            "--offline",
        ],
    )?;
    super::skill::execute(root, target)?;
    writeln!(
        std::io::stdout().lock(),
        "archive skill: warnings-denied compilation and all four independent policy pairs passed"
    )
    .map_err(|error| error.to_string())?;
    run(
        root,
        target,
        &[
            "check",
            "--workspace",
            "--all-features",
            "--target",
            "wasm32-unknown-unknown",
            "--locked",
            "--offline",
        ],
    )?;
    Ok(())
}

fn facade_controls(root: &Path, target: &Path) -> Result<(), String> {
    run(
        root,
        target,
        &[
            "check",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--locked",
            "--offline",
        ],
    )?;
    run(
        root,
        target,
        &[
            "nextest",
            "run",
            "-j1",
            "-p",
            "macroonz",
            "--test",
            "recipe_surface",
            "--all-features",
            "--locked",
            "--offline",
            "--no-tests",
            "fail",
        ],
    )?;
    for test in [
        "facade_surface",
        "recipe_no_harness_surface",
        "recipe_carrier_hostile",
    ] {
        run(
            root,
            target,
            &[
                "nextest",
                "run",
                "-j1",
                "-p",
                "macroonz",
                "--test",
                test,
                "--no-default-features",
                "--locked",
                "--offline",
                "--no-tests",
                "fail",
            ],
        )?;
    }
    Ok(())
}

fn examples(root: &Path, target: &Path) -> Result<(), String> {
    for (package, example) in [
        ("macroonz", "recipe"),
        ("macroonz", "compile_contract"),
        ("macroonz", "rustc_coverage"),
        ("macroonz-compiler", "custom_recipe_projector"),
        ("macroonz-harness", "temporal_property"),
    ] {
        run(
            root,
            target,
            &[
                "run",
                "-p",
                package,
                "--example",
                example,
                "--all-features",
                "--locked",
                "--offline",
            ],
        )?;
    }
    Ok(())
}
