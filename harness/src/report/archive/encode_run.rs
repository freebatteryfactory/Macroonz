//! Complete-run retention through the existing context, input and trial grammars.

use super::encode::write_input_metadata;
use super::size_run::{encoded_size, row_size};
use super::{
    ArchiveLimits, ArchiveRefusal, ArchivedRun, RUN_ARCHIVE_TAG, RunArchiveLimits, read_run,
    retain_trial,
};
use crate::descriptor::TablePosture;
use crate::identity::{ContentAddress, encode_bytes, encode_length};
use crate::report::encode::{encode_context, encode_input};
use crate::report::{
    EmptySelectionReason, NotSelectedReason, RunReport, SelectionDisposition, SelectionOutcome,
    TrialAccounting,
};

/// Retain every row and run coordinate as bounded historical data.
///
/// # Errors
///
/// Refuses independent population, field or envelope ceilings before allocating nested records.
pub fn retain_run(
    report: &RunReport,
    limits: RunArchiveLimits,
) -> Result<ArchivedRun, ArchiveRefusal> {
    let total = encoded_size(report, limits)?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    for word in [1u32, 3, 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    let mut context = Vec::new();
    encode_context(report.invocation(), report.target(), &mut context);
    encode_bytes(&context, &mut body);
    if let Some(input) = report.input() {
        body.push(1);
        let mut coordinates = Vec::new();
        encode_input(input, &mut coordinates);
        encode_bytes(&coordinates, &mut body);
        write_input_metadata(input, &mut body);
    } else {
        body.push(0);
    }
    match report.posture() {
        TablePosture::Authored => body.push(0),
        TablePosture::Staged { parent } => {
            body.push(1);
            parent.name().encode_into(&mut body);
        }
    }
    body.push(match report.selection() {
        SelectionOutcome::Satisfied => 0,
        SelectionOutcome::UnsatisfiedByEmptySelection => 1,
        SelectionOutcome::EmptyAsStated(EmptySelectionReason::CarriedOverFromAPreviousRun) => 2,
        SelectionOutcome::EmptyAsStated(EmptySelectionReason::AskingWhatTheWorldHolds) => 3,
    });
    encode_length(report.census().len(), &mut body);
    for row in report.census() {
        encode_bytes(&accounting(row, limits.bytes())?, &mut body);
    }
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(RUN_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_run(&encoded, limits)
}

fn accounting(row: &TrialAccounting, limits: ArchiveLimits) -> Result<Vec<u8>, ArchiveRefusal> {
    let mut body = Vec::with_capacity(row_size(row, limits)?);
    for address in [
        row.trial().address(),
        row.row().address(),
        row.revisions().subject().address(),
        row.revisions().check().address(),
    ] {
        encode_bytes(address.as_bytes(), &mut body);
    }
    row.claim().name().encode_into(&mut body);
    match row.disposition() {
        SelectionDisposition::Selected(report) => {
            body.push(0);
            encode_bytes(retain_trial(report, limits)?.encoded(), &mut body);
        }
        SelectionDisposition::NotSelected { trial: _, reason } => body.push(match reason {
            NotSelectedReason::OutsideSelection => 1,
            NotSelectedReason::SuiteNotRun => 2,
        }),
    }
    Ok(body)
}
