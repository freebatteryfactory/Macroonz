//! Complete mutation-run order, census and independent population bounds.

use super::mutation_vector::MutationVector;
use super::vector::{frame, hash};
use macroonz_harness::muterprater::verdict_archive::{
    MutationArchiveRefusal, MutationRunArchiveLimits, read_mutation_run, retain_mutation_run,
};
use macroonz_harness::muterprater::{BackendVersionPosture, BaselineAxis, MutationVerdict};
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal};
use std::error::Error;
use std::io::Read as _;

const LIMITS: MutationRunArchiveLimits =
    MutationRunArchiveLimits::declared(ArchiveLimits::declared(65_536, 32_768), 8);

fn envelope(baseline: u8, count: u64, records: &[Vec<u8>]) -> Vec<u8> {
    let mut body = Vec::new();
    for word in [1u32, 2, 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    body.push(baseline);
    body.extend_from_slice(&count.to_be_bytes());
    for record in records {
        frame(record, &mut body);
    }
    let mut encoded = hash("historical-mutation-run/v1", &body).to_vec();
    encoded.extend_from_slice(&body);
    encoded
}

fn mixed_records() -> Vec<Vec<u8>> {
    let kill = MutationVector::demonstrated().encoded();
    let mut survivor = MutationVector::demonstrated();
    survivor.outcome = vec![1];
    let mut inconclusive = MutationVector::backend();
    inconclusive.outcome = vec![2, 5];
    inconclusive.baseline = 2;
    vec![
        kill.clone(),
        survivor.encoded(),
        inconclusive.encoded(),
        kill,
    ]
}

#[test]
fn independent_run_preserves_order_duplicates_and_per_record_baseline() -> Result<(), ()> {
    let records = mixed_records();
    let encoded = envelope(0, 4, &records);
    let run = read_mutation_run(&encoded, LIMITS).map_err(|_| ())?;
    assert_eq!(run.encoded(), encoded);
    assert_eq!(run.baseline(), BaselineAxis::Qualified);
    assert_eq!(run.reports().len(), 4);
    assert_eq!(run.census().pressed(), 4);
    assert_eq!(run.census().killed(), 2);
    assert_eq!(run.census().survived(), 1);
    assert_eq!(run.census().inconclusive(), 1);
    for (record, original) in run.reports().iter().zip(&records) {
        assert_eq!(record.encoded(), original);
    }
    let [first, _, third, last] = run.reports() else {
        return Err(());
    };
    assert_eq!(first, last);
    assert_eq!(third.baseline(), BaselineAxis::NotRun);
    assert_eq!(
        MutationVerdict::from(third.outcome()),
        MutationVerdict::Inconclusive
    );
    Ok(())
}

#[test]
fn empty_run_is_historical_qualified_baseline_without_fabricated_pressure() -> Result<(), ()> {
    let encoded = envelope(0, 0, &[]);
    let limits = MutationRunArchiveLimits::declared(ArchiveLimits::declared(encoded.len(), 0), 0);
    let run = read_mutation_run(&encoded, limits).map_err(|_| ())?;
    assert!(run.reports().is_empty());
    assert_eq!(run.census().pressed(), 0);
    assert_eq!(run.census().killed(), 0);
    assert_eq!(run.census().survived(), 0);
    assert_eq!(run.census().inconclusive(), 0);
    Ok(())
}

#[test]
fn run_population_refuses_before_walking_and_nested_records_keep_integrity() -> Result<(), ()> {
    assert_eq!(
        read_mutation_run(&envelope(0, 9, &[]), LIMITS),
        Err(MutationArchiveRefusal::TooManyReports)
    );
    for baseline in [1, 2] {
        assert_eq!(
            read_mutation_run(&envelope(baseline, 0, &[]), LIMITS),
            Err(MutationArchiveRefusal::BaselineNotQualified)
        );
    }
    assert_eq!(
        read_mutation_run(&envelope(9, 0, &[]), LIMITS),
        Err(MutationArchiveRefusal::Record(ArchiveRefusal::InvalidSlot))
    );
    let mut record = MutationVector::backend().encoded();
    *record.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_mutation_run(&envelope(0, 1, &[record]), LIMITS),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::AddressMismatch
        ))
    );
    assert_eq!(
        read_mutation_run(&envelope(0, 1, &[]), LIMITS),
        Err(MutationArchiveRefusal::Record(ArchiveRefusal::Truncated))
    );
    assert_eq!(
        read_mutation_run(
            &envelope(0, 0, &[MutationVector::backend().encoded()]),
            LIMITS
        ),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::TrailingBytes
        ))
    );
    Ok(())
}

