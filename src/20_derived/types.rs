//! Derived data: `DataBlock` law, the two-seat row-domain identity, columns,
//! selection masks, payload locators, materialization, and physical plans.
//!
//! # The four core rules
//!
//! R1 — derived state is not authority: a `DataBlock` is always derived
//! physical acceleration; no validation, sealing, encryption, persistence,
//! attestation, qualification, or difficulty of reconstruction promotes it.
//! R2 — every derived occurrence binds exact authoritative sources and cuts.
//! R3 — deleting derived state changes only latency, memory, I/O, and
//! rebuild work — never accepted facts, query meaning, domain decisions,
//! effect legality, replay results, or historical truth. R4 — persisting a
//! derivation is an explicit effect distinct from computing it.
//!
//! # The load-bearing triple (three owners, never collapses)
//!
//! The history home's commit point ≠ this home's materialization applied cut
//! ≠ the runtime home's durable checkpoint. A lagging, missing, stale, or
//! corrupt materialization limits only claims about that materialization; it
//! cannot make an accepted event unaccepted.
//!
//! # The protected-index posture (settled; carried VERBATIM)
//!
//! The protected-index family question is settled: no first-party
//! protected-index family ships. Protected search stays bounded
//! decrypt-and-scan or explicitly application-owned indexing that receives no
//! machine authority. The nine if-admitted requirements remain only as a
//! REVERSIBLE STANDING BAR any future first-party family would have to clear
//! before admission — not an admitted capability, and never re-flattened
//! into a permanent ban. Encryption at rest neither creates this family nor
//! hides an undeclared leakage surface.

use crate::bytes::ContentRegionId;
use crate::history::FederationCutEntries;
use crate::identity::{
    AuthorityPosition, ByteIdentity, Commitment, CreationLaw, IdentityClass, IdentityRole,
    Occurrence,
};
use crate::refusal::{FamilyShape, RefusalFamily};
use crate::types::{Completeness, EvidenceRef};

// ---------------------------------------------------------------------------
// Identity roles — ten non-substitutable roles; the two-seat hybrid.
// ---------------------------------------------------------------------------

/// Layout-identity domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayoutDomain;

/// One physical layout's identity — block, tile, and chunk sizes and
/// alignment become part of it where they affect decoding. `Tile` names an
/// actually tiled layout only, never the generic physical noun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayoutId(pub Commitment<LayoutDomain>);

/// The identity role marker for materializations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterializationRole;

/// One materialization — Class D, fresh; SURVIVES rematerialization (the
/// owning contract states which roles survive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterializationId(Occurrence<MaterializationRole>);

impl IdentityRole for MaterializationId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl MaterializationId {
    /// In-crate mint for laws. Test-gated until publication minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<MaterializationRole>) -> Self {
        Self(occurrence)
    }
}

/// One materialization generation — Class C, scoped to its materialization;
/// CHANGES on rematerialization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterializationGeneration(pub AuthorityPosition<MaterializationId>);

/// Row-domain preimage domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowDomainDomain;

/// SEAT 1 of the two-seat hybrid: the semantic, PREIMAGE-DERIVED row-domain
/// identity (preimage: row-identity domain + exact source set and cut +
/// selection-and-order contract) gating every zip, mask, and kernel
/// composition. Occurrence-only fresh identities would make "rebuildable" a
/// lie — a faithful rebuild must be substitutable. When equality cannot be
/// PROVEN from the preimages, composition fails closed as a DISTINCT outcome
/// from a proven mismatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowDomainId(pub Commitment<RowDomainDomain>);

impl IdentityRole for RowDomainId {
    const CLASS: IdentityClass = IdentityClass::SemanticCommitment;
    const CREATION: CreationLaw = CreationLaw::DerivedFromAdmittedPreimage;
}

/// The identity role marker for derived occurrences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DerivedOccurrenceRole;

/// SEAT 2: a fresh identity naming WHICH BUILD — used together with the
/// row-domain seat, neither doing double duty. Equal rows never imply equal
/// occurrence identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OccurrenceId(Occurrence<DerivedOccurrenceRole>);

