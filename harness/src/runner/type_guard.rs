//! The runner's nucleus: the roads that build an invocation, a plan, and a seat's refusal, and the readers that hand their seats back.
//!
//! Declared inside `types.rs` as its own child, which is what keeps the private seats private.
//! A run's hosting facts are stated once, at the call that declares them.

use super::{
    FailedTrial, Invocation, ReplayedTrial, SeatFailure, SeatRefusal, Selection, SelectionPlan,
};
use crate::clock::HarnessClock;
use crate::descriptor::TrialTableRefusal;
use crate::identity::ContentAddress;
use crate::input::BoundInput;
use crate::input::InputEnvelope;
use crate::report::TrialReport;
use crate::report::archive::ArchivedCapsule;
use crate::report::replay::{ReplayJoinRefusal, ReplayReading, compare};
use crate::report::{
    EmptySelectionReason, ExecutionInput, InvocationProfile, SelectionExpectation, SkipReason,
    TargetBinding, TrialId, TrialSite,
};

impl Invocation {
    /// The invocation, over the facts a caller declares for one run.
    #[must_use]
    pub fn declared(
        profile: InvocationProfile,
        target: TargetBinding,
        site: TrialSite,
        clock: HarnessClock,
    ) -> Self {
        Self {
            profile,
            target,
            site,
            clock,
            input: (),
            input_standing: None,
            input_bytes: 0,
        }
    }

    /// The declared run facts joined to a decoder-admitted typed specimen.
    #[must_use]
    pub fn with_input<Input>(self, input: BoundInput<Input>) -> Invocation<BoundInput<Input>> {
        Invocation {
            profile: self.profile,
            target: self.target,
            site: self.site,
            clock: self.clock,
            input_standing: Some(ExecutionInput::of(&input)),
            input_bytes: input.envelope().payload().len(),
            input,
        }
    }
}

impl ReplayedTrial {
    /// The earned current report and admitted witness joined to their historical source.
    pub(in crate::runner) fn earned(
        historical: &ArchivedCapsule,
        report: TrialReport,
        witness: InputEnvelope,
    ) -> Self {
        Self {
            historical: historical.address(),
            comparison: compare(historical, &report, &witness),
            witness,
            report,
        }
    }

    /// The historical envelope this execution read.
    #[must_use]
    pub const fn historical(&self) -> ContentAddress {
        self.historical
    }

    /// The exact witness under its independently supplied current convention.
    #[must_use]
    pub const fn witness(&self) -> &InputEnvelope {
        &self.witness
    }

    /// The complete report earned by current execution.
    #[must_use]
    pub const fn report(&self) -> &TrialReport {
        &self.report
    }

    /// The report owner's comparison over the actual admitted witness.
    pub const fn comparison(&self) -> &Result<ReplayReading, ReplayJoinRefusal> {
        &self.comparison
    }
}

impl<Input> Invocation<Input> {
    /// The admitted input owned by this invocation, or its explicit unit value.
    #[must_use]
    pub const fn input(&self) -> &Input {
        &self.input
    }

    /// The input standing derived when the admitted specimen entered this invocation.
    #[must_use]
    pub const fn input_standing(&self) -> Option<ExecutionInput> {
        self.input_standing
    }

    /// Whether the one admitted specimen exceeds a declared execution budget.
    pub(in crate::runner) fn input_budget_refusal(&self) -> Option<SkipReason> {
        self.input_standing?;
        let bytes = u64::try_from(self.input_bytes).ok();
        if self.profile.cases().cases() == 0
            || bytes.is_none_or(|count| count > self.profile.bytes().bytes())
        {
            Some(SkipReason::BudgetExhausted)
        } else {
            None
        }
    }

    /// The conclusion-relevant budgets, as the callable reads them and the report records them.
    #[must_use]
    pub const fn profile(&self) -> InvocationProfile {
        self.profile
    }

    /// The target and toolchain this run stands on.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The site this run's reports are written at.
    #[must_use]
    pub const fn site(&self) -> TrialSite {
        self.site
    }

    /// The caller's clock.
    #[must_use]
    pub const fn clock(&self) -> HarnessClock {
        self.clock
    }
}

impl SelectionPlan {
    /// The ordinary plan: this selection, expected to name at least one row.
    ///
    /// The expectation is not a parameter because there is nothing to choose.
    /// Every run means to exercise something unless its caller says otherwise, and restating that at each call would be ceremony that can only be got wrong.
    #[must_use]
    pub fn of(chooses: Selection) -> Self {
        Self {
            chooses,
            expects: SelectionExpectation::AtLeastOne,
        }
    }

    /// The plan that admits a selection matching nothing, and states why.
    ///
    /// The reason travels with the plan into the run's own record, so a zero-work result always reads as something a caller declared rather than something that quietly happened.
    #[must_use]
    pub fn allowing_empty(chooses: Selection, reason: EmptySelectionReason) -> Self {
        Self {
            chooses,
            expects: SelectionExpectation::AllowEmpty(reason),
        }
    }

    /// What this plan chooses from the complete world.
    #[must_use]
    pub const fn chooses(&self) -> &Selection {
        &self.chooses
    }

    /// What this plan expects that choice to match.
    #[must_use]
    pub const fn expects(&self) -> SelectionExpectation {
        self.expects
    }
}

impl FailedTrial {
    /// One selected trial that did not conclude lawfully.
    #[must_use]
    pub fn recorded(trial: TrialId, site: TrialSite, failure: SeatFailure) -> Self {
        Self {
            trial,
            site,
            failure,
        }
    }

    /// The trial's semantic identity.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// Where the invocation that ran it was written.
    #[must_use]
    pub const fn site(&self) -> TrialSite {
        self.site
    }

    /// What the trial did instead of concluding lawfully.
    #[must_use]
    pub const fn failure(&self) -> &SeatFailure {
        &self.failure
    }
}

impl SeatRefusal {
    /// One trial's failure, as the refusal a named lens answers with.
    ///
    /// The box is stated here once, so no caller spells the allocation that keeps every other arm of this family small.
    pub fn trial_failed(failed: FailedTrial) -> Self {
        Self::TrialFailed(Box::new(failed))
    }
}

/// Every construction refusal on the stamped road reaches a seat unchanged.
///
/// This is the one step from a table that could not be built to the type a test function returns, and it is the only step: the engine call beside it refuses nothing, because a run always states a report.
impl From<TrialTableRefusal> for SeatRefusal {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::TableNotBuilt(refusal)
    }
}
