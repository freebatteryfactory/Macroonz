//! Read-only projections from admitted historical reduction accounts.

use super::{
    ArchivedReduction, ArchivedReductionCensus, ArchivedSemanticReducer, ReductionArchiveLimits,
};
use crate::generate::{ByteReducerExecution, ReductionBudget, ReductionHalt};
use crate::identity::ContentAddress;
use crate::report::ReplayPosture;
use crate::report::archive::{AddressClaim, ArchiveLimits, ArchivedCapsule, ArchivedName};

#[path = "read.rs"]
mod read;
#[path = "guard_account.rs"]
mod account;

pub use read::read_reduction;

impl ReductionArchiveLimits {
    /// The byte ceilings and maximum invoked-reducer population.
    #[must_use]
    pub const fn declared(bytes: ArchiveLimits, reducers: usize) -> Self {
        Self { bytes, reducers }
    }

    /// The envelope and framed-member ceilings.
    #[must_use]
    pub const fn bytes(self) -> ArchiveLimits {
        self.bytes
    }

    /// The maximum invoked-reducer population.
    #[must_use]
    pub const fn reducers(self) -> usize {
        self.reducers
    }
}

impl ArchivedSemanticReducer {
    /// The historical semantic reducer name.
    #[must_use]
    pub const fn name(&self) -> &ArchivedName {
        &self.name
    }

    /// The claimed callable revision.
    #[must_use]
    pub const fn revision(&self) -> AddressClaim {
        self.revision
    }

    /// The historical callable posture.
    #[must_use]
    pub const fn claimed_posture(&self) -> ReplayPosture {
        self.posture
    }

    /// The number of candidates the source claimed were offered.
    #[must_use]
    pub const fn candidates(&self) -> u64 {
        self.candidates
    }

    /// The number of those candidates the source claimed were probed.
    #[must_use]
    pub const fn probes(&self) -> u64 {
        self.probes
    }
}

impl ArchivedReductionCensus {
    /// The accepted candidate count.
    #[must_use]
    pub const fn accepted(&self) -> u32 {
        self.accepted
    }

    /// The moved-fingerprint candidate count.
    #[must_use]
    pub const fn fingerprint_moved(&self) -> u32 {
        self.fingerprint_moved
    }

    /// The no-failure candidate count.
    #[must_use]
    pub const fn no_failure(&self) -> u32 {
        self.no_failure
    }

    /// The checked total candidate count.
    #[must_use]
    pub const fn probes(&self) -> u32 {
        self.probes
    }
}

impl ArchivedReduction {
    /// The address derived over this historical envelope.
    #[must_use]
    pub const fn address(&self) -> ContentAddress {
        self.address
    }

    /// The complete historical capsule with its original case and reached witness.
    #[must_use]
    pub const fn capsule(&self) -> &ArchivedCapsule {
        &self.capsule
    }

    /// The original report ceiling claimed before reduction.
    #[must_use]
    pub const fn report_posture(&self) -> ReplayPosture {
        self.report_posture
    }

    /// The historical probe revision claim.
    #[must_use]
    pub const fn probe_revision(&self) -> AddressClaim {
        self.probe_revision
    }

    /// The historical probe posture.
    #[must_use]
    pub const fn probe_posture(&self) -> ReplayPosture {
        self.probe_posture
    }

    /// The candidate-probe budget claimed by the completed account.
    #[must_use]
    pub const fn budget(&self) -> ReductionBudget {
        self.budget
    }

    /// The recorded generic-reducer reach.
    #[must_use]
    pub const fn byte_reducer(&self) -> ByteReducerExecution {
        self.byte_reducer
    }

    /// The checked historical candidate census.
    #[must_use]
    pub const fn census(&self) -> ArchivedReductionCensus {
        self.census
    }

    /// The historical stopping posture.
    #[must_use]
    pub const fn halt(&self) -> ReductionHalt {
        self.halt
    }
}

impl ArchivedReduction {
    /// The exact envelope for caller-owned storage.
    #[must_use]
    pub fn encoded(&self) -> &[u8] {
        &self.encoded
    }

    /// The semantic reducers in their claimed execution order.
    #[must_use]
    pub fn semantic_reducers(&self) -> &[ArchivedSemanticReducer] {
        &self.semantic_reducers
    }
}