impl IdentityRole for OccurrenceId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl OccurrenceId {
    /// In-crate mint for laws. Test-gated until build minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<DerivedOccurrenceRole>) -> Self {
        Self(occurrence)
    }
}

/// The byte-role marker for occurrence digests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DerivedOccurrenceByteRole;

/// The occurrence's exact-byte digest — Class B, distinct from the
/// occurrence identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OccurrenceDigest(pub ByteIdentity<DerivedOccurrenceByteRole>);

// ---------------------------------------------------------------------------
// The DataBlock lifecycle and derivation frame.
// ---------------------------------------------------------------------------

/// The PERSISTED/inspect lifecycle state — live phases are consuming
/// builders (each transition consumes or invalidates the former ability to
/// mutate or publish; publication returns a FRESH published value, never an
/// in-place flag flip; failure cannot leave two independently publishable
/// owners). Typestate cannot prove source authority, derivation
/// equivalence, durable namespace publication, capability admission, or
/// independent qualification; persisted, mapped, remote, or decoded state
/// begins as untrusted bytes and re-enters through validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataBlockState {
    /// Filling within bounds — no reader treats it complete.
    Building,
    /// Framing, bounds, offsets, components, relationships pass — derivation
    /// truth not yet implied.
    StructurallyValidated,
    /// Immutable content, exact occurrence identity fixed — publication not
    /// yet implied.
    SealedDerivedOccurrence,
    /// Crossed its publication boundary; bound to one generation and applied
    /// cut.
    PublishedMaterialization,
    /// Superseded — no new consumer under the retired generation.
    RetiredOccurrence,
}

/// The ten derivation primitives.
pub const DERIVATION_PRIMITIVES: [&str; 10] = [
    "identity-domain",
    "row-domain",
    "typed-fact",
    "column",
    "relation",
    "selection",
    "transformation",
    "layout",
    "occurrence",
    "evidence",
];

/// The seven validity conditions one bitmap cannot collapse into one bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidityCondition {
    /// Missing.
    Missing,
    /// Null.
    Null,
    /// Unavailable.
    Unavailable,
    /// Shredded.
    Shredded,
    /// Unauthorized.
    Unauthorized,
    /// Invalid.
    Invalid,
    /// Corrupt.
    Corrupt,
}

// ---------------------------------------------------------------------------
// Source bindings, columns, and the mask.
// ---------------------------------------------------------------------------

/// Source-cut claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceCutClaim;

/// The source-binding carrier — TWO FORMS, and the forms are the law: the
/// mask family's binding-mismatch cause names disagreeing FORMS, while its
/// generation-mismatch cause names two generation-form carriers whose
/// generations differ (the source cut already sits inside the row-domain
/// preimage and cannot residually differ under an equal row domain; the
/// generation can, because rematerialization may preserve materialization
/// identity while changing generation).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceBinding {
    /// Bound to an exact source cut.
    CutForm(EvidenceRef<SourceCutClaim>),
    /// Bound to a materialization generation.
    GenerationForm(MaterializationGeneration),
}

/// Schema/field-identity domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColumnFieldDomain;
/// Value-sort/unit/numeric-profile domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColumnSortDomain;
/// Ordering/reversible-permutation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColumnOrderingDomain;
/// Validity/presence-semantics domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColumnValidityDomain;

/// A typed ordered view over ONE field and ONE bounded row domain. Columns
/// from different row domains cannot be zipped, compared row-wise, selected
/// by one mask, or passed to one kernel merely because their lengths are
/// equal. Physical column order corresponds to semantic row order unless the
/// layout carries an explicit reversible permutation — sorting, dictionary,
/// partition, compression, vectorization, and clustering never silently
/// reorder semantic results.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Column {
    /// The schema and field identity.
    pub field: Commitment<ColumnFieldDomain>,
    /// The value sort, unit, and numeric profile.
    pub sort: Commitment<ColumnSortDomain>,
    /// The row domain — seat 1.
    pub row_domain: RowDomainId,
    /// The logical length.
    pub length: u64,
    /// The source binding.
    pub binding: SourceBinding,
    /// The layout.
    pub layout: LayoutId,
    /// The ordering or reversible-permutation relationship.
    pub ordering: Commitment<ColumnOrderingDomain>,
    /// The validity and presence semantics.
    pub validity: Commitment<ColumnValidityDomain>,
}

