//! Outside claims over proposal offers, review custody, replay custody, admission, and proposal identity bytes.

use super::interpretation::interpreted_survivor;
use super::{
    MutationRoadFailure, REPLAY_SCHEMA_TAG, foreign_invocation, interpreted_kill, invocation,
    trial_binding_for,
};
use macroonz_harness::depot::capsules::{
    ReplayCapsuleEntry, ReplayDepotRefusal, ReplayDepotSink, StoredReplayEntryRef,
};
use macroonz_harness::descriptor::{
    AdmissionGround, AuthoredTableName, AuthoredTableRefusal, Binding, CheckRef, ClaimRef,
    Classification, ExecutableAttachment, ExecutionSuite, GeneratedSupportSchemaId,
    MutationPointRef, Origin, PopulationRef, ProposalId, Provenance, ReplayRef, RevisionBinding,
    Role, Row, SubjectRoute, SynthesisFacts, Tag, TrialTableRefusal,
};
use macroonz_harness::generate::{
    ByteReducerId, FingerprintPreservation, ProbeOutcome, ReductionBudget, ReductionPlan,
    ReductionProbeBinding, capture_replay, reduce,
};
use macroonz_harness::identity::{ContentAddress, encode_bytes};
use macroonz_harness::muterprater::propose::{
    human_admit_discharge, human_admit_replay, offer_claim_pin, offer_mutant_kill,
    offer_obligation_discharge, openings, pin_delta, prove_candidate, route, synthesize,
};
use macroonz_harness::muterprater::{
    CandidateSketch, Demonstration, DischargeEvidence, DischargeProposalRefusal, DuplicateRefusal,
    ExplanationRefusal, HumanAdmissionRefusal, IntendedRejection, KillProposalRefusal,
    MutantKilledProposal, MutationOutcome, MutationReport, NoComparisonReason, ObligationLane,
    OracleClass, OwedClaim, OwedDeclaration, PROPOSAL_TAG, ProofDelta, ProofShape,
    ProposalDestination, ProposalDocument, ProposalSink, ReplayBearingProposal, SinkRefusal,
    StoredProposalRef, SurvivorExplanation, SynthesisRefusal,
};
use macroonz_harness::report::{
    ClaimCoverage, Fingerprint, GenerationProfile, MinimizationProfile, ReplayCapsule,
    TrialConclusion, claim_coverage,
};
use macroonz_harness::runner::{
    Invocation, Selection, SelectionPlan, TrialBinding, TrialTable, run_all, trial_identity,
};
use std::collections::BTreeSet;

#[derive(Default)]
struct ReviewSink {
    proposals: Vec<ProposalId>,
}

impl ProposalSink for ReviewSink {
    fn store<Document: ProposalDocument>(
        &mut self,
        proposal: &Document,
    ) -> Result<StoredProposalRef, SinkRefusal> {
        let identity = proposal.identity();
        self.proposals.push(identity);
        StoredProposalRef::at(identity, "review://mutation-proposal")
    }
}

#[derive(Default)]
struct ReplayDepot {
    entries: Vec<ReplayCapsuleEntry>,
}

impl ReplayDepotSink for ReplayDepot {
    fn store(
        &mut self,
        entry: &ReplayCapsuleEntry,
    ) -> Result<StoredReplayEntryRef, ReplayDepotRefusal> {
        self.entries.push(entry.clone());
        StoredReplayEntryRef::at(entry.replay(), "depot://mutation-replay")
    }
}

struct MismatchedReplayDepot {
    entries: Vec<ReplayCapsuleEntry>,
    found: ReplayRef,
}

impl ReplayDepotSink for MismatchedReplayDepot {
    fn store(
        &mut self,
        entry: &ReplayCapsuleEntry,
    ) -> Result<StoredReplayEntryRef, ReplayDepotRefusal> {
        self.entries.push(entry.clone());
        StoredReplayEntryRef::at(self.found, "depot://foreign-replay")
    }
}

struct KillFixture {
    proposal: MutantKilledProposal,
    capsule: ReplayCapsule,
    demonstration: Demonstration,
}

