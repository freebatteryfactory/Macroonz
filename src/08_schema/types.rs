//! The schema plane: the nine-meaning non-collapse, contracts, the schema
//! model, the four value-shape axes, refinements, the validation pipeline,
//! validated views, structured issues, migration, compatibility, codec
//! profiles, and the seven construction-refusal families.
//!
//! # The nine-meaning non-collapse
//!
//! contract ≠ schema ≠ codec ≠ layout ≠ occurrence ≠ Rust type ≠ runtime schema
//! descriptor ≠ migration. One schema may have several codecs, layouts,
//! generated Rust realizations, and occurrences without their bytes,
//! identities, authority, or lifecycle becoming interchangeable. Rust memory
//! layout is not a durable schema; a derive implementation is not a semantic
//! owner; successful deserialization proves no schema validity, compatibility,
//! authority, admission, or acceptability. There is ONE descriptor type: an
//! admitted `RuntimeSchemaDescriptor` remains validated descriptor data and
//! gains no authority from any use — "meta descriptor" is a role phrase, never
//! a second type.
//!
//! # The type-and-schema dual agreement law
//!
//! One admitted semantic declaration drives both the Rust realization and the
//! language-neutral runtime schema. The two routes must agree: a typed
//! constructor accepts iff the portable schema accepts; a generated validator
//! accepts iff the independent interpreter accepts; a borrowed validated view
//! exposes exactly what the owned value would. A disagreement is a
//! qualification failure, and neither route wins by being faster or generated.
//! Generation grants no authority; a generated validator and codec cannot be
//! their own only oracle.
//!
//! # Version-role non-substitution
//!
//! A transport version cannot upgrade an image; a schema version cannot
//! substitute for a codec version; a layout version cannot define schema
//! meaning; a release version substitutes for none of them. Version ordering
//! alone proves no compatibility. No refusal variant exists for version-role
//! substitution: opaque owner-minted identity roles make it inexpressible
//! rather than merely unlawful.

use crate::identity::{
    AuthorityPosition, ByteIdentity, Commitment, CreationLaw, IdentityClass, IdentityRole,
    Occurrence,
};
use crate::refusal::{CompletionPosture, FamilyShape, ReasonId, RefusalFamily};
use crate::types::{Bounded, ConstLimit, EvidenceRef, Limit, NonEmptyBounded};
use crate::value::BoundedText;

// ---------------------------------------------------------------------------
// Identity instantiations — five production uses of the class calculus.
// ---------------------------------------------------------------------------

/// The identity role marker for schema families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaFamilyRole;

/// One schema family — Class D, fresh: a family is a happening, not content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaFamilyId(Occurrence<SchemaFamilyRole>);

impl IdentityRole for SchemaFamilyId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

/// One schema version — Class C, a u64 position scoped to its family with the
/// scope binding in the value: the first production instantiation of the
/// scope-guarded order shape. No `Ord` exists; comparison is same-scope only.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaVersion(AuthorityPosition<SchemaFamilyId>);

/// The identity role marker for fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldRole;

/// One stable field identity — Class D, fresh, never silently recycled: a
/// removed member reserves its identity unless reuse cannot be confused by any
/// supported reader, writer, migration, or history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldId(Occurrence<FieldRole>);

impl IdentityRole for FieldId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

/// The identity role marker for variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VariantRole;

/// One stable variant identity — Class D, fresh, never silently recycled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VariantId(Occurrence<VariantRole>);

impl IdentityRole for VariantId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

/// The domain marker for schema semantic commitments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaMeaningDomain;

/// The schema's semantic commitment — Class A over normalized schema meaning,
/// distinct from the descriptor digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaSemanticCommitment(Commitment<SchemaMeaningDomain>);

impl IdentityRole for SchemaSemanticCommitment {
    const CLASS: IdentityClass = IdentityClass::SemanticCommitment;
    const CREATION: CreationLaw = CreationLaw::DomainTaggedDigestOfMeaning;
}

impl SchemaSemanticCommitment {
    /// In-crate mint for laws. Test-gated until digest derivation exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(commitment: Commitment<SchemaMeaningDomain>) -> Self {
        Self(commitment)
    }
}

/// The identity role marker for schema descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaDescriptorRole;

