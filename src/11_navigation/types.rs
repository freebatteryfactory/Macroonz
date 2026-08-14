//! The semantic address space and navigation: reference frames, axes,
//! addresses, journal views, frame transformations, routes, `Fix<T>`, the
//! positioning refusal, bounded traversal, typed paths, paging, cursors, and
//! logical time-travel inspection.
//!
//! # The admission-vs-state law
//!
//! A fact's admission `Address` answers WHERE THAT FACT ENTERED the address
//! space; a state coordinate or `Fix<T>` answers WHERE ACCEPTED EVIDENCE
//! PLACES THE APPLICATION NOW. One never substitutes for the other.
//!
//! # Anti-laundering
//!
//! The machine exposes no generic position, path, progress-set, or cursor type
//! that launders the navigation roles into one another. Navigation consumes
//! exact cuts from history; it never mints order.

use crate::bounds::SemanticWork;
use crate::history::{CommitPoint, FederationCutVector, HistoryCut, SourceClosure, StoreLineageId};
use crate::identity::{Commitment, CreationLaw, IdentityClass, IdentityRole, Occurrence};
use crate::refusal::{FamilyShape, RefusalFamily};
use crate::types::{Bounded, ConstLimit, EvidenceRef, Freshness, Limit};
use crate::value::BoundedText;
use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// Reference frames and axes.
// ---------------------------------------------------------------------------

/// The identity role marker for reference frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReferenceFrameRole;

/// One named semantic coordinate system — Class D, fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReferenceFrameId(Occurrence<ReferenceFrameRole>);

impl IdentityRole for ReferenceFrameId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl ReferenceFrameId {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<ReferenceFrameRole>) -> Self {
        Self(occurrence)
    }
}

crate::scope_guard_version! {
    /// One version of a reference frame — Class C, scoped to its frame. The
    /// frame is a VALUE inside the position rather than a type parameter, so two
    /// frames' versions are ONE type and the compiler never tells them apart:
    /// what it refuses is the ambient comparison, which leaves
    /// `try_cmp_same_scope` as the only road to an ordering and makes the
    /// cross-frame answer that road's typed refusal. A version under another
    /// scope ROLE — a schema's, a profile's — is a different type outright, and
    /// that is the incomparability the stamp carries in the types.
    pub struct FrameVersion over ReferenceFrameId, seated in mod frame_version;
}

/// Compile-time bound for an axis's declared capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AxisCapabilityLimit;
impl Limit for AxisCapabilityLimit {}
impl ConstLimit for AxisCapabilityLimit {
    const MAX: usize = 9;
}

/// The nine declarable axis capabilities — closed. A compiler or runtime
/// cannot invent distance, averaging, nearest-neighbor meaning, or total order
/// for an axis that did not declare it: an undeclared capability is
/// unrepresentable, not runtime-refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisCapability {
    /// Equality.
    Equality,
    /// Total order.
    TotalOrder,
    /// Partial order.
    PartialOrder,
    /// Hierarchy.
    Hierarchy,
    /// Intervals.
    Intervals,
    /// Sets.
    Sets,
    /// Typed relationships.
    TypedRelationships,
    /// A metric under a named profile.
    MetricUnderNamedProfile,
    /// Admitted approximation.
    AdmittedApproximation,
}

/// One typed dimension with declared values and lawful operations. Only what
/// it declares exists for it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Axis {
    /// The declared capabilities.
    pub capabilities: Bounded<AxisCapability, AxisCapabilityLimit>,
}

// ---------------------------------------------------------------------------
// Addresses and the journal view.
// ---------------------------------------------------------------------------

mod sealed {
    /// The seal: another address role is admitted only when a real operation
    /// has distinct ownership, laws, and evidence for it — by decision, in this
    /// crate, never by downstream impl.
    #[expect(
        unnameable_types,
        reason = "the sealed-trait pattern makes the supertrait deliberately unnameable so downstream crates cannot implement the role"
    )]
    pub trait Sealed {}
}

/// The closed address-role contract (sealed — see the module's admission
/// rule).
pub trait AddressRole: sealed::Sealed {}

/// One complete coordinate under a frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PointRole;
impl sealed::Sealed for PointRole {}
impl AddressRole for PointRole {}

