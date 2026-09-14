//! Outer and nested integrity, complete framing and independently declared limits.

use super::interpreted::LIMITS;
use super::interpreted_vector::{InterpretedVector, envelope};
use macroonz_harness::muterprater::backend_archive::SuitePressureArchiveLimits;
use macroonz_harness::muterprater::discovery_archive::SurfaceArchiveLimits;
use macroonz_harness::muterprater::interpretation_archive::{
    InterpretedArchiveLimits, InterpretedArchiveRefusal, read_interpreted,
};
use macroonz_harness::muterprater::specimen_archive::ProjectionArchiveLimits;
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal};

#[test]
fn interpreted_unknown_headers_nested_integrity_and_trailing_material_refuse() -> Result<(), ()> {
    for (slot, word, cause) in [
        (0usize, 2, ArchiveRefusal::UnsupportedFormat { found: 2 }),
        (1, 2, ArchiveRefusal::WrongKind { found: 2 }),
        (2, 1, ArchiveRefusal::UnsupportedCustody { found: 1 }),
    ] {
        let mut vector = InterpretedVector::declared()?;
        *vector.header.get_mut(slot).ok_or(())? = word;
        assert_eq!(
            read_interpreted(&vector.encoded(), &LIMITS),
            Err(InterpretedArchiveRefusal::Record(cause))
        );
    }
    let vector = InterpretedVector::declared()?;
    for member in [0usize, 1, 2, 4, 5] {
        let mut members = vector.members();
        *members.get_mut(member).ok_or(())?.first_mut().ok_or(())? ^= 1;
        assert!(
            read_interpreted(&envelope(&vector.body_with(&members)), &LIMITS).is_err(),
            "member={member}"
        );
    }
    let mut corrupt = vector.encoded();
    *corrupt.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_interpreted(&corrupt, &LIMITS),
        Err(InterpretedArchiveRefusal::Record(
            ArchiveRefusal::AddressMismatch
        ))
    );
    let mut trailing = vector.body();
    trailing.push(0);
    assert_eq!(
        read_interpreted(&envelope(&trailing), &LIMITS),
        Err(InterpretedArchiveRefusal::Record(
            ArchiveRefusal::TrailingBytes
        ))
    );
    Ok(())
}

#[test]
fn every_interpreted_prefix_and_absent_huge_member_refuses() -> Result<(), ()> {
    let vector = InterpretedVector::declared()?;
    let body = vector.body();
    for length in 0..body.len() {
        assert!(
            read_interpreted(&envelope(body.get(..length).ok_or(())?), &LIMITS).is_err(),
            "prefix={length}"
        );
    }
    let mut huge = body.get(..12).ok_or(())?.to_vec();
    huge.extend_from_slice(&u64::MAX.to_be_bytes());
    assert_eq!(
        read_interpreted(&envelope(&huge), &LIMITS),
        Err(InterpretedArchiveRefusal::Record(ArchiveRefusal::Truncated))
    );
    Ok(())
}

#[test]
fn outer_each_nested_record_rosters_sources_and_active_value_have_independent_ceilings()
-> Result<(), ()> {
    let vector = InterpretedVector::declared()?;
    let bytes = vector.encoded();
    let small = ArchiveLimits::declared(1, 131_072);
    let outer = LIMITS.bytes();
    let surface = LIMITS.surface();
    let suite = LIMITS.suite();
    let projection = LIMITS.projection();
    for limits in [
        InterpretedArchiveLimits::declared(small, surface, suite, projection, outer, outer, 256),
        InterpretedArchiveLimits::declared(
            ArchiveLimits::declared(262_144, 31),
            surface,
            suite,
            projection,
            outer,
            outer,
            256,
        ),
        InterpretedArchiveLimits::declared(
            outer,
            SurfaceArchiveLimits::declared(small, 8, 8),
            suite,
            projection,
            outer,
            outer,
            256,
        ),
        InterpretedArchiveLimits::declared(
            outer,
            SurfaceArchiveLimits::declared(outer, 0, 8),
            suite,
            projection,
            outer,
            outer,
            256,
        ),
        InterpretedArchiveLimits::declared(
            outer,
            SurfaceArchiveLimits::declared(outer, 8, 0),
            suite,
            projection,
            outer,
            outer,
            256,
        ),
        InterpretedArchiveLimits::declared(
            outer,
            surface,
            SuitePressureArchiveLimits::declared(small, suite.backend()),
            projection,
            outer,
            outer,
            256,
        ),
        InterpretedArchiveLimits::declared(
            outer,
            surface,
            suite,
            ProjectionArchiveLimits::declared(small, projection.parity(), outer, outer, 4096),
            outer,
            outer,
            256,
        ),
        InterpretedArchiveLimits::declared(
            outer,
            surface,
            suite,
            ProjectionArchiveLimits::declared(outer, projection.parity(), outer, outer, 2),
            outer,
            outer,
            256,
        ),
        InterpretedArchiveLimits::declared(outer, surface, suite, projection, small, outer, 256),
        InterpretedArchiveLimits::declared(outer, surface, suite, projection, outer, small, 256),
        InterpretedArchiveLimits::declared(outer, surface, suite, projection, outer, outer, 2),
    ] {
        assert!(
            read_interpreted(&bytes, &limits).is_err(),
            "limits={limits:?}"
        );
    }
    Ok(())
}

#[test]
fn exact_interpreted_envelope_and_active_bounds_admit_while_one_byte_short_refuses()
-> Result<(), ()> {
    let bytes = InterpretedVector::declared()?.encoded();
    let outer = LIMITS.bytes();
    let surface = LIMITS.surface();
    let suite = LIMITS.suite();
    let projection = LIMITS.projection();
    let exact = InterpretedArchiveLimits::declared(
        ArchiveLimits::declared(bytes.len(), outer.field()),
        surface,
        suite,
        projection,
        outer,
        outer,
        3,
    );
    assert!(read_interpreted(&bytes, &exact).is_ok());
    let short = InterpretedArchiveLimits::declared(
        ArchiveLimits::declared(bytes.len().saturating_sub(1), outer.field()),
        surface,
        suite,
        projection,
        outer,
        outer,
        3,
    );
    assert_eq!(
        read_interpreted(&bytes, &short),
        Err(InterpretedArchiveRefusal::Record(
            ArchiveRefusal::EnvelopeTooLarge
        ))
    );
    Ok(())
}
