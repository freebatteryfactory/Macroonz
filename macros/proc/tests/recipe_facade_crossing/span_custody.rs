//! Real proc-token observers participate in the ordinary wall through their existing Rust-required fixture package.

use crate::scratch::{
    cargo, command_refusal, lock_from_repository, manifest_path, observed_in_scratch_for,
    repository_root,
};
use std::io::Write;
use std::path::Path;

fn observe(scratch: &Path) -> Result<(), String> {
    let repository = repository_root()?;
    let fixture = repository.join("macros/proc/tests/support/capture-observer");
    let manifest =
        std::fs::read_to_string(fixture.join("Cargo.toml")).map_err(|error| error.to_string())?;
    let relative = "path = \"../../../../compiler\"";
    if manifest.matches(relative).count() != 1 {
        return Err(
            "the proc observer must declare exactly one compiler dependency path".to_owned(),
        );
    }
    let compiler = manifest_path(&repository.join("macros/compiler"))?;
    let manifest = manifest.replace(relative, &format!("path = \"{compiler}\""));
    std::fs::write(scratch.join("Cargo.toml"), manifest).map_err(|error| error.to_string())?;
    for directory in ["src", "tests"] {
        std::fs::create_dir(scratch.join(directory)).map_err(|error| error.to_string())?;
    }
    for file in [
        "README.md",
        "src/lib.rs",
        "tests/recipe_span_custody.rs",
        "tests/token_normalization.rs",
    ] {
        std::fs::copy(fixture.join(file), scratch.join(file)).map_err(|error| error.to_string())?;
    }
    let locked = lock_from_repository(scratch)?;
    if !locked.status.success() {
        return Err(command_refusal(
            "proc observer lock reconciliation",
            &locked,
        ));
    }
    for target in ["recipe_span_custody", "token_normalization"] {
        let tested = cargo(
            scratch,
            &[
                "nextest",
                "run",
                "--test",
                target,
                "--locked",
                "--offline",
                "--no-tests",
                "fail",
                "-j1",
            ],
        )?;
        if !tested.status.success() {
            return Err(command_refusal(target, &tested));
        }
        writeln!(
            std::io::stdout().lock(),
            "Proc observer {target}:\n{}\n{}",
            String::from_utf8_lossy(&tested.stdout),
            String::from_utf8_lossy(&tested.stderr),
        )
        .map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[test]
fn real_proc_tokens_preserve_recipe_spans_and_normalization() -> Result<(), String> {
    observed_in_scratch_for("proc_span_observer", observe)
}
