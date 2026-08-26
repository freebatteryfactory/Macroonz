//! The invariant nucleus of mutation proposals and explicit admission.

use super::{
    ActivationDisposition, AdmissionGround, CandidateSketch, CheckGap, CheckRef, ClaimExercise,
    ClaimPinnedGround, ClaimPinnedProposal, ClaimRef, Classification, ContentAddress,
    DemonstratedRejection, Demonstration, DischargeAdmissionReceipt, DischargeEvidence,
    DuplicateRefusal, ExecutionKey, ExecutionSuite, ExplanationRefusal, FailureComparison,
    Fingerprint, InferredObligation, MutantKilledGround, MutantKilledProposal, MutationReport,
    MutationTarget, MutationVerdict, Namespace, NoComparison, NoComparisonReason,
    ObligationComparison, ObligationDischargedGround, ObligationDischargedProposal, ObligationLane,
    OracleClass, Origin, OwedClaim, OwedClaimRefusal, OwedDeclaration, PROPOSAL_TAG, PopulationRef,
    ProofDelta, ProofDeltaRefusal, ProofRefusal, ProofShape, ProposalDestination, ProposalDocument,
    ProposalId, ProposalRefusal, ReplayAdmissionReceipt, ReplayBearingGround,
    ReplayBearingProposal, ReplayCapsule, ReplayCapsuleEntry, Row, RunAttempt, RunReport,
    SinkRefusal, StoredProposalRef, StoredReplayEntryRef, SubjectRoute, SurvivorExplanation,
    SynthesisFacts, TablePosture, TrialConclusion, TrialId, TrialReport, encode_bytes,
};
/// The version of the proposal identity encoding.
const PROPOSAL_ENCODING_VERSION: u32 = 1;
impl SurvivorExplanation {
    /// The explanation one survivor's record hands into synthesis.
    ///
    /// # Errors
    ///
    /// Refuses a record whose verdict is not survived, then a target whose owning claim is unmapped.
    pub fn of(
        report: &MutationReport,
        missing: OracleClass,
        closing: CheckRef,
    ) -> Result<Self, ExplanationRefusal> {
        let verdict = report.verdict();
        match verdict {
            MutationVerdict::Survived => {}
            MutationVerdict::Killed | MutationVerdict::Inconclusive => {
                return Err(ExplanationRefusal::NotASurvivor(verdict));
            }
        }
        let Some(claim) = report.target().owning_claim() else {
            return Err(ExplanationRefusal::OwnerUnmapped);
        };
        Ok(Self {
            target: report.target().clone(),
            claim,
            missing,
            closing,
        })
    }

    /// The target that survived.
    #[must_use]
    pub const fn target(&self) -> &MutationTarget {
        &self.target
    }

    /// The claim that owns it.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }

    /// The oracle class no check of that claim supplies.
    #[must_use]
    pub const fn missing(&self) -> OracleClass {
        self.missing
    }

    /// The check reference that would close the opening.
    #[must_use]
    pub const fn closing(&self) -> CheckRef {
        self.closing
    }
}

impl CheckGap {
    /// The finding a synthesis raises where the closing check has no attachment.
    pub const fn found(claim: ClaimRef, check: CheckRef, missing: OracleClass) -> Self {
        Self {
            claim,
            check,
            missing,
        }
    }

    /// The claim the opening belongs to.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// The check reference nobody has written an attachment for.
    #[must_use]
    pub const fn check(self) -> CheckRef {
        self.check
    }

    /// The oracle class the missing check would supply.
    #[must_use]
    pub const fn missing(self) -> OracleClass {
        self.missing
    }
}

impl CandidateSketch {
    /// The row coordinates the caller states beside an explanation.
    #[must_use]
    pub fn stated(
        suite: ExecutionSuite,
        classification: Classification,
        subject: SubjectRoute,
        population: PopulationRef,
    ) -> Self {
        Self {
            suite,
            classification,
            subject,
            population,
        }
    }

    /// The aggregate seat the candidate would run under.
    #[must_use]
    pub const fn suite(&self) -> ExecutionSuite {
        self.suite
    }

