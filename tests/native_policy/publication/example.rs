use super::destination_fixture::snapshot;
use crate::compiler::configure::{bounds, host, root, spelling, target, tool};
use crate::compiler::types::Host;
use crate::presentation_formats::field;
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
        field(files(&prepared)?, "/0/canonical_digest")?,
        field(files(&inspected)?, "/0/canonical_digest")?
    );
    let before = snapshot(&destination)?;
    *configuration.get_mut("action").ok_or("action absent")? = json!("check");
    let missing = report(&run(&source, &host, &executable, &configuration)?, 1)?;
    assert_eq!(
        field(&missing, "/record/value/comparison/issues/0/problem")?,
        &json!("missing")
    );
    assert_eq!(snapshot(&destination)?, before);
    *configuration.get_mut("action").ok_or("action absent")? = json!("generate");
    let generated = report(&run(&source, &host, &executable, &configuration)?, 0)?;
    assert_eq!(files(&generated)?, files(&inspected)?);
    let binary = executable_path(&generated)?;
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
    assert_eq!(
        field(&current, "/record/value/comparison/is_current")?,
        &json!(true)
    );
    *configuration.get_mut("value").ok_or("value absent")? = json!(43_u64);
    let stale = report(&run(&source, &host, &executable, &configuration)?, 1)?;
    assert_eq!(
        field(&stale, "/record/value/comparison/issues/0/problem")?,
        &json!("stale")
    );
    assert_eq!(snapshot(&destination)?, installed);
    std::fs::write(
        destination.join("value.rs"),
        b"pub const VALUE: u64 = 99;\n",
    )
    .map_err(|error| error.to_string())?;
    *configuration.get_mut("value").ok_or("value absent")? = json!(42_u64);
    let tampered = report(&run(&source, &host, &executable, &configuration)?, 1)?;
    assert_eq!(
        field(&tampered, "/record/value/comparison/issues/0/problem")?,
        &json!("tampered")
    );
    let damaged = snapshot(&destination)?;
    *configuration.get_mut("action").ok_or("action absent")? = json!("generate");
    let refused = run(&source, &host, &executable, &configuration)?;
    let refusal = report(&refused, 1)?;
    assert_eq!(field(&refusal, "/kind")?, &json!("bake-error"));
    assert_eq!(
        field(&refusal, "/record/cause/kind")?,
        &json!("destination")
    );
    assert_eq!(field(&refusal, "/record/pending_cleanup")?, &json!(false));
    assert_eq!(snapshot(&destination)?, damaged);
    Ok(())
}

pub(super) fn files(report: &Value) -> Result<&Value, String> {
    let path = match field(report, "/record/kind")?.as_str() {
        Some("declared" | "prepared") => "/record/value/files",
        Some("checked" | "generated") => "/record/value/prepared/files",
        other => return Err(format!("no publication files in {other:?}")),
    };
    field(report, path)
}

pub(super) fn executable_path(report: &Value) -> Result<PathBuf, String> {
    let path = field(report, "/record/value/compiler/executable")?;
    assert_eq!(field(path, "/fidelity")?, &json!("unicode"));
    Ok(PathBuf::from(
        field(path, "/shown")?.as_str().ok_or("executable absent")?,
    ))
}

pub(super) fn report(output: &ProcessOutput, exit_code: i32) -> Result<Value, String> {
    assert_eq!(
        output.status().code(),
        Some(exit_code),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    let value: Value = serde_json::from_slice(output.stdout().bytes()).map_err(|error| {
        format!(
            "{error}: {}",
            String::from_utf8_lossy(output.stderr().bytes())
        )
    })?;
    assert_eq!(
        field(&value, "/schema")?,
        &json!("macroonz-presentation-v1")
    );
    assert_eq!(
        field(&value, "/owner")?,
        &json!("macroonz/native_publication/command")
    );
    Ok(value)
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
