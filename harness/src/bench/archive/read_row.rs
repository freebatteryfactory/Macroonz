//! Historical row decoding through the declaration's existing data invariants.

use super::super::{ArchivedBenchRow, BenchArchiveLimits, BenchArchiveRefusal};
use crate::bench::{
    BENCH_ROW_KEY_TAG, BenchMeasurement, ContentionPosture, DeclaredBudgets, InputSizeAxis,
    WorkFormula,
};
use crate::identity::ContentAddress;
use crate::report::archive::{ArchiveRefusal, cursor, finish, frame, name};

pub(super) fn row(
    encoded: &[u8],
    limits: BenchArchiveLimits,
) -> Result<ArchivedBenchRow, BenchArchiveRefusal> {
    use BenchArchiveRefusal::Canonical;
    let mut reader = cursor(encoded);
    let bytes = limits.bytes();
    let workload = name(&mut reader, bytes).map_err(Canonical)?;
    let count = reader.count().map_err(Canonical)?;
    if count > limits.axis() {
        return Err(BenchArchiveRefusal::TooManySizes);
    }
    let mut sizes = Vec::new();
    for _ in 0..count {
        sizes.push(reader.u64().map_err(Canonical)?);
    }
    let axis = InputSizeAxis::declared(sizes).map_err(BenchArchiveRefusal::Axis)?;
    let preflight = name(&mut reader, bytes).map_err(Canonical)?;
    let planted_worse = name(&mut reader, bytes).map_err(Canonical)?;
    let budgets = DeclaredBudgets::declared(
        reader.u32().map_err(Canonical)?,
        reader.u32().map_err(Canonical)?,
        reader.u64().map_err(Canonical)?,
        reader.u64().map_err(Canonical)?,
    )
    .map_err(BenchArchiveRefusal::Budgets)?;
    let contention = match reader.byte().map_err(Canonical)? {
        0 => ContentionPosture::NoDeclaredContention,
        _ => return Err(Canonical(ArchiveRefusal::InvalidSlot)),
    };
    let formula = match reader.byte().map_err(Canonical)? {
        0 => None,
        1 => Some(
            WorkFormula::encoded(frame(&mut reader, bytes).map_err(Canonical)?.to_vec())
                .map_err(BenchArchiveRefusal::Formula)?,
        ),
        _ => return Err(Canonical(ArchiveRefusal::InvalidSlot)),
    };
    let complexity = name(&mut reader, bytes).map_err(Canonical)?;
    finish(&reader).map_err(Canonical)?;
    Ok(ArchivedBenchRow {
        canonical: encoded.to_vec(),
        key: ContentAddress::derived(BENCH_ROW_KEY_TAG, encoded),
        workload,
        preflight,
        planted_worse,
        complexity,
        measurement: BenchMeasurement::declared(axis, budgets, contention, formula),
    })
}
