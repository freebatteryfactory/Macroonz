//! Historical candidate fields, bounded canonical grammar and outside authority refusals.

use super::{
    candidate_vector::{CandidateVector, Name, Opening},
    process, run_fixture,
};
use macroonz_harness::descriptor::archive::{
    ArchivedName, ArchivedSynthesis, CandidateArchiveLimits, CandidateArchiveRefusal,
    read_candidate, retain_candidate,
};
use macroonz_harness::descriptor::{MutationPointRef, NamespacedName, Origin, SynthesisFacts};
use macroonz_harness::report::TrialConclusion;
use macroonz_harness::report::archive::ArchivedName as ReportName;
use std::error::Error;
use std::io::Read as _;

const LIMITS: CandidateArchiveLimits = CandidateArchiveLimits::declared(4096, 64, 8);

fn reading(name: &ArchivedName) -> (&str, &str) {
    (name.namespace(), name.stem())
}

fn current(name: NamespacedName) -> (&'static str, &'static str) {
    (name.namespace().written(), name.stem().written())
}

#[test]
fn independent_candidates_retain_every_field_and_both_synthesis_arms() -> Result<(), ()> {
    for opening in [Opening::Gap, Opening::Survivor] {
        let survivor = matches!(opening, Opening::Survivor);
        let bytes = CandidateVector::declared(opening).encoded();
        let record = read_candidate(&bytes, LIMITS).map_err(|_| ())?;
        assert_eq!(record.canonical_bytes(), bytes);
        drop(bytes);
        assert_eq!(reading(record.claim()), ("données", "keeps-λ"));
        assert_eq!(
            reading(record.execution_suite()),
            ("execution-owner", "suite")
        );
        assert_eq!(reading(record.subject()), ("subject-owner", "route"));
        assert_eq!(reading(record.check()), ("check-owner", "law"));
        assert_eq!(reading(record.population()), ("population-owner", "bytes"));
        assert_eq!(
            record.roles().iter().map(reading).collect::<Vec<_>>(),
            [("a", "z"), ("b", "a")]
        );
        assert_eq!(
            record.tags().iter().map(reading).collect::<Vec<_>>(),
            [("n", "a"), ("n", "b")]
        );
        match record.synthesis() {
            ArchivedSynthesis::Survivor(point) => {
                assert!(survivor);
                assert_eq!(reading(point), ("mutation-owner", "point"));
            }
            ArchivedSynthesis::ProofGap => assert!(!survivor),
        }
        let old_path: &ReportName = record.claim();
        assert_eq!(reading(old_path), ("données", "keeps-λ"));
    }
    Ok(())
}

#[test]
fn actual_candidates_retain_the_existing_row_bytes_and_readable_coordinates() -> Result<(), ()> {
    let point = MutationPointRef::named("mutation-owner", "point").map_err(|_| ())?;
    for facts in [SynthesisFacts::ProofGap, SynthesisFacts::Survivor(point)] {
        let binding = run_fixture::binding::<()>("candidate", Origin::Candidate(facts), |_| {
            TrialConclusion::Passed
        })?;
        let row = binding.row();
        let record = retain_candidate(row, LIMITS).map_err(|_| ())?;
        assert_eq!(record.canonical_bytes(), row.canonical_bytes().as_bytes());
        for (historical, declared) in [
            (record.claim(), row.claim().name()),
            (record.execution_suite(), row.execution_suite().name()),
            (record.subject(), row.subject().name()),
            (record.check(), row.check().name()),
            (record.population(), row.population().name()),
        ] {
            assert_eq!(reading(historical), current(declared));
        }
        assert_eq!(
            record.roles().iter().map(reading).collect::<Vec<_>>(),
            row.roles()
                .iter()
                .map(|role| current(role.name()))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            record.tags().iter().map(reading).collect::<Vec<_>>(),
            row.tags()
                .iter()
                .map(|tag| current(tag.name()))
                .collect::<Vec<_>>()
        );
        match (facts, record.synthesis()) {
            (SynthesisFacts::ProofGap, ArchivedSynthesis::ProofGap) => {}
            (SynthesisFacts::Survivor(declared), ArchivedSynthesis::Survivor(historical)) => {
                assert_eq!(reading(historical), current(declared.name()));
            }
            _ => return Err(()),
        }
        let loaded = read_candidate(record.canonical_bytes(), LIMITS).map_err(|_| ())?;
        drop(binding);
        assert_eq!(loaded, record);
    }
    let authored = super::super::row()?;
    assert_eq!(
        retain_candidate(&authored, LIMITS),
        Err(CandidateArchiveRefusal::NotCandidate { found: 1 })
    );
    Ok(())
}

