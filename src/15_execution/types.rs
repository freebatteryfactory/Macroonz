//! Execution Form, the authored operator register, lowering and agreement,
//! the recursion witness, the effect batch, and the kernel contracts.
//!
//! # The agreement rule
//!
//! Every executable image contains or immutably binds BOTH forms. Before
//! execution an independent route validates and normalizes the Semantic Form,
//! independently lowers it into baseline Execution Form, compares against the
//! bound form under the selected comparison contract, and refuses the image
//! on disagreement. DISAGREEMENT IS NEVER REPAIRED BY PREFERRING THE
//! PRODUCER — and only the independent re-lowering comparison seam may mint
//! an agreement-checked image. Both routes are safe Rust; parser speed or
//! memory pressure authorizes no `unsafe`, C, or assembly.
//!
//! # The operator-set law
//!
//! The operator set is derived from admitted behavior, never an inherited
//! universal-operation table. No operator exists that only a private compiler
//! pass can emit or read, and adding, removing, or changing one changes the
//! Execution-Form version. Owner-derivedness is a property of the SET,
//! checked at Execution-Form version admission — not at construction.
//!
//! # The bounded lane
//!
//! Structured recursion operators are the only traversal: a fold's work bound
//! is computable by construction (structure size × per-node bound — the arena
//! knows the size), an unfold's bound is the fuel handed to it, and arbitrary
//! pointer-chasing loops do not exist — the undecidable bound-analysis case
//! is UNREPRESENTABLE, not checked-for.

use crate::bounds::DimensionId;
use crate::identity::{
    AuthorityPosition, Commitment, CreationLaw, IdentityClass, IdentityRole, Occurrence,
};
use crate::refusal::{CompletionPosture, FamilyShape, RefusalFamily};
use crate::semantic::BoundDimensionRow;
use crate::types::{Bounded, ConstLimit, EvidenceRef, Limit, NonEmptyBounded};

// ---------------------------------------------------------------------------
// The authored operator register (v1) and Execution-Form identity.
// ---------------------------------------------------------------------------

/// The v1 operator register — AUTHORED from the corpus's complete
/// named-operation harvest (the corpus fixes what every operator states and
/// how the set versions, and never enumerates the set). Grouped: the four
/// boundary forms; the two structured-recursion operators; fourteen iteration
/// operations; the three settled traversal spellings; two query operations
/// (`truncate` is the renamed truncation — iteration already owns lowercase
/// `take`, and one spelling never carries two meanings); positioning;
/// decision; the five derived-data operations (derivation pure, the four
/// publications effect nodes, never collapsed); and the six owner-specific
/// publication boundaries. The `group` row is bound by the grouping ruling
/// (one typed key tuple; each item in exactly one group; multi-membership is
/// `relation_expansion`, never a hidden mode).
pub const OPERATOR_REGISTER: [&str; 38] = [
    "ask",
    "do",
    "request",
    "pend",
    "fold",
    "unfold",
    "map",
    "filter",
    "reduce",
    "find",
    "first",
    "single",
    "any",
    "all",
    "take",
    "take_until",
    "join",
    "scan",
    "group",
    "relation_expansion",
    "seek",
    "children",
    "descendants",
    "truncate",
    "page",
    "resolve",
    "decide",
    "derive_datablock",
    "persist_datablock",
    "replace_datablock",
    "advertise_datablock",
    "publish_datablock",
    "event_publication",
    "effect_batch_admission",
    "artifact_publication",
    "result_publication",
    "checkpoint_advancement",
    "materialization_publication",
];

/// The identity role marker for Execution-Form families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionFormFamilyRole;

/// One Execution-Form family — Class D, fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionFormFamilyId(Occurrence<ExecutionFormFamilyRole>);

impl IdentityRole for ExecutionFormFamilyId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl ExecutionFormFamilyId {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<ExecutionFormFamilyRole>) -> Self {
        Self(occurrence)
    }
}

/// One Execution-Form version — Class C, scoped to its family. Adding,
/// removing, or changing an operator advances this version; no version is
/// bare, and numeric comparison across families is undefined.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExecutionFormVersion(pub AuthorityPosition<ExecutionFormFamilyId>);

