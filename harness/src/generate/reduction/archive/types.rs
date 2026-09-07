//! Historical reduction vocabulary without an execution or capsule mint.

use crate::generate::{ByteReducerExecution, ReductionBudget, ReductionHalt};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::report::ReplayPosture;
use crate::report::archive::{
    AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedCapsule, ArchivedName,
};

#[path = "type_guard.rs"]
mod guard;

pub use guard::read_reduction;

/// The envelope domain for historical reduction accounts.
pub const REDUCTION_ARCHIVE_TAG: DomainTag =
    DomainTag::declared("historical-reduction", IdentityProfileVersion::declared(1));

/// Independent byte and invoked-reducer ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReductionArchiveLimits {
    bytes: ArchiveLimits,
    reducers: usize,
}

/// A historical semantic invocation with portable offered and probed counts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedSemanticReducer {
    name: ArchivedName,
    revision: AddressClaim,
    posture: ReplayPosture,
    candidates: u64,
    probes: u64,
}

/// A historical candidate census whose checked sum fits the declared budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchivedReductionCensus {
    accepted: u32,
    fingerprint_moved: u32,
    no_failure: u32,
    probes: u32,
}

/// An owned historical reduction account with internally joined budget and participant claims.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedReduction {
    encoded: Vec<u8>,
    address: ContentAddress,
    capsule: ArchivedCapsule,
    report_posture: ReplayPosture,
    probe_revision: AddressClaim,
    probe_posture: ReplayPosture,
    budget: ReductionBudget,
    semantic_reducers: Vec<ArchivedSemanticReducer>,
    byte_reducer: ByteReducerExecution,
    census: ArchivedReductionCensus,
    halt: ReductionHalt,
}

/// Why a historical reduction account was not admitted.
#[must_use = "a refusal states why historical reduction data was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReductionArchiveRefusal {
    /// A shared envelope, field or nested capsule refused.
    Archive(ArchiveRefusal),
    /// The invoked roster exceeds its independently supplied ceiling.
    TooManyReducers,
    /// The declared budget permits no candidate probe.
    ZeroBudget,
    /// The invoked roster repeats a semantic name.
    DuplicateReducer,
    /// The candidate counts, phase reach or halt contradict the declared budget.
    AccountingMismatch,
    /// The capsule ceiling does not meet the retained participants, or the report exceeds its decoder.
    PostureMismatch,
}
