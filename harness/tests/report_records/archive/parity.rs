//! Independent parity integrity, role joins and authority ceilings.

use super::binding_vector::{name, revision};
use super::parity_vector::{ParityVector, envelope, finding};
use super::vector::{InputKind, Vector, frame};
use macroonz_harness::descriptor::archive::BindingArchiveLimits;
use macroonz_harness::muterprater::ParityQualificationRefusal;
use macroonz_harness::muterprater::interpretation_archive::{
    ArchivedParityDisposition, ArchivedSubstrate, ParityArchiveLimits, ParityArchiveRefusal,
    ValueRole, read_parity,
};
use macroonz_harness::report::archive::{
    ArchiveLimits, ArchiveRefusal, ArchivedConclusion, ArchivedMeasurement,
};
use macroonz_harness::report::{ReplayPosture, TextFidelity};
use std::error::Error;
use std::io::Read as _;

const LIMITS: ParityArchiveLimits = ParityArchiveLimits::declared(
    ArchiveLimits::declared(32768, 8192),
    BindingArchiveLimits::declared(8192, 256, 8),
    ArchiveLimits::declared(8192, 4096),
    256,
    8,
);

#[test]
fn complete_independent_vector_retains_each_role_and_exact_owned_bytes() -> Result<(), ()> {
    let vector = ParityVector::declared();
    let bytes = vector.encoded();
    let record = read_parity(&bytes, LIMITS).map_err(|_| ())?;
    assert_eq!(record.encoded(), bytes);
    drop(bytes);
    assert_eq!(
        record.production_report().key().trial().as_bytes(),
        &vector.witness.trial()
    );
    assert_eq!(record.pair().family().namespace(), "evaluation");
    assert_eq!(record.pair().family().stem(), "family");
    assert_eq!(record.pair().production_revision().revision(), &[5; 32]);
    assert_eq!(record.pair().evaluation_revision().revision(), &[6; 32]);
    assert_eq!(record.pair().surface().as_bytes(), &[7; 32]);
    assert_eq!(record.witness().subject_revision().revision(), &[2; 32]);
    assert_eq!(record.witness().check_revision().revision(), &[3; 32]);
    assert_eq!(record.input().bytes(), &[10]);
    assert_eq!(record.production().bytes(), &[11]);
    assert_eq!(record.evaluation().bytes(), &[12]);
    assert_eq!(
        record.production().convention(),
        record.evaluation().convention()
    );
    assert_eq!(record.input().convention().name().namespace(), "encoding");
    assert_eq!(record.input().convention().name().stem(), "bytes");
    assert_eq!(record.input().convention().version(), 3);
    assert_eq!(record.input().convention().schema().as_bytes(), &[8; 32]);
    assert_eq!(record.input().convention().revision().revision(), &[9; 32]);
    assert_eq!(
        record.production_report().key(),
        record.evaluation_report().key()
    );
    assert!(record.production_report().key().input().is_none());
    assert_eq!(
        record.production_report().claimed_posture(),
        ReplayPosture::DeclaredByAuthor
    );
    assert_eq!(
        record.production_report().measurement(),
        ArchivedMeasurement::Unavailable
    );
    assert_eq!(record.conclusion(), &ArchivedConclusion::Passed);
    assert_eq!(record.disposition(), ArchivedParityDisposition::Qualified);
    let ArchivedSubstrate::Standing(roster) = record.substrate() else {
        return Err(());
    };
    assert_eq!(
        roster
            .names()
            .iter()
            .map(|name| (name.namespace(), name.stem()))
            .collect::<Vec<_>>(),
        [("a", "base"), ("b", "base")]
    );
    Ok(())
}