// ---------------------------------------------------------------------------
// The per-operator declaration.
// ---------------------------------------------------------------------------

/// The five declarable algebraic laws — an operator declares only the laws it
/// ACTUALLY satisfies; a rewrite may rely on nothing else, and every declared
/// law owes an independently qualified witness before an optimizer relies on
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlgebraicLaw {
    /// Associativity.
    Associativity,
    /// Commutativity.
    Commutativity,
    /// Monotonicity.
    Monotonicity,
    /// Idempotence.
    Idempotence,
    /// Distributivity.
    Distributivity,
}

/// Compile-time bound for declared algebraic laws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlgebraicLawLimit;
impl Limit for AlgebraicLawLimit {}
impl ConstLimit for AlgebraicLawLimit {
    const MAX: usize = 5;
}

/// Domain markers for the operator declaration's committed facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperatorDomain;
/// Operand/result sort domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SortDomain;
/// Value/control dependency domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DependencyDomain;
/// Region/recursion relationship domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionRecursionDomain;
/// Effect/suspension posture domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectSuspensionDomain;
/// Origin-edge domain marker (the edge back to Semantic Form).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OriginEdgeDomain;

/// Limit family for an operator's work-charge rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkChargeLimit;
impl Limit for WorkChargeLimit {}

/// One operator's declaration — the seven facts every operator states.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OperatorDeclaration {
    /// The operator's identity.
    pub operator: Commitment<OperatorDomain>,
    /// Its operand sorts.
    pub operand_sorts: Commitment<SortDomain>,
    /// Its result sorts.
    pub result_sorts: Commitment<SortDomain>,
    /// Its value and control dependencies.
    pub dependencies: Commitment<DependencyDomain>,
    /// Its region and recursion relationship.
    pub region_recursion: Commitment<RegionRecursionDomain>,
    /// Its effect and suspension posture.
    pub effect_suspension: Commitment<EffectSuspensionDomain>,
    /// Its portable work charge — rows of the shared bound-dimension
    /// register; no operator-private bound language exists.
    pub work_charge: Bounded<BoundDimensionRow, WorkChargeLimit>,
    /// The algebraic laws it actually satisfies.
    pub laws: Bounded<AlgebraicLaw, AlgebraicLawLimit>,
    /// Its origin edge back to Semantic Form.
    pub origin: Commitment<OriginEdgeDomain>,
}

// ---------------------------------------------------------------------------
// The Execution Form phase root and its construction family.
// ---------------------------------------------------------------------------

/// The domain marker for Execution Form content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionFormDomain;

/// The typed portable structure the executor runs — one language-neutral
/// operator algebra whose unit is the OPERATOR, not the node. It is not
/// Source Form, Semantic Form, Rust MIR, LLVM IR, WebAssembly, machine code,
/// an opaque object graph, or a Physical Plan. A phase root, private behind
/// its checked constructor with a declared canonical order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionForm {
    content: Commitment<ExecutionFormDomain>,
}

impl ExecutionForm {
    /// In-crate mint for laws. Test-gated until the checked constructor
    /// exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(content: Commitment<ExecutionFormDomain>) -> Self {
        Self { content }
    }

    /// The content's commitment.
    #[must_use]
    pub fn content(&self) -> &Commitment<ExecutionFormDomain> {
        &self.content
    }
}

