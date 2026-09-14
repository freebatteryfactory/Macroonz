//! Complete historical origins are read independently of live admission and canonical writers.

use super::candidate_vector::{CandidateVector, Opening};
use super::{process, vector::frame};
use macroonz_harness::descriptor::archive::{
    ArchivedName, ArchivedOrigin, CandidateArchiveRefusal, RowArchiveLimits, RowArchiveRefusal,
    read_candidate, read_row, retain_row,
};
use macroonz_harness::descriptor::{
    DoorRef, Origin, ProducerFacts, ProjectionRef, ReplayBearingGround, Row, SynthesisFacts,
};
use std::error::Error;
use std::io::Read as _;

const LIMITS: RowArchiveLimits = RowArchiveLimits::declared(4096, 64, 8);

fn name(namespace: &[u8], stem: &[u8], into: &mut Vec<u8>) {
    frame(namespace, into);
    frame(stem, into);
}

fn reading(value: &ArchivedName) -> (&str, &str) {
    (value.namespace(), value.stem())
}

fn origins() -> Vec<Vec<u8>> {
    let mut generated = vec![2];
    name("généré".as_bytes(), b"door", &mut generated);
    name(b"projection-owner", b"shape", &mut generated);
    let mut origins = vec![
        vec![1],
        generated,
        CandidateVector::declared(Opening::Gap).origin,
        CandidateVector::declared(Opening::Survivor).origin,
    ];
    for ground in [1u8, 2] {
        let mut replay = vec![4];
        frame(&[19; 32], &mut replay);
        replay.push(ground);
        name(b"admission-owner", b"different-destination", &mut replay);
        frame(&[23; 32], &mut replay);
        origins.push(replay);
    }
    let mut discharge = vec![5];
    frame(&[29; 32], &mut discharge);
    name(b"admission-owner", b"discharged", &mut discharge);
    origins.push(discharge);
    origins
}

fn encoded(origin: Vec<u8>) -> Vec<u8> {
    let mut vector = CandidateVector::declared(Opening::Gap);
    vector.origin = origin;
    vector.encoded()
}

#[test]
fn every_origin_retains_its_complete_fields_without_acquiring_live_authority() -> Result<(), ()> {
    for origin in origins() {
        let slot = *origin.first().ok_or(())?;
        let expected_ground = match origin.get(41).copied() {
            Some(1) => Some(ReplayBearingGround::MutantKilled),
            Some(2) => Some(ReplayBearingGround::ClaimPinned),
            _ => None,
        };
        let bytes = encoded(origin);
        let record = read_row(&bytes, LIMITS).map_err(|_| ())?;
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
        match record.origin() {
            ArchivedOrigin::HandWritten => assert_eq!(slot, 1),
            ArchivedOrigin::Generated { door, projection } => {
                assert_eq!(slot, 2);
                assert_eq!(reading(door), ("généré", "door"));
                assert_eq!(reading(projection), ("projection-owner", "shape"));
            }
            ArchivedOrigin::Candidate(synthesis) => {
                assert_eq!(slot, 3);
                let candidate = read_candidate(record.canonical_bytes(), LIMITS).map_err(|_| ())?;
                assert_eq!(candidate.synthesis(), synthesis);
                assert_eq!(candidate.canonical_bytes(), record.canonical_bytes());
            }
            ArchivedOrigin::AdmittedReplay {
                proposal,
                ground,
                destination,
                replay,
            } => {
                assert_eq!(slot, 4);
                assert_eq!(proposal, &[19; 32]);
                assert_eq!(Some(*ground), expected_ground);
                assert_eq!(
                    reading(destination),
                    ("admission-owner", "different-destination")
                );
                assert_ne!(destination, record.execution_suite());
                assert_eq!(replay, &[23; 32]);
            }
            ArchivedOrigin::AdmittedDischarge {
                proposal,
                destination,
            } => {
                assert_eq!(slot, 5);
                assert_eq!(proposal, &[29; 32]);
                assert_eq!(reading(destination), ("admission-owner", "discharged"));
            }
        }
        if slot != 3 {
            assert_eq!(
                read_candidate(record.canonical_bytes(), LIMITS),
                Err(CandidateArchiveRefusal::NotCandidate { found: slot })
            );
        }
    }
    Ok(())
}

#[test]
fn existing_live_rows_retain_their_original_canonical_bytes() -> Result<(), ()> {
    let producer = ProducerFacts::emitted(
        DoorRef::named("outside", "door").map_err(|_| ())?,
        ProjectionRef::named("outside", "projection").map_err(|_| ())?,
    );
    for origin in [
        Origin::HandWritten,
        Origin::Generated(producer),
        Origin::Candidate(SynthesisFacts::ProofGap),
    ] {
        let template = super::super::row()?;
        let row = Row::declared(
            template.claim(),
            template.execution_suite(),
            template.classification().clone(),
            template.subject(),
            template.check(),
            template.population(),
            origin,
        )
        .map_err(|_| ())?;
        let retained = retain_row(&row, LIMITS).map_err(|_| ())?;
        assert_eq!(retained.canonical_bytes(), row.canonical_bytes().as_bytes());
        let loaded = read_row(retained.canonical_bytes(), LIMITS).map_err(|_| ())?;
        drop(row);
        assert_eq!(loaded, retained);
    }
    Ok(())
}

