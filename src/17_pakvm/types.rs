//! The executor's value machine: the closed value algebra, the two-tier
//! memory model, live admitted handles, the capture record, the one-shot
//! continuation record, the step machine's productions and terminals, and
//! the six closure obligations.
//!
//! # What the executor is NOT
//!
//! Not a source-language compiler, a Rust VM, an untyped stack machine, a
//! native instruction set, an operating system, an ambient syscall surface,
//! a physical-plan cache, or an authority that can mint capabilities or
//! durable facts by interpretation. An execution result never manufactures
//! durability.
//!
//! # The two-tier memory model
//!
//! Tier 1 is the content-addressed immutable store: an immutable byte
//! region's identity IS its exact-byte digest — dedup, integrity, and
//! fearless sharing by construction. Tier 2 is per-Turn scratch arenas:
//! bounded, generational-index-addressed, capacities pre-reserved at
//! admission, reset wholesale when the Turn ends; anything that survives the
//! Turn is frozen into Tier 1. There are no raw pointers or address
//! arithmetic, and every borrowed view names its lifetime against its owner
//! so no view outlives its backing state.
//!
//! # Numeric law without weakening
//!
//! The executor runs the shared numeric law unweakened: no optimization
//! replaces exact arithmetic with approximation, erases signed-zero or
//! non-finite evidence, reorders a non-associative aggregation, or moves a
//! declared rounding crossing without a different admitted contract — and
//! default Rust float equality, casts, and widths never choose the numeric
//! law.

use crate::identity::Commitment;
use crate::semantic::BoundDimensionRow;
use crate::time::ConsumedBudgetEvidence;
use crate::types::{Bounded, EvidenceRef, Limit};
use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// The closed value algebra.
// ---------------------------------------------------------------------------

/// The nine categories of the closed typed value algebra — an exhaustive
/// enum whose concrete carrier lands with the executor machinery; the
/// roster is law now. The five prohibited inhabitants are the companion
/// const.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueCategory {
    /// Exact primitives and admitted approximations.
    ExactPrimitivesAndApproximations,
    /// Bounded text and bytes.
    BoundedTextAndBytes,
    /// Products, records, variants, options, bounded collections.
    ProductsAndCollections,
    /// Recursive algebraic data.
    RecursiveAlgebraicData,
    /// Units, intervals, margins, decisions.
    UnitsIntervalsMarginsDecisions,
    /// Role-specific identities and references.
    IdentitiesAndReferences,
    /// Availability / completeness / freshness / proof / uncertainty.
    KnowledgeAxes,
    /// Source / cursor / checkpoint / event / effect / evidence values.
    SourceAndEvidenceValues,
    /// Typed request / response / suspension / terminal values.
    BoundaryValues,
}

/// The five prohibited inhabitants — none is representable in the algebra.
pub const PROHIBITED_INHABITANTS: [&str; 5] = [
    "any",
    "host-object",
    "function-pointer",
    "raw-callback",
    "ambient-handle",
];

/// The four lawful residences of a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueResidence {
    /// A validated frame.
    ValidatedFrame,
    /// A bounded arena.
    BoundedArena,
    /// A typed borrowed view — its lifetime named against its owner.
    TypedBorrowedView,
    /// An owned value.
    OwnedValue,
}

/// A dumb arena index with a generation — it LOCATES, and that is all;
/// policy lives in the operators, never in a pointer.
///
/// # The settled posture for a reference that is persisted or transported
///
/// A reference that leaves the arena it points into is identity PLUS
/// generation, and the generation is what makes the crossing back in an
/// explicit act rather than a dereference. [`located`](Self::located) is the
/// authoring road: it names a slot and the generation the namer believes that
/// slot stands at. It establishes neither. Whether the arena's slot is live and
/// whether its generation still matches is a validation crossing performed by
/// the consuming operator against a specific arena, and it is **owed** — no
/// road here performs it.
///
/// The live-arena shape is a different one and is not this. A handle that
/// cannot outlive its arena, and so needs no generation compared at all, is a
/// declared-and-owed shape behind the runtime gate; it is named here so this
/// type is not mistaken for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArenaIndex {
    index: u32,
    generation: u32,
}