/// The Execution Form construction issues — causes 1–11 read the per-operator
/// declaration roster as defects: what an operator fails to STATE, never what
/// it fails to AGREE WITH. `SortMismatch` names disagreement between adjacent
/// operators INSIDE one form — never disagreement with the bound Semantic
/// Form. `RecursionRelationshipInvalid` names the relationship the operator
/// STATES; recursion-WITNESS validation is a separate admission stage.
/// Cause 12 refuses both collapse modes: fusing the four boundary forms into
/// a generic effect node, or fusing pure result construction with external
/// publication. Cause 13 refuses a suspension lowered into anything but the
/// explicit typed one-shot continuation record. Causes 14–15 are this form's
/// OWN spellings of the two laws both forms carry — same cause vocabulary,
/// distinct semantic ownership, distinct repair direction. Every payload is
/// the canonical-order position, nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionFormConstructionIssue {
    /// An unknown operator.
    UnknownOperator {
        /// Position under the canonical order.
        position: u64,
    },
    /// An operator version mismatch.
    OperatorVersionMismatch {
        /// Position under the canonical order.
        position: u64,
    },
    /// An operand or result sort is missing.
    OperandOrResultSortMissing {
        /// Position under the canonical order.
        position: u64,
    },
    /// Adjacent operators inside this form disagree on sorts.
    SortMismatch {
        /// Position under the canonical order.
        position: u64,
    },
    /// A value or control dependency is invalid.
    ValueOrControlDependencyInvalid {
        /// Position under the canonical order.
        position: u64,
    },
    /// The region relationship is invalid.
    RegionRelationshipInvalid {
        /// Position under the canonical order.
        position: u64,
    },
    /// The stated recursion relationship is invalid.
    RecursionRelationshipInvalid {
        /// Position under the canonical order.
        position: u64,
    },
    /// The effect or suspension posture is unstated.
    EffectOrSuspensionPostureMissing {
        /// Position under the canonical order.
        position: u64,
    },
    /// The work charge is missing.
    WorkChargeMissing {
        /// Position under the canonical order.
        position: u64,
    },
    /// The algebraic-law declaration is missing.
    AlgebraicLawDeclarationMissing {
        /// Position under the canonical order.
        position: u64,
    },
    /// The origin edge is missing.
    OriginEdgeMissing {
        /// Position under the canonical order.
        position: u64,
    },
    /// A lowering collapsed an effect boundary.
    CollapsedEffectBoundaryOperation {
        /// Position under the canonical order.
        position: u64,
    },
    /// A suspension retained a host continuation.
    RetainedHostContinuation {
        /// Position under the canonical order.
        position: u64,
    },
    /// The form is not in its canonical order.
    NonCanonicalOrder {
        /// Position under the canonical order.
        position: u64,
    },
    /// An operator only a hidden producer can emit or read.
    HiddenProducerOnlyOperator {
        /// Position under the canonical order.
        position: u64,
    },
}

/// Limit family for Execution Form issues — a declared finite bound,
/// evidence-selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionFormIssueLimit;
impl Limit for ExecutionFormIssueLimit {}

/// Execution Form construction. Posture addition over the Semantic Form
/// family: the independent reference lowerer posts `EarlyStopped` at its
/// FIRST established issue, because the agreement comparison it feeds
/// consumes a verdict rather than a diagnosis. Five non-claims: no agreement
/// with the bound form; no satisfaction of the declared algebraic laws (that
/// is qualification); no recursion-witness validity; no claim the operator
/// set is owner-derived (version admission's); no assumed exhaustiveness.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExecutionFormConstruction {
    /// The established issues.
    pub issues: NonEmptyBounded<ExecutionFormConstructionIssue, ExecutionFormIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for ExecutionFormConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// What the independence seam MAY share between the production and reference
/// routes.
pub const INDEPENDENCE_MAY_SHARE: [&str; 6] = [
    "bounded-arenas",
    "deterministic-symbol-handling",
    "diagnostic-and-origin-map-machinery",
    "graph-utilities",
    "public-semantic-values",
    "published-schemas-that-do-not-encode-the-verdict",
];

/// What the independence seam MAY NOT share — sharing any of these would fuse
/// the very seam the independent route must challenge.
pub const INDEPENDENCE_MAY_NOT_SHARE: [&str; 11] = [
    "challenged-lexer",
    "parser-tables",
    "compiler-ast",
    "name-resolution-and-collision-helpers",
    "normalization-implementation",
    "identity-preimage-builder",
    "lowering-visitors",
    "meta-evaluator",
    "canonical-byte-reader",
    "golden-output-generator",
    "verdict-helper",
];

// ---------------------------------------------------------------------------
// Well-founded recursion: the measure algebra and the witness.
// ---------------------------------------------------------------------------

/// Limit family for lexicographic measure tuples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LexicographicLimit;
impl Limit for LexicographicLimit {}

/// The closed, independently executable measure algebra: bounded naturals and
/// lexicographic tuples of them, under an admitted well-founded order — never
/// an arbitrary callback the producer grades itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Measure {
    /// A bounded natural.
    BoundedNatural(u64),
    /// A lexicographic tuple of bounded naturals.
    Lexicographic(Bounded<u64, LexicographicLimit>),
}

