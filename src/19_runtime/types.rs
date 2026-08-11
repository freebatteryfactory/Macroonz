//! The runtime: the Stitch contract, the Turn and its phases, the identity
//! quartet, Attempt lineage, checkpoints, effect recovery, reconciliation,
//! the cancellation fact model, delivery, driving, and supervision.
//!
//! # The Stitch
//!
//! One synchronous semantic protocol: admitted logical state plus exactly one
//! typed observation yields one bounded deterministic transition, ending in
//! exactly one of the seven outputs. The semantic core is sans-I/O — it opens
//! no file, socket, clock, entropy provider, process, browser API, or async
//! executor; async is a host-driving strategy, never a second semantic
//! runtime, and a driver that requires an ambient executor for the machine's
//! meaning is refused. Sharing a crate never transfers ownership: the runtime
//! cannot call an ambient host mechanism, the executor cannot advance a
//! durable checkpoint, the membrane cannot decide semantic retry, and a
//! driver cannot turn a wakeup or timeout into durable progress. (The Stitch
//! trait's concrete shape lands with the runtime machinery — the contract's
//! outputs and invariance lists are law now.)
//!
//! # The logical thread
//!
//! Typed continuity across accepted facts, decisions, Turns, Attempts,
//! effects, outcomes, receipts, replay, and reconciliation — not a runtime
//! task, OS thread, queue, actor handle, connection, or universal correlation
//! string. There is no universal thread identity.
//!
//! # Two honest books
//!
//! The logical book (Turn, operation, semantic result, effect intent,
//! checkpoint meaning, retry legality) and the physical book (Attempt,
//! grants, port crossings, commit knowledge, observations, terminal
//! evidence). Receipts and reconciliation bind facts from both without
//! pretending they are one authority; a disagreement becomes an explicit
//! reconciliation posture, never an edit that makes the books appear to have
//! always agreed. No publication, checkpoint advance, or retry authority
//! enters the logical book without the evidence owned by its boundary; no
//! physical failure erases an already-admitted logical entry.

use crate::bvisor::{AttemptId, ReservationObservation};
use crate::history::CommitKnowledge;
use crate::identity::{Commitment, CreationLaw, IdentityClass, IdentityRole, Occurrence};
use crate::types::{Bounded, EvidenceRef, Limit};

// ---------------------------------------------------------------------------
// The Stitch contract and driver invariance.
// ---------------------------------------------------------------------------

/// The seven Stitch outputs — exactly one per transition.
pub const STITCH_OUTPUTS: [&str; 7] = [
    "value",
    "publication-intent",
    "port-request",
    "suspension",
    "refusal",
    "reconciliation-action",
    "terminal-evidence",
];

/// The closed list of what a driving strategy may NEVER change.
pub const DRIVER_INVARIANCE: [&str; 15] = [
    "turn-identity",
    "effect-meaning",
    "checkpoint-authority",
    "cancellation-posture",
    "retry-legality",
    "result-semantics",
    "delivery-admission-and-ordering",
    "one-shot-response-binding",
    "caller-abandonment-meaning",
    "deadline-policy-and-derived-allowances",
    "checkpoint-and-publication-ordering",
    "effect-recovery-and-outcome-unknown",
    "semantic-work-accounting",
    "receipt-and-reconciliation-obligations",
    "canonical-logical-traces",
];

/// What a driver MAY change — scheduling and physical evidence only.
pub const DRIVER_MAY_CHANGE: [&str; 7] = [
    "which-lane-runs-first",
    "pump-drain-count",
    "when-a-wake-occurs",
    "which-thread-acts",
    "poll-frequency",
    "batching",
    "wall-time",
];

/// The nine driver-family rows — all preserve the invariance list; an
/// adapter may expose host-native ergonomics but may not create a second
/// semantic API whose behavior changes when a future is dropped, a promise
/// loses its page, a task is aborted, a runtime shuts down, or a callback
/// arrives late.
pub const DRIVER_FAMILY: [&str; 9] = [
    "direct-blocking",
    "cooperative-poll-pump",
    "threaded-native",
    "rust-future",
    "browser-promise",
    "browser-worker",
    "ecosystem-async-runtime",
    "embedded-custom",
    "deterministic-testpak",
];

