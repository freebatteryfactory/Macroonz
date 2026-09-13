//! Display controls over actual coverage executions and independently configured failures.

use crate::presentation_formats::{decoded_hex, field, parsed};
use macroonz::harness::fuzz::{
    CoverageAdmission, CoverageCorpus, CoveragePoint, RustcProfileResult,
};
use macroonz::harness::fuzz::{CoverageHostFailure, RustcProfileRefusal};
use macroonz::native_coverage::{
    NativeCoverage, NativeCoverageFailure, NativeCoverageProcessError,
};
use macroonz::native_process::ProcessRun;
use macroonz::presentation::{self, Presentation};
use serde_json::Value;

pub(super) fn result(
    record: &RustcProfileResult,
    candidate: &[u8],
    case: u32,
) -> Result<(), String> {
    let value = parsed(&presentation::coverage_result(record))?;
    assert_eq!(field(&value, "/owner")?, "macroonz-harness/fuzz");
    assert_eq!(field(&value, "/record/case")?, case);
    assert_eq!(decoded_hex(field(&value, "/record/candidate")?)?, candidate);
    let points = field(&value, "/record/points")?
        .as_array()
        .ok_or("missing points")?;
    assert_eq!(points.len(), record.observation().points().len());
    for (shown, original) in points.iter().zip(record.observation().points()) {
        let (source, line) = match original {
            CoveragePoint::Line { source, line } => {
                assert_eq!(field(shown, "/kind")?, "line");
                assert_eq!(field(shown, "/value/branch")?, &Value::Null);
                (source, line)
            }
            CoveragePoint::Branch {
                source,
                line,
                block,
                branch,
            } => {
                assert_eq!(field(shown, "/kind")?, "branch");
                assert_eq!(field(shown, "/value/branch/block")?, *block);
                assert_eq!(field(shown, "/value/branch/branch")?, *branch);
                (source, line)
            }
        };
        assert_eq!(field(shown, "/value/line")?, *line);
        assert_eq!(field(shown, "/value/source/relative")?, source.relative());
        assert_eq!(
            field(shown, "/value/source/root/namespace")?,
            source.root().namespace().written()
        );
        assert_eq!(
            field(shown, "/value/source/root/stem")?,
            source.root().stem().written()
        );
    }
    Ok(())
}

pub(super) fn frontier(
    record: &CoverageCorpus,
    attempted: u32,
    inputs: &[&[u8]],
) -> Result<(), String> {
    let value = parsed(&presentation::coverage_corpus(record))?;
    assert_eq!(field(&value, "/record/attempted_cases")?, attempted);
    assert_eq!(
        field(&value, "/record/attempted_input_bytes")?,
        record.attempted_input_bytes()
    );
    assert_eq!(
        field(&value, "/record/retained_bytes")?,
        record.retained_bytes()
    );
    let retained = field(&value, "/record/interesting")?
        .as_array()
        .ok_or("missing inputs")?;
    assert_eq!(retained.len(), inputs.len());
    for (shown, expected) in retained.iter().zip(inputs) {
        assert_eq!(decoded_hex(shown)?, *expected);
    }
    assert_eq!(
        field(&value, "/record/observed")?
            .as_array()
            .ok_or("missing frontier")?
            .len(),
        record.observed().len()
    );
    assert!(value.pointer("/record/history").is_none());
    assert!(value.pointer("/record/verdict").is_none());
    Ok(())
}

pub(super) fn interesting(record: &CoverageAdmission, expected: &[u8]) -> Result<(), String> {
    let CoverageAdmission::Interesting(input) = record else {
        return Err("expected novel coverage".to_owned());
    };
    assert_eq!(input.as_bytes(), expected);
    let value = parsed(&presentation::coverage_admission(record))?;
    assert_eq!(field(&value, "/record/kind")?, "interesting");
    assert_eq!(decoded_hex(field(&value, "/record/value")?)?, expected);
    Ok(())
}

