//! Arbitrary declared formula bytes, zero counts and authored roster order survive actual retention.

use super::super::fixture as receiver;
use super::{
    LIMITS, fixture,
    types::{DeclarationVector, ReportVector, RowVector},
    wire,
};
use macroonz_harness::bench::archive::{
    ArchivedBenchOutcome, ArchivedWorkConclusion, retain_report,
};
use macroonz_harness::bench::{
    BenchBinding, BenchMeasurement, BenchReferences, BenchRow, ContentionPosture, DeclaredBudgets,
    InputSizeAxis, WorkConclusion, WorkFormula, WorkGapStanding, WorkJudgment, WorkJudgmentInput,
    WorkObservationRef,
};
use macroonz_harness::clock::HarnessClock;
use macroonz_harness::descriptor::Provenance;
use macroonz_harness::report::FindingCause;
use std::error::Error;

fn exact_causes(_input: &WorkJudgmentInput<'_>) -> WorkJudgment {
    WorkJudgment::stated(
        WorkConclusion::Refused(FindingCause::named("", "measured\0é")),
        WorkConclusion::Refused(FindingCause::named("control\0é", "")),
        WorkGapStanding::Distinguished,
    )
}

#[test]
fn actual_rows_keep_binary_or_absent_formula_zero_sizes_counts_and_authored_order()
-> Result<(), Box<dyn Error>> {
    for (formula, warmups) in [(None, 0), (Some(vec![0xff, 0, 7]), 2)] {
        let report = declared_report(formula.as_deref(), warmups)?;
        let trial = fixture::preflights(&report)?
            .into_iter()
            .next()
            .ok_or("preflight")?;
        let mut vector_row = RowVector::qualified(trial, &[8, 0, 2]);
        vector_row.declaration = DeclarationVector {
            sizes: vec![8, 0, 2],
            samples: 3,
            warmups,
            ratio: (9, 7),
            contention: 0,
            formula: formula.clone(),
        }
        .encoded();
        vector_row.stage = 2;
        vector_row.measured = wire::curve(
            &[8, 0, 2].map(|size| (size, vec![("zero-work", 0), ("unit-work", size * 3)])),
        );
        vector_row.planted_worse = wire::curve(
            &[8, 0, 2].map(|size| (size, vec![("zero-work", 0), ("unit-work", size * size * 3)])),
        );
        vector_row.judgment =
            wire::judgment([Some(("", "measured\0é")), Some(("control\0é", "")), None]);
        let record = retain_report(&report, LIMITS).map_err(|e| format!("retain: {e:?}"))?;
        assert_eq!(
            record.encoded(),
            ReportVector::declared(vec![vector_row]).encoded()
        );
        let reading = record.readings().first().ok_or("reading")?;
        assert_eq!(
            reading
                .row()
                .measurement()
                .formula()
                .map(WorkFormula::bytes),
            formula.as_deref()
        );
        assert_eq!(reading.row().measurement().budgets().samples(), 3);
        assert_eq!(reading.row().measurement().budgets().warmups(), warmups);
        assert_eq!(reading.row().measurement().budgets().ratio().numerator(), 9);
        assert_eq!(
            reading.row().measurement().budgets().ratio().denominator(),
            7
        );
        let ArchivedBenchOutcome::PrimaryWorkRefused {
            measured, judgment, ..
        } = reading.outcome()
        else {
            return Err("primary refusal".into());
        };
        for (point, size) in measured.points().iter().zip([8, 0, 2]) {
            assert_eq!(point.input_size(), size);
            let [zero, work] = point.counts() else {
                return Err("roster".into());
            };
            assert_eq!((zero.observation().stem(), zero.count()), ("zero-work", 0));
            assert_eq!(
                (work.observation().stem(), work.count()),
                ("unit-work", size * 3)
            );
        }
        let ArchivedWorkConclusion::Refused(cause) = judgment.measured() else {
            return Err("cause".into());
        };
        assert_eq!((cause.family(), cause.local()), ("", "measured\0é"));
    }
    Ok(())
}

fn declared_report(
    formula: Option<&[u8]>,
    warmups: u32,
) -> Result<macroonz_harness::bench::BenchReport, Box<dyn Error>> {
    let template = receiver::lawful_row().map_err(|e| format!("{e:?}"))?;
    let measurement = BenchMeasurement::declared(
        InputSizeAxis::declared(vec![8, 0, 2]).map_err(|e| format!("{e:?}"))?,
        DeclaredBudgets::declared(3, warmups, 9, 7).map_err(|e| format!("{e:?}"))?,
        ContentionPosture::NoDeclaredContention,
        formula
            .map(<[u8]>::to_vec)
            .map(WorkFormula::encoded)
            .transpose()
            .map_err(|e| format!("{e:?}"))?,
    );
    let row = BenchRow::declared(
        BenchReferences::declared(
            template.workload(),
            template.preflight(),
            template.planted_worse(),
            template.complexity(),
        ),
        measurement,
    )
    .map_err(|e| format!("{e:?}"))?;
    let observations = vec![
        WorkObservationRef::named("harness.bench.consumer", "zero-work")
            .map_err(|e| format!("{e:?}"))?,
        WorkObservationRef::named("harness.bench.consumer", "unit-work")
            .map_err(|e| format!("{e:?}"))?,
    ];
    let attachment = receiver::attachment_with_refs(
        row.workload(),
        row.planted_worse(),
        row.complexity(),
        receiver::measured,
        receiver::planted_worse,
        exact_causes,
        observations,
    )
    .map_err(|e| format!("{e:?}"))?;
    let preflight =
        receiver::lawful_preflight(receiver::preflight_passes).map_err(|e| format!("{e:?}"))?;
    let binding = BenchBinding::bound(row, attachment, preflight).map_err(|e| format!("{e:?}"))?;
    fixture::report(
        vec![binding],
        Provenance::Unproduced,
        HarnessClock::unavailable(),
    )
}
