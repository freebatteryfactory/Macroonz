//! The public caller actually executes both generated synchronization faces.

use super::configure::{bounds, host, root, spelling, target, tool};
use super::types::Host;
use macroonz::native_process::{self, CaptureEnd};
use std::path::Path;
use std::process::Output;

#[test]
fn public_shadow_example_executes_ordinary_and_modeled_counter_checks() -> Result<(), String> {
    let run = root()?;
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let subject = run.join("shadow-workflow");
    prepare(repository, &subject)?;
    let root_lock = std::fs::read(repository.join("Cargo.lock")).map_err(debug)?;
    std::fs::write(subject.join("Cargo.lock"), &root_lock).map_err(debug)?;
    accepted(&crate::check::cargo(
        &subject,
        repository,
        &run,
        "shadow-lock",
        &["update", "--workspace", "--offline"],
    )?)?;
    let host = host(&run)?;
    accepted(&crate::check::cargo(
        &subject,
        repository,
        &run,
        "shadow-format",
        &["fmt", "--all", "--", "--check"],
    )?)?;
    for (name, command, configuration, expected) in [
        (
            "ordinary",
            "build",
            vec![],
            b"ordinary: sequential check and concurrent repair agree\n".as_slice(),
        ),
        (
            "shadow",
            "rustc",
            vec!["--cfg", "loom"],
            b"shadow: sequential check holds; lost update caught; repair holds within bounds\n"
                .as_slice(),
        ),
    ] {
        let mut arguments = vec![command, "--bin", "shadow-workflow", "--locked", "--offline"];
        if !configuration.is_empty() {
            arguments.push("--");
            arguments.extend(configuration.iter().copied());
        }
        accepted(&crate::check::cargo(
            &subject, repository, &run, name, &arguments,
        )?)?;
        execute(&host, &subject, expected)?;
        let lint = [
            "clippy",
            "--bin",
            "shadow-workflow",
            "--locked",
            "--offline",
            "--",
            "-Dwarnings",
        ]
        .into_iter()
        .chain(configuration)
        .collect::<Vec<_>>();
        accepted(&crate::check::cargo(
            &subject,
            repository,
            &run,
            &format!("{name}-clippy"),
            &lint,
        )?)?;
    }
    let imports = std::fs::read_to_string(subject.join("synchronization/mod.rs")).map_err(debug)?;
    std::fs::write(
        subject.join("synchronization/mod.rs"),
        imports.replace("AtomicUsize, Ordering", "InventedAtomic, Ordering"),
    )
    .map_err(debug)?;
    let refused = crate::check::cargo(
        &subject,
        repository,
        &run,
        "shadow-unknown-name",
        &["check", "--bin", "shadow-workflow", "--locked", "--offline"],
    )?;
    assert!(!refused.status.success());
    let diagnostic = String::from_utf8_lossy(&refused.stderr);
    assert!(diagnostic.contains("InventedAtomic"), "{diagnostic}");
    assert!(
        diagnostic.contains("a chosen name is not one the shadow roster covers"),
        "{diagnostic}"
    );
    assert_eq!(
        std::fs::read(repository.join("Cargo.lock")).map_err(debug)?,
        root_lock
    );
    Ok(())
}

fn prepare(repository: &Path, subject: &Path) -> Result<(), String> {
    let example = repository.join("examples/shadow_workflow");
    std::fs::create_dir_all(subject.join("synchronization")).map_err(debug)?;
    for source in [
        "consumer.rs",
        "increment.rs",
        "observation.rs",
        "synchronization/mod.rs",
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

fn execute(host: &Host, subject: &Path, expected: &[u8]) -> Result<(), String> {
    let executable = target()?
        .join("debug")
        .join(format!("shadow-workflow{}", std::env::consts::EXE_SUFFIX));
    let request = tool(host, &executable, subject, bounds()?)?
        .invocation(Vec::new())
        .map_err(debug)?;
    let output = crate::process::completed(native_process::run(&request, None).map_err(debug)?)?;
    assert!(
        output.status().success(),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    assert_eq!(output.stdout().end(), &CaptureEnd::Eof);
    assert_eq!(output.stdout().bytes(), expected);
    Ok(())
}

fn accepted(output: &Output) -> Result<(), String> {
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
