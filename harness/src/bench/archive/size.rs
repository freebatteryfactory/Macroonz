//! Full byte and population admission before benchmark encoding allocates.

use super::{BenchArchiveLimits, BenchArchiveRefusal};
use crate::bench::{
    BenchOutcome, BenchReading, BenchReport, BenchRow, SecondaryObservation, WorkConclusion,
    WorkCurve, WorkGapStanding, WorkJudgment,
};
use crate::descriptor::archive::{BindingArchiveLimits, provenance_size};
use crate::report::FindingCause;
use crate::report::archive::{
    ArchiveRefusal, bounded, measurement_size, name_size, sum, trial_size,
};

pub(super) fn report_size(
    report: &BenchReport,
    limits: BenchArchiveLimits,
) -> Result<usize, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    if report.denominator() > limits.rows() {
        return Err(BenchArchiveRefusal::TooManyRows);
    }
    let bytes = limits.bytes();
    let provenance = provenance_size(
        report.provenance(),
        BindingArchiveLimits::declared(bytes.envelope(), bytes.field(), 0),
    )
    .map_err(BenchArchiveRefusal::Provenance)?;
    let mut total = sum(&[
        60,
        name_size(report.table().name(), bytes).map_err(Canonical)?,
        bounded(provenance, bytes).map_err(Canonical)?,
    ])
    .map_err(Canonical)?;
    for reading in report.readings() {
        total = sum(&[total, reading_size(reading, limits)?]).map_err(Canonical)?;
    }
    if total > bytes.envelope() {
        return Err(Canonical(ArchiveRefusal::EnvelopeTooLarge));
    }
    Ok(total)
}

fn reading_size(
    reading: &BenchReading,
    limits: BenchArchiveLimits,
) -> Result<usize, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let bytes = limits.bytes();
    let row = bounded(row_size(reading.row(), limits)?, bytes).map_err(Canonical)?;
    let preflight = bounded(
        trial_size(reading.preflight(), bytes).map_err(Canonical)?,
        bytes,
    )
    .map_err(Canonical)?;
    sum(&[
        32,
        row,
        preflight,
        bounded(reading.target().target().spelling().len(), bytes).map_err(Canonical)?,
        bounded(reading.target().toolchain().spelling().len(), bytes).map_err(Canonical)?,
        outcome_size(reading.outcome(), limits)?,
    ])
    .map_err(Canonical)
}

fn row_size(row: &BenchRow, limits: BenchArchiveLimits) -> Result<usize, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let bytes = limits.bytes();
    let sizes = row.input_sizes().sizes().len();
    if sizes > limits.axis() {
        return Err(BenchArchiveRefusal::TooManySizes);
    }
    let axis = sizes
        .checked_mul(8)
        .ok_or(Canonical(ArchiveRefusal::SizeOutsidePlatform))?;
    let formula = match row.formula() {
        None => 0,
        Some(formula) => sum(&[8, bounded(formula.bytes().len(), bytes).map_err(Canonical)?])
            .map_err(Canonical)?,
    };
    sum(&[
        34,
        axis,
        formula,
        name_size(row.workload().name(), bytes).map_err(Canonical)?,
        name_size(row.preflight().name(), bytes).map_err(Canonical)?,
        name_size(row.planted_worse().name(), bytes).map_err(Canonical)?,
        name_size(row.complexity().name(), bytes).map_err(Canonical)?,
    ])
    .map_err(Canonical)
}

fn curve_size(curve: &WorkCurve, limits: BenchArchiveLimits) -> Result<usize, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    if curve.points().len() > limits.axis() {
        return Err(BenchArchiveRefusal::TooManySizes);
    }
    let mut total = 8;
    for point in curve.points() {
        if point.counts().len() > limits.observations() {
            return Err(BenchArchiveRefusal::TooManyObservations);
        }
        total = sum(&[total, 16]).map_err(Canonical)?;
        for count in point.counts() {
            total = sum(&[
                total,
                8,
                name_size(count.observation().name(), limits.bytes()).map_err(Canonical)?,
            ])
            .map_err(Canonical)?;
        }
    }
    Ok(total)
}

fn cause_size(
    cause: FindingCause,
    limits: BenchArchiveLimits,
) -> Result<usize, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    sum(&[
        16,
        bounded(cause.family().len(), limits.bytes()).map_err(Canonical)?,
        bounded(cause.local().len(), limits.bytes()).map_err(Canonical)?,
    ])
    .map_err(Canonical)
}

fn conclusion_size(
    conclusion: WorkConclusion,
    limits: BenchArchiveLimits,
) -> Result<usize, BenchArchiveRefusal> {
    match conclusion {
        WorkConclusion::Satisfied => Ok(1),
        WorkConclusion::Refused(cause) => {
            sum(&[1, cause_size(cause, limits)?]).map_err(BenchArchiveRefusal::Canonical)
        }
    }
}

fn judgment_size(
    judgment: WorkJudgment,
    limits: BenchArchiveLimits,
) -> Result<usize, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let gap = match judgment.gap() {
        WorkGapStanding::Distinguished => 1,
        WorkGapStanding::NotDistinguished(cause) => {
            sum(&[1, cause_size(cause, limits)?]).map_err(Canonical)?
        }
    };
    sum(&[
        conclusion_size(judgment.measured(), limits)?,
        conclusion_size(judgment.planted_worse(), limits)?,
        gap,
    ])
    .map_err(Canonical)
}

fn secondary_size(
    secondary: &SecondaryObservation,
    limits: BenchArchiveLimits,
) -> Result<usize, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    if secondary.measurements().len() > limits.measurements() {
        return Err(BenchArchiveRefusal::TooManyMeasurements);
    }
    let mut total = sum(&[
        9,
        curve_size(secondary.work(), limits)?,
        judgment_size(secondary.judgment(), limits)?,
    ])
    .map_err(Canonical)?;
    for measurement in secondary.measurements() {
        total = sum(&[total, measurement_size(*measurement)]).map_err(Canonical)?;
    }
    Ok(total)
}

fn outcome_size(
    outcome: &BenchOutcome,
    limits: BenchArchiveLimits,
) -> Result<usize, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let (measured, planted_worse, judgment, secondary) = match outcome {
        BenchOutcome::PreflightRefused => return Ok(1),
        BenchOutcome::PlantedWorseNotDistinguished {
            measured,
            planted_worse,
            judgment,
        }
        | BenchOutcome::PrimaryWorkRefused {
            measured,
            planted_worse,
            judgment,
        } => (measured, planted_worse, *judgment, 0),
        BenchOutcome::Qualified {
            measured,
            planted_worse,
            judgment,
            secondary,
        } => (
            measured,
            planted_worse,
            *judgment,
            secondary_size(secondary, limits)?,
        ),
    };
    sum(&[
        1,
        curve_size(measured, limits)?,
        curve_size(planted_worse, limits)?,
        judgment_size(judgment, limits)?,
        secondary,
    ])
    .map_err(Canonical)
}