#[test]
fn every_rejection_and_combined_failure_preserves_first_refusal_and_full_finding() -> Result<(), ()>
{
    for (production, evaluation, firings, disagreed, expected, slot) in [
        (
            true,
            true,
            4u32,
            true,
            ParityQualificationRefusal::ProductionDidNotQualify,
            0u8,
        ),
        (
            false,
            true,
            4,
            true,
            ParityQualificationRefusal::EvaluationDidNotQualify,
            1,
        ),
        (
            false,
            false,
            4,
            true,
            ParityQualificationRefusal::NoMutationActivated { firings: 4 },
            2,
        ),
        (
            false,
            false,
            0,
            true,
            ParityQualificationRefusal::MeaningsDisagreed,
            3,
        ),
    ] {
        let mut vector = ParityVector::declared();
        let refused = finding(&vector.witness.trial());
        if production {
            vector.production_report.attempt.clone_from(&refused);
        }
        if evaluation {
            vector.evaluation_report.attempt.clone_from(&refused);
        }
        if disagreed {
            vector.conclusion.clone_from(&refused);
        }
        vector.firings = firings;
        vector.disposition = vec![2, slot];
        if slot == 2 {
            vector.disposition.extend_from_slice(&firings.to_be_bytes());
        }
        let record = read_parity(&vector.encoded(), LIMITS).map_err(|_| ())?;
        assert_eq!(
            record.disposition(),
            ArchivedParityDisposition::Rejected(expected)
        );
        assert_eq!(record.evaluation_firings(), firings);
        let ArchivedConclusion::Refused(finding) = record.conclusion() else {
            return Err(());
        };
        assert_eq!(finding.file(), "check.rs");
        assert_eq!(finding.line(), 29);
        let foreign = finding.foreign().ok_or(())?;
        assert_eq!(foreign.bytes(), &[b'x', 0xff]);
        assert_eq!(foreign.fidelity(), TextFidelity::LossyReplacement);
        vector.disposition = vec![1];
        assert_eq!(
            read_parity(&vector.encoded(), LIMITS),
            Err(ParityArchiveRefusal::DispositionMismatch)
        );
        vector.disposition = vec![0];
        assert_eq!(
            read_parity(&vector.encoded(), LIMITS)
                .map_err(|_| ())?
                .disposition(),
            ArchivedParityDisposition::Raw
        );
    }
    Ok(())
}

#[test]
fn all_nonpassing_attempts_remain_nonqualifying_and_measurement_does_not_decide() -> Result<(), ()>
{
    for attempt in [
        vec![2, 0],
        vec![2, 1],
        vec![2, 2],
        vec![2, 3],
        vec![3],
        vec![4, 0, 0],
        vec![4, 5, 0],
    ] {
        for production in [true, false] {
            let mut vector = ParityVector::declared();
            if production {
                vector.production_report.attempt.clone_from(&attempt);
            } else {
                vector.evaluation_report.attempt.clone_from(&attempt);
            }
            assert_eq!(
                read_parity(&vector.encoded(), LIMITS),
                Err(ParityArchiveRefusal::DispositionMismatch)
            );
            vector.disposition = vec![2, u8::from(!production)];
            assert!(read_parity(&vector.encoded(), LIMITS).is_ok());
        }
    }
    let mut vector = ParityVector::declared();
    vector.production_report.measurement = vec![2, 0];
    vector.evaluation_report.measurement = vec![0];
    vector
        .evaluation_report
        .measurement
        .extend_from_slice(&57u64.to_be_bytes());
    let record = read_parity(&vector.encoded(), LIMITS).map_err(|_| ())?;
    assert_ne!(
        record.production_report().measurement(),
        record.evaluation_report().measurement()
    );
    assert_eq!(record.disposition(), ArchivedParityDisposition::Qualified);
    Ok(())
}

#[test]
fn recomputed_integrity_cannot_hide_any_witness_report_join() -> Result<(), ()> {
    for production in [true, false] {
        for offset in [8usize, 48, 88, 123, 131, 139, 148] {
            let mut vector = ParityVector::declared();
            let report = if production {
                &mut vector.production_report
            } else {
                &mut vector.evaluation_report
            };
            *report.key.get_mut(offset).ok_or(())? ^= 1;
            assert!(
                read_parity(&vector.encoded(), LIMITS).is_err(),
                "key role={production} offset={offset}"
            );
        }
        let mut relocated = ParityVector::declared();
        let relocated_report = if production {
            &mut relocated.production_report
        } else {
            &mut relocated.evaluation_report
        };
        *relocated_report.site.last_mut().ok_or(())? ^= 1;
        assert_eq!(
            read_parity(&relocated.encoded(), LIMITS),
            Err(ParityArchiveRefusal::ReportJoinMismatch)
        );
        for posture in [0u8, 2] {
            let mut vector = ParityVector::declared();
            let report = if production {
                &mut vector.production_report
            } else {
                &mut vector.evaluation_report
            };
            report.posture = posture;
            assert_eq!(
                read_parity(&vector.encoded(), LIMITS),
                Err(ParityArchiveRefusal::ReportJoinMismatch)
            );
        }
    }
    let mut foreign_finding = ParityVector::declared();
    foreign_finding.conclusion = finding(&[99; 32]);
    foreign_finding.disposition = vec![0];
    assert_eq!(
        read_parity(&foreign_finding.encoded(), LIMITS),
        Err(ParityArchiveRefusal::Record(
            ArchiveRefusal::IdentityJoinMismatch
        ))
    );
    for both in [true, false] {
        let mut vector = ParityVector::declared();
        let typed = Vector::declared(InputKind::Bound);
        let mut key = Vec::new();
        frame(&vector.witness.trial(), &mut key);
        key.extend(typed.key.into_iter().skip(40));
        vector.production_report.key.clone_from(&key);
        vector
            .production_report
            .metadata
            .clone_from(&typed.metadata);
        if both {
            vector.evaluation_report.key = key;
            vector.evaluation_report.metadata = typed.metadata;
        }
        assert_eq!(
            read_parity(&vector.encoded(), LIMITS),
            Err(ParityArchiveRefusal::ReportJoinMismatch)
        );
    }
    Ok(())
}

