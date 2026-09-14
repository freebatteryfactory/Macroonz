//! Loaded assessments retain historical data and cannot supply current callable qualification.

use macroonz_harness::muterprater::interpretation_archive::ArchivedAssessment;
use macroonz_harness::muterprater::MutationAssessment;

fn elevate(record: ArchivedAssessment) -> MutationAssessment<'static, (), ()> {
    record
}

fn main() {}
