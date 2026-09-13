//! A faithful survivor reaches typed staged proof, an unadmitted offer and saved-input replay.

use super::support::{
    CompiledRosterMeaning, OWNER, REVISION_TAG, check, invocation, trial_binding_with,
};
use arbitrary::Unstructured;
use macroonz_harness::descriptor::{
    AuthoredTableName, DerivedRevision, GeneratedSupportSchemaId, MutationPointRef, NamespacedName,
    Origin, Provenance, RevisionBinding, SynthesisFacts, TablePosture,
};
use macroonz_harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, capture_replay, reduce,
};
use macroonz_harness::identity::ContentAddress;
use macroonz_harness::input::{BoundInput, InputBinding, InputLimits, InputProfile, pack};
use macroonz_harness::muterprater::proposal_archive::{
    ArchivedProposalGround, ProposalArchiveLimits, read_proposal, retain_mutant_kill,
};
use macroonz_harness::muterprater::propose::{offer_mutant_kill, prove_candidate, synthesize};
use macroonz_harness::muterprater::{
    CandidateSketch, MutationAssessment, OracleClass, ProposalDestination, ProposalDocument,
    SurvivorExplanation,
};
use macroonz_harness::report::archive::ArchiveLimits;
use macroonz_harness::report::{
    Fingerprint, GenerationProfile, MinimizationProfile, RunAttempt, TrialConclusion,
};
use macroonz_harness::runner::{Invocation, TrialBinding, TrialTable, replay, run_one};
use std::collections::BTreeSet;
use std::io::Read as _;

const INPUT_LIMITS: InputLimits = InputLimits::declared(512, 45);
const ARCHIVE_LIMITS: ProposalArchiveLimits =
    ProposalArchiveLimits::declared(ArchiveLimits::declared(131_072, 65536), 8, 16, 8);

#[derive(Debug)]
struct CapturedResult {
    source: [u8; 32],
    input: [u32; 3],
    meaning: CompiledRosterMeaning,
}

fn decode(source: &mut Unstructured<'_>) -> arbitrary::Result<CapturedResult> {
    let identity = source
        .bytes(32)?
        .try_into()
        .map_err(|_| arbitrary::Error::IncorrectFormat)?;
    let mut input = [0u32; 3];
    for word in &mut input {
        *word = u32::from_be_bytes(
            source
                .bytes(4)?
                .try_into()
                .map_err(|_| arbitrary::Error::IncorrectFormat)?,
        );
    }
    let meaning = match source.bytes(1)? {
        [0] => CompiledRosterMeaning::Unstated,
        [1] => CompiledRosterMeaning::Stated(1),
        _ => return Err(arbitrary::Error::IncorrectFormat),
    };
    Ok(CapturedResult {
        source: identity,
        input,
        meaning,
    })
}

fn decoder() -> Result<InputBinding<CapturedResult>, String> {
    let profile = InputProfile::declared(
        NamespacedName::named(OWNER, "selected-result").map_err(|cause| format!("{cause:?}"))?,
        1,
        ContentAddress::derived(REVISION_TAG, b"source32-input3xbe-result-tag"),
    );
    Ok(InputBinding::declared(
        profile,
        RevisionBinding::derived(DerivedRevision::from_material(include_bytes!(
            "faithful_proposal.rs"
        ))),
        decode,
    ))
}

fn call(invocation: &Invocation<BoundInput<CapturedResult>>) -> TrialConclusion {
    check(&invocation.input().value().meaning)
}

fn candidate() -> Result<TrialBinding<BoundInput<CapturedResult>>, String> {
    trial_binding_with(
        "comparison-behaviour",
        Origin::Candidate(SynthesisFacts::Survivor(
            MutationPointRef::named(OWNER, "comparison-edge")
                .map_err(|cause| format!("{cause:?}"))?,
        )),
        RevisionBinding::declared(ContentAddress::derived(
            REVISION_TAG,
            b"requires-the-result",
        )),
        call,
    )
    .map_err(|cause| format!("{cause:?}"))
}

fn bound(bytes: &[u8]) -> Result<Invocation<BoundInput<CapturedResult>>, String> {
    let decoder = decoder()?;
    let input = decoder
        .decode(pack(decoder.profile(), bytes, INPUT_LIMITS).map_err(|cause| format!("{cause:?}"))?)
        .map_err(|cause| format!("{cause:?}"))?;
    Ok(invocation()
        .map_err(|cause| format!("{cause:?}"))?
        .with_input(input))
}

