//! The values the two readings hand back: the difference between two reports, and the exercise counts over one.

use crate::descriptor::ClaimRef;
use crate::report::{
    CensusDelta, CensusDirection, ClaimCoverage, ClaimExercise, ConclusionFlip,
    ExecutionRevisionChange, ExecutionRevisions, InvocationProfile, InvocationProfileChange,
    OutcomeClass, ReportDiff, ReportExecutionDiff, ReportPopulationDiff, RowRevisionChange,
    RowRevisionId, TargetBinding, TargetBindingChange, TrialId,
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

crate::report::implement_copy_change_pair! {
    RowRevisionChange {
        context { trial: TrialId => "The trial.", }
        value: RowRevisionId,
        construction: "One trial whose authored row was edited between the two runs.",
        before: "The row revision the baseline recorded.",
        after: "The row revision the current report records.",
    }
}

crate::report::implement_copy_change_pair! {
    ExecutionRevisionChange {
        context { trial: TrialId => "The trial.", }
        value: ExecutionRevisions,
        construction: "One trial whose subject or check revision standing moved between the two runs.",
        before: "The baseline's subject and check revisions.",
        after: "The current report's subject and check revisions.",
    }
}

crate::report::implement_copy_change_pair! {
    InvocationProfileChange {
        context {}
        value: InvocationProfile,
        construction: "One changed conclusion-relevant invocation profile.",
        before: "The baseline's invocation profile.",
        after: "The current report's invocation profile.",
    }
}

crate::report::implement_borrowed_change_pair! {
    TargetBindingChange {
        context {}
        value: TargetBinding,
        construction: "One changed target and toolchain pair.",
        before: "The baseline's target and toolchain.",
        after: "The current report's target and toolchain.",
    }
}

crate::report::implement_copy_change_pair! {
    ConclusionFlip {
        context { trial: TrialId => "The trial.", }
        value: OutcomeClass,
        construction: "One trial whose outcome differs between the two runs.",
        before: "What the baseline recorded.",
        after: "What the current report records.",
    }
}

impl ReportPopulationDiff {
    /// The table-population half of a report difference.
    #[must_use]
    pub(in crate::report) fn stated(
        added: Vec<TrialId>,
        removed: Vec<TrialId>,
        revised: Vec<RowRevisionChange>,
        census: CensusDelta,
    ) -> Self {
        Self {
            added,
            removed,
            revised,
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

    /// How the denominator moved.
    #[must_use]
    pub const fn census(&self) -> CensusDelta {
        self.census
    }
}

impl ReportExecutionDiff {
    /// The execution-standing half of a report difference.
    #[must_use]
    pub(in crate::report) fn stated(
        revisions: Vec<ExecutionRevisionChange>,
        flips: Vec<ConclusionFlip>,
        invocation: Option<InvocationProfileChange>,
        target: Option<TargetBindingChange>,
    ) -> Self {
        Self {
            revisions,
            flips,
            invocation,
            target: target.map(Box::new),
        }
    }

    /// Trials in both runs whose subject or check revision standing moved.
    #[must_use]
    pub fn revisions(&self) -> &[ExecutionRevisionChange] {
        &self.revisions
    }

    /// Trials in both runs whose outcome differs.
    #[must_use]
    pub fn flips(&self) -> &[ConclusionFlip] {
        &self.flips
    }

    /// How the invocation's case, byte, or time budget moved, where any did.
    #[must_use]
    pub const fn invocation(&self) -> Option<InvocationProfileChange> {
        self.invocation
    }

    /// How the target triple or toolchain moved, where either did.
    #[must_use]
    pub fn target(&self) -> Option<&TargetBindingChange> {
        self.target.as_deref()
    }
}

impl ReportDiff {
    /// The complete declared population and execution-standing comparison reading between two reports.
    #[must_use]
    pub(in crate::report) const fn stated(
        population: ReportPopulationDiff,
        execution: ReportExecutionDiff,
    ) -> Self {
        Self {
            population,
            execution,
        }
    }

    /// The table-population difference.
    #[must_use]
    pub const fn population(&self) -> &ReportPopulationDiff {
        &self.population
    }

    /// The execution-standing difference.
    #[must_use]
    pub const fn execution(&self) -> &ReportExecutionDiff {
        &self.execution
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