#[test]
fn byte_field_and_roster_bounds_apply_to_retention_and_reading() -> Result<(), ()> {
    let binding = run_fixture::binding::<()>(
        "candidate",
        Origin::Candidate(SynthesisFacts::ProofGap),
        |_| TrialConclusion::Passed,
    )?;
    let row = binding.row();
    let bytes = row.canonical_bytes().as_bytes();
    let record = retain_candidate(row, LIMITS).map_err(|_| ())?;
    let widest = [
        record.claim(),
        record.execution_suite(),
        record.subject(),
        record.check(),
        record.population(),
    ]
    .into_iter()
    .chain(record.roles())
    .chain(record.tags())
    .flat_map(|name| [name.namespace().len(), name.stem().len()])
    .max()
    .ok_or(())?;
    let exact = CandidateArchiveLimits::declared(bytes.len(), widest, 1);
    assert_eq!(exact.bytes(), bytes.len());
    assert_eq!(exact.field(), widest);
    assert_eq!(exact.labels(), 1);
    assert_eq!(retain_candidate(row, exact).map_err(|_| ())?, record);
    assert_eq!(read_candidate(bytes, exact).map_err(|_| ())?, record);
    for (limits, refusal) in [
        (
            CandidateArchiveLimits::declared(bytes.len().saturating_sub(1), widest, 1),
            CandidateArchiveRefusal::BytesTooLarge,
        ),
        (
            CandidateArchiveLimits::declared(bytes.len(), widest.saturating_sub(1), 1),
            CandidateArchiveRefusal::FieldTooLarge,
        ),
        (
            CandidateArchiveLimits::declared(bytes.len(), widest, 0),
            CandidateArchiveRefusal::TooManyLabels,
        ),
    ] {
        assert_eq!(retain_candidate(row, limits), Err(refusal));
        assert_eq!(read_candidate(bytes, limits), Err(refusal));
    }
    let mut empty = CandidateVector::declared(Opening::Gap);
    empty.roles.clear();
    empty.tags.clear();
    assert!(
        read_candidate(
            &empty.encoded(),
            CandidateArchiveLimits::declared(4096, 64, 0)
        )
        .is_ok()
    );
    for tags in [false, true] {
        let mut vector = CandidateVector::declared(Opening::Gap);
        if tags {
            vector.roles.clear();
        } else {
            vector.tags.clear();
        }
        assert_eq!(
            read_candidate(
                &vector.encoded(),
                CandidateArchiveLimits::declared(4096, 64, 1)
            ),
            Err(CandidateArchiveRefusal::TooManyLabels)
        );
    }
    Ok(())
}

#[test]
fn duplicate_or_reordered_labels_refuse_in_both_rosters() -> Result<(), ()> {
    for tags in [false, true] {
        for duplicate in [false, true] {
            let mut vector = CandidateVector::declared(Opening::Gap);
            let labels = if tags {
                &mut vector.tags
            } else {
                &mut vector.roles
            };
            if duplicate {
                labels.push(*labels.last().ok_or(())?);
            } else {
                labels.reverse();
            }
            assert_eq!(
                read_candidate(&vector.encoded(), LIMITS),
                Err(CandidateArchiveRefusal::NonCanonicalLabels)
            );
        }
    }
    Ok(())
}

#[test]
fn names_are_required_utf8_without_a_static_or_ascii_restriction() -> Result<(), ()> {
    for bad in [b"".as_slice(), &[0xff]] {
        for namespace in [false, true] {
            let invalid = if namespace {
                (bad, b"name".as_slice())
            } else {
                (b"owner".as_slice(), bad)
            };
            assert_invalid_name(invalid)?;
        }
    }
    Ok(())
}