fn probe(bytes: &[u8]) -> ProbeOutcome {
    let (Ok(binding), Ok(invocation)) = (candidate(), bound(bytes)) else {
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

fn payload(
    assessment: &MutationAssessment<'_, [u32; 3], CompiledRosterMeaning>,
) -> Result<Vec<u8>, String> {
    let execution = assessment.reading().qualification();
    let observed = execution.compiled().observation();
    let mut bytes = execution
        .compiled()
        .selected_content()
        .identity()
        .address()
        .as_bytes()
        .to_vec();
    bytes.extend(observed.input().iter().flat_map(|word| word.to_be_bytes()));
    match execution.selected() {
        CompiledRosterMeaning::Unstated => bytes.push(0),
        CompiledRosterMeaning::Stated(1) => bytes.push(1),
        CompiledRosterMeaning::Stated(_)
        | CompiledRosterMeaning::SetupRefused
        | CompiledRosterMeaning::ReadingRefused(_) => {
            return Err("unexpected captured meaning".to_owned());
        }
    }
    Ok(bytes)
}

pub(super) fn controls(
    weak: &MutationAssessment<'_, [u32; 3], CompiledRosterMeaning>,
    strong: &MutationAssessment<'_, [u32; 3], CompiledRosterMeaning>,
) -> Result<(), String> {
    let binding = candidate()?;
    let row = binding.row();
    let explanation =
        SurvivorExplanation::of(weak.mutation(), OracleClass::GoldenVector, row.check())
            .map_err(|cause| format!("{cause:?}"))?;
    let sketch = CandidateSketch::stated(
        row.execution_suite(),
        row.classification().clone(),
        row.subject(),
        row.population(),
    );
    let synthesized = synthesize(&explanation, &sketch, &BTreeSet::from([row.check()]))
        .map_err(|cause| format!("{cause:?}"))?;
    assert_eq!(&synthesized, row);
    let parent = TrialTable::authored(
        AuthoredTableName::named(OWNER, "faithful-parent").map_err(|cause| format!("{cause:?}"))?,
        Provenance::Unproduced,
        vec![
            trial_binding_with(
                "other-authored-claim",
                Origin::HandWritten,
                decoder()?.revision(),
                call,
            )
            .map_err(|cause| format!("{cause:?}"))?,
        ],
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let bytes = payload(strong)?;
    let invocation = bound(&bytes)?;
    assert_eq!(
        invocation.input().value().source.as_slice(),
        strong
            .reading()
            .qualification()
            .compiled()
            .selected_content()
            .identity()
            .address()
            .as_bytes()
    );
    assert_eq!(invocation.input().value().input, [1, 0, 0]);
    assert_eq!(
        &invocation.input().value().meaning,
        strong.reading().qualification().selected()
    );
    let proof = prove_candidate(&parent, binding, strong.mutation().target(), &invocation)
        .map_err(|cause| format!("{cause:?}"))?;
    assert_eq!(proof.report().denominator(), 2);
    assert_eq!(
        proof.report().posture(),
        TablePosture::Staged {
            parent: parent.name()
        }
    );
    assert_eq!(
        proof.trial_report().standing().key().input(),
        invocation.input_standing()
    );
    let probe = ReductionProbeBinding::bound(
        proof.trial_report(),
        GenerationProfile::declared("captured-selected-result", 1),
        GeneratedSupportSchemaId::over(decoder()?.profile().schema()),
        decoder()?.revision(),
        probe,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let plan = ReductionPlan::declared(
        MinimizationProfile::declared("captured-result", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(1),
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let reduced = reduce(&plan, &bytes, &probe).map_err(|cause| format!("{cause:?}"))?;
    let capsule = capture_replay(&reduced);
    let offered = offer_mutant_kill(
        synthesized,
        strong.mutation(),
        capsule,
        proof,
        Vec::new(),
        ProposalDestination::naming(row_suite()?),
    )
    .map_err(|cause| format!("{cause:?}"))?;
    assert!(matches!(offered.candidate().origin(), Origin::Candidate(_)));
    assert_eq!(parent.bindings().len(), 1);
    let retained =
        retain_mutant_kill(&offered, ARCHIVE_LIMITS).map_err(|cause| format!("{cause:?}"))?;
    let returned = super::archive_process::round_trip(
        retained.encoded(),
        "faithful_proposal::child_replays_unadmitted_proposal",
    )
    .map_err(|cause| format!("{cause:?}"))?;
    assert_eq!(returned, retained.encoded());
    Ok(())
}

fn row_suite() -> Result<macroonz_harness::descriptor::ExecutionSuite, String> {
    macroonz_harness::descriptor::ExecutionSuite::named(OWNER, "review-only")
        .map_err(|cause| format!("{cause:?}"))
}

#[test]
#[ignore = "driven by the faithful survivor proposal crossing"]
fn child_replays_unadmitted_proposal() -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take(131_073)
        .read_to_end(&mut bytes)?;
    let record = read_proposal(&bytes, ARCHIVE_LIMITS).map_err(|cause| format!("{cause:?}"))?;
    let ArchivedProposalGround::MutantKilled(ground) = record.ground() else {
        return Err("unexpected proposal ground".into());
    };
    let replayed = replay(
        ground.capsule(),
        &candidate()?,
        &decoder()?,
        invocation().map_err(|cause| format!("{cause:?}"))?,
        INPUT_LIMITS,
    )
    .map_err(|cause| format!("{cause:?}"))?;
    let report = replayed.report();
    let RunAttempt::Executed(TrialConclusion::Refused(finding)) = report.attempt() else {
        return Err("saved result did not reproduce rejection".into());
    };
    assert_eq!(
        Fingerprint::of(report.trial(), finding).address(),
        ground.rejection().fingerprint().address()
    );
    assert_eq!(
        report.standing().key().address(),
        ground.trial_report().key().address()
    );
    super::archive_process::publish(record.encoded())
}