/// The twelve liveness facts every driver/delivery profile declares —
/// fairness is a declared claim, not queue-library folklore; no wake
/// protocol may lose work in the check-to-register/register-to-park window;
/// the last row is the honest statement of "stuck", never a false claim of
/// progress.
pub const LIVENESS_DECLARATION: [&str; 12] = [
    "who-drives",
    "when-driving-is-required",
    "max-work-per-pump",
    "runnable-lane-fairness",
    "starvation-posture",
    "wake-registration-and-lost-wakeup-prevention",
    "spurious-wake-and-coalescing-handling",
    "reentrancy",
    "callback-custody",
    "shutdown-drain-restart",
    "panic-and-host-failure-containment",
    "which-liveness-claim-becomes-unavailable",
];

// ---------------------------------------------------------------------------
// The identity quartet and the Turn.
// ---------------------------------------------------------------------------

/// The identity role marker for logical operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogicalOperationRole;

/// One requested semantic operation ACROSS lawful retries — Class D, fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogicalOperationId(Occurrence<LogicalOperationRole>);

impl IdentityRole for LogicalOperationId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

/// The identity role marker for Turns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TurnRole;

/// One bounded logical transition over frozen inputs — Class D, DERIVED:
/// replay-stable under the derived-seat law (replay is the named consumer of
/// convergence; the runtime custodies the preimage). Replaying the same
/// transition RECONSTRUCTS the same identity; changing any identity-bearing
/// input creates a different Turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TurnId(Occurrence<TurnRole>);

impl IdentityRole for TurnId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::DerivedFromAdmittedPreimage;
}

impl TurnId {
    /// In-crate mint for laws. Test-gated until replay derivation exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<TurnRole>) -> Self {
        Self(occurrence)
    }
}

/// The Turn preimage's at-least list — a Turn freezes a scope-local exact
/// cut per source, never a generic progress summary, HLC, page cursor,
/// route, delivery sequence, or wall-clock instant; a missing source stays
/// explicit and is never silently replaced by the newest reachable one.
pub const TURN_PREIMAGE: [&str; 7] = [
    "process-contract",
    "process-coordinate",
    "process-generation",
    "exact-input-source-set-and-frozen-cuts",
    "logical-operation",
    "application-generation",
    "partition-epoch-where-governing",
];

/// The identity role marker for effect intents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectIntentRole;

/// One durable external-effect request — Class D, fresh, INDEPENDENT of any
/// Attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectIntentId(Occurrence<EffectIntentRole>);

impl IdentityRole for EffectIntentId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl EffectIntentId {
    /// In-crate mint for laws. Test-gated until intent admission exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<EffectIntentRole>) -> Self {
        Self(occurrence)
    }
}

/// The fourteen Turn phases — semantic phases realized as a runtime state
/// machine PLUS persisted data, never one mutable object (the prose pairs
/// executing/live-suspended and the three outstanding/settled pairs; the
/// flat fourteen is the persisted vocabulary). Initial posture is
/// `Runnable`; terminal is `ReconciliationComplete`, past which replay
/// identity does not silently resume; every unmatched (phase, observation)
/// pair yields a typed refusal — never a silent drop, never a panic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnPhase {
    /// The one lawful initial posture.
    Runnable,
    /// The exact input cuts are frozen.
    CutFrozen,
    /// Planned.
    Planned,
    /// An Attempt is requested.
    AttemptRequested,
    /// Executing.
    Executing,
    /// Live-suspended.
    LiveSuspended,
    /// Physically observed.
    PhysicallyObserved,
    /// Semantically interpreted.
    SemanticallyInterpreted,
    /// Publication outstanding.
    PublicationOutstanding,
    /// Publication admitted.
    PublicationAdmitted,
    /// Checkpoint outstanding.
    CheckpointOutstanding,
    /// Checkpoint advanced.
    CheckpointAdvanced,
    /// Reconciliation outstanding.
    ReconciliationOutstanding,
    /// Terminal — replay identity does not silently resume past it.
    ReconciliationComplete,
}

// ---------------------------------------------------------------------------
// Attempt lineage — message-passing, not inheritance.
// ---------------------------------------------------------------------------

/// The role-qualified cause sum — names exactly ONE causal predecessor,
/// binding one endpoint: no edge identity, no second endpoint, no
/// edge-specific evidence (the `…Edge` spelling is reserved for a value that
/// independently binds both endpoints; a bare `Cause` is refused —
/// Attempt-lineage causation, diagnostic causation, and handling causes are
/// three vocabularies that stay apart).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttemptCause {
    /// Caused by a Turn.
    Turn(TurnId),
    /// Caused by an effect intent.
    EffectIntent(EffectIntentId),
    /// Caused by a prior Attempt.
    Attempt(AttemptId),
}

