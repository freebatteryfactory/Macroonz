//! One execution's record, and the complete-table census a run is stated over.

use crate::clock::MeasurementReading;
use crate::descriptor::{ClaimRef, TablePosture};
use crate::report::{
    ExecutionKey, Exercise, HostTrialRecord, InvocationProfile, NotSelectedReason, OutcomeClass,
    ReplayPosture, RowRevisionId, RunAttempt, RunReport, SelectionDisposition,
    SelectionExpectation, SelectionOutcome, TrialAccounting, TrialConclusion, TrialId, TrialReport,
    TrialRunStanding, TrialSite,
};

impl HostTrialRecord {
    /// One host's typed input about one selected trial.
    #[must_use]
    pub fn recorded(trial: TrialId, attempt: RunAttempt, measurement: MeasurementReading) -> Self {
        Self {
            trial,
            attempt,
            measurement,
        }
    }

    /// The semantic trial the host says this input belongs to.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// What the host says became of the attempt.
    #[must_use]
    pub const fn attempt(&self) -> &RunAttempt {
        &self.attempt
    }

    /// The wall-measurement posture the host recorded.
    pub const fn measurement(&self) -> MeasurementReading {
        self.measurement
    }

    /// The three host-authored seats, for the runner join that admits them.
    pub(crate) fn into_parts(self) -> (TrialId, RunAttempt, MeasurementReading) {
        (self.trial, self.attempt, self.measurement)
    }
}

impl TrialRunStanding {
    /// The standing the runner derived from one binding and one invocation.
    #[must_use]
    pub(crate) fn derived(key: ExecutionKey, replay: ReplayPosture) -> Self {
        Self { key, replay }
    }

    /// The exact execution key this trial ran under.
    #[must_use]
    pub const fn key(&self) -> &ExecutionKey {
        &self.key
    }

    /// The replay ceiling derived from the attachment's revision posture meet.
    #[must_use]
    pub const fn replay(&self) -> ReplayPosture {
        self.replay
    }
}

impl TrialReport {
    /// One execution's record.
    #[must_use]
    pub(crate) fn recorded(
        standing: TrialRunStanding,
        site: TrialSite,
        attempt: RunAttempt,
        measurement: MeasurementReading,
    ) -> Self {
        Self {
            standing,
            site,
            attempt,
            measurement,
        }
    }

    /// The trial's semantic identity.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.standing.key().trial()
    }

    /// The exact execution standing this report was admitted under.
    #[must_use]
    pub const fn standing(&self) -> &TrialRunStanding {
        &self.standing
    }

    /// Where the trial is written.
    #[must_use]
    pub const fn site(&self) -> TrialSite {
        self.site
    }

    /// What became of the attempt.
    #[must_use]
    pub const fn attempt(&self) -> &RunAttempt {
        &self.attempt
    }

    /// The wall-measurement posture recorded around the attempt.
    pub const fn measurement(&self) -> MeasurementReading {
        self.measurement
    }
}

impl SelectionDisposition {
    /// Selected, carrying its execution record.
    #[must_use]
    pub(crate) fn selected(report: TrialReport) -> Self {
        Self::Selected(Box::new(report))
    }

    /// Not selected, for a stated reason.
    #[must_use]
    pub(crate) const fn not_selected(trial: TrialId, reason: NotSelectedReason) -> Self {
        Self::NotSelected { trial, reason }
    }