/// One typed bounded relationship among admitted endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelationRole;
impl sealed::Sealed for RelationRole {}
impl AddressRole for RelationRole {}

/// One bounded set or predicate over coordinates — the domain `SourceClosure`
/// closes over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionRole;
impl sealed::Sealed for RegionRole {}
impl AddressRole for RegionRole {}

/// The domain marker for address coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AddressCoordinateDomain;

/// One typed address under a frame — role-parameterized, never a universal
/// untyped enum. Canonical coordinate bytes are the schema register's address
/// row; the coordinate rides as its commitment here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Address<Role: AddressRole> {
    /// The frame that gives the coordinate meaning.
    pub frame: FrameVersion,
    /// The coordinate's commitment.
    pub coordinate: Commitment<AddressCoordinateDomain>,
    _role: PhantomData<Role>,
}

impl<Role: AddressRole> Address<Role> {
    /// Checked composition from a frame and coordinate commitment.
    #[must_use]
    pub const fn at(frame: FrameVersion, coordinate: Commitment<AddressCoordinateDomain>) -> Self {
        Self {
            frame,
            coordinate,
            _role: PhantomData,
        }
    }
}

/// The governed terrain of one reference frame. Its constructor is total (one
/// typed member, presence enforced by the type), so no construction family is
/// owed — the same argument that keeps `DurationLimit`'s typed route
/// family-free.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddressSpace {
    /// The governing frame version.
    pub frame: FrameVersion,
}

/// The accepted history relevant to one typed address — a logical lens, not a
/// database, file, writer, actor, or socket per coordinate. One accepted
/// event may participate in several views without duplication,
/// re-identification, or re-admission; every view resolves to the accepted
/// authority and exact cut supporting its claim. (`Send` posture is
/// deliberately left to the default; flagged, not decided.)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JournalView<'a, Role: AddressRole> {
    /// The address this view serves.
    pub address: &'a Address<Role>,
}

// ---------------------------------------------------------------------------
// Frame transformations.
// ---------------------------------------------------------------------------

/// Limit family for navigation text members.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NavigationTextLimit;
impl Limit for NavigationTextLimit {}

/// Whether a transformation covers its whole source domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DomainPosture {
    /// Total over the source domain.
    Total,
    /// Partial — undefined coordinates refuse.
    Partial,
}

/// The transformation's multiplicity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MultiplicityPosture {
    /// One-to-one.
    OneToOne,
    /// One coordinate may map to several.
    Multiplicity,
}

/// Whether the transformation is exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExactnessPosture {
    /// Exact.
    Exact,
    /// Approximate — its loss is declared.
    Approximate,
}

/// Whether the transformation can be inverted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReversibilityPosture {
    /// Invertible.
    Invertible,
    /// Not invertible.
    NotInvertible,
}

/// One declared frame transformation — AUTHORED name over the nine declared
/// facets. The immutability law: historic facts
/// remain bound to the frame under which they were admitted — a newer frame
/// may derive or explicitly migrate an interpretation but cannot silently
/// rewrite the old address.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameTransformation {
    /// The version transformed from.
    pub source: FrameVersion,
    /// The version transformed to.
    pub target: FrameVersion,
    /// Total or partial domain.
    pub domain: DomainPosture,
    /// One-to-one or multiplicity.
    pub multiplicity: MultiplicityPosture,
    /// What is lost, declared.
    pub loss: BoundedText<NavigationTextLimit>,
    /// Exact or approximate.
    pub exactness: ExactnessPosture,
    /// Invertible or not.
    pub reversibility: ReversibilityPosture,
    /// Who may perform it, declared.
    pub authority: BoundedText<NavigationTextLimit>,
    /// The bounded cost.
    pub work_bound: u64,
    /// The bounded output growth.
    pub expansion_bound: u64,
}

// ---------------------------------------------------------------------------
// Navigation roles: destination, path program, routes.
// ---------------------------------------------------------------------------

/// The six destination kinds — closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DestinationKind {
    /// An exact address.
    ExactAddress,
    /// A bounded region.
    BoundedRegion,
    /// A typed state predicate.
    StatePredicate,
    /// A decision condition.
    DecisionCondition,
    /// A fixed-point or closure condition.
    FixedPointCondition,
    /// A set of admissible terminal states.
    AdmissibleTerminalSet,
}