fn candidate_trial_call(_invocation: &Invocation) -> TrialConclusion {
    super::check(&super::CompiledRosterMeaning::Unstated)
}

fn candidate_binding(point: MutationPointRef) -> Result<TrialBinding, TrialTableRefusal> {
    let subject = SubjectRoute::named(super::OWNER, "comparison-subject")?;
    let check_ref = CheckRef::named(super::OWNER, "comparison-check")?;
    let row = Row::declared(
        ClaimRef::named(super::OWNER, "comparison-behaviour")?,
        ExecutionSuite::named(super::OWNER, "mutation-receiver")?,
        Classification::authored(
            vec![Role::named(super::OWNER, "mutation")?],
            vec![Tag::named(super::OWNER, "outside-consumer")?],
        )?,
        subject,
        check_ref,
        PopulationRef::named(super::OWNER, "one-input")?,
        Origin::Candidate(SynthesisFacts::Survivor(point)),
    )?;
    let revision =
        RevisionBinding::declared(ContentAddress::derived(super::REVISION_TAG, b"trial"));
    Binding::bound(
        row,
        ExecutableAttachment::attached(
            subject,
            check_ref,
            revision,
            revision,
            candidate_trial_call,
        ),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)
}

fn authored_parent() -> Result<TrialTable, TrialTableRefusal> {
    TrialTable::authored(
        AuthoredTableName::named(super::OWNER, "mutation-parent")?,
        Provenance::Unproduced,
        vec![trial_binding_for("parent-behaviour")?],
    )
    .map_err(TrialTableRefusal::TableNotAuthored)
}

fn replay_probe(_input: &[u8]) -> ProbeOutcome {
    let Ok(point) = MutationPointRef::named(super::OWNER, "comparison-edge") else {
        return ProbeOutcome::NoFailure;
    };
    let Ok(binding) = candidate_binding(point) else {
        return ProbeOutcome::NoFailure;
    };
    let Ok(invocation) = invocation() else {
        return ProbeOutcome::NoFailure;
    };
    let TrialConclusion::Refused(finding) = candidate_trial_call(&invocation) else {
        return ProbeOutcome::NoFailure;
    };
    ProbeOutcome::Reproduced(Fingerprint::of(trial_identity(binding.row()), &finding))
}

fn demonstrate_mutation(
    mutation: &MutationReport,
) -> Result<(Row, Demonstration), MutationRoadFailure> {
    let Some(point) = mutation.target().identity().point() else {
        return Err(MutationRoadFailure::MissingActiveSelection);
    };
    let candidate = candidate_binding(point)?;
    let candidate_key = candidate.trial_key();
    let candidate_row = candidate.row().clone();
    assert!(matches!(
        TrialTable::authored(
            AuthoredTableName::named(super::OWNER, "candidate-cannot-enter")
                .map_err(|_| MutationRoadFailure::Name)?,
            Provenance::Unproduced,
            vec![candidate.clone()],
        ),
        Err(AuthoredTableRefusal::CandidateOrigin(found)) if found == candidate_key
    ));
    let demonstration = prove_candidate(
        &authored_parent()?,
        candidate,
        mutation.target(),
        &invocation()?,
    )?;
    let mutation_fingerprint = match mutation.outcome() {
        MutationOutcome::Killed(IntendedRejection::Demonstrated(rejection)) => {
            rejection.fingerprint()
        }
        MutationOutcome::Killed(IntendedRejection::ReportedByBackend { stated: _ })
        | MutationOutcome::Survived
        | MutationOutcome::Inconclusive(_) => {
            return Err(MutationRoadFailure::MissingActiveSelection);
        }
    };
    assert_eq!(
        demonstration.rejection().fingerprint(),
        mutation_fingerprint
    );
    Ok((candidate_row, demonstration))
}

fn capture_demonstration(
    demonstration: &Demonstration,
) -> Result<ReplayCapsule, MutationRoadFailure> {
    capture_demonstration_under(demonstration, "mutation-replay")
}

