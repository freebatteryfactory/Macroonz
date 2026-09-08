//! Independent feature-isolated Cargo consumers of the root clock entrance.

use super::check::cargo;
use std::path::Path;

fn manifest(root: &Path, features: &str) -> Result<String, String> {
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
            "fn main() { let _clock = macroonz::native_clock::source(); }\n",
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
        {
            return Err(format!(
                "{name}: expected native-clock feature refusal\n{stderr}"
            ));
        }
    }
    observe_native(&subject, scratch, strict)?;
    if std::fs::read(root.join("Cargo.lock")).map_err(|error| error.to_string())? != root_lock {
        return Err("the root lock changed during consumer qualification".to_owned());
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
