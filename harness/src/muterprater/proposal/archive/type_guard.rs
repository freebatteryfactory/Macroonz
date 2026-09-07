//! Historical-only proposal projections and caller-declared resource bounds.

use super::{
    ArchivedDischargeGround, ArchivedKillGround, ArchivedPinGround, ArchivedProposal,
    ArchivedProposalGround, ProposalArchiveLimits,
};
use crate::descriptor::AdmissionGround;
use crate::descriptor::archive::{ArchivedCandidate, ArchivedName};
use crate::identity::ContentAddress;
use crate::muterprater::verdict::archive::{ArchivedActivation, ArchivedMutationTarget};
use crate::muterprater::{NoComparisonReason, ObligationLane};
use crate::report::archive::{
    AddressClaim, ArchiveLimits, ArchivedCapsule, ArchivedExecution, ArchivedFinding,
    ArchivedFingerprint, ArchivedRun, ArchivedTrial,
};

#[path = "read.rs"]
mod read;
pub use read::read_proposal;

impl ProposalArchiveLimits {
    /// Declare the complete-envelope, per-field and independent roster ceilings.
    #[must_use]
    pub const fn declared(bytes: ArchiveLimits, labels: usize, rows: usize, known: usize) -> Self {
        Self {
            bytes,
            labels,
            rows,
            known,
        }
    }

    /// The complete-envelope and per-field byte ceilings.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.bytes
    }

    /// The maximum population of each candidate role or tag roster.
    #[must_use]
    pub const fn labels(self) -> usize {
        self.labels
    }

    /// The maximum staged census population.
    #[must_use]
    pub const fn rows(self) -> usize {
        self.rows
    }

    /// The maximum known-failure comparison population.
    #[must_use]
    pub const fn known(self) -> usize {
        self.known
    }
}

impl ArchivedProposalGround {
    /// The historical admission-ground word, without admission authority.
    #[must_use]
    pub const fn summary(&self) -> AdmissionGround {
        match self {
            Self::MutantKilled(_) => AdmissionGround::MutantKilled,
            Self::ClaimPinned(_) => AdmissionGround::ClaimPinned,
            Self::ObligationDischarged(_) => AdmissionGround::ObligationDischarged,
        }
    }
}

impl ArchivedProposal {
    /// The complete integrity-bearing envelope for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The integrity address over every retained field.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The proposal identity recomputed through the existing owner writer.
    #[must_use]
    pub const fn identity(&self) -> ContentAddress {
        self.identity
    }

    /// The complete historical candidate descriptor.
    #[must_use]
    pub const fn candidate(&self) -> &ArchivedCandidate {
        &self.candidate
    }

    /// The historical destination suite name.
    #[must_use]
    pub const fn destination(&self) -> &ArchivedName {
        &self.destination
    }

    /// The concrete historical ground and its retained comparison.
    #[must_use]
    pub const fn ground(&self) -> &ArchivedProposalGround {
        &self.ground
    }
}

impl ArchivedKillGround {
    /// The historical damaged target.
    #[must_use]
    pub const fn target(&self) -> &ArchivedMutationTarget {
        &self.target
    }

    /// The historical activation disposition.
    #[must_use]
    pub const fn activation(&self) -> &ArchivedActivation {
        &self.activation
    }

    /// The internally joined historical replay capsule.
    #[must_use]
    pub const fn capsule(&self) -> &ArchivedCapsule {
        &self.capsule
    }

    /// The complete staged historical census.
    #[must_use]
    pub const fn report(&self) -> &ArchivedRun {
        &self.report
    }

    /// The selected refusing trial read from that census.
    #[must_use]
    pub const fn trial_report(&self) -> &ArchivedTrial {
        &self.trial
    }

    /// The complete finding read from the selected trial.
    #[must_use]
    pub const fn rejection(&self) -> &ArchivedFinding {
        &self.rejection
    }

    /// The prior fingerprints in their original comparison order.
    #[must_use]
    pub fn known(&self) -> &[ArchivedFingerprint] {
        &self.known
    }
}

impl ArchivedPinGround {
    /// The historical claim pinned by the offer.
    #[must_use]
    pub const fn claim(&self) -> &ArchivedName {
        &self.claim
    }

    /// The retained capsule without inferred claim or execution joins.
    #[must_use]
    pub const fn capsule(&self) -> &ArchivedCapsule {
        &self.capsule
    }

    /// The original proof count before the proposed addition.
    #[must_use]
    pub const fn before(&self) -> u64 {
        self.before
    }

    /// The strictly greater proof count after the proposed addition.
    #[must_use]
    pub const fn after(&self) -> u64 {
        self.after
    }

    /// The reason this ground retains no failure comparison.
    #[must_use]
    pub const fn comparison() -> NoComparisonReason {
        NoComparisonReason::GroundCarriesNoFailure
    }
}

impl ArchivedDischargeGround {
    /// The historical owed claim.
    #[must_use]
    pub const fn owed(&self) -> &ArchivedName {
        &self.owed
    }

    /// The exact nonempty opening condition.
    #[must_use]
    pub fn opening_condition(&self) -> &str {
        &self.opening
    }

    /// The recorded discharge lane.
    #[must_use]
    pub const fn lane(&self) -> ObligationLane {
        self.lane
    }

    /// The independently recorded discharge trial claim.
    #[must_use]
    pub const fn trial(&self) -> AddressClaim {
        self.trial
    }

    /// The recorded execution key without an inferred discharge-trial join.
    #[must_use]
    pub const fn key(&self) -> &ArchivedExecution {
        &self.key
    }

    /// The owed claim compared against an empty prior-discharge roster.
    #[must_use]
    pub const fn compared_owed(&self) -> &ArchivedName {
        &self.owed
    }
}
