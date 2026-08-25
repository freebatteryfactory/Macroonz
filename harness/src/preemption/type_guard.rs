//! Constructors and readers for the preemption vocabulary.

use super::{
    PreemptionBound, PreemptionBounds, PreemptionBoundsRefusal, PreemptionModelFailure,
    PreemptionOutcome, PreemptionReading,
};
use crate::report::ForeignText;

impl PreemptionBounds {
    /// The budget its author declared: the preemption seat, and the branch budget one execution may take.
    ///
    /// # Errors
    ///
    /// Refuses a branch budget of zero, because no execution could take a single step under it.
    pub const fn declared(
        preemptions: PreemptionBound,
        branches: u32,
    ) -> Result<Self, PreemptionBoundsRefusal> {
        if branches == 0u32 {
            return Err(PreemptionBoundsRefusal::ZeroBranches);
        }
        Ok(Self {
            preemptions,
            branches,
        })
    }

    /// How many preemptions one execution may spend.
    #[must_use]
    pub const fn preemptions(self) -> PreemptionBound {
        self.preemptions
    }

    /// How many branches one execution may take.
    #[must_use]
    pub const fn branches(self) -> u32 {
        self.branches
    }
}

impl PreemptionModelFailure {
    /// Refuse one scheduled execution with no foreign report.
    #[must_use]
    pub const fn unreported() -> Self {
        Self { report: None }
    }

    /// Refuse one scheduled execution with bounded report material.
    #[must_use]
    pub fn reported(material: &[u8]) -> Self {
        Self {
            report: Some(ForeignText::admitted(material)),
        }
    }

    /// The bounded report the model supplied, where it supplied one.
    #[must_use]
    pub const fn report(&self) -> Option<&ForeignText> {
        self.report.as_ref()
    }
}

impl PreemptionReading {
    /// One exploration's product, minted only by the explore road.
    #[must_use]
    pub(crate) const fn read(bounds: PreemptionBounds, outcome: PreemptionOutcome) -> Self {
        Self { bounds, outcome }
    }

    /// The bounds the exploration was asked to run under.
    #[must_use]
    pub const fn bounds(&self) -> PreemptionBounds {
        self.bounds
    }

    /// The strongest outcome the backend established.
    #[must_use]
    pub const fn outcome(&self) -> &PreemptionOutcome {
        &self.outcome
    }
}
