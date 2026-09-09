//! Independent feature-isolated Cargo consumers of the root native entrances.

use super::check::cargo;
use super::types::StorageDependencies;
use std::path::Path;

pub(super) fn manifest(root: &Path, features: &str) -> Result<String, String> {
    let path = root.to_str().ok_or("the dependency path is not UTF-8")?;
    if path.chars().any(char::is_control) {
        return Err("the dependency path contains a control character".to_owned());
    }
    let path = path.replace('\\', "/").replace('"', "\\\"");
    Ok(format!(
        "[package]\nname = \"native-clock-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\nbuild = false\n[[bin]]\nname = \"native-clock-consumer\"\npath = \"main.rs\"\n[lints.rust]\nwarnings = \"deny\"\nunsafe_code = \"forbid\"\n[dependencies]\nmacroonz = {{ path = \"{path}\", default-features = false, features = [{features}] }}\n[workspace]\n"
    ))
}

pub(super) fn observe(root: &Path, scratch: &Path, strict: &Path) -> Result<(), String> {
    let subject = scratch.join("consumer");
    std::fs::create_dir(&subject).map_err(|error| error.to_string())?;
    let root_lock = std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())?;
    for (name, features) in [
        ("consumer-diet", ""),
        ("consumer-harness", "\"harness\""),
        ("consumer-full", "\"full\""),
        ("consumer-native", "\"native-tooling\""),
    ] {
        std::fs::write(subject.join("Cargo.toml"), manifest(root, features)?)
            .map_err(|error| error.to_string())?;
        std::fs::write(subject.join("Cargo.lock"), &root_lock)
            .map_err(|error| error.to_string())?;
        std::fs::write(
            subject.join("main.rs"),
            "fn main() { let _clock = macroonz::native_clock::source(); let _name = macroonz::native_storage::StorageName::informed(\"run\"); }\n",
        )
        .map_err(|error| error.to_string())?;
        let lock = cargo(
            &subject,
            strict,
            scratch,
            &format!("{name}-lock"),
            &["update", "--workspace", "--offline"],
        )?;
        if !lock.status.success() {
            return Err(format!(
                "{name} lock: {}",
                String::from_utf8_lossy(&lock.stderr)
            ));
        }
        let storage = if name == "consumer-native" {
            StorageDependencies::Native
        } else {
            StorageDependencies::Absent
        };
        dependency_graph(&subject, scratch, strict, name, storage)?;
        let output = cargo(
            &subject,
            strict,
            scratch,
            name,
            &["check", "-j1", "--locked", "--offline"],
        )?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        if name == "consumer-native" {
            if !output.status.success() {
                return Err(format!("{name}: {stderr}"));
            }
        } else if output.status.code() != Some(101_i32)
            || !stderr.contains("E0433")
            || !stderr.contains("native_clock")
            || !stderr.contains("native_storage")
        {
            return Err(format!(
                "{name}: expected both native entrance feature refusals\n{stderr}"
            ));
        }
    }
    observe_native(&subject, scratch, strict)?;
    if std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())? != root_lock {
        return Err("the root lock changed during consumer qualification".to_owned());
    }
    Ok(())
}

fn dependency_graph(
    subject: &Path,
    scratch: &Path,
    strict: &Path,
    name: &str,
    storage: StorageDependencies,
) -> Result<(), String> {
    for target in ["all", "wasm32-unknown-unknown"] {
        let output = cargo(
            subject,
            strict,
            scratch,
            &format!("{name}-graph-{target}"),
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
                "--target",
                target,
            ],
        )?;
        if !output.status.success() {
            return Err(format!(
                "{name} graph: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        let graph = String::from_utf8(output.stdout).map_err(|error| error.to_string())?;
        for package in [
            "cap-std",
            "cap-primitives",
            "io-extras",
            "io-lifetimes",
            "fs-set-times",
            "winx",
        ] {
            let prefix = format!("{package} v");
            let present = graph.lines().any(|line| line.starts_with(&prefix));
            if present != (storage == StorageDependencies::Native && target == "all") {
                return Err(format!(
                    "{name}/{target}: unexpected {package} presence={present}"
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn qualify_graphs() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scratch = super::check::scratch()?;
    let subject = scratch.join("graph-consumer");
    std::fs::create_dir(&subject).map_err(|error| error.to_string())?;
    std::fs::write(subject.join("main.rs"), "fn main() {}\n").map_err(|error| error.to_string())?;
    let root_lock = std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())?;
    for (name, features, storage) in [
        ("diet", "", StorageDependencies::Absent),
        ("harness", "\"harness\"", StorageDependencies::Absent),
        ("full", "\"full\"", StorageDependencies::Absent),
        ("native", "\"native-tooling\"", StorageDependencies::Native),
        (
            "full-native",
            "\"full\", \"native-tooling\"",
            StorageDependencies::Native,
        ),
    ] {
        std::fs::write(subject.join("Cargo.toml"), manifest(root, features)?)
            .map_err(|error| error.to_string())?;
        std::fs::write(subject.join("Cargo.lock"), &root_lock)
            .map_err(|error| error.to_string())?;
        let output = cargo(
            &subject,
            root,
            &scratch,
            name,
            &["update", "--workspace", "--offline"],
        )?;
        if !output.status.success() {
            return Err(format!(
                "{name} lock: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        dependency_graph(&subject, &scratch, root, name, storage)?;
        workflow_surface(&subject, &scratch, root, name, storage)?;
    }
    if std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())? != root_lock {
        return Err("the root lock changed during graph qualification".to_owned());
    }
    Ok(())
}

fn workflow_surface(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
    storage: StorageDependencies,
) -> Result<(), String> {
    let run_refusal = if posture == "diet" {
        Some("E0433")
    } else {
        None
    };
    let retained_refusal = if storage == StorageDependencies::Native {
        None
    } else if posture == "diet" {
        Some("E0433")
    } else {
        Some("E0425")
    };
    for (name, source, expected) in [
        (
            "execute",
            "fn main() { let _run = macroonz::workflow::run::<Vec<u8>>; }\n",
            run_refusal,
        ),
        (
            "retained",
            "fn main() { let _record = Option::<macroonz::workflow::StoredRun>::None; }\n",
            retained_refusal,
        ),
    ] {
        std::fs::write(subject.join("main.rs"), source).map_err(|error| error.to_string())?;
        let output = cargo(
            subject,
            profile,
            scratch,
            &format!("{posture}-workflow-{name}"),
            &["check", "-j1", "--locked", "--offline"],
        )?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        match expected {
            None if output.status.success() => {}
            Some(code) if output.status.code() == Some(101_i32) && stderr.contains(code) => {}
            _ => {
                return Err(format!(
                    "{posture}/{name}: unexpected workflow feature posture\n{stderr}"
                ));
            }
        }
    }
    Ok(())
}

fn observe_native(subject: &Path, scratch: &Path, strict: &Path) -> Result<(), String> {
    std::fs::write(subject.join("main.rs"), include_str!("subject.rs"))
        .map_err(|error| error.to_string())?;
    for (name, arguments) in [
        ("consumer-run", vec!["run", "-j1", "--locked", "--offline"]),
        (
            "consumer-wasm",
            vec![
                "check",
                "-j1",
                "--locked",
                "--offline",
                "--target",
                "wasm32-unknown-unknown",
            ],
        ),
    ] {
        let output = cargo(subject, strict, scratch, name, &arguments)?;
        if !output.status.success() {
            return Err(format!(
                "{name}: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }
    Ok(())
}
