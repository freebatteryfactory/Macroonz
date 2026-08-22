//! The proposal road: survivor to candidate, candidate to demonstrated kill,
//! opening to routed obligation, and the exit where a human admits.
//!
//! # Candidate proving is in memory
//!
//! A candidate binding stages in the staged view and executes against it through
//! the one report engine. No scratch directory exists anywhere on this road, and
//! [`ProposalSink`](super::ProposalSink) is the only storage seam in this crate:
//! a claimed mutant kill is DEMONSTRATED on the evaluation surface with that
//! mutant active, never asserted, and only then does the lane propose.
//!
//! # Synthesis is scoped to what is already authored
//!
//! Descriptors, never programs. A candidate references a check the harness
//! already has an executable attachment for; where the explanation names a check
//! nobody has written one for, the opening is a [`CheckGap`] finding and no
//! candidate is cut. That is what keeps candidates constructible in memory and
//! keeps a proposal from ever serializing executable code.
//!
//! # Where proof is missing
//!
//! Claim coverage over reports, never a structural scan. [`openings`] reads a
//! coverage value against the claims a declaration states are owed, and the
//! lane an opening is routed to follows from the SHAPE of proof it asks for.
//!
//! # The exit
//!
//! A human explicitly invokes one of this module's admission operations after a
//! caller-owned sink has taken durable custody of the proposal. Replay-bearing
//! proposals additionally cross the caller-owned replay depot before the
//! admitted row is returned. Runtime evidence never invokes either operation or
//! writes authored specification by itself.

use super::types::{
    CandidateSketch, CheckGap, Demonstration, DischargeAdmissionReceipt, FailureComparison,
    HumanAdmissionRefusal, InferredObligation, IntendedRejection, KillProposalRefusal,
    MutantKilledGround, MutantKilledProposal, MutationOutcome, MutationReport, MutationTarget,
    ObligationDischargedProposal, ObligationLane, OwedDeclaration, ProofDelta, ProofDeltaRefusal,
    ProofRefusal, ProposalDestination, ProposalDocument, ReplayAdmissionReceipt,
    ReplayBearingProposal, StoredProposalRef, SurvivorExplanation, SynthesisRefusal,
};
use super::wrap::mutant_scoped;
use crate::depot::capsules::{ReplayCapsuleEntry, ReplayDepotSink};
use crate::descriptor::{
    CheckRef, ClaimRef, DischargeAdmission, Origin, ReplayAdmission, Row, StagedTableView,
    SynthesisFacts,
};
use crate::report::{ClaimCoverage, Fingerprint, ReplayCapsule};
use crate::runner::{Invocation, SelectionPlan, TrialBinding, TrialTable, run_all, trial_identity};
use std::collections::BTreeSet;

/// Synthesize the candidate row one survivor explanation asks for.
///
/// # Authority
///
/// Scoped to already-authored executable attachments. The roster the caller
/// hands in is the set of checks the harness has a callable for, and a check
/// outside it produces the [`CheckGap`] finding rather than a candidate
/// referencing a callable nobody wrote.
///
/// # Errors
///
/// Refuses, in a declared dependent order: a closing check with no authored
/// attachment, a survivor whose target names no mutation point — the descriptor
/// vocabulary's candidate arm carries a point or a proof gap and nothing else —
/// and a row the descriptor constructor refused.
pub fn synthesize(
    explanation: &SurvivorExplanation,
    sketch: &CandidateSketch,
    authored_checks: &BTreeSet<CheckRef>,
) -> Result<Row, SynthesisRefusal> {
    let closing = explanation.closing();
    if !authored_checks.contains(&closing) {
        return Err(SynthesisRefusal::CheckGapFound(CheckGap::found(
            explanation.claim(),
            closing,
            explanation.missing(),
        )));
    }
    let Some(point) = explanation.target().identity().point() else {
        return Err(SynthesisRefusal::ExternalSurvivorNamesNoPoint);
    };
    Row::declared(
        explanation.claim(),
        sketch.suite(),
        sketch.classification().clone(),
        sketch.subject(),
        closing,
        sketch.population(),
        Origin::Candidate(SynthesisFacts::Survivor(point)),
    )
    .map_err(SynthesisRefusal::RowRefused)
}