#[test]
fn run_byte_and_population_ceilings_admit_exact_bounds() -> Result<(), ()> {
    let records = mixed_records();
    let encoded = envelope(0, 4, &records);
    let largest = records.iter().map(Vec::len).max().ok_or(())?;
    let exact =
        MutationRunArchiveLimits::declared(ArchiveLimits::declared(encoded.len(), largest), 4);
    assert_eq!(
        read_mutation_run(&encoded, exact)
            .map_err(|_| ())?
            .encoded(),
        encoded
    );
    for (limits, refusal) in [
        (
            MutationRunArchiveLimits::declared(
                ArchiveLimits::declared(encoded.len().saturating_sub(1), largest),
                4,
            ),
            MutationArchiveRefusal::Record(ArchiveRefusal::EnvelopeTooLarge),
        ),
        (
            MutationRunArchiveLimits::declared(
                ArchiveLimits::declared(encoded.len(), largest.saturating_sub(1)),
                4,
            ),
            MutationArchiveRefusal::Record(ArchiveRefusal::FieldTooLarge),
        ),
        (
            MutationRunArchiveLimits::declared(ArchiveLimits::declared(encoded.len(), largest), 3),
            MutationArchiveRefusal::TooManyReports,
        ),
    ] {
        assert_eq!(read_mutation_run(&encoded, limits), Err(refusal));
    }
    Ok(())
}

/// Synthetic console input exercises the live parser and record writer without claiming a backend process ran.
#[test]
fn parsed_backend_run_writer_keeps_derived_census_and_independent_limits() -> Result<(), ()> {
    let reading = macroonz_harness::muterprater::wrap::read_output(
        "ok Unmutated baseline\ncaught source.rs:1:1: replace true with false\nmissed source.rs:2:1: replace true with false\n",
        BackendVersionPosture::Unstated,
        |_| None,
        |_, _| None,
    ).map_err(|_| ())?;
    let original = reading.run();
    let archived = retain_mutation_run(original, LIMITS).map_err(|_| ())?;
    assert_eq!(archived.census(), original.census());
    assert_eq!(archived.reports().len(), 2);
    let exact = MutationRunArchiveLimits::declared(
        ArchiveLimits::declared(archived.encoded().len(), LIMITS.bytes().field()),
        original.reports().len(),
    );
    assert_eq!(
        retain_mutation_run(original, exact).map_err(|_| ())?,
        archived
    );
    assert_eq!(
        retain_mutation_run(
            original,
            MutationRunArchiveLimits::declared(LIMITS.bytes(), 1)
        ),
        Err(MutationArchiveRefusal::TooManyReports)
    );
    assert_eq!(
        retain_mutation_run(
            original,
            MutationRunArchiveLimits::declared(
                ArchiveLimits::declared(archived.encoded().len().saturating_sub(1), 32_768),
                2
            )
        ),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::EnvelopeTooLarge
        ))
    );
    assert_eq!(
        retain_mutation_run(
            original,
            MutationRunArchiveLimits::declared(ArchiveLimits::declared(65_536, 31), 2)
        ),
        Err(MutationArchiveRefusal::Record(
            ArchiveRefusal::FieldTooLarge
        ))
    );
    Ok(())
}

#[test]
fn complete_mutation_run_readback_owns_all_records_in_a_fresh_process() -> Result<(), Box<dyn Error>>
{
    let encoded = envelope(0, 4, &mixed_records());
    let returned = crate::archive_process::round_trip(
        &encoded,
        "archive::mutation_runs::mutation_run_archive_child",
    )?;
    assert_eq!(returned, encoded);
    Ok(())
}

#[test]
#[ignore = "invoked by the parent with a bounded complete historical mutation run"]
fn mutation_run_archive_child() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(65_537)
        .read_to_end(&mut encoded)?;
    let run = read_mutation_run(&encoded, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(encoded);
    crate::archive_process::publish(run.encoded())
}
