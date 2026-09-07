//! Owned historical capsule vocabulary and bounded admission failures.

use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::report::{FailureClass, InvocationProfile, ReplayPosture, TargetBinding};

#[path = "type_guard.rs"]
mod guard;

pub use guard::read_capsule;

/// The envelope domain for historical capsule data.
pub const CAPSULE_ARCHIVE_TAG: DomainTag = DomainTag::declared(
    "historical-replay-capsule",
    IdentityProfileVersion::declared(1),
);

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

/// Why no historical capsule could be admitted.
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
    /// A text field is not UTF-8 or a required input profile name is empty.
    InvalidText,
    /// A nested digest is not exactly thirty-two bytes.
    InvalidAddressWidth,
    /// Fully read fields leave undeclared bytes.
    TrailingBytes,
    /// A nested identity disagrees with its supplied preimage or trial.
    IdentityJoinMismatch,
    /// The capsule claims a stronger ceiling than its decoder permits.
    PostureMismatch,
}
