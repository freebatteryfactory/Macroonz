//! Complete report content, independent framing and exact admission boundaries.

use super::{
    LIMITS, fixture,
    types::{ReportVector, RowVector},
    wire,
};
use macroonz_harness::bench::archive::{
    ArchivedBenchOutcome, ArchivedBenchReading, ArchivedBenchReport, ArchivedWorkConclusion,
    ArchivedWorkCurve, ArchivedWorkGap, ArchivedWorkJudgment, BenchArchiveLimits,
    BenchArchiveRefusal, read_report, retain_report,
};
use macroonz_harness::bench::{BenchStage, WorkFormula};
use macroonz_harness::clock::{ClockAttribution, HarnessClock};
use macroonz_harness::descriptor::{Provenance, archive::ArchivedProvenance};
use macroonz_harness::report::archive::{
    ArchiveLimits, ArchiveRefusal, ArchivedAttempt, ArchivedConclusion, ArchivedMeasurement,
};
use std::error::Error;

pub(super) fn observe_stages(record: &ArchivedBenchReport) -> Result<(), Box<dyn Error>> {
    assert_eq!(record.denominator(), 4);
    assert_eq!(
        (record.table().namespace(), record.table().stem()),
        ("harness.bench.archive", "complete-table")
    );
    assert_eq!(record.provenance(), &ArchivedProvenance::Unproduced);
    let stages = [
        BenchStage::PreflightRefused,
        BenchStage::PlantedWorseNotDistinguished,
        BenchStage::PrimaryWorkRefused,
        BenchStage::Qualified,
    ];
    for ((reading, axis), stage) in record.readings().iter().zip(fixture::AXES).zip(stages) {
        observe_reading(reading, &axis, stage)?;
    }
    Ok(())
}

fn observe_reading(
    reading: &ArchivedBenchReading,
    axis: &[u64],
    stage: BenchStage,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(reading.outcome().stage(), stage);
    let row = reading.row();
    assert_eq!(row.workload().stem(), "linear-workload");
    assert_eq!(row.preflight().stem(), "correctness-preflight");
    assert_eq!(row.planted_worse().stem(), "quadratic-control");
    assert_eq!(row.complexity().stem(), "linear-growth");
    assert_eq!(row.measurement().input_sizes().sizes(), axis);
    assert_eq!(row.measurement().budgets().samples(), 2);
    assert_eq!(row.measurement().budgets().warmups(), 1);
    assert_eq!(row.measurement().budgets().ratio().numerator(), 2);
    assert_eq!(row.measurement().budgets().ratio().denominator(), 1);
    assert_eq!(
        row.measurement().formula().map(WorkFormula::bytes),
        Some(b"work=samples*n".as_slice())
    );
    assert_eq!(reading.target().target().spelling(), "neutral-bench-target");
    assert_eq!(reading.target().toolchain().spelling(), "1.98.0");
    assert_eq!(reading.preflight().key().target(), reading.target());
    assert_eq!(
        reading.preflight().clock_attribution(),
        ClockAttribution::Unspecified
    );
    assert_eq!(
        reading.preflight().measurement(),
        ArchivedMeasurement::Unavailable
    );
    let (measured, worse, judgment) = match reading.outcome() {
        ArchivedBenchOutcome::PreflightRefused => {
            assert!(matches!(
                reading.preflight().attempt(),
                ArchivedAttempt::Executed(ArchivedConclusion::Refused(_))
            ));
            return Ok(());
        }
        ArchivedBenchOutcome::PlantedWorseNotDistinguished {
            measured,
            planted_worse,
            judgment,
        }
        | ArchivedBenchOutcome::PrimaryWorkRefused {
            measured,
            planted_worse,
            judgment,
        } => (measured, planted_worse, judgment),
        ArchivedBenchOutcome::Qualified {
            measured,
            planted_worse,
            judgment,
            secondary,
        } => {
            assert_eq!(secondary.work(), measured);
            assert_eq!(secondary.judgment(), judgment);
            assert_eq!(
                secondary.measurements(),
                &[ArchivedMeasurement::Unavailable; 6]
            );
            assert_eq!(secondary.clock_attribution(), ClockAttribution::Unspecified);
            (measured, planted_worse, judgment)
        }
    };
    assert_eq!(
        reading.preflight().attempt(),
        &ArchivedAttempt::Executed(ArchivedConclusion::Passed)
    );
    observe_curve(measured, axis, 1)?;
    observe_curve(worse, axis, 2)?;
    observe_judgment(judgment, stage)
}

