//! The proposal roads: where a row would land, the three proposals, their shared document seam, and the one identity every proposal derives.

use crate::descriptor::{
    AdmissionGround, ExecutionSuite, Namespace, Origin, ProposalId, ReplayBearingGround, Row,
    SynthesisFacts,
};
use crate::identity::ContentAddress;
use crate::muterprater::MutationTarget;
use crate::muterprater::proposal::types::{
    ClaimPinnedGround, ClaimPinnedProposal, FailureComparison, MutantKilledGround,
    MutantKilledProposal, NoComparison, NoComparisonReason, ObligationComparison,
    ObligationDischargedGround, ObligationDischargedProposal, PROPOSAL_TAG, ProposalDestination,
    ProposalDocument, ProposalRefusal, ReplayBearingProposal,
};
use crate::report::{ReplayCapsule, encode_bytes};

/// The version of the proposal identity encoding.
const PROPOSAL_ENCODING_VERSION: u32 = 1;

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
