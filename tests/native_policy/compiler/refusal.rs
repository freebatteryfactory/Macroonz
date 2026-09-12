use super::configure::{anchor, bounds, finish, host, locus, root, tool};
use super::types::Host;
use macroonz::harness::oracle::{DeclaredBehavior, DeclaredCompilation, RelativeSourcePath};
use macroonz::native_compiler::{
    self, CargoFixture, CargoTarget, CompilerError, CompilerObservationError, CompilerRequest,
    ReadBackError,
};
use macroonz::native_process::{ProcessError, ProcessLimits, ProcessStop};
use std::path::{Path, PathBuf};
use std::time::Duration;

const DIAGNOSTIC: &str = r#"{"$message_type":"diagnostic","level":"error","code":{"code":"E0308"},"spans":[{"file_name":"fixture.rs","line_start":2,"line_end":2,"column_start":21,"column_end":28,"is_primary":true}]}"#;

#[test]
fn compiler_path_admission_refuses_ambiguous_roots_and_outputs_before_execution()
-> Result<(), String> {
    let root = root()?;
    for (directory, artifact) in [
        (root.join(".").join("source"), root.join("output")),
        (root.clone(), root.join(".").join("output")),
        (root.clone(), PathBuf::from("relative-output")),
    ] {
        let selected = macroonz::native_process::ProcessTool::informed(
            root.join("missing-tool"),
            directory,
            Vec::new(),
            bounds()?,
            &[],
        )
        .map_err(|error| error.to_string())?;
        assert!(matches!(
            CompilerRequest::rustc(&selected, locus()?, artifact, "wasm32-unknown-unknown"),
            Err(CompilerError::Configuration(_))
        ));
    }
    for manifest in [
        PathBuf::from("Cargo.toml"),
        root.join(".").join("Cargo.toml"),
    ] {
        assert!(matches!(
            CargoFixture::informed(
                manifest,
                root.join("target"),
                "fixture".to_owned(),
                CargoTarget::Library,
                "wasm32-unknown-unknown".to_owned()
            ),
            Err(CompilerError::Configuration(_))
        ));
    }
    Ok(())
}

#[test]
fn malformed_ambiguous_and_foreign_diagnostics_do_not_establish_refusal() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let second_span = r#",{"file_name":"fixture.rs","line_start":2,"line_end":2,"column_start":30,"column_end":32,"is_primary":true}]"#;
    let cases = [
        ("not-json".to_owned(), "json"),
        (format!("{DIAGNOSTIC} trailing"), "json"),
        (
            DIAGNOSTIC.replace(
                "\"code\":\"E0308\"",
                "\"code\":\"E0308\",\"code\":\"E0277\"",
            ),
            "json",
        ),
        (format!("{DIAGNOSTIC}\n{DIAGNOSTIC}"), "diagnostics"),
        (DIAGNOSTIC.replace("true}", "false}"), "diagnostics"),
        (DIAGNOSTIC.replace(']', second_span), "spans"),
        (DIAGNOSTIC.replace("fixture.rs", "../fixture.rs"), "source"),
        (
            DIAGNOSTIC.replace("\"line_start\":2", "\"line_start\":0"),
            "source",
        ),
        (DIAGNOSTIC.replace("{\"code\":\"E0308\"}", "null"), "code"),
    ];
    for (at, (payload, category)) in cases.iter().enumerate() {
        let executable = standin(
            &root,
            &host,
            &format!("diagnostic-{at}"),
            &writer(payload, "stderr", 1),
        )?;
        let request = CompilerRequest::rustc(
            &tool(&host, &executable, &root, bounds()?)?,
            locus()?,
            root.join("observed-artifact"),
            &host.triple,
        )
        .map_err(|error| error.to_string())?;
        let output =
            finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
        let Err(error) = output.observed() else {
            return Err("hostile output established an observation".to_owned());
        };
        let matches = match *category {
            "json" => matches!(error, CompilerObservationError::InvalidJson { .. }),
            "diagnostics" => matches!(error, CompilerObservationError::DiagnosticCount(_)),
            "spans" => matches!(error, CompilerObservationError::PrimarySpanCount(2)),
            "source" => matches!(error, CompilerObservationError::Source(_)),
            "code" => error == &CompilerObservationError::UncodedDiagnostic,
            _ => false,
        };
        assert!(matches, "{category}: {output:?}");
    }
    let executable = standin(
        &root,
        &host,
        "future-fields",
        &writer(
            &DIAGNOSTIC.replace(
                "\"level\"",
                "\"extra\":{\"future\":[null,1.25,\"\\uD83D\\uDE00\"]},\"level\"",
            ),
            "stderr",
            1,
        ),
    )?;
    let request = CompilerRequest::rustc(
        &tool(&host, &executable, &root, bounds()?)?,
        locus()?,
        root.join("observed-artifact"),
        &host.triple,
    )
    .map_err(|error| error.to_string())?;
    let output = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    let expected = anchor("E0308", 28)?;
    assert_eq!(
        output.observed().map(|value| value.refusal()),
        Ok(Some(&expected))
    );
    Ok(())
}

