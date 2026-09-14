//! Complete-census retention, independent byte claims and hostile historical joins.

use super::{
    fixture, run_fixture,
    run_vector::{self, RunVector},
    trial_vector,
};
use macroonz_harness::clock::MeasurementReading;
use macroonz_harness::descriptor::{ExecutionSuite, Origin, StagedTableView, SynthesisFacts};
use macroonz_harness::report::archive::{
    ArchiveLimits, ArchiveRefusal, ArchivedDisposition, ArchivedTablePosture, RunArchiveLimits,
    read_run, retain_run, retain_trial,
};
use macroonz_harness::report::{
    EmptySelectionReason, HostTrialRecord, NotSelectedReason, RunAttempt, SelectionOutcome,
    SkipReason, TrialConclusion,
};
use macroonz_harness::runner::{Selection, SelectionPlan, record_all, run_all};
use std::collections::BTreeSet;

pub(super) const LIMITS: RunArchiveLimits =
    RunArchiveLimits::declared(ArchiveLimits::declared(32768, 16384), 8);

#[test]
fn independent_mixed_census_retains_order_all_coordinates_and_unselected_reasons() -> Result<(), ()>
{
    let vector = RunVector::mixed();
    let record = read_run(&vector.encoded(), LIMITS).map_err(|_| ())?;
    assert_eq!(
        record.address().as_bytes(),
        &super::vector::hash("historical-run-report/v1", &vector.body())
    );
    assert_eq!(
        record
            .census()
            .iter()
            .map(|row| *row.trial().as_bytes())
            .collect::<Vec<_>>(),
        vec![[9; 32], [1; 32], [11; 32]]
    );
    assert_eq!(
        record
            .census()
            .iter()
            .map(|row| *row.row().as_bytes())
            .collect::<Vec<_>>(),
        vec![[10; 32], [8; 32], [12; 32]]
    );
    for row in record.census() {
        assert_eq!(row.subject().as_bytes(), &[2; 32]);
        assert_eq!(row.check().as_bytes(), &[3; 32]);
        assert_eq!(row.claim().namespace(), "outside");
        assert_eq!(row.claim().stem(), "retention");
    }
    assert_eq!(
        record.census().first().ok_or(())?.disposition(),
        &ArchivedDisposition::NotSelected(NotSelectedReason::OutsideSelection)
    );
    assert_eq!(
        record.census().last().ok_or(())?.disposition(),
        &ArchivedDisposition::NotSelected(NotSelectedReason::SuiteNotRun)
    );
    let ArchivedDisposition::Selected(trial) = record.census().get(1).ok_or(())?.disposition()
    else {
        return Err(());
    };
    assert_eq!(
        trial.encoded(),
        trial_vector::envelope(&trial_vector::body(&[3], &[1]))
    );
    assert_eq!(trial.key().input(), record.input());
    assert_eq!(record.invocation(), trial.key().invocation());
    assert_eq!(record.target(), trial.key().target());
    assert_eq!(record.selection(), SelectionOutcome::Satisfied);
    let ArchivedTablePosture::Staged { parent } = record.posture() else {
        return Err(());
    };
    assert_eq!((parent.namespace(), parent.stem()), ("outside", "parent"));
    let mut reordered = RunVector::mixed();
    reordered.rows.reverse();
    let reordered = read_run(&reordered.encoded(), LIMITS).map_err(|_| ())?;
    assert_ne!(record.address(), reordered.address());
    assert_eq!(
        reordered.census().first().ok_or(())?.trial().as_bytes(),
        &[11; 32]
    );
    Ok(())
}

