//! Historical attachment standing cannot become callable bindings, witnesses or revisions.

use macroonz_harness::descriptor::archive::{ArchivedBinding, ArchivedRevisionBinding};
use macroonz_harness::descriptor::RevisionBinding;
use macroonz_harness::muterprater::MutationWitness;
use macroonz_harness::runner::TrialBinding;

fn binding(historical: ArchivedBinding) -> TrialBinding {
    historical
}

fn witness(historical: ArchivedBinding) -> MutationWitness<()> {
    historical
}

fn revision(historical: ArchivedRevisionBinding) -> RevisionBinding {
    historical
}

fn main() {}
