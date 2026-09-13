//! Independent report census, adverse outcome and markup observations.

use super::fixture::{self, mapped};
use macroonz::harness::clock::{
    ClockAttribution, ClockFailure, MeasurementReading, RecordedDuration,
};
use macroonz::harness::input;
use macroonz::harness::report::archive::{self, ArchiveLimits, RunArchiveLimits};
use macroonz::harness::report::{
    ForeignText, HostTrialRecord, InfrastructureFailure, InfrastructureFault, RunAttempt,
    RunReport, SkipReason, TrialConclusion,
};
use macroonz::harness::runner;
use macroonz::presentation;
use serde_json::Value;
use std::fmt::Write;

use crate::presentation_formats as formats;
use crate::presentation_formats::field;

fn parsed(value: &presentation::Presentation) -> Result<Value, String> {
    formats::agree(value)?;
    mapped(serde_json::from_str(&value.json()))
}

fn complete(value: &Value, first: &str, second: &str) -> bool {
    value.pointer("/record/denominator").and_then(Value::as_u64) == Some(2)
        && value
            .pointer("/record/census")
            .and_then(Value::as_array)
            .is_some_and(|rows| rows.len() == 2)
        && value
            .pointer("/record/census/0/disposition/value/attempt/value/kind")
            .and_then(Value::as_str)
            == Some("refused")
        && value
            .pointer("/record/census/1/disposition/kind")
            .and_then(Value::as_str)
            == Some("not-selected")
        && value
            .pointer("/record/census/0/trial")
            .and_then(Value::as_str)
            == Some(first)
        && value
            .pointer("/record/census/1/trial")
            .and_then(Value::as_str)
            == Some(second)
}

#[test]
fn presentation_keeps_the_failure_and_complete_denominator() -> Result<(), String> {
    let run = fixture::run(&[1, 2])?;
    let shown = presentation::run(run.report());
    let value = parsed(&shown)?;
    let identities = run
        .report()
        .census()
        .iter()
        .map(|row| {
            let mut identity = String::new();
            for byte in row.trial().address().as_bytes() {
                let _written = write!(identity, "{byte:02x}");
            }
            identity
        })
        .collect::<Vec<_>>();
    let [first, second] = identities.as_slice() else {
        return Err("fixture census changed".to_owned());
    };
    assert!(complete(&value, first, second));
    let mut absent_unselected = value.clone();
    let rows_remaining = absent_unselected
        .pointer_mut("/record/census")
        .and_then(Value::as_array_mut)
        .ok_or("missing census")?;
    rows_remaining.pop();
    assert!(!complete(&absent_unselected, first, second));
    let mut damaged = value.clone();
    let rows = damaged
        .pointer_mut("/record/census")
        .and_then(Value::as_array_mut)
        .ok_or("missing census")?;
    rows.remove(0);
    assert!(!complete(&damaged, first, second));
    for format in [shown.markdown(), shown.html()] {
        assert!(format.contains(first));
        assert!(format.contains(second));
        assert!(format.contains("refused"));
        assert!(format.contains("denominator"));
    }
    assert_eq!(field(&value, "/record/input/name/stem")?, "bytes");
    assert_eq!(field(&value, "/record/input/version")?, 1u32);
    assert_eq!(field(&value, "/record/invocation/bytes")?, 64u64);
    assert_eq!(
        field(
            &value,
            "/record/census/0/disposition/value/measurement/kind"
        )?,
        "unavailable"
    );
    assert_eq!(
        field(
            &value,
            "/record/census/0/disposition/value/attempt/value/value/fingerprint/local"
        )?,
        "length-disagreement"
    );
    Ok(())
}

fn observed(attempt: RunAttempt, measurement: MeasurementReading) -> Result<RunReport, String> {
    observed_input(&[1, 2], attempt, measurement)
}

pub(super) fn observed_input(
    payload: &[u8],
    attempt: RunAttempt,
    measurement: MeasurementReading,
) -> Result<RunReport, String> {
    let table = fixture::table()?;
    let bound = mapped(fixture::decoder()?.decode(mapped(input::pack(
        fixture::decoder()?.profile(),
        payload,
        fixture::INPUT_LIMITS,
    ))?))?;
    let selected = table.bindings().first().ok_or("missing fixture binding")?;
    let host = HostTrialRecord::recorded_with_attribution(
        runner::trial_identity(selected.row()),
        attempt,
        measurement,
        ClockAttribution::Synthetic,
    )
    .with_input(&bound);
    mapped(runner::record_input_all(
        &table.view(),
        &fixture::selection()?,
        &fixture::invocation(1).with_input(bound),
        vec![host],
    ))
}

