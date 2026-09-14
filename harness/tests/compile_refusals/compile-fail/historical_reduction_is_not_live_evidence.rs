//! Historical reductions cannot enter the live capsule mint or change an admitted budget.

use macroonz_harness::generate::{ReductionBudget, ReductionEvidence, capture_replay};
use macroonz_harness::generate::reduce::archive::ArchivedReduction;
use macroonz_harness::report::ReplayCapsule;

fn promote(historical: ArchivedReduction) -> ReductionEvidence {
    historical
}

fn capture(historical: &ArchivedReduction) -> ReplayCapsule {
    capture_replay(historical)
}

fn change_budget(historical: &mut ArchivedReduction) {
    historical.budget = ReductionBudget::declared(999);
}

fn main() {}
