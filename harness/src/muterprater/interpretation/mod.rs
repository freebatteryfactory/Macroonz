#![doc = include_str!("README.md")]

pub mod interpret;
mod types;

pub use types::{
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
