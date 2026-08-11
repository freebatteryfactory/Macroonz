//! Band 19 — runtime: the Stitch, the Turn, Attempt lineage, checkpoints,
//! effect recovery, reconciliation, cancellation, delivery, supervision.

pub mod types;

pub use types::{
    AttemptCause, AttemptLineageNode, BoundOutcome, BoundedCauseSet, CANCELLATION_DISTINCT_FACTS,
    CHECKPOINT_NON_REASONS, CancellationDurablePosition, CancellationObservation,
    CancellationPhysicalPosition, CancellationRequest, CompensationSupport, CompletionTerminal,
    ConcurrencyConstraints, DRIVER_FAMILY, DRIVER_INVARIANCE, DRIVER_MAY_CHANGE, DeliveryRole,
    DurableCheckpoint, EffectIntentId, EffectReconciliationRecord, EffectRecoveryProfile,
    EvidenceRetention, ExternalOutcome, FOUR_MOTIONS, IdempotencyKeySupport, IdempotencyPosture,
    LIVENESS_DECLARATION, LogicalOperationId, MAILBOX_FACTS, NEVER_SUFFICIENT, OutcomeKnowledge,
    OutcomeQuerySupport, Permit, ProcessStateRole, RECOVERY_ACTIONS, ReconciliationDisposition,
    ReconciliationLifecycle, ReplayPosture, STITCH_OUTPUTS, SemanticRecoveryAuthority, Supervision,
    TURN_PREIMAGE, TurnId, TurnPhase,
};