/// The domain marker for destination conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DestinationDomain;

/// The condition sought — AUTHORED name for ladder role 1.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NavigationRequest {
    /// The destination kind.
    pub kind: DestinationKind,
    /// The condition's commitment.
    pub condition: Commitment<DestinationDomain>,
}

/// The domain marker for path programs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathProgramDomain;

/// The bounded computation describing lawful navigation — AUTHORED name for
/// ladder role 2. The computation itself is program-plane; navigation carries
/// its commitment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticPathProgram(pub Commitment<PathProgramDomain>);

/// The domain marker for relation sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelationSetDomain;

/// One strategy under a named frame, relation set, source set, and exact
/// cuts — AUTHORED name for ladder role 3.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedRoute {
    /// The frame the route resolves under.
    pub frame: FrameVersion,
    /// The relation set's commitment.
    pub relation_set: Commitment<RelationSetDomain>,
    /// One exact cut per participating authority.
    pub source_cuts: FederationCutVector,
}

/// Claim markers for the admitted route's four evidence members.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteCapabilityClaim;
/// Bounds-evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteBoundsClaim;
/// Generation-evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteGenerationClaim;
/// Deadline-evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteDeadlineClaim;

/// Route plus capabilities, bounds, evidence, generations, deadlines —
/// AUTHORED name for ladder role 4. Ladder roles 5 and 6 (Execution Form,
/// Physical Plan) are owned by the execution and derived homes and only
/// referenced by navigation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdmittedRoute {
    /// The resolved route admitted.
    pub route: ResolvedRoute,
    /// The capability evidence.
    pub capabilities: EvidenceRef<RouteCapabilityClaim>,
    /// The bounds evidence.
    pub bounds: EvidenceRef<RouteBoundsClaim>,
    /// The generation evidence.
    pub generations: EvidenceRef<RouteGenerationClaim>,
    /// The deadline evidence.
    pub deadlines: EvidenceRef<RouteDeadlineClaim>,
}

// ---------------------------------------------------------------------------
// Fix<T> — derived positional evidence.
// ---------------------------------------------------------------------------

/// The domain marker for state predicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StatePredicateDomain;

/// What kind of position was derived. `Region` and `State` payloads are
/// AUTHORED here rather than left empty: a region-shaped fix
/// returns the region address it derived; a state-shaped fix returns the
/// predicate it established. `Ambiguous` carries no payload — the ambiguity's
/// account rides the fix's explanation evidence.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FixShape<T> {
    /// An exact result.
    Exact(T),
    /// A derived region.
    Region(Address<RegionRole>),
    /// An established state predicate.
    State(Commitment<StatePredicateDomain>),
    /// An admitted approximation.
    Approximate(T),
    /// An ambiguous destination.
    Ambiguous,
}

/// The three multi-authority relationship postures — closed, with an AUTHORED
/// carrier. A cut vector alone proves none of these and
/// invents no distributed snapshot or transaction; under the coordination
/// posture, the named profile is what proves more, never the vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MultiAuthorityRelationship {
    /// Each authority's cut frozen on its own; no cross-authority claim.
    IndependentlyFrozen,
    /// The admitted causation relation bounds the combination.
    CausationConstrained,
    /// A selected stronger coordination profile proves more.
    CoordinationProfile(Commitment<CoordinationProfileDomain>),
}

/// The domain marker for coordination profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CoordinationProfileDomain;

/// Limit family for lawful-alternative sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlternativeLimit;
impl Limit for AlternativeLimit {}

/// Claim markers for the fix's evidence members.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixProvenanceClaim;
/// Causal-dependency claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixCausationClaim;
/// Access-posture claim marker (availability and authorization).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixAccessClaim;
/// Explanation claim marker (the derivation's own account).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixExplanationClaim;
/// Bounds claim marker (the bounds the derivation ran under).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixBoundsClaim;

