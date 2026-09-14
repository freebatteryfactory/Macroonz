//! Malformed but independently readdressed records challenge each historical boundary.

use super::{
    LIMITS, fixture,
    types::{DeclarationVector, ReportVector, RowVector},
    wire,
};
use macroonz_harness::bench::archive::{
    ArchivedBenchOutcome, ArchivedWorkConclusion, ArchivedWorkGap, BenchArchiveRefusal, read_report,
};
use macroonz_harness::bench::{DeclaredBudgetsRefusal, InputSizeAxisRefusal, WorkFormulaRefusal};
use macroonz_harness::report::archive::{ArchiveRefusal, read_trial};
use std::error::Error;

fn refused(vector: &ReportVector, expected: BenchArchiveRefusal) {
    assert_eq!(read_report(&vector.encoded(), LIMITS), Err(expected));
}

fn first(vector: &mut ReportVector) -> Result<&mut RowVector, Box<dyn Error>> {
    vector
        .rows
        .first_mut()
        .ok_or_else(|| "missing vector row".into())
}

#[test]
fn headers_integrity_truncation_and_trailing_data_refuse() -> Result<(), Box<dyn Error>> {
    let vector = fixture::vector()?;
    let bytes = vector.encoded();
    for length in 0..bytes.len() {
        assert!(read_report(bytes.get(..length).ok_or("prefix")?, LIMITS).is_err());
    }
    let body = vector.body();
    for length in 0..body.len() {
        let truncated = wire::address(body.get(..length).ok_or("body prefix")?);
        assert!(
            read_report(&truncated, LIMITS).is_err(),
            "readdressed prefix {length}"
        );
    }
    let mut damaged = bytes;
    *damaged.first_mut().ok_or("address")? ^= 1;
    assert_eq!(
        read_report(&damaged, LIMITS),
        Err(BenchArchiveRefusal::Canonical(
            ArchiveRefusal::AddressMismatch
        ))
    );
    let mut bad = vector.clone();
    bad.format = 2;
    refused(
        &bad,
        BenchArchiveRefusal::Canonical(ArchiveRefusal::UnsupportedFormat { found: 2 }),
    );
    bad = vector.clone();
    bad.kind = 2;
    refused(
        &bad,
        BenchArchiveRefusal::Canonical(ArchiveRefusal::WrongKind { found: 2 }),
    );
    bad = vector.clone();
    bad.custody = 1;
    refused(
        &bad,
        BenchArchiveRefusal::Canonical(ArchiveRefusal::UnsupportedCustody { found: 1 }),
    );
    let mut trailing = body;
    trailing.push(0);
    assert_eq!(
        read_report(&wire::address(&trailing), LIMITS),
        Err(BenchArchiveRefusal::Canonical(
            ArchiveRefusal::TrailingBytes
        ))
    );
    Ok(())
}

#[test]
fn declared_populations_refuse_before_missing_payload_is_walked() -> Result<(), Box<dyn Error>> {
    let mut vector = fixture::vector()?;
    vector.rows.clear();
    refused(&vector, BenchArchiveRefusal::EmptyReport);
    let mut body = vector.body();
    let count_start = body.len().checked_sub(8).ok_or("row count")?;
    body.truncate(count_start);
    body.extend_from_slice(&u64::MAX.to_be_bytes());
    assert_eq!(
        read_report(&wire::address(&body), LIMITS),
        Err(BenchArchiveRefusal::TooManyRows)
    );
    vector = fixture::vector()?;
    let mut declaration = Vec::new();
    wire::name(
        b"harness.bench.consumer",
        b"linear-workload",
        &mut declaration,
    );
    declaration.extend_from_slice(&u64::MAX.to_be_bytes());
    first(&mut vector)?.declaration = declaration;
    refused(&vector, BenchArchiveRefusal::TooManySizes);
    vector = fixture::vector()?;
    let mut measured = 3u64.to_be_bytes().to_vec();
    measured.extend_from_slice(&2u64.to_be_bytes());
    measured.extend_from_slice(&u64::MAX.to_be_bytes());
    first(&mut vector)?.measured = measured;
    refused(&vector, BenchArchiveRefusal::TooManyObservations);
    vector = fixture::vector()?;
    let row = first(&mut vector)?;
    row.secondary = row.measured.clone();
    row.secondary.extend_from_slice(&row.judgment);
    row.secondary.push(0);
    row.secondary.extend_from_slice(&u64::MAX.to_be_bytes());
    refused(&vector, BenchArchiveRefusal::TooManyMeasurements);
    Ok(())
}

