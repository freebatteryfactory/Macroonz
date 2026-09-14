//! Actual witness readings cannot mint an assessment by bypassing its qualification operation.

use macroonz_harness::muterprater::{MutationAssessment, MutationWitnessReading, MutationReport};

fn mint<'scope>(reading: MutationWitnessReading<'scope, (), ()>, mutation: MutationReport) -> MutationAssessment<'scope, (), ()> {
    MutationAssessment { reading, mutation }
}

fn main() {}