#[test]
fn process_failures_and_exhausted_resources_never_become_expected_diagnostics() -> Result<(), String>
{
    let root = root()?;
    let host = host(&root)?;
    let missing = CompilerRequest::rustc(
        &tool(&host, &root.join("missing-tool"), &root, bounds()?)?,
        locus()?,
        root.join("output"),
        &host.triple,
    )
    .map_err(|error| error.to_string())?;
    assert!(matches!(
        native_compiler::compile(&missing),
        Err(CompilerError::Process(ProcessError::Start(_)))
    ));
    let failure = standin(
        &root,
        &host,
        "process-failure",
        &writer(DIAGNOSTIC, "stderr", 7),
    )?;
    let failing_request = CompilerRequest::rustc(
        &tool(&host, &failure, &root, bounds()?)?,
        locus()?,
        root.join("output"),
        &host.triple,
    )
    .map_err(|error| error.to_string())?;
    let failed_output =
        finish(native_compiler::compile(&failing_request).map_err(|error| error.to_string())?)?;
    assert_eq!(
        failed_output.observed().err(),
        Some(&CompilerObservationError::ProcessFailure)
    );
    for (name, body, stop) in [
        (
            "timeout",
            "fn main() { std::thread::sleep(std::time::Duration::from_secs(10)); }".to_owned(),
            ProcessStop::Deadline,
        ),
        (
            "flood",
            writer(&"x".repeat(131_072), "stderr", 1),
            ProcessStop::OutputLimit,
        ),
    ] {
        let executable = standin(&root, &host, name, &body)?;
        let limits = ProcessLimits::informed(
            Duration::from_millis(500),
            Duration::from_secs(3),
            1024,
            1024,
        )
        .map_err(|error| error.to_string())?;
        let request = CompilerRequest::rustc(
            &tool(&host, &executable, &root, limits)?,
            locus()?,
            root.join("output"),
            &host.triple,
        )
        .map_err(|error| error.to_string())?;
        let output =
            finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
        assert_eq!(
            output.observed().err(),
            Some(&CompilerObservationError::Interrupted(stop))
        );
        assert!(
            output
                .compared(&DeclaredCompilation::refuses(anchor("E0308", 28)?))
                .is_err()
        );
    }
    Ok(())
}

#[test]
fn cargo_completion_and_selected_artifact_are_required_for_success() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    for (at, payload) in [
        "",
        "{\"reason\":\"build-finished\",\"success\":true}",
        "{\"reason\":\"build-finished\",\"success\":false}",
    ]
    .iter()
    .enumerate()
    {
        let executable = standin(
            &root,
            &host,
            &format!("cargo-protocol-{at}"),
            &writer(payload, "stdout", 0),
        )?;
        let fixture = CargoFixture::informed(
            root.join("Cargo.toml"),
            root.join("target"),
            "fixture".to_owned(),
            CargoTarget::Binary("reader".to_owned()),
            host.triple.clone(),
        )
        .map_err(|error| error.to_string())?;
        let request = CompilerRequest::cargo(
            &tool(&host, &executable, &root, bounds()?)?,
            fixture,
            locus()?,
        )
        .map_err(|error| error.to_string())?;
        let output =
            finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
        assert!(matches!(
            output.observed(),
            Err(CompilerObservationError::Protocol(_))
        ));
    }
    Ok(())
}

