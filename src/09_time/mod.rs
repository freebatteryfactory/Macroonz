//! Band 09 — time: the typed temporal algebra (T1–T4), the tick, the deadline
//! split, and HLC chronology.

pub mod types;

pub use types::{
    AcceptedHlc, ChronologyAdmissionClock, ChronologyAnchor, ChronologyMerge, ChronologyProfileId,
    ChronologySummary, ClockDomainId, ClockDomainRole, ClockObservation,
    ClockObservationProvenance, ClockPolicyLimit, ClockSkewDisposition, ClockSourcePolicy,
    ConsumedBudgetEvidence, DeadlineDimension, DeadlinePolicy, DeadlinePolicyConstruction,
    DeadlinePostureView, DurationLimit, DurationLimitConstruction, HlcCoordinate,
    LiveMonotonicDeadline, ObservedWallTime, ProvenanceLimit, RecordingSite, SourceHlc,
    SpendCoordinateClaim, SpendLimit, SpendRecord, TimeDelta,
};
