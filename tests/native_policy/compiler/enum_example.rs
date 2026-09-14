//! The public enum declaration delivers its carrier across a real Rust crate boundary.

use super::configure::{bounds, host, root, spelling, target, tool};
use macroonz::native_process::{self, CaptureEnd};
use std::path::Path;

#[test]
fn public_enum_example_executes_permitted_and_withheld_carriers() -> Result<(), String> {
    let run = root()?;
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let subject = run.join("enum-mutation-workflow");
    prepare(repository, &subject)?;
    let root_lock = std::fs::read(repository.join("Cargo.lock")).map_err(debug)?;
    std::fs::write(subject.join("Cargo.lock"), &root_lock).map_err(debug)?;
    for (name, arguments) in [
        ("enum-lock", vec!["update", "--workspace", "--offline"]),
        ("enum-format", vec!["fmt", "--all", "--", "--check"]),
        (
            "enum-clippy",
            vec![
                "clippy",
                "--all-targets",
                "--locked",
                "--offline",
                "--",
                "-Dwarnings",
            ],
        ),
        (
            "enum-build",
            vec![
                "build",
                "--bin",
                "enum-mutation-workflow",
                "--locked",
                "--offline",
            ],
        ),
    ] {
        let output = crate::check::cargo(&subject, repository, &run, name, &arguments)?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let host = host(&run)?;
    let executable = target()?.join("debug").join(format!(
        "enum-mutation-workflow{}",
        std::env::consts::EXE_SUFFIX
    ));
    let request = tool(&host, &executable, &subject, bounds()?)?
        .invocation(Vec::new())
        .map_err(debug)?;
    let output = crate::process::completed(native_process::run(&request, None).map_err(debug)?)?;
    assert!(
        output.status().success(),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    assert_eq!(output.stdout().end(), &CaptureEnd::Eof);
    assert_eq!(output.stdout().bytes(), b"enum: unchanged order agrees; two alternatives disagree; unpermitted point stays discoverable\n");
    let types = std::fs::read_to_string(subject.join("types.rs")).map_err(debug)?;
    let malformed = types.replace(
        "pub enum Priority {\n    /// Work that must be handled first.\n    Urgent = 7,\n    /// Ordinary pending work.\n    Normal = 11,\n    /// Work the caller may defer.\n    Deferred = 23,\n}",
        "pub struct Priority;",
    ).replace("#[repr(u8)]\n", "");
    assert_ne!(malformed, types);
    assert!(malformed.contains("pub struct Priority;"));
    std::fs::write(subject.join("types.rs"), malformed).map_err(debug)?;
    let refused = crate::check::cargo(
        &subject,
        repository,
        &run,
        "enum-wrong-item",
        &["check", "--lib", "--locked", "--offline"],
    )?;
    assert!(!refused.status.success());
    let diagnostic = String::from_utf8_lossy(&refused.stderr);
    assert!(
        diagnostic.contains("the item the helper sits on states no declared order"),
        "{diagnostic}"
    );
    assert!(diagnostic.contains("types.rs:3:1"), "{diagnostic}");
    assert_eq!(
        std::fs::read(repository.join("Cargo.lock")).map_err(debug)?,
        root_lock
    );
    Ok(())
}

fn prepare(repository: &Path, subject: &Path) -> Result<(), String> {
    let example = repository.join("examples/enum_mutation");
    std::fs::create_dir(subject).map_err(debug)?;
    for source in [
        "consumer.rs",
        "declaration.rs",
        "observation.rs",
        "types.rs",
    ] {
        std::fs::copy(example.join(source), subject.join(source)).map_err(debug)?;
    }
    let manifest = std::fs::read_to_string(example.join("consumer.toml")).map_err(debug)?;
    let dependency = spelling(repository)?
        .replace('\\', "/")
        .replace('"', "\\\"");
    std::fs::write(
        subject.join("Cargo.toml"),
        manifest.replace("/absolute/path/to/Macroonz", &dependency),
    )
    .map_err(debug)
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
