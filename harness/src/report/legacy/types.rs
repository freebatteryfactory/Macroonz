//! Sparse historical fields and their bounded admission vocabulary.

use crate::report::archive::AddressClaim;
use std::collections::BTreeSet;

#[path = "type_guard.rs"]
mod guard;

pub use guard::read_record;

/// The identity domain for exact historical JSON source bytes.
pub const LEGACY_SOURCE_TAG: crate::identity::DomainTag = crate::identity::DomainTag::declared(
    "historical-json-source",
    crate::identity::IdentityProfileVersion::declared(1),
);

/// Whether the source omitted a field, wrote null, or supplied a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegacyPresence<T> {
    /// No member was written.
    Missing,
    /// The member explicitly contained null.
    Null,
    /// The member contained this admitted historical value.
    Present(T),
}

/// The independently expected source labels for the supported field grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyProfile<'kind> {
    kind: &'kind str,
    schema: u64,
}

/// Independent ceilings defined by the [bounded reader](super#bounds).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyLimits {
    source: usize,
    text: usize,
    witness: usize,
    members: usize,
    depth: usize,
    retained: usize,
}

/// A field in the supported historical object or its partial profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LegacyField {
    /// The supplied kind header.
    Kind,
    /// The supplied schema header.
    Schema,
    /// The exact saved witness, including an explicitly empty specimen.
    Witness,
    /// The partial historical input convention.
    InputProfile,
    /// The source's trial label.
    TrialName,
    /// The source's subject label.
    SubjectName,
    /// The source's check label.
    CheckName,
    /// The source's opaque subject revision marker.
    SubjectRevision,
    /// The source's opaque check revision marker.
    CheckRevision,
    /// The source's target label.
    Target,
    /// The source's toolchain label.
    Toolchain,
    /// The claimed execution digest without its preimage.
    ExecutionDigest,
    /// The claimed fingerprint digest without its preimage.
    FingerprintDigest,
    /// The source's uninterpreted outcome label.
    ReportedOutcome,
    /// The input profile's uninterpreted name.
    ProfileName,
    /// The input profile's numeric version claim.
    ProfileRevision,
}

/// A partial historical profile without a schema or admitted current name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyInputProfile {
    name: LegacyPresence<String>,
    revision: LegacyPresence<u64>,
}

/// An owned source with only the historical fields it actually supplied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyRecord {
    source: Vec<u8>,
    kind: LegacyPresence<String>,
    schema: LegacyPresence<u64>,
    witness: LegacyPresence<Vec<u8>>,
    input_profile: LegacyPresence<LegacyInputProfile>,
    trial_name: LegacyPresence<String>,
    subject_name: LegacyPresence<String>,
    check_name: LegacyPresence<String>,
    subject_revision: LegacyPresence<u64>,
    check_revision: LegacyPresence<u64>,
    target: LegacyPresence<String>,
    toolchain: LegacyPresence<String>,
    execution_digest: LegacyPresence<AddressClaim>,
    fingerprint_digest: LegacyPresence<AddressClaim>,
    reported_outcome: LegacyPresence<String>,
}

/// Why no sparse historical record was admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyRefusal {
    /// The complete source exceeds its declared byte ceiling.
    SourceTooLarge,
    /// A decoded text value exceeds its byte ceiling.
    TextTooLarge,
    /// The witness exceeds its octet ceiling.
    WitnessTooLarge,
    /// The total object-member population exceeds its ceiling.
    TooManyMembers,
    /// A container exceeds the declared nesting ceiling.
    TooDeep,
    /// The retained logical bytes exceed their combined ceiling.
    RetainedTooLarge,
    /// Retained-byte or population arithmetic exceeds the platform.
    SizeOutsidePlatform,
    /// A decoded member name is outside the declared object shape.
    UnknownField,
    /// A decoded member appeared more than once.
    DuplicateField(LegacyField),
    /// The supplied kind header differs from the expected label.
    KindMismatch,
    /// The supplied schema header differs from the expected label.
    SchemaMismatch,
    /// A digest is not exactly sixty-four lowercase hexadecimal digits.
    InvalidDigest,
    /// JSON syntax, a value type or complete input consumption refused.
    InvalidJson {
        /// The decoder's one-based source line, or zero when unavailable.
        line: usize,
        /// The decoder's source column, or zero when unavailable.
        column: usize,
    },
}

struct Admission {
    limits: LegacyLimits,
    retained: usize,
    members: usize,
    seen: BTreeSet<LegacyField>,
    refusal: Option<LegacyRefusal>,
}

#[derive(Clone, Copy)]
enum Object {
    Record,
    Profile,
}

struct RecordSeed<'admission>(&'admission mut Admission);
struct ProfileSeed<'admission>(&'admission mut Admission);
struct TextSeed<'admission>(&'admission mut Admission);
struct NumberSeed<'admission>(&'admission mut Admission);
struct DigestSeed<'admission>(&'admission mut Admission);
struct WitnessSeed<'admission>(&'admission mut Admission);
struct KeySeed<'admission>(&'admission mut Admission, Object);
struct Nullable<Seed>(Seed);
