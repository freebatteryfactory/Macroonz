//! Claim: a report retains one reading per authored binding and verdict chooses the first red row.
//! Subject: complete table execution, report denominator, and `bench_verdict` order.
//! Population: two distinct rows in positive and positive-then-hostile orderings.
//! Reversal: the hostile second row must be the exact refusal identity rather than an aggregate failure.
//! Denominator: both authored bindings in each table.
//! Evidence ceiling: in-process report ordering only, not persistence or serialization.
//! Retained regression: reordered readings, lost rows, or verdict selection detached from authored order.

use super::{fixture, support::*};

#[test]
fn complete_report_retains_two_distinct_rows_in_table_order() -> Result<(), BenchRoadFailure> {
    let positive_first = fixture::lawful_binding()?;
    let second = BenchBinding::bound(
        fixture::row_with_axis(vec![2u64, 8u64, 32u64])?,
        fixture::lawful_attachment(
            fixture::measured,
            fixture::planted_worse,
            fixture::lawful_judge,
        )?,
        fixture::lawful_preflight(fixture::preflight_passes)?,
    )
    .map_err(BenchStampRefusal::from)?;
    let bindings = vec![positive_first, second];
    let expected_rows = bindings
        .iter()
        .map(|binding| binding.row().key())
        .collect::<Vec<_>>();
    let table = BenchTable::authored(
        BenchTableName::named(OWNER, "same-workload-distinct-rows")?,
        Provenance::Unproduced,
        bindings,
    )
    .map_err(BenchStampRefusal::from)?;
    let report = run_all(&table, &fixture::invocation())?;
    let found_rows = report
        .readings()
        .iter()
        .map(|reading| reading.row().key())
        .collect::<Vec<_>>();
    assert_eq!(found_rows, expected_rows);
    assert_eq!(report.denominator(), table.len());
    bench_verdict(&report)?;

    let verdict_first = fixture::lawful_binding()?;
    let hostile = BenchBinding::bound(
        fixture::row_with_axis(vec![2u64, 8u64, 32u64])?,
        fixture::lawful_attachment(fixture::measured, fixture::measured, fixture::lawful_judge)?,
        fixture::lawful_preflight(fixture::preflight_passes)?,
    )
    .map_err(BenchStampRefusal::from)?;
    let expected_refusal_row = hostile.row().key();
    let hostile_table = BenchTable::authored(
        BenchTableName::named(OWNER, "first-red-in-authored-order")?,
        Provenance::Unproduced,
        vec![verdict_first, hostile],
    )
    .map_err(BenchStampRefusal::from)?;
    let hostile_report = run_all(&hostile_table, &fixture::invocation())?;
    let Err(refusal) = bench_verdict(&hostile_report) else {
        return Err(BenchRoadFailure::MissingVerdictRefusal);
    };
    assert_eq!(refusal.row(), expected_refusal_row);
    assert_eq!(refusal.stage(), BenchStage::PlantedWorseNotDistinguished);
    Ok(())
}