#[test]
fn infrastructure_prose_cannot_become_a_subject_finding() -> Result<(), String> {
    let hostile = b"passed refused property-disagreement <script>alert('x')</script> | [go](https://bad.invalid) `x` &\n\xff";
    let run = observed(
        RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(
            InfrastructureFault::CaptureFailed,
            Some(ForeignText::admitted(hostile)),
        )),
        MeasurementReading::Failed(ClockFailure::ClosingRefused),
    )?;
    let shown = presentation::run(&run);
    formats::agree(&shown)?;
    let value = parsed(&shown)?;
    let attempt = field(&value, "/record/census/0/disposition/value/attempt")?;
    assert_eq!(field(attempt, "/kind")?, "infrastructure-failed");
    assert_eq!(field(attempt, "/value/fault")?, "capture-failed");
    assert_eq!(
        field(attempt, "/value/foreign/fidelity")?,
        "lossy-replacement"
    );
    assert!(attempt.pointer("/value/fingerprint").is_none());
    assert_eq!(
        field(attempt, "/value/foreign/shown")?,
        String::from_utf8_lossy(hostile).as_ref()
    );
    assert_eq!(
        field(
            &value,
            "/record/census/0/disposition/value/measurement/value"
        )?,
        "closing-refused"
    );
    let html = shown.html();
    assert!(!html.contains("<script>"));
    assert!(html.contains("&lt;script&gt;"));
    assert!(!html.contains("<a "));
    let markdown = shown.markdown();
    assert!(!markdown.contains("[go](https://bad.invalid)"));
    assert!(markdown.contains("\\|"));
    assert!(markdown.contains("&lt;script&gt;"));
    assert!(markdown.contains("\\`x\\`"));
    let saved = mapped(archive::retain_run(
        &run,
        RunArchiveLimits::declared(ArchiveLimits::declared(65536, 16384), 2),
    ))?;
    let loaded = mapped(archive::read_run(
        saved.encoded(),
        RunArchiveLimits::declared(ArchiveLimits::declared(65536, 16384), 2),
    ))?;
    let historical = parsed(&presentation::archived_run(&loaded))?;
    assert_eq!(
        field(&historical, "/standing")?,
        "historical-unauthenticated"
    );
    assert_eq!(
        field(&historical, "/record/census/0/disposition/value/attempt")?,
        attempt
    );
    assert_eq!(
        field(&historical, "/record/input")?,
        field(&value, "/record/input")?
    );
    Ok(())
}

#[test]
fn each_attempt_and_measurement_remains_distinct() -> Result<(), String> {
    for (attempt, expected) in [
        (RunAttempt::Executed(TrialConclusion::Passed), "executed"),
        (
            RunAttempt::SkippedWithReason(SkipReason::PrerequisiteAbsent),
            "skipped-with-reason",
        ),
        (RunAttempt::TimedOut, "timed-out"),
    ] {
        let run = observed(
            attempt,
            MeasurementReading::Observed(RecordedDuration::recorded(0)),
        )?;
        let value = parsed(&presentation::run(&run))?;
        assert_eq!(
            field(&value, "/record/census/0/disposition/value/attempt/kind")?,
            expected
        );
        assert_eq!(
            field(
                &value,
                "/record/census/0/disposition/value/measurement/kind"
            )?,
            "observed"
        );
        assert_eq!(
            field(
                &value,
                "/record/census/0/disposition/value/measurement/value"
            )?,
            0u64
        );
        assert_eq!(
            field(
                &value,
                "/record/census/0/disposition/value/clock_attribution"
            )?,
            "synthetic"
        );
    }
    Ok(())
}

#[test]
fn presentation_retains_exact_truncation_and_a_split_utf8_sequence() -> Result<(), String> {
    let mut offered = vec![b'A'; 4095];
    offered.extend_from_slice("é!".as_bytes());
    let run = observed(
        RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(
            InfrastructureFault::CaptureFailed,
            Some(ForeignText::admitted(&offered)),
        )),
        MeasurementReading::Unavailable,
    )?;
    let saved = mapped(archive::retain_run(
        &run,
        RunArchiveLimits::declared(ArchiveLimits::declared(65536, 16384), 2),
    ))?;
    for shown in [presentation::run(&run), presentation::archived_run(&saved)] {
        let value = parsed(&shown)?;
        let text = field(
            &value,
            "/record/census/0/disposition/value/attempt/value/foreign",
        )?;
        assert_eq!(
            field(text, "/truncation")?,
            &serde_json::json!({
                "kind":"truncated-at","value":{"admitted":4096usize,"offered":4098usize},
            })
        );
        assert_eq!(field(text, "/fidelity")?, "lossy-replacement");
        assert_eq!(
            field(text, "/bytes")?,
            format!("{}c3", "41".repeat(4095)).as_str()
        );
        assert_eq!(
            field(text, "/shown")?,
            format!("{}�", "A".repeat(4095)).as_str()
        );
    }
    Ok(())
}

#[test]
fn capsule_projection_preserves_the_reached_bytes_without_execution() -> Result<(), String> {
    let run = fixture::run(&[1, 2])?;
    let capsule = fixture::capsule(&run)?;
    let saved = mapped(archive::retain_capsule(
        &capsule,
        ArchiveLimits::declared(65536, 16384),
    ))?;
    fixture::reset();
    let current = parsed(&presentation::capsule(&capsule))?;
    let historical = parsed(&presentation::archived_capsule(&saved))?;
    assert_eq!(fixture::observations(), (0, 0, 0));
    assert_eq!(field(&current, "/record/input")?, "01");
    for key in [
        "identity",
        "key",
        "fingerprint",
        "input",
        "generation",
        "minimization",
        "schema",
        "posture",
    ] {
        let path = format!("/record/{key}");
        assert_eq!(field(&current, &path)?, field(&historical, &path)?);
    }
    assert_ne!(
        field(&current, "/standing")?,
        field(&historical, "/standing")?
    );
    Ok(())
}