#[test]
fn only_four_semantic_coordinates_move_the_trial_and_every_posture_meet_is_exact() -> Result<(), ()>
{
    let original = ParityVector::declared();
    for coordinate in [0usize, 2, 3, 4] {
        let mut vector = ParityVector::declared();
        let changed = (b"changed".as_slice(), b"coordinate".as_slice());
        *vector.witness.row.names.get_mut(coordinate).ok_or(())? = changed;
        if coordinate == 2 {
            vector.witness.subject = changed;
        }
        if coordinate == 3 {
            vector.witness.check = changed;
        }
        assert_ne!(vector.witness.trial(), original.witness.trial());
        assert_eq!(
            read_parity(&vector.encoded(), LIMITS),
            Err(ParityArchiveRefusal::ReportJoinMismatch)
        );
        let trial = vector.witness.trial();
        for report in [&mut vector.production_report, &mut vector.evaluation_report] {
            report.key.get_mut(8..40).ok_or(())?.copy_from_slice(&trial);
        }
        assert!(read_parity(&vector.encoded(), LIMITS).is_ok());
    }
    let mut moved = ParityVector::declared();
    *moved.witness.row.names.get_mut(1).ok_or(())? = (b"elsewhere", b"suite");
    moved.witness.row.roles.clear();
    moved.witness.row.tags.clear();
    moved.witness.row.origin = vec![1];
    assert_eq!(moved.witness.trial(), original.witness.trial());
    assert!(read_parity(&moved.encoded(), LIMITS).is_ok());
    for subject in [0u8, 1, 2] {
        for check in [0u8, 1, 2] {
            let mut vector = ParityVector::declared();
            vector.witness.subject_revision = revision(&[2; 32], subject);
            vector.witness.check_revision = revision(&[3; 32], check);
            vector.production_report.posture = subject.max(check);
            vector.evaluation_report.posture = subject.max(check);
            assert!(read_parity(&vector.encoded(), LIMITS).is_ok());
        }
    }
    Ok(())
}

#[test]
fn bounded_canonical_substrates_cannot_silently_become_independence() -> Result<(), ()> {
    let mut vector = ParityVector::declared();
    vector.substrate = vec![0];
    assert_eq!(
        read_parity(&vector.encoded(), LIMITS)
            .map_err(|_| ())?
            .substrate(),
        &ArchivedSubstrate::DeclaredIndependent
    );
    for names in [
        vec![],
        vec![(b"a".as_slice(), b"base".as_slice()); 2],
        vec![
            (b"b".as_slice(), b"base".as_slice()),
            (b"a".as_slice(), b"base".as_slice()),
        ],
    ] {
        vector.substrate = vec![1];
        vector
            .substrate
            .extend_from_slice(&u64::try_from(names.len()).map_err(|_| ())?.to_be_bytes());
        for substrate in names {
            name(substrate, &mut vector.substrate);
        }
        assert_eq!(
            read_parity(&vector.encoded(), LIMITS),
            Err(ParityArchiveRefusal::InvalidSubstrate)
        );
    }
    vector.substrate = vec![1];
    vector.substrate.extend_from_slice(&9u64.to_be_bytes());
    assert_eq!(
        read_parity(&vector.encoded(), LIMITS),
        Err(ParityArchiveRefusal::TooManySubstrates)
    );
    Ok(())
}

