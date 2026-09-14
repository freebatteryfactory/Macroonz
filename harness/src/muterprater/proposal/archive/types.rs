//! Owned historical proposal vocabulary and independent archive bounds.

use crate::descriptor::archive::{ArchivedCandidate, ArchivedName, CandidateArchiveRefusal};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::muterprater::ObligationLane;
use crate::muterprater::verdict::archive::{ArchivedActivation, ArchivedMutationTarget};
use crate::report::archive::{
    AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedCapsule, ArchivedExecution,
    ArchivedFinding, ArchivedFingerprint, ArchivedRun, ArchivedTrial,
};

#[path = "type_guard.rs"]
mod guard;
pub use guard::read_proposal;

/// The integrity domain for historical proposal envelopes.
pub const PROPOSAL_ARCHIVE_TAG: DomainTag =
    DomainTag::declared("historical-proposal", IdentityProfileVersion::declared(1));

/// Independent byte, candidate-label, staged-census and known-failure ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProposalArchiveLimits {
    bytes: ArchiveLimits,
    labels: usize,
    rows: usize,
    known: usize,
}

/// The complete retained ground of a historical kill proposal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedKillGround {
    target: ArchivedMutationTarget,
    activation: ArchivedActivation,
    capsule: ArchivedCapsule,
    report: ArchivedRun,
    trial: ArchivedTrial,
    rejection: ArchivedFinding,
    known: Vec<ArchivedFingerprint>,
}

/// The historical claim, capsule and positive proof-count movement of a pin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedPinGround {
    claim: ArchivedName,
    capsule: ArchivedCapsule,
    before: u64,
    after: u64,
}

/// The historical owed claim, opening and discharge coordinates without a replay entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedDischargeGround {
    owed: ArchivedName,
    opening: String,
    lane: ObligationLane,
    trial: AddressClaim,
    key: ArchivedExecution,
}

/// The concrete historical ground encoded by this archive format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedProposalGround {
    /// A retained demonstrated-kill ground and known-failure comparison.
    MutantKilled(Box<ArchivedKillGround>),
    /// A retained claim-pinning ground with no failure comparison.
    ClaimPinned(Box<ArchivedPinGround>),
    /// A retained discharge ground with an empty prior-discharge comparison.
    ObligationDischarged(Box<ArchivedDischargeGround>),
}

/// An owned historical offer whose evidence integrity is separate from proposal identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedProposal {
    encoded: Vec<u8>,
    address: ContentAddress,
    identity: ContentAddress,
    candidate: ArchivedCandidate,
    destination: ArchivedName,
    ground: ArchivedProposalGround,
}

/// Why a historical proposal failed bounded admission.
#[must_use = "a refusal states why historical proposal data was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalArchiveRefusal {
    /// Existing archive framing, field, identity or custody admission refused.
    Historical(ArchiveRefusal),
    /// The descriptor owner's historical candidate reader refused.
    Candidate(CandidateArchiveRefusal),
    /// The known-failure roster exceeds its independent ceiling.
    TooManyKnownFailures,
    /// The ground discriminant has no reading in this format.
    InvalidGround,
    /// The complete staged census does not demonstrate the named trial's refusal.
    DemonstrationRequired,
    /// The recorded proof counts do not increase.
    InvalidProofDelta,
    /// The owed claim names no opening condition.
    MissingOpeningCondition,
    /// A known fingerprint equals the demonstrated candidate failure.
    FailureAlreadyKnown,
    /// The candidate and target name different survivor points.
    SurvivorPointMismatch,
}
