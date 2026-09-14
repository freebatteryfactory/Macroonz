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
    /// Sparse legacy claims omit the complete historical execution and fingerprint preimages.
    IncompleteLegacyRecord,
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

/// A supplied legacy coordinate's relation to current evidence, without authenticating the claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyClaimRelation {
    /// The historical member was omitted.
    Missing,
    /// The historical member explicitly contained null.
    Null,
    /// The containing profile object was omitted.
    ParentMissing,
    /// The containing profile object was null, with no leaf supplied.
    ParentNull,
    /// The supplied value is retained without assigning current identity or outcome meaning.
    Uninterpreted,
    /// The supplied claim equals the current coordinate.
    SameClaim,
    /// The supplied claim differs from the current coordinate.
    MovedClaim,
    /// Current execution earned no counterpart, such as a refusal fingerprint.
    CurrentUnavailable,
}

/// Why a sparse historical witness cannot be joined to a current report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyJoinRefusal {
    /// No witness member was recorded.
    MissingWitness,
    /// The witness member was null.
    NullWitness,
    /// The current report and admitted envelope did not join the saved bytes.
    Current(ReplayJoinRefusal),
}

/// A complete field-presence account over a joined legacy witness and current report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyReading {
    claims: std::collections::BTreeMap<crate::report::legacy::LegacyField, LegacyClaimRelation>,
}