    /// How the candidate would be classified.
    #[must_use]
    pub const fn classification(&self) -> &Classification {
        &self.classification
    }

    /// What the candidate would exercise.
    #[must_use]
    pub const fn subject(&self) -> SubjectRoute {
        self.subject
    }

    /// The population that would supply its inputs.
    #[must_use]
    pub const fn population(&self) -> PopulationRef {
        self.population
    }
}
impl OwedClaim {
    /// The owed posture a claim's declaration states.
    ///
    /// # Errors
    ///
    /// Refuses a posture naming no opening condition, because an obligation that never comes due is one nobody can discharge.
    pub const fn declared(
        claim: ClaimRef,
        opening_condition: &'static str,
    ) -> Result<Self, OwedClaimRefusal> {
        if opening_condition.is_empty() {
            return Err(OwedClaimRefusal::NoOpeningCondition);
        }
        Ok(Self {
            claim,
            opening_condition,
        })
    }

    /// The claim declared owed.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// The condition its declaration named as the opening.
    #[must_use]
    pub const fn opening_condition(self) -> &'static str {
        self.opening_condition
    }
}

impl OwedDeclaration {
    /// One owed claim and the shape of proof its opening asks for.
    #[must_use]
    pub const fn stated(owed: OwedClaim, shape: ProofShape) -> Self {
        Self { owed, shape }
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(self) -> OwedClaim {
        self.owed
    }

    /// The shape of proof it asks for.
    #[must_use]
    pub const fn shape(self) -> ProofShape {
        self.shape
    }
}

impl InferredObligation {
    /// One opening a coverage reading stated.
    #[must_use]
    pub const fn inferred(owed: OwedClaim, exercise: ClaimExercise, shape: ProofShape) -> Self {
        Self {
            owed,
            exercise,
            shape,
        }
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(self) -> OwedClaim {
        self.owed
    }

    /// The counts the coverage reading recorded for it.
    #[must_use]
    pub const fn exercise(self) -> ClaimExercise {
        self.exercise
    }

    /// The shape of proof the opening asks for.
    #[must_use]
    pub const fn shape(self) -> ProofShape {
        self.shape
    }
}

impl DischargeEvidence {
    /// What discharged one owed claim.
    #[must_use]
    pub fn recorded(lane: ObligationLane, trial: TrialId, key: ExecutionKey) -> Self {
        Self { lane, trial, key }
    }

    /// The lane the obligation was routed to.
    #[must_use]
    pub const fn lane(&self) -> ObligationLane {
        self.lane
    }

    /// The trial that discharged it.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// The key that trial ran under.
    #[must_use]
    pub const fn key(&self) -> &ExecutionKey {
        &self.key
    }
}

// ---------------------------------------------------------------------------
// Demonstration, and the duplicate comparisons.
// ---------------------------------------------------------------------------

impl Demonstration {
    /// Read a demonstrated kill out of the report a staged run wrote.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a report standing over the authored world rather than a staged view, a census that does not carry the candidate, a candidate the selection passed over, a candidate that did not execute, and a candidate that executed and did not refuse.
    pub fn read(report: RunReport, candidate: TrialId) -> Result<Self, ProofRefusal> {
        require_staged(&report)?;
        let trial_report = candidate_report(&report, candidate)?;
        let rejection = demonstrated_rejection(&trial_report, candidate)?;
        Ok(Self {
            report,
            trial_report,
            rejection,
        })
    }

    /// The report the staged run wrote.
    #[must_use]
    pub const fn report(&self) -> &RunReport {
        &self.report
    }

    /// The candidate trial report the rejection was read from.
    #[must_use]
    pub const fn trial_report(&self) -> &TrialReport {
        &self.trial_report
    }

