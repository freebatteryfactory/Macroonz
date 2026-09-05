//! Compiled-backend plans retain the complete target roster while narrowing only explicitly mapped witness selections.

use super::support::{MutationRoadFailure, claim};
use macroonz_harness::muterprater::wrap::{mutant_scoped, plan_pass, read_output};
use macroonz_harness::muterprater::{
    BackendVersionPosture, PlanRefusal, PlannedDamage, PressureBudget, PressureLane, ScopeShape,
    ScopedInvocation,
};
use macroonz_harness::report::{ByteBudget, CaseBudget, InvocationProfile, TimeBudget};
use macroonz_harness::runner::Selection;
use std::collections::BTreeSet;

/// Synthetic backend text supplies two independently named targets, not a claim of a real mutation execution.
#[test]
fn compiled_plans_preserve_mapped_and_unmapped_targets_and_refuse_incomplete_budgets()
-> Result<(), MutationRoadFailure> {
    let reading = read_output(
        "Found 2 mutants\nok Unmutated baseline\ncaught mapped.rs:1:1: replace true with false\nmissed unknown.rs:2:1: replace false with true\n",
        BackendVersionPosture::Unstated,
        |coordinate| {
            (coordinate.file() == "mapped.rs")
                .then(claim)
                .and_then(Result::ok)
        },
        |_coordinate, _damage| None,
    )?;
    let [mapped, unmapped] = reading.run().reports() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    let selected = Selection::ByClaim(BTreeSet::from([claim()?]));
    assert_eq!(mutant_scoped(mapped.target()), selected);
    assert_eq!(mutant_scoped(unmapped.target()), Selection::All);
    let targets = [mapped.target().clone(), unmapped.target().clone()];
    let profile = InvocationProfile::declared(
        CaseBudget::declared(1),
        ByteBudget::declared(64),
        TimeBudget::declared(1_000_000_000),
    );
    let budget = PressureBudget::declared(2, profile)
        .map_err(|_| MutationRoadFailure::MissingAlternative)?;
    let scope = ScopedInvocation::scoped(ScopeShape::RepoWide, budget);
    let plan = plan_pass(&targets, scope.clone())?;
    assert_eq!(plan.scope(), &scope);
    let [first, second] = plan.runs() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    for (run, target, selection) in [
        (first, mapped.target(), &selected),
        (second, unmapped.target(), &Selection::All),
    ] {
        assert_eq!(run.lane(), PressureLane::CompiledMutation);
        assert_eq!(run.target(), target.identity());
        assert_eq!(run.damage(), PlannedDamage::BackendChosen);
        assert_eq!(run.selection(), selection);
        assert_eq!(run.budget(), budget);
    }
    let reversed = plan_pass(
        &[unmapped.target().clone(), mapped.target().clone()],
        scope.clone(),
    )?;
    assert_eq!(reversed.runs(), &[second.clone(), first.clone()]);
    assert_eq!(plan_pass(&[], scope), Err(PlanRefusal::NoRunPlanned));
    let small = PressureBudget::declared(1, profile)
        .map_err(|_| MutationRoadFailure::MissingAlternative)?;
    assert_eq!(
        plan_pass(
            &targets,
            ScopedInvocation::scoped(ScopeShape::RepoWide, small)
        ),
        Err(PlanRefusal::BudgetOverspent {
            admitted: 1,
            planned: 2
        })
    );
    Ok(())
}
