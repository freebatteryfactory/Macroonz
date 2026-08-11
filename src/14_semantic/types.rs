//! The judgment and Semantic Form: the phase root, its fifteen-cause
//! construction family, the nine-axis complete expression judgment, and the
//! kernel-inventory contract frame.
//!
//! # Semantic identity
//!
//! Equivalent admitted source expressions lower to equivalent Semantic Form;
//! parser spelling, comments, source layout, Rust enum order, serialized
//! offsets, and optimization choices do not define semantic identity.
//! Canonical order is the checked constructor's declared deterministic
//! order — never Rust enum order, hash order, or serialized offsets.
//!
//! # No secret second language
//!
//! Every admitted operation has at least one lawful public construction route
//! and one neutral inspection route. Any producer — a macro expansion, a Rust
//! API, an SDK, a generated builder, a future language frontend — reaches the
//! same Semantic Form through the same normalization, independent
//! re-lowering, validation, and admission. A node only a hidden compiler or
//! ambient registry can create is refused, and origin (human / generated /
//! searched / repaired / transferred) never changes the contract.
//!
//! # The composition law (owned here; source spelling is the declaration
//! home's)
//!
//! Sequential composition preserves typed refusal union PLUS first-observable
//! order, ordered effect regions, capability-REQUIREMENT union (never grant
//! union), combined source/cut posture, owner-derived symbolic bounds, and
//! structural explanation/evidence. Choice retains the scrutinee judgment,
//! guarded branch effects and refusals, the common normal-result type, and
//! the MAXIMUM lawful branch bound. Higher-order composition folds the
//! callback's complete judgment into the caller and cannot erase an effect,
//! refusal, source requirement, capability, bound, or explanation obligation
//! because the callback's normal return type is compatible. Refusal is not an
//! effect: a refusing expression's normal-value type is the root's
//! uninhabited `Never`, and the refusal channel carries the inhabited family
//! body.

use crate::bounds::{BoundClass, DimensionId};
use crate::declaration::Stage;
use crate::identity::{Commitment, CreationLaw, IdentityClass, IdentityRole};
use crate::refusal::{CompletionPosture, FamilyShape, RefusalFamily};
use crate::types::{Bounded, Limit, NonEmptyBounded};

// ---------------------------------------------------------------------------
// The phase root and its identity.
// ---------------------------------------------------------------------------

/// The domain marker for Semantic Form content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticFormDomain;

/// The canonical typed meaning of a program — a phase root, private behind
/// its checked constructor with a declared canonical order. Never one
/// "program IR" with Execution Form: two role-distinct types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticForm {
    content: Commitment<SemanticFormDomain>,
}

impl SemanticForm {
    /// In-crate mint for laws. Test-gated until the checked constructor
    /// exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(content: Commitment<SemanticFormDomain>) -> Self {
        Self { content }
    }

    /// The normalized content's commitment.
    #[must_use]
    pub fn content(&self) -> &Commitment<SemanticFormDomain> {
        &self.content
    }
}

/// The twelve stated content facts of one Semantic Form, in the order the
/// corpus lists them. Within one form, every declaration, definition, and
/// node identity is unique under the form's canonical order.
pub const SEMANTIC_FORM_CONTENT: [&str; 12] = [
    "resolved-contracts-and-definitions",
    "value-record-variant-unit-numeric-meaning",
    "source-and-historical-cut-requirements",
    "query-traversal-grouping-matching-decision-semantics",
    "truth-uncertainty-completeness-freshness-proof-posture",
    "event-and-effect-intent",
    "capabilities",
    "bounds-and-portable-work-formulas",
    "failure-and-refusal-behavior",
    "explanation-structure",
    "completed-judgments",
    "imports-and-qualified-kernel-interfaces",
];

/// The domain marker for semantic-graph digests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticGraphDomain;

/// Normalized executable MEANING identity — never substitutable for an exact
/// serialized-bytes identity (the image home's), and a content digest proves
/// only the exact byte role named by its own preimage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticGraphDigest(pub Commitment<SemanticGraphDomain>);

impl IdentityRole for SemanticGraphDigest {
    const CLASS: IdentityClass = IdentityClass::SemanticCommitment;
    const CREATION: CreationLaw = CreationLaw::DomainTaggedDigestOfMeaning;
}

// ---------------------------------------------------------------------------
// SemanticFormConstruction — the fifteen causes.
// ---------------------------------------------------------------------------