#[test]
fn declaration_guards_keep_exact_owner_refusals() -> Result<(), Box<dyn Error>> {
    let base = fixture::vector()?;
    for (declaration, expected) in [
        (
            DeclarationVector {
                sizes: vec![2],
                ..DeclarationVector::lawful(&[2, 4, 8])
            },
            BenchArchiveRefusal::Axis(InputSizeAxisRefusal::TooShort { found: 1 }),
        ),
        (
            DeclarationVector {
                sizes: vec![2, 2],
                ..DeclarationVector::lawful(&[2, 4, 8])
            },
            BenchArchiveRefusal::Axis(InputSizeAxisRefusal::DuplicateSize {
                size: 2,
                first: 0,
                duplicate: 1,
            }),
        ),
        (
            DeclarationVector {
                samples: 0,
                ..DeclarationVector::lawful(&[2, 4, 8])
            },
            BenchArchiveRefusal::Budgets(DeclaredBudgetsRefusal::NoSamples),
        ),
        (
            DeclarationVector {
                ratio: (0, 1),
                ..DeclarationVector::lawful(&[2, 4, 8])
            },
            BenchArchiveRefusal::Budgets(DeclaredBudgetsRefusal::ZeroRatioNumerator),
        ),
        (
            DeclarationVector {
                ratio: (1, 0),
                ..DeclarationVector::lawful(&[2, 4, 8])
            },
            BenchArchiveRefusal::Budgets(DeclaredBudgetsRefusal::ZeroRatioDenominator),
        ),
        (
            DeclarationVector {
                contention: 1,
                ..DeclarationVector::lawful(&[2, 4, 8])
            },
            BenchArchiveRefusal::Canonical(ArchiveRefusal::InvalidSlot),
        ),
        (
            DeclarationVector {
                formula: Some(Vec::new()),
                ..DeclarationVector::lawful(&[2, 4, 8])
            },
            BenchArchiveRefusal::Formula(WorkFormulaRefusal::Empty),
        ),
    ] {
        let mut vector = base.clone();
        first(&mut vector)?.declaration = declaration.encoded();
        refused(&vector, expected);
    }
    let mut trailing = base;
    first(&mut trailing)?.declaration.push(0);
    refused(
        &trailing,
        BenchArchiveRefusal::Canonical(ArchiveRefusal::TrailingBytes),
    );
    Ok(())
}

#[test]
fn axis_observation_and_secondary_populations_cannot_drift() -> Result<(), Box<dyn Error>> {
    let base = fixture::vector()?;
    for points in [
        vec![],
        vec![(2, vec![("unit-work", 4)])],
        vec![
            (4, vec![("unit-work", 8)]),
            (2, vec![("unit-work", 4)]),
            (8, vec![("unit-work", 16)]),
        ],
        vec![
            (2, vec![]),
            (4, vec![("unit-work", 8)]),
            (8, vec![("unit-work", 16)]),
        ],
        vec![
            (2, vec![("unit-work", 4), ("unit-work", 0)]),
            (4, vec![("unit-work", 8)]),
            (8, vec![("unit-work", 16)]),
        ],
        vec![
            (2, vec![("unit-work", 4)]),
            (4, vec![("different-work", 8)]),
            (8, vec![("unit-work", 16)]),
        ],
    ] {
        let mut vector = base.clone();
        first(&mut vector)?.measured = wire::curve(&points);
        refused(&vector, BenchArchiveRefusal::WorkPopulationMismatch);
    }
    let different = wire::curve(
        &[2u64, 4, 8]
            .iter()
            .map(|size| (*size, vec![("different-work", 0)]))
            .collect::<Vec<_>>(),
    );
    for secondary in [false, true] {
        let mut vector = base.clone();
        let row = first(&mut vector)?;
        if secondary {
            row.secondary = wire::secondary(&different, &row.judgment, 0, &vec![vec![1]; 6]);
        } else {
            row.planted_worse = different.clone();
        }
        refused(&vector, BenchArchiveRefusal::WorkPopulationMismatch);
    }
    for count in [0, 5, 7] {
        let mut vector = base.clone();
        let row = first(&mut vector)?;
        row.secondary = wire::secondary(&row.measured, &row.judgment, 0, &vec![vec![1]; count]);
        refused(&vector, BenchArchiveRefusal::WorkPopulationMismatch);
    }
    Ok(())
}