/// The descriptor's exact-byte digest — Class B, never substitutable for the
/// semantic commitment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaDescriptorDigest(ByteIdentity<SchemaDescriptorRole>);

impl IdentityRole for SchemaDescriptorDigest {
    const CLASS: IdentityClass = IdentityClass::ByteDigest;
    const CREATION: CreationLaw = CreationLaw::DigestOfExactBytes;
}

// ---------------------------------------------------------------------------
// Contracts.
// ---------------------------------------------------------------------------

/// The eight semantic axes a valid contract makes explicit, finite,
/// owner-bound, reference-closed, and internally coherent. An omitted
/// applicable axis is absent by law — never left for a mechanism,
/// implementation, generated artifact, runtime observation, or ambient state to
/// invent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractAxis {
    /// Inputs and normal results.
    InputsAndResults,
    /// Invariants and lawful transitions.
    InvariantsAndTransitions,
    /// Effects and authority/capability requirements.
    EffectsAndAuthority,
    /// Source and cut requirements, where applicable.
    SourceAndCuts,
    /// Bounds.
    Bounds,
    /// Refusal, uncertainty, and recovery behavior.
    RefusalUncertaintyRecovery,
    /// Explanation and evidence obligations.
    ExplanationAndEvidence,
    /// Compatibility / evolution posture.
    CompatibilityPosture,
}

/// Limit family for a contract's declared-axis set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContractAxisLimit;
impl Limit for ContractAxisLimit {}

/// An authored checked contract declaration (authored v1 core: the declared
/// axes; per-axis content rides the declaration surfaces).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Contract {
    /// The axes this contract declares.
    pub declared_axes: Bounded<ContractAxis, ContractAxisLimit>,
}

// ---------------------------------------------------------------------------
// The four value-shape axes — four closed enums, never one.
// ---------------------------------------------------------------------------

/// Field cardinality. Optional does not imply nullable; missing implies none of
/// false/zero/empty/shredded/unavailable/unauthorized — a field's shape is not
/// its resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldCardinality {
    /// Exactly one.
    Required,
    /// Zero or one.
    Optional,
    /// Zero or more.
    Repeated,
}

/// Nullability — null is a value-domain fact, distinct from absence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nullability {
    /// The domain has no null inhabitant.
    NonNullable,
    /// The domain declares its one typed null inhabitant.
    Nullable,
}

/// The domain marker for bounded-lane transform identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransformDomain;

/// A declared transformation in the bounded evaluation lane — never an
/// arbitrary host closure; no unbounded work on the admitted path (authored:
/// identified by its Class-A commitment).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransformRef(Commitment<TransformDomain>);

/// Default policy. A default is never silently inserted by validation because a
/// Rust field has `Default` — insertion is an explicit named transformation
/// with inspectable source/target version, value, policy, effect, and evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefaultPolicy {
    /// No default exists.
    NoDefault,
    /// The declared bounded-lane transformation.
    Declared(TransformRef),
}

/// Unknown-member policy — the fourth axis. The schema declares the semantic
/// law; the codec declares the physical mechanism. Skipping is not
/// automatically lossless; preserving opaque bytes is not understanding them;
/// no extension smuggles an effect, capability, executable operation, authority
/// claim, or unbounded allocation through a field treated as ignorable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnknownMemberPolicy {
    /// Every unknown member refuses.
    Closed,
    /// Skippable or preservable without changing base meaning.
    OptionalExtension,
    /// A reader that does not understand refuses.
    RequiredExtension,
    /// Carried without interpretation or authority. This is monotone
    /// extendability, NOT pending: there is no future in which THIS schema
    /// determines an unknown member.
    OpaquePreserved,
}

// ---------------------------------------------------------------------------
// Refinements.
// ---------------------------------------------------------------------------

/// The nine registered refinement kinds — closed, because an unregistered kind
/// retains no reference meaning another implementation can evaluate. Kinds are
/// what a refinement IS, never why one refuses: a range refinement and a
/// uniqueness refinement fail `NotTotal` identically. Contextual claims
/// (database uniqueness, current authorization, secret availability,
/// foreign-key existence, remote attestation, present-time policy) are never
/// pure refinements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefinementKind {
    /// Integer/value range.
    Range,
    /// Length constraint.
    Length,
    /// Set membership.
    Membership,
    /// Cross-field equality/order.
    CrossField,
    /// Unit compatibility.
    Unit,
    /// Interval well-formedness.
    Interval,
    /// Variant-dependent conditions.
    VariantDependent,
    /// Collection uniqueness.
    Uniqueness,
    /// Well-founded recursive measure.
    Measure,
}