/// Limit family for cause sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CauseSetLimit;
impl Limit for CauseSetLimit {}

/// A SET under a declared bound: membership is what it states — the same
/// causes in a different order are the same value. Storage order may be made
/// deterministic for canonical emission only; that determinism never
/// promises insertion, storage, or iteration order carries causal meaning.
/// Joins and reconciliation may name several causes jointly; single-parent
/// is the common case.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundedCauseSet {
    /// The member causes.
    pub causes: Bounded<AttemptCause, CauseSetLimit>,
}

/// One node of the id-keyed causal DAG (an owner table — never a pointer
/// graph, never a subclass): an immutable value with a fresh Attempt
/// identity and its bounded cause set. Cancellation and termination
/// propagate along a typed fate-link expressed as DATA — never "a parent
/// Attempt owning and destroying children." A retry creates a fresh Attempt
/// against the same lawful logical identity and never adopts a prior
/// Attempt's response.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttemptLineageNode {
    /// The Attempt.
    pub attempt: AttemptId,
    /// Its causes.
    pub causes: BoundedCauseSet,
}

// ---------------------------------------------------------------------------
// The durable checkpoint.
// ---------------------------------------------------------------------------

/// Process/subscription identity domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessIdentityDomain;
/// Checkpoint source-binding claim marker (lineages, generations, cuts).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckpointSourceClaim;
/// Prior-checkpoint relationship claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PriorCheckpointClaim;
/// Required result/publication evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckpointEvidenceClaim;
/// Outstanding effect/reconciliation posture domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutstandingPostureDomain;
/// Admitting-boundary evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdmittingBoundaryClaim;

/// The SINGLE authority that permits a later execution to skip accepted
/// inputs before its declared cut. Survives loss of process memory, driver,
/// host task, connection, transport session, wake source, and every
/// disposable cache. Advances ONLY on complete prerequisites. Accepted-event
/// truth follows the event authority's own boundary and never waits on a
/// projection, `DataBlock`, index, or cache; a process may separately require
/// composite materialization evidence, narrowing its progress claim without
/// making an accepted event untrue.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DurableCheckpoint {
    /// The process or subscription identity and generation.
    pub process: Commitment<ProcessIdentityDomain>,
    /// The exact source lineages, generations, and cuts.
    pub sources: EvidenceRef<CheckpointSourceClaim>,
    /// The prior-checkpoint relationship.
    pub prior: EvidenceRef<PriorCheckpointClaim>,
    /// The required result and publication evidence.
    pub evidence: EvidenceRef<CheckpointEvidenceClaim>,
    /// The outstanding effect/reconciliation posture.
    pub outstanding: Commitment<OutstandingPostureDomain>,
    /// Evidence of the boundary that admitted it.
    pub admitted_by: EvidenceRef<AdmittingBoundaryClaim>,
}

/// The eight closed non-reasons — a checkpoint NEVER advances because of
/// any of these.
pub const CHECKPOINT_NON_REASONS: [&str; 8] = [
    "a-notification-arrived",
    "an-hlc-advanced",
    "a-cursor-moved",
    "an-attempt-started-or-terminated",
    "a-result-was-computed-but-not-admitted",
    "a-caller-stopped-waiting",
    "a-route-changed",
    "a-derived-view-caught-up",
];

/// The four process-state roles — AUTHORED name (the roles are fixed, the
/// enum name never was). Encryption, persistence, sealing, or reconstruction
/// expense never chooses the role; the fourth is admitted only under a
/// complete owner/boundary/lineage/recovery contract, and missing mutable
/// authority is refused, never silently treated as empty state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessStateRole {
    /// Event-reconstructible state.
    EventReconstructible,
    /// Derived fast-start state.
    DerivedFastStart,
    /// Durable checkpoint authority.
    DurableCheckpointAuthority,
    /// Genuine mutable authority.
    GenuineMutableAuthority,
}

// ---------------------------------------------------------------------------
// Effect recovery.
// ---------------------------------------------------------------------------

/// Idempotency-key domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdempotencyKeyDomain;
/// Idempotency-scope domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdempotencyScopeDomain;

/// Limit family for supported-key sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeySupportLimit;
impl Limit for KeySupportLimit {}