/// The Semantic Form construction issues — the content roster read as
/// defects, not a general graph-validation vocabulary. Every issue carries
/// ONE payload: the position of the offending declaration, definition, or
/// node under the form's own canonical order — no source text, decoded
/// bytes, host content, or rejected value. Cause 8 is a DECLARATION defect
/// only (satisfaction against the admitted grant is admission's); cause 10
/// pairs refusal behavior with explanation structure — never with evidence,
/// which enters only through the completed judgment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticFormConstructionIssue {
    /// An unresolved reference.
    UnresolvedReference {
        /// Position under the canonical order.
        position: u64,
    },
    /// An invalid type or value shape.
    InvalidTypeOrValueShape {
        /// Position under the canonical order.
        position: u64,
    },
    /// A value outside the closed algebra.
    ValueOutsideClosedAlgebra {
        /// Position under the canonical order.
        position: u64,
    },
    /// An authority-bearing capture.
    AuthorityBearingCapture {
        /// Position under the canonical order.
        position: u64,
    },
    /// A source or historical-cut requirement is incomplete.
    SourceOrHistoricalCutRequirementIncomplete {
        /// Position under the canonical order.
        position: u64,
    },
    /// Query or decision semantics are incomplete.
    QueryOrDecisionSemanticsIncomplete {
        /// Position under the canonical order.
        position: u64,
    },
    /// The truth or proof posture is incomplete.
    TruthOrProofPostureIncomplete {
        /// Position under the canonical order.
        position: u64,
    },
    /// An effect or capability DECLARATION is incomplete.
    EffectOrCapabilityDeclarationIncomplete {
        /// Position under the canonical order.
        position: u64,
    },
    /// A bound or work formula is missing.
    BoundOrWorkFormulaMissing {
        /// Position under the canonical order.
        position: u64,
    },
    /// Refusal behavior or explanation structure is incomplete.
    RefusalOrExplanationStructureIncomplete {
        /// Position under the canonical order.
        position: u64,
    },
    /// A judgment is incomplete.
    IncompleteJudgment {
        /// Position under the canonical order.
        position: u64,
    },
    /// The import or kernel-interface closure is incomplete.
    ImportOrKernelInterfaceClosureIncomplete {
        /// Position under the canonical order.
        position: u64,
    },
    /// The form is not in its canonical order.
    NonCanonicalOrder {
        /// Position under the canonical order.
        position: u64,
    },
    /// A node only a hidden producer can create — the no-secret-second-
    /// language law's teeth.
    HiddenProducerOnlyNode {
        /// Position under the canonical order.
        position: u64,
    },
    /// A duplicate or colliding identity.
    DuplicateOrCollidingIdentity {
        /// Position under the canonical order.
        position: u64,
    },
}

/// Limit family for Semantic Form issues — a declared finite bound,
/// evidence-selected (several issues of one kind are lawful at once).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticFormIssueLimit;
impl Limit for SemanticFormIssueLimit {}

/// Semantic Form construction. Completion posture rule: complete diagnosis
/// is PERMITTED, NEVER REQUIRED, AND NEVER ASSUMED — a first-party producer
/// building a form it authored may run every applicable check and post
/// `Complete`; a form reconstructed from image bytes is diagnosed under the
/// admission rule that cheap safe refusals may precede expensive proof work.
/// An issue whose proof work the constructor lawfully did not perform is not
/// an issue it established. Five explicit non-claims: no agreement with the
/// bound Execution Form (ONLY the independent re-lowering comparison seam
/// may mint an agreement-checked image); no capability satisfaction; no
/// effect admission; no recursion-witness validity; no exhaustiveness unless
/// `Complete`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemanticFormConstruction {
    /// The established issues.
    pub issues: NonEmptyBounded<SemanticFormConstructionIssue, SemanticFormIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for SemanticFormConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

// ---------------------------------------------------------------------------
// The nine-axis judgment — carriers AUTHORED thin, thickened by their owners.
// ---------------------------------------------------------------------------

/// The domain marker for semantic types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticTypeDomain;

/// The normal value type's reference — AUTHORED thin carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SemanticTypeRef(pub Commitment<SemanticTypeDomain>);

/// The domain marker for refusal-family references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalFamilyRefDomain;

/// Limit family for a judgment's refusal set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalSetLimit;
impl Limit for RefusalSetLimit {}

/// The typed refusal families a judgment declares — AUTHORED thin carrier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefusalSet {
    /// The declared families.
    pub families: Bounded<Commitment<RefusalFamilyRefDomain>, RefusalSetLimit>,
}

/// The domain marker for effect regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectRegionDomain;

/// Limit family for a judgment's effect regions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectRegionLimit;
impl Limit for EffectRegionLimit {}