/// The nine declared properties every refinement must hold — this roster IS the
/// refinement-construction refusal's subject matter.
pub const REFINEMENT_PROPERTIES: [&str; 9] = [
    "pure",
    "total-over-declared-domain",
    "bounded",
    "deterministic",
    "language-neutral",
    "inspectable",
    "independently-executable",
    "explicit-about-exceptional-values",
    "explicit-about-failure-reason",
];

// ---------------------------------------------------------------------------
// The validation pipeline.
// ---------------------------------------------------------------------------

/// One stage of the staged validation boundary, each refusing for its own
/// reason. Malformed bytes are not a schema failure; a structurally valid value
/// can fail a refinement; a schema-valid value can still be unauthorized,
/// unavailable, stale, unsupported, or inadmissible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationStage {
    /// Untrusted bytes or a foreign value arrive.
    UntrustedInput,
    /// Role and bound preflight.
    RoleAndBoundPreflight,
    /// Bounded codec parsing and canonicality check.
    CodecParsing,
    /// Structural schema validation.
    StructuralValidation,
    /// Pure bounded semantic refinements.
    Refinements,
    /// A validated borrowed or owned value exists.
    ValidatedValue,
    /// Contextual evidence/capability/policy/admission checks — explicit
    /// operations, never hidden inside validation.
    ContextualAdmission,
}

/// The declared pipeline. Validation performs no hidden I/O, ambient-state
/// query, key fetch, clock read, service contact, or effect — the
/// refinement-versus-contextual-admission boundary.
pub const VALIDATION_PIPELINE: [ValidationStage; 7] = [
    ValidationStage::UntrustedInput,
    ValidationStage::RoleAndBoundPreflight,
    ValidationStage::CodecParsing,
    ValidationStage::StructuralValidation,
    ValidationStage::Refinements,
    ValidationStage::ValidatedValue,
    ValidationStage::ContextualAdmission,
];

/// A validated borrowed view into bounded input bytes, exposing only
/// schema-proven meaning.
///
/// The borrow discipline stands today: the lifetime parameter is what makes the
/// view unable to outlive or detach from the bytes it validated, and no method
/// here hands the extent back out. The SCOPE of the claim is settled too, and is
/// a statement about what this type will never mean — it proves only memory
/// validity and the validation its constructor represents, never current
/// authority, freshness, generation compatibility, or external durability; those
/// remain runtime witnesses established elsewhere.
///
/// The constructor it names is owed. No road mints a [`ValidatedView`] today,
/// because minting one is the validator running against a schema, and the
/// validator lands when implementation opens for this home on explicit
/// authorization. The type is a declaration of what a validated view will prove,
/// and nothing has proven one yet.
#[derive(Debug)]
pub struct ValidatedView<'bytes> {
    extent: &'bytes [u8],
    schema: SchemaSemanticCommitment,
}

impl ValidatedView<'_> {
    /// The exact schema this view was validated under.
    #[must_use]
    pub fn schema(&self) -> SchemaSemanticCommitment {
        self.schema
    }

    /// The validated extent's byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.extent.len()
    }

    /// Whether the validated extent is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.extent.is_empty()
    }
}

/// A validated owned value: the same meaning without incidental input
/// representation, materialized only within declared bounds. Declared alongside
/// [`ValidatedView`] and minted by nothing for the same reason — the
/// materialization road is the validator's, and it is owed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidatedOwned {
    schema: SchemaSemanticCommitment,
}

impl ValidatedOwned {
    /// The exact schema this value was validated under.
    #[must_use]
    pub fn schema(&self) -> SchemaSemanticCommitment {
        self.schema
    }
}

// ---------------------------------------------------------------------------
// Structured validation issues.
// ---------------------------------------------------------------------------

/// Limit family for field paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FieldPathLimit;
impl Limit for FieldPathLimit {}

/// One segment of a stable issue path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathSegment {
    /// A field, by stable identity.
    Field(FieldId),
    /// A variant, by stable identity.
    Variant(VariantId),
    /// A collection index.
    Index(u32),
    /// A reference hop.
    Reference,
}

