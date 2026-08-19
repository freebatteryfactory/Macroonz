//! The runner's invariant nucleus: the roads that build an invocation, the
//! roads that build a seat's refusal, and the readers that hand their seats
//! back.
//!
//! Declared inside `types.rs` as its own child, which is what keeps the
//! invocation's seats private: a run's hosting facts are stated once, at the
//! call that declares them, and nothing reaches in afterwards to change what a
//! report will say the run stood on.

use super::{FailedTrial, HostClock, Invocation, SeatFailure, SeatRefusal};
use crate::descriptor::{EncodeRefusal, TrialTableRefusal};
use crate::report::{InvocationProfile, TargetBinding, TrialId, TrialSite};

/// The reading that does not move, for a caller with no measurement to offer.
const fn unmoving() -> u64 {
    0
}

impl HostClock {
    /// The clock, over the caller's own nanosecond reading.
    #[must_use]
    pub const fn reading(read: fn() -> u64) -> Self {
        Self(read)
    }

    /// The clock a caller with no measurement to offer declares.
    ///
    /// # Nonclaims
    ///
    /// It is not a measurement and never becomes one. The reading does not
    /// move, so every [`RecordedDuration`](crate::report::RecordedDuration)
    /// taken over it reads zero — and zero here means NO MEASUREMENT WAS TAKEN,
    /// never an execution that took no time. A rendering that reads it as speed
    /// is reading a number nobody produced.
    #[must_use]
    pub const fn unmeasured() -> Self {
        Self(unmoving)
    }

    /// One reading, in nanoseconds on the caller's own origin.
    #[must_use]
    pub fn nanoseconds(self) -> u64 {
        (self.0)()
    }
}

impl Invocation {
    /// The invocation, over the facts a caller declares for one run.
    #[must_use]
    pub fn declared(
        profile: InvocationProfile,
        target: TargetBinding,
        site: TrialSite,
        clock: HostClock,
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
    pub const fn clock(&self) -> HostClock {
        self.clock
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
    #[must_use]
    pub fn trial_failed(failed: FailedTrial) -> Self {
        Self::TrialFailed(Box::new(failed))
    }
}

/// Every construction refusal on the stamped road reaches a seat unchanged.
///
/// The stamped seats build their world through this crate's public
/// constructors and carry whatever those refuse into [`TrialTableRefusal`].
/// This is the one step from there to the type a test function returns, which
/// is what makes `?` the whole ceremony at a seat.
impl From<TrialTableRefusal> for SeatRefusal {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::TableNotBuilt(refusal)
    }
}

/// A row whose canonical bytes could not be written reaches a seat unchanged.
///
/// The second step of the same law: the engine carries the descriptor home's own
/// encoding refusal, and this is the one road from there to the type a test
/// function returns, so a stamped seat spells `?` over the engine call and
/// nothing else.
impl From<EncodeRefusal> for SeatRefusal {
    fn from(refusal: EncodeRefusal) -> Self {
        Self::RowNotEncoded(refusal)
    }
}
