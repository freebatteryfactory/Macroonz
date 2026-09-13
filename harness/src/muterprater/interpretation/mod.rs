#![doc = include_str!("README.md")]

pub mod interpret;
pub mod archive;
mod qualify;
mod observe;
mod correspond;
mod judge;
mod types;
pub(super) use observe::selection_for;

pub use types::{
    EvaluationBinding, EvaluationCall, EvaluationObservation, EvaluationPair,
    EvaluationPairRefusal, EvaluationPairStanding, EvaluationPairStandingMismatch,
    InterpretedExecutionRefusal, InterpretedMutationEvidence, InterpretedTrust,
    InterpreterAvailability, MUTERPRATER_NAMESPACE, MeaningCheck, MissingTrustEvidence,
    MutationAssessment, MutationObservation, MutationObservationRefusal,
    MutationQualificationRefusal, MutationWitness, MutationWitnessObservationRefusal,
    MutationWitnessQualificationRefusal, MutationWitnessReading, MutationWitnessRefusal,
    NO_MUTATION_PAIRING, NoMutationObservationRefusal, NoMutationParityQualification,
    NoMutationParityReading, NoMutationParityStanding, NoMutationResults,
    PARITY_DECLARATION_SUBSTRATE, PARITY_RENDERING_SUBSTRATE, ParityQualificationRefusal,
    ParityRefusal, ProductionBinding, ProductionCall, QualifiedMutation, RejectedMutationWitness,
    RejectedNoMutationParity,
};