#[test]
fn complete_rows_refuse_malformed_origins_and_replay_grounds() {
    for found in [0u8, 6, 255] {
        assert_eq!(
            read_row(&encoded(vec![found]), LIMITS),
            Err(RowArchiveRefusal::InvalidOrigin { found })
        );
    }
    for found in [0u8, 3, 255] {
        let mut origin = vec![4];
        frame(&[19; 32], &mut origin);
        origin.push(found);
        assert_eq!(
            read_row(&encoded(origin), LIMITS),
            Err(RowArchiveRefusal::InvalidReplayGround { found })
        );
    }
    for width in [0usize, 31, 33] {
        for replay in [false, true] {
            let mut origin = vec![4];
            frame(&vec![19; if replay { 32 } else { width }], &mut origin);
            origin.push(1);
            name(b"owner", b"suite", &mut origin);
            frame(&vec![23; if replay { width } else { 32 }], &mut origin);
            assert_eq!(
                read_row(&encoded(origin), LIMITS),
                Err(RowArchiveRefusal::InvalidAddressWidth)
            );
        }
    }
}

#[test]
fn every_origin_requires_complete_canonical_framing_and_independent_bounds() -> Result<(), ()> {
    for origin in origins() {
        let bytes = encoded(origin);
        for length in 0..bytes.len() {
            assert!(read_row(bytes.get(..length).ok_or(())?, LIMITS).is_err());
        }
        assert!(read_row(&bytes, RowArchiveLimits::declared(bytes.len(), 64, 2)).is_ok());
        for (limits, cause) in [
            (
                RowArchiveLimits::declared(bytes.len().saturating_sub(1), 64, 2),
                CandidateArchiveRefusal::BytesTooLarge,
            ),
            (
                RowArchiveLimits::declared(bytes.len(), 0, 2),
                CandidateArchiveRefusal::FieldTooLarge,
            ),
            (
                RowArchiveLimits::declared(bytes.len(), 64, 1),
                CandidateArchiveRefusal::TooManyLabels,
            ),
        ] {
            assert_eq!(
                read_row(&bytes, limits),
                Err(RowArchiveRefusal::Canonical(cause))
            );
        }
        let mut trailing = bytes;
        trailing.push(0);
        assert_eq!(
            read_row(&trailing, LIMITS),
            Err(RowArchiveRefusal::Canonical(
                CandidateArchiveRefusal::TrailingBytes
            ))
        );
    }
    Ok(())
}

#[test]
fn historical_address_fields_obey_the_field_ceiling_before_width_admission() -> Result<(), ()> {
    let replay = origins()
        .into_iter()
        .find(|origin| origin.first() == Some(&4))
        .ok_or(())?;
    assert_eq!(
        read_row(&encoded(replay), RowArchiveLimits::declared(4096, 31, 8)),
        Err(RowArchiveRefusal::Canonical(
            CandidateArchiveRefusal::FieldTooLarge
        ))
    );
    Ok(())
}

#[test]
#[ignore = "driven by the complete row process-boundary claim"]
fn child_loads_complete_row() -> Result<(), Box<dyn Error>> {
    let mut bytes = Vec::new();
    std::io::stdin().lock().take(4097).read_to_end(&mut bytes)?;
    let record = read_row(&bytes, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(bytes);
    process::publish(record.canonical_bytes())
}

#[test]
fn new_origin_names_obey_the_same_utf8_and_nonempty_boundary() {
    for invalid in [b"".as_slice(), &[0xff]] {
        for (slot, second_generated, component) in [
            (2u8, false, false),
            (2, false, true),
            (2, true, false),
            (2, true, true),
            (4, false, false),
            (4, false, true),
            (5, false, false),
            (5, false, true),
        ] {
            let mut origin = vec![slot];
            if slot != 2 {
                frame(&[19; 32], &mut origin);
            }
            if slot == 4 {
                origin.push(1);
            }
            if second_generated {
                name(b"owner", b"door", &mut origin);
            }
            if component {
                name(invalid, b"stem", &mut origin);
            } else {
                name(b"owner", invalid, &mut origin);
            }
            assert_eq!(
                read_row(&encoded(origin), LIMITS),
                Err(RowArchiveRefusal::Canonical(
                    CandidateArchiveRefusal::InvalidName
                ))
            );
        }
    }
}

#[test]
fn complete_rows_keep_shared_version_label_and_count_refusals() {
    let mut vector = CandidateVector::declared(Opening::Gap);
    vector.origin = vec![1];
    vector.version = 9;
    assert_eq!(
        read_row(&vector.encoded(), LIMITS),
        Err(RowArchiveRefusal::Canonical(
            CandidateArchiveRefusal::UnsupportedFormat { found: 9 }
        ))
    );
    vector.version = 2;
    for tags in [false, true] {
        for duplicate in [false, true] {
            let labels = if tags {
                &mut vector.tags
            } else {
                &mut vector.roles
            };
            *labels = if duplicate {
                vec![(b"a", b"b"), (b"a", b"b")]
            } else {
                vec![(b"b", b"a"), (b"a", b"b")]
            };
            assert_eq!(
                read_row(&vector.encoded(), LIMITS),
                Err(RowArchiveRefusal::Canonical(
                    CandidateArchiveRefusal::NonCanonicalLabels
                ))
            );
            if tags {
                vector.tags.clear();
            } else {
                vector.roles.clear();
            }
        }
    }
    let mut excess = vector.prefix();
    excess.extend_from_slice(&9u64.to_be_bytes());
    assert_eq!(
        read_row(&excess, LIMITS),
        Err(RowArchiveRefusal::Canonical(
            CandidateArchiveRefusal::TooManyLabels
        ))
    );
}

#[test]
fn every_historical_origin_survives_owned_readback_in_a_fresh_process() -> Result<(), Box<dyn Error>>
{
    for origin in origins() {
        let bytes = encoded(origin);
        assert_eq!(
            process::round_trip(&bytes, "archive::rows::child_loads_complete_row")?,
            bytes
        );
    }
    Ok(())
}
