//! Band 10 — history: the four-object split, exact local order, lineage,
//! partitions, federation, commit knowledge, durability, the storage port,
//! authenticated history, authorized removal, and `.tlog` recovery.

pub mod types;

pub use types::{
    AcceptedEventRecord, AuthenticatedAuthorship, AuthorityGeneration, AuthoritySequence,
    CausationEdge, CausationEdgeKindId, CommitKnowledge, CommitPoint, CommitReconciliation,
    CoverageWitness, CutTranslationWitness, DurabilityClaimAxis, DurabilityProfile,
    EpochValidation, EventCommitment, EventId, EventRole, EventSemanticBody,
    ExternallyWitnessedFreshness, FederationComposition, FederationCutEntries, FederationCutVector,
    ForeignLineageEvidence, HandoffState, HistoryAccumulatorBinding, HistoryAccumulatorRoot,
    HistoryCut, HistoryDisposition, HistoryIntegrityEvidence, HistoryPrefixBinding,
    HistoryReadRefusal, HistoryReading, HistorySegmentSealRef, ImmediateHistoryPredecessor,
    LineageRefusal, LineageRefusalEvidence, LineageRefusalReason, LineageTransition,
    LocalConsistency, OpenReadOnly, OpenWritable, PartitionId, PublicationRecord, RECOVERY_SCAN,
    ReceiptCompleteness, RecoveryOutcome, RecoveryReceipt, RemovalAdmission,
    RemovalAuthorizationClaim, RemovalAuthorizationClaimConstruction,
    RemovalAuthorizationClaimConstructionIssue, RemovalCommitment, RemovalPlan,
    RemovalPlanConstruction, RemovalPlanConstructionIssue, RemovalPolicyBasis, RemovalRefusal,
    RemovalRefusalIssue, ScopeAppliedCut, SourceClosure, SourceRegions, StoreId, StoreLineageId,
    SuccessionWitness, TlogFrameKind, TurnInputCut, WriteAuthorityEpoch, WriterOrderScope,
};
