//! Retention of the complete existing report through its owning canonical writers.

use super::size::report_size;
use super::{
    ArchivedBenchReport, BENCH_ARCHIVE_TAG, BenchArchiveLimits, BenchArchiveRefusal, read_report,
};
use crate::bench::{
    BenchOutcome, BenchReading, BenchReport, SecondaryObservation, WorkConclusion, WorkCurve,
    WorkGapStanding, WorkJudgment,
};
use crate::descriptor::archive::write_provenance;
use crate::identity::{ContentAddress, encode_bytes, encode_length};
use crate::report::FindingCause;
use crate::report::archive::{attribution_slot, retain_trial, write_measurement};

/// Retain every row and reached stage of an existing benchmark report.
///
/// # Errors
///
/// Refuses independent byte and population bounds before allocating canonical buffers.
/// The result retains historical data and cannot grant current work qualification.
pub fn retain_report(
    report: &BenchReport,
    limits: BenchArchiveLimits,
) -> Result<ArchivedBenchReport, BenchArchiveRefusal> {
    let total = report_size(report, limits)?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());
    report.table().name().encode_into(&mut body);
    let mut provenance = Vec::new();
    write_provenance(report.provenance(), &mut provenance);
    encode_bytes(&provenance, &mut body);
    encode_length(report.denominator(), &mut body);
    for reading in report.readings() {
        write_reading(reading, limits, &mut body)?;
    }
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(BENCH_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_report(&encoded, limits)
}

fn write_reading(
    reading: &BenchReading,
    limits: BenchArchiveLimits,
    body: &mut Vec<u8>,
) -> Result<(), BenchArchiveRefusal> {
    encode_bytes(
        &reading
            .row()
            .canonical_preimage()
            .map_err(BenchArchiveRefusal::Declaration)?,
        body,
    );
    encode_bytes(reading.target().target().spelling().as_bytes(), body);
    encode_bytes(reading.target().toolchain().spelling().as_bytes(), body);
    let preflight = retain_trial(reading.preflight(), limits.bytes())
        .map_err(BenchArchiveRefusal::Canonical)?;
    encode_bytes(preflight.encoded(), body);
    let (measured, planted_worse, judgment, secondary) = match reading.outcome() {
        BenchOutcome::PreflightRefused => {
            body.push(0);
            return Ok(());
        }
        BenchOutcome::PlantedWorseNotDistinguished {
            measured,
            planted_worse,
            judgment,
        } => {
            body.push(1);
            (measured, planted_worse, *judgment, None)
        }
        BenchOutcome::PrimaryWorkRefused {
            measured,
            planted_worse,
            judgment,
        } => {
            body.push(2);
            (measured, planted_worse, *judgment, None)
        }
        BenchOutcome::Qualified {
            measured,
            planted_worse,
            judgment,
            secondary,
        } => {
            body.push(3);
            (measured, planted_worse, *judgment, Some(secondary))
        }
    };
    write_curve(measured, body);
    write_curve(planted_worse, body);
    write_judgment(judgment, body);
    if let Some(secondary) = secondary {
        write_secondary(secondary, body);
    }
    Ok(())
}

fn write_curve(curve: &WorkCurve, body: &mut Vec<u8>) {
    encode_length(curve.points().len(), body);
    for point in curve.points() {
        body.extend_from_slice(&point.input_size().to_be_bytes());
        encode_length(point.counts().len(), body);
        for count in point.counts() {
            count.observation().name().encode_into(body);
            body.extend_from_slice(&count.count().to_be_bytes());
        }
    }
}

fn write_cause(cause: FindingCause, body: &mut Vec<u8>) {
    encode_bytes(cause.family().as_bytes(), body);
    encode_bytes(cause.local().as_bytes(), body);
}

fn write_conclusion(conclusion: WorkConclusion, body: &mut Vec<u8>) {
    match conclusion {
        WorkConclusion::Satisfied => body.push(0),
        WorkConclusion::Refused(cause) => {
            body.push(1);
            write_cause(cause, body);
        }
    }
}

fn write_judgment(judgment: WorkJudgment, body: &mut Vec<u8>) {
    write_conclusion(judgment.measured(), body);
    write_conclusion(judgment.planted_worse(), body);
    match judgment.gap() {
        WorkGapStanding::Distinguished => body.push(0),
        WorkGapStanding::NotDistinguished(cause) => {
            body.push(1);
            write_cause(cause, body);
        }
    }
}

fn write_secondary(secondary: &SecondaryObservation, body: &mut Vec<u8>) {
    write_curve(secondary.work(), body);
    write_judgment(secondary.judgment(), body);
    body.push(attribution_slot(secondary.clock_attribution()));
    encode_length(secondary.measurements().len(), body);
    for reading in secondary.measurements() {
        write_measurement(*reading, body);
    }
}