/// What the effect contract supports — `None` is the explicit weaker posture
/// recorded UP FRONT, never missing data (how a stable key is established is
/// the ingress home's; this axis owns what the contract supports).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdempotencyKeySupport {
    /// No key support — the weaker posture, stated.
    None,
    /// Supported key identities.
    Supported(Bounded<Commitment<IdempotencyKeyDomain>, KeySupportLimit>),
}

/// The idempotency pair-fact axis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyPosture {
    /// The key support.
    pub key: IdempotencyKeySupport,
    /// The scope.
    pub scope: Commitment<IdempotencyScopeDomain>,
}

/// Outcome-query availability domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutcomeQueryDomain;
/// Outcome-evidence-strength domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutcomeEvidenceDomain;

/// The outcome-query pair-fact axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutcomeQuerySupport {
    /// The availability.
    pub availability: Commitment<OutcomeQueryDomain>,
    /// The evidence strength.
    pub evidence: Commitment<OutcomeEvidenceDomain>,
}

/// Compensation-availability domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompensationDomain;
/// Compensation-preconditions domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompensationPreconditionDomain;

/// The compensation pair-fact axis — compensation is always a NEW admitted
/// effect that neither erases the original nor promises the outside world
/// returned to an identical state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompensationSupport {
    /// The availability.
    pub availability: Commitment<CompensationDomain>,
    /// The preconditions.
    pub preconditions: Commitment<CompensationPreconditionDomain>,
}

/// Duplicate-execution posture domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DuplicatePostureDomain;

/// Replay posture — closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplayPosture {
    /// Replayable.
    Replayable,
    /// Not replayable.
    NonReplayable,
}

/// Concurrency-limit domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConcurrencyLimitDomain;
/// Lease-constraint domain marker — a worker lease coordinates mechanics and
/// grants no semantic retry authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LeaseConstraintDomain;

/// The concurrency pair-fact axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConcurrencyConstraints {
    /// The concurrency limit.
    pub concurrency: Commitment<ConcurrencyLimitDomain>,
    /// The lease constraint.
    pub lease: Commitment<LeaseConstraintDomain>,
}

/// External-acknowledgement semantics domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AckSemanticsDomain;
/// Evidence-retention domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvidenceRetentionDomain;
/// Evidence-freshness-requirement domain marker — deliberately NOT the root
/// freshness axis: that axis states whether one evidence value IS currently
/// fresh; this states what the recovery contract DEMANDS of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FreshnessRequirementDomain;

/// The evidence pair-fact axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvidenceRetention {
    /// The retention.
    pub retention: Commitment<EvidenceRetentionDomain>,
    /// The freshness requirement.
    pub freshness_requirement: Commitment<FreshnessRequirementDomain>,
}

/// Manual-intervention requirement domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ManualInterventionDomain;

/// The nine orthogonal coexisting recovery properties — never one packed
/// enum; the five pair-fact axes are records with named typed sub-axes,
/// never booleans. Every property the profile claims must be bound and
/// usable BEFORE the irreversible Attempt; a profile missing a fact records
/// the weaker posture up front and can never acquire retry, query, or
/// compensation authority from residue observed afterward.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectRecoveryProfile {
    /// 1 — idempotency.
    pub idempotency: IdempotencyPosture,
    /// 2 — outcome query.
    pub outcome_query: OutcomeQuerySupport,
    /// 3 — compensation.
    pub compensation: CompensationSupport,
    /// 4 — duplicate execution.
    pub duplicate_execution: Commitment<DuplicatePostureDomain>,
    /// 5 — replay.
    pub replay: ReplayPosture,
    /// 6 — concurrency.
    pub concurrency: ConcurrencyConstraints,
    /// 7 — external acknowledgement.
    pub external_ack: Commitment<AckSemanticsDomain>,
    /// 8 — evidence.
    pub evidence: EvidenceRetention,
    /// 9 — manual intervention.
    pub manual_intervention: Commitment<ManualInterventionDomain>,
}

/// The seven lawful recovery actions from the current fact set.
pub const RECOVERY_ACTIONS: [&str; 7] = [
    "return-an-existing-terminal-result",
    "resume-an-admitted-live-attempt",
    "query-the-external-outcome",
    "start-a-fresh-idempotent-attempt",
    "propose-a-compensation-effect",
    "remain-outcome-unknown",
    "require-an-authorized-human-decision",
];

