//! The values the two readings hand back: the difference between two reports, and the exercise counts over one.

use crate::descriptor::ClaimRef;
use crate::report::{
    CensusDelta, CensusDirection, ClaimCoverage, ClaimExercise, ConclusionFlip, OutcomeClass,
    ReportDiff, RowRevisionChange, RowRevisionId, TrialId,
};
use core::cmp::Ordering;

impl CensusDelta {
    /// How the denominator moved between two runs.
    #[must_use]
    pub(in crate::report) fn between(before: usize, after: usize) -> Self {
        let direction = match after.cmp(&before) {
            Ordering::Greater => CensusDirection::Grew,
            Ordering::Equal => CensusDirection::Unchanged,
            Ordering::Less => CensusDirection::Shrank,
        };
        Self {
            before,
            after,
            direction,
        }
    }

    /// The baseline's denominator.
    #[must_use]
    pub const fn before(self) -> usize {
        self.before
    }

    /// The current report's denominator.
    #[must_use]
    pub const fn after(self) -> usize {
        self.after
    }

    /// Which way it moved.
    #[must_use]
    pub const fn direction(self) -> CensusDirection {
        self.direction
    }
}

impl RowRevisionChange {
    /// One trial whose authored row was edited between the two runs.
    #[must_use]
    pub(in crate::report) const fn between(
        trial: TrialId,
        before: RowRevisionId,
        after: RowRevisionId,
    ) -> Self {
        Self {
            trial,
            before,
            after,
        }
    }

    /// The trial.
    #[must_use]
    pub const fn trial(self) -> TrialId {
        self.trial
    }

    /// The row revision the baseline recorded.
    #[must_use]
    pub const fn before(self) -> RowRevisionId {
        self.before
    }

    /// The row revision the current report records.
    #[must_use]
    pub const fn after(self) -> RowRevisionId {
        self.after
    }
}

impl ConclusionFlip {
    /// One trial whose outcome differs between the two runs.
    #[must_use]
    pub(in crate::report) const fn between(
        trial: TrialId,
        before: OutcomeClass,
        after: OutcomeClass,
    ) -> Self {
        Self {
            trial,
            before,
            after,
        }
    }

    /// The trial.
    #[must_use]
    pub const fn trial(self) -> TrialId {
        self.trial
    }

    /// What the baseline recorded.
    #[must_use]
    pub const fn before(self) -> OutcomeClass {
        self.before
    }

    /// What the current report records.
    #[must_use]
    pub const fn after(self) -> OutcomeClass {
        self.after
    }
}

impl ReportDiff {
    /// The difference between two reports.
    #[must_use]
    pub(in crate::report) fn stated(
        added: Vec<TrialId>,
        removed: Vec<TrialId>,
        revised: Vec<RowRevisionChange>,
        flips: Vec<ConclusionFlip>,
        census: CensusDelta,
    ) -> Self {
        Self {
            added,
            removed,
            revised,
            flips,
            census,
        }
    }

    /// Trials the current report has and the baseline did not.
    #[must_use]
    pub fn added(&self) -> &[TrialId] {
        &self.added
    }

    /// Trials the baseline had and the current report does not.
    #[must_use]
    pub fn removed(&self) -> &[TrialId] {
        &self.removed
    }

    /// Trials in both runs whose authored row was edited.
    #[must_use]
    pub fn revised(&self) -> &[RowRevisionChange] {
        &self.revised
    }

    /// Trials in both runs whose outcome differs.
    #[must_use]
    pub fn flips(&self) -> &[ConclusionFlip] {
        &self.flips
    }

    /// How the denominator moved.
    #[must_use]
    pub const fn census(&self) -> CensusDelta {
        self.census
    }
}

impl ClaimExercise {
    /// One claim's counts over the denominator.
    #[must_use]
    pub(in crate::report) const fn counted(
        claim: ClaimRef,
        exercised: usize,
        unexercised: usize,
    ) -> Self {
        Self {
            claim,
            exercised,
            unexercised,
        }
    }

    /// The claim.
    #[must_use]
    pub const fn claim(self) -> ClaimRef {
        self.claim
    }

    /// How many of the claim's rows executed.
    #[must_use]
    pub const fn exercised(self) -> usize {
        self.exercised
    }

    /// How many of the claim's rows did not.
    #[must_use]
    pub const fn unexercised(self) -> usize {
        self.unexercised
    }

    /// How many rows the claim owns in the denominator.
    #[must_use]
    pub const fn denominator(self) -> usize {
        self.exercised.saturating_add(self.unexercised)
    }
}

impl ClaimCoverage {
    /// The reading, one entry per claim the denominator names.
    #[must_use]
    pub(in crate::report) fn read(entries: Vec<ClaimExercise>) -> Self {
        Self { entries }
    }

    /// Every claim the denominator names, with its counts.
    #[must_use]
    pub fn entries(&self) -> &[ClaimExercise] {
        &self.entries
    }

    /// One claim's reading, or a zero reading when the denominator names no such claim.
    #[must_use]
    pub fn exercise_or_zero(&self, claim: ClaimRef) -> ClaimExercise {
        self.entries
            .iter()
            .copied()
            .find(|entry| entry.claim() == claim)
            .unwrap_or_else(|| ClaimExercise::counted(claim, 0usize, 0usize))
    }
}
