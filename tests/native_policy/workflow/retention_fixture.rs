//! Two independently reduced failing rows beside one unselected row.

use super::fixture::{self, mapped};
use macroonz::harness::descriptor::{AuthoredTableName, GeneratedSupportSchemaId, Provenance};
use macroonz::harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, capture_replay, reduce,
};
use macroonz::harness::report::{
    Fingerprint, GenerationProfile, MinimizationProfile, ReplayCapsule, RunAttempt, TrialConclusion,
};
use macroonz::harness::runner::{Selection, SelectionPlan, TrialTable, trial_identity};
use macroonz::workflow::{self, InputRun};

pub(super) fn run(payload: &[u8]) -> Result<InputRun, String> {
    let first = fixture::binding("first", fixture::revision(), fixture::defective)?;
    let second = fixture::binding("second", fixture::revision(), fixture::defective)?;
    let selected = [trial_identity(first.row()), trial_identity(second.row())]
        .into_iter()
        .collect();
    let table = mapped(TrialTable::authored(
        mapped(AuthoredTableName::named("display-retention", "world"))?,
        Provenance::Unproduced,
        vec![
            fixture::binding("unselected", fixture::revision(), fixture::defective)?,
            first,
            second,
        ],
    ))?;
    mapped(workflow::run(
        &table.view(),
        &SelectionPlan::of(Selection::ByTrialIds(selected)),
        &fixture::decoder()?,
        payload,
        fixture::invocation(1),
        fixture::INPUT_LIMITS,
    ))
}

fn probe(payload: &[u8], row: usize) -> ProbeOutcome {
    let Ok(observed) = run(payload) else {
        return ProbeOutcome::NoFailure;
    };
    let Some(report) = observed
        .report()
        .census()
        .get(row)
        .and_then(|entry| entry.disposition().report())
    else {
        return ProbeOutcome::NoFailure;
    };
    match report.attempt() {
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
            ProbeOutcome::Reproduced(Fingerprint::of(report.trial(), finding))
        }
        RunAttempt::Executed(TrialConclusion::Passed)
        | RunAttempt::SkippedWithReason(_)
        | RunAttempt::TimedOut
        | RunAttempt::InfrastructureFailed(_) => ProbeOutcome::NoFailure,
    }
}

fn first_probe(payload: &[u8]) -> ProbeOutcome {
    probe(payload, 1)
}
fn second_probe(payload: &[u8]) -> ProbeOutcome {
    probe(payload, 2)
}

pub(super) fn capsule(original: &InputRun, row: usize) -> Result<ReplayCapsule, String> {
    let report = original
        .report()
        .census()
        .get(row)
        .and_then(|entry| entry.disposition().report())
        .ok_or("missing selected row")?;
    let probe = match row {
        1 => first_probe,
        2 => second_probe,
        _ => return Err("unselected row has no probe".to_owned()),
    };
    let binding = mapped(ReductionProbeBinding::bound(
        report,
        GenerationProfile::declared("workflow-input", 1),
        GeneratedSupportSchemaId::over(fixture::decoder()?.profile().schema()),
        fixture::revision(),
        probe,
    ))?;
    let plan = mapped(ReductionPlan::declared(
        MinimizationProfile::declared("workflow-shrink", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(32),
    ))?;
    Ok(capture_replay(&mapped(reduce(
        &plan,
        original.input().payload(),
        &binding,
    ))?))
}