/// The qualified mask representations — shape law; membership earns
/// selection through evidence, none by familiarity. Representation is
/// derived physical policy: changing it cannot change membership, logical
/// length, deterministic iteration, source binding, or evidence meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaskRepresentation {
    /// A dense bitset.
    DenseBitset,
    /// Sparse ordered indices.
    SparseIndices,
    /// Bounded runs/ranges.
    Runs,
    /// Small-domain inline words.
    InlineWord,
}

/// Iteration-contract domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IterationContractDomain;
/// Mask completeness/validity posture domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaskPostureDomain;

/// One derived set of rows over one exact row domain — derived selection
/// state, NOT a cursor, checkpoint, capability, publication acknowledgement,
/// durable membership claim, or event authority (that prohibition is
/// type-level and unrepresentable, never a runtime refusal). Unused physical
/// bits cannot select nonexistent rows; iteration never exposes physical
/// padding or stale capacity. Completeness and validity postures COMPOSE and
/// propagate through mask operations but never gate them — a union of a
/// complete and an incomplete mask never carries the stronger posture.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelectionMask {
    /// The row domain — seat 1, gating every composition.
    pub row_domain: RowDomainId,
    /// The build — seat 2.
    pub occurrence: OccurrenceId,
    /// The logical length — bounded complement's bound.
    pub length: u64,
    /// The source binding.
    pub binding: SourceBinding,
    /// The deterministic iteration contract.
    pub iteration: Commitment<IterationContractDomain>,
    /// The representation.
    pub representation: MaskRepresentation,
    /// The completeness/validity posture.
    pub posture: Commitment<MaskPostureDomain>,
}

/// The mask-construction refusal — a closed single-cause enum of nine
/// causes whose CHECK ORDER IS NORMATIVE LAW, not an implementation detail:
/// the checks are dependent, not independent — row-domain equality gates
/// every other question, so no cause after the gate is reportable over a
/// pair whose row domains were not proven equal (a disagreement between
/// uninterpretable operands is not an established fact about a comparable
/// pair; reporting one commits the row-length category collapse in refusal
/// form). The first step whose check establishes its condition selects the
/// value and no later check runs; Rust declaration order is never the rule.
/// Covers only mask-YIELDING operations (nullary constructors, intersection,
/// union, difference, bounded complement, representation conversion, and the
/// untrusted-bytes re-entry path); membership, count, and iteration are
/// queries whose refusals belong to the query surface's own owner.
///
/// Six deliberate absences, each a decision: no derivation-truth judgement;
/// no completeness/validity cause (those compose, never gate); no
/// this-is-not-a-cursor cause (type-level, unrepresentable); no
/// complement-out-of-bounds (one operand carries its own bound — demoting a
/// structural invariant into a runtime check inverts the discipline); no
/// budget/resource cause (the bounds law already keeps those distinct — a
/// third spelling would be that distinction's second home); no completion
/// posture (single-cause bodies report one established cause).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SelectionMaskConstruction {
    /// The preimages are comparable AND THEY DIFFER.
    RowDomainMismatch,
    /// Equality could not be DECIDED from the preimages — fails closed, and
    /// is never reported as a mismatch: equality undetermined and equality
    /// disproved are different established facts, and fail-closed is not
    /// permission to report the stronger one.
    RowDomainEqualityUnproven,
    /// The logical lengths differ. The canonical body may carry both; a row
    /// count is a cardinality disclosure NO releasing boundary emits — nor a
    /// row index, an iteration position, or row-domain preimage material —
    /// absent a contract that permits it.
    LengthMismatch {
        /// The left logical length.
        left: u64,
        /// The right logical length.
        right: u64,
    },
    /// The source-binding carrier FORMS disagree: one cut-form, one
    /// generation-form.
    SourceBindingMismatch,
    /// Both carriers are generation-form and the generations differ.
    GenerationMismatch,
    /// The iteration contracts differ.
    IterationContractMismatch,
    /// A declared qualified representation this build does not carry — a
    /// profile gap; sending a caller to repair data when the defect is a
    /// profile gap reports the wrong fact.
    RepresentationUnsupported,
    /// The representation's structure is invalid (word count vs length,
    /// unordered or duplicated sparse indices, overlapping runs).
    RepresentationInvalid,
    /// Unused physical bits are set — refuses; masking attacker-set bits off
    /// is a silent repair.
    UnusedBitsSet,
}

