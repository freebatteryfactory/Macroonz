//! Historical binding fields, bounds and semantic joins observed from outside.

use super::binding_vector::{BindingVector, name, revision};
use super::vector::frame;
use macroonz_harness::descriptor::archive::{
    ArchivedProvenance, BindingArchiveLimits, BindingArchiveRefusal, CandidateArchiveRefusal,
    read_binding, retain_binding,
};
use macroonz_harness::descriptor::{
    Binding, DerivedRevision, DoorRef, ExecutableAttachment, GeneratedSupportSchema, Origin,
    ProducerFacts, ProducerName, ProjectionRef, Provenance, RevisionBinding, RevisionPosture, Row,
    SynthesisFacts,
};

const LIMITS: BindingArchiveLimits = BindingArchiveLimits::declared(8192, 256, 8);

static ATTACHMENT_CALLS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

const COUNT_ATTACHMENT: fn(&()) = |_input| {
    ATTACHMENT_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
};

fn produced() -> Vec<u8> {
    let mut bytes = vec![1];
    name((b"producer", b"compiler"), &mut bytes);
    frame(&[7; 32], &mut bytes);
    bytes
}

#[test]
fn all_postures_and_lawful_origin_provenance_pairs_remain_historical() -> Result<(), ()> {
    let mut generated = vec![2];
    name((b"origin", b"door"), &mut generated);
    name((b"origin", b"projection"), &mut generated);
    let mut discharge = vec![5];
    frame(&[8; 32], &mut discharge);
    name((b"destination", b"other-suite"), &mut discharge);
    let mut origins = vec![vec![1], generated, vec![3, 2], discharge];
    let mut survivor = vec![3, 1];
    name((b"mutation", b"point"), &mut survivor);
    origins.push(survivor);
    for ground in [1u8, 2] {
        let mut replay = vec![4];
        frame(&[8; 32], &mut replay);
        replay.push(ground);
        name((b"destination", b"other-suite"), &mut replay);
        frame(&[9; 32], &mut replay);
        origins.push(replay);
    }
    for origin in origins {
        for provenance in [vec![0], produced()] {
            observe_postures(&origin, &provenance)?;
        }
    }
    Ok(())
}

fn observe_postures(origin: &[u8], provenance: &[u8]) -> Result<(), ()> {
    let postures = [
        (0u8, RevisionPosture::Derived),
        (1, RevisionPosture::Declared),
        (2, RevisionPosture::Untracked),
    ];
    for subject in postures {
        for check in postures {
            observe_binding(origin, provenance, subject, check)?;
        }
    }
    Ok(())
}

fn observe_binding(
    origin: &[u8],
    provenance: &[u8],
    subject: (u8, RevisionPosture),
    check: (u8, RevisionPosture),
) -> Result<(), ()> {
    let mut vector = BindingVector::declared();
    vector.row.origin = origin.to_vec();
    vector.provenance = provenance.to_vec();
    vector.subject_revision = revision(&[2; 32], subject.0);
    vector.check_revision = revision(&[3; 32], check.0);
    let record = read_binding(&vector.encoded(), LIMITS);
    if origin.first() == Some(&2) && provenance == [0] {
        assert_eq!(
            record,
            Err(BindingArchiveRefusal::GeneratedWithoutSchemaPin)
        );
        return Ok(());
    }
    let record = record.map_err(|_| ())?;
    assert_eq!(record.encoded(), vector.encoded());
    assert_eq!(record.subject(), record.row().subject());
    assert_eq!(record.check(), record.row().check());
    assert_eq!(record.subject_revision().revision(), &[2; 32]);
    assert_eq!(record.check_revision().revision(), &[3; 32]);
    assert_eq!(record.subject_revision().posture(), subject.1);
    assert_eq!(record.check_revision().posture(), check.1);
    match record.provenance() {
        ArchivedProvenance::Unproduced => assert_eq!(provenance, [0]),
        ArchivedProvenance::Produced { producer, schema } => {
            assert_eq!(
                (producer.namespace(), producer.stem()),
                ("producer", "compiler")
            );
            assert_eq!(schema, &[7; 32]);
        }
    }
    Ok(())
}
#[test]
fn binding_joins_refuse_in_live_semantic_order() {
    let mut vector = BindingVector::declared();
    vector.subject = (b"other", b"subject");
    vector.check = (b"other", b"check");
    assert_eq!(
        read_binding(&vector.encoded(), LIMITS),
        Err(BindingArchiveRefusal::SubjectMismatch)
    );
    vector.subject = (b"subject-owner", b"route");
    assert_eq!(
        read_binding(&vector.encoded(), LIMITS),
        Err(BindingArchiveRefusal::CheckMismatch)
    );
}

