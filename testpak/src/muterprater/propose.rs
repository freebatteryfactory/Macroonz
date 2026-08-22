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
//! A human admits, and admission is out of scope here: it is a two-part
//! human-authored patch — the authored row, plus, for a replay-bearing ground,
//! the capsule entering the harness's own depot as an entry the admission act
//! itself authors. [`admission_patch`] names which of the two shapes a proposal
//! would require, and nothing in this crate performs either. Runtime evidence
//! never writes authored specification.

use super::types::{
    ActivationDisposition, AdmissionPatch, CandidateSketch, CheckGap, Demonstration,
    FailureComparison, InferredObligation, KillProposalRefusal, MutantKilledGround,
    MutantKilledProposal, MutationTarget, ObligationLane, OwedDeclaration, ProofDelta,
    ProofDeltaRefusal, ProofRefusal, ProposalDestination, ProposalDocument, SurvivorExplanation,
    SynthesisRefusal,
};
use super::wrap::mutant_scoped;
use crate::descriptor::{CheckRef, ClaimRef, Origin, Row, StagedTableView, SynthesisFacts};
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
/// The demonstration is required by the SHAPE of this call: there is no road
/// here from a claimed kill to a proposal that does not pass through the report
/// a staged run wrote. The not-a-duplicate evidence is COMPUTED from that
/// demonstration's own fingerprint rather than supplied, so the comparison is
/// over the failure the run actually reached.
///
/// # Errors
///
/// Refuses a candidate whose failure the known roster already carries, then
/// whatever the proposal constructor refuses about the assembled values.
pub fn offer_mutant_kill(
    candidate: Row,
    target: MutationTarget,
    activation: ActivationDisposition,
    capsule: ReplayCapsule,
    demonstration: Demonstration,
    known: Vec<Fingerprint>,
    destination: ProposalDestination,
) -> Result<MutantKilledProposal, KillProposalRefusal> {
    let fingerprint = demonstration.rejection().fingerprint();
    let duplicate =
        FailureComparison::compared(fingerprint, known).map_err(KillProposalRefusal::Duplicate)?;
    let ground = MutantKilledGround::shown(target, activation, capsule, demonstration);
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

/// Which two-part patch a human admitting this proposal would author.
///
/// # Authority
///
/// A statement about the road's EXIT and nothing else. Admission is a human act
/// this crate never performs: the authored row is written by hand, and a
/// replay-bearing ground's capsule enters the harness's own depot as an entry
/// the admission act itself authors, with the row's replay reference pointing at
/// it.
#[must_use]
pub fn admission_patch(proposal: &impl ProposalDocument) -> AdmissionPatch {
    AdmissionPatch::from(proposal.ground_summary().capsule_posture())
}
