//! Complete historical mutation encoding from an existing live record.

use super::size_record::encoded_size;
use super::{
    ArchivedMutation, MUTATION_ARCHIVE_TAG, MutationArchiveRefusal, activation_size, read_mutation,
    target_size, write_activation, write_target,
};
use crate::identity::{ContentAddress, encode_length};
use crate::muterprater::{
    BaselineAxis, EquivalenceAxis, ExecutionAxis, InconclusiveCause, IntendedRejection,
    MaterializationAxis, MutationOutcome, MutationReport,
};
use crate::report::archive::{ArchiveLimits, write_finding, write_foreign};

/// Retain every mutation axis and rejection member as bounded historical data.
///
/// # Errors
///
/// Refuses independent field and envelope ceilings before allocating preimages.
pub fn retain_mutation(
    report: &MutationReport,
    limits: ArchiveLimits,
) -> Result<ArchivedMutation, MutationArchiveRefusal> {
    let total = encoded_size(report, limits)?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    for word in [1u32, 1, 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    encode_length(target_size(report.target(), limits)?, &mut body);
    write_target(report.target(), &mut body);
    body.push(match report.baseline() {
        BaselineAxis::Qualified => 0,
        BaselineAxis::Failed => 1,
        BaselineAxis::NotRun => 2,
    });
    body.push(match report.materialization() {
        MaterializationAxis::Built => 0,
        MaterializationAxis::Unviable => 1,
        MaterializationAxis::ToolFailed => 2,
    });
    encode_length(activation_size(report.activation(), limits)?, &mut body);
    write_activation(report.activation(), &mut body);
    body.push(match report.execution() {
        ExecutionAxis::Completed => 0,
        ExecutionAxis::NotExecuted => 1,
        ExecutionAxis::TimedOut => 2,
        ExecutionAxis::Crashed => 3,
        ExecutionAxis::InfrastructureFailed => 4,
    });
    outcome(report.outcome(), &mut body);
    body.push(match report.equivalence() {
        EquivalenceAxis::NotAssessed => 0,
        EquivalenceAxis::ProvenInScope => 1,
        EquivalenceAxis::Refuted => 2,
        EquivalenceAxis::Inconclusive => 3,
    });
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(MUTATION_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_mutation(&encoded, limits)
}

fn outcome(outcome: &MutationOutcome, body: &mut Vec<u8>) {
    match outcome {
        MutationOutcome::Killed(rejection) => {
            body.push(0);
            match rejection {
                IntendedRejection::Demonstrated(rejection) => {
                    body.push(0);
                    write_finding(rejection.trial(), rejection.finding(), body);
                }
                IntendedRejection::ReportedByBackend { stated } => {
                    body.push(1);
                    write_foreign(Some(stated), body);
                }
            }
        }
        MutationOutcome::Survived => body.push(1),
        MutationOutcome::Inconclusive(cause) => {
            body.push(2);
            body.push(match cause {
                InconclusiveCause::BaselineNotQualified => 0,
                InconclusiveCause::NotMaterialized => 1,
                InconclusiveCause::NotActivated => 2,
                InconclusiveCause::WitnessIncomplete => 3,
                InconclusiveCause::UnobservableAndUnrejected => 4,
                InconclusiveCause::ProvenEquivalentInScope => 5,
            });
        }
    }
}