#[test]
fn binding_fields_require_complete_bounded_canonical_grammar() -> Result<(), ()> {
    let vector = BindingVector::declared();
    let bytes = vector.encoded();
    for length in 0..bytes.len() {
        assert!(read_binding(bytes.get(..length).ok_or(())?, LIMITS).is_err());
    }
    let exact = BindingArchiveLimits::declared(bytes.len(), 256, 2);
    assert!(read_binding(&bytes, exact).is_ok());
    assert_eq!(
        read_binding(
            &bytes,
            BindingArchiveLimits::declared(bytes.len().saturating_sub(1), 256, 2)
        ),
        Err(BindingArchiveRefusal::Canonical(
            CandidateArchiveRefusal::BytesTooLarge
        ))
    );
    assert!(read_binding(&bytes, BindingArchiveLimits::declared(8192, 31, 2)).is_err());
    assert!(read_binding(&bytes, BindingArchiveLimits::declared(8192, 256, 1)).is_err());
    for subject in [true, false] {
        for width in [0usize, 31, 33] {
            let mut malformed = BindingVector::declared();
            let field = if subject {
                &mut malformed.subject_revision
            } else {
                &mut malformed.check_revision
            };
            *field = revision(&vec![2; width], 0);
            assert_eq!(
                read_binding(&malformed.encoded(), LIMITS),
                Err(BindingArchiveRefusal::InvalidAddressWidth)
            );
        }
        let mut malformed = BindingVector::declared();
        let field = if subject {
            &mut malformed.subject_revision
        } else {
            &mut malformed.check_revision
        };
        *field = revision(&[2; 32], 3);
        assert_eq!(
            read_binding(&malformed.encoded(), LIMITS),
            Err(BindingArchiveRefusal::InvalidSlot)
        );
    }
    for invalid in [b"".as_slice(), &[0xff]] {
        let mut malformed = BindingVector::declared();
        malformed.subject = (invalid, b"route");
        assert_eq!(
            read_binding(&malformed.encoded(), LIMITS),
            Err(BindingArchiveRefusal::Canonical(
                CandidateArchiveRefusal::InvalidName
            ))
        );
    }
    let mut malformed = BindingVector::declared();
    malformed.version = 2;
    assert_eq!(
        read_binding(&malformed.encoded(), LIMITS),
        Err(BindingArchiveRefusal::Canonical(
            CandidateArchiveRefusal::UnsupportedFormat { found: 2 }
        ))
    );
    malformed.version = 1;
    malformed.provenance = vec![9];
    assert_eq!(
        read_binding(&malformed.encoded(), LIMITS),
        Err(BindingArchiveRefusal::InvalidSlot)
    );
    let mut trailing = bytes;
    trailing.push(0);
    assert_eq!(
        read_binding(&trailing, LIMITS),
        Err(BindingArchiveRefusal::Canonical(
            CandidateArchiveRefusal::TrailingBytes
        ))
    );
    Ok(())
}

#[test]
fn live_binding_retention_keeps_both_revisions_without_calling_the_attachment() -> Result<(), ()> {
    ATTACHMENT_CALLS.store(0, std::sync::atomic::Ordering::SeqCst);
    let row = super::super::row()?;
    let subject = RevisionBinding::derived(DerivedRevision::from_material(b"subject"));
    let check = RevisionBinding::untracked(
        RevisionBinding::derived(DerivedRevision::from_material(b"check")).revision(),
    );
    let attachment = ExecutableAttachment::attached(
        row.subject(),
        row.check(),
        subject,
        check,
        COUNT_ATTACHMENT,
    );
    let binding = Binding::bound(row, attachment, Provenance::Unproduced).map_err(|_| ())?;
    let record = retain_binding(&binding, LIMITS).map_err(|_| ())?;
    assert_eq!(
        record.subject_revision().revision(),
        subject.revision().as_bytes()
    );
    assert_eq!(
        record.check_revision().revision(),
        check.revision().as_bytes()
    );
    assert_eq!(
        record.check_revision().posture(),
        RevisionPosture::Untracked
    );
    assert_eq!(
        retain_binding(
            &binding,
            BindingArchiveLimits::declared(record.encoded().len(), 256, 8)
        )
        .map_err(|_| ())?,
        record
    );
    assert_eq!(
        ATTACHMENT_CALLS.load(std::sync::atomic::Ordering::SeqCst),
        0
    );
    Ok(())
}

