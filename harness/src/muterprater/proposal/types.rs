//! Survivor explanation, obligations, proposal grounds, custody, and admission receipts.

use crate::depot::capsules::{ReplayCapsuleEntry, ReplayDepotRefusal, StoredReplayEntryRef};
use crate::descriptor::{
    AdmissionGround, CheckRef, ClaimRef, Classification, ExecutionSuite, MutationPointRef,
    Namespace, Origin, PopulationRef, ProposalId, ReplayBearingGround, Row, RowRefusal,
    StagedTableRefusal, SubjectRoute, SynthesisFacts, TablePosture,
};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::muterprater::{
    ActivationDisposition, DemonstratedRejection, MutationReport, MutationTarget, MutationVerdict,
};
use crate::report::{
    ClaimExercise, ExecutionKey, Fingerprint, ReplayCapsule, RunAttempt, RunReport,
    TrialConclusion, TrialId, TrialReport, encode_bytes,
};
#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// Survivor explanation, and the check gap.
// ---------------------------------------------------------------------------

/// Which independence lane a survivor's explanation names as the missing judge.
///
/// The roster is the independence annex's own lanes ([`crate::oracle`]), named here so an explanation says which kind of judge is absent rather than that one is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OracleClass {
    /// Bytes a specification states for an input.
    GoldenVector,
    /// A published identity re-derived from its published specification.
    IndependentTranscript,
    /// What a rendered artifact declares.
    StructuralRead,
    /// What a compiled artifact hands back as values.
    CompiledReadBack,
}

/// One survivor, explained: the target, the claim that owns it, the oracle class no check supplies, and the check that would close the opening.
///
/// An explanation over an owner-unmapped target is refused rather than guessed, so no candidate is cut against a claim nobody established.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SurvivorExplanation {
    target: MutationTarget,
    claim: ClaimRef,
    missing: OracleClass,
    closing: CheckRef,
}

/// Why one survivor explanation was refused.
#[must_use = "a refusal is the reason a survivor was not explained"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplanationRefusal {
    /// The record's verdict is not survived, so there is no survivor to explain.
    NotASurvivor(MutationVerdict),
    /// The target's owning claim is unmapped, so the explanation would have to invent it.
    OwnerUnmapped,
}

/// The typed finding a synthesis raises instead of a candidate it cannot honestly build.
///
/// Synthesis is scoped to already-authored executable attachments, so where the named check has no attachment the opening is this finding rather than a candidate citing a callable nobody wrote.
#[must_use = "a check gap is a finding, never a candidate"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckGap {
    claim: ClaimRef,
    check: CheckRef,
    missing: OracleClass,
}

/// The row coordinates a synthesis cannot read off a survivor.
///
/// The explanation names the claim and the check; the suite, classification, subject route, and population are the caller's to state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateSketch {
    suite: ExecutionSuite,
    classification: Classification,
    subject: SubjectRoute,
    population: PopulationRef,
}

/// Why one candidate row could not be synthesized.
///
/// Dependent checks in a declared order: the attachment roster, then the synthesis facts the origin arm needs, then the row itself.
#[must_use = "a refusal is the reason a candidate was not synthesized"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthesisRefusal {
    /// The named check has no authored executable attachment, so the opening is a check gap.
    CheckGapFound(CheckGap),
    /// The explained record names a coordinate rather than a mutation point.
    ///
    /// A guard on the identity shape rather than on a lane: earning the survived verdict takes observed activation, and the one wrapped backend offers no channel that could observe a firing.
    ExternalSurvivorNamesNoPoint,
    /// The row constructor refused the values the synthesis assembled.
    RowRefused(RowRefusal),
}
// ---------------------------------------------------------------------------
// The obligation road.
// ---------------------------------------------------------------------------

/// A claim declared owed: its identity, and the opening condition its declaration named.
///
/// Owed is a posture on a claim and never a genus, and an obligation that never comes due is refused, so no value here is an obligation nobody can discharge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwedClaim {
    claim: ClaimRef,
    opening_condition: &'static str,
}

