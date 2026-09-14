//! Independent suite joins and outer-versus-nested resource refusals.

use super::backend_vector::{self as backend, BackendVector, CONSOLE, FILE, Material, SOURCE};
use super::suite_vector::{self as vector, SuiteVector};
use super::suites::{LIMITS, fixture};
use macroonz_harness::muterprater::backend_archive::{
    BackendArchiveLimits, BackendArchiveRefusal, SuitePressureArchiveLimits,
    SuitePressureArchiveRefusal, read_suite_pressure, retain_suite_pressure,
    retain_suite_pressure_with_material,
};
use macroonz_harness::muterprater::verdict_archive::{
    MutationArchiveRefusal, MutationRunArchiveLimits,
};
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal};

#[test]
fn first_kill_ordinal_cannot_choose_a_nonkill_later_kill_or_absent_position() {
    for ordinal in [0u64, 2, 3, u64::MAX] {
        let mut vector = SuiteVector::mixed();
        vector.ordinal = ordinal;
        assert_eq!(
            read_suite_pressure(&vector.encoded(), LIMITS),
            Err(SuitePressureArchiveRefusal::FirstKillMismatch)
        );
    }
    for records in [vec![vector::missed()], vec![]] {
        let mut nested = BackendVector::declared(Material::Absent);
        if records.is_empty() {
            nested.sources = backend::sources(0, &[]);
        }
        nested.run = backend::run(&records);
        let mut vector = SuiteVector::declared(Material::Absent);
        vector.backend = nested.encoded();
        assert_eq!(
            read_suite_pressure(&vector.encoded(), LIMITS),
            Err(SuitePressureArchiveRefusal::FirstKillMismatch)
        );
    }
}

#[test]
fn checked_grammar_version_is_required_and_joins_the_single_manifest_profile() {
    for (checked, expected) in [
        (0u8, SuitePressureArchiveRefusal::QualificationMismatch),
        (
            2,
            SuitePressureArchiveRefusal::Record(ArchiveRefusal::InvalidSlot),
        ),
    ] {
        let mut vector = SuiteVector::declared(Material::Absent);
        vector.checked = checked;
        assert_eq!(
            read_suite_pressure(&vector.encoded(), LIMITS),
            Err(expected)
        );
    }
    for version in [b"".as_slice(), b"other-version"] {
        let mut vector = SuiteVector::declared(Material::Absent);
        vector.version = version.to_vec();
        assert_eq!(
            read_suite_pressure(&vector.encoded(), LIMITS),
            Err(SuitePressureArchiveRefusal::QualificationMismatch)
        );
    }
    let mut vector = SuiteVector::declared(Material::Absent);
    vector.version = vec![255];
    assert_eq!(
        read_suite_pressure(&vector.encoded(), LIMITS),
        Err(SuitePressureArchiveRefusal::Record(
            ArchiveRefusal::InvalidText
        ))
    );
    let mut nested = BackendVector::declared(Material::Absent);
    nested.profile = backend::profile(b"moved");
    vector.version = b"moved".to_vec();
    vector.backend = nested.encoded();
    assert_eq!(
        read_suite_pressure(&vector.encoded(), LIMITS),
        Err(SuitePressureArchiveRefusal::Backend(
            BackendArchiveRefusal::ProfileMismatch
        ))
    );
}