/// The declared effects PLUS their first-observable ordering — the order is
/// the collection's own order, carried by construction. AUTHORED thin
/// carrier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderedEffectRegions {
    /// The regions, in first-observable order.
    pub regions: Bounded<Commitment<EffectRegionDomain>, EffectRegionLimit>,
}

/// The domain marker for capability requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityRequirementDomain;

/// Limit family for a judgment's capability requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityRequirementLimit;
impl Limit for CapabilityRequirementLimit {}

/// Capability REQUIREMENTS — never grants; composition unions requirements
/// and can never union grants. AUTHORED thin carrier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityRequirements {
    /// The required capabilities.
    pub requirements: Bounded<Commitment<CapabilityRequirementDomain>, CapabilityRequirementLimit>,
}

/// The domain marker for source/cut postures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceCutPostureDomain;

/// The frames / journal-view / cut / freshness / completeness posture of what
/// the expression reads — AUTHORED thin carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceCutPosture(pub Commitment<SourceCutPostureDomain>);

/// One applicable bound-dimension row: dimension, class, and maximum. Class
/// is a stated fact per row, never an inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoundDimensionRow {
    /// The registered dimension.
    pub dimension: DimensionId,
    /// The dimension's stated class.
    pub class: BoundClass,
    /// The declared maximum.
    pub maximum: u64,
}

/// Limit family for a judgment's symbolic bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolicBoundLimit;
impl Limit for SymbolicBoundLimit {}

/// The bounded canonical collection of the dimensions that ACTUALLY APPLY to
/// one judgment — never a padded universal struct.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymbolicBounds {
    /// The applicable rows.
    pub dimensions: Bounded<BoundDimensionRow, SymbolicBoundLimit>,
}

/// The domain marker for explanation obligations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExplanationDomain;

/// The structural explanation obligation — AUTHORED thin carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExplanationObligation(pub Commitment<ExplanationDomain>);

/// The domain marker for evidence obligations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvidenceObligationDomain;

/// The claim-specific evidence obligation — AUTHORED thin carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvidenceObligation(pub Commitment<EvidenceObligationDomain>);

/// The complete expression judgment — nine axes, nine typed members:
/// `Γ ⊢ e : T stage P refuses R effects E requires C reads S within B
/// explains X evidences V`. Public operations expose the closed COMPLETED
/// judgment; omission from convenient source is not absence from meaning.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Judgment {
    /// T — the normal value type.
    pub normal_type: SemanticTypeRef,
    /// P — the stage, over exactly four values.
    pub stage: Stage,
    /// R — the typed refusal families.
    pub refuses: RefusalSet,
    /// E — declared effects plus first-observable ordering.
    pub effects: OrderedEffectRegions,
    /// C — capability requirements, never grants.
    pub requires: CapabilityRequirements,
    /// S — the source/cut posture.
    pub reads: SourceCutPosture,
    /// B — the applicable symbolic bounds.
    pub bounds: SymbolicBounds,
    /// X — the explanation obligation.
    pub explains: ExplanationObligation,
    /// V — the evidence obligation.
    pub evidences: EvidenceObligation,
}

// ---------------------------------------------------------------------------
// The kernel-inventory contract frame.
// ---------------------------------------------------------------------------

/// The seven behavior families the owner-derived operation inventory
/// resolves — not an inherited universal table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BehaviorFamily {
    /// Values and structure.
    ValuesAndStructure,
    /// Truth and decisions.
    TruthAndDecisions,
    /// Definitions and kernels.
    DefinitionsAndKernels,
    /// Source and query.
    SourceAndQuery,
    /// Semantic positioning and navigation.
    SemanticPositioningAndNavigation,
    /// Physical derivation.
    PhysicalDerivation,
    /// Publication and effects.
    PublicationAndEffects,
}

/// The definition boundary: which behaviors are primitive operators, which
/// are definitions over smaller operators, and which are qualified opaque
/// kernels — so NO operation is two of these.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefinitionBoundary {
    /// A primitive operator.
    PrimitiveOperator,
    /// A definition over smaller operators.
    DefinitionOverSmallerOperators,
    /// A qualified opaque kernel.
    QualifiedOpaqueKernel,
}

/// The ten facts every admitted operation states — the per-operation contract
/// column set of the owner-derived inventory (the register's rows are the
/// execution home's authored work).
pub const OPERATION_CONTRACT_FACTS: [&str; 10] = [
    "owner-and-version",
    "operand-and-result-sorts",
    "purity-observation-intent-boundary-posture",
    "capabilities",
    "source-and-authority-requirements",
    "static-bounds-and-work-formula",
    "failure-refusal-uncertainty-recovery-behavior",
    "explanation-contribution",
    "lawful-authoring-and-inspection-surfaces",
    "independent-evidence-route",
];
