//! Complete benchmark admission with historical stage and population joins.

use super::super::{
    ArchivedBenchOutcome, ArchivedBenchReading, ArchivedBenchReport, ArchivedBenchRow,
    BENCH_ARCHIVE_TAG, BenchArchiveLimits, BenchArchiveRefusal,
};
use super::{row, work};
use crate::bench::work::WorkStage;
use crate::descriptor::archive::{BindingArchiveLimits, read_provenance};
use crate::identity::BodyReader;
use crate::report::archive::{
    ArchiveRefusal, ArchivedAttempt, ArchivedConclusion, ArchivedTrial, envelope, finish, frame,
    name, read_trial, text,
};
use crate::report::{TargetBinding, TargetTriple, ToolchainIdentity};
use std::collections::BTreeSet;

/// Read a complete historical benchmark report under independent resource ceilings.
///
/// # Errors
///
/// Refuses malformed framing, excess populations, invalid declarations and inconsistent retained stages.
/// Loading invokes no caller code and grants no current execution or qualification.
pub fn read_report(
    encoded: &[u8],
    limits: BenchArchiveLimits,
) -> Result<ArchivedBenchReport, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let bytes = limits.bytes();
    let (address, mut reader) =
        envelope(encoded, BENCH_ARCHIVE_TAG, 1, bytes).map_err(Canonical)?;
    let table = name(&mut reader, bytes).map_err(Canonical)?;
    let provenance = read_provenance(
        frame(&mut reader, bytes).map_err(Canonical)?,
        BindingArchiveLimits::declared(bytes.envelope(), bytes.field(), 0),
    )
    .map_err(BenchArchiveRefusal::Provenance)?;
    let count = reader.count().map_err(Canonical)?;
    if count > limits.rows() {
        return Err(BenchArchiveRefusal::TooManyRows);
    }
    if count == 0 {
        return Err(BenchArchiveRefusal::EmptyReport);
    }
    let mut seen = BTreeSet::new();
    let mut readings = Vec::new();
    for _ in 0..count {
        let reading = reading(&mut reader, limits)?;
        if !seen.insert(*reading.row().key().as_bytes()) {
            return Err(BenchArchiveRefusal::DuplicateRow);
        }
        if readings
            .first()
            .is_some_and(|first: &ArchivedBenchReading| first.target() != reading.target())
        {
            return Err(BenchArchiveRefusal::TargetMismatch);
        }
        readings.push(reading);
    }
    finish(&reader).map_err(Canonical)?;
    Ok(ArchivedBenchReport {
        encoded: encoded.to_vec(),
        address,
        table,
        provenance,
        readings,
    })
}

fn reading(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: BenchArchiveLimits,
) -> Result<ArchivedBenchReading, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let bytes = limits.bytes();
    let row = row::row(frame(reader, bytes).map_err(Canonical)?, limits)?;
    let target = TargetBinding::bound(
        TargetTriple::declared(text(reader, bytes).map_err(Canonical)?),
        ToolchainIdentity::declared(text(reader, bytes).map_err(Canonical)?),
    );
    let preflight =
        read_trial(frame(reader, bytes).map_err(Canonical)?, bytes).map_err(Canonical)?;
    if preflight.key().target() != &target {
        return Err(BenchArchiveRefusal::TargetMismatch);
    }
    let outcome = outcome(reader, &row, &preflight, limits)?;
    Ok(ArchivedBenchReading {
        row,
        target,
        preflight,
        outcome,
    })
}

fn outcome(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    row: &ArchivedBenchRow,
    preflight: &ArchivedTrial,
    limits: BenchArchiveLimits,
) -> Result<ArchivedBenchOutcome, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let slot = reader.byte().map_err(Canonical)?;
    let preflight_passed = matches!(
        preflight.attempt(),
        ArchivedAttempt::Executed(ArchivedConclusion::Passed)
    );
    if slot == 0 {
        if preflight_passed {
            return Err(BenchArchiveRefusal::StageMismatch);
        }
        return Ok(ArchivedBenchOutcome::PreflightRefused);
    }
    if slot > 3 {
        return Err(Canonical(ArchiveRefusal::InvalidSlot));
    }
    if !preflight_passed {
        return Err(BenchArchiveRefusal::StageMismatch);
    }
    let measured = work::curve(reader, row, limits)?;
    let planted_worse = work::curve(reader, row, limits)?;
    work::join_curves(&measured, &planted_worse)?;
    let judgment = work::judgment(reader, limits)?;
    match (slot, work::stage(&judgment)) {
        (1, WorkStage::ControlNotDistinguished) => {
            Ok(ArchivedBenchOutcome::PlantedWorseNotDistinguished {
                measured,
                planted_worse,
                judgment,
            })
        }
        (2, WorkStage::MeasuredRefused) => Ok(ArchivedBenchOutcome::PrimaryWorkRefused {
            measured,
            planted_worse,
            judgment,
        }),
        (3, WorkStage::Qualified) => {
            let secondary = work::secondary(reader, row, &measured, limits)?;
            Ok(ArchivedBenchOutcome::Qualified {
                measured,
                planted_worse,
                judgment,
                secondary,
            })
        }
        _ => Err(BenchArchiveRefusal::StageMismatch),
    }
}
