//! Complete byte and population admission before nested run encoding allocates.

use super::encode::{bounded, sum};
use super::size_trial;
use super::{ArchiveLimits, ArchiveRefusal, RunArchiveLimits};
use crate::descriptor::{NamespacedName, TablePosture};
use crate::report::{RunReport, SelectionDisposition, TrialAccounting};

pub(super) fn name_size(
    name: NamespacedName,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    sum(&[
        16,
        bounded(name.namespace().written().len(), limits)?,
        bounded(name.stem().written().len(), limits)?,
    ])
}

pub(super) fn row_size(
    row: &TrialAccounting,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    bounded(32, limits)?;
    let disposition = match row.disposition() {
        SelectionDisposition::Selected(report) => sum(&[
            9,
            bounded(size_trial::encoded_size(report, limits)?, limits)?,
        ])?,
        SelectionDisposition::NotSelected {
            trial: _,
            reason: _,
        } => 1,
    };
    bounded(
        sum(&[160, name_size(row.claim().name(), limits)?, disposition])?,
        limits,
    )
}

pub(super) fn encoded_size(
    report: &RunReport,
    limits: RunArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    if report.census().len() > limits.rows() {
        return Err(ArchiveRefusal::TooManyRows);
    }
    u64::try_from(report.census().len()).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
    let bytes = limits.bytes();
    let target = report.target();
    let context = bounded(
        sum(&[
            36,
            bounded(target.target().spelling().len(), bytes)?,
            bounded(target.toolchain().spelling().len(), bytes)?,
        ])?,
        bytes,
    )?;
    let input = if let Some(input) = report.input() {
        bounded(81, bytes)?;
        bounded(32, bytes)?;
        sum(&[134, name_size(input.profile().name(), bytes)?])?
    } else {
        1
    };
    let posture = match report.posture() {
        TablePosture::Authored => 1,
        TablePosture::Staged { parent } => sum(&[1, name_size(parent.name(), bytes)?])?,
    };
    let mut total = sum(&[61, context, input, posture])?;
    for row in report.census() {
        total = sum(&[total, 8, row_size(row, bytes)?])?;
    }
    if total > bytes.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge);
    }
    Ok(total)
}