/// Claim markers for the witness's evidence members.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrictDecreaseClaim;
/// Witness-origin claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WitnessOriginClaim;

/// Domain markers for the witness's committed facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CallGraphDomain;
/// Edge-set domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeSetDomain;
/// Measure-trace domain marker (the measure before and after each edge).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeasureTraceDomain;

/// The independently checkable recursion witness, one per recursive strongly
/// connected component — mutual recursion admits only under ONE SCC-wide
/// decreasing construction. Refused: unbounded recursion, arbitrary backward
/// jumps, unproved cycles, measure smuggling through an opaque kernel,
/// universal static unrolling. Two locks: runtime metering remains active
/// after static admission, and tail position is never itself a termination
/// proof.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecursionWitness {
    /// The call graph.
    pub call_graph: Commitment<CallGraphDomain>,
    /// The base and recursive edges.
    pub edges: Commitment<EdgeSetDomain>,
    /// The measure before and after each edge.
    pub measures: Commitment<MeasureTraceDomain>,
    /// Strict decrease on every cycle — the owed evidence.
    pub strict_decrease: EvidenceRef<StrictDecreaseClaim>,
    /// The input bounds.
    pub input_bounds: u64,
    /// The finite depth.
    pub depth: u64,
    /// The total portable work.
    pub total_work: u64,
    /// The memory and frame bounds.
    pub memory_and_frames: u64,
    /// The output bounds.
    pub output_bounds: u64,
    /// The effect and suspension bounds.
    pub effect_and_suspension_bounds: u64,
    /// The origins.
    pub origins: EvidenceRef<WitnessOriginClaim>,
}

/// The two admitted effectful-recursion lanes. Invariant: an effect admitted
/// before a later recursive refusal STAYS ADMITTED AND RECEIPTED — recursion
/// grants no rollback, erases no receipt, converts no uncertainty into retry
/// permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectfulRecursionLane {
    /// Build one bounded typed effect batch as pure data; cross no physical
    /// boundary until evaluation completes and admission succeeds — a refusal
    /// publishes nothing.
    AtomicPlanning,
    /// Cross request/pend while recursion is active — only when the witness
    /// additionally closes the aggregate totals.
    Interleaved,
}

/// The aggregate totals the interleaved lane's witness must additionally
/// close.
pub const INTERLEAVED_CLOSURE_TOTALS: [&str; 8] = [
    "effect-count-and-order",
    "capabilities",
    "recursion-and-continuation-depth",
    "captured-bytes",
    "suspensions-and-responses",
    "work-memory-output-artifacts",
    "deadline",
    "recovery-posture",
];

// ---------------------------------------------------------------------------
// The effect batch — intent as pure data.
// ---------------------------------------------------------------------------

/// The six command kinds. Distinct kinds share a batch ONLY when one selected
/// local authority profile proves their exact common boundary — a semantic
/// path crossing several authority regions acquires no atomicity from the
/// path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandKind {
    /// An event append.
    EventAppend,
    /// An effect-intent admission.
    EffectIntentAdmission,
    /// A checkpoint advance.
    CheckpointAdvance,
    /// An artifact publication.
    ArtifactPublication,
    /// A protected-payload publication.
    ProtectedPayloadPublication,
    /// A secret-authority mutation.
    SecretAuthorityMutation,
}

/// A bounded position in the AUTHORED command order — never a host or wire
/// identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandOrdinal(pub u32);

/// The command-contract domain marker (expected result / receipt /
/// reconciliation contracts).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandContractDomain;

/// One command of an effect batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectCommand {
    /// Its position in the authored order.
    pub ordinal: CommandOrdinal,
    /// Its kind.
    pub kind: CommandKind,
    /// Its bound contracts.
    pub contracts: Commitment<CommandContractDomain>,
}

/// Limit family for a batch's commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatchCommandLimit;
impl Limit for BatchCommandLimit {}

/// Limit family for a batch's declared bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatchBoundLimit;
impl Limit for BatchBoundLimit {}

