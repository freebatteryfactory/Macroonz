//! Hostile backend envelopes with recomputed integrity and independent resource ceilings.

use super::backend_vector::{self as vector, BackendVector, CONSOLE, FILE, Material, SOURCE};
use super::backends::{LIMITS, fixture};
use super::trial_vector::foreign;
use super::vector::{frame, hash};
use macroonz_harness::muterprater::backend_archive::{
    BackendArchiveLimits, BackendArchiveRefusal, read_backend, retain_backend_with_material,
};
use macroonz_harness::muterprater::verdict_archive::{
    MutationArchiveRefusal, MutationRunArchiveLimits,
};
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal};

fn record_error(error: ArchiveRefusal) -> BackendArchiveRefusal {
    BackendArchiveRefusal::Record(error)
}

#[test]
fn backend_bytes_and_each_population_admit_the_exact_bound_then_refuse_excess() -> Result<(), ()> {
    let vector = BackendVector::declared(Material::Complete);
    let bytes = vector.encoded();
    let manifest = fixture(CONSOLE, &[(FILE, SOURCE)])?;
    let exact = BackendArchiveLimits::declared(
        MutationRunArchiveLimits::declared(
            ArchiveLimits::declared(bytes.len(), vector.run.len()),
            1,
        ),
        3,
        1,
        1,
    );
    let record = read_backend(&bytes, exact).map_err(|_| ())?;
    assert_eq!(
        retain_backend_with_material(&manifest, CONSOLE, &[(FILE, SOURCE)], exact)
            .map_err(|_| ())?,
        record
    );
    for (limits, expected) in [
        (
            BackendArchiveLimits::declared(
                MutationRunArchiveLimits::declared(
                    ArchiveLimits::declared(bytes.len().saturating_sub(1), vector.run.len()),
                    1,
                ),
                3,
                1,
                1,
            ),
            record_error(ArchiveRefusal::EnvelopeTooLarge),
        ),
        (
            BackendArchiveLimits::declared(
                MutationRunArchiveLimits::declared(
                    ArchiveLimits::declared(bytes.len(), vector.run.len().saturating_sub(1)),
                    1,
                ),
                3,
                1,
                1,
            ),
            record_error(ArchiveRefusal::FieldTooLarge),
        ),
        (
            BackendArchiveLimits::declared(exact.run(), 2, 1, 1),
            BackendArchiveRefusal::TooManyArguments,
        ),
        (
            BackendArchiveLimits::declared(exact.run(), 3, 0, 1),
            BackendArchiveRefusal::TooManySources,
        ),
        (
            BackendArchiveLimits::declared(exact.run(), 3, 1, 0),
            BackendArchiveRefusal::TooManyUnparsed,
        ),
        (
            BackendArchiveLimits::declared(
                MutationRunArchiveLimits::declared(exact.bytes(), 0),
                3,
                1,
                1,
            ),
            BackendArchiveRefusal::Mutation(MutationArchiveRefusal::TooManyReports),
        ),
    ] {
        assert_eq!(read_backend(&bytes, limits), Err(expected.clone()));
        assert_eq!(
            retain_backend_with_material(&manifest, CONSOLE, &[(FILE, SOURCE)], limits),
            Err(expected)
        );
    }
    Ok(())
}

#[test]
fn original_console_and_source_frames_have_their_own_byte_admission() -> Result<(), ()> {
    for (console, source) in [
        (
            format!(
                "ok Unmutated baseline {}\ncaught {FILE}:43:7: damage\n",
                " ".repeat(4096)
            ),
            SOURCE.to_vec(),
        ),
        (CONSOLE.to_owned(), vec![255; 4096]),
    ] {
        let manifest = fixture(&console, &[(FILE, &source)])?;
        let record = retain_backend_with_material(&manifest, &console, &[(FILE, &source)], LIMITS)
            .map_err(|_| ())?;
        let largest = console
            .len()
            .max(source.len())
            .max(record.run().encoded().len());
        for field in [largest, largest.saturating_sub(1)] {
            let limits = BackendArchiveLimits::declared(
                MutationRunArchiveLimits::declared(
                    ArchiveLimits::declared(record.encoded().len(), field),
                    1,
                ),
                3,
                1,
                1,
            );
            let read = read_backend(record.encoded(), limits);
            let write =
                retain_backend_with_material(&manifest, &console, &[(FILE, &source)], limits);
            if field == largest {
                assert_eq!(read.map_err(|_| ())?, record);
                assert_eq!(write.map_err(|_| ())?, record);
            } else {
                assert_eq!(read, Err(record_error(ArchiveRefusal::FieldTooLarge)));
                assert_eq!(write, Err(record_error(ArchiveRefusal::FieldTooLarge)));
            }
        }
    }
    Ok(())
}