fn capture_demonstration_under(
    demonstration: &Demonstration,
    generation: &'static str,
) -> Result<ReplayCapsule, MutationRoadFailure> {
    let reduction_binding = ReductionProbeBinding::bound(
        demonstration.trial_report(),
        GenerationProfile::declared(generation, 1),
        GeneratedSupportSchemaId::over(ContentAddress::derived(
            REPLAY_SCHEMA_TAG,
            b"mutation-replay-schema",
        )),
        RevisionBinding::declared(ContentAddress::derived(
            super::REVISION_TAG,
            b"replay-probe",
        )),
        replay_probe,
    )?;
    let reduction_plan = ReductionPlan::declared(
        MinimizationProfile::declared("mutation-replay", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        Vec::new(),
        FingerprintPreservation::Required,
        ReductionBudget::declared(1),
    )?;
    let reduction = reduce(&reduction_plan, &[1u8, 2u8], &reduction_binding)?;
    let capsule = capture_replay(&reduction);
    assert_eq!(capsule.key(), demonstration.trial_report().standing().key());
    assert_eq!(
        capsule.fingerprint(),
        demonstration.rejection().fingerprint()
    );
    Ok(capsule)
}

fn reject_capsule_for_another_execution(
    candidate: &Row,
    mutation: &MutationReport,
    demonstration: &Demonstration,
    destination: ProposalDestination,
) -> Result<(), MutationRoadFailure> {
    let Some(point) = mutation.target().identity().point() else {
        return Err(MutationRoadFailure::MissingActiveSelection);
    };
    let foreign_demonstration = prove_candidate(
        &authored_parent()?,
        candidate_binding(point)?,
        mutation.target(),
        &foreign_invocation(),
    )?;
    let foreign_capsule = capture_demonstration(&foreign_demonstration)?;
    let replay = foreign_capsule.key().address();
    let expected = demonstration.trial_report().standing().key().address();
    assert!(matches!(
        offer_mutant_kill(
            candidate.clone(),
            mutation,
            foreign_capsule,
            demonstration.clone(),
            Vec::new(),
            destination,
        ),
        Err(KillProposalRefusal::ReplayExecutionMismatch {
            replay: found,
            demonstration: required,
        }) if found == replay && required == expected
    ));
    Ok(())
}

fn offered_kill(mutation: &MutationReport) -> Result<KillFixture, MutationRoadFailure> {
    let (candidate, demonstration) = demonstrate_mutation(mutation)?;
    let capsule = capture_demonstration(&demonstration)?;
    let destination = ProposalDestination::naming(
        ExecutionSuite::named(super::OWNER, "mutation-receiver")
            .map_err(|_| MutationRoadFailure::Name)?,
    );
    reject_capsule_for_another_execution(&candidate, mutation, &demonstration, destination)?;
    let proposal = offer_mutant_kill(
        candidate,
        mutation,
        capsule.clone(),
        demonstration.clone(),
        Vec::new(),
        destination,
    )?;
    Ok(KillFixture {
        proposal,
        capsule,
        demonstration,
    })
}

fn reject_foreign_proposal_custody(
    proposal: &impl ReplayBearingProposal,
) -> Result<(), MutationRoadFailure> {
    let foreign = ProposalId::over(ContentAddress::derived(
        REPLAY_SCHEMA_TAG,
        b"foreign-proposal",
    ));
    let custody = StoredProposalRef::at(foreign, "review://foreign")
        .map_err(MutationRoadFailure::ProposalSink)?;
    let mut depot = ReplayDepot::default();
    assert!(matches!(
        human_admit_replay(proposal, custody, &mut depot),
        Err(HumanAdmissionRefusal::ProposalCustodyMismatch { expected, found })
            if expected == proposal.identity() && found == foreign
    ));
    assert!(depot.entries.is_empty());
    Ok(())
}

fn admit_row(admitted: Row) -> Result<(), MutationRoadFailure> {
    let subject = admitted.subject();
    let check = admitted.check();
    let revision =
        RevisionBinding::declared(ContentAddress::derived(super::REVISION_TAG, b"trial"));
    let binding = Binding::bound(
        admitted,
        ExecutableAttachment::attached(subject, check, revision, revision, candidate_trial_call),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)?;
    let world = TrialTable::authored(
        AuthoredTableName::named(super::OWNER, "admitted-world")
            .map_err(|_| MutationRoadFailure::Name)?,
        Provenance::Unproduced,
        vec![binding],
    )
    .map_err(TrialTableRefusal::TableNotAuthored)?;
    assert_eq!(world.bindings().len(), 1);
    Ok(())
}

fn independent_proposal_identity(
    proposal: &impl ProposalDocument,
    ground: AdmissionGround,
) -> ProposalId {
    let mut preimage = Vec::new();
    preimage.extend_from_slice(&1u32.to_be_bytes());
    encode_bytes(
        proposal.candidate().canonical_bytes().as_bytes(),
        &mut preimage,
    );
    preimage.push(ground.slot());
    proposal
        .destination()
        .suite()
        .name()
        .encode_into(&mut preimage);
    ProposalId::over(ContentAddress::derived(PROPOSAL_TAG, &preimage))
}

fn coverage_for(
    table_stem: &'static str,
    claim_stem: &'static str,
) -> Result<ClaimCoverage, MutationRoadFailure> {
    let table = TrialTable::authored(
        AuthoredTableName::named(super::OWNER, table_stem)
            .map_err(|_| MutationRoadFailure::Name)?,
        Provenance::Unproduced,
        vec![trial_binding_for(claim_stem)?],
    )
    .map_err(TrialTableRefusal::TableNotAuthored)?;
    let report = run_all(
        &table.view(),
        &SelectionPlan::of(Selection::All),
        &invocation()?,
    );
    Ok(claim_coverage(&report)?)
}

/// Claim: Survivor synthesis names an authored executable check and preserves the survivor's informed origin.
/// Subject: One interpreted survivor and one caller-stated candidate sketch.
/// Population: The survivor target, owner claim, closing check, and every candidate row coordinate.
/// Hostile control: A killed record cannot become an explanation, and an unattached closing check becomes a typed gap instead of a row.
/// Denominator: Every public explanation and synthesis boundary reachable from these records.
/// Evidence ceiling: This outside test establishes in-memory synthesis behavior only, not attachment execution or review admission.
/// Retained regression: Verdict laundering and candidates naming unattached checks remain permanent owner regressions.
#[test]
fn survivor_synthesis_requires_an_authored_closing_check() -> Result<(), MutationRoadFailure> {
    let killed = interpreted_kill()?;
    let closing =
        CheckRef::named(super::OWNER, "survivor-closing").map_err(|_| MutationRoadFailure::Name)?;
    assert!(matches!(
        SurvivorExplanation::of(&killed, OracleClass::GoldenVector, closing),
        Err(ExplanationRefusal::NotASurvivor(found))
            if found == killed.verdict()
    ));

    let survivor = interpreted_survivor()?;
    let explanation = SurvivorExplanation::of(&survivor, OracleClass::GoldenVector, closing)
        .map_err(|_| MutationRoadFailure::MissingActiveSelection)?;
    let sketch = CandidateSketch::stated(
        ExecutionSuite::named(super::OWNER, "mutation-receiver")
            .map_err(|_| MutationRoadFailure::Name)?,
        Classification::authored(
            vec![Role::named(super::OWNER, "mutation").map_err(|_| MutationRoadFailure::Name)?],
            vec![
                Tag::named(super::OWNER, "outside-consumer")
                    .map_err(|_| MutationRoadFailure::Name)?,
            ],
        )
        .map_err(|_| MutationRoadFailure::Name)?,
        SubjectRoute::named(super::OWNER, "comparison-subject")
            .map_err(|_| MutationRoadFailure::Name)?,
        PopulationRef::named(super::OWNER, "one-input").map_err(|_| MutationRoadFailure::Name)?,
    );
    assert!(matches!(
        synthesize(&explanation, &sketch, &BTreeSet::new()),
        Err(SynthesisRefusal::CheckGapFound(gap))
            if gap.claim() == explanation.claim()
                && gap.check() == closing
                && gap.missing() == OracleClass::GoldenVector
    ));
    let candidate = synthesize(&explanation, &sketch, &BTreeSet::from([closing]))
        .map_err(|_| MutationRoadFailure::MissingActiveSelection)?;
    assert_eq!(candidate.claim(), explanation.claim());
    assert_eq!(candidate.check(), closing);
    assert!(matches!(
        candidate.origin(),
        Origin::Candidate(SynthesisFacts::Survivor(point))
            if Some(point) == survivor.target().identity().point()
    ));
    Ok(())
}

/// Claim: Openings and pin deltas are readings of report coverage, and proof shape alone selects the discharge lane.
/// Subject: One exercised claim, one unrelated baseline claim, and three absent owed claims.
/// Population: Both complete coverage readings and every declared proof shape.
/// Hostile control: The exercised owed claim is filtered out while absent claims enter with explicit zero counts.
/// Denominator: Every claim entry in both reports and all three proof-shape routes.
/// Evidence ceiling: This outside test establishes coverage-derived planning only, not whether any proposed discharge is sufficient.
/// Retained regression: Structural scanning, absent-claim omission, and policy-based route selection remain permanent owner regressions.
#[test]
fn coverage_readings_open_and_route_only_missing_proof() -> Result<(), MutationRoadFailure> {
    let claim = super::claim()?;
    let before = coverage_for("proposal-before", "unrelated-behaviour")?;
    let after = coverage_for("proposal-after", "comparison-behaviour")?;
    let delta = pin_delta(&before, &after, claim).map_err(MutationRoadFailure::Delta)?;
    assert_eq!(delta.before(), 0usize);
    assert_eq!(delta.after(), 1usize);

    let exercised =
        OwedClaim::declared(claim, "not-exercised").map_err(MutationRoadFailure::Owed)?;
    let stated = OwedClaim::declared(
        ClaimRef::named(super::OWNER, "owed-stated").map_err(|_| MutationRoadFailure::Name)?,
        "no-stated-case",
    )
    .map_err(MutationRoadFailure::Owed)?;
    let generated = OwedClaim::declared(
        ClaimRef::named(super::OWNER, "owed-generated").map_err(|_| MutationRoadFailure::Name)?,
        "no-generated-search",
    )
    .map_err(MutationRoadFailure::Owed)?;
    let scheduled = OwedClaim::declared(
        ClaimRef::named(super::OWNER, "owed-scheduled").map_err(|_| MutationRoadFailure::Name)?,
        "no-scheduled-fault",
    )
    .map_err(MutationRoadFailure::Owed)?;
    let declared = [
        OwedDeclaration::stated(exercised, ProofShape::StatedCase),
        OwedDeclaration::stated(stated, ProofShape::StatedCase),
        OwedDeclaration::stated(generated, ProofShape::GeneratedSearch),
        OwedDeclaration::stated(scheduled, ProofShape::ScheduledFault),
    ];
    let found = openings(&after, &declared);
    assert_eq!(found.len(), 3usize);
    assert_eq!(
        found
            .iter()
            .map(|opening| opening.owed())
            .collect::<Vec<_>>(),
        vec![stated, generated, scheduled]
    );
    assert!(found.iter().all(|opening| {
        opening.exercise().exercised() == 0usize && opening.exercise().unexercised() == 0usize
    }));
    let [stated_opening, generated_opening, scheduled_opening] = found.as_slice() else {
        return Err(MutationRoadFailure::MissingActiveSelection);
    };
    assert_eq!(route(stated_opening), ObligationLane::TestRow);
    assert_eq!(route(generated_opening), ObligationLane::FuzzSeed);
    assert_eq!(route(scheduled_opening), ObligationLane::ChaosScenario);
    Ok(())
}

/// Claim: Replay-bearing admission joins the exact reviewed proposal, replay capsule, depot custody, and admitted row.
/// Subject: One demonstrated mutant-kill proposal crossing caller-owned review and replay sinks.
/// Population: The proposal, both custody tokens, the stored entry, and the admitted descriptor row.
/// Hostile control: Custody for another proposal refuses before storage, while a depot token for another replay refuses after the entry reaches storage.
/// Denominator: Every custody value and admitted value produced by this proposal crossing.
/// Evidence ceiling: This outside test establishes local proposal admission behavior only, not durable storage or human identity.
/// Retained regression: Proposal-custody checks and replay-depot ordering remain permanent owner regressions.
#[test]
fn replay_admission_joins_both_custody_roads() -> Result<(), MutationRoadFailure> {
    let fixture = offered_kill(&interpreted_kill()?)?;
    let mut review = ReviewSink::default();
    let proposal_custody = review
        .store(&fixture.proposal)
        .map_err(MutationRoadFailure::ProposalSink)?;
    assert_eq!(proposal_custody.proposal(), fixture.proposal.identity());
    assert_eq!(review.proposals.as_slice(), &[fixture.proposal.identity()]);
    reject_foreign_proposal_custody(&fixture.proposal)?;

    let foreign_capsule = capture_demonstration_under(&fixture.demonstration, "foreign-replay")?;
    assert_ne!(foreign_capsule.identity(), fixture.capsule.identity());
    let foreign_pin = offer_claim_pin(
        fixture.proposal.candidate().clone(),
        fixture.proposal.candidate().claim(),
        foreign_capsule,
        ProofDelta::between(0, 1).map_err(MutationRoadFailure::Delta)?,
        fixture.proposal.destination(),
    )
    .map_err(MutationRoadFailure::PinProposal)?;
    let foreign_custody = review
        .store(&foreign_pin)
        .map_err(MutationRoadFailure::ProposalSink)?;
    let mut foreign_depot = ReplayDepot::default();
    let foreign_receipt = human_admit_replay(&foreign_pin, foreign_custody, &mut foreign_depot)?;
    let mut mismatched = MismatchedReplayDepot {
        entries: Vec::new(),
        found: foreign_receipt.entry().replay(),
    };
    let mismatch_custody = review
        .store(&fixture.proposal)
        .map_err(MutationRoadFailure::ProposalSink)?;
    assert!(matches!(
        human_admit_replay(&fixture.proposal, mismatch_custody, &mut mismatched),
        Err(HumanAdmissionRefusal::ReplayCustodyMismatch { expected, found })
            if expected.address() == fixture.capsule.identity()
                && found == foreign_receipt.entry().replay()
    ));
    assert_eq!(mismatched.entries.len(), 1usize);

    let mut depot = ReplayDepot::default();
    let receipt = human_admit_replay(&fixture.proposal, proposal_custody, &mut depot)?;
    assert_eq!(receipt.entry().proposal(), fixture.proposal.identity());
    assert_eq!(receipt.entry().capsule(), &fixture.capsule);
    assert_eq!(receipt.replay_custody().replay(), receipt.entry().replay());
    assert_eq!(depot.entries.as_slice(), &[receipt.entry().clone()]);
    assert!(matches!(receipt.row().origin(), Origin::AdmittedReplay(_)));
    admit_row(receipt.row().clone())
}

/// Claim: Claim-pin and obligation-discharge offers retain distinct grounds through explicit admission.
/// Subject: One candidate offered first as a replay-bearing pin and then as a replay-free discharge.
/// Population: Both offers, both review tokens, one replay entry, and both admitted rows.
/// Hostile control: A discharge whose trial is already recorded refuses before review custody.
/// Denominator: Every proposal ground and admission receipt constructed from the candidate.
/// Evidence ceiling: This outside test establishes ground and admission distinctions only, not completeness of owed declarations.
/// Retained regression: Replay leaking into discharge or duplicate discharge reaching review remains a permanent owner regression.
#[test]
fn pin_and_discharge_keep_distinct_admission_grounds() -> Result<(), MutationRoadFailure> {
    let mutation = interpreted_kill()?;
    let (candidate, demonstration) = demonstrate_mutation(&mutation)?;
    let capsule = capture_demonstration(&demonstration)?;
    let destination = ProposalDestination::naming(
        ExecutionSuite::named(super::OWNER, "mutation-receiver")
            .map_err(|_| MutationRoadFailure::Name)?,
    );
    let claim = candidate.claim();
    let delta = ProofDelta::between(0, 1).map_err(MutationRoadFailure::Delta)?;
    let pin = offer_claim_pin(
        candidate.clone(),
        claim,
        capsule.clone(),
        delta,
        destination,
    )
    .map_err(MutationRoadFailure::PinProposal)?;
    assert_eq!(
        pin.duplicate().reason(),
        NoComparisonReason::GroundCarriesNoFailure
    );
    let mut review = ReviewSink::default();
    let pin_custody = review
        .store(&pin)
        .map_err(MutationRoadFailure::ProposalSink)?;
    let mut depot = ReplayDepot::default();
    let pin_receipt = human_admit_replay(&pin, pin_custody, &mut depot)?;
    assert!(matches!(
        pin_receipt.row().origin(),
        Origin::AdmittedReplay(_)
    ));
    assert_eq!(pin_receipt.entry().capsule(), &capsule);

    let trial = trial_identity(&candidate);
    let owed = OwedClaim::declared(claim, "uncovered-claim").map_err(MutationRoadFailure::Owed)?;
    let evidence = DischargeEvidence::recorded(
        ObligationLane::TestRow,
        trial,
        demonstration.trial_report().standing().key().clone(),
    );
    assert!(matches!(
        offer_obligation_discharge(
            candidate.clone(),
            owed,
            evidence.clone(),
            &[trial],
            destination,
        ),
        Err(DischargeProposalRefusal::Duplicate(
            DuplicateRefusal::ObligationAlreadyDischarged(recorded)
        )) if recorded == trial
    ));
    let discharge = offer_obligation_discharge(candidate, owed, evidence, &[], destination)
        .map_err(MutationRoadFailure::DischargeProposal)?;
    assert_eq!(discharge.duplicate().owed(), claim);
    let discharge_custody = review
        .store(&discharge)
        .map_err(MutationRoadFailure::ProposalSink)?;
    let receipt = human_admit_discharge(&discharge, discharge_custody)?;
    assert!(matches!(
        receipt.row().origin(),
        Origin::AdmittedDischarge(_)
    ));
    admit_row(receipt.row().clone())
}

/// Claim: Proposal identity is the documented candidate, ground, and destination preimage under the proposal domain tag.
/// Subject: One kill proposal and one pin proposal over the same candidate and destination.
/// Population: Every field of both proposal identity preimages.
/// Hostile control: Changing only the admission ground must move the identity.
/// Denominator: The canonical candidate bytes, ground slot, destination namespace, and destination stem.
/// Evidence ceiling: This outside test establishes exact preimage bytes only, not collision resistance.
/// Retained regression: Field-order, framing, domain-tag, and ground-slot drift remain permanent owner regressions.
#[test]
fn proposal_identity_preimages_are_independently_read() -> Result<(), MutationRoadFailure> {
    let fixture = offered_kill(&interpreted_kill()?)?;
    let destination = fixture.proposal.destination();
    let candidate = fixture.proposal.candidate().clone();
    let claim = candidate.claim();
    let pin = offer_claim_pin(
        candidate,
        claim,
        fixture.capsule,
        ProofDelta::between(0, 1).map_err(MutationRoadFailure::Delta)?,
        destination,
    )
    .map_err(MutationRoadFailure::PinProposal)?;
    assert_eq!(
        fixture.proposal.identity(),
        independent_proposal_identity(&fixture.proposal, AdmissionGround::MutantKilled)
    );
    assert_eq!(
        pin.identity(),
        independent_proposal_identity(&pin, AdmissionGround::ClaimPinned)
    );
    assert_ne!(fixture.proposal.identity(), pin.identity());
    Ok(())
}