impl ArenaIndex {
    /// Name one arena slot at one generation.
    #[must_use]
    pub const fn located(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }

    /// The named slot, unresolved against any arena.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.index
    }

    /// The generation the namer believes that slot stands at, uncompared
    /// against any arena.
    #[must_use]
    pub const fn generation(self) -> u32 {
        self.generation
    }
}

// ---------------------------------------------------------------------------
// Live admitted handles — !Send, !Sync, never serializable as authority.
// ---------------------------------------------------------------------------

/// Live capability authority in the executor — a separate opaque,
/// invocation-scoped, generation-scoped, non-forgeable sort that is NOT
/// serializable as authority and never crosses the program value boundary.
/// Structurally `!Send`/`!Sync`: live authority never crosses threads by
/// trait accident; a cross-thread transfer is a NAMED consuming operation
/// minting a fresh role-specific handle — the crossing is visible, never
/// ambient. (An image serializes a capability REQUIREMENT, never live
/// authority. The Attempt handle is the membrane's; the secret-use handle is
/// the security home's.)
#[derive(Debug)]
pub struct CapabilityHandle {
    _execution_context_local: PhantomData<*const ()>,
}

/// Live port authority — same laws as the capability handle.
#[derive(Debug)]
pub struct PortHandle {
    _execution_context_local: PhantomData<*const ()>,
}

/// One-shot reply authority: grants only the live response crossing —
/// consumed on use, never reusable, never a bearer token.
#[derive(Debug)]
pub struct ReplyHandle {
    _execution_context_local: PhantomData<*const ()>,
}

// ---------------------------------------------------------------------------
// The capture record.
// ---------------------------------------------------------------------------

/// Closed-definition domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClosedDefinitionDomain;
/// Capture-environment domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CaptureDomain;
/// Capture-origin claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CaptureOriginClaim;

/// Limit family for capture environments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CaptureLimit;
impl Limit for CaptureLimit {}

/// Every portable function or lambda lowers into a closed semantic
/// definition plus this bounded typed capture record — the minimal semantic
/// free-variable environment in CANONICAL BINDING ORDER, never
/// host-map/allocation/traversal order — plus its completed judgment and
/// origins.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaptureRecord {
    /// The closed definition.
    pub definition: Commitment<ClosedDefinitionDomain>,
    /// The captures, in canonical binding order.
    pub captures: Bounded<Commitment<CaptureDomain>, CaptureLimit>,
    /// The origins.
    pub origins: EvidenceRef<CaptureOriginClaim>,
}

/// The seven invalid captures — a captured one of these is invalid, refused
/// at Semantic Form construction (the authority-bearing-capture cause).
pub const INVALID_CAPTURES: [&str; 7] = [
    "live-grant",
    "port",
    "continuation",
    "reply-authority",
    "attempt",
    "host-handle",
    "secret-authority",
];

/// The derived boundary posture of a lambda.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LambdaBoundaryPosture {
    /// Inline-only.
    InlineOnly,
    /// Invocation-bound.
    InvocationBound,
    /// Portable.
    Portable,
    /// Nonserializable.
    Nonserializable,
}

// ---------------------------------------------------------------------------
// The step machine.
// ---------------------------------------------------------------------------

/// The six productions the executor advances synchronously toward, then
/// returns control — sync-native suspension: no Rust `async`, `Future`, OS
/// thread, browser `Promise`, or async runtime.
pub const STEP_PRODUCTIONS: [&str; 6] = [
    "semantic-value",
    "typed-refusal",
    "bounded-publication-intent",
    "typed-port-request",
    "bounded-suspended-state",
    "terminal-evidence",
];

