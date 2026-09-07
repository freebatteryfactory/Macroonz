//! Complete mutation size admission before encoding allocates preimages.

use super::{MutationArchiveRefusal, activation_size, target_size};
use crate::muterprater::{IntendedRejection, MutationOutcome, MutationReport};
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, finding_size, foreign_size, sum};

pub(super) fn encoded_size(
    report: &MutationReport,
    limits: ArchiveLimits,
) -> Result<usize, MutationArchiveRefusal> {
    let target = target_size(report.target(), limits)?;
    let activation = activation_size(report.activation(), limits)?;
    let outcome = match report.outcome() {
        MutationOutcome::Killed(IntendedRejection::Demonstrated(rejection)) => {
            sum(&[2, finding_size(rejection.finding(), limits)?])?
        }
        MutationOutcome::Killed(IntendedRejection::ReportedByBackend { stated }) => {
            sum(&[2, foreign_size(Some(stated), limits)?])?
        }
        MutationOutcome::Survived => 1,
        MutationOutcome::Inconclusive(_) => 2,
    };
    let total = sum(&[64, target, activation, outcome])?;
    if total > limits.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge.into());
    }
    Ok(total)
}