/// Boundary-requirement domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundaryRequirementDomain;
/// Group/fence declaration domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GroupFenceDomain;
/// Idempotency-binding domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdempotencyBindingDomain;

/// A bounded ordered LOCAL publication intent — not a distributed
/// transaction, not a bag that acquires atomicity by co-location. INTENT
/// ONLY: no result or receipt field is representable inside the unsubmitted
/// intent, and none may be added later — a shape law with no refusal variant.
/// Per admitted atomic group: failure before the boundary publishes NONE;
/// success accounts for EVERY command; a partial durable subset is forbidden.
/// Dropping an unsubmitted batch publishes nothing — only explicit admitted
/// submission crosses a durable boundary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectBatch {
    /// The ordered commands.
    pub commands: Bounded<EffectCommand, BatchCommandLimit>,
    /// The declared boundary requirement (composition proves NO boundary —
    /// the authority/backend contract owns that proof).
    pub boundary: Commitment<BoundaryRequirementDomain>,
    /// The declared groups and fences.
    pub groups_and_fences: Commitment<GroupFenceDomain>,
    /// The bound idempotency relationships.
    pub idempotency: Commitment<IdempotencyBindingDomain>,
    /// The batch's own declared bounds.
    pub bounds: Bounded<BoundDimensionRow, BatchBoundLimit>,
}

/// The six forbidden idempotency-identity derivation sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForbiddenIdentitySource {
    /// A timestamp bucket.
    TimestampBucket,
    /// An Attempt identity.
    AttemptIdentity,
    /// A worker.
    Worker,
    /// A route.
    Route,
    /// A session.
    Session,
    /// A host.
    Host,
}

/// The three group/fence structural defects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroupFenceDefect {
    /// Overlapping groups.
    OverlappingGroups,
    /// A fence interior to an admitted group.
    FenceInteriorToAdmittedGroup,
    /// An empty group.
    EmptyGroup,
}

/// The three required per-command contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequiredContractKind {
    /// An expected-result contract.
    ExpectedResult,
    /// A receipt contract.
    Receipt,
    /// A reconciliation requirement.
    ReconciliationRequirement,
}

/// The effect-batch composition issues — composition consults ONLY the
/// batch's own authored declared data (no live grant, no authority profile,
/// no backend, no host fact), so every applicable check is decidable in one
/// bounded evaluation and several issues may hold at once. One instance per
/// established subject: three commands missing three contracts are three
/// issues, never one token. Payloads are typed classifications and bounded
/// ordinals into the authored order only — never raw command payloads,
/// attacker-controlled text, and never a constraint-source pair (that belongs
/// to the authority meet; composition has no meet).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectBatchCompositionIssue {
    /// A declared command kind outside the batch's DECLARED boundary
    /// requirement.
    BoundaryMismatch {
        /// The command.
        command: CommandOrdinal,
        /// Its kind.
        kind: CommandKind,
    },
    /// The authored composition exceeds the batch's OWN declared bounds —
    /// declared over an unsubmitted intent; charges and reserves nothing.
    DeclaredBoundsExceeded {
        /// The exceeded dimension.
        dimension: DimensionId,
    },
    /// A bound idempotency relationship declares a forbidden derivation
    /// source (where no source is declared, the check is admission's).
    IdempotencyIdentitySourceForbidden {
        /// The command.
        command: CommandOrdinal,
        /// The forbidden source.
        source: ForbiddenIdentitySource,
    },
    /// Declared groups or fences are not structurally well-formed over the
    /// command order.
    GroupOrFenceIllFormed {
        /// The site.
        at: CommandOrdinal,
        /// The defect.
        defect: GroupFenceDefect,
    },
    /// A command binds no expected-result, receipt, or reconciliation
    /// contract.
    RequiredContractUnbound {
        /// The command.
        command: CommandOrdinal,
        /// The unbound contract kind.
        contract: RequiredContractKind,
    },
}

/// Limit family for composition issues — a declared finite bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectBatchIssueLimit;
impl Limit for EffectBatchIssueLimit {}