fn observe_curve(
    curve: &ArchivedWorkCurve,
    axis: &[u64],
    exponent: u32,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(curve.points().len(), axis.len());
    for (point, size) in curve.points().iter().zip(axis) {
        assert_eq!(point.input_size(), *size);
        let [count] = point.counts() else {
            return Err("observation population".into());
        };
        assert_eq!(
            (count.observation().namespace(), count.observation().stem()),
            ("harness.bench.consumer", "unit-work")
        );
        let expected = size
            .checked_pow(exponent)
            .and_then(|units| units.checked_mul(2))
            .ok_or("expected work overflow")?;
        assert_eq!(count.count(), expected);
    }
    Ok(())
}

fn observe_judgment(
    judgment: &ArchivedWorkJudgment,
    stage: BenchStage,
) -> Result<(), Box<dyn Error>> {
    match stage {
        BenchStage::PlantedWorseNotDistinguished => {
            assert_eq!(judgment.measured(), &ArchivedWorkConclusion::Satisfied);
            assert_eq!(judgment.planted_worse(), &ArchivedWorkConclusion::Satisfied);
            let ArchivedWorkGap::NotDistinguished(cause) = judgment.gap() else {
                return Err("missing gap cause".into());
            };
            assert_eq!(
                (cause.family(), cause.local()),
                ("harness.bench.receiver", "declared-gap-not-observed")
            );
        }
        BenchStage::PrimaryWorkRefused => {
            let ArchivedWorkConclusion::Refused(cause) = judgment.measured() else {
                return Err("missing measured cause".into());
            };
            assert_eq!(
                (cause.family(), cause.local()),
                ("harness.bench.receiver", "measured-work-refused")
            );
        }
        BenchStage::Qualified => {
            assert_eq!(judgment.measured(), &ArchivedWorkConclusion::Satisfied);
        }
        BenchStage::PreflightRefused => return Err("unreachable preflight".into()),
    }
    Ok(())
}

#[test]
fn all_stages_match_independent_complete_bytes_and_owned_projections() -> Result<(), Box<dyn Error>>
{
    let report = fixture::all_stages()?;
    let trials = fixture::preflights(&report)?;
    let mut rows: Vec<_> = trials
        .into_iter()
        .zip(fixture::AXES)
        .map(|(trial, axis)| RowVector::qualified(trial, &axis))
        .collect();
    let [preflight, control, primary, _] = rows.as_mut_slice() else {
        return Err("fixture rows".into());
    };
    preflight.stage = 0;
    control.stage = 1;
    control.judgment = wire::judgment([
        None,
        None,
        Some(("harness.bench.receiver", "declared-gap-not-observed")),
    ]);
    primary.stage = 2;
    primary.judgment = wire::judgment([
        Some(("harness.bench.receiver", "measured-work-refused")),
        Some(("harness.bench.receiver", "planted-worse-refused")),
        None,
    ]);
    let expected = ReportVector::declared(rows).encoded();
    let record = retain_report(&report, LIMITS).map_err(|e| format!("retain: {e:?}"))?;
    assert_eq!(record.encoded(), expected);
    assert_eq!(
        record.address().as_bytes(),
        expected.get(..32).ok_or("address")?
    );
    for (historical, live) in record.readings().iter().zip(report.readings()) {
        assert_eq!(historical.row().key(), live.row().key().address());
        assert_eq!(
            historical.preflight().encoded(),
            macroonz_harness::report::archive::retain_trial(live.preflight(), LIMITS.bytes())
                .map_err(|e| format!("trial: {e:?}"))?
                .encoded()
        );
    }
    drop(report);
    observe_stages(&record)?;
    let mut input = expected;
    let loaded = read_report(&input, LIMITS).map_err(|e| format!("read: {e:?}"))?;
    input.fill(0);
    assert_eq!(loaded, record);
    observe_stages(&loaded)
}