fn refused_limits(
    outer: ArchiveLimits,
    backend: BackendArchiveLimits,
) -> [(SuitePressureArchiveLimits, SuitePressureArchiveRefusal); 8] {
    let bytes = backend.bytes();
    [
        (
            SuitePressureArchiveLimits::declared(
                ArchiveLimits::declared(outer.envelope().saturating_sub(1), outer.field()),
                backend,
            ),
            SuitePressureArchiveRefusal::Record(ArchiveRefusal::EnvelopeTooLarge),
        ),
        (
            SuitePressureArchiveLimits::declared(
                ArchiveLimits::declared(outer.envelope(), outer.field().saturating_sub(1)),
                backend,
            ),
            SuitePressureArchiveRefusal::Record(ArchiveRefusal::FieldTooLarge),
        ),
        (
            SuitePressureArchiveLimits::declared(
                outer,
                BackendArchiveLimits::declared(
                    MutationRunArchiveLimits::declared(
                        ArchiveLimits::declared(bytes.envelope().saturating_sub(1), bytes.field()),
                        1,
                    ),
                    3,
                    1,
                    1,
                ),
            ),
            SuitePressureArchiveRefusal::Backend(BackendArchiveRefusal::Record(
                ArchiveRefusal::EnvelopeTooLarge,
            )),
        ),
        (
            SuitePressureArchiveLimits::declared(
                outer,
                BackendArchiveLimits::declared(
                    MutationRunArchiveLimits::declared(
                        ArchiveLimits::declared(bytes.envelope(), bytes.field().saturating_sub(1)),
                        1,
                    ),
                    3,
                    1,
                    1,
                ),
            ),
            SuitePressureArchiveRefusal::Backend(BackendArchiveRefusal::Record(
                ArchiveRefusal::FieldTooLarge,
            )),
        ),
        (
            SuitePressureArchiveLimits::declared(
                outer,
                BackendArchiveLimits::declared(backend.run(), 2, 1, 1),
            ),
            SuitePressureArchiveRefusal::Backend(BackendArchiveRefusal::TooManyArguments),
        ),
        (
            SuitePressureArchiveLimits::declared(
                outer,
                BackendArchiveLimits::declared(backend.run(), 3, 0, 1),
            ),
            SuitePressureArchiveRefusal::Backend(BackendArchiveRefusal::TooManySources),
        ),
        (
            SuitePressureArchiveLimits::declared(
                outer,
                BackendArchiveLimits::declared(backend.run(), 3, 1, 0),
            ),
            SuitePressureArchiveRefusal::Backend(BackendArchiveRefusal::TooManyUnparsed),
        ),
        (
            SuitePressureArchiveLimits::declared(
                outer,
                BackendArchiveLimits::declared(
                    MutationRunArchiveLimits::declared(bytes, 0),
                    3,
                    1,
                    1,
                ),
            ),
            SuitePressureArchiveRefusal::Backend(BackendArchiveRefusal::Mutation(
                MutationArchiveRefusal::TooManyReports,
            )),
        ),
    ]
}

#[test]
fn outer_and_nested_envelopes_fields_and_populations_have_independent_bounds() -> Result<(), ()> {
    let pressure = fixture(CONSOLE, &[(FILE, SOURCE)])?;
    for material in [Material::Absent, Material::Complete] {
        let originals = matches!(material, Material::Complete);
        let vector = SuiteVector::declared(material);
        let nested = BackendVector::declared(material);
        let encoded = vector.encoded();
        let bytes = ArchiveLimits::declared(vector.backend.len(), nested.run.len());
        let backend =
            BackendArchiveLimits::declared(MutationRunArchiveLimits::declared(bytes, 1), 3, 1, 1);
        let outer = ArchiveLimits::declared(encoded.len(), vector.backend.len());
        let exact = SuitePressureArchiveLimits::declared(outer, backend);
        let record = read_suite_pressure(&encoded, exact).map_err(|_| ())?;
        let written = if originals {
            retain_suite_pressure_with_material(&pressure, CONSOLE, &[(FILE, SOURCE)], exact)
        } else {
            retain_suite_pressure(&pressure, exact)
        }
        .map_err(|_| ())?;
        assert_eq!(record, written);
        let cases = refused_limits(outer, backend);
        for (limits, expected) in cases {
            assert_eq!(read_suite_pressure(&encoded, limits), Err(expected.clone()));
            let refused_write = if originals {
                retain_suite_pressure_with_material(&pressure, CONSOLE, &[(FILE, SOURCE)], limits)
            } else {
                retain_suite_pressure(&pressure, limits)
            };
            assert_eq!(refused_write, Err(expected));
        }
    }
    Ok(())
}