/// Prove one candidate in memory, against the complete world it would join.
///
/// # Authority
///
/// Three steps and no fourth: the candidate stages over the authored parent, the
/// mutant-scoped selection joins on a shape the rows already carry, and the one
/// report engine runs it. The demonstrated kill is READ out of the report the run
/// wrote, so a proposal on the mutant-killed ground can only be assembled from a
/// rejection that actually happened.
///
/// The world never shrinks: the staged view overlays the candidate on the
/// complete authored table, and the report is stated over every row of it
/// however few this selection named.
///
/// # Errors
///
/// Refuses a staging the descriptor vocabulary rejected, then every way the
/// report can fail to demonstrate a kill — a report over the authored world, a
/// census without the candidate, a candidate the selection passed over, one that
/// did not execute, and one that executed and did not refuse. The engine call
/// between them refuses nothing: a candidate that reached this road is a row
/// that was built, and a built row carries the bytes its census entry is named
/// from.
pub fn prove_candidate(
    parent: &TrialTable,
    candidate: TrialBinding,
    target: &MutationTarget,
    invocation: &Invocation,
) -> Result<Demonstration, ProofRefusal> {
    let trial = trial_identity(candidate.row());
    let staged =
        StagedTableView::staged(parent, vec![candidate]).map_err(ProofRefusal::StagingRefused)?;
    let selection = SelectionPlan::of(mutant_scoped(target));
    let report = run_all(&staged.view(), &selection, invocation);
    Demonstration::read(report, trial)
}

/// Offer one proposal on the mutant-killed ground.
///
/// # Authority
///
/// The mutation report supplies the target, activation, and harness-demonstrated
/// rejection as one closed record. The staged demonstration must name that same
/// failure, and the capsule must stand over the demonstrating trial report and
/// preserve its fingerprint. The not-a-duplicate evidence is computed from the
/// joined demonstration rather than supplied.
///
/// # Errors
///
/// Refuses a mutation report without a harness-demonstrated rejection, a
/// mutation and staged demonstration naming different failures, a capsule over
/// another execution or fingerprint, a candidate whose failure the known
/// roster already carries, then whatever the proposal constructor refuses.
pub fn offer_mutant_kill(
    candidate: Row,
    mutation: &MutationReport,
    capsule: ReplayCapsule,
    demonstration: Demonstration,
    known: Vec<Fingerprint>,
    destination: ProposalDestination,
) -> Result<MutantKilledProposal, KillProposalRefusal> {
    let mutation_rejection = match mutation.outcome() {
        MutationOutcome::Killed(IntendedRejection::Demonstrated(rejection)) => rejection,
        MutationOutcome::Killed(IntendedRejection::ReportedByBackend { stated: _ })
        | MutationOutcome::Survived
        | MutationOutcome::Inconclusive(_) => {
            return Err(KillProposalRefusal::MutationNotDemonstrated {
                verdict: mutation.verdict(),
            });
        }
    };
    let fingerprint = demonstration.rejection().fingerprint();
    let mutation_fingerprint = mutation_rejection.fingerprint();
    if mutation_fingerprint != fingerprint {
        return Err(KillProposalRefusal::DemonstrationMismatch {
            mutation: mutation_fingerprint.address(),
            demonstration: fingerprint.address(),
        });
    }
    let replay_execution = capsule.key().address();
    let demonstration_execution = demonstration.trial_report().standing().key().address();
    if replay_execution != demonstration_execution {
        return Err(KillProposalRefusal::ReplayExecutionMismatch {
            replay: replay_execution,
            demonstration: demonstration_execution,
        });
    }
    if capsule.fingerprint() != fingerprint {
        return Err(KillProposalRefusal::ReplayFingerprintMismatch {
            replay: capsule.fingerprint().address(),
            demonstration: fingerprint.address(),
        });
    }
    let duplicate =
        FailureComparison::compared(fingerprint, known).map_err(KillProposalRefusal::Duplicate)?;
    let ground = MutantKilledGround::shown(
        mutation.target().clone(),
        mutation.activation(),
        capsule,
        demonstration,
    );
    MutantKilledProposal::offered(candidate, ground, duplicate, destination)
        .map_err(KillProposalRefusal::Refused)
}

/// The proof one candidate added to the claim it pins, read from two coverage
/// values.
///
/// # Authority
///
/// Computed from reports and never hand-counted: the before and after are the
/// claim's exercised counts in two coverage readings, and a claim the reading
/// does not name counts as zero — a claim with no row in the denominator is
/// exercised by nothing.
///
/// # Errors
///
/// Refuses a pair that does not move: a candidate that leaves the claim's
/// exercised count where it was pins nothing.
pub fn pin_delta(
    before: &ClaimCoverage,
    after: &ClaimCoverage,
    claim: ClaimRef,
) -> Result<ProofDelta, ProofDeltaRefusal> {
    ProofDelta::between(exercised(before, claim), exercised(after, claim))
}

/// One claim's exercised count in one coverage reading, or zero where the
/// reading does not name it.
fn exercised(coverage: &ClaimCoverage, claim: ClaimRef) -> usize {
    coverage.exercise_or_zero(claim).exercised()
}

/// The openings one coverage reading states over the claims declared owed.
///
/// # Authority
///
/// "Where is proof missing" is claim coverage over reports, never a structural
/// scan. An owed claim the reading names and nothing exercised is an opening; an
/// owed claim the reading does not name at all is the STRONGEST opening — no row
/// in the denominator serves it — and it is reported with zero counts rather
/// than dropped for being absent.
///
/// # Nonclaims
///
/// An opening states that proof is missing. It states nothing about whether the
/// claim is wrong, whether the subject is wrong, or which lane should close it —
/// the last is [`route`]'s reading over the shape of proof the opening asks for.
#[must_use]
pub fn openings(coverage: &ClaimCoverage, declared: &[OwedDeclaration]) -> Vec<InferredObligation> {
    declared
        .iter()
        .filter_map(|declaration| opening(coverage, *declaration))
        .collect()
}