/// A stable field/variant/index/reference path.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldPath {
    /// The segments, outermost first.
    pub segments: Bounded<PathSegment, FieldPathLimit>,
}

/// Limit family for issue text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IssueTextLimit;
impl Limit for IssueTextLimit {}

/// One structured validation issue — evidence, not prose. Aggregation is
/// bounded (hostile input allocates no unbounded error tree; stopping reports
/// incomplete enumeration), and localized narration is a projection that
/// neither defines stable issue identity nor participates in commitments.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValidationIssue {
    /// The schema family.
    pub family: SchemaFamilyId,
    /// The schema version.
    pub version: SchemaVersion,
    /// The stable issue path.
    pub path: FieldPath,
    /// The expected condition.
    pub expected: BoundedText<IssueTextLimit>,
    /// The observed classification — without leaking prohibited data.
    pub observed: BoundedText<IssueTextLimit>,
    /// The stable reason identity.
    pub reason: ReasonId,
    /// Whether validation completed or stopped at a declared bound.
    pub posture: CompletionPosture,
}

// ---------------------------------------------------------------------------
// Migration and dynamic values.
// ---------------------------------------------------------------------------

/// The closed twelve-member migration-boundary vocabulary. The generic word
/// "upgrade" authorizes none of them to perform another silently; a
/// declaration vocabulary is never coarser than the consequences it declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MigrationBoundary {
    /// Source-language change.
    SourceLanguage,
    /// Schema-meaning change.
    SchemaMeaning,
    /// Codec re-encoding.
    CodecReencoding,
    /// Image-format change.
    ImageFormat,
    /// Accepted-history format change.
    AcceptedHistoryFormat,
    /// Layout rematerialization.
    LayoutRematerialization,
    /// Derived-block rebuild.
    DataBlockRebuild,
    /// Protected re-encryption.
    ProtectedReencryption,
    /// Key rewrap — replaces wrapping without moving the key lineage.
    KeyRewrap,
    /// Key rotation — advances the secret-authority generation. A separate
    /// crossing from rewrap on purpose.
    KeyRotation,
    /// Application data correction.
    ApplicationDataCorrection,
    /// Effectful backfill.
    EffectfulBackfill,
}

/// The closed six-member protected-data transformation vocabulary.
/// Re-encryption or rewrap drift none of event identity, schema identity,
/// store lineage, key scope, or protected meaning; shred migrates protected
/// data into none of Missing, empty bytes, a default, or nullable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtectedDataTransformation {
    /// Replaces ciphertext under a declared protection operation.
    Reencryption,
    /// Replaces the wrapping of an existing key without changing meaning.
    KeyRewrap,
    /// Advances the secret-authority generation.
    KeyRotation,
    /// Changes logical interpretation.
    SchemaMigration,
    /// Changes physical representation.
    CodecReencoding,
    /// Durably destroys key authority; resolution becomes shredded.
    Shred,
}

/// A bounded dynamic semantic value for interpreting a historical or foreign
/// schema whose static Rust type is not compiled in. Supports only the closed
/// value algebra; retains exact schema/version binding and stable identities;
/// enforces all bounds; carries no live capabilities, secrets, host objects, or
/// code; never becomes the universal runtime representation and never erases
/// the stronger target Rust type after validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynamicValue {
    /// The exact schema binding.
    pub schema: SchemaSemanticCommitment,
}

// ---------------------------------------------------------------------------
// Compatibility.
// ---------------------------------------------------------------------------

/// Limit family for edge text members.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeLimit;
impl Limit for EdgeLimit {}

/// The claim marker for compatibility evidence references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompatibilityClaim;

/// Edge direction — added by the directedness sentence: compatibility is a
/// directed relationship among exact contracts, never inferred from version
/// numbers or parser luck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeDirection {
    /// Source interprets/produces toward target.
    SourceToTarget,
    /// Target interprets/produces toward source.
    TargetToSource,
}