/// Effect-batch composition. Posture: `Complete` when every applicable check
/// ran; `EarlyStopped` only when (a) an established issue makes remaining
/// checks undecidable over the authored value, or (b) the declared bounds are
/// already exceeded — enumerating per-command issues over an over-bound
/// composition is exactly the unbounded verification buffer the
/// retained-diagnostic law forbids. A composition refusal publishes, charges,
/// and reserves nothing; names no capability or grant verdict; asserts no
/// outcome; proves no boundary; creates no identity/existence/capability/
/// secret/freshness/workload oracle.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectBatchComposition {
    /// The established issues.
    pub issues: NonEmptyBounded<EffectBatchCompositionIssue, EffectBatchIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for EffectBatchComposition {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

// ---------------------------------------------------------------------------
// Kernels — five role-distinct types, three families.
// ---------------------------------------------------------------------------

/// The identity role marker for semantic-kernel families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticKernelFamilyRole;

/// One semantic-kernel family — Class D, fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticKernelFamilyId(Occurrence<SemanticKernelFamilyRole>);

impl IdentityRole for SemanticKernelFamilyId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl SemanticKernelFamilyId {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<SemanticKernelFamilyRole>) -> Self {
        Self(occurrence)
    }
}

/// One semantic-kernel version — Class C, ordered ONLY within its family; no
/// version is bare.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemanticKernelVersion(pub AuthorityPosition<SemanticKernelFamilyId>);

/// Kernel semantic-contract domain marker (the MEANING half).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelSemanticDomain;
/// Kernel interface-contract domain marker (the BOUNDARY half).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelInterfaceDomain;
/// Kernel-qualification claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelQualificationClaim;
/// Substitution-scope domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubstitutionScopeDomain;
/// Fallback-policy domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FallbackDomain;

/// The kernel's semantic contract — the MEANING half: operation identity and
/// version, operand and result sorts, laws, work formula, purity/effect
/// posture. The inventory PARTITIONS across the contract trio; it never
/// duplicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelSemanticContract(pub Commitment<KernelSemanticDomain>);

/// The kernel's interface contract — the BOUNDARY half: operand/result
/// carriage, capability and source requirements, bounds, and the refusal /
/// evidence / inspection routes at the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelInterfaceContract(pub Commitment<KernelInterfaceDomain>);

/// The identity role marker for kernel realizations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelRealizationRole;

/// One kernel realization — a runtime role, not a contract; Class D, fresh.
/// No display name, Rust path, function pointer, source-file digest, or
/// ambient registry resolves it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelRealizationId(Occurrence<KernelRealizationRole>);

impl IdentityRole for KernelRealizationId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl KernelRealizationId {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<KernelRealizationRole>) -> Self {
        Self(occurrence)
    }
}

/// Kernel qualification evidence — an evidence role, not a contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelQualificationEvidence(pub EvidenceRef<KernelQualificationClaim>);

/// A reference to a kernel semantic contract — exact identity plus version,
/// never a display name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelSemanticContractRef {
    /// The exact contract identity.
    pub contract: Commitment<KernelSemanticDomain>,
    /// The version.
    pub version: u64,
}

/// A reference to a kernel interface contract — exact identity plus version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelInterfaceContractRef {
    /// The exact contract identity.
    pub contract: Commitment<KernelInterfaceDomain>,
    /// The version.
    pub version: u64,
}

/// The admissible qualified-substitution scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelSubstitutionScope(pub Commitment<SubstitutionScopeDomain>);

/// The declared fallback-or-refusal behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelFallbackPolicy(pub Commitment<FallbackDomain>);

/// The neutral view of a binding policy's selected arm — reveals without
/// granting literal construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KernelBindingPosture {
    /// Exact-realization pinning.
    ExactRealization,
    /// Qualified substitution under a declared scope.
    QualifiedSubstitution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum KernelBindingArm {
    ExactRealization { realization: KernelRealizationId },
    QualifiedSubstitution { scope: KernelSubstitutionScope },
}

/// The kernel binding policy — an OPAQUE struct over the private closed pair,
/// because a public enum's variants are public construction. Payload-bearing:
/// every policy-specific value present, never nullable companion fields. The
/// authored-route constructors discharge the payload law BY SHAPE, so no
/// cause of the decode family is reachable on the authored route.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelBindingPolicy {
    arm: KernelBindingArm,
}

