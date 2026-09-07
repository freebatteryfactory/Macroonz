//! Owned historical record vocabulary and bounded admission failures.

pub use crate::descriptor::archive::ArchivedName;

use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::report::{
    FailureClass, InfrastructureFault, InvocationProfile, NotSelectedReason, ReplayPosture,
    SelectionOutcome, SkipReason, TargetBinding, TextFidelity,
};

#[path = "type_guard.rs"]
mod guard;

pub use guard::{read_capsule, read_run, read_trial};

pub(crate) use guard::{claim, cursor, envelope, finish, frame, name, posture};

/// The envelope domain for historical complete-run records.
pub const RUN_ARCHIVE_TAG: DomainTag =
    DomainTag::declared("historical-run-report", IdentityProfileVersion::declared(1));

/// Independent byte and census-row ceilings for complete-run retention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunArchiveLimits {
    bytes: ArchiveLimits,
    rows: usize,
}

/// The historical table posture, retaining only the name the report actually holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedTablePosture {
    /// The source claimed an authored table.
    Authored,
    /// The source claimed candidates overlaid on this authored parent.
    Staged {
        /// The exact historical parent name.
        parent: ArchivedName,
    },
}

/// What the source claimed about one census row's selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedDisposition {
    /// Selected, with the entire historical trial record.
    Selected(Box<ArchivedTrial>),
    /// Unselected, with the original reason.
    NotSelected(NotSelectedReason),
}

/// A historical census row with all its retained revision and claim coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedAccounting {
    trial: AddressClaim,
    row: AddressClaim,
    subject: AddressClaim,
    check: AddressClaim,
    claim: ArchivedName,
    disposition: ArchivedDisposition,
}

/// An owned historical complete run whose selected records agree with its context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedRun {
    encoded: Vec<u8>,
    address: ContentAddress,
    invocation: InvocationProfile,
    target: TargetBinding,
    input: Option<ArchivedInput>,
    posture: ArchivedTablePosture,
    selection: SelectionOutcome,
    census: Vec<ArchivedAccounting>,
}

/// The envelope domain for historical capsule data.
pub const CAPSULE_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-replay-capsule",
    IdentityProfileVersion::declared(1),
);

/// The envelope domain for historical trial records.
pub const TRIAL_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-trial-report",
    IdentityProfileVersion::declared(1),
);

/// The owned source site claimed by a historical trial.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedSite {
    module_path: String,
    file: String,
    line: u32,
    name: String,
}

/// The original foreign-material counts, independent of the reader's pointer width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchivedTruncation {
    /// All offered bytes were retained.
    Complete,
    /// Only the stated prefix was retained.
    TruncatedAt {
        /// The retained byte count.
        admitted: u64,
        /// The original offered byte count.
        offered: u64,
    },
}

/// Exact admitted foreign bytes with their original loss markers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedForeignText {
    bytes: Vec<u8>,
    truncation: ArchivedTruncation,
    fidelity: TextFidelity,
}

/// A historical failure identity with the original location and foreign material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedFinding {
    fingerprint: ArchivedFingerprint,
    file: String,
    line: u32,
    foreign: Option<ArchivedForeignText>,
}

/// The recorded conclusion of a historical executed trial.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedConclusion {
    /// The source claimed its check was satisfied.
    Passed,
    /// The source retained this typed refusal.
    Refused(Box<ArchivedFinding>),
}

/// Every historical attempt axis without a current execution mint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchivedAttempt {
    /// An execution and its recorded conclusion.
    Executed(ArchivedConclusion),
    /// A selected trial that did not execute.
    SkippedWithReason(SkipReason),
    /// A trial stopped at its declared time budget.
    TimedOut,
    /// A failure around the subject, without a subject conclusion.
    InfrastructureFailed {
        /// The recorded infrastructure fault.
        fault: InfrastructureFault,
        /// Material retained by that boundary.
        foreign: Option<ArchivedForeignText>,
    },
}

/// A historical clock failure whose raw readings grant no live tick authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchivedClockFailure {
    /// The opening read refused.
    OpeningRefused,
    /// The closing read refused.
    ClosingRefused,
    /// The opening read unwound.
    OpeningUnwound,
    /// The closing read unwound.
    ClosingUnwound,
    /// The closing reading preceded the opening reading.
    Regressed {
        /// The historical opening reading in nanoseconds.
        opened: u64,
        /// The historical closing reading in nanoseconds.
        closed: u64,
    },
}