/// One declared compatibility edge — the nine members (authored v1 cores). The
/// twelve compatibility axes and six seed labels stay deliberately OPEN
/// vocabulary: contradiction is judged over the axes an edge declares, never
/// over a frozen roster. `ThreadPak` defines the algebra; promising a row belongs
/// to the release owner and is consumed here, never re-authored.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompatibilityEdge {
    /// The source contract.
    pub source: SchemaSemanticCommitment,
    /// The target contract.
    pub target: SchemaSemanticCommitment,
    /// The operation the edge governs.
    pub operation: BoundedText<EdgeLimit>,
    /// The direction.
    pub direction: EdgeDirection,
    /// The declared assumptions.
    pub assumptions: BoundedText<EdgeLimit>,
    /// The unknown-field posture under this edge.
    pub unknown_field_posture: UnknownMemberPolicy,
    /// The declared losses.
    pub losses: BoundedText<EdgeLimit>,
    /// The supporting evidence.
    pub evidence: EvidenceRef<CompatibilityClaim>,
    /// The promise owner.
    pub promise_owner: BoundedText<EdgeLimit>,
}

// ---------------------------------------------------------------------------
// Codec profiles — seated here by band math: a codec binds an admitted schema
// relationship, and bytes (07) cannot import schema (08). Bytes owns the
// primitives; this home owns the declaration that binds them to schemas.
// ---------------------------------------------------------------------------

/// An authored codec profile (AUTHORED v1 core: the bound schema; the full
/// fifteen-item roster is documented law — the 14-vs-15 count discrepancy is
/// flagged and unresolved).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecProfile {
    /// The admitted schema relationship.
    pub schema: SchemaSemanticCommitment,
}

// ---------------------------------------------------------------------------
// Payload rosters the issue variants name.
// ---------------------------------------------------------------------------

/// Which of the four value-shape axes an incomplete declaration omitted — the
/// variant is unreportable without it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueShapeAxis {
    /// Field cardinality.
    Cardinality,
    /// Nullability.
    Nullability,
    /// Default policy.
    DefaultPolicy,
    /// Unknown-member policy.
    UnknownMemberPolicy,
}

/// Which of the four forbidden weakenings a composition attempted — none is
/// lawful merely because the host Rust type can represent more values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeakeningKind {
    /// Weakening a child invariant.
    WeakenChildInvariant,
    /// Erasing an unknown-field policy.
    EraseUnknownFieldPolicy,
    /// Broadening a refinement.
    BroadenRefinement,
    /// Removing a bound.
    RemoveBound,
}

/// Which of the five collision axes two imports collided on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportCollisionAxis {
    /// Display name.
    DisplayName,
    /// Durable identity.
    DurableIdentity,
    /// Field identity.
    FieldIdentity,
    /// Version role.
    VersionRole,
    /// Incompatible commitment.
    IncompatibleCommitment,
}

/// Which of the eight checkable objects a layout left uncheckable before use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckableObject {
    /// Offsets.
    Offsets,
    /// Lengths.
    Lengths,
    /// Alignment.
    Alignment,
    /// Overlap.
    Overlap,
    /// Allocation.
    Allocation,
    /// Decoding.
    Decoding,
    /// Expansion.
    Expansion,
    /// Selective access.
    SelectiveAccess,
}

/// Which preservation object a layout failed to preserve — AUTHORED as seven
/// (the null-and-extension law reads as one object); the 5-vs-7 count
/// ambiguity is flagged, not resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreservationObject {
    /// Schema meaning.
    SchemaMeaning,
    /// Field identity.
    FieldIdentity,
    /// The null-and-extension law.
    NullAndExtensionLaw,
    /// Source binding.
    SourceBinding,
    /// Row-domain binding.
    RowDomainBinding,
    /// Cut binding.
    CutBinding,
    /// Generation binding.
    GenerationBinding,
}

// ---------------------------------------------------------------------------
// The seven construction-refusal families. All collection-shaped: role-specific
// issue carriers in role-specific bounded collections (a schema issue and a
// codec issue never typecheck in each other's position), family-level reason,
// no primary issue, posture as instance value, compile-time bound = the
// roster's own cardinality. Boundary rule: a construction family carries only
// facts its constructor can establish from the declaration in front of it.
// ---------------------------------------------------------------------------

/// The contract-construction issue roster (six).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractConstructionIssue {
    /// An applicable axis is missing — names the axis.
    ApplicableAxisMissing(ContractAxis),
    /// A reference is unresolved.
    ReferenceUnresolved,
    /// The declaration contradicts itself.
    Contradictory,
    /// The declaration is not finite.
    Unbounded,
    /// The declaration has no owner.
    Ownerless,
    /// The declaration defers to a mechanism.
    MechanismDefined,
}