/// Derived positional evidence — SUCCESS postures only, never a refusal,
/// never authority. A STRUCT binding orthogonal axes, not an enum that
/// flattens them: a fix can be Approximate AND Incomplete AND Stale at once.
/// The nine navigation postures resolve structurally: one admitted route IS
/// the success channel; alternatives are the bounded member; incomplete
/// search is the closure axis; ambiguity and approximation are shapes;
/// staleness is the freshness axis; the remaining four are
/// [`PositioningRefusal`]'s causes — every posture lands on exactly one
/// owner, and no posture enum exists.
#[must_use = "a fix is the position that positioning derived, with every axis it resolved"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fix<T> {
    /// What kind of position was derived.
    pub shape: FixShape<T>,
    /// The frame and version that give it meaning.
    pub frame: FrameVersion,
    /// One exact cut per participating authority.
    pub source_cuts: FederationCutVector,
    /// The declared relationship among the authorities.
    pub relationship: MultiAuthorityRelationship,
    /// The completeness axis — never erased by shape.
    pub closure: SourceClosure,
    /// The freshness axis — derived-and-stale is representable.
    pub freshness: Freshness<T, HistoryCut>,
    /// Several lawful alternatives, where they exist.
    pub alternatives: Bounded<T, AlternativeLimit>,
    /// The availability/authorization posture evidence.
    pub access: EvidenceRef<FixAccessClaim>,
    /// The provenance evidence.
    pub provenance: EvidenceRef<FixProvenanceClaim>,
    /// The causal-dependency evidence.
    pub causation: EvidenceRef<FixCausationClaim>,
    /// The semantic work performed.
    pub work: SemanticWork,
    /// The bounds the derivation ran under.
    pub bounds: EvidenceRef<FixBoundsClaim>,
    /// The derivation's own account.
    pub explanation: EvidenceRef<FixExplanationClaim>,
}

// ---------------------------------------------------------------------------
// The positioning refusal.
// ---------------------------------------------------------------------------

/// The claim marker for route-closure witnesses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteClosureClaim;

/// The final no-route claim's witness: the exact searched region and the
/// cut-closure evidence that excludes every lawful route. Claim-specific,
/// never a universal evidence record.
#[must_use = "a witness is the proof the searched region excludes every lawful route"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RouteClosureEvidence {
    /// The exact searched region.
    pub region: Address<RegionRole>,
    /// The cut-closure witness.
    pub witness: EvidenceRef<RouteClosureClaim>,
}

/// The positioning refusal — four causes, exactly the refusal half of the
/// nine navigation postures. The order is semantic precedence among conditions
/// actually established, not preference and not an execution schedule: a
/// boundary runs its checks in whatever order its threat profile requires,
/// and whether an unauthorized caller learns the operation exists at all is
/// the releasing boundary's information-release contract to decide, never
/// this rule's. The three unit causes are unit deliberately: which bound was
/// exceeded, which region was unauthorized, and which operation was
/// unsupported are facts already in the caller's hand, and a refusal that
/// copied them would store one fact in two homes. Its `Unauthorized` is
/// source-region authorization only — protected-payload resolution answers a
/// different question and the two never merge.
#[must_use = "a positioning refusal carries the established cause the route was not taken"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PositioningRefusal {
    /// The route does not offer this operation — an operation that does not
    /// exist has no authorization question to answer.
    Unsupported,
    /// The source region is unauthorized — an established authorization
    /// condition outranks a bound condition; an exhausted budget never masks
    /// a missing authorization.
    Unauthorized,
    /// A declared bound was exceeded — a truncated search refuses here,
    /// never `NoRoute`.
    OverBudget,
    /// No route exists under proven closure — ranks last because it alone
    /// owes closure evidence, which a search cut short by a bound cannot
    /// produce.
    NoRoute {
        /// The owed closure witness.
        evidence: RouteClosureEvidence,
    },
}

impl RefusalFamily for PositioningRefusal {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] =
        &["Unsupported", "Unauthorized", "OverBudget", "NoRoute"];
}

// ---------------------------------------------------------------------------
// Bounded traversal, paths, paging.
// ---------------------------------------------------------------------------

/// The four lawful traversal forms — closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraversalForm {
    /// Fold a finite journal or resolved path.
    FoldFiniteJournal,
    /// Unfold a finite next-expansion set under a decreasing measure or an
    /// explicit traversal bound.
    UnfoldBoundedExpansion,
    /// Combine both without retaining an unbounded intermediate graph.
    CombinedFoldUnfold,
    /// Compute a monotone fixed point over a declared finite region.
    MonotoneFixedPoint,
}