/// The historical measurement axis, separate from the attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchivedMeasurement {
    /// An observed elapsed duration in nanoseconds, including zero.
    Observed(u64),
    /// No clock was declared.
    Unavailable,
    /// An offered measurement failed at this boundary.
    Failed(ArchivedClockFailure),
}

/// An owned historical trial record with no conversion into a current report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedTrial {
    encoded: Vec<u8>,
    address: ContentAddress,
    key: ArchivedExecution,
    posture: ReplayPosture,
    site: ArchivedSite,
    attempt: ArchivedAttempt,
    measurement: ArchivedMeasurement,
}

/// Independent byte ceilings for the complete envelope and each framed member.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArchiveLimits {
    envelope: usize,
    field: usize,
}

/// A historical nested digest whose original preimage has not been supplied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AddressClaim([u8; 32]);

/// An owned historical profile spelling and version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedProfile {
    name: String,
    version: u32,
}

/// The original case and decoder claims alongside their historical profile metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedInput {
    namespace: String,
    profile: ArchivedProfile,
    schema: AddressClaim,
    case: AddressClaim,
    decoder: AddressClaim,
    posture: ReplayPosture,
}

/// An execution-key preimage interpreted solely as historical claims.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedExecution {
    address: ContentAddress,
    trial: AddressClaim,
    subject: AddressClaim,
    check: AddressClaim,
    invocation: InvocationProfile,
    target: TargetBinding,
    input: Option<ArchivedInput>,
}

/// An owned historical failure identity with its caller-owned cause spelling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedFingerprint {
    address: ContentAddress,
    trial: AddressClaim,
    family: String,
    local: String,
    class: FailureClass,
}

/// An integrity-checked historical capsule with no conversion to live execution evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchivedCapsule {
    encoded: Vec<u8>,
    address: ContentAddress,
    identity: ContentAddress,
    key: ArchivedExecution,
    fingerprint: ArchivedFingerprint,
    input: Vec<u8>,
    generation: ArchivedProfile,
    minimization: ArchivedProfile,
    schema: AddressClaim,
    posture: ReplayPosture,
}

/// Why no historical record could be admitted.
#[must_use = "a refusal states why historical data was not admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveRefusal {
    /// The complete envelope exceeds its independent ceiling.
    EnvelopeTooLarge,
    /// A framed member exceeds its independent ceiling.
    FieldTooLarge,
    /// Size arithmetic cannot be represented on this platform.
    SizeOutsidePlatform,
    /// A declared field extends beyond the supplied material.
    Truncated,
    /// A declared length cannot be indexed on this platform.
    LengthOutsidePlatform {
        /// The unrepresentable length.
        declared: u64,
    },
    /// The leading envelope address does not match the body.
    AddressMismatch,
    /// The version is not understood by this reader.
    UnsupportedFormat {
        /// The offered format.
        found: u32,
    },
    /// The envelope contains another record kind.
    WrongKind {
        /// The offered kind.
        found: u32,
    },
    /// The envelope attempts to assert unsupported custody.
    UnsupportedCustody {
        /// The offered custody slot.
        found: u32,
    },
    /// An enum discriminant has no meaning in this format.
    InvalidSlot,
    /// A text field is not UTF-8 or a required name component is empty.
    InvalidText,
    /// A nested digest is not exactly thirty-two bytes.
    InvalidAddressWidth,
    /// Fully read fields leave undeclared bytes.
    TrailingBytes,
    /// A nested identity disagrees with its supplied preimage or trial.
    IdentityJoinMismatch,
    /// The record claims a stronger ceiling than its decoder permits.
    PostureMismatch,
    /// Foreign byte counts or fidelity contradict the retained material.
    InvalidForeignText,
    /// A claimed clock regression does not contain backwards readings.
    InvalidMeasurement,
    /// The census exceeds its independent row ceiling.
    TooManyRows,
    /// Two census rows claim the same semantic trial.
    DuplicateTrial,
    /// A selected trial disagrees with the run's invocation, target or input.
    RunContextMismatch,
    /// The selection outcome contradicts the presence of selected census rows.
    SelectionMismatch,
}