/// Compile-time bound for contract issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContractIssueLimit;
impl Limit for ContractIssueLimit {}
impl ConstLimit for ContractIssueLimit {
    const MAX: usize = 6;
}

/// Contract construction — posture normally Complete (fixed finite axis set,
/// no recursion). Claims nothing about whether a behavior SHOULD declare a
/// given axis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContractConstruction {
    /// The established issues.
    pub issues: NonEmptyBounded<ContractConstructionIssue, ContractIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for ContractConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// The refinement-construction issue roster (eleven).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefinementConstructionIssue {
    /// Hidden I/O or effect — the purity property.
    HiddenIoOrEffect,
    /// Not total over its declared input domain.
    NotTotal,
    /// Not bounded.
    Unbounded,
    /// Not deterministic.
    Nondeterministic,
    /// Not language-neutral.
    NotLanguageNeutral,
    /// Not inspectable.
    NotInspectable,
    /// The declaration retains no reference meaning another implementation can
    /// evaluate — the declaration-side obligation, not the qualification
    /// chapter's independent-route requirement.
    NotIndependentlyEvaluable,
    /// No exceptional-value posture declared.
    ExceptionalValuePostureMissing,
    /// No failure reason declared.
    FailureReasonMissing,
    /// A contextual claim declared as a refinement — a stated category error,
    /// not smuggling.
    ContextualClaimDeclared,
    /// The declared kind is not registered — names the kind as authored.
    RefinementKindNotRegistered,
}

/// Compile-time bound for refinement issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefinementIssueLimit;
impl Limit for RefinementIssueLimit {}
impl ConstLimit for RefinementIssueLimit {
    const MAX: usize = 11;
}

/// Refinement construction — the one family in the register whose posture is
/// ALWAYS Complete: one object, no site structure, no bound to stop at.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefinementConstruction {
    /// The established issues.
    pub issues: NonEmptyBounded<RefinementConstructionIssue, RefinementIssueLimit>,
    /// The enumeration posture — always Complete for this family.
    pub posture: CompletionPosture,
}

impl RefusalFamily for RefinementConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// The migration-construction issue roster (eleven).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MigrationConstructionIssue {
    /// No boundary declared from the closed twelve.
    MigrationBoundaryMissing,
    /// The declared boundary understates the consequences — carries both.
    BoundaryUnderstated {
        /// The boundary declared.
        declared: MigrationBoundary,
        /// The boundary the consequences imply.
        implied: MigrationBoundary,
    },
    /// Source or target missing — carries the construction-side residue of
    /// exact-base binding.
    SourceOrTargetMissing,
    /// A declared path is not adjacent.
    DeclaredPathNotAdjacent,
    /// A default or transformation is not declared.
    DefaultOrTransformationNotDeclared,
    /// Hidden I/O or effect.
    HiddenIoOrEffect,
    /// Loss posture missing — binds loss, narrowing, ambiguity, unsupported;
    /// no separate partiality variant ("partial" is the compatibility axis
    /// roster's word).
    LossPostureMissing,
    /// Not deterministic.
    Nondeterministic,
    /// A transform outside the bounded lane, reached through declared
    /// transforms.
    TransformOutsideBoundedLane,
    /// The edge declares an accepted-history rewrite — migration is not
    /// history erasure.
    AcceptedHistoryRewriteDeclared,
    /// The edge maps shred to absence — the mapping prohibition only; shred
    /// authority is the security home's.
    ShredMappedToAbsence,
}

/// Compile-time bound for migration issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MigrationIssueLimit;
impl Limit for MigrationIssueLimit {}
impl ConstLimit for MigrationIssueLimit {
    const MAX: usize = 11;
}

