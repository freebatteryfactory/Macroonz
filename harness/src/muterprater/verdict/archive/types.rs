//! Historical mutation coordinates without execution or activation authority.

use crate::descriptor::archive::ArchivedName;
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::muterprater::{
    BaselineAxis, EquivalenceAxis, ExecutionAxis, InconclusiveCause, MaterializationAxis,
    MutationCensus, SourceCoordinate,
};
use crate::report::archive::{
    AddressClaim, ArchiveLimits, ArchiveRefusal, ArchivedFinding, ArchivedForeignText,
};

#[path = "type_guard.rs"]
mod guard;

pub use guard::read_mutation;
pub use guard::read_mutation_run;
pub(crate) use guard::{read_activation, read_target};

/// The envelope domain for complete ordered historical mutation runs.
pub const MUTATION_RUN_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-mutation-run",
    IdentityProfileVersion::declared(1),
);

/// Independent byte and report-population ceilings for historical mutation runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MutationRunArchiveLimits {
    bytes: ArchiveLimits,
    reports: usize,
}

/// A complete ordered historical mutation run and its derived accounting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedMutationRun {
    encoded: Vec<u8>,
    address: ContentAddress,
    baseline: BaselineAxis,
    reports: Vec<ArchivedMutation>,
    census: MutationCensus,
}

/// The envelope domain for a complete historical mutation record.
pub const MUTATION_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-mutation-report",
    IdentityProfileVersion::declared(1),
);

/// The retained rejection claim, with no live demonstration authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedRejection {
    /// The historical trial and cause preimage, location and optional foreign material.
    Demonstrated(Box<ArchivedFinding>),
    /// The backend's exact retained bytes and loss markers, without a fingerprint.
    ReportedByBackend(ArchivedForeignText),
}

/// The complete historical outcome and the evidence its arm retains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedMutationOutcome {
    /// The source claimed rejection under a qualified execution chain.
    Killed(ArchivedRejection),
    /// The source claimed acceptance after positive observed activation.
    Survived,
    /// The source claimed no conclusion for this stated cause.
    Inconclusive(InconclusiveCause),
}

/// An owned complete historical mutation record whose outcome respects its axis ceiling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedMutation {
    encoded: Vec<u8>,
    address: ContentAddress,
    target: ArchivedMutationTarget,
    baseline: BaselineAxis,
    materialization: MaterializationAxis,
    activation: ArchivedActivation,
    execution: ExecutionAxis,
    outcome: ArchivedMutationOutcome,
    equivalence: EquivalenceAxis,
}

/// Why a complete historical mutation record could not be admitted.
#[must_use = "a refusal explains why no historical mutation record was admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationArchiveRefusal {
    /// A bounded field or envelope failed the common historical record grammar.
    Record(ArchiveRefusal),
    /// A kill or survivor claims more than its retained axes permit.
    OutcomeAxesMismatch,
    /// The run claims a baseline other than a qualified pass.
    BaselineNotQualified,
    /// The declared report population exceeds its independent ceiling.
    TooManyReports,
    /// A composed interpreted record disagreed with its complete trial or required axes.
    InterpretedTrialMismatch,
}

/// The historical identity of a damaged subject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedMutationIdentity {
    /// A backend coordinate-and-damage address whose preimage is absent.
    External(AddressClaim),
    /// An interpreted point and its historical alternative.
    Interpreted {
        /// The historical point name.
        point: ArchivedName,
        /// The claimed alternative address.
        alternative: AddressClaim,
    },
    /// A compiled projection's point and historical alternative.
    CompiledProjection {
        /// The historical point name.
        point: ArchivedName,
        /// The claimed alternative address.
        alternative: AddressClaim,
    },
}

/// The source or declared site retained by a historical target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedMutationSite {
    /// An exact reported source coordinate.
    Reported(SourceCoordinate),
    /// A historical declared activation name.
    Declared(ArchivedName),
}

/// A target's retained identity, attribution, site and optional owning claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedMutationTarget {
    identity: ArchivedMutationIdentity,
    family: Option<String>,
    site: ArchivedMutationSite,
    owner: Option<ArchivedName>,
}

/// A positive historical callback count and its exact selection and witness claims.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedActivationReading {
    surface: AddressClaim,
    point: ArchivedName,
    alternative: AddressClaim,
    witness: AddressClaim,
    firings: u32,
}

/// The historical activation disposition without a live activation mint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedActivation {
    /// A positive callback report under these historical coordinates.
    Observed(ArchivedActivationReading),
    /// No positive activation was reported.
    NotObserved,
    /// The source backend had no activation channel.
    UnobservableUnderBackend,
}
