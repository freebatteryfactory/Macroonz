//! The observed survivor becomes an unadmitted offer through existing staged proving.

use super::{capture, types::Sample};
use macroonz::harness::descriptor::{
    AuthoredTableName, ExecutionSuite, GeneratedSupportSchemaId, Origin, Provenance, TablePosture,
};
use macroonz::harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, capture_replay, reduce,
};
use macroonz::harness::muterprater::proposal_archive::{
    ArchivedProposal, ProposalArchiveLimits, retain_mutant_kill,
};
use macroonz::harness::muterprater::{
    CandidateSketch, MutationAssessment, OracleClass, ProposalDestination, ProposalDocument,
    SurvivorExplanation, propose,
};
use macroonz::harness::report::archive::ArchiveLimits;
use macroonz::harness::report::{
    Fingerprint, GenerationProfile, MinimizationProfile, RunAttempt, TrialConclusion,
};
use macroonz::harness::runner::{Invocation, TrialTable, run_one};
use std::collections::BTreeSet;

pub(super) const LIMITS: ProposalArchiveLimits =
    ProposalArchiveLimits::declared(ArchiveLimits::declared(131_072, 65_536), 8, 2, 0);

pub(super) fn offer(
    weak: &MutationAssessment<'_, Sample<'_>, u32>,
    strong: &MutationAssessment<'_, Sample<'_>, u32>,
    invocation: &Invocation,
) -> Result<ArchivedProposal, String> {
    let binding = capture::candidate()?;
    let row = binding.row().clone();
    let explanation =
        SurvivorExplanation::of(weak.mutation(), OracleClass::GoldenVector, row.check())
            .map_err(super::debug)?;
    let sketch = CandidateSketch::stated(
        row.execution_suite(),
        row.classification().clone(),
        row.subject(),
        row.population(),
    );
    let synthesized = propose::synthesize(&explanation, &sketch, &BTreeSet::from([row.check()]))
        .map_err(super::debug)?;
    assert_eq!(synthesized, row);
    let parent = TrialTable::authored(
        AuthoredTableName::named("nonzero", "authored-parent").map_err(super::debug)?,
        Provenance::Unproduced,
        vec![capture::parent()?],
    )
    .map_err(super::debug)?;
    let bytes = payload(strong);
    let current = capture::bound(&bytes, invocation.clone())?;
    let observed = strong.reading().qualification();
    assert_eq!(
        &current.input().value().source,
        observed
            .compiled()
            .selected_content()
            .identity()
            .address()
            .as_bytes()
    );
    assert_eq!(
        current.input().value().input,
        observed.compiled().observation().input().value
    );
    assert_eq!(&current.input().value().meaning, observed.selected());
    let proof = propose::prove_candidate(&parent, binding, strong.mutation().target(), &current)
        .map_err(super::debug)?;
    assert_eq!(proof.report().denominator(), 2);
    assert_eq!(
        proof.report().posture(),
        TablePosture::Staged {
            parent: parent.name()
        }
    );
    let probe = ReductionProbeBinding::bound(
        proof.trial_report(),
        GenerationProfile::declared("captured-selected-result", 1),
        GeneratedSupportSchemaId::over(capture::decoder()?.profile().schema()),
        super::checks::revision(b"captured-positive-input-probe-v1"),
        probe,
    )
    .map_err(super::debug)?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("captured-result", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(1),
    )
    .map_err(super::debug)?;
    let reduced = reduce(&plan, &bytes, &probe).map_err(super::debug)?;
    let capsule = capture_replay(&reduced);
    assert_eq!(capsule.input(), bytes);
    let offer = propose::offer_mutant_kill(
        synthesized,
        strong.mutation(),
        capsule,
        proof,
        Vec::new(),
        ProposalDestination::naming(
            ExecutionSuite::named("nonzero", "review-only").map_err(super::debug)?,
        ),
    )
    .map_err(super::debug)?;
    assert!(matches!(offer.candidate().origin(), Origin::Candidate(_)));
    assert_eq!(parent.bindings().len(), 1);
    retain_mutant_kill(&offer, LIMITS).map_err(super::debug)
}

fn payload(assessment: &MutationAssessment<'_, Sample<'_>, u32>) -> Vec<u8> {
    let execution = assessment.reading().qualification();
    let mut bytes = execution
        .compiled()
        .selected_content()
        .identity()
        .address()
        .as_bytes()
        .to_vec();
    bytes.extend(
        execution
            .compiled()
            .observation()
            .input()
            .value
            .to_be_bytes(),
    );
    bytes.extend(execution.selected().to_be_bytes());
    bytes
}

fn probe(bytes: &[u8]) -> ProbeOutcome {
    let (Ok(binding), Ok(invocation)) = (
        capture::candidate(),
        capture::bound(
            bytes,
            super::invocation("saved-result-probe", "declared-current-check"),
        ),
    ) else {
        return ProbeOutcome::NoFailure;
    };
    let report = run_one(&binding, &invocation);
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
