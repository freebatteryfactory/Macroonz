//! Ordinary workflow display preserves original input and admission boundaries.

use super::fixture::{self, mapped};
use crate::presentation_formats::{decoded_hex, field, parsed};
use macroonz::harness::input::{self, InputBinding};
use macroonz::harness::report::archive::{self, ArchiveLimits, RunArchiveLimits};
use macroonz::{presentation, workflow};
use serde_json::{Value, json};

#[test]
fn presentation_input_run_keeps_original_envelope_and_complete_report_without_execution()
-> Result<(), String> {
    let run = fixture::run(&[7, 1, 9])?;
    let before = fixture::observations();
    let shown = parsed(&presentation::input_run(&run))?;
    let report = parsed(&presentation::run(run.report()))?;
    assert_eq!(field(&shown, "/record/report")?, field(&report, "/record")?);
    assert_eq!(
        decoded_hex(field(&shown, "/record/input/payload")?)?,
        [7, 1, 9]
    );
    assert_eq!(
        decoded_hex(field(&shown, "/record/input/encoded")?)?,
        run.input().encoded()
    );
    assert_eq!(
        decoded_hex(field(&shown, "/record/input/case")?)?,
        run.input().case().address().as_bytes()
    );
    assert_eq!(field(&shown, "/record/report/denominator")?, 2_usize);
    assert_eq!(fixture::observations(), before);
    Ok(())
}

#[test]
fn presentation_suite_failure_keeps_first_request_position_without_partial_selection()
-> Result<(), String> {
    let table = fixture::table()?;
    let known = ("workflow-count", "model");
    for (requested, expected) in [
        (vec![], json!({"kind":"empty","value":null})),
        (
            vec![known, ("unknown", "model")],
            json!({"kind":"unknown","value":1_usize}),
        ),
        (
            vec![known, known],
            json!({"kind":"duplicate","value":1_usize}),
        ),
    ] {
        let error = workflow::select_suites(&table.view(), &requested)
            .err()
            .ok_or("invalid selection admitted")?;
        let shown = parsed(&presentation::suite_selection_refusal(&error))?;
        assert_eq!(field(&shown, "/record")?, &expected);
        assert!(shown.pointer("/record/selection").is_none());
    }
    Ok(())
}

#[test]
fn presentation_admission_keeps_decoder_failure_distinct_from_unread_input_and_archive()
-> Result<(), String> {
    let profile = fixture::decoder()?.profile();
    let envelope = mapped(input::pack(profile, &[255], fixture::INPUT_LIMITS))?;
    let decoder = InputBinding::declared(profile, fixture::revision(), |source| {
        source.bytes(2).map(<[u8]>::to_vec)
    });
    let insufficient = decoder
        .decode(envelope.clone())
        .err()
        .ok_or("insufficient input decoded")?;
    let decoder_display = parsed(&presentation::input_refusal(&insufficient))?;
    assert_eq!(field(&decoder_display, "/record/kind")?, "decoder-refused");
    assert_eq!(field(&decoder_display, "/record/value/owner")?, "arbitrary");
    assert_eq!(
        field(&decoder_display, "/record/value/kind")?,
        "NotEnoughData"
    );
    let incomplete = InputBinding::declared(profile, fixture::revision(), |_source| Ok(()));
    let unread = incomplete
        .decode(envelope.clone())
        .err()
        .ok_or("unread input admitted")?;
    let unread_display = parsed(&presentation::input_refusal(&unread))?;
    assert_eq!(
        field(&unread_display, "/record")?,
        &json!({"kind":"trailing-input-bytes","value":1_usize})
    );
    let mut input_bytes = envelope.encoded().to_vec();
    *input_bytes.first_mut().ok_or("missing address")? ^= 1;
    let bad_input = input::read(profile, &input_bytes, fixture::INPUT_LIMITS)
        .err()
        .ok_or("corrupt input read")?;
    let input_display = parsed(&presentation::input_refusal(&bad_input))?;
    let run = fixture::run(&[7, 1, 9])?;
    let limits = RunArchiveLimits::declared(ArchiveLimits::declared(16_384, 8192), 16);
    let report = mapped(archive::retain_run(run.report(), limits))?;
    let mut report_bytes = report.encoded().to_vec();
    *report_bytes.first_mut().ok_or("missing report address")? ^= 1;
    let bad_report = archive::read_run(&report_bytes, limits)
        .err()
        .ok_or("corrupt report read")?;
    let report_display = parsed(&presentation::archive_refusal(&bad_report))?;
    assert_eq!(
        field(&input_display, "/record")?,
        field(&report_display, "/record")?
    );
    assert_eq!(field(&report_display, "/record/value")?, &Value::Null);
    assert_ne!(
        field(&input_display, "/owner")?,
        field(&report_display, "/owner")?
    );
    Ok(())
}