/// The ONLY terminals the executor itself may return. The physical Attempt
/// facts (completed / failed / refused / resource-exhausted / outcome-
/// unknown) are the membrane's observations; cancellation and reconciliation
/// are the runtime's interpretations — the executor can construct NONE of
/// them. Each owner's outcomes compose by typed reference in the operation's
/// result: orthogonal observables are named axes on the outcome that has
/// them, never packed into one optional-field envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VmTerminal {
    /// A pure value.
    PureValue,
    /// An effect-intent plan awaiting admission.
    EffectIntentPlan,
    /// A port request with bounded suspension.
    PortRequestSuspended,
    /// A semantic refusal before admission.
    SemanticRefusal,
    /// VM budget exhaustion at its own boundary.
    VmBudgetExceeded,
}

// ---------------------------------------------------------------------------
// The one-shot continuation record.
// ---------------------------------------------------------------------------

/// Program-identity claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProgramIdentityClaim;
/// Bounded-frame domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameDomain;
/// Request/response contract domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RequestContractDomain;
/// Request-identity domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RequestIdentityDomain;
/// Effect-intent claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectIntentClaim;
/// Attempt-binding claim marker (the live Attempt is the membrane's; the
/// persisted record names it as data).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttemptBindingClaim;
/// Generation-binding domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContinuationGenerationDomain;
/// Deadline-policy claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadlinePolicyClaim;
/// Cancellation/terminal posture domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContinuationPostureDomain;

/// Limit family for a continuation's remaining bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContinuationBoundLimit;
impl Limit for ContinuationBoundLimit {}

/// The explicit typed one-shot continuation record — suspension lowers into
/// THIS, never a retained Rust closure, native stack, host callback, task,
/// `Future`, or `Promise` as program meaning. It is resumed or terminated
/// exactly once; the resume-refusal union is enforced through the port
/// home's response-binding family (never a second family here). A persisted
/// continuation is INERT DATA and budget evidence, never live resume
/// authority: after process death only a freshly validated live handle,
/// minted by the membrane for a new lawful Attempt, may resume.
///
/// # The deadline-carriage rule
///
/// The persisted record carries the deadline-policy REFERENCE plus the
/// consumed-budget evidence — a policy has no "remainder"; the remaining
/// allowance is derived at resume by the rebase morphism, and a live
/// monotonic deadline is never persisted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContinuationRecord {
    /// The image / admitted-program identity.
    pub program: EvidenceRef<ProgramIdentityClaim>,
    /// The resume coordinate.
    pub resume_coordinate: u64,
    /// The bounded frame.
    pub frame: Commitment<FrameDomain>,
    /// The request/response contract.
    pub contract: Commitment<RequestContractDomain>,
    /// The request identity.
    pub request: Commitment<RequestIdentityDomain>,
    /// The durable effect intent.
    pub effect_intent: EvidenceRef<EffectIntentClaim>,
    /// The live Attempt binding, as data.
    pub attempt: EvidenceRef<AttemptBindingClaim>,
    /// The generations.
    pub generations: Commitment<ContinuationGenerationDomain>,
    /// The remaining bounds.
    pub remaining_bounds: Bounded<BoundDimensionRow, ContinuationBoundLimit>,
    /// The deadline-policy reference.
    pub deadline_policy: EvidenceRef<DeadlinePolicyClaim>,
    /// The recorded spend — budget evidence, rebased at resume.
    pub spend: ConsumedBudgetEvidence,
    /// The cancellation/terminal posture.
    pub posture: Commitment<ContinuationPostureDomain>,
}

// ---------------------------------------------------------------------------
// Transition-System Closure.
// ---------------------------------------------------------------------------

/// The six closure obligations every lifecycle state machine owes — a
/// conformance bar, deliberately NOT a universal state-machine type. The
/// structural spine: a compile-time shape makes the wrong move
/// unrepresentable, and a runtime-validated fact carried in the value's own
/// canonical bytes enforces the right move at the operation boundary.
/// Evidence, not assertion: the owner produces a generated transition table
/// or a simple reference model, and an independent route compares the
/// running machine against it — the machine and its judge do not share the
/// dispatch path being judged.
pub const CLOSURE_OBLIGATIONS: [&str; 6] = [
    "exact-initial-posture",
    "reachable-state-analysis",
    "valid-transition-endpoints",
    "terminal-state-law",
    "deterministic-dispatch-or-declared-ambiguity",
    "total-typed-refusal",
];
