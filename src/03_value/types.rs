//! The value plane's laws, made machine-readable: how absence is classified, in
//! what order raw input earns trust, the stages a foreign value crosses to become
//! an accepted fact, and which lossy operations exist.
//!
//! # The no-null law
//!
//! Every foreign absence is classified exactly once, at decode, into its typed
//! axis; after admission, unclassified null does not exist. Where a schema
//! declares a field `Nullable`, the classified arrival is that schema's typed
//! null value — one declared meaning in its value domain, **never a universal
//! sentinel**. Accordingly this home ships no null type: the sentinel's
//! nonexistence is the design, and a universal sentinel appearing anywhere in
//! the machine is a defect.
//!
//! # Stages pipeline, never merge
//!
//! Field-name similarity, a valid transport message, or a successful decode
//! chooses no domain transformation and grants no admission. Automation may
//! pipeline the inbound stages but never merges them.

use crate::types::{Bounded, Limit};

/// The six absence worlds — closed. Classification routes each foreign absence
/// into the axis that owns it; this enum is the classification namespace, not a
/// result axis (it grows no lifecycle postures).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Absence {
    /// The shape declares the slot optional and it was not supplied.
    ShapeOptional,
    /// The schema's declared typed null — one meaning in that value domain.
    ValueNull,
    /// Present but not readable under the caller's authority.
    Unauthorized,
    /// Not yet materialized at the consulted cut.
    Unmaterialized,
    /// Not yet knowable — routes to the `Truth` knowledge axis, which owns the
    /// `Pending` word; this variant names that world, it is not a new posture.
    Pending,
    /// The outcome of an admitted effect is unknown — routes to the runtime's
    /// outcome-knowledge axis.
    OutcomeUnknown,
}

/// One check in the pre-authority validation ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreAuthorityCheck {
    /// Declared lengths hold.
    Lengths,
    /// Declared counts hold.
    Counts,
    /// Declared offsets are coherent.
    Offsets,
    /// Expansion stays within admitted bounds.
    Expansion,
    /// The bytes carry the expected role.
    Role,
}

/// The declared ladder: readers validate these five, in this order, before any
/// allocation or authority. The bytes home's readers cite this ladder; they
/// never restate it.
pub const PRE_AUTHORITY_LADDER: [PreAuthorityCheck; 5] = [
    PreAuthorityCheck::Lengths,
    PreAuthorityCheck::Counts,
    PreAuthorityCheck::Offsets,
    PreAuthorityCheck::Expansion,
    PreAuthorityCheck::Role,
];

/// One stage of the canonical inbound path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InboundStage {
    /// Carrier or host bytes arrive.
    CarrierBytes,
    /// Bounded physical framing and decode.
    BoundedFramingAndDecode,
    /// The result is a typed foreign claim — nothing more.
    TypedForeignClaim,
    /// Structural, version, identity, and replay validation.
    StructuralValidation,
    /// Application-owned semantic transformation.
    SemanticTransformation,
    /// Capability, policy, and authority admission.
    AuthorityAdmission,
    /// An accepted event or role-specific fact exists.
    AcceptedFact,
    /// Asynchronous derived materialization at its own cut.
    DerivedMaterialization,
}

/// The declared canonical inbound path: eight stages that pipeline and never
/// merge.
pub const CANONICAL_INBOUND_PATH: [InboundStage; 8] = [
    InboundStage::CarrierBytes,
    InboundStage::BoundedFramingAndDecode,
    InboundStage::TypedForeignClaim,
    InboundStage::StructuralValidation,
    InboundStage::SemanticTransformation,
    InboundStage::AuthorityAdmission,
    InboundStage::AcceptedFact,
    InboundStage::DerivedMaterialization,
];

/// The seven lossy operations — closed, and they stay distinct: never collapsed
/// into one generic transform. Each owner performing one owes its own
/// disclosure row (policy, discarded distinctions, reversibility posture,
/// explanation, evidence).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LossyOperation {
    /// Value quantization under a declared rounding contract.
    Quantization,
    /// Removal of protected or unreleased content.
    Redaction,
    /// Reduction to a summary that discards members.
    Summarization,
    /// Selection of declared components of a value.
    Projection,
    /// Selection of a subset by sampling policy.
    Sampling,
    /// Cutting to a declared bound.
    Truncation,
    /// Filtering by a declared predicate.
    Selection,
}

/// The pinned Unicode version of the machine's text admission profile — the old
/// book's own pin, machine-readable.
pub const TEXT_PROFILE_UNICODE_PIN: &str = "17.0.0";

/// The value-plane text-defect vocabulary — the roster schemas draw from when
/// declaring text refinements. **Per-schema force, never universal admission**:
/// a schema whose text shape is a single-line label may refuse controls; a
/// multi-line memo schema does not. The language's lexical rules (the sealed
/// capsule's single-line law) died with the language; what survives is
/// value-plane: where a schema declares normalized text, non-NFC refuses and is
/// never silently normalized, and value-level validation is explicit because
/// NFC is not closed under concatenation — normalization protects canonical
/// bytes and digest identity, not syntax. Bidirectional-control refusal serves
/// the data-as-instruction firewall (hostile text riding as data), not lexing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAdmissionIssue {
    /// A control character (including tab, newline, carriage return).
    DisallowedControl,
    /// A line or paragraph separator.
    DisallowedSeparator,
    /// A surrogate code unit in decoded input.
    Surrogate,
    /// A Unicode noncharacter.
    Noncharacter,
    /// A bidirectional ordering control.
    BidirectionalControl,
    /// A default-ignorable scalar outside the admitted set.
    DisallowedDefaultIgnorable,
    /// ZWNJ/ZWJ or a variation selector outside an admitted joining or emoji
    /// context (kept a separate cause because its repair is a different act).
    InvalidJoinControlContext,
    /// The scalar sequence is not in NFC.
    NotNfc,
}

/// One established text-admission issue: the kind, the one offending scalar,
/// and its typed coordinate — nothing further; no issue is payload-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextIssue {
    /// The defect kind.
    pub kind: TextAdmissionIssue,
    /// The one offending scalar.
    pub scalar: char,
    /// The scalar coordinate in the flattened stream.
    pub coordinate: u32,
}

/// Bounded text as a semantic value, carrying its limit family like every other
/// bounded value. Admission checks are the declaring schema's selected text
/// refinements (drawn from [`TextAdmissionIssue`]); the checker rides an
/// admitted external Unicode mechanism behind a machine-owned role contract —
/// the mechanism is swappable, the role contract is not. The constructor
/// lands with that mechanism's admission.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundedText<L: Limit> {
    bytes: Bounded<u8, L>,
}

impl<L: Limit> BoundedText<L> {
    /// Byte length of the text.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the text is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}