/// Why one owed-claim posture was refused.
#[must_use = "a refusal is the reason an owed claim was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwedClaimRefusal {
    /// The posture names no opening condition, so nothing states when the claim comes due.
    NoOpeningCondition,
}

/// What shape of proof one opening asks for.
///
/// Which lane discharges an obligation follows from this and nothing else, and that map is declared once in `type_contract.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofShape {
    /// One stated input and its stated answer.
    StatedCase,
    /// A search over a generated population.
    GeneratedSearch,
    /// A fault placed at a sequence position.
    ScheduledFault,
}

/// Which lane one inferred obligation is routed to discharge in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObligationLane {
    /// A descriptor row in the authored table.
    TestRow,
    /// A seed in the fuzz lane's warm start.
    FuzzSeed,
    /// A scenario in the chaos lane's campaign.
    ChaosScenario,
}

/// One claim declared owed, and the shape of proof its opening asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwedDeclaration {
    owed: OwedClaim,
    shape: ProofShape,
}

/// One opening a coverage reading states: an owed claim the denominator names and no report exercised.
///
/// Where proof is missing is claim coverage over reports and never a structural scan, so this value is born from a coverage entry and carries the counts it was born from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InferredObligation {
    owed: OwedClaim,
    exercise: ClaimExercise,
    shape: ProofShape,
}

/// What discharged one owed claim: the lane it was routed to, the trial that discharged it, and the key that trial ran under.
///
/// A discharge authors no capsule, because the admitted row is its permanent record and rerunning it regenerates the behavioral evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DischargeEvidence {
    lane: ObligationLane,
    trial: TrialId,
    key: ExecutionKey,
}

// ---------------------------------------------------------------------------
// The proposal road.
// ---------------------------------------------------------------------------

/// One demonstrated kill: the report the staged run wrote, and the rejection read out of it.
///
/// A claimed kill is demonstrated on the evaluation surface with the mutant active, never asserted, and the mutant-killed ground cannot be built without one of these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Demonstration {
    report: RunReport,
    trial_report: TrialReport,
    rejection: DemonstratedRejection,
}

/// Why no kill was demonstrated.
///
/// Dependent checks in a declared order: the view's posture, then the census, then the candidate's own disposition.
#[must_use = "a refusal is the reason a kill was not demonstrated"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofRefusal {
    /// The staged view could not be built.
    StagingRefused(StagedTableRefusal),
    /// The report stands over the authored world rather than a staged view.
    NotStaged,
    /// The report's census does not carry the candidate's trial at all.
    CandidateNotInCensus,
    /// The run's selection passed the candidate over, so it never executed.
    CandidateNotSelected,
    /// The candidate was selected and did not execute.
    CandidateDidNotExecute,
    /// The candidate executed and did not refuse, so the claimed kill is asserted rather than shown.
    CandidateDidNotRefuse,
}

/// How much proof one candidate adds to the claim it pins.
///
/// [`ProofDelta::between`] refuses a pair that does not move, because a pin that adds nothing is not a pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProofDelta {
    before: usize,
    after: usize,
}

/// Why one proof delta was refused.
#[must_use = "a refusal is the reason a proof delta was not stated"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofDeltaRefusal {
    /// The candidate leaves the claim's exercised count where it was.
    NoProofAdded {
        /// The count before.
        before: usize,
        /// The count after.
        after: usize,
    },
}

/// The ground a mutant-killed proposal stands on: a kill shown on the surface with the mutant active.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutantKilledGround {
    /// What was damaged.
    target: MutationTarget,
    /// What the damage's activation was.
    activation: ActivationDisposition,
    /// The reproduction account of the demonstrating run.
    capsule: ReplayCapsule,
    /// The demonstrated kill.
    demonstration: Demonstration,
}

