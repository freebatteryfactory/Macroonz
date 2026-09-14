//! The public Job compiler caller retains independent diagnostics and executed behavior.

use super::configure::{host, root, spelling, target, tool};
use macroonz::native_process::{self, CaptureEnd, ProcessLimits, ProcessOutput, ProcessTool};
use serde_json::Value;
use std::path::Path;
use std::time::Duration;

#[test]
fn public_job_compiler_keeps_exact_refusal_and_executed_lawful_controls() -> Result<(), String> {
    let run = root()?;
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let subject = run.join("job-compiler");
    let package = prepare(repository, &subject, &run)?;
    for (directory, name, arguments) in [
        (
            subject.as_path(),
            "job-lock",
            vec!["generate-lockfile", "--offline"],
        ),
        (
            repository,
            "job-compiler-build",
            vec![
                "build",
                "--example",
                "job_compiler",
                "--no-default-features",
                "--features",
                "native-tooling",
                "--locked",
                "--offline",
                "-j1",
            ],
        ),
    ] {
        let output = crate::check::cargo(directory, repository, &run, name, &arguments)?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let host = host(&run)?;
    let limits = ProcessLimits::informed(
        Duration::from_secs(300),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(debug)?;
    let executable = target()?
        .join("debug/examples")
        .join(format!("job_compiler{}", std::env::consts::EXE_SUFFIX));
    let caller = tool(&host, &executable, &subject, limits)?;
    let configuration = serde_json::json!({
        "cargo": spelling(&host.cargo)?, "directory": spelling(&subject)?,
        "manifest": spelling(&subject.join("Cargo.toml"))?,
        "target_directory": spelling(&target()?)?, "package": package,
        "target": host.triple, "environment": host.environment,
    });
    let output = invoke(&caller, &run, "lawful", &configuration)?;
    assert!(
        output.status().success(),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    assert_eq!(output.stdout().end(), &CaptureEnd::Eof);
    assert_eq!(output.stdout().bytes(), b"lawful: compiled and executed\nwrong-state: E0308 at fixtures/wrong-state.rs:3:26..29\ncontrols: wrong code, shifted span and lawful twin disagree\n");
    let wrong_path = subject.join("fixtures/wrong-state.rs");
    let lawful_path = subject.join("fixtures/lawful-state.rs");
    let wrong = std::fs::read(&wrong_path).map_err(debug)?;
    let lawful = std::fs::read_to_string(&lawful_path).map_err(debug)?;
    std::fs::write(&wrong_path, &lawful).map_err(debug)?;
    let accepted_twin = invoke(&caller, &run, "accepted-twin", &configuration)?;
    refuses(&accepted_twin, "AcceptedWhereRefusalDeclared");
    std::fs::write(&wrong_path, wrong).map_err(debug)?;
    let failed_assertion = lawful.replace("Ok(Stage::Queued)", "Ok(Stage::Done)");
    assert_ne!(failed_assertion, lawful);
    std::fs::write(lawful_path, failed_assertion).map_err(debug)?;
    let failed_read = invoke(&caller, &run, "failed-assertion", &configuration)?;
    refuses(
        &failed_read,
        "lawful execution was not established: ProcessFailure",
    );
    Ok(())
}

pub(crate) fn prepare(repository: &Path, subject: &Path, run: &Path) -> Result<String, String> {
    let example = repository.join("examples/job_compiler");
    std::fs::create_dir_all(subject.join("fixtures")).map_err(debug)?;
    for fixture in ["lawful-state.rs", "wrong-state.rs", "coverage-record.rs"] {
        std::fs::copy(
            example.join("fixtures").join(fixture),
            subject.join("fixtures").join(fixture),
        )
        .map_err(debug)?;
    }
    let suffix = run
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("missing scratch suffix")?;
    let package = format!("neutral-job-adopter-{suffix}");
    let manifest = std::fs::read_to_string(example.join("consumer.toml")).map_err(debug)?;
    let dependency = spelling(repository)?
        .replace('\\', "/")
        .replace('"', "\\\"");
    std::fs::write(
        subject.join("Cargo.toml"),
        manifest
            .replace("/absolute/path/to/Macroonz", &dependency)
            .replace(
                "name = \"neutral-job-adopter\"",
                &format!("name = \"{package}\""),
            ),
    )
    .map_err(debug)?;
    Ok(package)
}

pub(crate) fn invoke(
    caller: &ProcessTool,
    run: &Path,
    name: &str,
    configuration: &Value,
) -> Result<ProcessOutput, String> {
    let input = run.join("job-request.json");
    std::fs::write(&input, serde_json::to_vec(configuration).map_err(debug)?).map_err(debug)?;
    let request = caller.invocation(Vec::new()).map_err(debug)?;
    let output = crate::process::completed(
        native_process::run(&request, Some(std::fs::File::open(input).map_err(debug)?))
            .map_err(debug)?,
    )?;
    std::fs::write(
        run.join(format!("{name}.stdout.log")),
        output.stdout().bytes(),
    )
    .map_err(debug)?;
    std::fs::write(
        run.join(format!("{name}.stderr.log")),
        output.stderr().bytes(),
    )
    .map_err(debug)?;
    Ok(output)
}

fn refuses(output: &ProcessOutput, expected: &str) {
    assert!(!output.status().success());
    assert!(output.stdout().bytes().is_empty());
    let diagnostic = String::from_utf8_lossy(output.stderr().bytes());
    assert!(diagnostic.contains(expected), "{diagnostic}");
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