/// Never sufficient on its own to authorize any recovery action.
pub const NEVER_SUFFICIENT: [&str; 6] = [
    "elapsed-wall-time",
    "process-death",
    "expired-lease",
    "missing-acknowledgement",
    "lost-connection",
    "missing-waiter",
];

// ---------------------------------------------------------------------------
// Reconciliation — separate axes, never one fused enum.
// ---------------------------------------------------------------------------

/// External-outcome domain marker — typed by the OWNING effect contract;
/// never a bare success/failure bit. The membrane reports physical
/// observations; the knowledge CONCLUSION is the runtime's, never the
/// observer's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalOutcomeDomain;

/// The external outcome, contract-typed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExternalOutcome(pub Commitment<ExternalOutcomeDomain>);

/// Do we know the external outcome? Owned here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutcomeKnowledge {
    /// The outcome remains unknown — uncertainty stays uncertainty.
    OutcomeUnknown,
    /// The outcome is known.
    Known(ExternalOutcome),
}

/// The terminal handling meaning that exists ONLY at completion — carrying
/// it inside `Complete` makes disposition-without-completion unrepresentable,
/// and `Outstanding` can never masquerade as resolved handling. Lifecycle
/// answers WHETHER; disposition answers HOW; the two never share one enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReconciliationDisposition {
    /// Reconciled.
    Reconciled,
    /// A compensation was proposed.
    CompensationProposed,
    /// Manual intervention is required — a typed authorized decision that
    /// cannot mint a physical fact it did not observe.
    ManualInterventionRequired,
    /// Further automatic action is refused.
    AutomaticActionRefused,
}

/// The reconciliation lifecycle. Effect-outcome reconciliation is THIS
/// home's family; commit reconciliation is history's — one name never spans
/// both owners.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReconciliationLifecycle {
    /// No reconciliation is required.
    NotRequired,
    /// Owed but not yet performed.
    Outstanding,
    /// Complete, with its disposition.
    Complete(ReconciliationDisposition),
}

/// One reconciliation record — append-or-reference-only: it never edits the
/// original to make uncertainty disappear, and later green evidence never
/// erases an earlier red or unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectReconciliationRecord {
    /// The commit-knowledge axis (history's, by reference).
    pub commit: CommitKnowledge,
    /// The outcome-knowledge axis.
    pub outcome: OutcomeKnowledge,
    /// The lifecycle axis.
    pub lifecycle: ReconciliationLifecycle,
}

// ---------------------------------------------------------------------------
// The cancellation FACT MODEL — no single outcome enum exists.
// ---------------------------------------------------------------------------

/// Cancellation-actor domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CancellationActorDomain;
/// Cancelled-operation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CancelledOperationDomain;

/// A cancellation request: which authorized actor, for which operation and
/// generation. (The former two-variant cancellation outcome is RETIRED — it
/// collapsed one axis into the whole truth; several facts may hold at once.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CancellationRequest {
    /// The authorized actor.
    pub actor: Commitment<CancellationActorDomain>,
    /// The operation and generation.
    pub operation: Commitment<CancelledOperationDomain>,
}

/// Where cancellation stood relative to durable admission. After admission,
/// cancellation or expiry is NEVER proof of noncommit or noncompletion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CancellationDurablePosition {
    /// Before durable admission.
    BeforeDurableAdmission,
    /// After durable admission.
    AfterDurableAdmission,
}

/// Where cancellation stood physically — observed by the membrane, modeled
/// here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CancellationPhysicalPosition {
    /// Before Attempt admission.
    BeforeAttemptAdmission,
    /// After Attempt admission, before a host crossing.
    AfterAttemptAdmissionBeforeHostCrossing,
    /// During or after a host crossing.
    DuringOrAfterHostCrossing,
}

/// What the physical boundary established about a cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CancellationObservation {
    /// A late observation after abandonment or expiry.
    LateObservation,
    /// The mechanism does not support cancellation.
    MechanismUnsupported,
    /// The mechanism accepted cancellation; the external outcome remains
    /// unknown.
    AcceptedOutcomeUnknown,
}

/// The eight distinct cancellation-adjacent facts kept apart — observer
/// abandonment is its own fact and cancels nothing; deadline expiries are
/// temporal observations, never cancellation variants.
pub const CANCELLATION_DISTINCT_FACTS: [&str; 8] = [
    "caller-abandonment",
    "cancellation-request",
    "before-admission",
    "after-durable-admission",
    "attempt-deadline-expiry",
    "operation-deadline-expiry",
    "late-observation",
    "cancellation-requiring-reconciliation",
];

