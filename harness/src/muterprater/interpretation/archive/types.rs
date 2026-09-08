//! Historical parity records and explicit typed encoding bindings.

use crate::descriptor::archive::{
    ArchivedBinding, ArchivedName, ArchivedRevisionBinding, BindingArchiveLimits,
    BindingArchiveRefusal,
};
use crate::descriptor::{NamespacedName, RevisionBinding};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::muterprater::ParityQualificationRefusal;
use crate::muterprater::backend_archive::{
    ArchivedSuitePressure, SuitePressureArchiveLimits, SuitePressureArchiveRefusal,
};
use crate::muterprater::discovery_archive::{
    ArchivedEvaluationSurface, SurfaceArchiveLimits, SurfaceArchiveRefusal,
};
use crate::muterprater::specimen_archive::{
    ArchivedProjectionPressure, ProjectionArchiveLimits, ProjectionArchiveRefusal,
};
use crate::muterprater::verdict_archive::{ArchivedMutation, MutationArchiveRefusal};
use crate::report::ForeignText;
use crate::report::archive::{
    AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedConclusion, ArchivedTrial,
};

#[path = "type_guard.rs"]
mod guard;
pub use guard::read_interpreted;
pub use guard::read_parity;

/// The historical no-mutation parity envelope domain.
pub const PARITY_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-no-mutation-parity",
    IdentityProfileVersion::declared(1),
);

/// A capture-free caller encoding and the convention and revision it declares.
pub struct ValueEncoder<Value> {
    convention: NamespacedName,
    version: u32,
    schema: ContentAddress,
    revision: RevisionBinding,
    encode: fn(&Value) -> Result<Vec<u8>, ValueEncodingRefusal>,
}

/// A caller-owned reason that a retained value could not be encoded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueEncodingRefusal {
    cause: NamespacedName,
    foreign: Option<ForeignText>,
}

/// The semantic role of a value being retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueRole {
    /// The input supplied separately to both callables.
    Input,
    /// The meaning returned by production.
    Production,
    /// The meaning returned by evaluation.
    Evaluation,
}

/// Independent complete-envelope, nested-record, value and substrate ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParityArchiveLimits {
    bytes: ArchiveLimits,
    binding: BindingArchiveLimits,
    trial: ArchiveLimits,
    value: usize,
    substrates: usize,
}

/// Historical convention metadata with no current encoder or decoder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedValueConvention {
    name: ArchivedName,
    version: u32,
    schema: AddressClaim,
    revision: ArchivedRevisionBinding,
}

/// Exact caller-encoded bytes and their original convention claims.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedValue {
    convention: ArchivedValueConvention,
    bytes: Vec<u8>,
}

/// The separate family, revisions and surface claims of a historical pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedEvaluationPair {
    family: ArchivedName,
    production: ArchivedRevisionBinding,
    evaluation: ArchivedRevisionBinding,
    surface: AddressClaim,
}

/// A nonempty, strictly ordered historical substrate roster.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedSubstrateRoster {
    names: Vec<ArchivedName>,
}

/// The historical declaration of what the two roads shared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedSubstrate {
    /// The source explicitly claimed independence.
    DeclaredIndependent,
    /// These foundations were declared, without independence from them.
    Standing(ArchivedSubstrateRoster),
}

/// The recorded disposition of a historical reading without live qualification authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchivedParityDisposition {
    /// No qualification disposition was retained.
    Raw,
    /// The source claimed qualification, consistent with the recorded facts.
    Qualified,
    /// The source retained the first qualification refusal.
    Rejected(ParityQualificationRefusal),
}

/// An owned historical comparison whose witness, reports and disposition agree internally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedParity {
    encoded: Vec<u8>,
    address: ContentAddress,
    pair: ArchivedEvaluationPair,
    witness: ArchivedBinding,
    input: ArchivedValue,
    production: ArchivedValue,
    evaluation: ArchivedValue,
    firings: u32,
    substrate: ArchivedSubstrate,
    conclusion: ArchivedConclusion,
    production_report: ArchivedTrial,
    evaluation_report: ArchivedTrial,
    disposition: ArchivedParityDisposition,
}

/// Why no complete historical parity record was admitted.
#[must_use = "a refusal explains why historical parity was not retained"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParityArchiveRefusal {
    /// Common envelope, field, finding or nested trial grammar refused.
    Record(ArchiveRefusal),
    /// Historical witness or revision grammar refused.
    Binding(BindingArchiveRefusal),
    /// A caller encoder refused at this value role.
    Encoder {
        /// The value whose encoding refused.
        role: ValueRole,
        /// The caller's actual refusal, with bounded foreign material.
        cause: ValueEncodingRefusal,
    },
    /// Encoded value bytes exceeded their independent ceiling.
    ValueTooLarge {
        /// The oversized value role.
        role: ValueRole,
    },
    /// The substrate population exceeded its independent ceiling.
    TooManySubstrates,
    /// A standing roster was empty, duplicated or not strictly ordered.
    InvalidSubstrate,
    /// Witness and report identity, revisions, context, site or posture disagreed.
    ReportJoinMismatch,
    /// The recorded qualification disposition contradicted its underlying facts.
    DispositionMismatch,
}

/// The complete historical interpreted-evidence envelope domain.
pub const INTERPRETED_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-interpreted-evidence",
    IdentityProfileVersion::declared(1),
);

/// Independent outer, nested surface, suite, projection, report and active-value ceilings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterpretedArchiveLimits {
    bytes: ArchiveLimits,
    surface: SurfaceArchiveLimits,
    suite: SuitePressureArchiveLimits,
    projection: ProjectionArchiveLimits,
    trial: ArchiveLimits,
    mutation: ArchiveLimits,
    active: usize,
}

/// The complete historical surface and both pressure books without current trust authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedInterpretedTrust {
    surface: ArchivedEvaluationSurface,
    suite: ArchivedSuitePressure,
    projection: ArchivedProjectionPressure,
}

/// Complete owned interpreted evidence without callable meanings or current execution authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedInterpretedEvidence {
    encoded: Vec<u8>,
    address: ContentAddress,
    trust: ArchivedInterpretedTrust,
    meaning: ArchivedValue,
    report: ArchivedTrial,
    mutation: ArchivedMutation,
}

/// Why complete historical interpreted evidence could not be retained or read.
#[must_use = "a refusal explains why historical interpreted evidence was not admitted"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpretedArchiveRefusal {
    /// A common field or envelope refused.
    Record(ArchiveRefusal),
    /// The complete nested surface refused.
    Surface(SurfaceArchiveRefusal),
    /// The complete nested generic suite pressure refused.
    Suite(SuitePressureArchiveRefusal),
    /// The complete nested projection pressure refused.
    Projection(ProjectionArchiveRefusal),
    /// The nested mutation or its relationship to the active trial refused.
    Mutation(MutationArchiveRefusal),
    /// The caller encoder refused the active meaning after the parity value roles.
    ActiveEncoder(ValueEncodingRefusal),
    /// Active meaning bytes exceeded their independent ceiling.
    ActiveValueTooLarge,
    /// The retained surface disagreed with the pair or selected membership.
    SurfaceJoinMismatch,
    /// A compiled or active target disagreed with its retained surface point and alternative.
    TargetJoinMismatch,
    /// The active report disagreed with the projection's execution or replay posture.
    ReportJoinMismatch,
    /// Positive activation disagreed with the exact selection or witness.
    ActivationJoinMismatch,
}
