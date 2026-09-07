//! Outcome, lineage and historical accountability read after the witness join.

use super::{
    HistoricalReplayRefusal, HistoricalReplayStanding, ReplayCoordinate, ReplayMovement,
    ReplayNonReproduction, ReplayOutcome, WitnessLineage,
};
use crate::report::{
    ExecutionInput, Fingerprint, ReplayPosture, RunAttempt, TrialConclusion, TrialReport,
    archive::ArchivedCapsule,
};

pub(super) fn standing(
    historical: &ArchivedCapsule,
    current: &TrialReport,
) -> HistoricalReplayStanding {
    if historical.key().input().is_none() {
        return HistoricalReplayStanding::Unverifiable(HistoricalReplayRefusal::InputUnrecorded);
    }
    let ceiling = historical
        .claimed_posture()
        .meet(current.standing().replay());
    if ceiling == ReplayPosture::UnavailableBecauseUntracked {
        HistoricalReplayStanding::Unverifiable(HistoricalReplayRefusal::Untracked)
    } else {
        HistoricalReplayStanding::Comparable(ceiling)
    }
}

pub(super) fn lineage(
    historical: &ArchivedCapsule,
    input: ExecutionInput,
    movement: ReplayMovement,
) -> WitnessLineage {
    let Some(previous) = historical.key().input() else {
        return WitnessLineage::HistoricalInputUnrecorded;
    };
    if movement.profile() != ReplayCoordinate::Same || movement.schema() != ReplayCoordinate::Same {
        return WitnessLineage::ConventionMoved;
    }
    if previous.case().as_bytes() == input.case().address().as_bytes() {
        WitnessLineage::OriginalCase
    } else {
        WitnessLineage::ReachedWitness
    }
}

pub(super) fn outcome(
    historical: &ArchivedCapsule,
    current: &TrialReport,
    movement: ReplayMovement,
    standing: HistoricalReplayStanding,
) -> ReplayOutcome {
    if movement.trial() != ReplayCoordinate::Same {
        return ReplayOutcome::NotReproduced(ReplayNonReproduction::TrialMoved);
    }
    match current.attempt() {
        RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
            if Fingerprint::of(current.trial(), finding).address()
                == historical.fingerprint().address()
            {
                ReplayOutcome::DefectReproduced
            } else {
                ReplayOutcome::NotReproduced(ReplayNonReproduction::FingerprintMoved)
            }
        }
        RunAttempt::Executed(TrialConclusion::Passed) => repair(movement, standing),
        RunAttempt::SkippedWithReason(_)
        | RunAttempt::TimedOut
        | RunAttempt::InfrastructureFailed(_) => {
            ReplayOutcome::NotReproduced(ReplayNonReproduction::DidNotConclude)
        }
    }
}

fn repair(movement: ReplayMovement, standing: HistoricalReplayStanding) -> ReplayOutcome {
    let independent = [
        movement.check(),
        movement.profile(),
        movement.schema(),
        movement.decoder(),
        movement.target(),
        movement.toolchain(),
        movement.invocation(),
    ];
    if movement.subject() == ReplayCoordinate::Moved
        && independent
            .into_iter()
            .all(|coordinate| coordinate == ReplayCoordinate::Same)
        && matches!(standing, HistoricalReplayStanding::Comparable(_))
    {
        ReplayOutcome::FixedOnWitness
    } else {
        ReplayOutcome::NotReproduced(ReplayNonReproduction::PassedWithoutRepairStanding)
    }
}