/// The ground a claim-pinned proposal stands on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimPinnedGround {
    /// The claim pinned.
    claim: ClaimRef,
    /// The reproduction account of the pinning run.
    capsule: ReplayCapsule,
    /// What the pin added to the claim's proof.
    delta: ProofDelta,
}

/// The ground an obligation-discharged proposal stands on.
///
/// No capsule, and no seat for one: the admitted row is the discharge's permanent record, and the two grounds that do author a capsule each carry it as a field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObligationDischargedGround {
    /// The owed claim's identity.
    owed: OwedClaim,
    /// What discharged it.
    discharge: DischargeEvidence,
}

/// Why one comparison had no subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoComparisonReason {
    /// The ground carries no failure, so there is no fingerprint to compare.
    GroundCarriesNoFailure,
    /// Nothing comparable was kept: no previous fingerprint and no discharge roster.
    NoKnownMaterial,
}

/// The evidence a failure-bearing proposal is not a duplicate: the candidate's fingerprint, against every fingerprint already known.
///
/// The comparison happens where the value is built, so a duplicate is a refusal rather than a paragraph a reader has to check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailureComparison {
    /// The fingerprint this candidate carries.
    candidate: Fingerprint,
    /// The fingerprints already known, in the order they were compared.
    known: Vec<Fingerprint>,
}

/// The evidence a discharge proposal is not a duplicate: the owed claim, compared against the discharges already recorded for it.
///
/// The comparison happens where the value is built and only an empty roster survives it, so holding one IS holding the evidence — no roster seat rides along.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObligationComparison {
    /// The owed claim.
    owed: ClaimRef,
}

/// The statement a proposal with no comparable subject makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoComparison {
    /// Why nothing was compared.
    reason: NoComparisonReason,
}

/// Why one duplicate comparison refused its candidate.
#[must_use = "a refusal is the reason a proposal was not offered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DuplicateRefusal {
    /// The candidate's fingerprint is one the known roster already carries.
    FingerprintAlreadyKnown(Fingerprint),
    /// The owed claim already carries a discharge, so this one discharges nothing new.
    ObligationAlreadyDischarged(TrialId),
}

/// Where an admitted row would land: a semantic owner and a suite, never a file path.
///
/// One field, because the suite's own namespace is the semantic owner, and a second owner field here would be a second authority answering one question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProposalDestination {
    suite: ExecutionSuite,
}

/// The domain tag every proposal identity is derived under.
pub const PROPOSAL_TAG: DomainTag =
    DomainTag::declared("proposal", IdentityProfileVersion::declared(1));

/// What every proposal is, whichever ground it stands on: a candidate row, a ground word, a destination, and the identity those three derive.
///
/// Open, so a consumer with a proposal document shape of its own implements it in its own crate and reaches every [`ProposalSink`] through the same seam.
/// A road that stores or reports a proposal takes one of these rather than a sum type every ground would have to fit inside, which is what keeps a discharge proposal from being as large as a kill's demonstration.
///
/// # Nonclaims
///
/// It reaches no ground's own contents: what a kill demonstrated and what a pin moved are read off the concrete proposal, because they are exactly the facts the implementations do not share.
pub trait ProposalDocument {
    /// The candidate row.
    fn candidate(&self) -> &Row;

    /// The ground at summary width — the word an admission act states.
    fn ground_summary(&self) -> AdmissionGround;

    /// Where it would land.
    fn destination(&self) -> ProposalDestination;

    /// The proposal's content identity, which is permanent provenance.
    ///
    /// # The specification
    ///
    /// Two primitives: `u32be(n)`, and `bytes(x)` — `u64be(len(x))` followed by the bytes of `x`.
    ///
    /// The members, in exactly this order:
    ///
    /// | # | member | encoding |
    /// | - | ------ | -------- |
    /// | 1 | encoding version | `u32be` |
    /// | 2 | candidate row | `bytes(…)` of the descriptor home's canonical row bytes |
    /// | 3 | ground | one byte, [`AdmissionGround::slot`] |
    /// | 4 | destination namespace | `bytes(utf8)` |
    /// | 5 | destination stem | `bytes(utf8)` |
    ///
    /// # Nonclaims
    ///
    /// The evidence is deliberately absent: the capsule, the demonstration, and the duplicate comparison are what stands behind a proposal rather than what it proposes.
    /// Two offers of one row on one ground into one destination therefore share an identity, which is what makes an admitted origin's citation stable across a rerun.
    fn identity(&self) -> ProposalId;
}