pub(super) fn readiness(coverage: &NativeCoverage, triple: Option<&str>) -> Result<(), String> {
    let pure = parsed(&presentation::coverage_readiness(coverage.ready()))?;
    let native = parsed(&presentation::native_coverage(coverage))?;
    assert_eq!(field(&native, "/record/ready")?, field(&pure, "/record")?);
    assert_eq!(field(&pure, "/record/release")?, "1.98.1");
    assert_eq!(
        field(&pure, "/record/request/target/triple")?,
        &Value::from(triple)
    );
    assert_eq!(
        field(&pure, "/record/request/target/arguments")?,
        &serde_json::json!([])
    );
    assert_eq!(
        field(&native, "/record/target_limits/stdout")?,
        coverage.target_limits().stdout()
    );
    assert_eq!(
        field(&native, "/record/tool/limits/stdout")?,
        coverage.tool().limits().stdout()
    );
    let environment = field(&native, "/record/tool/environment")?
        .as_array()
        .ok_or("missing environment")?;
    assert_eq!(environment.len(), coverage.tool().environment().len());
    for (shown, (key, value)) in environment.iter().zip(coverage.tool().environment()) {
        assert_eq!(field(shown, "/key")?, key);
        assert_eq!(field(shown, "/value")?, value);
    }
    let request = coverage.ready().request();
    assert_eq!(
        decoded_hex(field(&pure, "/record/request/scratch/bytes")?)?,
        request.scratch().as_os_str().as_encoded_bytes()
    );
    let declared = field(&pure, "/record/request/source_roots")?
        .as_array()
        .ok_or("missing declared roots")?;
    let canonical = field(&pure, "/record/canonical_source_roots")?
        .as_array()
        .ok_or("missing canonical roots")?;
    assert_eq!(declared.len(), request.source_roots().iter().count());
    assert_eq!(
        canonical.len(),
        coverage.ready().source_roots().iter().count()
    );
    for (shown, root) in declared.iter().zip(request.source_roots().iter()) {
        assert_eq!(
            decoded_hex(field(shown, "/checkout/bytes")?)?,
            root.checkout().as_os_str().as_encoded_bytes()
        );
    }
    for (shown, root) in canonical.iter().zip(coverage.ready().source_roots().iter()) {
        assert_eq!(
            decoded_hex(field(shown, "/checkout/bytes")?)?,
            root.checkout().as_os_str().as_encoded_bytes()
        );
    }
    Ok(())
}

pub(super) fn interrupted(shown: &Presentation, role: &str, mode: &str) -> Result<(), String> {
    let value = parsed(shown)?;
    assert_eq!(field(&value, "/record/cause/kind")?, "executor");
    assert_eq!(
        field(&value, "/record/cause/value/case_cleanup")?,
        &Value::Null
    );
    let operation = field(&value, "/record/cause/value/operation")?;
    match role {
        "verbose" => assert_eq!(
            operation,
            &serde_json::json!({"kind":"rustc","value":"verbose-version"})
        ),
        "sysroot" => assert_eq!(
            operation,
            &serde_json::json!({"kind":"rustc","value":"sysroot"})
        ),
        "profdata-version" => assert_eq!(
            operation,
            &serde_json::json!({"kind":"version","value":"profdata"})
        ),
        "cov-version" => assert_eq!(
            operation,
            &serde_json::json!({"kind":"version","value":"cov"})
        ),
        _ => assert_eq!(operation, &serde_json::json!({"kind":role,"value":null})),
    }
    let process = field(&value, "/record/cause/value/error/value/process")?;
    assert_eq!(field(process, "/state")?, "finished");
    assert_eq!(
        field(process, "/stop/kind")?,
        if mode == "deadline" {
            "deadline"
        } else {
            "output-limit"
        }
    );
    if mode != "deadline" {
        let capture = field(process, &format!("/{mode}"))?;
        let bytes = decoded_hex(field(capture, "/bytes")?)?;
        assert!(!bytes.is_empty());
        assert!(bytes.iter().all(|byte| *byte == b'x'));
        assert_eq!(field(capture, "/retained_bytes")?, bytes.len());
    }
    Ok(())
}

pub(super) fn failure(
    record: &NativeCoverageFailure<RustcProfileRefusal>,
    expected: &str,
) -> Result<(), String> {
    let value = parsed(&presentation::coverage_profile_error(record))?;
    assert_eq!(field(&value, "/record/phase")?, "profile-observation");
    assert_eq!(field(&value, "/record/cause/kind")?, "refused");
    assert_eq!(field(&value, "/record/cause/value/kind")?, expected);
    assert_eq!(field(&value, "/record/cleanup_error")?, &Value::Null);
    Ok(())
}

pub(super) fn cleanup(
    record: &NativeCoverageFailure<RustcProfileRefusal>,
    state: &str,
) -> Result<Presentation, String> {
    let shown = presentation::coverage_profile_error(record);
    let value = parsed(&shown)?;
    assert_eq!(
        field(&value, "/record/cause/value/error/value/process/state")?,
        state
    );
    let CoverageHostFailure::Executor {
        error: NativeCoverageProcessError::Execution { run, .. },
        cleanup,
        ..
    } = record.cause()
    else {
        return Err("expected retained process failure".to_owned());
    };
    if let Some(cleanup) = cleanup {
        assert_eq!(
            decoded_hex(field(&value, "/record/cause/value/case_cleanup/bytes")?)?,
            cleanup.directory().as_os_str().as_encoded_bytes()
        );
        assert!(matches!(run.as_ref(), ProcessRun::Pending(_)));
    } else {
        assert_eq!(
            field(&value, "/record/cause/value/case_cleanup")?,
            &Value::Null
        );
        assert!(matches!(run.as_ref(), ProcessRun::Finished(_)));
    }
    Ok(shown)
}
