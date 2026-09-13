use super::destination_fixture::snapshot;
use crate::compiler::configure::{bounds, host, root, spelling, target, tool};
use crate::compiler::types::Host;
use macroonz::native_process::{self, ProcessLimits, ProcessOutput};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[test]
fn executable_publication_example_generates_and_checks_in_fresh_processes() -> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let executable = build(&source, &host)?;
    let work = root()?;
    let destination = root()?;
    std::fs::write(
        destination.join("authored.txt"),
        b"retain this authored neighbor",
    )
    .map_err(|error| error.to_string())?;
    let rustfmt = host
        .rustc
        .parent()
        .ok_or("toolchain parent absent")?
        .join(format!("rustfmt{}", std::env::consts::EXE_SUFFIX));
    let mut configuration = json!({
        "action": "prepare", "value": 42_u64,
        "workspace": spelling(&work)?, "destination": spelling(&destination)?,
        "rustc": spelling(&host.rustc)?, "target": host.triple, "environment": host.environment,
        "rustfmt": spelling(&rustfmt)?, "format_configuration": spelling(&super::configure::configuration(&source)?)?,
    });
    let prepared = report(&run(&source, &host, &executable, &configuration)?, 0)?;
    assert!(snapshot(&work)?.is_empty());
    *configuration.get_mut("action").ok_or("action absent")? = json!("inspect");
    let inspected = report(&run(&source, &host, &executable, &configuration)?, 0)?;
    assert_eq!(
        prepared.pointer("/files/0/canonical_digest"),
        inspected.pointer("/files/0/canonical_digest")
    );
    let before = snapshot(&destination)?;
    *configuration.get_mut("action").ok_or("action absent")? = json!("check");
    let missing = report(&run(&source, &host, &executable, &configuration)?, 1)?;
    assert_eq!(
        missing.pointer("/issues/0/problem"),
        Some(&json!("Missing"))
    );
    assert_eq!(snapshot(&destination)?, before);
    *configuration.get_mut("action").ok_or("action absent")? = json!("generate");
    let generated = report(&run(&source, &host, &executable, &configuration)?, 0)?;
    assert_eq!(generated.get("files"), inspected.get("files"));
    let binary = PathBuf::from(
        generated
            .get("executable")
            .and_then(Value::as_str)
            .ok_or("executable absent")?,
    );
    let request = tool(&host, &binary, &source, bounds()?)?
        .invocation(Vec::new())
        .map_err(|error| error.to_string())?;
    let observation = crate::process::completed(
        native_process::run(&request, None).map_err(|error| error.to_string())?,
    )?;
    assert!(observation.status().success());
    assert_eq!(observation.stdout().bytes(), b"42\n");
    *configuration.get_mut("action").ok_or("action absent")? = json!("check");
    let installed = snapshot(&destination)?;
    let current = report(&run(&source, &host, &executable, &configuration)?, 0)?;
    assert_eq!(current.get("current"), Some(&json!(true)));
    *configuration.get_mut("value").ok_or("value absent")? = json!(43_u64);
    let stale = report(&run(&source, &host, &executable, &configuration)?, 1)?;
    assert_eq!(stale.pointer("/issues/0/problem"), Some(&json!("Stale")));
    assert_eq!(snapshot(&destination)?, installed);
    std::fs::write(
        destination.join("value.rs"),
        b"pub const VALUE: u64 = 99;\n",
    )
    .map_err(|error| error.to_string())?;
    *configuration.get_mut("value").ok_or("value absent")? = json!(42_u64);
    let tampered = report(&run(&source, &host, &executable, &configuration)?, 1)?;
    assert_eq!(
        tampered.pointer("/issues/0/problem"),
        Some(&json!("Tampered"))
    );
    let damaged = snapshot(&destination)?;
    *configuration.get_mut("action").ok_or("action absent")? = json!("generate");
    let refused = run(&source, &host, &executable, &configuration)?;
    assert!(!refused.status().success());
    assert_eq!(snapshot(&destination)?, damaged);
    Ok(())
}

pub(super) fn report(output: &ProcessOutput, exit_code: i32) -> Result<Value, String> {
    assert_eq!(
        output.status().code(),
        Some(exit_code),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    serde_json::from_slice(output.stdout().bytes()).map_err(|error| {
        format!(
            "{error}: {}",
            String::from_utf8_lossy(output.stderr().bytes())
        )
    })
}

pub(super) fn run(
    source: &Path,
    host: &Host,
    executable: &Path,
    configuration: &Value,
) -> Result<ProcessOutput, String> {
    let input = source.join("publication-configuration.json");
    std::fs::write(
        &input,
        serde_json::to_vec(configuration).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let limits = ProcessLimits::informed(
        Duration::from_secs(210),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(|error| error.to_string())?;
    let request = tool(host, executable, source, limits)?
        .invocation(Vec::new())
        .map_err(|error| error.to_string())?;
    crate::process::completed(
        native_process::run(
            &request,
            Some(std::fs::File::open(input).map_err(|error| error.to_string())?),
        )
        .map_err(|error| error.to_string())?,
    )
}

fn build(source: &Path, host: &Host) -> Result<PathBuf, String> {
    let limits = ProcessLimits::informed(
        Duration::from_secs(180),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(|error| error.to_string())?;
    let selected = tool(
        host,
        &host.cargo,
        Path::new(env!("CARGO_MANIFEST_DIR")),
        limits,
    )?;
    let mut arguments = [
        "build",
        "--example",
        "publication_workflow",
        "--features",
        "native-tooling",
        "--locked",
        "--offline",
        "-j1",
        "--target-dir",
    ]
    .map(str::to_owned)
    .to_vec();
    arguments.push(spelling(&target()?)?);
    let request = selected
        .invocation(arguments)
        .map_err(|error| error.to_string())?;
    let output = crate::process::completed(
        native_process::run(&request, None).map_err(|error| error.to_string())?,
    )?;
    assert!(
        output.status().success(),
        "{}: {}",
        source.display(),
        String::from_utf8_lossy(output.stderr().bytes())
    );
    Ok(target()?.join("debug/examples").join(format!(
        "publication_workflow{}",
        std::env::consts::EXE_SUFFIX
    )))
}