fn assert_invalid_name(invalid: Name) -> Result<(), ()> {
    for field in 0..5 {
        let mut vector = CandidateVector::declared(Opening::Gap);
        *vector.names.get_mut(field).ok_or(())? = invalid;
        assert_eq!(
            read_candidate(&vector.encoded(), LIMITS),
            Err(CandidateArchiveRefusal::InvalidName)
        );
    }
    for tags in [false, true] {
        let mut vector = CandidateVector::declared(Opening::Gap);
        let labels = if tags {
            &mut vector.tags
        } else {
            &mut vector.roles
        };
        *labels.first_mut().ok_or(())? = invalid;
        assert_eq!(
            read_candidate(&vector.encoded(), LIMITS),
            Err(CandidateArchiveRefusal::InvalidName)
        );
    }
    let mut vector = CandidateVector::declared(Opening::Survivor);
    vector.origin = vec![3, 1];
    super::vector::frame(invalid.0, &mut vector.origin);
    super::vector::frame(invalid.1, &mut vector.origin);
    assert_eq!(
        read_candidate(&vector.encoded(), LIMITS),
        Err(CandidateArchiveRefusal::InvalidName)
    );
    Ok(())
}

#[test]
fn versions_origins_synthesis_truncation_and_trailing_bytes_refuse() -> Result<(), ()> {
    for opening in [Opening::Gap, Opening::Survivor] {
        let vector = CandidateVector::declared(opening);
        let bytes = vector.encoded();
        for length in 0..bytes.len() {
            assert!(read_candidate(bytes.get(..length).ok_or(())?, LIMITS).is_err());
        }
        let mut trailing = bytes;
        trailing.push(0);
        assert_eq!(
            read_candidate(&trailing, LIMITS),
            Err(CandidateArchiveRefusal::TrailingBytes)
        );
    }
    let mut vector = CandidateVector::declared(Opening::Gap);
    vector.version = 3;
    assert_eq!(
        read_candidate(&vector.encoded(), LIMITS),
        Err(CandidateArchiveRefusal::UnsupportedFormat { found: 3 })
    );
    vector.version = 2;
    for found in [0, 1, 2, 4, 5, 255] {
        vector.origin = vec![found];
        assert_eq!(
            read_candidate(&vector.encoded(), LIMITS),
            Err(CandidateArchiveRefusal::NotCandidate { found })
        );
    }
    for slot in [0, 3, 255] {
        vector.origin = vec![3, slot];
        assert_eq!(
            read_candidate(&vector.encoded(), LIMITS),
            Err(CandidateArchiveRefusal::InvalidSynthesis)
        );
    }
    let mut excess_count = vector.prefix();
    excess_count.extend_from_slice(&9u64.to_be_bytes());
    assert_eq!(
        read_candidate(&excess_count, LIMITS),
        Err(CandidateArchiveRefusal::TooManyLabels)
    );
    let mut huge_count = vector.prefix();
    huge_count.extend_from_slice(&u64::MAX.to_be_bytes());
    assert!(matches!(
        read_candidate(&huge_count, LIMITS),
        Err(CandidateArchiveRefusal::TooManyLabels
            | CandidateArchiveRefusal::LengthOutsidePlatform { declared: u64::MAX })
    ));
    let mut huge_field = 2u32.to_be_bytes().to_vec();
    huge_field.extend_from_slice(&u64::MAX.to_be_bytes());
    assert!(read_candidate(&huge_field, LIMITS).is_err());
    Ok(())
}

#[test]
#[ignore = "driven by the candidate process-boundary claim"]
fn child_loads_candidate() -> Result<(), Box<dyn Error>> {
    let mut encoded = Vec::new();
    std::io::stdin()
        .lock()
        .take(4097)
        .read_to_end(&mut encoded)?;
    let record = read_candidate(&encoded, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(encoded);
    assert!(!record.claim().namespace().is_empty());
    process::publish(record.canonical_bytes())
}

#[test]
fn actual_and_independent_candidates_survive_a_fresh_process() -> Result<(), Box<dyn Error>> {
    for opening in [Opening::Gap, Opening::Survivor] {
        let encoded = CandidateVector::declared(opening).encoded();
        assert_eq!(
            process::round_trip(&encoded, "archive::candidates::child_loads_candidate")?,
            encoded
        );
    }
    let binding = run_fixture::binding::<()>(
        "candidate",
        Origin::Candidate(SynthesisFacts::ProofGap),
        |_| TrialConclusion::Passed,
    )
    .map_err(|()| std::io::Error::other("candidate fixture refused"))?;
    let record = retain_candidate(binding.row(), LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(binding);
    assert_eq!(
        process::round_trip(
            record.canonical_bytes(),
            "archive::candidates::child_loads_candidate"
        )?,
        record.canonical_bytes()
    );
    Ok(())
}