    /// The rejection read out of it.
    #[must_use]
    pub const fn rejection(&self) -> &DemonstratedRejection {
        &self.rejection
    }
}

/// Require the table posture one candidate demonstration stands over.
fn require_staged(report: &RunReport) -> Result<(), ProofRefusal> {
    match report.posture() {
        TablePosture::Staged { parent: _ } => Ok(()),
        TablePosture::Authored => Err(ProofRefusal::NotStaged),
    }
}

/// Read the selected candidate's trial report from the complete staged census.
fn candidate_report(report: &RunReport, candidate: TrialId) -> Result<TrialReport, ProofRefusal> {
    let Some(entry) = report
        .census()
        .iter()
        .find(|accounting| accounting.trial() == candidate)
    else {
        return Err(ProofRefusal::CandidateNotInCensus);
    };
    let Some(executed) = entry.disposition().report() else {
        return Err(ProofRefusal::CandidateNotSelected);
    };
    Ok(executed.clone())
}

/// Read the demonstrated rejection from one selected candidate report.
fn demonstrated_rejection(
    report: &TrialReport,
    candidate: TrialId,
) -> Result<DemonstratedRejection, ProofRefusal> {
    match report.attempt() {
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => Ok(
            DemonstratedRejection::demonstrated(candidate, finding.clone()),
        ),
        RunAttempt::Executed(TrialConclusion::Passed) => Err(ProofRefusal::CandidateDidNotRefuse),
        RunAttempt::SkippedWithReason(_)
        | RunAttempt::TimedOut
        | RunAttempt::InfrastructureFailed(_) => Err(ProofRefusal::CandidateDidNotExecute),
    }
}

impl ProofDelta {
    /// How much proof one candidate added to the claim it pins.
    ///
    /// # Errors
    ///
    /// Refuses a pair that does not move, because a candidate that leaves the exercised count where it was pins nothing.
    pub const fn between(before: usize, after: usize) -> Result<Self, ProofDeltaRefusal> {
        if after <= before {
            return Err(ProofDeltaRefusal::NoProofAdded { before, after });
        }
        Ok(Self { before, after })
    }

    /// The exercised count before the candidate ran.
    #[must_use]
    pub const fn before(self) -> usize {
        self.before
    }

    /// The exercised count after it ran.
    #[must_use]
    pub const fn after(self) -> usize {
        self.after
    }
}

impl FailureComparison {
    /// The comparison a failure-bearing ground offers.
    ///
    /// # Errors
    ///
    /// Refuses a candidate whose fingerprint the known roster already carries.
    pub fn compared(
        candidate: Fingerprint,
        known: Vec<Fingerprint>,
    ) -> Result<Self, DuplicateRefusal> {
        if known.contains(&candidate) {
            return Err(DuplicateRefusal::FingerprintAlreadyKnown(candidate));
        }
        Ok(Self { candidate, known })
    }

    /// The fingerprint this candidate carries.
    #[must_use]
    pub const fn candidate(&self) -> Fingerprint {
        self.candidate
    }

    /// The fingerprints already known, in the order they were compared.
    pub fn known(&self) -> impl Iterator<Item = &Fingerprint> {
        self.known.iter()
    }
}

impl ObligationComparison {
    /// The comparison a discharge ground offers.
    ///
    /// # Errors
    ///
    /// Refuses an owed claim that already carries a discharge, naming the first one recorded for it.
    pub fn compared(owed: ClaimRef, discharges: &[TrialId]) -> Result<Self, DuplicateRefusal> {
        if let Some(first) = discharges.first() {
            return Err(DuplicateRefusal::ObligationAlreadyDischarged(*first));
        }
        Ok(Self { owed })
    }

    /// The owed claim.
    #[must_use]
    pub const fn owed(&self) -> ClaimRef {
        self.owed
    }
}

impl NoComparison {
    /// The statement a ground with no comparable subject makes.
    #[must_use]
    pub const fn stated(reason: NoComparisonReason) -> Self {
        Self { reason }
    }