#[test]
fn every_truncated_prefix_and_each_resource_axis_refuses() -> Result<(), ()> {
    let original = ParityVector::declared();
    let body = original.body();
    for length in 0..body.len() {
        assert!(read_parity(&envelope(body.get(..length).ok_or(())?), LIMITS).is_err());
    }
    let bytes = original.encoded();
    let exact = ParityArchiveLimits::declared(
        ArchiveLimits::declared(bytes.len(), 8192),
        LIMITS.binding(),
        LIMITS.trial(),
        1,
        2,
    );
    assert!(read_parity(&bytes, exact).is_ok());
    let all_limits = [
        ParityArchiveLimits::declared(
            ArchiveLimits::declared(bytes.len().saturating_sub(1), 8192),
            LIMITS.binding(),
            LIMITS.trial(),
            256,
            8,
        ),
        ParityArchiveLimits::declared(
            ArchiveLimits::declared(32768, 31),
            LIMITS.binding(),
            LIMITS.trial(),
            256,
            8,
        ),
        ParityArchiveLimits::declared(
            LIMITS.bytes(),
            BindingArchiveLimits::declared(1, 256, 8),
            LIMITS.trial(),
            256,
            8,
        ),
        ParityArchiveLimits::declared(
            LIMITS.bytes(),
            LIMITS.binding(),
            ArchiveLimits::declared(1, 4096),
            256,
            8,
        ),
        ParityArchiveLimits::declared(LIMITS.bytes(), LIMITS.binding(), LIMITS.trial(), 256, 1),
    ];
    for limits in all_limits {
        assert!(read_parity(&bytes, limits).is_err());
    }
    for (role, offered) in [
        (ValueRole::Input, 0u8),
        (ValueRole::Production, 1),
        (ValueRole::Evaluation, 2),
    ] {
        let mut vector = ParityVector::declared();
        vector.input.clear();
        vector.production.clear();
        vector.evaluation.clear();
        match offered {
            0 => vector.input.push(1),
            1 => vector.production.push(1),
            _ => vector.evaluation.push(1),
        }
        let limits =
            ParityArchiveLimits::declared(LIMITS.bytes(), LIMITS.binding(), LIMITS.trial(), 0, 8);
        assert_eq!(
            read_parity(&vector.encoded(), limits),
            Err(ParityArchiveRefusal::ValueTooLarge { role })
        );
    }
    Ok(())
}

#[test]
fn unsupported_headers_slots_names_and_trailing_material_refuse() -> Result<(), ()> {
    for (at, value, cause) in [
        (0usize, 2u32, ArchiveRefusal::UnsupportedFormat { found: 2 }),
        (1, 2, ArchiveRefusal::WrongKind { found: 2 }),
        (2, 1, ArchiveRefusal::UnsupportedCustody { found: 1 }),
    ] {
        let mut vector = ParityVector::declared();
        *vector.header.get_mut(at).ok_or(())? = value;
        assert_eq!(
            read_parity(&vector.encoded(), LIMITS),
            Err(ParityArchiveRefusal::Record(cause))
        );
    }
    for at in [0u8, 1, 2] {
        let mut vector = ParityVector::declared();
        match at {
            0 => vector.substrate = vec![9],
            1 => vector.conclusion = vec![9],
            _ => vector.disposition = vec![9],
        }
        assert_eq!(
            read_parity(&vector.encoded(), LIMITS),
            Err(ParityArchiveRefusal::Record(ArchiveRefusal::InvalidSlot))
        );
    }
    for invalid in [b"".as_slice(), &[0xff]] {
        for role in [0u8, 1, 2] {
            let mut vector = ParityVector::declared();
            let mut prefix = Vec::new();
            name((invalid, b"bytes"), &mut prefix);
            match role {
                0 => vector.pair = prefix,
                1 => vector.input_convention = prefix,
                _ => vector.meaning_convention = prefix,
            }
            assert_eq!(
                read_parity(&vector.encoded(), LIMITS),
                Err(ParityArchiveRefusal::Record(ArchiveRefusal::InvalidText))
            );
        }
    }
    let mut body = ParityVector::declared().body();
    body.push(0);
    assert_eq!(
        read_parity(&envelope(&body), LIMITS),
        Err(ParityArchiveRefusal::Record(ArchiveRefusal::TrailingBytes))
    );
    let mut bytes = ParityVector::declared().encoded();
    *bytes.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_parity(&bytes, LIMITS),
        Err(ParityArchiveRefusal::Record(
            ArchiveRefusal::AddressMismatch
        ))
    );
    Ok(())
}

#[test]
#[ignore = "driven by the historical parity process-boundary claim"]
fn child_loads_parity() -> Result<(), Box<dyn Error>> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take(32769)
        .read_to_end(&mut bytes)?;
    let record =
        read_parity(&bytes, LIMITS).map_err(|cause| std::io::Error::other(format!("{cause:?}")))?;
    drop(bytes);
    super::process::publish(record.encoded())
}

#[test]
fn historical_parity_survives_owned_readback_in_a_fresh_process() -> Result<(), Box<dyn Error>> {
    for disposition in [vec![0], vec![1], vec![2, 3]] {
        let mut vector = ParityVector::declared();
        if disposition == [2, 3] {
            vector.conclusion = finding(&vector.witness.trial());
        }
        vector.disposition = disposition;
        let bytes = vector.encoded();
        assert_eq!(
            super::process::round_trip(&bytes, "archive::parity::child_loads_parity")?,
            bytes
        );
    }
    Ok(())
}