#[test]
fn actual_typed_authored_and_staged_runs_retain_each_complete_trial() -> Result<(), ()> {
    let report = run_fixture::typed()?;
    let record = retain_run(&report, LIMITS).map_err(|_| ())?;
    assert_eq!(record.posture(), &ArchivedTablePosture::Authored);
    assert_eq!(record.census().len(), 2);
    for (row, historical) in report.census().iter().zip(record.census()) {
        assert_eq!(historical.row().as_bytes(), row.row().address().as_bytes());
        assert_eq!(
            historical.subject().as_bytes(),
            row.revisions().subject().address().as_bytes()
        );
        assert_eq!(
            historical.check().as_bytes(),
            row.revisions().check().address().as_bytes()
        );
        let ArchivedDisposition::Selected(trial) = historical.disposition() else {
            return Err(());
        };
        assert_eq!(
            trial.as_ref(),
            &retain_trial(row.disposition().report().ok_or(())?, LIMITS.bytes()).map_err(|_| ())?
        );
    }
    assert_eq!(
        record.input().ok_or(())?.case().as_bytes(),
        report.input().ok_or(())?.case().address().as_bytes()
    );
    Ok(())
}

#[test]
fn actual_staged_overlay_retains_parent_and_authored_then_candidate_order() -> Result<(), ()> {
    let table = run_fixture::table(vec![run_fixture::binding(
        "authored",
        Origin::HandWritten,
        |_| TrialConclusion::Passed,
    )?])?;
    let staged = StagedTableView::staged(
        &table,
        vec![run_fixture::binding(
            "candidate",
            Origin::Candidate(SynthesisFacts::ProofGap),
            |_| TrialConclusion::Passed,
        )?],
    )
    .map_err(|_| ())?;
    let report = run_all(
        &staged.view(),
        &SelectionPlan::of(Selection::All),
        &fixture::invocation(),
    );
    let record = retain_run(&report, LIMITS).map_err(|_| ())?;
    let ArchivedTablePosture::Staged { parent } = record.posture() else {
        return Err(());
    };
    assert_eq!(
        (parent.namespace(), parent.stem()),
        ("archive-run", "world")
    );
    for (actual, historical) in report.census().iter().zip(record.census()) {
        assert_eq!(
            actual.trial().address().as_bytes(),
            historical.trial().as_bytes()
        );
    }
    assert_eq!(record.census().len(), 2);
    assert!(record.input().is_none());
    Ok(())
}

#[test]
fn empty_selection_preserves_input_even_without_any_census_rows() -> Result<(), ()> {
    for populated in [false, true] {
        let rows = if populated {
            vec![run_fixture::binding("unused", Origin::HandWritten, |_| {
                TrialConclusion::Passed
            })?]
        } else {
            Vec::new()
        };
        let table = run_fixture::table(rows)?;
        let invocation = fixture::typed_invocation(&[7])?;
        let plans = [
            SelectionPlan::of(Selection::ByTrialIds(BTreeSet::new())),
            SelectionPlan::allowing_empty(
                Selection::ByTrialIds(BTreeSet::new()),
                EmptySelectionReason::CarriedOverFromAPreviousRun,
            ),
            SelectionPlan::allowing_empty(
                Selection::ByTrialIds(BTreeSet::new()),
                EmptySelectionReason::AskingWhatTheWorldHolds,
            ),
        ];
        for plan in plans {
            let report = run_all(&table.view(), &plan, &invocation);
            let record = retain_run(&report, LIMITS).map_err(|_| ())?;
            assert_eq!(record.selection(), report.selection());
            assert_eq!(record.census().len(), usize::from(populated));
            assert_eq!(record.invocation(), report.invocation());
            assert_eq!(record.target(), report.target());
            assert_eq!(
                record.input().ok_or(())?.case().as_bytes(),
                report.input().ok_or(())?.case().address().as_bytes()
            );
        }
        let report = run_all(
            &table.view(),
            &SelectionPlan::of(Selection::ByExecutionSuite(BTreeSet::from([
                ExecutionSuite::named("archive-run", "absent").map_err(|_| ())?,
            ]))),
            &invocation,
        );
        let record = retain_run(&report, LIMITS).map_err(|_| ())?;
        for row in record.census() {
            assert_eq!(
                row.disposition(),
                &ArchivedDisposition::NotSelected(NotSelectedReason::SuiteNotRun)
            );
        }
    }
    Ok(())
}

