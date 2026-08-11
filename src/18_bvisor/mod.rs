//! Band 18 — bvisor: the boundary supervisor — physical admission, the
//! Attempt lifecycle, reservations, observations, witnesses, containment.

pub mod types;

pub use types::{
    ADMISSION_DEPENDENCY_ORDER, ADMISSION_INPUTS, AdmissionOutcome, AdmittedAttempt,
    AttemptAdmission, AttemptAdmissionIssue, AttemptId, AttemptReport, AttemptState,
    AuthenticitySubject, AuthenticityWitness, AvailabilitySubject, BVISOR_IS_NOT, BindingSubject,
    BudgetWitness, CANCELLATION_FACTS, CapabilityWitness, CarrierWitness, CompatibilitySubject,
    ConsumedVerdictSubject, ContainmentProfile, DeclaredEvidenceRequirement, DerivedFloorBreach,
    DurabilityWitness, EffectProgressWitness, GenerationAxis, GenerationPosture, InteractionShape,
    LiveSuspendedAttempt, MeetFailure, NarrowingInput, PAIRWISE_NON_SUBSTITUTION,
    PHYSICAL_OBSERVATION_KINDS, PORT_REQUEST_VALIDATION, PhysicalEstimate, PlannedInvocation,
    PortRequest, PortRequestId, PortResponseObservation, ProcessExitObservation,
    RequestedReservationSemantics, RequiredEvidencePosture, ReservationEvidence,
    ReservationObservation, ResourceReservation, RunningAttempt, StorageDurabilityObservation,
    TerminalAttempt,
};