/// Migration construction — enumeration over the edge's members and a composed
/// path's edges (the exact path stays reportable); a long path reports
/// `EarlyStopped`. Apply-time facts (wrong-base application, target validation,
/// atomic visibility) stay out; independent testability has no
/// declaration-side counterpart.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MigrationConstruction {
    /// The established issues.
    pub issues: NonEmptyBounded<MigrationConstructionIssue, MigrationIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for MigrationConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// The compatibility-edge construction issue roster (eleven).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompatibilityEdgeConstructionIssue {
    /// Source or target missing.
    SourceOrTargetMissing,
    /// Operation missing.
    OperationMissing,
    /// Direction missing.
    DirectionMissing,
    /// Assumptions missing.
    AssumptionsMissing,
    /// Unknown-field posture missing.
    UnknownFieldPostureMissing,
    /// Loss posture missing.
    LossPostureMissing,
    /// Evidence missing.
    EvidenceMissing,
    /// Promise owner missing — this home checks only that an owner is named,
    /// never whether a promise exceeds its evidence.
    PromiseOwnerMissing,
    /// Version ordering substituted for compatibility.
    VersionOrderSubstitution,
    /// Parser behavior substituted for compatibility — minted together with
    /// the above: "this implementation currently deserializes it" is no claim.
    ParserBehaviorSubstitution,
    /// Two declared axes contradict — names both.
    ContradictoryDeclaredAxes {
        /// The first contradicting axis, as declared.
        first: BoundedText<EdgeLimit>,
        /// The second contradicting axis, as declared.
        second: BoundedText<EdgeLimit>,
    },
}

/// Compile-time bound for compatibility-edge issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompatibilityIssueLimit;
impl Limit for CompatibilityIssueLimit {}
impl ConstLimit for CompatibilityIssueLimit {
    const MAX: usize = 11;
}

/// Compatibility-edge construction — posture normally Complete over the nine
/// members (no recursive structure).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompatibilityEdgeConstruction {
    /// The established issues.
    pub issues: NonEmptyBounded<CompatibilityEdgeConstructionIssue, CompatibilityIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for CompatibilityEdgeConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// The schema-construction issue roster (eighteen). The three nested-cause
/// variants stay separate precisely because they nest different families —
/// one fused variant would carry a union-typed nested cause, the forbidden
/// fusion.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchemaConstructionIssue {
    /// Identity or version declaration incomplete.
    IdentityOrVersionIncomplete,
    /// Two members collide on a stable identity.
    FieldOrVariantIdentityCollision,
    /// A stable identity was recycled.
    FieldOrVariantIdentityRecycled,
    /// A value-shape axis is undeclared — names which of the four.
    ValueShapeIncomplete(ValueShapeAxis),
    /// Numeric or unit domain incomplete.
    ValueDomainIncomplete,
    /// Collection or recursion bounds missing.
    CollectionOrRecursionBoundsMissing,
    /// Recursive data without a well-founded shape.
    RecursionNotWellFounded,
    /// Composition weakens a child — names which of the four weakenings.
    CompositionWeakensChild(WeakeningKind),
    /// Two imports collide — names which of the five axes.
    ImportCollision(ImportCollisionAxis),
    /// A declared refinement is invalid — nests that family's refusal.
    RefinementInvalid(Box<RefinementConstruction>),
    /// A default or transformation is incomplete.
    DefaultOrTransformationIncomplete,
    /// A transform outside the bounded lane.
    TransformOutsideBoundedLane,
    /// Reference closure incomplete.
    ReferenceClosureIncomplete,
    /// Reference-frame or axis declaration incomplete.
    ReferenceFrameOrAxisIncomplete,
    /// Per-field classification posture missing — an unlabeled field is absent
    /// by law, never public by default.
    ClassificationPostureMissing,
    /// Canonical valid-and-invalid example binding missing.
    CanonicalExampleBindingMissing,
    /// A compatibility relationship is invalid — nests that family's refusal.
    CompatibilityRelationshipInvalid(Box<CompatibilityEdgeConstruction>),
    /// A migration edge is invalid — nests that family's refusal.
    MigrationEdgeInvalid(Box<MigrationConstruction>),
}

/// Compile-time bound for schema issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaIssueLimit;
impl Limit for SchemaIssueLimit {}
impl ConstLimit for SchemaIssueLimit {
    const MAX: usize = 18;
}

/// Schema construction — the family that most often reports `EarlyStopped`
/// (recursive declarations reach the bound). No variant refuses a version-role
/// substitution: opaque owner-minted identity roles make it inexpressible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaConstruction {
    /// The established issues.
    pub issues: NonEmptyBounded<SchemaConstructionIssue, SchemaIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for SchemaConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// The layout-construction issue roster (fourteen). `OutOfRange` and
