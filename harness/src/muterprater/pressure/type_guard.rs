//! The invariant nucleus of scoped pressure plans.

use super::{
    BudgetRefusal, DiffPath, DiffPathRefusal, InvocationProfile, MutationIdentity, PlanRefusal,
    PlannedDamage, PlannedRun, PressureBudget, PressureLane, ProofPlan, ScopeShape,
    ScopedInvocation, Selection,
};

impl DiffPath {
    /// One path a diff touched.
    ///
    /// # Errors
    ///
    /// Refuses an empty spelling, which names nothing.
    pub fn reported(spelling: &str) -> Result<Self, DiffPathRefusal> {
        if spelling.is_empty() {
            return Err(DiffPathRefusal::EmptyPath);
        }
        Ok(Self(spelling.to_owned()))
    }

    /// The spelling the caller read.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl PressureBudget {
    /// What one scoped run may spend.
    ///
    /// # Errors
    ///
    /// Refuses a budget admitting no mutant, because the run it bounds would press nothing.
    pub const fn declared(
        mutants: u32,
        invocation: InvocationProfile,
    ) -> Result<Self, BudgetRefusal> {
        if mutants == 0_u32 {
            return Err(BudgetRefusal::ZeroMutants);
        }
        Ok(Self {
            mutants,
            invocation,
        })
    }

    /// The greatest number of mutants the run may press.
    #[must_use]
    pub const fn mutants(self) -> u32 {
        self.mutants
    }

    /// The per-trial budgets every witness run stands under.
    #[must_use]
    pub const fn invocation(self) -> InvocationProfile {
        self.invocation
    }
}

impl ScopedInvocation {
    /// One scope shape with the budget its run may spend.
    #[must_use]
    pub fn scoped(scope: ScopeShape, budget: PressureBudget) -> Self {
        Self { scope, budget }
    }

    /// What the run is scoped to.
    #[must_use]
    pub const fn scope(&self) -> &ScopeShape {
        &self.scope
    }

    /// What it may spend.
    #[must_use]
    pub const fn budget(&self) -> PressureBudget {
        self.budget
    }
}

impl PlannedRun {
    /// One intended run.
    #[must_use]
    pub fn intended(
        lane: PressureLane,
        target: MutationIdentity,
        damage: PlannedDamage,
        selection: Selection,
        budget: PressureBudget,
    ) -> Self {
        Self {
            lane,
            target,
            damage,
            selection,
            budget,
        }
    }

    /// Which lane it belongs to.
    #[must_use]
    pub const fn lane(&self) -> PressureLane {
        self.lane
    }

    /// What it presses.
    #[must_use]
    pub const fn target(&self) -> MutationIdentity {
        self.target
    }

    /// Which damage of that target it presses.
    #[must_use]
    pub const fn damage(&self) -> PlannedDamage {
        self.damage
    }

    /// What it selects from the complete world.
    #[must_use]
    pub const fn selection(&self) -> &Selection {
        &self.selection
    }

    /// What it may spend.
    #[must_use]
    pub const fn budget(&self) -> PressureBudget {
        self.budget
    }
}

impl ProofPlan {
    /// The complete statement of an intended pressure pass.
    ///
    /// # Errors
    ///
    /// Refuses a plan stating no run, then a plan stating more runs than the scope's mutant budget admits — so a budget is weighed before it is spent rather than discovered spent.
    pub fn planned(scope: ScopedInvocation, runs: Vec<PlannedRun>) -> Result<Self, PlanRefusal> {
        if runs.is_empty() {
            return Err(PlanRefusal::NoRunPlanned);
        }
        let admitted = scope.budget().mutants();
        let planned = runs.len();
        let within = u32::try_from(planned).is_ok_and(|count| count <= admitted);
        if !within {
            return Err(PlanRefusal::BudgetOverspent { admitted, planned });
        }
        Ok(Self { scope, runs })
    }

    /// The scope and budget the pass runs under.
    #[must_use]
    pub const fn scope(&self) -> &ScopedInvocation {
        &self.scope
    }

    /// Every intended run, in planned order.
    #[must_use]
    pub fn runs(&self) -> &[PlannedRun] {
        &self.runs
    }
}