#[test]
fn live_producer_standing_is_retained_on_every_publicly_constructible_origin() -> Result<(), ()> {
    let template = super::super::row()?;
    let facts = ProducerFacts::emitted(
        DoorRef::named("producer", "door").map_err(|_| ())?,
        ProjectionRef::named("producer", "projection").map_err(|_| ())?,
    );
    let producer = ProducerName::named("producer", "compiler").map_err(|_| ())?;
    let schema = GeneratedSupportSchema::published()
        .map_err(|_| ())?
        .identity()
        .map_err(|_| ())?;
    let revision = RevisionBinding::derived(DerivedRevision::from_material(b"revision"));
    for origin in [
        Origin::HandWritten,
        Origin::Generated(facts),
        Origin::Candidate(SynthesisFacts::ProofGap),
    ] {
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
        let attachment = ExecutableAttachment::attached(
            row.subject(),
            row.check(),
            revision,
            revision,
            |_input: &()| {},
        );
        let binding = Binding::bound(row, attachment, Provenance::Produced { producer, schema })
            .map_err(|_| ())?;
        let record = retain_binding(&binding, LIMITS).map_err(|_| ())?;
        let ArchivedProvenance::Produced {
            producer: retained,
            schema: retained_schema,
        } = record.provenance()
        else {
            return Err(());
        };
        assert_eq!(
            (retained.namespace(), retained.stem()),
            ("producer", "compiler")
        );
        assert_eq!(retained_schema, schema.address().as_bytes());
        assert_eq!(
            record.row().canonical_bytes(),
            binding.row().canonical_bytes().as_bytes()
        );
        let exact = BindingArchiveLimits::declared(record.encoded().len(), 256, 8);
        assert_eq!(retain_binding(&binding, exact).map_err(|_| ())?, record);
    }
    Ok(())
}

#[test]
fn producer_names_schema_width_and_revision_trailing_bytes_refuse() -> Result<(), ()> {
    for component in [0u8, 1] {
        for invalid in [b"".as_slice(), &[0xff]] {
            let mut vector = BindingVector::declared();
            vector.provenance = vec![1];
            let fields = if component == 0 {
                (invalid, b"compiler".as_slice())
            } else {
                (b"producer".as_slice(), invalid)
            };
            name(fields, &mut vector.provenance);
            frame(&[7; 32], &mut vector.provenance);
            assert_eq!(
                read_binding(&vector.encoded(), LIMITS),
                Err(BindingArchiveRefusal::Canonical(
                    CandidateArchiveRefusal::InvalidName
                ))
            );
        }
    }
    for width in [0usize, 31, 33] {
        let mut vector = BindingVector::declared();
        vector.provenance = vec![1];
        name((b"producer", b"compiler"), &mut vector.provenance);
        frame(&vec![7; width], &mut vector.provenance);
        assert_eq!(
            read_binding(&vector.encoded(), LIMITS),
            Err(BindingArchiveRefusal::InvalidAddressWidth)
        );
    }
    let mut vector = BindingVector::declared();
    vector.subject_revision.push(0);
    assert_eq!(
        read_binding(&vector.encoded(), LIMITS),
        Err(BindingArchiveRefusal::Canonical(
            CandidateArchiveRefusal::TrailingBytes
        ))
    );
    vector = BindingVector::declared();
    vector.provenance = produced();
    let bytes = vector.encoded();
    for length in 0..bytes.len() {
        assert!(read_binding(bytes.get(..length).ok_or(())?, LIMITS).is_err());
    }
    Ok(())
}