/// The replay-bearing subset of proposal documents.
///
/// A discharge proposal cannot implement this trait and so cannot reach the replay admission operation.
pub trait ReplayBearingProposal: ProposalDocument {
    /// The run-bound capsule this proposal carries.
    fn replay_capsule(&self) -> &ReplayCapsule;

    /// The replay-bearing ground the human admission states.
    fn replay_ground(&self) -> ReplayBearingGround;
}

/// One proposal on the mutant-killed ground.
///
/// Process-local until a caller's own sink stores it, and constructing one asserts nothing about admission.
/// The comparison seat takes a [`FailureComparison`] and admits nothing else, so evidence that does not fit the ground is unwritable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutantKilledProposal {
    candidate: Row,
    ground: MutantKilledGround,
    duplicate: FailureComparison,
    destination: ProposalDestination,
}

/// One proposal on the claim-pinned ground.
///
/// Its comparison seat takes a [`NoComparison`], because a pin carries no failure to fingerprint and discharges no obligation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimPinnedProposal {
    candidate: Row,
    ground: ClaimPinnedGround,
    duplicate: NoComparison,
    destination: ProposalDestination,
}

/// One proposal on the obligation-discharged ground.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObligationDischargedProposal {
    candidate: Row,
    ground: ObligationDischargedGround,
    duplicate: ObligationComparison,
    destination: ProposalDestination,
}

/// Why one proposal was refused.
///
/// Dependent checks in a declared order: the row's origin, then — where the ground names a mutation point — the survivor point against the target.
/// There is no evidence-against-ground cause, because each proposal's comparison seat admits exactly the comparison its ground owes.
#[must_use = "a refusal is the reason a proposal was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProposalRefusal {
    /// The row does not carry the candidate origin arm.
    NotACandidate,
    /// The row's synthesis facts and the ground's target name different mutation points.
    SurvivorPointMismatch {
        /// The point the row's synthesis facts name.
        synthesis: MutationPointRef,
        /// The point the ground's target names.
        target: MutationPointRef,
    },
}

/// Why one mutant-killed proposal was not offered.
///
/// Dependent checks in a declared order: a harness-demonstrated rejection, agreement with the staged demonstration, replay execution and fingerprint binding, duplicate comparison, then proposal construction.
#[must_use = "a refusal is the reason a mutant-killed proposal was not offered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KillProposalRefusal {
    /// The mutation report does not carry a harness-demonstrated rejection.
    MutationNotDemonstrated {
        /// The verdict the report actually earned.
        verdict: MutationVerdict,
    },
    /// The mutation report and staged demonstration name different failures.
    DemonstrationMismatch {
        /// The content address of the failure the mutation report names.
        mutation: ContentAddress,
        /// The content address of the failure the staged demonstration names.
        demonstration: ContentAddress,
    },
    /// The replay capsule stands over another execution.
    ReplayExecutionMismatch {
        /// The execution address the capsule names.
        replay: ContentAddress,
        /// The execution address the demonstrating trial report names.
        demonstration: ContentAddress,
    },
    /// The replay capsule preserved another failure.
    ReplayFingerprintMismatch {
        /// The content address of the failure the capsule preserved.
        replay: ContentAddress,
        /// The content address of the failure the staged demonstration names.
        demonstration: ContentAddress,
    },
    /// The comparison found the candidate's failure already known.
    Duplicate(DuplicateRefusal),
    /// The proposal constructor refused the values that were assembled.
    Refused(ProposalRefusal),
}

