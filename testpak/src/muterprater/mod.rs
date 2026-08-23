#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! `types.rs` declares this home's public vocabulary: the verdict chain's
//! axes, the per-mutant record and its run, the mutation target and its owner
//! mapping, the wrap lane's adapter profile and generic suite-pressure vocabulary,
//! the exact compiled-projection road, the interpreted lane's evaluation surface
//! and selection-scoped trust gate, the rewrite lane's descriptors, the artifact-mutation seed
//! roster, the survivor explanation and the check gap, the scope shapes and the
//! proof plan, and the whole proposal road. Every road that reaches one of its
//! private fields is its own child, `type_guard.rs`; declarative trait
//! participation is in `type_contract.rs`.
//!
//! The lanes are role-named operation modules. [`wrap`] reads a compiled mutation backend's console output into the axis-composed record under the profile that states which grammar the reading stands on and what it may claim, then plans its witness runs.
//! [`specimen`] owns separately rendered compiled projection pressure over caller-host reports; the outside lane observes the admitted host's real compiler execution. [`interpret`] owns the compile-once receiver: exact production/evaluation pairing, evaluation-only selection, mandatory no-mutation parity, and the typed trust boundary that joins generic suite bite to one exact compiled selection.
//! [`rewrite`] plans structural-rewrite descriptors as audit candidates, admitted
//! last. [`propose`] is the proposal road end to end — survivor to candidate,
//! candidate to demonstrated kill, opening to routed obligation, and the exit
//! where a human admits.
//!
//! # The execution boundary
//!
//! Muterprater invokes the production/evaluation pair, the separately bound specimen materializer, and the caller-owned specimen host at their typed operations. The exact [`crate::runner::TrialBinding`] and [`crate::report`] vocabulary remain the owners of trial judgment and execution evidence, so mutation-specific control does not become a second general runner or enter production.

pub mod interpret;
pub mod discover;
pub mod propose;
pub mod rewrite;
pub mod specimen;
pub mod wrap;

mod encode;
mod type_contract;
mod types;

pub use types::{
    ARTIFACT_CONTENT_TAG, ARTIFACT_MUTATIONS, ActivationAxis, ActivationDisposition,
    ActivationEvidence, ActivationSite, ActiveSelection, AdapterProfile, AdapterQualification,
    AdmittedAlternative, AlternativeDeclaration, AlternativeId, AnnouncedRoster, ArtifactContent,
    ArtifactContentId, ArtifactMutation, BackendVersion, BackendVersionPosture,
    BackendVersionRefusal, BaselineAxis, BaselinePrecondition, BaselineQualification,
    BudgetRefusal, CandidateSketch, CheckGap, ClaimCeiling, ClaimPinnedGround, ClaimPinnedProposal,
    CompiledProjectionPressure, CompiledProjectionRefusal, CompiledSpecimenHost,
    CompiledSpecimenHostRefusal, CompiledSpecimenObservation, CompiledSpecimenObservationMismatch,
    CompiledSpecimenRequest, CompiledSpecimenRole, CompiledSpecimenStanding, CompiledSuitePressure,
    CoordinateRefusal, DemonstratedRejection, Demonstration, DiffPath, DiffPathRefusal,
    DischargeAdmissionReceipt, DischargeEvidence, DiscoveredMutationSite, DiscoveryDisposition,
    DiscoveryEntry, DiscoveryLoweringRefusal, DiscoveryRefusal, DudPlant, DuplicateRefusal,
    EVALUATION_SURFACE_TAG, EquivalenceAxis, EvaluationBinding, EvaluationCall,
    EvaluationCallRefusal, EvaluationDirective, EvaluationFamilyRef, EvaluationObservation,
    EvaluationPair, EvaluationPairRefusal, EvaluationPairStanding, EvaluationPairStandingMismatch,
    EvaluationSurface, EvaluationSurfaceId, ExecutionAxis, ExplanationRefusal, FailureComparison,
    FamilyAttribution, FamilyLookup, GrammarStanding, GrammarVersion, HumanAdmissionRefusal,
    InconclusiveCause, InferredObligation, IntendedRejection, InterpretedExecutionRefusal,
    InterpretedMutationEvidence, InterpretedTrust, InterpreterAvailability, KillProposalRefusal,
    KillRefusal, MUTATION_ALTERNATIVE_TAG, MUTATION_DISCOVERY_TAG, MUTATION_POLICY_TAG,
    MUTATION_TARGET_TAG, MUTERPRATER_NAMESPACE, MappedUnpermittedCause, MappingPosture,
    MaterializationAxis, MeaningCheck, MissingTrustEvidence, MutantId, MutantKilledGround,
    MutantKilledProposal, MutationCensus, MutationDiscoveryId, MutationDiscoveryReading,
    MutationIdentity, MutationOutcome, MutationPermission, MutationPoint, MutationPolicy,
    MutationPolicyId, MutationReport, MutationRun, MutationSite, MutationSurfaceLowering,
    MutationTarget, MutationVerdict, MutationWitness, MutationWitnessRefusal, NO_MUTATION_PAIRING,
    NoComparison, NoComparisonReason, NoMutationObservationRefusal, NoMutationParityQualification,
    NoMutationParityReading, NoMutationParityStanding, NoMutationResults, ObligationComparison,
    ObligationDischargedGround, ObligationDischargedProposal, ObligationLane, OperatorFamilyRef,
    OracleClass, OwedClaim, OwedClaimRefusal, OwedDeclaration, OwnerClaimMapping, OwnerLookup,
    PARITY_DECLARATION_SUBSTRATE, PARITY_RENDERING_SUBSTRATE, PROPOSAL_TAG,
    ParityQualificationRefusal, ParityRefusal, PermissionRefusal, PlanRefusal, PlannedDamage,
    PlannedRun, PointCatalogPosture, PolicyMembership, PolicyRefusal, PressureBudget, PressureLane,
    ProductionBinding, ProductionCall, ProofDelta, ProofDeltaRefusal, ProofPlan, ProofRefusal,
    ProofShape, ProposalDestination, ProposalDocument, ProposalRefusal, ProposalSink,
    QualificationRefusal, ReadingSource, RejectedNoMutationParity, RejectionIdentity,
    ReplayAdmissionReceipt, ReplayBearingProposal, ResolvedMutation, RewriteAdmission,
    RewriteCandidate, RewriteDescriptor, RewriteRefusal, RewriteRoster, RewriteTrust,
    RewriteWithheld, RosterRefusal, ScopeShape, ScopedInvocation, SelectionRefusal, SinkRefusal,
    SourceCoordinate, SpecimenMaterializerBinding, SpecimenMaterializerCall,
    SpecimenMaterializerRefusal, StoredProposalRef, SuitePressureRefusal, SurvivorExplanation,
    SynthesisRefusal, UnparsedLine, WrapOutcomeWord, WrapReading, WrapRefusal, WrapStanding,
    WrappedBackend,
};