// ---------------------------------------------------------------------------
// Delivery and bounds.
// ---------------------------------------------------------------------------

/// The four delivery roles — there is no universal concurrency owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeliveryRole {
    /// Bounded many-submitters to one logical consumer lane ("one logical
    /// consumer" = one authority advances the lane's state at a time, not
    /// one global thread).
    Mailbox,
    /// One admitted operation to one eventual terminal observation —
    /// resolves AT MOST once.
    Completion,
    /// One producer to multiple independently positioned observers under one
    /// declared retention/overrun contract.
    Broadcast,
    /// Bounded reusable capacity acquired and released under one policy.
    Permit,
}

/// The nine separate Mailbox facts — the implication chain never runs
/// backward, and a wake proves none of them; wall-clock arrival never
/// silently becomes semantic order.
pub const MAILBOX_FACTS: [&str; 9] = [
    "submit-attempted",
    "capacity-requested",
    "capacity-reserved",
    "item-validated",
    "item-admitted",
    "wake-requested",
    "item-selected-by-consumer",
    "item-processed",
    "owning-operation-completed",
];

/// The Completion role's exhaustive terminal set — it promises one honest
/// relationship for observing the strongest terminal fact the operation can
/// establish, NOT that an external system will eventually yield a knowable
/// success or failure; dropping a Completion observer destroys none of the
/// underlying facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompletionTerminal {
    /// A completed result.
    CompletedResult,
    /// A typed refusal.
    TypedRefusal,
    /// A cancellation observation.
    CancellationObservation,
    /// Closed before observation.
    ClosedBeforeObservation,
    /// A budget or resource terminal.
    BudgetResourceTerminal,
    /// The outcome remains unknown.
    OutcomeUnknown,
}

/// Affine bounded reusable capacity — custody is not proof: affine custody
/// is compile-time hygiene, and where external accounting needs a release
/// FACT, an explicit close produces typed release evidence per resource
/// kind — drop emits no claim, and there is no generic released flag. A
/// Permit is NOT authority to perform the operation, a capability grant,
/// semantic budget, carrier credit, or the membrane's reservation.
#[derive(Debug)]
pub struct Permit {
    _process_local: core::marker::PhantomData<*const ()>,
}

/// The four never-substitutable motions — where recovery is promised,
/// authenticated bounded pull from accepted history plus a durable
/// checkpoint is the source of truth; push may live indefinitely but its
/// memory may not.
pub const FOUR_MOTIONS: [&str; 4] = ["pull", "push", "wake", "durable-checkpoint"];

/// How an ADMITTED operation ended at a bound — never semantic invalidity;
/// neither variant permits partial publication or checkpoint advancement,
/// and composition-time declared-bound refusals never reuse this vocabulary.
/// The resource-exhausted arm BINDS the membrane's physical record (the
/// Attempt-existence line: the admission issue answers why no Attempt was
/// admitted; this answers how an admitted one ended — no conversion).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BoundOutcome {
    /// Declared work exceeded its admitted bound — this home's own fact.
    BudgetExceeded,
    /// A physical resource was exhausted.
    ResourceExhausted {
        /// The membrane's observation, bound never re-established.
        observation: ReservationObservation,
    },
}

// ---------------------------------------------------------------------------
// Supervision.
// ---------------------------------------------------------------------------

/// Fate-sharing domain marker (which termination may cause another).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FateSharingDomain;
/// Termination-observation claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminationObservationClaim;

/// The recovery authority — observing termination never grants it, fate
/// sharing never decides retry legality, and a restart strategy never
/// converts a crash into retry authority. (Strategy names — one-for-one,
/// one-for-all, rest-for-one — are explicitly NOT frozen public names.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticRecoveryAuthority {
    /// Restart.
    Restart,
    /// Resume.
    Resume,
    /// Compensate.
    Compensate,
    /// Quarantine.
    Quarantine,
    /// Escalate.
    Escalate,
}

/// The three role-distinct supervision relationships. Semantic supervision
/// is not physical isolation: containment profiles change mechanisms and
/// crash evidence without changing program semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Supervision {
    /// Fate sharing.
    pub fate: Commitment<FateSharingDomain>,
    /// The bounded terminal notification.
    pub observation: EvidenceRef<TerminationObservationClaim>,
    /// The recovery authority.
    pub recovery: SemanticRecoveryAuthority,
}
