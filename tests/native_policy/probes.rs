//! Independently authored compiler subjects for permitted and forbidden operations.

use super::check::cargo;
use std::path::Path;

const MANIFEST: &str = "[package]\nname = \"native-policy-subject\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\nbuild = false\n[lib]\npath = \"subject.rs\"\n[lints.rust]\nwarnings = \"deny\"\nunsafe_code = \"forbid\"\n[workspace]\n";

pub(super) fn observe(scratch: &Path, strict: &Path, native: &Path) -> Result<(), String> {
    let root = scratch.join("subject");
    std::fs::create_dir(&root).map_err(|error| error.to_string())?;
    std::fs::write(root.join("Cargo.toml"), MANIFEST).map_err(|error| error.to_string())?;
    let clock = "pub fn probe() -> std::time::Instant { std::time::Instant::now() }\n";
    std::fs::write(root.join("subject.rs"), clock).map_err(|error| error.to_string())?;
    let lock = cargo(
        &root,
        strict,
        scratch,
        "probe-lock",
        &["generate-lockfile", "--offline"],
    )?;
    if !lock.status.success() {
        return Err(format!(
            "probe lock failed: {}",
            String::from_utf8_lossy(&lock.stderr)
        ));
    }
    let system = "pub fn probe() -> std::time::SystemTime { std::time::SystemTime::now() }\n";
    let environment = "pub fn probe() -> Option<std::ffi::OsString> { std::env::var_os(\"UNREAD_QUALIFICATION_INPUT\") }\n";
    let mut last_source = clock;
    for (name, source, profile, expected) in [
        (
            "strict-clock",
            clock,
            strict,
            Some(("disallowed_types", "std::time::Instant")),
        ),
        ("native-clock", clock, native, None),
        (
            "strict-again",
            clock,
            strict,
            Some(("disallowed_types", "std::time::Instant")),
        ),
        (
            "native-system",
            system,
            native,
            Some(("disallowed_types", "std::time::SystemTime")),
        ),
        (
            "native-environment",
            environment,
            native,
            Some(("disallowed_methods", "std::env::var_os")),
        ),
    ] {
        if source != last_source {
            std::fs::write(root.join("subject.rs"), source).map_err(|error| error.to_string())?;
            last_source = source;
        }
        let output = cargo(
            &root,
            profile,
            scratch,
            name,
            &[
                "clippy",
                "-j1",
                "--lib",
                "--locked",
                "--offline",
                "--",
                "-Dwarnings",
            ],
        )?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        match expected {
            None if output.status.success() => {}
            Some((lint, operation))
                if output.status.code() == Some(101_i32)
                    && stderr.contains(lint)
                    && stderr.contains(operation) => {}
            _ => {
                return Err(format!(
                    "{name}: unexpected status {}\n{stderr}",
                    output.status
                ));
            }
        }
    }
    Ok(())
}