impl KernelBindingPolicy {
    /// Exact-realization pinning, for reproducibility.
    #[must_use]
    pub const fn exact_realization(realization: KernelRealizationId) -> Self {
        Self {
            arm: KernelBindingArm::ExactRealization { realization },
        }
    }

    /// Qualified substitution under a declared scope — explicit and recorded
    /// in evidence.
    #[must_use]
    pub const fn qualified_substitution(scope: KernelSubstitutionScope) -> Self {
        Self {
            arm: KernelBindingArm::QualifiedSubstitution { scope },
        }
    }

    /// The neutral posture view.
    #[must_use]
    pub const fn posture(&self) -> KernelBindingPosture {
        match &self.arm {
            KernelBindingArm::ExactRealization { .. } => KernelBindingPosture::ExactRealization,
            KernelBindingArm::QualifiedSubstitution { .. } => {
                KernelBindingPosture::QualifiedSubstitution
            }
        }
    }
}

/// One kernel requirement: both contract references, the binding policy, the
/// qualification evidence, and the fallback policy. It never copies contract
/// contents, resolves through a display name, or claims which qualified
/// realization ran before admission selects one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelRequirement {
    /// The semantic-contract reference.
    pub semantic: KernelSemanticContractRef,
    /// The interface-contract reference.
    pub interface: KernelInterfaceContractRef,
    /// The binding policy.
    pub binding: KernelBindingPolicy,
    /// The qualification evidence.
    pub qualification: EvidenceRef<KernelQualificationClaim>,
    /// The fallback policy.
    pub fallback: KernelFallbackPolicy,
}

/// Limit family for kernel requirement sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelSetLimit;
impl Limit for KernelSetLimit {}

/// The bounded canonical kernel-requirement set — duplicates and
/// contradictions refuse.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelRequirementSet {
    /// The requirements.
    pub requirements: Bounded<KernelRequirement, KernelSetLimit>,
}

/// The kernel semantic-contract construction issues (the MEANING half) —
/// eight, each naming the inventory fact by that fact's identity and nothing
/// else. `OperandOrResultSortUnclosed` refuses a sort admitting an inhabitant
/// the closed value algebra excludes — the same prohibition read at the
/// boundary is the INTERFACE contract's, a distinct defect with a distinct
/// repair. `LawDeclarationMissing` is a DECLARATION defect: the contract is
/// never asked to prove the laws it names, only to name them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KernelSemanticContractConstructionIssue {
    /// The operation owner, identity, or version is missing — owner is a
    /// stated fact BESIDE identity and version, not a synonym for either.
    OperationOwnerIdentityOrVersionMissing,
    /// An operand or result sort is missing.
    OperandOrResultSortMissing,
    /// An operand or result sort is unclosed.
    OperandOrResultSortUnclosed,
    /// The law declaration is missing.
    LawDeclarationMissing,
    /// The work formula is missing.
    WorkFormulaMissing,
    /// The work formula is in a kernel-private bound language — the shared
    /// bound-dimension register is the only lawful work vocabulary, and this
    /// contract is where that prohibition binds because the work formula is a
    /// meaning fact.
    WorkFormulaInKernelPrivateBoundLanguage,
    /// The purity or effect posture is missing.
    PurityOrEffectPostureMissing,
    /// The contract leaves an operation two of — or none of — primitive
    /// operator, definition over smaller operators, qualified opaque kernel.
    DefinitionBoundaryUnfixedOrAmbiguous,
}

/// Limit family for semantic-contract issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelSemanticIssueLimit;
impl Limit for KernelSemanticIssueLimit {}

/// Kernel semantic-contract construction. `Complete` is the paved default
/// here — an authored first-party declaration whose issues are independent
/// facts one author can violate at once — never required; `EarlyStopped`
/// states its reason. Non-claims: no boundary facts; no satisfaction of the
/// declared laws; no agreement with the interface contract; no assumed
/// exhaustiveness.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelSemanticContractConstruction {
    /// The established issues.
    pub issues: NonEmptyBounded<KernelSemanticContractConstructionIssue, KernelSemanticIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for KernelSemanticContractConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// The kernel interface-contract construction issues (the BOUNDARY half) —