/// `WrongProfile` trace to layout validity verbatim, not to byte law — the
/// class-level byte law and the frame grammar are byte-plane rows with their
/// own typed refusals, and this family does not reach for them. No variant
/// refuses a workload mismatch (declaration is required; mismatch is
/// application time).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutConstructionIssue {
    /// The semantic or schema role is missing — keeps both halves.
    SemanticOrSchemaRoleMissing,
    /// The declared workload is missing.
    WorkloadMissing,
    /// A component, field, or relationship is not closed — keeps all three.
    ComponentFieldOrRelationshipNotClosed,
    /// Deterministic order or explicit reversible permutation missing.
    OrderOrPermutationMissing,
    /// A bound is not checkable before use — names which of the eight objects.
    BoundsNotCheckableBeforeUse(CheckableObject),
    /// A preservation object is not preserved — names which.
    MeaningOrBindingNotPreserved(PreservationObject),
    /// Authority claimed from physical arrangement.
    AuthorityClaimedFromArrangement,
    /// Ambiguous organization.
    Ambiguous,
    /// Overlapping organization.
    Overlapping,
    /// Out-of-range organization.
    OutOfRange,
    /// Wrong-role organization.
    WrongRole,
    /// Wrong-profile organization.
    WrongProfile,
    /// Non-reconstructable organization.
    NonReconstructable,
    /// Unbounded organization.
    Unbounded,
}

/// Compile-time bound for layout issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayoutIssueLimit;
impl Limit for LayoutIssueLimit {}
impl ConstLimit for LayoutIssueLimit {
    const MAX: usize = 14;
}

/// Layout construction — enumeration runs over components and fields; a wide
/// layout reports `EarlyStopped`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LayoutConstruction {
    /// The established issues.
    pub issues: NonEmptyBounded<LayoutConstructionIssue, LayoutIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for LayoutConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// The codec-construction issue roster (seventeen). Decode-time wrong-role
/// refusal belongs to the codec's own declared refusal taxonomy (whose ABSENCE
/// refuses here); reader behavior and round-trip disagreement are
/// qualification verdicts, not construction facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CodecConstructionIssue {
    /// Codec identity or version missing.
    CodecIdentityOrVersionMissing,
    /// The admitted schema relationship is missing.
    SchemaRelationshipMissing,
    /// The artifact or field role is missing.
    ArtifactOrFieldRoleMissing,
    /// Framing or component order missing.
    FramingOrComponentOrderMissing,
    /// Numeric or offset representation missing.
    NumericOrOffsetRepresentationMissing,
    /// Collection ordering missing.
    CollectionOrderingMissing,
    /// Duplicate and trailing-data posture missing.
    DuplicateOrTrailingDataPostureMissing,
    /// Unknown-field physical handling missing.
    UnknownFieldHandlingMissing,
    /// The physical mechanism contradicts the bound schema's declared policy.
    UnknownFieldHandlingContradictsSchemaPolicy,
    /// Extension preservation missing.
    ExtensionPreservationMissing,
    /// Bounds-before-allocation missing.
    BoundsBeforeAllocationMissing,
    /// Encode/decode and canonical re-encode behavior missing.
    EncodeDecodeOrReencodeBehaviorMissing,
    /// Selective access skips required validation.
    SelectiveAccessSkipsRequiredValidation,
    /// The refusal taxonomy is missing.
    RefusalTaxonomyMissing,
    /// No typed reference binds the canonical and hostile vector sets.
    CanonicalOrHostileVectorsMissing,
    /// The profile admits two byte forms for one semantic value.
    CanonicalEncodingNotUnique,
    /// Raw memory, discriminants, derive behavior, debug output, or serializer
    /// defaults adopted as durable bytes.
    HostRepresentationAsCanonicalBytes,
}

/// Compile-time bound for codec issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodecIssueLimit;
impl Limit for CodecIssueLimit {}
impl ConstLimit for CodecIssueLimit {
    const MAX: usize = 17;
}

/// Codec construction — posture normally Complete over the profile items; a
/// partially authored profile typically reports several at once. Seated here
/// by band math (the profile binds a schema relationship).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecConstruction {
    /// The established issues.
    pub issues: NonEmptyBounded<CodecConstructionIssue, CodecIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for CodecConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}
