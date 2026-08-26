#![doc = include_str!("README.md")]

mod type_contract;
mod types;

pub use types::{
    ActivationAxis, ActivationDisposition, ActivationEvidence, BaselineAxis, BaselinePrecondition,
    BaselineQualification, CoordinateRefusal, DemonstratedRejection, DudPlant, EquivalenceAxis,
    ExecutionAxis, FamilyAttribution, InconclusiveCause, IntendedRejection, KillRefusal,
    MUTATION_TARGET_TAG, MappingPosture, MaterializationAxis, MutantId, MutationCensus,
    MutationIdentity, MutationOutcome, MutationReport, MutationRun, MutationSite, MutationTarget,
    MutationVerdict, OperatorFamilyRef, RejectionIdentity, SourceCoordinate,
};