#[test]
fn selected_host_skip_retains_satisfied_selection_without_inventing_execution() -> Result<(), ()> {
    let table = run_fixture::table(vec![run_fixture::binding(
        "host",
        Origin::HandWritten,
        |_| TrialConclusion::Passed,
    )?])?;
    let invocation = fixture::invocation();
    let plan = SelectionPlan::of(Selection::All);
    let executed = run_all(&table.view(), &plan, &invocation);
    let row = executed.census().first().ok_or(())?;
    let report = record_all(
        &table.view(),
        &plan,
        &invocation,
        vec![HostTrialRecord::recorded(
            row.trial(),
            RunAttempt::SkippedWithReason(SkipReason::PrerequisiteAbsent),
            MeasurementReading::Unavailable,
        )],
    )
    .map_err(|_| ())?;
    let record = retain_run(&report, LIMITS).map_err(|_| ())?;
    assert_eq!(record.selection(), SelectionOutcome::Satisfied);
    let ArchivedDisposition::Selected(trial) = record.census().first().ok_or(())?.disposition()
    else {
        return Err(());
    };
    assert_eq!(
        trial.attempt(),
        &macroonz_harness::report::archive::ArchivedAttempt::SkippedWithReason(
            SkipReason::PrerequisiteAbsent
        )
    );
    Ok(())
}