#[test]
fn produced_and_unproduced_table_standing_remain_distinct_from_row_identity()
-> Result<(), Box<dyn Error>> {
    let mut row_key = None;
    for provenance in [Provenance::Unproduced, fixture::produced()?] {
        let binding = fixture::binding(
            &[2, 4, 8],
            super::super::fixture::lawful_judge,
            super::super::fixture::preflight_passes,
        )?;
        let report = fixture::report(vec![binding], provenance, HarnessClock::unavailable())?;
        let mut expected = fixture::vector()?;
        match provenance {
            Provenance::Unproduced => {}
            Provenance::Produced {
                producer: _,
                schema,
            } => {
                expected.provenance = vec![1];
                wire::name(b"outside", b"producer", &mut expected.provenance);
                wire::frame(schema.address().as_bytes(), &mut expected.provenance);
            }
        }
        let record = retain_report(&report, LIMITS).map_err(|e| format!("retain: {e:?}"))?;
        assert_eq!(record.encoded(), expected.encoded());
        let [reading] = record.readings() else {
            return Err("one row".into());
        };
        assert_eq!(
            reading.row().canonical_bytes(),
            super::types::DeclarationVector::lawful(&[2, 4, 8]).encoded()
        );
        assert_eq!(
            reading.row().key().as_bytes(),
            &blake3::derive_key(
                "macroonz/harness-identity/bench-row-key/v1",
                reading.row().canonical_bytes()
            )
        );
        if let Some(previous) = row_key {
            assert_eq!(reading.row().key(), previous);
        }
        row_key = Some(reading.row().key());
        expected.table.1 = b"another-table".to_vec();
        let renamed = read_report(&expected.encoded(), LIMITS).map_err(|e| format!("{e:?}"))?;
        assert_ne!(renamed.address(), record.address());
        assert_eq!(
            renamed.readings().first().ok_or("renamed row")?.row().key(),
            reading.row().key()
        );
    }
    Ok(())
}

#[test]
fn writer_and_reader_agree_at_exact_independent_resource_ceilings() -> Result<(), Box<dyn Error>> {
    let report = fixture::qualified()?;
    let vector = fixture::vector()?;
    let bytes = vector.encoded();
    let row = vector.rows.first().ok_or("row")?;
    let field = row.preflight.len().max(row.declaration.len());
    let exact =
        BenchArchiveLimits::declared(ArchiveLimits::declared(bytes.len(), field), 1, 3, 1, 6);
    let record = retain_report(&report, exact).map_err(|e| format!("exact retain: {e:?}"))?;
    assert_eq!(record.encoded(), bytes);
    assert_eq!(
        read_report(&bytes, exact).map_err(|e| format!("{e:?}"))?,
        record
    );
    for (limits, refusal) in [
        (
            BenchArchiveLimits::declared(
                ArchiveLimits::declared(bytes.len() - 1, field),
                1,
                3,
                1,
                6,
            ),
            BenchArchiveRefusal::Canonical(ArchiveRefusal::EnvelopeTooLarge),
        ),
        (
            BenchArchiveLimits::declared(
                ArchiveLimits::declared(bytes.len(), field - 1),
                1,
                3,
                1,
                6,
            ),
            BenchArchiveRefusal::Canonical(ArchiveRefusal::FieldTooLarge),
        ),
        (
            BenchArchiveLimits::declared(exact.bytes(), 0, 3, 1, 6),
            BenchArchiveRefusal::TooManyRows,
        ),
        (
            BenchArchiveLimits::declared(exact.bytes(), 1, 2, 1, 6),
            BenchArchiveRefusal::TooManySizes,
        ),
        (
            BenchArchiveLimits::declared(exact.bytes(), 1, 3, 0, 6),
            BenchArchiveRefusal::TooManyObservations,
        ),
        (
            BenchArchiveLimits::declared(exact.bytes(), 1, 3, 1, 5),
            BenchArchiveRefusal::TooManyMeasurements,
        ),
    ] {
        assert_eq!(retain_report(&report, limits), Err(refusal.clone()));
        assert_eq!(read_report(&bytes, limits), Err(refusal));
    }
    Ok(())
}