    /// Why nothing was compared.
    #[must_use]
    pub const fn reason(self) -> NoComparisonReason {
        self.reason
    }
}

// ---------------------------------------------------------------------------
// The three proposal grounds.
// ---------------------------------------------------------------------------

impl MutantKilledGround {
    /// The ground a demonstrated kill stands on.
    #[must_use]
    pub(in crate::muterprater) const fn shown(
        target: MutationTarget,
        activation: ActivationDisposition,
        capsule: ReplayCapsule,
        demonstration: Demonstration,
    ) -> Self {
        Self {
            target,
            activation,
            capsule,
            demonstration,
        }
    }

    /// What was damaged.
    #[must_use]
    pub const fn target(&self) -> &MutationTarget {
        &self.target
    }

    /// What the damage's activation was.
    #[must_use]
    pub const fn activation(&self) -> ActivationDisposition {
        self.activation
    }

    /// The reproduction account of the demonstrating run.
    #[must_use]
    pub const fn capsule(&self) -> &ReplayCapsule {
        &self.capsule
    }

    /// The demonstrated kill.
    #[must_use]
    pub const fn demonstration(&self) -> &Demonstration {
        &self.demonstration
    }
}

impl ClaimPinnedGround {
    /// The ground a pin stands on.
    #[must_use]
    pub const fn moved(claim: ClaimRef, capsule: ReplayCapsule, delta: ProofDelta) -> Self {
        Self {
            claim,
            capsule,
            delta,
        }
    }

    /// The claim pinned.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }

    /// The reproduction account of the pinning run.
    #[must_use]
    pub const fn capsule(&self) -> &ReplayCapsule {
        &self.capsule
    }

    /// What the pin added to the claim's proof.
    #[must_use]
    pub const fn delta(&self) -> ProofDelta {
        self.delta
    }
}

impl ObligationDischargedGround {
    /// The ground a discharge stands on.
    #[must_use]
    pub const fn discharged(owed: OwedClaim, discharge: DischargeEvidence) -> Self {
        Self { owed, discharge }
    }

    /// The owed claim's identity.
    #[must_use]
    pub const fn owed(&self) -> &OwedClaim {
        &self.owed
    }

    /// What discharged it.
    #[must_use]
    pub const fn discharge(&self) -> &DischargeEvidence {
        &self.discharge
    }
}

// ---------------------------------------------------------------------------
// The proposals, and their one identity road.
// ---------------------------------------------------------------------------

impl ProposalDestination {
    /// Where an admitted row would land.
    #[must_use]
    pub const fn naming(suite: ExecutionSuite) -> Self {
        Self { suite }
    }

    /// The aggregate seat it would land in.
    #[must_use]
    pub const fn suite(self) -> ExecutionSuite {
        self.suite
    }

    /// The semantic owner: the suite's own namespace.
    #[must_use]
    pub const fn owner(self) -> Namespace {
        self.suite.name().namespace()
    }
}

impl MutantKilledProposal {
    /// One proposal on the mutant-killed ground, offered.
    ///
    /// # Errors
    ///
    /// Refuses, in a declared dependent order: a row that does not carry the candidate origin arm, and a survivor synthesis fact naming a different point than the ground's target names.
    pub(in crate::muterprater) fn offered(
        candidate: Row,
        ground: MutantKilledGround,
        duplicate: FailureComparison,
        destination: ProposalDestination,
    ) -> Result<Self, ProposalRefusal> {
        let facts = candidate_facts(&candidate)?;
        survivor_point_agrees(facts, ground.target())?;
        Ok(Self {
            candidate,
            ground,
            duplicate,
            destination,
        })
    }

    /// The ground it stands on.
    #[must_use]
    pub const fn ground(&self) -> &MutantKilledGround {
        &self.ground
    }

    /// The evidence it is not a duplicate.
    #[must_use]
    pub const fn duplicate(&self) -> &FailureComparison {
        &self.duplicate
    }
}

impl ClaimPinnedProposal {
    /// One proposal on the claim-pinned ground, offered.
    ///
    /// The comparison seat states its own vacancy: a pin carries no failure to fingerprint, so [`NoComparisonReason::GroundCarriesNoFailure`] is the whole of what there is to compare.
    ///
    /// # Errors
    ///
    /// Refuses a row that does not carry the candidate origin arm.
    pub(in crate::muterprater) fn offered(
        candidate: Row,
        ground: ClaimPinnedGround,
        destination: ProposalDestination,
    ) -> Result<Self, ProposalRefusal> {
        candidate_facts(&candidate)?;
        Ok(Self {
            candidate,
            ground,
            duplicate: NoComparison::stated(NoComparisonReason::GroundCarriesNoFailure),
            destination,
        })
    }

