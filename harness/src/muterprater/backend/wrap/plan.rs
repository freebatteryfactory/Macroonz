//! The plan role: the selection one mutant's witness run is executed under, and the pass planned over every target a reading recovered.

use crate::muterprater::types::{
    MutationTarget, PlanRefusal, PlannedDamage, PlannedRun, PressureLane, ProofPlan,
    ScopedInvocation,
};
use crate::runner::Selection;
use std::collections::BTreeSet;

/// The selection one mutant's witness run is executed under.
///
/// A mapped target names the claim that owns its site, and the rows serving that claim are the ones worth running.
/// An unmapped target widens to the whole world — the conservative selection, because a narrower one would rest on a claim nobody established.
/// A selection narrows a run and never the denominator: the report a narrowed run writes still stands over every row of the complete table.
#[must_use]
pub fn mutant_scoped(target: &MutationTarget) -> Selection {
    match target.owning_claim() {
        Some(claim) => Selection::ByClaim(BTreeSet::from([claim])),
        None => Selection::All,
    }
}

/// Plan one compiled-mutation pass over the targets a reading recovered.
///
/// A pure function of its arguments that spends nothing: the plan lists every intended run with the selection it would use and the budget it would spend, so a caller reads the whole pass before the first mutant is pressed.
///
/// # Errors
///
/// Refuses a pass with no target, then one stating more runs than the scope's mutant budget admits.
pub fn plan_pass(
    targets: &[MutationTarget],
    scope: ScopedInvocation,
) -> Result<ProofPlan, PlanRefusal> {
    let budget = scope.budget();
    let runs: Vec<PlannedRun> = targets
        .iter()
        .map(|target| {
            PlannedRun::intended(
                PressureLane::CompiledMutation,
                target.identity(),
                PlannedDamage::BackendChosen,
                mutant_scoped(target),
                budget,
            )
        })
        .collect();
    ProofPlan::planned(scope, runs)
}