#[test]
fn hostile_populations_refuse_before_any_declared_member_is_walked() {
    let mut vector = BackendVector::declared(Material::Absent);
    vector.invocation = vector::invocation(b"27-test", b"cargo", 9, &[]);
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::TooManyArguments)
    );
    vector = BackendVector::declared(Material::Absent);
    vector.sources = vector::sources(9, &[]);
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::TooManySources)
    );
    vector = BackendVector::declared(Material::Absent);
    vector.unparsed = vector::unread(9, &[]);
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::TooManyUnparsed)
    );
}

#[test]
fn source_files_and_unread_ordinals_refuse_duplicates_and_reversed_order() {
    let revision = [3u8; 32];
    for entries in [
        vec![
            (FILE.as_bytes(), revision.as_slice()),
            (FILE.as_bytes(), revision.as_slice()),
        ],
        vec![
            (b"z.rs".as_slice(), revision.as_slice()),
            (b"a.rs".as_slice(), revision.as_slice()),
        ],
    ] {
        let mut vector = BackendVector::declared(Material::Absent);
        vector.sources = vector::sources(2, &entries);
        assert_eq!(
            read_backend(&vector.encoded(), LIMITS),
            Err(BackendArchiveRefusal::InvalidSourceOrder)
        );
    }
    for ordinals in [[3, 3], [4, 3]] {
        let mut vector = BackendVector::declared(Material::Absent);
        let [first, second] = ordinals;
        vector.unparsed = vector::unread(
            2,
            &[
                (first, foreign(b"first", &[0, 0])),
                (second, foreign(b"second", &[0, 0])),
            ],
        );
        assert_eq!(
            read_backend(&vector.encoded(), LIMITS),
            Err(BackendArchiveRefusal::InvalidUnparsedOrder)
        );
    }
    let mut vector = BackendVector::declared(Material::Absent);
    vector.sources = vector::sources(0, &[]);
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::SourceMissing(FILE.to_owned()))
    );
    vector.sources = vector::sources(2, &[(b"a.rs", &revision), (FILE.as_bytes(), &revision)]);
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::SourceUnexpected("a.rs".to_owned()))
    );
}

#[test]
fn profile_join_and_required_utf8_fields_refuse_without_current_bindings() -> Result<(), ()> {
    let mut vector = BackendVector::declared(Material::Absent);
    vector.profile = vector::profile(b"other-version");
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::ProfileMismatch)
    );
    for byte in [0, 9] {
        vector = BackendVector::declared(Material::Absent);
        *vector.profile.get_mut(1).ok_or(())? = byte;
        let expected = if byte == 0 {
            BackendArchiveRefusal::ProfileMismatch
        } else {
            record_error(ArchiveRefusal::InvalidSlot)
        };
        assert_eq!(read_backend(&vector.encoded(), LIMITS), Err(expected));
    }
    vector = BackendVector::declared(Material::Absent);
    *vector.profile.last_mut().ok_or(())? = 2;
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::ProfileMismatch)
    );
    for (version, executable) in [
        (b"".as_slice(), b"cargo".as_slice()),
        (b"27-test", b""),
        (&[255], b"cargo"),
        (b"27-test", &[255]),
    ] {
        vector = BackendVector::declared(Material::Absent);
        vector.invocation = vector::invocation(version, executable, 0, &[]);
        assert_eq!(
            read_backend(&vector.encoded(), LIMITS),
            Err(record_error(ArchiveRefusal::InvalidText))
        );
    }
    for file in [b"".as_slice(), &[255]] {
        vector = BackendVector::declared(Material::Absent);
        vector.sources = vector::sources(1, &[(file, &[1; 32])]);
        assert_eq!(
            read_backend(&vector.encoded(), LIMITS),
            Err(record_error(ArchiveRefusal::InvalidText))
        );
    }
    Ok(())
}