    /// The ground it stands on.
    #[must_use]
    pub const fn ground(&self) -> &ClaimPinnedGround {
        &self.ground
    }

    /// The stated reason nothing was compared.
    #[must_use]
    pub const fn duplicate(&self) -> NoComparison {
        self.duplicate
    }
}

impl ObligationDischargedProposal {
    /// One proposal on the obligation-discharged ground, offered.
    ///
    /// # Errors
    ///
    /// Refuses a row that does not carry the candidate origin arm.
    pub(in crate::muterprater) fn offered(
        candidate: Row,
        ground: ObligationDischargedGround,
        duplicate: ObligationComparison,
        destination: ProposalDestination,
    ) -> Result<Self, ProposalRefusal> {
        candidate_facts(&candidate)?;
        Ok(Self {
            candidate,
            ground,
            duplicate,
            destination,
        })
    }

    /// The ground it stands on.
    #[must_use]
    pub const fn ground(&self) -> &ObligationDischargedGround {
        &self.ground
    }

    /// The evidence it is not a duplicate.
    #[must_use]
    pub const fn duplicate(&self) -> &ObligationComparison {
        &self.duplicate
    }
}

impl ProposalDocument for MutantKilledProposal {
    fn candidate(&self) -> &Row {
        &self.candidate
    }

    fn ground_summary(&self) -> AdmissionGround {
        AdmissionGround::from(&self.ground)
    }

    fn destination(&self) -> ProposalDestination {
        self.destination
    }

    fn identity(&self) -> ProposalId {
        proposal_identity(&self.candidate, self.ground_summary(), self.destination)
    }
}

impl ProposalDocument for ClaimPinnedProposal {
    fn candidate(&self) -> &Row {
        &self.candidate
    }

    fn ground_summary(&self) -> AdmissionGround {
        AdmissionGround::from(&self.ground)
    }

    fn destination(&self) -> ProposalDestination {
        self.destination
    }

    fn identity(&self) -> ProposalId {
        proposal_identity(&self.candidate, self.ground_summary(), self.destination)
    }
}

impl ProposalDocument for ObligationDischargedProposal {
    fn candidate(&self) -> &Row {
        &self.candidate
    }

    fn ground_summary(&self) -> AdmissionGround {
        AdmissionGround::from(&self.ground)
    }

    fn destination(&self) -> ProposalDestination {
        self.destination
    }

    fn identity(&self) -> ProposalId {
        proposal_identity(&self.candidate, self.ground_summary(), self.destination)
    }
}

impl ReplayBearingProposal for MutantKilledProposal {
    fn replay_capsule(&self) -> &ReplayCapsule {
        self.ground.capsule()
    }

    fn replay_ground(&self) -> ReplayBearingGround {
        ReplayBearingGround::MutantKilled
    }
}

impl ReplayBearingProposal for ClaimPinnedProposal {
    fn replay_capsule(&self) -> &ReplayCapsule {
        self.ground.capsule()
    }

