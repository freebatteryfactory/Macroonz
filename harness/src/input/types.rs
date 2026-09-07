//! Profiles, bounded specimen envelopes and the typed decoder that admits their values.

use crate::descriptor::{NamespacedName, RevisionBinding};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};

#[path = "type_guard.rs"]
mod guard;

/// The input-envelope format understood by this home.
pub const INPUT_FORMAT_VERSION: u32 = 1;

/// The address domain for one complete input-envelope body.
pub const INPUT_CASE_TAG: DomainTag =
    DomainTag::declared("trial-input", IdentityProfileVersion::declared(1));

/// The caller's named input convention, version and schema commitment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputProfile {
    name: NamespacedName,
    version: u32,
    schema: ContentAddress,
}

/// Independent ceilings on encoded envelope bytes and decoded specimen bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputLimits {
    envelope: usize,
    payload: usize,
}

crate::identity::content_address_reference! {
    /// The identity of one specimen under its exact input profile and schema.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InputCaseId;
}

/// One completely framed specimen under an admitted profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputEnvelope {
    profile: InputProfile,
    case: InputCaseId,
    payload: Vec<u8>,
    encoded: Vec<u8>,
}

/// A profile-specific typed decoder with its independently supplied executable revision.
pub struct InputBinding<Input> {
    profile: InputProfile,
    revision: RevisionBinding,
    decode: fn(&mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Input>,
}

/// A typed specimen retained beside the exact envelope and decoder revision that admitted it.
#[derive(Debug)]
pub struct BoundInput<Input> {
    value: Input,
    envelope: InputEnvelope,
    revision: RevisionBinding,
}

/// Why an envelope or typed specimen was not admitted.
#[must_use = "a refusal states why no input was admitted"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputRefusal {
    /// The complete envelope exceeds the independently supplied byte ceiling.
    EnvelopeTooLarge,
    /// The specimen exceeds the independently supplied byte ceiling.
    PayloadTooLarge,
    /// The encoded envelope size cannot be represented on this platform.
    SizeOutsidePlatform,
    /// A declared field ends before its framing admits it.
    Truncated,
    /// The body does not derive the leading content claim.
    AddressMismatch,
    /// The body names an unsupported input format.
    UnsupportedFormat {
        /// The format found in the body.
        found: u32,
    },
    /// The name or version differs from the independently expected input profile.
    ProfileMismatch,
    /// The schema differs from the independently expected schema commitment.
    SchemaMismatch,
    /// A declared length cannot be indexed on this platform.
    LengthOutsidePlatform {
        /// The unrepresentable length.
        declared: u64,
    },
    /// Complete envelope fields leave additional undeclared bytes.
    TrailingEnvelopeBytes {
        /// The number of bytes left unread.
        count: usize,
    },
    /// The caller's decoder refused the specimen.
    DecoderRefused(arbitrary::Error),
    /// The caller's decoder returned a value without consuming the complete specimen.
    TrailingInputBytes {
        /// The number of specimen bytes left unread.
        count: usize,
    },
}
