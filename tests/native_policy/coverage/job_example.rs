//! The Job coverage caller preserves fixture-only novelty and independent accounting.

use crate::compiler::configure::{host, root, spelling, target, tool};
use crate::compiler::job_example::{invoke, prepare};
use macroonz::native_process::{CaptureEnd, ProcessLimits};
use serde_json::{Value, json};
use std::path::Path;
use std::time::Duration;

#[test]
fn public_job_coverage_preserves_source_scope_novelty_and_budgets() -> Result<(), String> {
    let run = root()?;
    let repository = Path::new(env!("CARGO_MANIFEST_DIR"));
    let subject = run.join("job-coverage");
    let package = prepare(repository, &subject, &run)?;
    for (directory, name, arguments) in [
        (
            subject.as_path(),
            "coverage-lock",
            vec!["generate-lockfile", "--offline"],
        ),
        (
            repository,
            "job-coverage-build",
            vec![
                "build",
                "--example",
                "job_coverage",
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
        .join(format!("job_coverage{}", std::env::consts::EXE_SUFFIX));
    let caller = tool(&host, &executable, &subject, limits)?;
    let mut configuration = json!({
        "cargo": spelling(&host.cargo)?, "rustc": spelling(&host.rustc)?,
        "directory": spelling(&subject)?, "manifest": spelling(&subject.join("Cargo.toml"))?,
        "target_directory": spelling(&target()?)?, "package": package,
        "target": host.triple, "environment": host.environment,
        "declaration": spelling(&repository.join("examples/job_workflow/declaration.rs"))?,
        "scratch": spelling(&run.join("cases"))?,
    });
    let output = invoke(&caller, &run, "coverage", &configuration)?;
    assert!(
        output.status().success(),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    assert_eq!(output.stdout().end(), &CaptureEnd::Eof);
    report(output.stdout().bytes(), &host.triple)?;
    let source = subject.join("fixtures/coverage-record.rs");
    let original = std::fs::read_to_string(&source).map_err(debug)?;
    let damaged = original.replace(
        "Record::decode_canonical(&bytes)",
        "Record::decode_canonical(&[0])",
    );
    assert_ne!(damaged, original);
    std::fs::write(source, damaged).map_err(debug)?;
    let scratch = configuration
        .get_mut("scratch")
        .ok_or("missing scratch field")?;
    *scratch = Value::String(spelling(&run.join("damaged-cases"))?);
    let refused = invoke(&caller, &run, "lost-novelty", &configuration)?;
    assert!(!refused.status().success());
    assert!(refused.stdout().bytes().is_empty());
    let diagnostic = String::from_utf8_lossy(refused.stderr().bytes());
    assert!(
        diagnostic.contains("unexpected novelty at 1: Known"),
        "{diagnostic}"
    );
    Ok(())
}

fn report(bytes: &[u8], target: &str) -> Result<(), String> {
    let rows = std::str::from_utf8(bytes)
        .map_err(debug)?
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(debug)?;
    let [ready, corpus] = rows.as_slice() else {
        return Err("expected readiness and corpus records".to_owned());
    };
    assert_eq!(ready.pointer("/record/release"), Some(&json!("1.98.1")));
    assert_eq!(
        ready.pointer("/record/request/target/triple"),
        Some(&json!(target))
    );
    for (path, expected) in [
        ("/record/attempted_cases", json!(3u32)),
        ("/record/attempted_input_bytes", json!(18u64)),
        ("/record/retained_bytes", json!(17u64)),
        (
            "/record/interesting",
            json!(["00", "00000000000002010000000000000007"]),
        ),
    ] {
        assert_eq!(corpus.pointer(path), Some(&expected));
    }
    let points = corpus
        .pointer("/record/observed")
        .and_then(Value::as_array)
        .ok_or("missing points")?;
    assert!(!points.is_empty());
    for point in points {
        assert_eq!(
            point.pointer("/value/source/relative"),
            Some(&json!("fixtures/coverage-record.rs"))
        );
    }
    Ok(())
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