    fn replay_ground(&self) -> ReplayBearingGround {
        ReplayBearingGround::ClaimPinned
    }
}

/// The one road every proposal's identity is derived by, over the three readings the three of them share.
///
/// The candidate row's canonical bytes were written where that row was born, so this reads them rather than encoding a row a second time.
/// Written once rather than per proposal: three copies of one preimage agree until one is edited, and the specification is stated on [`ProposalDocument::identity`].
fn proposal_identity(
    candidate: &Row,
    ground: AdmissionGround,
    destination: ProposalDestination,
) -> ProposalId {
    let mut preimage = Vec::new();
    preimage.extend_from_slice(&PROPOSAL_ENCODING_VERSION.to_be_bytes());
    encode_bytes(candidate.canonical_bytes().as_bytes(), &mut preimage);
    preimage.push(ground.slot());
    destination.suite().name().encode_into(&mut preimage);
    ProposalId::over(ContentAddress::derived(PROPOSAL_TAG, &preimage))
}

/// The synthesis facts a candidate row carries, or the refusal it earns.
fn candidate_facts(candidate: &Row) -> Result<SynthesisFacts, ProposalRefusal> {
    match candidate.origin() {
        Origin::Candidate(facts) => Ok(facts),
        Origin::HandWritten
        | Origin::Generated(_)
        | Origin::AdmittedReplay(_)
        | Origin::AdmittedDischarge(_) => Err(ProposalRefusal::NotACandidate),
    }
}

/// Whether the row's survivor point and the ground's target name one point.
///
/// The check is possible only where both name a point: an external target names a coordinate, and a proof-gap synthesis names no point at all.
/// It takes the target rather than a ground, because the target is the whole of what it reads and only one ground has one.
fn survivor_point_agrees(
    facts: SynthesisFacts,
    target: &MutationTarget,
) -> Result<(), ProposalRefusal> {
    let SynthesisFacts::Survivor(synthesis) = facts else {
        return Ok(());
    };
    let Some(point) = target.identity().point() else {
        return Ok(());
    };
    if synthesis == point {
        return Ok(());
    }
    Err(ProposalRefusal::SurvivorPointMismatch {
        synthesis,
        target: point,
    })
}

// ---------------------------------------------------------------------------
// Custody, and the admission receipts.
// ---------------------------------------------------------------------------

impl StoredProposalRef {
    /// Bind a sink's storage location to the proposal it stored.
    ///
    /// # Errors
    ///
    /// Refuses an empty token, which names nowhere.
    pub fn at(proposal: ProposalId, token: &str) -> Result<Self, SinkRefusal> {
        if token.is_empty() {
            return Err(SinkRefusal::EmptyLocation);
        }
        Ok(Self {
            proposal,
            token: token.to_owned(),
        })
    }

    /// The content identity of the proposal stored at this location.
    #[must_use]
    pub const fn proposal(&self) -> ProposalId {
        self.proposal
    }

    /// The token, for a sink to read its own location back.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }
}

impl ReplayAdmissionReceipt {
    /// Retain the exact outputs of one completed replay-bearing human admission.
    #[must_use]
    pub(in crate::muterprater) fn completed(
        row: Row,
        entry: ReplayCapsuleEntry,
        proposal_custody: StoredProposalRef,
        replay_custody: StoredReplayEntryRef,
    ) -> Self {
        Self {
            row,
            entry,
            proposal_custody,
            replay_custody,
        }
    }

    /// The row whose candidate origin became human-admitted provenance.
    #[must_use]
    pub const fn row(&self) -> &Row {
        &self.row
    }

    /// The exact capsule entry the human admission stored.
    #[must_use]
    pub const fn entry(&self) -> &ReplayCapsuleEntry {
        &self.entry
    }

    /// The caller's review-durable custody of the proposal.
    #[must_use]
    pub const fn proposal_custody(&self) -> &StoredProposalRef {
        &self.proposal_custody
    }

    /// The caller's storage location for the replay entry.
    #[must_use]
    pub const fn replay_custody(&self) -> &StoredReplayEntryRef {
        &self.replay_custody
    }
}

impl DischargeAdmissionReceipt {
    /// Retain the outputs of one completed obligation-discharge human admission.
    #[must_use]
    pub(in crate::muterprater) fn completed(row: Row, proposal_custody: StoredProposalRef) -> Self {
        Self {
            row,
            proposal_custody,
        }
    }

    /// The row whose candidate origin became human-admitted provenance.
    #[must_use]
    pub const fn row(&self) -> &Row {
        &self.row
    }

    /// The caller's review-durable custody of the proposal.
    #[must_use]
    pub const fn proposal_custody(&self) -> &StoredProposalRef {
        &self.proposal_custody
    }
}