impl RefusalFamily for SelectionMaskConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "RowDomainMismatch",
        "RowDomainEqualityUnproven",
        "LengthMismatch",
        "SourceBindingMismatch",
        "GenerationMismatch",
        "IterationContractMismatch",
        "RepresentationUnsupported",
        "RepresentationInvalid",
        "UnusedBitsSet",
    ];
}

// ---------------------------------------------------------------------------
// Payload locators and block tables.
// ---------------------------------------------------------------------------

/// A bounded ordinal into the block's extent table — AUTHORED thin (the
/// corpus uses the reference without defining it).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtentEntryRef(pub u32);

/// A bounded ordinal into the block's binding table — AUTHORED thin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindingEntryRef(pub u32);

/// The closed two-form payload locator — never four context-free scalars,
/// never one all-fields record: an offset or length alone locates nothing,
/// and a binding-table locator carries no phantom extent coordinates. A
/// locator NEVER stores the payload's current resolution outcome, and is
/// meaningful only under the block's declared binding — it cannot be
/// transplanted across lineage, generation, row domain, payload profile, or
/// authority scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PayloadLocator {
    /// A slice of one extent.
    ExtentSlice {
        /// The extent-table entry.
        extent: ExtentEntryRef,
        /// The bounded byte offset within it.
        offset: u64,
        /// The encoded length.
        length: u64,
    },
    /// A binding-table entry.
    BindingEntry {
        /// The binding-table entry.
        binding: BindingEntryRef,
    },
}

/// Extent-location-role domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtentLocationDomain;
/// Extent compression/protection-profile domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtentProfileDomain;

/// One extent-table row. The extent identity IS a Tier-1 content region —
/// no parallel notion of region identity exists. Under a tree-digest family
/// a byte range proves itself against the region digest without reading the
/// whole region; where the selected family carries no slice proofs, the
/// capability is realized by bounded chunked re-digest under the same read
/// bounds, or the profile narrows its verified-slice-read claim explicitly —
/// never a silent assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExtentEntry {
    /// The extent's identity — the Tier-1 content region.
    pub extent: ContentRegionId,
    /// The location role.
    pub location: Commitment<ExtentLocationDomain>,
    /// The compression and protection profiles.
    pub profiles: Commitment<ExtentProfileDomain>,
    /// The decoded bounds.
    pub decoded_bounds: u64,
}

/// Binding-table payload-schema domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PayloadSchemaDomain;
/// Key-scope-relationship domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyScopeRelationDomain;
/// Secret-authority-generation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SecretGenerationDomain;
/// Protected-commitment-profile domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommitmentProfileDomain;
/// Access-contract domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccessContractDomain;

/// One binding-table row — the base scan plane contains only public or
/// policy-permitted derived columns and typed opaque payload locators: no
/// raw protected payload bytes and no undeclared protected search token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BindingEntry {
    /// The payload schema and codec.
    pub payload_schema: Commitment<PayloadSchemaDomain>,
    /// The key-scope relationship.
    pub key_scope: Commitment<KeyScopeRelationDomain>,
    /// The secret-authority generation.
    pub generation: Commitment<SecretGenerationDomain>,
    /// The protected-semantic commitment profile.
    pub commitment_profile: Commitment<CommitmentProfileDomain>,
    /// The access contract.
    pub access: Commitment<AccessContractDomain>,
}

