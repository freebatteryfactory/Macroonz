//! Bounded complete-census admission and historical cross-record joins.

use super::super::{
    ArchiveLimits, ArchiveRefusal, ArchivedAccounting, ArchivedDisposition, ArchivedName,
    ArchivedRun, ArchivedTablePosture, RUN_ARCHIVE_TAG, RunArchiveLimits, read_trial,
};
use super::identity::{claim, context, cursor, envelope, finish, frame, input, text};
use crate::identity::BodyReader;
use crate::report::{EmptySelectionReason, NotSelectedReason, SelectionOutcome};
use std::collections::BTreeSet;

/// Read a complete historical census under independent byte and row ceilings.
///
/// # Errors
///
/// Refuses malformed material, excess bounds, duplicate trials and contradictory joins.
/// Admission establishes internal consistency without authenticating the census or its producer.
pub fn read_run(encoded: &[u8], limits: RunArchiveLimits) -> Result<ArchivedRun, ArchiveRefusal> {
    let bytes = limits.bytes();
    let (address, mut reader) = envelope(encoded, RUN_ARCHIVE_TAG, 3, bytes)?;
    let mut context_reader = cursor(frame(&mut reader, bytes)?);
    let (invocation, target) = context(&mut context_reader, bytes)?;
    finish(&context_reader)?;
    let input = match reader.byte()? {
        0 => None,
        1 => {
            let mut coordinates = cursor(frame(&mut reader, bytes)?);
            let input = input(&mut coordinates, &mut reader, bytes)?;
            finish(&coordinates)?;
            Some(input)
        }
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    let posture = match reader.byte()? {
        0 => ArchivedTablePosture::Authored,
        1 => ArchivedTablePosture::Staged {
            parent: name(&mut reader, bytes)?,
        },
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    let selection = selection(reader.byte()?)?;
    let count = reader.count()?;
    if count > limits.rows() {
        return Err(ArchiveRefusal::TooManyRows);
    }
    let mut census = Vec::new();
    let mut trials = BTreeSet::new();
    for _ in 0..count {
        let row = accounting(frame(&mut reader, bytes)?, bytes)?;
        if !trials.insert(*row.trial.as_bytes()) {
            return Err(ArchiveRefusal::DuplicateTrial);
        }
        census.push(row);
    }
    finish(&reader)?;
    joined(ArchivedRun {
        encoded: encoded.to_vec(),
        address,
        invocation,
        target,
        input,
        posture,
        selection,
        census,
    })
}

fn name(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedName, ArchiveRefusal> {
    let namespace = text(reader, limits)?;
    let stem = text(reader, limits)?;
    if namespace.is_empty() || stem.is_empty() {
        return Err(ArchiveRefusal::InvalidText);
    }
    Ok(ArchivedName {
        namespace: namespace.to_owned(),
        stem: stem.to_owned(),
    })
}

fn selection(slot: u8) -> Result<SelectionOutcome, ArchiveRefusal> {
    match slot {
        0 => Ok(SelectionOutcome::Satisfied),
        1 => Ok(SelectionOutcome::UnsatisfiedByEmptySelection),
        2 => Ok(SelectionOutcome::EmptyAsStated(
            EmptySelectionReason::CarriedOverFromAPreviousRun,
        )),
        3 => Ok(SelectionOutcome::EmptyAsStated(
            EmptySelectionReason::AskingWhatTheWorldHolds,
        )),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn accounting(bytes: &[u8], limits: ArchiveLimits) -> Result<ArchivedAccounting, ArchiveRefusal> {
    let mut reader = cursor(bytes);
    let trial = claim(&mut reader, limits)?;
    let row = claim(&mut reader, limits)?;
    let subject = claim(&mut reader, limits)?;
    let check = claim(&mut reader, limits)?;
    let claim = name(&mut reader, limits)?;
    let disposition = match reader.byte()? {
        0 => ArchivedDisposition::Selected(Box::new(read_trial(
            frame(&mut reader, limits)?,
            limits,
        )?)),
        1 => ArchivedDisposition::NotSelected(NotSelectedReason::OutsideSelection),
        2 => ArchivedDisposition::NotSelected(NotSelectedReason::SuiteNotRun),
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    finish(&reader)?;
    if let ArchivedDisposition::Selected(report) = &disposition
        && (report.key().trial() != trial
            || report.key().subject() != subject
            || report.key().check() != check)
    {
        return Err(ArchiveRefusal::IdentityJoinMismatch);
    }
    Ok(ArchivedAccounting {
        trial,
        row,
        subject,
        check,
        claim,
        disposition,
    })
}

fn joined(run: ArchivedRun) -> Result<ArchivedRun, ArchiveRefusal> {
    let mut selected = false;
    for row in &run.census {
        if let ArchivedDisposition::Selected(report) = &row.disposition {
            selected = true;
            let key = report.key();
            if key.invocation() != run.invocation
                || key.target() != &run.target
                || key.input() != run.input.as_ref()
            {
                return Err(ArchiveRefusal::RunContextMismatch);
            }
        }
    }
    if selected != (run.selection == SelectionOutcome::Satisfied) {
        return Err(ArchiveRefusal::SelectionMismatch);
    }
    Ok(run)
}
