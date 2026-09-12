//! Target and reverse-edge controls for the narrowly admitted Windows parser dependency.

use super::check::cargo;
use super::types::StorageDependencies;
use std::path::Path;

pub(super) fn observe(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
    native: StorageDependencies,
) -> Result<(), String> {
    for target in [
        "x86_64-pc-windows-msvc",
        "x86_64-unknown-linux-gnu",
        "aarch64-apple-darwin",
        "wasm32-unknown-unknown",
    ] {
        let output = cargo(
            subject,
            profile,
            scratch,
            &format!("{posture}-process-graph-{target}"),
            &[
                "tree",
                "--locked",
                "--offline",
                "--edges",
                "normal,build",
                "--prefix",
                "none",
                "--format",
                "{p}",
                "--target",
                target,
            ],
        )?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into_owned());
        }
        let graph = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
        let expected = native == StorageDependencies::Native && target == "x86_64-pc-windows-msvc";
        for prefix in [
            "process-wrap v",
            "syn v2.",
            "windows-implement v",
            "windows-interface v",
        ] {
            if graph.lines().any(|line| line.starts_with(prefix)) != expected {
                return Err(format!(
                    "{posture}/{target}: unexpected {prefix} dependency"
                ));
            }
        }
        if expected {
            observe_reverse(subject, scratch, profile, posture, target)?;
        }
    }
    Ok(())
}

fn observe_reverse(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
    target: &str,
) -> Result<(), String> {
    let output = cargo(
        subject,
        profile,
        scratch,
        &format!("{posture}-process-reverse-{target}"),
        &[
            "tree",
            "--locked",
            "--offline",
            "--edges",
            "normal,build",
            "--prefix",
            "depth",
            "--no-dedupe",
            "--format",
            "{p}",
            "--target",
            target,
            "--invert",
            "syn@2.0.119",
        ],
    )?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    reverse_paths(&String::from_utf8(output.stdout).map_err(|error| error.to_string())?)
}

fn reverse_paths(graph: &str) -> Result<(), String> {
    let mut path = Vec::<&str>::new();
    for line in graph.lines() {
        let digits = line
            .find(|letter: char| !letter.is_ascii_digit())
            .ok_or("missing package")?;
        let (depth_text, package_text) = line.split_at(digits);
        let depth = depth_text
            .parse::<usize>()
            .map_err(|error| error.to_string())?;
        let package = package_text.split(" (").next().ok_or("missing package")?;
        if depth > path.len() {
            return Err("missing dependency ancestor".to_owned());
        }
        if depth < path.len() && path.last().copied() != Some("native-clock-consumer v0.0.0") {
            return Err("dependency path does not end at the independent consumer".to_owned());
        }
        path.truncate(depth);
        let allowed = match (path.last().copied(), package) {
            (None, "syn v2.0.119") => path.is_empty(),
            (Some("syn v2.0.119"), "windows-implement v0.60.2" | "windows-interface v0.59.3")
            | (
                Some("windows-implement v0.60.2" | "windows-interface v0.59.3"),
                "windows-core v0.62.2",
            )
            | (
                Some("windows-core v0.62.2"),
                "windows v0.62.2"
                | "windows-collections v0.3.2"
                | "windows-future v0.3.2"
                | "windows-numerics v0.3.1",
            )
            | (
                Some(
                    "windows-collections v0.3.2"
                    | "windows-future v0.3.2"
                    | "windows-numerics v0.3.1",
                ),
                "windows v0.62.2",
            )
            | (Some("windows v0.62.2"), "process-wrap v10.0.0" | "macroonz v0.2.0")
            | (Some("process-wrap v10.0.0"), "macroonz v0.2.0")
            | (Some("macroonz v0.2.0"), "native-clock-consumer v0.0.0") => true,
            _ => false,
        };
        if !allowed {
            return Err(format!(
                "unapproved parser dependency path: {path:?} -> {package}"
            ));
        }
        path.push(package);
    }
    if path.last().copied() == Some("native-clock-consumer v0.0.0") {
        Ok(())
    } else {
        Err("no complete parser dependency path".to_owned())
    }
}

#[test]
fn parser_exception_refuses_direct_and_unrelated_dependency_paths() {
    for hostile in [
        "",
        "0syn v2.0.119\n1macroonz v0.2.0\n2native-clock-consumer v0.0.0\n",
        "0syn v2.0.119\n1windows-implement v0.60.2\n2macroonz v0.2.0\n3native-clock-consumer v0.0.0\n",
        "0syn v2.0.119\n1foreign-parser v1.0.0\n2native-clock-consumer v0.0.0\n",
        "0syn v2.0.119\n2windows-implement v0.60.2\n",
    ] {
        assert!(reverse_paths(hostile).is_err());
    }
}

pub(super) fn refuse_injected(root: &Path, subject: &Path, scratch: &Path) -> Result<(), String> {
    let manifest = super::consumer::manifest(root, "\"native-tooling\"")?.replace("[dependencies]\n", "[dependencies]\nstray_parser = { package = \"syn\", version = \"=2.0.119\", default-features = false }\n");
    std::fs::write(subject.join("Cargo.toml"), manifest).map_err(|error| error.to_string())?;
    let output = cargo(
        subject,
        root,
        scratch,
        "injected-parser-lock",
        &["update", "--workspace", "--offline"],
    )?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    match observe_reverse(
        subject,
        scratch,
        root,
        "injected-parser",
        "x86_64-pc-windows-msvc",
    ) {
        Err(error) if error.contains("unapproved parser dependency path") => Ok(()),
        other => Err(format!(
            "the real direct parser injection was not refused for its path: {other:?}"
        )),
    }
}
