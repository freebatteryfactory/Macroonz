//! The declaration-free compatibility bridge over the eight Muterprater semantic owners.
//!
//! Every public name is reexported from its semantic owner so the existing parent roster keeps one stable path.
//! This file declares no semantic type, invariant, constructor, or contract.

pub use super::backend::types::{
    AdapterProfile, AdapterQualification, AnnouncedRoster, ArtifactCustodyRefusal,
    ArtifactManifestRefusal, BACKEND_OUTPUT_TAG, BackendCommand, BackendCommandRefusal,
    BackendOutputId, BackendVersion, BackendVersionPosture, BackendVersionRefusal, ClaimCeiling,
    CompiledSuiteArtifactCustody, CompiledSuiteArtifactManifest, CompiledSuiteArtifactStanding,
    CompiledSuitePressure, FamilyLookup, GrammarStanding, GrammarVersion,
    MUTATION_SOURCE_REVISION_TAG, MutationBackendInvocation, MutationSourceRevision,
    MutationSourceRevisionId, OwnerLookup, QualificationRefusal, ReadingSource,
    SuitePressureRefusal, UnparsedLine, WrapOutcomeWord, WrapReading, WrapRefusal, WrappedBackend,
};
pub use super::pressure::types::{
    BudgetRefusal, DiffPath, DiffPathRefusal, PlanRefusal, PlannedDamage, PlannedRun,
    PressureBudget, PressureLane, ProofPlan, ScopeShape, ScopedInvocation,
};
pub use super::verdict::{
    ActivationAxis, ActivationDisposition, ActivationEvidence, BaselineAxis, BaselinePrecondition,
    BaselineQualification, CoordinateRefusal, DemonstratedRejection, DudPlant, EquivalenceAxis,
    ExecutionAxis, FamilyAttribution, InconclusiveCause, IntendedRejection, KillRefusal,
    MUTATION_TARGET_TAG, MappingPosture, MaterializationAxis, MutantId, MutationCensus,
    MutationIdentity, MutationOutcome, MutationReport, MutationRun, MutationSite, MutationTarget,
    MutationVerdict, OperatorFamilyRef, RejectionIdentity, SourceCoordinate,
};

pub use super::discovery::types::{
    ActivationSite, ActiveSelection, AdmittedAlternative, AlternativeDeclaration, AlternativeId,
    DiscoveredMutationSite, DiscoveryDisposition, DiscoveryEntry, DiscoveryLoweringRefusal,
    DiscoveryRefusal, EVALUATION_SURFACE_TAG, EvaluationCallRefusal, EvaluationDirective,
    EvaluationFamilyRef, EvaluationSurface, EvaluationSurfaceId, MUTATION_ALTERNATIVE_TAG,
    MUTATION_DISCOVERY_TAG, MUTATION_POLICY_TAG, MappedUnpermittedCause, MutationDiscoveryId,
    MutationDiscoveryReading, MutationPermission, MutationPoint, MutationPolicy, MutationPolicyId,
    MutationSurfaceLowering, OwnerClaimMapping, PermissionRefusal, PointCatalogPosture,
    PolicyMembership, PolicyRefusal, ResolvedMutation, SelectionRefusal,
};

pub use super::interpretation::{
    EvaluationBinding, EvaluationCall, EvaluationObservation, EvaluationPair,
    EvaluationPairRefusal, EvaluationPairStanding, EvaluationPairStandingMismatch,
    InterpretedExecutionRefusal, InterpretedMutationEvidence, InterpretedTrust,
    InterpreterAvailability, MUTERPRATER_NAMESPACE, MeaningCheck, MissingTrustEvidence,
    MutationWitness, MutationWitnessRefusal, NO_MUTATION_PAIRING, NoMutationObservationRefusal,
    NoMutationParityQualification, NoMutationParityReading, NoMutationParityStanding,
    NoMutationResults, PARITY_DECLARATION_SUBSTRATE, PARITY_RENDERING_SUBSTRATE,
    ParityQualificationRefusal, ParityRefusal, ProductionBinding, ProductionCall,
    RejectedNoMutationParity,
};
pub use super::specimen::types::{
    ARTIFACT_CONTENT_TAG, ArtifactContent, ArtifactContentId, CompiledProjectionPressure,
    CompiledProjectionRefusal, CompiledSpecimenHost, CompiledSpecimenHostRefusal,
    CompiledSpecimenObservation, CompiledSpecimenObservationMismatch, CompiledSpecimenRequest,
    CompiledSpecimenRole, CompiledSpecimenStanding, SpecimenMaterializerBinding,
    SpecimenMaterializerCall, SpecimenMaterializerRefusal,
};

pub use super::rewrite::types::{
    ARTIFACT_MUTATIONS, ArtifactMutation, RewriteAdmission, RewriteCandidate, RewriteDescriptor,
    RewriteRefusal, RewriteRoster, RewriteTrust, RewriteWithheld, RosterRefusal,
};

pub use super::proposal::{
    CandidateSketch, CheckGap, ClaimPinnedGround, ClaimPinnedProposal, Demonstration,
    DischargeAdmissionReceipt, DischargeEvidence, DischargeProposalRefusal, DuplicateRefusal,
    ExplanationRefusal, FailureComparison, HumanAdmissionRefusal, InferredObligation,
    KillProposalRefusal, MutantKilledGround, MutantKilledProposal, NoComparison,
    NoComparisonReason, ObligationComparison, ObligationDischargedGround,
    ObligationDischargedProposal, ObligationLane, OracleClass, OwedClaim, OwedClaimRefusal,
    OwedDeclaration, PROPOSAL_TAG, ProofDelta, ProofDeltaRefusal, ProofRefusal, ProofShape,
    ProposalDestination, ProposalDocument, ProposalRefusal, ProposalSink, ReplayAdmissionReceipt,
    ReplayBearingProposal, SinkRefusal, StoredProposalRef, SurvivorExplanation, SynthesisRefusal,
};