/// The opening one owed declaration states, where the coverage leaves one open.
fn opening(coverage: &ClaimCoverage, declaration: OwedDeclaration) -> Option<InferredObligation> {
    let owed = declaration.owed();
    let counted = coverage.exercise_or_zero(owed.claim());
    if counted.exercised() == 0_usize {
        return Some(InferredObligation::inferred(
            owed,
            counted,
            declaration.shape(),
        ));
    }
    None
}

/// The lane one inferred obligation is routed to discharge in.
///
/// # Authority
///
/// A planning decision, and it follows from the shape of proof the opening asks
/// for and nothing else — the map is declared once in this home's
/// `type_contract.rs`, so lane choice is a reading a caller can check rather
/// than a branch inside a planner.
#[must_use]
pub fn route(obligation: &InferredObligation) -> ObligationLane {
    ObligationLane::from(obligation.shape())
}

/// Admit one replay-bearing proposal after a human has ruled on stored review material.
///
/// # Authority
///
/// Calling this function is the explicit human boundary. Rust cannot establish
/// who called it; the operation establishes only that the supplied proposal
/// custody names this proposal, that its typed ground carries a replay capsule,
/// that the caller-owned depot accepted the exact derived entry, and that the
/// returned row cites those joined facts. No runtime road invokes it implicitly.
///
/// # Errors
///
/// Refuses, before replay storage, proposal custody for another proposal and a
/// row whose canonical encoding refused. After storage it refuses a location
/// bound to another replay entry.
pub fn human_admit_replay<Document, Sink>(
    proposal: &Document,
    proposal_custody: StoredProposalRef,
    depot: &mut Sink,
) -> Result<ReplayAdmissionReceipt, HumanAdmissionRefusal>
where
    Document: ReplayBearingProposal,
    Sink: ReplayDepotSink,
{
    proposal_custody_agrees(proposal, &proposal_custody)?;
    let entry =
        ReplayCapsuleEntry::admitted(proposal.identity(), proposal.replay_capsule().clone());
    let replay = entry.replay();
    let row = admitted_row(
        proposal,
        Origin::AdmittedReplay(ReplayAdmission::admitted(
            proposal.identity(),
            proposal.replay_ground(),
            proposal.destination().suite(),
            replay,
        )),
    )?;
    let replay_custody = depot
        .store(&entry)
        .map_err(HumanAdmissionRefusal::ReplayDepotRefused)?;
    if replay_custody.replay() != replay {
        return Err(HumanAdmissionRefusal::ReplayCustodyMismatch {
            expected: replay,
            found: replay_custody.replay(),
        });
    }
    Ok(ReplayAdmissionReceipt::completed(
        row,
        entry,
        proposal_custody,
        replay_custody,
    ))
}

/// Admit one obligation-discharge proposal after a human has ruled on stored review material.
///
/// # Authority
///
/// The proposal's type makes replay custody inapplicable. The operation checks
/// exact proposal custody and returns the row whose origin cites that admission;
/// no runtime road invokes it implicitly.
///
/// # Errors
///
/// Refuses proposal custody for another proposal, then a row whose canonical
/// encoding refused.
pub fn human_admit_discharge(
    proposal: &ObligationDischargedProposal,
    proposal_custody: StoredProposalRef,
) -> Result<DischargeAdmissionReceipt, HumanAdmissionRefusal> {
    proposal_custody_agrees(proposal, &proposal_custody)?;
    let row = admitted_row(
        proposal,
        Origin::AdmittedDischarge(DischargeAdmission::admitted(
            proposal.identity(),
            proposal.destination().suite(),
        )),
    )?;
    Ok(DischargeAdmissionReceipt::completed(row, proposal_custody))
}

/// Check that stored review custody names the proposal being admitted.
fn proposal_custody_agrees(
    proposal: &impl ProposalDocument,
    custody: &StoredProposalRef,
) -> Result<(), HumanAdmissionRefusal> {
    let expected = proposal.identity();
    let found = custody.proposal();
    if expected == found {
        return Ok(());
    }
    Err(HumanAdmissionRefusal::ProposalCustodyMismatch { expected, found })
}

/// Re-author one candidate row under the exact admitted origin its proposal earned.
fn admitted_row(
    proposal: &impl ProposalDocument,
    origin: Origin,
) -> Result<Row, HumanAdmissionRefusal> {
    let candidate = proposal.candidate();
    Row::declared(
        candidate.claim(),
        proposal.destination().suite(),
        candidate.classification().clone(),
        candidate.subject(),
        candidate.check(),
        candidate.population(),
        origin,
    )
    .map_err(HumanAdmissionRefusal::RowRefused)
}