// The late-materialization outcome IS the authority home's eight-outcome
// `ProtectedResolution` — referenced under its own name, never redefined and
// never aliased (one meaning, one spelling). A block never stores blank
// bytes, sentinel bytes, a null pointer, or a stale enum as permanent proof
// of one of these outcomes; shredding changes secret authority and current
// resolution without patching immutable event frames or public block bytes;
// and late materialization may decode a bounded chunk but never scans,
// decrypts, maps, or decodes an unbounded population and then claims a small
// returned limit bounded the work.

// ---------------------------------------------------------------------------
// Materialization: applied cuts, generations, the three axes.
// ---------------------------------------------------------------------------

/// The per-materialization source-cut carrier — a role-distinct newtype over
/// the history home's closed store-id-sorted mechanism (one mechanism, never
/// one meaning; the DRY law makes sharing the mechanism the default and the
/// newtype keeps the meanings apart).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterializationSourceCuts(pub FederationCutEntries);

/// Each published materialization's exact applied cut: which authoritative
/// input has been incorporated. NOT the accepted-event visibility cut, an
/// HLC summary, a cursor, a subscription checkpoint, a delivery index, a
/// wall-clock freshness claim, or proof that out-of-scope sources were
/// processed. The load-bearing triple never collapses.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterializationAppliedCut {
    /// The materialization.
    pub materialization: MaterializationId,
    /// Its generation.
    pub generation: MaterializationGeneration,
    /// The incorporated source cuts.
    pub sources: MaterializationSourceCuts,
}

/// Whether ANY qualifying published occurrence exists at all — the distinct
/// unmaterialized absence, never conflated with reachability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterializationPresence {
    /// No qualifying published occurrence exists.
    NotMaterialized,
    /// A qualifying published occurrence exists.
    Materialized,
}

/// Whether a materialized occurrence can currently be reached and opened —
/// NON-PROTECTED access only; protected access resolves through the
/// authority home's resolution enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterializationAvailability {
    /// Reachable and openable.
    Available,
    /// Not currently reachable.
    Unavailable,
}

/// How much of the declared domain is covered — the non-erasable domain
/// parameter rides the root completeness shape; never a bare "complete" that
/// could masquerade as source closure or a verification denominator.
/// Staleness is a FOURTH, separate axis (evidence): a materialization can be
/// materialized, available, AND stale at once — staleness never occupies a
/// presence or availability variant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MaterializationCoverage<D> {
    /// The coverage over its non-erasable domain.
    pub coverage: Completeness<D>,
}

// ---------------------------------------------------------------------------
// Physical plans and kernels.
// ---------------------------------------------------------------------------

/// Plan-template member domain markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanStaticFactsDomain;
/// Plan-binding admission-evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanAdmissionClaim;
/// Plan-binding cut claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanCutClaim;
/// Plan-binding capability-posture domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanPostureDomain;

/// The reusable qualified realization key — binds every correctness-bearing
/// STATIC mechanism fact with exact-equality comparison (a field may be
/// absent only with a recorded proof it cannot influence the plan); caches
/// across cuts. Neither key can capture a dynamic fact as static. A
/// deliberately cut-bound specialization is a NAMED ephemeral bound
/// specialization, never a cached template. A plan-cache miss causes
/// replanning or reference fallback, never semantic failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanTemplate {
    /// The complete static mechanism-fact key.
    pub static_facts: Commitment<PlanStaticFactsDomain>,
}

/// One admitted use — exact cuts, generations, behavior-affecting
/// capability/policy posture, admission evidence: per-invocation authority
/// that NEVER contaminates the reusable template key, validated per
/// execution; a stale generation or wider authority posture can never reuse
/// a binding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanBinding {
    /// The exact source cuts.
    pub cuts: EvidenceRef<PlanCutClaim>,
    /// The behavior-affecting capability and policy posture.
    pub posture: Commitment<PlanPostureDomain>,
    /// The admission evidence.
    pub admission: EvidenceRef<PlanAdmissionClaim>,
}