#[test]
fn all_judgment_combinations_keep_precedence_and_exact_free_causes() -> Result<(), Box<dyn Error>> {
    let base = fixture::vector()?;
    for (measured_refuses, control_refuses, gap_distinguishes, expected_stage) in [
        (false, false, false, 1),
        (false, false, true, 1),
        (false, true, false, 1),
        (false, true, true, 3),
        (true, false, false, 1),
        (true, false, true, 1),
        (true, true, false, 1),
        (true, true, true, 2),
    ] {
        let mut vector = base.clone();
        let row = first(&mut vector)?;
        row.stage = expected_stage;
        row.judgment = wire::judgment([
            measured_refuses.then_some(("", "measured\0é")),
            control_refuses.then_some(("control\0é", "")),
            (!gap_distinguishes).then_some(("", "")),
        ]);
        let record =
            read_report(&vector.encoded(), LIMITS).map_err(|e| format!("matrix: {e:?}"))?;
        let reading = record.readings().first().ok_or("matrix row")?;
        let judgment = match reading.outcome() {
            ArchivedBenchOutcome::PlantedWorseNotDistinguished { judgment, .. }
            | ArchivedBenchOutcome::PrimaryWorkRefused { judgment, .. }
            | ArchivedBenchOutcome::Qualified { judgment, .. } => judgment,
            ArchivedBenchOutcome::PreflightRefused => return Err("matrix preflight".into()),
        };
        if let ArchivedWorkConclusion::Refused(cause) = judgment.measured() {
            assert_eq!((cause.family(), cause.local()), ("", "measured\0é"));
        } else {
            assert!(!measured_refuses);
        }
        if let ArchivedWorkConclusion::Refused(cause) = judgment.planted_worse() {
            assert_eq!((cause.family(), cause.local()), ("control\0é", ""));
        } else {
            assert!(!control_refuses);
        }
        if let ArchivedWorkGap::NotDistinguished(cause) = judgment.gap() {
            assert_eq!((cause.family(), cause.local()), ("", ""));
        } else {
            assert!(gap_distinguishes);
        }
        for wrong in [0, 1, 2, 3]
            .into_iter()
            .filter(|slot| *slot != expected_stage)
        {
            let mut wrong_vector = vector.clone();
            first(&mut wrong_vector)?.stage = wrong;
            refused(&wrong_vector, BenchArchiveRefusal::StageMismatch);
        }
    }
    Ok(())
}

#[test]
fn stage_and_measurement_slots_do_not_admit_invented_meaning() -> Result<(), Box<dyn Error>> {
    let base = fixture::vector()?;
    for judgment in [vec![9], vec![0, 9], vec![0, 0, 9]] {
        let mut vector = base.clone();
        first(&mut vector)?.judgment = judgment;
        refused(
            &vector,
            BenchArchiveRefusal::Canonical(ArchiveRefusal::InvalidSlot),
        );
    }
    let mut invalid_stage = base.clone();
    first(&mut invalid_stage)?.stage = 4;
    refused(
        &invalid_stage,
        BenchArchiveRefusal::Canonical(ArchiveRefusal::InvalidSlot),
    );
    for (attribution, measurement, expected) in [
        (3, vec![1], ArchiveRefusal::InvalidSlot),
        (0, vec![3], ArchiveRefusal::InvalidSlot),
        (0, vec![2, 5], ArchiveRefusal::InvalidSlot),
        (
            0,
            [
                vec![2, 4],
                7u64.to_be_bytes().to_vec(),
                7u64.to_be_bytes().to_vec(),
            ]
            .concat(),
            ArchiveRefusal::InvalidMeasurement,
        ),
        (
            0,
            [
                vec![2, 4],
                7u64.to_be_bytes().to_vec(),
                8u64.to_be_bytes().to_vec(),
            ]
            .concat(),
            ArchiveRefusal::InvalidMeasurement,
        ),
    ] {
        let mut vector = base.clone();
        let row = first(&mut vector)?;
        row.secondary = wire::secondary(
            &row.measured,
            &row.judgment,
            attribution,
            &vec![measurement; 6],
        );
        refused(&vector, BenchArchiveRefusal::Canonical(expected));
    }
    let mut invalid_secondary = base;
    let row = first(&mut invalid_secondary)?;
    row.secondary = wire::secondary(
        &row.measured,
        &wire::judgment([None, None, None]),
        0,
        &vec![vec![1]; 6],
    );
    refused(&invalid_secondary, BenchArchiveRefusal::StageMismatch);
    Ok(())
}