/// thirteen. Causes 9–13 are the five smuggling prohibitions read as
/// construction defects: a declared surface admitting an ambient callback,
/// live authority, a host object, unmetered work, or an undeclared effect is
/// refused at construction — BEFORE any realization is qualified. One
/// declaration can admit several at once, which is why this family's
/// collection is its strongest case. An evidence route that reuses the
/// producer's verdict logic is a MISSING independent route, not a present
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KernelInterfaceContractConstructionIssue {
    /// The bound-operation reference is missing or unresolvable — refused
    /// before any boundary fact is read.
    BoundOperationReferenceMissing,
    /// The operand or result carriage is missing — carriage is the
    /// boundary's own fact, never a second spelling of the sorts.
    OperandOrResultCarriageMissing,
    /// A capability, source, or authority requirement is missing.
    CapabilitySourceOrAuthorityRequirementMissing,
    /// The bounds are missing.
    BoundsMissing,
    /// The refusal route is missing.
    RefusalRouteMissing,
    /// The independent evidence route is missing.
    IndependentEvidenceRouteMissing,
    /// The public construction route is missing.
    PublicConstructionRouteMissing,
    /// The inspection route is missing.
    InspectionRouteMissing,
    /// An ambient callback in the interface.
    AmbientCallbackInInterface,
    /// An authority-bearing interface surface.
    AuthorityBearingInterfaceSurface,
    /// A host object in the interface.
    HostObjectInInterface,
    /// An unmetered work surface.
    UnmeteredWorkSurface,
    /// An undeclared effect surface.
    UndeclaredEffectSurface,
}

/// Limit family for interface-contract issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelInterfaceIssueLimit;
impl Limit for KernelInterfaceIssueLimit {}

/// Kernel interface-contract construction. Non-claims: no meaning facts; no
/// agreement with the semantic contract; no freedom from smuggling IN THE
/// REALIZATION (that is qualification, with the physical membrane supplying
/// enforcement facts); no resolution of which realization serves; no assumed
/// exhaustiveness.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelInterfaceContractConstruction {
    /// The established issues.
    pub issues:
        NonEmptyBounded<KernelInterfaceContractConstructionIssue, KernelInterfaceIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for KernelInterfaceContractConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// Kernel binding-policy construction — the machine's first DECODE-ROUTE-ONLY
/// single-cause family: NO cause is reachable on the authored route, because
/// the typed constructors discharge the payload law by shape (a missing
/// payload, a mismatched payload, a both-arms-or-neither policy, and a
/// display-name realization are unrepresentable rather than diagnosed). The
/// causes exist for the decode-reconstruction route, on the ladder each rung
/// of which presupposes the one below: the selected arm → that arm's required
/// policy-specific value → the absence of any companion field → the form of
/// the realization identity. At most one cause is establishable at a time;
/// the resolution cause names the resolution form attempted, never the
/// decoded name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KernelBindingPolicyConstruction {
    /// The policy arm is missing or ambiguous.
    PolicyArmMissingOrAmbiguous,
    /// The exact-realization identity is missing.
    ExactRealizationIdentityMissing,
    /// The substitution scope is missing.
    SubstitutionScopeMissing,
    /// A nullable companion field is present.
    NullableCompanionFieldPresent,
    /// The realization was resolved by display name or path.
    RealizationResolvedByDisplayNameOrPath,
}

impl RefusalFamily for KernelBindingPolicyConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "PolicyArmMissingOrAmbiguous",
        "ExactRealizationIdentityMissing",
        "SubstitutionScopeMissing",
        "NullableCompanionFieldPresent",
        "RealizationResolvedByDisplayNameOrPath",
    ];
}

/// The nine consumed work dimensions static validation computes or checks
/// maxima over. Division of labor: static validation computes maxima; the
/// physical membrane reserves an admissible envelope; the executor charges
/// actual work under the selected versioned contract.
pub const WORK_DIMENSIONS: [&str; 9] = [
    "semantic-operations-and-recursive-edges",
    "decoded-bytes-and-validated-values",
    "rows-groups-matches-joins-traversal-steps",
    "definition-and-kernel-calls",
    "memory-and-active-frames",
    "result-and-artifact-bytes",
    "effects-and-publication-intents",
    "suspended-frames-and-responses",
    "explanation-and-evidence-construction",
];
