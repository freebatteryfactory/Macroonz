//! The runner's invariant nucleus: the roads that build an invocation and hand
//! its seats back.
//!
//! Declared inside `types.rs` as its own child, which is what keeps the
//! invocation's seats private: a run's hosting facts are stated once, at the
//! call that declares them, and nothing reaches in afterwards to change what a
//! report will say the run stood on.

use super::{HostClock, Invocation};
use crate::report::{InvocationProfile, TargetBinding, TrialSite};

impl HostClock {
    /// The clock, over the caller's own nanosecond reading.
    #[must_use]
    pub const fn reading(read: fn() -> u64) -> Self {
        Self(read)
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
