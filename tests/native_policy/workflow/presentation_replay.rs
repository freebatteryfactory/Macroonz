//! Non-reproduction retains its specific cause without hiding the current attempt.

use super::{
    fixture::{self, mapped},
    presentation::observed_input,
};
use crate::presentation_formats::{field, parsed};
use macroonz::harness::clock::MeasurementReading;
use macroonz::harness::descriptor::{AuthoredTableName, Provenance};
use macroonz::harness::report::{
    FailureClass, FindingCause, FindingLocation, InfrastructureFailure, InfrastructureFault,
    RunAttempt, SkipReason, TrialConclusion, TrialFinding,
    archive::{ArchiveLimits, retain_capsule},
    replay::compare,
};
use macroonz::harness::runner::{Selection, SelectionPlan, TrialTable};
use macroonz::{presentation, workflow};
use serde_json::json;

#[test]
fn presentation_replay_preserves_each_nonreproduction_cause_and_current_attempt()
-> Result<(), String> {
    let original = fixture::run(&[1])?;
    let capsule = mapped(retain_capsule(
        &fixture::capsule(&original)?,
        ArchiveLimits::declared(65536, 16384),
    ))?;
    for (attempt, expected, current_kind) in [
        (
            RunAttempt::Executed(TrialConclusion::Passed),
            "passed-without-repair-standing",
            "executed",
        ),
        (
            RunAttempt::Executed(TrialConclusion::Refused(TrialFinding::established(
                FailureClass::RefusedByCheck,
                FindingCause::named("different", "failure"),
                FindingLocation::at("different.rs", 9),
                None,
            ))),
            "fingerprint-moved",
            "executed",
        ),
        (
            RunAttempt::SkippedWithReason(SkipReason::PrerequisiteAbsent),
            "did-not-conclude",
            "skipped-with-reason",
        ),
        (RunAttempt::TimedOut, "did-not-conclude", "timed-out"),
        (
            RunAttempt::InfrastructureFailed(InfrastructureFailure::recorded(
                InfrastructureFault::CaptureFailed,
                None,
            )),
            "did-not-conclude",
            "infrastructure-failed",
        ),
    ] {
        let current = observed_input(&[1], attempt, MeasurementReading::Unavailable)?;
        let report = current
            .census()
            .first()
            .and_then(|row| row.disposition().report())
            .ok_or("current report missing")?;
        let comparison = mapped(compare(&capsule, report, original.input()))?;
        fixture::reset();
        let shown = parsed(&presentation::replay_comparison(&comparison))?;
        assert_eq!(fixture::observations(), (0, 0, 0));
        assert_eq!(
            field(&shown, "/record/outcome")?,
            &json!({"kind":"not-reproduced","value":expected})
        );
        assert_eq!(field(&shown, "/record/lineage")?, "original-case");
        let present = parsed(&presentation::trial(report))?;
        assert_eq!(field(&present, "/record/attempt/kind")?, current_kind);
        assert_eq!(field(&shown, "/record/movement/subject")?, "same");
    }
    Ok(())
}

#[test]
fn presentation_replay_keeps_changed_trial_distinct_from_a_changed_fingerprint()
-> Result<(), String> {
    let original = fixture::run(&[1])?;
    let capsule = mapped(retain_capsule(
        &fixture::capsule(&original)?,
        ArchiveLimits::declared(65536, 16384),
    ))?;
    let table = mapped(TrialTable::authored(
        mapped(AuthoredTableName::named("other", "world"))?,
        Provenance::Unproduced,
        vec![fixture::binding(
            "other",
            fixture::revision(),
            fixture::defective,
        )?],
    ))?;
    let current = mapped(workflow::run(
        &table.view(),
        &SelectionPlan::of(Selection::All),
        &fixture::decoder()?,
        &[1],
        fixture::invocation(1),
        fixture::INPUT_LIMITS,
    ))?;
    let comparison = mapped(compare(
        &capsule,
        fixture::selected(&current)?,
        current.input(),
    ))?;
    let shown = parsed(&presentation::replay_comparison(&comparison))?;
    assert_eq!(
        field(&shown, "/record/outcome")?,
        &json!({"kind":"not-reproduced","value":"trial-moved"})
    );
    assert_eq!(field(&shown, "/record/movement/trial")?, "moved");
    Ok(())
}
