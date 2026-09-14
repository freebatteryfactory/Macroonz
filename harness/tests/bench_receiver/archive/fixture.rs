//! Actual receiver executions used as inputs to historical retention controls.

use super::super::{fixture as receiver, support};
use super::LIMITS;
use super::types::{ReportVector, RowVector};
use macroonz_harness::bench::{
    BenchBinding, BenchReport, BenchTable, BenchTableName, WorkJudge, run_all,
};
use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::{GeneratedSupportSchema, ProducerName, Provenance};
use macroonz_harness::report::TrialConclusion;
use macroonz_harness::report::archive::retain_trial;
use macroonz_harness::runner::Invocation;
use std::error::Error;

pub(super) const AXES: [[u64; 3]; 4] = [[8, 2, 4], [9, 3, 6], [10, 2, 5], [12, 4, 8]];

pub(super) fn binding(
    sizes: &[u64],
    judge: WorkJudge,
    preflight: fn(&Invocation) -> TrialConclusion,
) -> Result<BenchBinding, Box<dyn Error>> {
    let row = receiver::row_with_axis(sizes.to_vec()).map_err(|e| format!("row: {e:?}"))?;
    let attachment =
        receiver::lawful_attachment(receiver::measured, receiver::planted_worse, judge)
            .map_err(|e| format!("attachment: {e:?}"))?;
    let preflight =
        receiver::lawful_preflight(preflight).map_err(|e| format!("preflight: {e:?}"))?;
    BenchBinding::bound(row, attachment, preflight).map_err(|e| format!("binding: {e:?}").into())
}

pub(super) fn report(
    bindings: Vec<BenchBinding>,
    provenance: Provenance,
    clock: HarnessClock,
) -> Result<BenchReport, Box<dyn Error>> {
    let name = BenchTableName::named("harness.bench.archive", "complete-table")
        .map_err(|e| format!("table name: {e:?}"))?;
    let table =
        BenchTable::authored(name, provenance, bindings).map_err(|e| format!("table: {e:?}"))?;
    run_all(&table, &receiver::invocation_with(clock)).map_err(|e| format!("run: {e:?}").into())
}

pub(super) fn qualified() -> Result<BenchReport, Box<dyn Error>> {
    report(
        vec![binding(
            &[2, 4, 8],
            receiver::lawful_judge,
            receiver::preflight_passes,
        )?],
        Provenance::Unproduced,
        HarnessClock::unavailable(),
    )
}

pub(super) fn all_stages() -> Result<BenchReport, Box<dyn Error>> {
    let [first, second, third, fourth] = AXES;
    report(
        vec![
            binding(&first, receiver::lawful_judge, support::preflight_refuses)?,
            binding(&second, support::always_satisfy, receiver::preflight_passes)?,
            binding(&third, support::always_refuse, receiver::preflight_passes)?,
            binding(&fourth, receiver::lawful_judge, receiver::preflight_passes)?,
        ],
        Provenance::Unproduced,
        HarnessClock::unavailable(),
    )
}

pub(super) fn produced() -> Result<Provenance, Box<dyn Error>> {
    let producer = ProducerName::named("outside", "producer").map_err(|e| format!("{e:?}"))?;
    let schema = GeneratedSupportSchema::published()
        .map_err(|e| format!("{e:?}"))?
        .identity()
        .map_err(|e| format!("{e:?}"))?;
    Ok(Provenance::Produced { producer, schema })
}

pub(super) fn preflights(report: &BenchReport) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    // Nested trial bytes have their own independent vectors in report_records.
    report
        .readings()
        .iter()
        .map(|reading| {
            retain_trial(reading.preflight(), LIMITS.bytes())
                .map(|record| record.encoded().to_vec())
                .map_err(|e| format!("nested trial: {e:?}").into())
        })
        .collect()
}

pub(super) fn vector() -> Result<ReportVector, Box<dyn Error>> {
    let report = qualified()?;
    let trial = preflights(&report)?
        .into_iter()
        .next()
        .ok_or("missing preflight")?;
    Ok(ReportVector::declared(vec![RowVector::qualified(
        trial,
        &[2, 4, 8],
    )]))
}
