//! The witness join, comparison outcome and independently retained movement axes.

use crate::report::ReplayPosture;

#[path = "type_guard.rs"]
mod guard;

/// Why the supplied current report cannot be compared on the saved witness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayJoinRefusal {
    /// The envelope carries different bytes from the retained witness.
    WitnessBytesDiffer,
    /// The current report records no admitted input.
    CurrentInputUnrecorded,
    /// The current report records another input convention.
    CurrentProfileDiffers,
    /// The current report records another specimen case.
    CurrentCaseDiffers,
}

/// The relation between one historical coordinate and its current counterpart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayCoordinate {
    /// Both coordinates agree.
    Same,
    /// Both coordinates exist and differ.
    Moved,
    /// The historical record lacks this coordinate.
    Unrecorded,
}

/// Every execution coordinate compared independently of the outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayMovement {
    trial: ReplayCoordinate,
    subject: ReplayCoordinate,
    check: ReplayCoordinate,
    profile: ReplayCoordinate,
    schema: ReplayCoordinate,
    decoder: ReplayCoordinate,
    target: ReplayCoordinate,
    toolchain: ReplayCoordinate,
    invocation: ReplayCoordinate,
}

/// The reached witness's case relative to the historically recorded original case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WitnessLineage {
    /// The original and reached cases agree under the same convention.
    OriginalCase,
    /// The reached case differs under the same convention.
    ReachedWitness,
    /// Profile or schema movement prevents that case comparison.
    ConventionMoved,
    /// The historical record has no original input coordinate.
    HistoricalInputUnrecorded,
}

/// Why the historical reproduction account cannot be verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoricalReplayRefusal {
    /// No original input standing was retained.
    InputUnrecorded,
    /// At least one participating executable has untracked standing.
    Untracked,
}

/// The historical comparison's accountability ceiling, without authenticating custody.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoricalReplayStanding {
    /// The weaker historical claimed and current earned ceiling.
    Comparable(ReplayPosture),
    /// A missing or untracked coordinate prevents historical verification.
    Unverifiable(HistoricalReplayRefusal),
}

/// Why the current attempt does not establish reproduction of the historical defect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayNonReproduction {
    /// The witness passed without the comparison standing required for a repair claim.
    PassedWithoutRepairStanding,
    /// The current refusal has another fingerprint.
    FingerprintMoved,
    /// The current binding names another semantic trial.
    TrialMoved,
    /// The current attempt did not produce a conclusion.
    DidNotConclude,
}

/// What the current attempt establishes on the joined saved witness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayOutcome {
    /// The current refusal has the historical trial and fingerprint.
    DefectReproduced,
    /// The changed subject passes under the bounded [repair comparison](super#outcome-and-movement).
    FixedOnWitness,
    /// The current report does not reproduce the historical defect.
    NotReproduced(ReplayNonReproduction),
}

/// A comparison admitted only after the witness and current report agree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayReading {
    movement: ReplayMovement,
    lineage: WitnessLineage,
    standing: HistoricalReplayStanding,
    outcome: ReplayOutcome,
}
