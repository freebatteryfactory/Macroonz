//! Scope, budget, and inspectable pressure plans.

use crate::descriptor::SubjectRoute;
use crate::muterprater::{AlternativeId, MutationIdentity};
use crate::report::InvocationProfile;
use crate::runner::Selection;
#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// Scope, budget, and the proof plan.
// ---------------------------------------------------------------------------

/// One path a diff touched, as the caller read it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DiffPath(String);

/// Why one diff path was refused.
#[must_use = "a refusal is the reason a diff path was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiffPathRefusal {
    /// The path is empty, so it names nothing.
    EmptyPath,
}

/// What one invocation of a lane is scoped to.
///
/// Scope shapes are invocation parameters and never a second world: each narrows a run, and every report is still stated over the complete table.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeShape {
    /// One seam, entered through the subject route the run is narrowed to.
    SeamScoped {
        /// The route the seam is entered through.
        route: SubjectRoute,
    },
    /// Every row the world presents.
    RepoWide,
    /// Only what a diff touched.
    DiffScoped {
        /// The paths the diff touched, in the order the caller read them.
        touched: Vec<DiffPath>,
    },
}

/// What one scoped pressure run may spend.
///
/// The mutant bound is this home's; the per-trial budgets are the invocation profile's, borrowed rather than restated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PressureBudget {
    mutants: u32,
    invocation: InvocationProfile,
}

/// Why one pressure budget was refused.
#[must_use = "a refusal is the reason a budget was not declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetRefusal {
    /// The budget admits no mutant, so the run it bounds would press nothing.
    ZeroMutants,
}

/// One scope shape with the budget its run may spend.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopedInvocation {
    scope: ScopeShape,
    budget: PressureBudget,
}

/// Which lane one planned run belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PressureLane {
    /// Compiled mutation, wrapped around an external backend.
    CompiledMutation,
    /// The interpreted route's rapid loop.
    InterpretedMutation,
    /// Structure-aware generation with budgeted minimization.
    Fuzz,
    /// Campaigns over the fault adapters.
    Chaos,
}

/// Which damage one planned run presses.
///
/// An external backend chooses its own damage and the mutant identity already names it; an interpreted run states which admitted alternative it selects, which is what keeps two runs over one point from reading as one run stated twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlannedDamage {
    /// The backend's own damage, already named by the mutant identity.
    BackendChosen,
    /// One admitted alternative of an interpreted point.
    Alternative(AlternativeId),
}

/// One intended run: the lane, the target, the damage, what the run selects, and what it may spend.
///
/// A planned run is a value and spends nothing, which is what makes planning inspectable before a budget is committed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedRun {
    lane: PressureLane,
    target: MutationIdentity,
    damage: PlannedDamage,
    selection: Selection,
    budget: PressureBudget,
}

/// The complete statement of what one pressure pass intends to run, before any budget is spent.
///
/// Planning is a pure function and this is its image: a caller reads every intended run and its budget, and decides, before the first mutant is pressed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofPlan {
    scope: ScopedInvocation,
    runs: Vec<PlannedRun>,
}

/// Why one proof plan was refused.
///
/// Dependent checks in a declared order: the roster is read before it is weighed against the budget.
#[must_use = "a refusal is the reason a proof plan was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanRefusal {
    /// The plan states no run at all, so it presses nothing.
    NoRunPlanned,
    /// The plan states more runs than the scope's mutant budget admits.
    BudgetOverspent {
        /// How many mutants the budget admits.
        admitted: u32,
        /// How many runs the plan states.
        planned: usize,
    },
}