#[test]
fn archival_integrity_does_not_hide_console_source_or_parsed_fact_disagreement() {
    let mut vector = BackendVector::declared(Material::Complete);
    vector.material = vector::material(b"changed", &[SOURCE]);
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::OutputMaterialMismatch)
    );
    vector.material = vector::material(CONSOLE.as_bytes(), &[b"changed"]);
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::SourceMaterialMismatch(
            FILE.to_owned()
        ))
    );
    for console in [
        CONSOLE.replace("43:7", "44:7"),
        CONSOLE.replace("with false", "with zero"),
        CONSOLE.replace("caught", "missed"),
        CONSOLE.replace("Found 19", "Found 20"),
        CONSOLE.replace("ok Unmutated", "failed Unmutated"),
        CONSOLE.replace("summary", "changed"),
        CONSOLE.replace("summary\n", ""),
        format!("extra\n{CONSOLE}"),
        format!("{CONSOLE}summary\n"),
    ] {
        vector = BackendVector::declared(Material::Complete);
        vector.output.clear();
        frame(
            &hash("mutation-backend-output/v1", console.as_bytes()),
            &mut vector.output,
        );
        vector.material = vector::material(console.as_bytes(), &[SOURCE]);
        assert_eq!(
            read_backend(&vector.encoded(), LIMITS),
            Err(BackendArchiveRefusal::ConsoleReadingMismatch)
        );
    }
    let mut mutation = vector::record();
    mutation.outcome = vec![0, 1];
    mutation
        .outcome
        .extend(foreign(b"caught another line", &[0, 0]));
    vector = BackendVector::declared(Material::Complete);
    vector.run = vector::run(&[mutation.encoded()]);
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::ConsoleReadingMismatch)
    );
}

#[test]
fn an_identity_only_backend_record_still_cannot_claim_another_mutation_road() {
    let mut equivalent = vector::record();
    equivalent.equivalence = 2;
    let mut observed = vector::record();
    observed.activation = super::proposal_vector::activation(1);
    let mut survivor = vector::record();
    survivor.activation = super::proposal_vector::activation(1);
    survivor.outcome = vec![1];
    let mut demonstrated = super::mutation_vector::MutationVector::demonstrated();
    demonstrated.target = vector::record().target;
    let mut nonqualified = vector::record();
    nonqualified.outcome = vec![2, 3];
    nonqualified.baseline = 2;
    let cases = [equivalent, observed, survivor, demonstrated, nonqualified];
    for mutation in cases {
        let mut vector = BackendVector::declared(Material::Absent);
        vector.run = vector::run(&[mutation.encoded()]);
        assert_eq!(
            read_backend(&vector.encoded(), LIMITS),
            Err(BackendArchiveRefusal::BackendRecordMismatch)
        );
    }
}

#[test]
fn headers_slots_nested_integrity_prefixes_and_trailing_material_refuse() -> Result<(), ()> {
    for (header, error) in [
        ([2, 1, 0], ArchiveRefusal::UnsupportedFormat { found: 2 }),
        ([1, 2, 0], ArchiveRefusal::WrongKind { found: 2 }),
        ([1, 1, 2], ArchiveRefusal::UnsupportedCustody { found: 2 }),
    ] {
        let mut vector = BackendVector::declared(Material::Complete);
        vector.header = header;
        assert_eq!(
            read_backend(&vector.encoded(), LIMITS),
            Err(record_error(error))
        );
    }
    let mut vector = BackendVector::declared(Material::Complete);
    for length in 0..vector.body().len() {
        let body = vector.body();
        assert!(read_backend(&vector::envelope(body.get(..length).ok_or(())?), LIMITS).is_err());
    }
    let mut body = vector.body();
    body.push(0);
    assert_eq!(
        read_backend(&vector::envelope(&body), LIMITS),
        Err(record_error(ArchiveRefusal::TrailingBytes))
    );
    let mut bytes = vector.encoded();
    *bytes.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_backend(&bytes, LIMITS),
        Err(record_error(ArchiveRefusal::AddressMismatch))
    );
    vector = BackendVector::declared(Material::Complete);
    *vector.run.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_backend(&vector.encoded(), LIMITS),
        Err(BackendArchiveRefusal::Mutation(
            MutationArchiveRefusal::Record(ArchiveRefusal::AddressMismatch)
        ))
    );
    for seat in [0u8, 1, 2, 3, 4] {
        vector = BackendVector::declared(Material::Absent);
        let member = match seat {
            0 => &mut vector.invocation,
            1 => &mut vector.profile,
            2 => &mut vector.announced,
            3 => &mut vector.material,
            _ => &mut vector.output,
        };
        *member.first_mut().ok_or(())? = 9;
        assert!(read_backend(&vector.encoded(), LIMITS).is_err());
    }
    Ok(())
}