#[test]
fn nested_preflight_integrity_and_stage_are_independent_obligations() -> Result<(), Box<dyn Error>>
{
    let base = fixture::vector()?;
    let report = fixture::all_stages()?;
    let refusal = fixture::preflights(&report)?
        .into_iter()
        .next()
        .ok_or("refused preflight")?;
    let mut vector = base.clone();
    first(&mut vector)?.preflight = refusal;
    refused(&vector, BenchArchiveRefusal::StageMismatch);
    first(&mut vector)?.stage = 0;
    assert!(read_report(&vector.encoded(), LIMITS).is_ok());
    vector = base.clone();
    *first(&mut vector)?
        .preflight
        .first_mut()
        .ok_or("trial address")? ^= 1;
    refused(
        &vector,
        BenchArchiveRefusal::Canonical(ArchiveRefusal::AddressMismatch),
    );
    vector = base.clone();
    first(&mut vector)?.preflight.truncate(31);
    refused(
        &vector,
        BenchArchiveRefusal::Canonical(ArchiveRefusal::Truncated),
    );
    vector = base;
    first(&mut vector)?.target = b"another-target".to_vec();
    refused(&vector, BenchArchiveRefusal::TargetMismatch);
    Ok(())
}

#[test]
fn whole_table_targets_and_unique_rows_are_checked_after_nested_admission()
-> Result<(), Box<dyn Error>> {
    let base = fixture::vector()?;
    let row = base.rows.first().ok_or("row")?.clone();
    let duplicate = ReportVector::declared(vec![row.clone(), row.clone()]);
    refused(&duplicate, BenchArchiveRefusal::DuplicateRow);
    let mut foreign = row.clone();
    foreign.declaration = DeclarationVector::lawful(&[8, 4, 2]).encoded();
    foreign.measured = wire::lawful_curve(&[8, 4, 2], 1);
    foreign.planted_worse = wire::lawful_curve(&[8, 4, 2], 2);
    foreign.secondary = wire::secondary(&foreign.measured, &foreign.judgment, 0, &vec![vec![1]; 6]);
    let old = b"neutral-bench-target";
    let replacement = b"changed-bench-target";
    let mut body = foreign.preflight.get(32..).ok_or("trial body")?.to_vec();
    let positions: Vec<_> = body
        .windows(old.len())
        .enumerate()
        .filter_map(|(at, bytes)| (bytes == old).then_some(at))
        .collect();
    assert_eq!(positions.len(), 1);
    for at in positions {
        body.get_mut(at..at + old.len())
            .ok_or("trial target")?
            .copy_from_slice(replacement);
    }
    foreign.preflight = blake3::derive_key(
        "macroonz/harness-identity/historical-trial-report/v1",
        &body,
    )
    .to_vec();
    foreign.preflight.extend_from_slice(&body);
    foreign.target = replacement.to_vec();
    let trial = read_trial(&foreign.preflight, LIMITS.bytes())
        .map_err(|e| format!("foreign trial: {e:?}"))?;
    assert_eq!(
        trial.key().target().target().spelling(),
        "changed-bench-target"
    );
    assert!(
        read_report(
            &ReportVector::declared(vec![foreign.clone()]).encoded(),
            LIMITS
        )
        .is_ok()
    );
    refused(
        &ReportVector::declared(vec![row, foreign]),
        BenchArchiveRefusal::TargetMismatch,
    );
    Ok(())
}
