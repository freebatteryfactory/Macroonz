//! A historical mutation and its fields cannot enter live mutation evidence seams.

use macroonz_harness::muterprater::verdict_archive::{ArchivedMutation, ArchivedMutationRun, ArchivedRejection};
use macroonz_harness::muterprater::{ActivationDisposition, BaselineQualification, IntendedRejection, MutationReport, MutationRun};

fn promote_record(historical: ArchivedMutation) -> MutationReport { historical }
fn promote_rejection(historical: ArchivedRejection) -> IntendedRejection { historical }
fn promote_baseline(historical: &ArchivedMutation) -> BaselineQualification { historical.baseline() }
fn promote_activation(historical: &ArchivedMutation) -> ActivationDisposition { historical.activation().clone() }
fn promote_run(historical: ArchivedMutationRun) -> MutationRun { historical }

fn main() {}
