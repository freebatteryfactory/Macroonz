//! The runner's invariant nucleus: the roads that build an invocation, the
//! roads that build a seat's refusal, and the readers that hand their seats
//! back.
//!
//! Declared inside `types.rs` as its own child, which is what keeps the
//! invocation's seats private: a run's hosting facts are stated once, at the
//! call that declares them, and nothing reaches in afterwards to change what a
//! report will say the run stood on.

use super::{FailedTrial, Invocation, SeatFailure, SeatRefusal, Selection, SelectionPlan};
use crate::clock::HarnessClock;
use crate::descriptor::TrialTableRefusal;
use crate::report::{
    EmptySelectionReason, InvocationProfile, SelectionExpectation, TargetBinding, TrialId,
    TrialSite,
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
        }
    }

    /// The conclusion-relevant budgets, as the callable reads them and as the
    /// report records them.
    #[must_use]
    pub const fn profile(&self) -> InvocationProfile {
        self.profile
    }

    /// The target and toolchain the run stands on.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The site the reports of this run are written at.
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
    /// The expectation is not a parameter, because there is nothing to choose
    /// here: every run means to exercise something unless its caller says
    /// otherwise, and asking each caller to restate the standing expectation
    /// would be ceremony that can only be got wrong.
    #[must_use]
    pub fn of(chooses: Selection) -> Self {
        Self {
            chooses,
            expects: SelectionExpectation::AtLeastOne,
        }
    }

    /// The plan that admits a selection matching nothing, and states why.
    ///
    /// The one road to an empty-tolerant run. The reason travels with the plan
    /// into the run's own record, so a zero-work result is always readable as
    /// something a caller declared rather than something that quietly happened.
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

    /// What this plan expects the choice to match.
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

    /// The trial's semantic identity — the name this failure keeps across a
    /// refactor.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// Where the invocation that ran it was written — the rail a reader jumps
    /// on.
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
    /// The box is the arm's own, stated once here, so no caller spells the
    /// allocation that keeps every other arm of this family small.
    pub fn trial_failed(failed: FailedTrial) -> Self {
        Self::TrialFailed(Box::new(failed))
    }
}

/// Every construction refusal on the stamped road reaches a seat unchanged.
///
/// The stamped seats build their world through this crate's public
/// constructors and carry whatever those refuse into [`TrialTableRefusal`].
/// This is the one step from there to the type a test function returns, which
/// is what makes `?` the whole ceremony at a seat — and it is the ONLY step,
/// because the engine call beside it refuses nothing: a run always states a
/// report, and what that report says is the verdict road's to read.
impl From<TrialTableRefusal> for SeatRefusal {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::TableNotBuilt(refusal)
    }
}
