#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! `types.rs` declares this home's public vocabulary: the verdict chain's
//! axes, the per-mutant record and its run, the mutation target and its owner
//! mapping, the wrap lane's adapter profile and reading vocabulary, the two
//! facts a reading opens trust with, the interpreted lane's evaluation surface
//! and trust gate, the rewrite lane's descriptors, the artifact-mutation seed
//! roster, the survivor explanation and the check gap, the scope shapes and the
//! proof plan, and the whole proposal road. Every road that reaches one of its
//! private fields is its own child, `type_guard.rs`; the total maps its arms
//! are read through are `type_contract.rs`.
//!
//! The lanes are role-named pure-function modules. [`wrap`] reads a compiled
//! mutation backend's console output into the axis-composed record — under the
//! profile that states which grammar the reading stands on and what it may
//! claim — and plans its witness runs. [`interpret`] is the compile-once
//! interpreter's rapid loop: selection over an evaluation surface, the mandatory
//! no-mutation parity, and the typed availability that stands where a crippled
//! fake interpreter would — a gate that consumes the qualification of the
//! adapter and the witness rejection shown under it, never a run stripped of
//! the profile it was read through.
//! [`rewrite`] plans structural-rewrite descriptors as audit candidates, admitted
//! last. [`propose`] is the proposal road end to end — survivor to candidate,
//! candidate to demonstrated kill, opening to routed obligation, and the exit
//! where a human admits.
//!
//! # The one pair of hands
//!
//! Nothing in this home executes a semantic trial. Every run it needs is
//! [`crate::runner::run_all`] over the one complete table with a selection, and
//! every verdict it explains is a record the record vocabulary
//! ([`crate::report`]) already wrote. This home is a planner and an explainer,
//! and it grows no hands of its own.

pub mod interpret;
pub mod propose;
pub mod rewrite;
pub mod wrap;

mod type_contract;
mod types;

pub use types::{
    ARTIFACT_MUTATIONS, ActivationAxis, ActivationDisposition, ActivationEvidence, ActivationSite,
    ActiveMutant, ActiveSelection, AdapterProfile, AdapterQualification, AdmissionPatch,
    AlternativeIndex, AnnouncedRoster, ArtifactMutation, BackendVersion, BackendVersionPosture,
    BackendVersionRefusal, BaselineAxis, BaselinePrecondition, BaselineQualification,
    BudgetRefusal, CandidateSketch, CheckGap, ClaimCeiling, ClaimPinnedGround, ClaimPinnedProposal,
    CompiledPressureWitness, CoordinateRefusal, DemonstratedRejection, Demonstration, DiffPath,
    DiffPathRefusal, DischargeEvidence, DudPlant, DuplicateRefusal, EquivalenceAxis,
    EvaluationSurface, ExecutionAxis, ExplanationRefusal, FailureComparison, FamilyAttribution,
    FamilyLookup, GrammarStanding, GrammarVersion, InconclusiveCause, InferredObligation,
    IntendedRejection, InterpreterAvailability, KillProposalRefusal, KillRefusal,
    MUTATION_TARGET_TAG, MUTERPRATER_NAMESPACE, MappingPosture, MaterializationAxis,
    MissingTrustEvidence, MutantId, MutantKilledGround, MutantKilledProposal, MutationCensus,
    MutationIdentity, MutationOutcome, MutationPoint, MutationReport, MutationRun, MutationSite,
    MutationTarget, MutationVerdict, NO_MUTATION_PAIRING, NoComparison, NoComparisonReason,
    ObligationComparison, ObligationDischargedGround, ObligationDischargedProposal, ObligationLane,
    OperatorFamilyRef, OracleClass, OwedClaim, OwedClaimRefusal, OwedDeclaration, OwnerLookup,
    PARITY_DECLARATION_SUBSTRATE, PARITY_RENDERING_SUBSTRATE, PROPOSAL_TAG, ParityRefusal,
    ParityStanding, PlanRefusal, PlannedDamage, PlannedRun, PointRefusal, PressureBudget,
    PressureLane, PressureWitnessRefusal, ProofDelta, ProofDeltaRefusal, ProofPlan, ProofRefusal,
    ProofShape, ProposalDestination, ProposalDocument, ProposalRefusal, ProposalSink,
    QualificationRefusal, ReadingSource, RejectionIdentity, RewriteAdmission, RewriteCandidate,
    RewriteDescriptor, RewriteRefusal, RewriteRoster, RewriteTrust, RewriteWithheld, RosterRefusal,
    ScopeShape, ScopedInvocation, SelectionRefusal, SinkRefusal, SourceCoordinate,
    StoredProposalRef, SurfaceRefusal, SurvivalRefusal, SurvivorExplanation, SynthesisRefusal,
    UnparsedLine, WrapOutcomeWord, WrapReading, WrapRefusal, WrapStanding, WrappedBackend,
};