#[test]
fn row_and_byte_limits_apply_independently_to_writer_and_reader() -> Result<(), ()> {
    let report = run_fixture::typed()?;
    let record = retain_run(&report, LIMITS).map_err(|_| ())?;
    let exact = RunArchiveLimits::declared(
        ArchiveLimits::declared(record.encoded().len(), LIMITS.bytes().field()),
        2,
    );
    assert_eq!(retain_run(&report, exact).map_err(|_| ())?, record);
    assert_eq!(read_run(record.encoded(), exact).map_err(|_| ())?, record);
    for (limits, refusal) in [
        (
            RunArchiveLimits::declared(exact.bytes(), 1),
            ArchiveRefusal::TooManyRows,
        ),
        (
            RunArchiveLimits::declared(
                ArchiveLimits::declared(
                    record.encoded().len().saturating_sub(1),
                    exact.bytes().field(),
                ),
                2,
            ),
            ArchiveRefusal::EnvelopeTooLarge,
        ),
        (
            RunArchiveLimits::declared(ArchiveLimits::declared(32768, 31), 2),
            ArchiveRefusal::FieldTooLarge,
        ),
    ] {
        assert_eq!(retain_run(&report, limits), Err(refusal));
        assert_eq!(read_run(record.encoded(), limits), Err(refusal));
    }
    let mut vector = RunVector::mixed();
    vector.count = u64::MAX;
    assert_eq!(
        read_run(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::TooManyRows)
    );
    vector.count = 4;
    assert_eq!(
        read_run(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::Truncated)
    );
    vector.count = 2;
    assert_eq!(
        read_run(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    Ok(())
}

#[test]
fn recomputed_integrity_cannot_hide_duplicate_trials_or_selected_revision_disagreement()
-> Result<(), ()> {
    let mut duplicate = RunVector::mixed();
    duplicate.rows.push(run_vector::row(1, 99, 1, &[]));
    duplicate.count = 4;
    assert_eq!(
        read_run(&duplicate.encoded(), LIMITS),
        Err(ArchiveRefusal::DuplicateTrial)
    );
    for offset in [8, 88, 128] {
        let mut vector = RunVector::mixed();
        *vector
            .rows
            .get_mut(1)
            .ok_or(())?
            .get_mut(offset)
            .ok_or(())? ^= 1;
        assert_eq!(
            read_run(&vector.encoded(), LIMITS),
            Err(ArchiveRefusal::IdentityJoinMismatch)
        );
    }
    let mut vector = RunVector::mixed();
    *vector.rows.get_mut(1).ok_or(())?.get_mut(48).ok_or(())? ^= 1;
    assert!(
        read_run(&vector.encoded(), LIMITS).is_ok(),
        "a row revision is a claim without its descriptor preimage"
    );
    Ok(())
}

#[test]
fn every_run_context_coordinate_must_agree_with_selected_trial_standing() -> Result<(), ()> {
    for offset in [0, 4, 12, 28, 44] {
        let mut vector = RunVector::mixed();
        *vector.context.get_mut(offset).ok_or(())? ^= 1;
        assert_eq!(
            read_run(&vector.encoded(), LIMITS),
            Err(ArchiveRefusal::RunContextMismatch)
        );
    }
    for offset in [8, 48, 80] {
        let mut vector = RunVector::mixed();
        *vector
            .coordinates
            .as_mut()
            .ok_or(())?
            .get_mut(offset)
            .ok_or(())? ^= 1;
        assert_eq!(
            read_run(&vector.encoded(), LIMITS),
            Err(ArchiveRefusal::RunContextMismatch)
        );
    }
    for offset in [8, 23, 28, 40] {
        let mut vector = RunVector::mixed();
        *vector.metadata.get_mut(offset).ok_or(())? ^= 1;
        assert_eq!(
            read_run(&vector.encoded(), LIMITS),
            Err(ArchiveRefusal::RunContextMismatch)
        );
    }
    let mut unit = RunVector::mixed();
    unit.coordinates = None;
    assert_eq!(
        read_run(&unit.encoded(), LIMITS),
        Err(ArchiveRefusal::RunContextMismatch)
    );
    for slot in 1..=3 {
        let mut vector = RunVector::mixed();
        vector.selection = slot;
        assert_eq!(
            read_run(&vector.encoded(), LIMITS),
            Err(ArchiveRefusal::SelectionMismatch)
        );
    }
    unit.rows.clear();
    unit.count = 0;
    assert_eq!(
        read_run(&unit.encoded(), LIMITS),
        Err(ArchiveRefusal::SelectionMismatch)
    );
    Ok(())
}

#[test]
fn malformed_run_frames_and_nested_trials_refuse_under_recomputed_addresses() -> Result<(), ()> {
    let pristine = RunVector::mixed();
    let complete = pristine.body();
    for length in 0..complete.len() {
        assert!(
            read_run(
                &run_vector::envelope(complete.get(..length).ok_or(())?),
                LIMITS
            )
            .is_err()
        );
    }
    for (offset, value, refusal) in [
        (0, 2u32, ArchiveRefusal::UnsupportedFormat { found: 2 }),
        (4, 2, ArchiveRefusal::WrongKind { found: 2 }),
        (8, 1, ArchiveRefusal::UnsupportedCustody { found: 1 }),
    ] {
        let mut body = pristine.body();
        body.get_mut(offset..offset + 4)
            .ok_or(())?
            .copy_from_slice(&value.to_be_bytes());
        assert_eq!(read_run(&run_vector::envelope(&body), LIMITS), Err(refusal));
    }
    let mut corrupt = pristine.encoded();
    *corrupt.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_run(&corrupt, LIMITS),
        Err(ArchiveRefusal::AddressMismatch)
    );
    let mut vector = RunVector::mixed();
    vector.selection = 9;
    assert_eq!(
        read_run(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::InvalidSlot)
    );
    vector = RunVector::mixed();
    vector.posture = vec![9];
    assert_eq!(
        read_run(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::InvalidSlot)
    );
    vector = RunVector::mixed();
    vector.context.push(0);
    assert_eq!(
        read_run(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    vector = RunVector::mixed();
    vector.coordinates.as_mut().ok_or(())?.push(0);
    assert_eq!(
        read_run(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    vector = RunVector::mixed();
    vector.rows.first_mut().ok_or(())?.push(0);
    assert_eq!(
        read_run(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    vector = RunVector::mixed();
    let mut trial = trial_vector::body(&[3], &[1]);
    trial.push(0);
    *vector.rows.get_mut(1).ok_or(())? = run_vector::row(1, 8, 0, &trial_vector::envelope(&trial));
    assert_eq!(
        read_run(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::TrailingBytes)
    );
    Ok(())
}

#[test]
fn complete_rows_are_fields_and_names_cannot_be_empty_or_non_utf8() -> Result<(), ()> {
    let independent = RunVector::mixed();
    let largest = independent.rows.iter().map(Vec::len).max().ok_or(())?;
    let vector_exact = RunArchiveLimits::declared(ArchiveLimits::declared(32768, largest), 3);
    assert!(read_run(&independent.encoded(), vector_exact).is_ok());
    let vector_short =
        RunArchiveLimits::declared(ArchiveLimits::declared(32768, largest.saturating_sub(1)), 3);
    assert_eq!(
        read_run(&independent.encoded(), vector_short),
        Err(ArchiveRefusal::FieldTooLarge)
    );
    let report = run_fixture::typed()?;
    let trial = report
        .census()
        .first()
        .ok_or(())?
        .disposition()
        .report()
        .ok_or(())?;
    let trial = retain_trial(trial, LIMITS.bytes()).map_err(|_| ())?;
    let claim = report.census().first().ok_or(())?.claim().name();
    let row_bytes = 185
        + claim.namespace().written().len()
        + claim.stem().written().len()
        + trial.encoded().len();
    let exact = RunArchiveLimits::declared(ArchiveLimits::declared(32768, row_bytes), 2);
    let record = retain_run(&report, exact).map_err(|_| ())?;
    let short = RunArchiveLimits::declared(
        ArchiveLimits::declared(32768, row_bytes.saturating_sub(1)),
        2,
    );
    assert_eq!(
        retain_run(&report, short),
        Err(ArchiveRefusal::FieldTooLarge)
    );
    assert_eq!(
        read_run(record.encoded(), short),
        Err(ArchiveRefusal::FieldTooLarge)
    );
    for (namespace, stem) in [
        (b"".as_slice(), b"name".as_slice()),
        (b"owner".as_slice(), b"".as_slice()),
        (&[0xff], b"name".as_slice()),
        (b"owner".as_slice(), &[0xff]),
    ] {
        let mut names = Vec::new();
        super::vector::frame(namespace, &mut names);
        super::vector::frame(stem, &mut names);
        let mut vector = RunVector::mixed();
        vector.posture = vec![1];
        vector.posture.extend_from_slice(&names);
        assert_eq!(
            read_run(&vector.encoded(), LIMITS),
            Err(ArchiveRefusal::InvalidText)
        );
        vector = RunVector::mixed();
        let mut row = vector
            .rows
            .first()
            .ok_or(())?
            .get(..160)
            .ok_or(())?
            .to_vec();
        row.extend_from_slice(&names);
        row.push(1);
        *vector.rows.first_mut().ok_or(())? = row;
        assert_eq!(
            read_run(&vector.encoded(), LIMITS),
            Err(ArchiveRefusal::InvalidText)
        );
    }
    let mut vector = RunVector::mixed();
    *vector.rows.first_mut().ok_or(())?.last_mut().ok_or(())? = 9;
    assert_eq!(
        read_run(&vector.encoded(), LIMITS),
        Err(ArchiveRefusal::InvalidSlot)
    );
    let mut body = RunVector::mixed().body();
    body.get_mut(12..20)
        .ok_or(())?
        .copy_from_slice(&u64::MAX.to_be_bytes());
    assert!(read_run(&run_vector::envelope(&body), LIMITS).is_err());
    Ok(())
}