/// The nine things a physical plan cannot do.
pub const PLAN_CANNOT: [&str; 9] = [
    "reinterpret-schema-meaning",
    "change-source-cuts",
    "introduce-an-undeclared-protected-index",
    "widen-information-release",
    "alter-numeric-or-k3-behavior",
    "remove-required-completeness-or-evidence",
    "reset-semantic-work-or-deadlines",
    "make-an-unavailable-mechanism-semantically-mandatory",
    "become-the-only-route-capable-of-explaining-the-result",
];

/// The optimized-kernel admission gate — wall-clock speed alone is
/// insufficient, no kernel is its own only oracle, and the safe scalar
/// kernel is the reference behavior.
pub const KERNEL_ADMISSION_GATE: [&str; 11] = [
    "scalar-and-independent-model-equivalence",
    "exact-binding-of-schema-source-cut-layout-generation-row-domain",
    "deterministic-order-and-mask-parity",
    "numeric-k3-completeness-availability-proof-parity",
    "bounded-decode-allocation-execution-diagnostics",
    "corruption-and-stale-input-refusal",
    "safe-portable-fallback",
    "representative-and-adverse-workload-value",
    "target-profile-qualification",
    "no-hidden-capability-or-ambient-io",
    "full-evidence-denominator-honesty",
];

/// The data-execution portable semantic-work categories (one SHAPE of the
/// physical-observation family, not a universal contract).
pub const DATA_SEMANTIC_WORK: [&str; 10] = [
    "source-rows-considered",
    "values-validated",
    "predicate-operations",
    "groups-matches-joins-traversal-steps",
    "rows-selected",
    "rows-or-fields-semantically-materialized",
    "logical-comparisons-and-numeric-operations",
    "bounded-derivation-steps",
    "bytes-inspected-under-the-work-profile",
    "result-explanation-evidence-items-produced",
];

/// The data-execution mechanism diagnostics — kept DISTINCT: diagnostics
/// never change canonical results, become semantic-work units, prove
/// durability, advance an applied cut, authorize a protected read, or
/// create a support claim; elapsed time alone proves no bound.
pub const DATA_MECHANISM_DIAGNOSTICS: [&str; 12] = [
    "kernel-and-layout-selected",
    "blocks-and-compressed-members-read",
    "physical-bytes",
    "payload-chunks-touched",
    "mask-representation-and-conversion",
    "copies-and-allocations",
    "mapping-and-page-fault-behavior",
    "cache-and-branch-observations",
    "vector-and-instruction-observations",
    "contention",
    "wall-and-cpu-time",
    "host-counters",
];

/// The fifteen refusal classes derived-data operations must distinguish —
/// unsupported, corrupt, stale, unauthorized, shredded, unavailable,
/// incomplete, and exhausted remain different outcomes, and budget
/// exhaustion stays distinct from physical resource exhaustion.
pub const DERIVED_REFUSAL_CLASSES: [&str; 15] = [
    "unsupported-profile-or-version",
    "malformed-descriptor",
    "wrong-coordinate",
    "invalid-ordering-or-permutation",
    "out-of-range-overlapping-overflowing-offset",
    "invalid-validity-dictionary-mask-auxiliary-structure",
    "compression-or-expansion-violation",
    "missing-or-corrupt-extent",
    "invalid-payload-binding",
    "stale-protected-index",
    "incomplete-or-partial-publication",
    "budget-exhaustion",
    "physical-resource-exhaustion",
    "qualification-or-support-unavailable",
    "contextual-admission-refusal",
];

/// The nine-item reversible standing bar any future first-party
/// protected-index family must clear BEFORE admission — a standing bar, not
/// an admitted capability, and never re-flattened into a permanent ban.
pub const PROTECTED_INDEX_STANDING_BAR: [&str; 9] = [
    "outside-the-public-base-plane",
    "explicitly-selected-never-ambient",
    "declares-its-complete-leakage",
    "protected-or-pseudonymous-never-anonymous",
    "binds-one-keyscope-generation-cut-derivation-profile-occurrence",
    "fails-closed-after-generation-change-invalidates-on-shred",
    "verifies-proposed-matches-against-authoritative-meaning",
    "derived-rebuildable-retireable-independently-qualified",
    "exposes-its-use-in-explanation-evidence-configuration-support",
];