/// Why one obligation-discharge proposal was not offered.
///
/// Dependent checks in a declared order: duplicate comparison, then proposal construction.
#[must_use = "a refusal is the reason a discharge proposal was not offered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DischargeProposalRefusal {
    /// The owed claim already carries a discharge.
    Duplicate(DuplicateRefusal),
    /// The proposal constructor refused the values that were assembled.
    Refused(ProposalRefusal),
}

/// The location one sink stored a proposal at.
///
/// Opaque and mortal: the review artifact may die after any ruling, which is why an admitted origin cites the proposal's [`ProposalId`] and never this token.
/// It is not an identity, not a path this crate can interpret, and not evidence that the destination is durable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredProposalRef {
    proposal: ProposalId,
    token: String,
}

/// Why one sink did not store a proposal.
///
/// The durability arm is the sink's own statement: this crate reaches no filesystem and can establish nothing about where a sink writes.
#[must_use = "a refusal is the reason a proposal was not stored"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SinkRefusal {
    /// The sink is not accepting proposals.
    Unavailable,
    /// The sink already holds a proposal under this content identity.
    AlreadyStored(ProposalId),
    /// The location offered is empty, so it names nowhere.
    EmptyLocation,
    /// The destination lies inside the repository tree or a build cache.
    ///
    /// Undischarged review evidence may never live there: deleting a cache must change only cost, never truth.
    DestinationNotDurable,
}

/// The caller-supplied storage the proposal road ends at.
///
/// One of the crate's two caller-owned storage seams — the other is the depot home's [`ReplayDepotSink`](crate::depot::capsules::ReplayDepotSink), which replay-bearing admission calls for the admitted entry; no realization is declared at either, no filesystem is reached, and no scratch directory exists.
/// Storing is not admitting — a stored proposal is review material a human rules on, and the ruling is what discharges it.
pub trait ProposalSink {
    /// Store one proposal, and hand back the location custody begins at.
    ///
    /// Generic over proposal documents rather than one sum type, so a discharge proposal does not have to be as large as a kill's demonstration to reach this seam.
    ///
    /// # Errors
    ///
    /// The sink's own refusal: unavailable, already stored under this identity, an empty location, or a destination that is not durable.
    fn store<Document: ProposalDocument>(
        &mut self,
        proposal: &Document,
    ) -> Result<StoredProposalRef, SinkRefusal>;
}

/// A completed human admission on a replay-bearing proposal.
///
/// The admitted row, the depot entry, proposal custody, and depot custody ride together, and construction happens only after the caller's sink reports the exact entry stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayAdmissionReceipt {
    row: Row,
    entry: ReplayCapsuleEntry,
    proposal_custody: StoredProposalRef,
    replay_custody: StoredReplayEntryRef,
}

/// A completed human admission on an obligation-discharge proposal.
///
/// The discharge authors no replay entry, because the admitted row is its durable behavioral record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DischargeAdmissionReceipt {
    row: Row,
    proposal_custody: StoredProposalRef,
}

/// Why an explicit human admission did not complete.
///
/// Checks precede caller storage: proposal custody, then row construction, then the replay depot's result and its exact-reference binding.
#[must_use = "a refusal is the reason human admission did not complete"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HumanAdmissionRefusal {
    /// The supplied review custody belongs to another proposal.
    ProposalCustodyMismatch {
        /// The proposal being admitted.
        expected: ProposalId,
        /// The proposal the storage reference names.
        found: ProposalId,
    },
    /// The admitted row could not be encoded.
    RowRefused(RowRefusal),
    /// The caller's replay depot refused storage.
    ReplayDepotRefused(ReplayDepotRefusal),
    /// The sink reported a location bound to another replay entry.
    ReplayCustodyMismatch {
        /// The content-derived replay reference being admitted.
        expected: crate::descriptor::ReplayRef,
        /// The replay reference the sink's location names.
        found: crate::descriptor::ReplayRef,
    },
}