#[test]
fn cargo_diagnostics_must_belong_to_the_selected_manifest_and_target() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    let message: serde_json::Value =
        serde_json::from_str(DIAGNOSTIC).map_err(|error| error.to_string())?;
    for (at, (manifest, name, kind)) in [
        (root.join("foreign.toml"), "reader", "bin"),
        (root.join("Cargo.toml"), "foreign", "bin"),
        (root.join("Cargo.toml"), "reader", "example"),
    ]
    .into_iter()
    .enumerate()
    {
        let row = serde_json::json!({
            "reason": "compiler-message",
            "manifest_path": manifest.to_str().ok_or("non-Unicode manifest")?,
            "target": { "kind": [kind], "name": name },
            "message": message,
        });
        let payload = format!("{row}\n{{\"reason\":\"build-finished\",\"success\":false}}");
        let executable = standin(
            &root,
            &host,
            &format!("cargo-selection-{at}"),
            &writer(&payload, "stdout", 101),
        )?;
        let fixture = CargoFixture::informed(
            root.join("Cargo.toml"),
            root.join("target"),
            "fixture".to_owned(),
            CargoTarget::Binary("reader".to_owned()),
            host.triple.clone(),
        )
        .map_err(|error| error.to_string())?;
        let request = CompilerRequest::cargo(
            &tool(&host, &executable, &root, bounds()?)?,
            fixture,
            locus()?,
        )
        .map_err(|error| error.to_string())?;
        let output =
            finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
        assert_eq!(
            output.observed().err(),
            Some(&CompilerObservationError::ProcessFailure)
        );
    }
    Ok(())
}

#[test]
fn read_back_failures_do_not_invoke_the_semantic_decoder() -> Result<(), String> {
    let root = root()?;
    let host = host(&root)?;
    for (name, body, expected) in [
        (
            "reader-failure",
            writer("42", "stdout", 7),
            ReadBackError::ProcessFailure,
        ),
        (
            "reader-flood",
            writer(&"4".repeat(131_072), "stdout", 0),
            ReadBackError::Interrupted(ProcessStop::OutputLimit),
        ),
    ] {
        let executable = standin(&root, &host, name, &body)?;
        let limits =
            ProcessLimits::informed(Duration::from_secs(5), Duration::from_secs(3), 1024, 1024)
                .map_err(|error| error.to_string())?;
        let request = tool(&host, &executable, &root, limits)?
            .invocation(Vec::new())
            .map_err(|error| error.to_string())?;
        let output = super::super::process::completed(
            macroonz::native_process::run(&request, None).map_err(|error| error.to_string())?,
        )?;
        let called = std::cell::Cell::new(false);
        let compared = native_compiler::compared_read_back(
            &output,
            |_| {
                called.set(true);
                Ok(Vec::new())
            },
            &DeclaredBehavior::RefusedByCompiler,
        );
        assert_eq!(compared, Err(expected));
        assert!(!called.get());
    }
    Ok(())
}

fn writer(payload: &str, stream: &str, code: u8) -> String {
    format!(
        "use std::io::Write;\nfn main() -> std::process::ExitCode {{\n    if std::io::{stream}().write_all({payload:?}.as_bytes()).is_err() {{ return std::process::ExitCode::from(99u8); }}\n    std::process::ExitCode::from({code}u8)\n}}\n"
    )
}

fn standin(root: &Path, host: &Host, name: &str, source: &str) -> Result<PathBuf, String> {
    let filename = format!("{name}.rs");
    std::fs::write(root.join(&filename), source).map_err(|error| error.to_string())?;
    let executable = root.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let source_path =
        RelativeSourcePath::informed(&filename).map_err(|error| format!("{error:?}"))?;
    let request = CompilerRequest::rustc(
        &tool(host, &host.rustc, root, bounds()?)?,
        source_path,
        executable.clone(),
        &host.triple,
    )
    .map_err(|error| error.to_string())?;
    let output = finish(native_compiler::compile(&request).map_err(|error| error.to_string())?)?;
    if output.observed().is_err()
        || output
            .observed()
            .is_ok_and(|value| value.refusal().is_some())
    {
        return Err(format!("stand-in failed to compile: {output:?}"));
    }
    Ok(executable)
}