/// The five claims that cannot be made without [`SourceClosure`] evidence
/// over the relevant regions and cuts.
pub const CLOSURE_REQUIRED_CLAIMS: [&str; 5] = [
    "absence",
    "negation",
    "exhaustive-search",
    "final-order",
    "top-k-finality",
];

/// The eight fusible fold outputs — independent component folds remain the
/// reference meaning; fusion is an optimization, never a new meaning.
pub const FUSIBLE_FOLD_OUTPUTS: [&str; 8] = [
    "value",
    "cuts",
    "provenance",
    "completeness",
    "explanation",
    "dependencies",
    "work",
    "materialization-inputs",
];

/// The five incomparable route dimensions no universal scalar score may
/// collapse into one number.
pub const INCOMPARABLE_ROUTE_DIMENSIONS: [&str; 5] =
    ["latency", "disclosure", "risk", "evidence", "resources"];

/// The eight facets a typed path defines as its own declared facts.
pub const PATH_CONTRACT_FACETS: [&str; 8] = [
    "normalization",
    "segment-identity",
    "canonical-collation",
    "wildcards-and-selectors",
    "depth",
    "length",
    "comparison",
    "compatibility",
];

/// The five prohibited silent mergers: path identity comparison is
/// locale-independent and non-coercing — none of these may silently merge
/// distinct admitted segments.
pub const PROHIBITED_SILENT_MERGERS: [&str; 5] = [
    "host-locale",
    "filesystem-case-behavior",
    "unicode-display-equivalence",
    "numeric-string-conversion",
    "adapter-normalization",
];

/// The registered path selectors — a closed typed set, never a table of
/// spellings: how a frontend or host writes a selector is that surface's own
/// vocabulary. Extension is by registration under this owner, never by parser
/// fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathSelector {
    /// Matches exactly one segment at this position.
    SegmentWildcard,
    /// Matches zero or more segments, descending.
    RecursiveDescent,
}

/// The three explicit downgrade triggers of paged-evidence composition. Pages
/// taken under one frozen cut through an unbroken cursor chain establish, in
/// union, exactly the claim one full bounded walk establishes — closure
/// included when the chain completes the region. These downgrade the
/// composite claim EXPLICITLY, never silently; pagination can neither
/// strengthen nor quietly weaken what the walk proves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageDowngradeTrigger {
    /// The cursor chain broke.
    BrokenChain,
    /// The chain crossed a cut.
    CrossedCut,
    /// The chain mixed generations.
    MixedGeneration,
}

// ---------------------------------------------------------------------------
// Cursors and the continuation split.
// ---------------------------------------------------------------------------

/// The cursor's traversal direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorDirection {
    /// Forward.
    Forward,
    /// Backward.
    Backward,
}

/// Domain markers for the cursor's committed facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CursorFamilyDomain;
/// Operation-identity domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperationIdentityDomain;
/// Source-set domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CursorSourceDomain;
/// Selector-set domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SelectorSetDomain;
/// Canonical-order domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanonicalOrderDomain;

/// Generation-evidence claim marker for cursors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CursorGenerationClaim;
/// Integrity/confidentiality-profile claim marker for cursors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CursorProfileClaim;

/// Immutable continuation evidence for one bounded operation — not a mutable
/// offset and not durable progress authority. Changing any identity-bearing
/// fact creates an incompatible continuation. Internals may be opaque to
/// clients while still producing structured refusals and neutral inspection;
/// opacity is not permission for an unversioned or forgeable opaque token.
/// The two-worlds law: lineage identity is duplicated by any physical copy
/// and answers only WHICH LINEAGE, never WHETHER DIVERGENCE OCCURRED — the
/// commitment at the cut is what tells two worlds apart.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    /// The cursor family.
    pub family: Commitment<CursorFamilyDomain>,
    /// The family version.
    pub version: u32,
    /// The operation identity.
    pub operation: Commitment<OperationIdentityDomain>,
    /// The source set.
    pub source: Commitment<CursorSourceDomain>,
    /// The store lineage.
    pub lineage: StoreLineageId,
    /// The source/authority/materialization generation evidence.
    pub generations: EvidenceRef<CursorGenerationClaim>,
    /// The exact historical cut.
    pub cut: CommitPoint,
    /// The selectors and filters.
    pub selectors: Commitment<SelectorSetDomain>,
    /// The canonical order and tie-breaks.
    pub ordering: Commitment<CanonicalOrderDomain>,
    /// The direction.
    pub direction: CursorDirection,
    /// The partition or lane scope, where applicable.
    pub scope: Option<BoundedText<NavigationTextLimit>>,
    /// The page bound.
    pub page_bound: u64,
    /// The work bound.
    pub work_bound: u64,
    /// The compatibility posture, declared.
    pub compatibility: BoundedText<NavigationTextLimit>,
    /// The integrity or confidentiality profile.
    pub profile: EvidenceRef<CursorProfileClaim>,
}

