//! Bounded complete mutation admission at the live owner's historical ceiling.

use super::super::{
    ArchivedActivation, ArchivedMutation, ArchivedMutationOutcome, ArchivedRejection,
    MUTATION_ARCHIVE_TAG, MutationArchiveRefusal,
};
use super::read::{read_activation, read_target};
use crate::identity::BodyReader;
use crate::muterprater::{
    BaselineAxis, EquivalenceAxis, ExecutionAxis, InconclusiveCause, MaterializationAxis,
};
use crate::report::archive::{
    ArchiveLimits, ArchiveRefusal, envelope, finding, finish, foreign, frame,
};

/// Read a complete historical mutation without constructing live execution evidence.
///
/// # Errors
///
/// Refuses malformed or oversized records and outcomes stronger than their retained axes.
pub fn read_mutation(
    encoded: &[u8],
    limits: ArchiveLimits,
) -> Result<ArchivedMutation, MutationArchiveRefusal> {
    let (address, mut reader) = envelope(encoded, MUTATION_ARCHIVE_TAG, 1, limits)?;
    let target = read_target(frame(&mut reader, limits)?, limits)?;
    let baseline = baseline(reader.byte()?)?;
    let materialization = materialization(reader.byte()?)?;
    let activation = read_activation(frame(&mut reader, limits)?, limits)?;
    let execution = execution(reader.byte()?)?;
    let outcome = outcome(&mut reader, limits)?;
    let equivalence = equivalence(reader.byte()?)?;
    match &outcome {
        ArchivedMutationOutcome::Killed(_) | ArchivedMutationOutcome::Survived => {
            if baseline != BaselineAxis::Qualified
                || materialization != MaterializationAxis::Built
                || execution != ExecutionAxis::Completed
                || matches!(activation, ArchivedActivation::NotObserved)
            {
                return Err(MutationArchiveRefusal::OutcomeAxesMismatch);
            }
            if matches!(outcome, ArchivedMutationOutcome::Survived)
                && !matches!(activation, ArchivedActivation::Observed(_))
            {
                return Err(MutationArchiveRefusal::OutcomeAxesMismatch);
            }
        }
        ArchivedMutationOutcome::Inconclusive(_) => {}
    }
    finish(&reader)?;
    Ok(ArchivedMutation {
        encoded: encoded.to_vec(),
        address,
        target,
        baseline,
        materialization,
        activation,
        execution,
        outcome,
        equivalence,
    })
}

fn baseline(slot: u8) -> Result<BaselineAxis, ArchiveRefusal> {
    match slot {
        0 => Ok(BaselineAxis::Qualified),
        1 => Ok(BaselineAxis::Failed),
        2 => Ok(BaselineAxis::NotRun),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn materialization(slot: u8) -> Result<MaterializationAxis, ArchiveRefusal> {
    match slot {
        0 => Ok(MaterializationAxis::Built),
        1 => Ok(MaterializationAxis::Unviable),
        2 => Ok(MaterializationAxis::ToolFailed),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn execution(slot: u8) -> Result<ExecutionAxis, ArchiveRefusal> {
    match slot {
        0 => Ok(ExecutionAxis::Completed),
        1 => Ok(ExecutionAxis::NotExecuted),
        2 => Ok(ExecutionAxis::TimedOut),
        3 => Ok(ExecutionAxis::Crashed),
        4 => Ok(ExecutionAxis::InfrastructureFailed),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn equivalence(slot: u8) -> Result<EquivalenceAxis, ArchiveRefusal> {
    match slot {
        0 => Ok(EquivalenceAxis::NotAssessed),
        1 => Ok(EquivalenceAxis::ProvenInScope),
        2 => Ok(EquivalenceAxis::Refuted),
        3 => Ok(EquivalenceAxis::Inconclusive),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn cause(slot: u8) -> Result<InconclusiveCause, ArchiveRefusal> {
    match slot {
        0 => Ok(InconclusiveCause::BaselineNotQualified),
        1 => Ok(InconclusiveCause::NotMaterialized),
        2 => Ok(InconclusiveCause::NotActivated),
        3 => Ok(InconclusiveCause::WitnessIncomplete),
        4 => Ok(InconclusiveCause::UnobservableAndUnrejected),
        5 => Ok(InconclusiveCause::ProvenEquivalentInScope),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn outcome(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedMutationOutcome, ArchiveRefusal> {
    match reader.byte()? {
        0 => {
            let rejection = match reader.byte()? {
                0 => ArchivedRejection::Demonstrated(Box::new(finding(reader, limits)?)),
                1 => ArchivedRejection::ReportedByBackend(
                    foreign(reader, limits)?.ok_or(ArchiveRefusal::InvalidForeignText)?,
                ),
                _ => return Err(ArchiveRefusal::InvalidSlot),
            };
            Ok(ArchivedMutationOutcome::Killed(rejection))
        }
        1 => Ok(ArchivedMutationOutcome::Survived),
        2 => Ok(ArchivedMutationOutcome::Inconclusive(cause(
            reader.byte()?,
        )?)),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}