    /// The semantic identity of this census seat.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        match self {
            Self::Selected(report) => report.trial(),
            Self::NotSelected { trial, reason: _ } => *trial,
        }
    }

    /// The execution record, where the invocation selected the trial.
    #[must_use]
    pub fn report(&self) -> Option<&TrialReport> {
        match self {
            Self::Selected(report) => Some(report.as_ref()),
            Self::NotSelected {
                trial: _,
                reason: _,
            } => None,
        }
    }

    /// Whether this row of the denominator was actually exercised.
    #[must_use]
    pub fn exercise(&self) -> Exercise {
        match self {
            Self::Selected(report) => match report.attempt() {
                RunAttempt::Executed(_) => Exercise::Exercised,
                RunAttempt::SkippedWithReason(_)
                | RunAttempt::TimedOut(_)
                | RunAttempt::InfrastructureFailed(_) => Exercise::Unexercised,
            },
            Self::NotSelected {
                trial: _,
                reason: _,
            } => Exercise::Unexercised,
        }
    }

    /// The normalized outcome, for a comparison to read.
    #[must_use]
    pub fn outcome(&self) -> OutcomeClass {
        match self {
            Self::Selected(report) => match report.attempt() {
                RunAttempt::Executed(TrialConclusion::Passed) => OutcomeClass::Passed,
                RunAttempt::Executed(TrialConclusion::Refused(finding)) => {
                    OutcomeClass::Refused(finding.class())
                }
                RunAttempt::SkippedWithReason(reason) => OutcomeClass::Skipped(*reason),
                RunAttempt::TimedOut(_) => OutcomeClass::TimedOut,
                RunAttempt::InfrastructureFailed(fault) => {
                    OutcomeClass::InfrastructureFailed(*fault)
                }
            },
            Self::NotSelected { trial: _, reason } => OutcomeClass::NotSelected(*reason),
        }
    }
}

impl TrialAccounting {
    /// One row of the denominator, and what this invocation did about it.
    #[must_use]
    pub(crate) fn recorded(
        row: RowRevisionId,
        claim: ClaimRef,
        disposition: SelectionDisposition,
    ) -> Self {
        Self {
            row,
            claim,
            disposition,
        }
    }

    /// The trial's semantic identity.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.disposition.trial()
    }

    /// The authored row's revision identity.
    #[must_use]
    pub const fn row(&self) -> RowRevisionId {
        self.row
    }

    /// The claim the row serves.
    #[must_use]
    pub const fn claim(&self) -> ClaimRef {
        self.claim
    }

    /// What the invocation did about it.
    #[must_use]
    pub const fn disposition(&self) -> &SelectionDisposition {
        &self.disposition
    }
}

impl SelectionOutcome {
    /// Read one run's selection against what the run expected of it.
    ///
    /// A total map over two facts the run already holds — how many rows the selection named, and what the caller declared beforehand — so the same pair always reads the same way.
    #[must_use]
    pub const fn read(expectation: SelectionExpectation, selected: usize) -> Self {
        if selected > 0_usize {
            return Self::Satisfied;
        }
        match expectation {
            SelectionExpectation::AtLeastOne => Self::UnsatisfiedByEmptySelection,
            SelectionExpectation::AllowEmpty(reason) => Self::EmptyAsStated(reason),
        }
    }
}

impl RunReport {
    /// One run's complete-table accounting.
    ///
    /// The census arrives complete, one entry per row of the table the run stood over, because a report that dropped its unselected rows would state a smaller world than the one it ran in.
    /// The selection outcome arrives already read, because what a run expected of its selection is the engine's parameter and a census cannot state an expectation.
    #[must_use]
    pub(crate) fn recorded(
        census: Vec<TrialAccounting>,
        posture: TablePosture,
        selection: SelectionOutcome,
        invocation: InvocationProfile,
    ) -> Self {
        Self {
            census,
            posture,
            selection,
            invocation,
        }
    }

    /// What this run's selection matched, read against what it expected.
    #[must_use]
    pub const fn selection(&self) -> SelectionOutcome {
        self.selection
    }

    /// Every row of the denominator, with its disposition.
    #[must_use]
    pub fn census(&self) -> &[TrialAccounting] {
        &self.census
    }

    /// How many rows the run was stated over.
    #[must_use]
    pub fn denominator(&self) -> usize {
        self.census.len()
    }

    /// Which table the run stood over.
    #[must_use]
    pub const fn posture(&self) -> TablePosture {
        self.posture
    }

    /// The invocation profile the run ran under.
    #[must_use]
    pub const fn invocation(&self) -> InvocationProfile {
        self.invocation
    }
}
