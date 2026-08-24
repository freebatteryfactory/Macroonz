#![doc = include_str!("README.md")]
//!
//! The operation modules are role-named.
//! [`discover`] lowers a producer's discovery roster through owner policy into the one executable surface.
//! [`wrap`] reads an external backend's console output into the axis-composed record, and plans the witness runs over it.
//! [`specimen`] presses one separately rendered compiled projection across a caller-owned compiler host.
//! [`interpret`] owns the compile-once receiver: pairing, no-mutation parity, the trust boundary, and the one authorized active execution.
//! [`rewrite`] plans structural-rewrite descriptors as audit candidates.
//! [`propose`] is the proposal road end to end, ending where a human admits.
//!
//! # The execution boundary
//!
//! This home invokes the caller's production, evaluation, materializer, and host callables at their typed operations, and nothing else.
//! Trial judgment and execution evidence stay with [`crate::runner::TrialBinding`] and [`crate::report`], so mutation-specific control never becomes a second general runner.

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
    DischargeAdmissionReceipt, DischargeEvidence, DischargeProposalRefusal, DiscoveredMutationSite,
    DiscoveryDisposition, DiscoveryEntry, DiscoveryLoweringRefusal, DiscoveryRefusal, DudPlant,
    DuplicateRefusal, EVALUATION_SURFACE_TAG, EquivalenceAxis, EvaluationBinding, EvaluationCall,
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