#[test]
fn suite_originals_preserve_backend_roster_and_content_refusals() -> Result<(), ()> {
    let pressure = fixture(CONSOLE, &[(FILE, SOURCE)])?;
    for (sources, expected) in [
        (
            vec![],
            BackendArchiveRefusal::SourceMissing(FILE.to_owned()),
        ),
        (
            vec![(FILE, SOURCE), (FILE, SOURCE)],
            BackendArchiveRefusal::DuplicateMaterialSource(FILE.to_owned()),
        ),
        (
            vec![(FILE, SOURCE), ("extra", SOURCE)],
            BackendArchiveRefusal::SourceUnexpected("extra".to_owned()),
        ),
        (
            vec![(FILE, b"moved".as_slice())],
            BackendArchiveRefusal::SourceMaterialMismatch(FILE.to_owned()),
        ),
    ] {
        assert_eq!(
            retain_suite_pressure_with_material(&pressure, CONSOLE, &sources, LIMITS),
            Err(SuitePressureArchiveRefusal::Backend(expected))
        );
    }
    assert_eq!(
        retain_suite_pressure_with_material(&pressure, "moved", &[(FILE, SOURCE)], LIMITS),
        Err(SuitePressureArchiveRefusal::Backend(
            BackendArchiveRefusal::OutputMaterialMismatch
        ))
    );
    let mut nested = BackendVector::declared(Material::Complete);
    nested.material = backend::material(CONSOLE.as_bytes(), &[b"moved"]);
    let mut vector = SuiteVector::declared(Material::Complete);
    vector.backend = nested.encoded();
    assert_eq!(
        read_suite_pressure(&vector.encoded(), LIMITS),
        Err(SuitePressureArchiveRefusal::Backend(
            BackendArchiveRefusal::SourceMaterialMismatch(FILE.to_owned())
        ))
    );
    Ok(())
}

#[test]
fn suite_headers_integrity_nested_integrity_prefixes_and_trailing_bytes_refuse() -> Result<(), ()> {
    for (header, expected) in [
        ([2u32, 1, 0], ArchiveRefusal::UnsupportedFormat { found: 2 }),
        ([1, 2, 0], ArchiveRefusal::WrongKind { found: 2 }),
        ([1, 1, 1], ArchiveRefusal::UnsupportedCustody { found: 1 }),
    ] {
        let mut vector = SuiteVector::declared(Material::Complete);
        vector.header = header;
        assert_eq!(
            read_suite_pressure(&vector.encoded(), LIMITS),
            Err(SuitePressureArchiveRefusal::Record(expected))
        );
    }
    let mut vector = SuiteVector::declared(Material::Complete);
    let body = vector.body();
    for length in 0..body.len() {
        assert!(
            read_suite_pressure(&vector::envelope(body.get(..length).ok_or(())?), LIMITS).is_err()
        );
    }
    let mut trailing = body;
    trailing.push(0);
    assert_eq!(
        read_suite_pressure(&vector::envelope(&trailing), LIMITS),
        Err(SuitePressureArchiveRefusal::Record(
            ArchiveRefusal::TrailingBytes
        ))
    );
    let mut corrupt = vector.encoded();
    *corrupt.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_suite_pressure(&corrupt, LIMITS),
        Err(SuitePressureArchiveRefusal::Record(
            ArchiveRefusal::AddressMismatch
        ))
    );
    *vector.backend.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_suite_pressure(&vector.encoded(), LIMITS),
        Err(SuitePressureArchiveRefusal::Backend(
            BackendArchiveRefusal::Record(ArchiveRefusal::AddressMismatch)
        ))
    );
    for field in [13usize, 28] {
        let mut oversized = SuiteVector::declared(Material::Complete).body();
        oversized
            .get_mut(field..field.saturating_add(8))
            .ok_or(())?
            .copy_from_slice(&u64::MAX.to_be_bytes());
        assert_eq!(
            read_suite_pressure(&vector::envelope(&oversized), LIMITS),
            Err(SuitePressureArchiveRefusal::Record(
                ArchiveRefusal::Truncated
            ))
        );
    }
    Ok(())
}
