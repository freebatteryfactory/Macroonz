//! Constructors and readers for the preemption vocabulary.

use super::{
    PreemptionBound, PreemptionBounds, PreemptionBoundsRefusal, PreemptionReading,
    PreemptionVerdict,
};

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

impl PreemptionReading {
    /// One exploration's product, minted only by the explore road.
    #[must_use]
    pub(crate) const fn read(bounds: PreemptionBounds, verdict: PreemptionVerdict) -> Self {
        Self { bounds, verdict }
    }

    /// The bounds the exploration ran under.
    #[must_use]
    pub const fn bounds(&self) -> PreemptionBounds {
        self.bounds
    }

    /// What the bounded exploration established.
    #[must_use]
    pub const fn verdict(&self) -> &PreemptionVerdict {
        &self.verdict
    }
}