/// Cursor transplantation refusal — eight causes. The selection order is
/// AUTHORED, because a single-cause family owes a declared order: a
/// wrong-family cursor cannot even be decoded, so
/// no other question exists; source precedes generation because generations
/// are scoped to their source; the query precedes its own refinements
/// (filter, order, direction); the cut ranks last because comparing cuts
/// means anything only once everything above matches.
#[must_use = "a transplantation refusal carries why the cursor was not honored"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorTransplantation {
    /// Wrong cursor family.
    WrongFamily,
    /// Different source.
    CrossSource,
    /// Different generation.
    CrossGeneration,
    /// Different query.
    CrossQuery,
    /// Different filters.
    CrossFilter,
    /// Different canonical order.
    CrossOrder,
    /// Different direction.
    CrossDirection,
    /// Different cut.
    CrossCut,
}

impl RefusalFamily for CursorTransplantation {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "WrongFamily",
        "CrossSource",
        "CrossGeneration",
        "CrossQuery",
        "CrossFilter",
        "CrossOrder",
        "CrossDirection",
        "CrossCut",
    ];
}

/// The ten things that cannot advance a durable checkpoint. The
/// checkpoint itself is runtime-owned and referenced, never redefined, here;
/// `DeliveryIndex` is the application home's and is not `AuthoritySequence`,
/// `CommitPoint`, HLC, cursor, checkpoint, Turn, Attempt, or federation cut.
/// A checkpoint survives reconnection through a new compatible session, and
/// session identity alone can neither validate nor invalidate it.
pub const CHECKPOINT_NON_ADVANCERS: [&str; 10] = [
    "computed-but-unpublished-progress",
    "cursor",
    "push-notification",
    "hlc",
    "delivery-index",
    "route",
    "connection-close",
    "page-count",
    "observed-wall-time",
    "derived-fast-start-state",
];

// ---------------------------------------------------------------------------
// Logical time-travel inspection.
// ---------------------------------------------------------------------------

/// The six reconstructable facets of logical time-travel inspection.
pub const RECONSTRUCTABLE_FACETS: [&str; 6] = [
    "knew",
    "decided",
    "intended",
    "attempted",
    "observed",
    "later-reconciled",
];

/// The current-versus-historical support distinction — a past view is never
/// mistaken for a present guarantee.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportDistinction {
    /// Historical view only.
    Historical,
    /// Currently supported.
    Current,
}

/// Claim markers for reconstruction evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReconstructionSourceClaim;
/// Reconstruction-evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReconstructionClaim;

/// One logical time-travel reconstruction — AUTHORED name for the read-only
/// navigation capability. NOT state rollback and not reversal of external
/// reality: it holds references, re-executes no effect, and mutates no
/// accepted history. Material that was authorizedly removed, shredded, or is
/// unauthorized stays unavailable under the removal and resolution rules — a
/// historical view never resurrects it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoricalReconstruction {
    /// The exact cut reconstructed at.
    pub cut: CommitPoint,
    /// The source and owner evidence.
    pub source: EvidenceRef<ReconstructionSourceClaim>,
    /// The explanation and evidence.
    pub evidence: EvidenceRef<ReconstructionClaim>,
    /// The completeness posture.
    pub completeness: SourceClosure,
    /// The protected-data posture, declared.
    pub protected_posture: BoundedText<NavigationTextLimit>,
    /// Whether this is a past view or a present guarantee.
    pub support: SupportDistinction,
}
