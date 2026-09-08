//! Independent historical surface bytes, hostile joins and actual public lowering.

use super::binding_vector::name;
use super::process;
use super::surface_vector::{SurfaceVector, envelope};
use super::vector::{frame, hash};
use macroonz_harness::descriptor::archive::ArchivedName;
use macroonz_harness::descriptor::{ClaimRef, MutationPointRef};
use macroonz_harness::muterprater::discover::lower_discoveries;
use macroonz_harness::muterprater::discovery_archive::{
    ArchivedEvaluationSurface, SurfaceArchiveLimits, SurfaceArchiveRefusal, read_surface,
    retain_surface,
};
use macroonz_harness::muterprater::{
    ActivationSite, AlternativeDeclaration, DiscoveredMutationSite, EvaluationFamilyRef,
    EvaluationSurface, MutationPermission, MutationPolicy, OperatorFamilyRef, OwnerClaimMapping,
};
use macroonz_harness::report::archive::{ArchiveLimits, ArchiveRefusal};
use std::error::Error;
use std::io::Read as _;

const LIMITS: SurfaceArchiveLimits =
    SurfaceArchiveLimits::declared(ArchiveLimits::declared(16384, 8192), 4, 4);

fn spelling(name: &ArchivedName) -> (&str, &str) {
    (name.namespace(), name.stem())
}

fn observe(vector: &SurfaceVector, record: &ArchivedEvaluationSurface) -> Result<(), ()> {
    assert_eq!(record.encoded(), vector.encoded());
    assert_eq!(record.canonical_bytes(), vector.canonical());
    assert_eq!(
        record.address().as_bytes(),
        &hash("historical-evaluation-surface/v1", &vector.body())
    );
    assert_eq!(
        record.identity().as_bytes(),
        &hash("evaluation-surface/v1", &vector.canonical())
    );
    assert_eq!(record.policy().as_bytes().as_slice(), vector.policy);
    assert_eq!(spelling(record.family()), ("évaluation", "family"));
    assert_eq!(record.points().len(), vector.points.len());
    for (found, expected) in record.points().iter().zip(&vector.points) {
        assert_eq!(
            spelling(found.name()),
            (
                core::str::from_utf8(expected.name.0).map_err(|_| ())?,
                core::str::from_utf8(expected.name.1).map_err(|_| ())?
            )
        );
        assert_eq!(spelling(found.owner_claim()), ("owner", "law"));
        assert_eq!(found.original_operation(), expected.original);
        assert_eq!(spelling(found.activation_site()), ("site", "fires"));
        assert_eq!(found.alternatives().len(), expected.alternatives.len());
        for (alternative, expected_alternative) in
            found.alternatives().iter().zip(&expected.alternatives)
        {
            assert_eq!(
                alternative.identity().as_bytes(),
                &expected_alternative.address(expected.name)
            );
            assert_eq!(alternative.family().as_bytes(), expected_alternative.family);
            assert_eq!(alternative.operation(), expected_alternative.operation);
        }
    }
    Ok(())
}

#[test]
fn independent_surface_material_stays_owned_without_current_catalogue_lookup() -> Result<(), ()> {
    for historical_family in [
        b"boolean-operators".as_slice(),
        b"retired-independent-family",
    ] {
        let mut vector = SurfaceVector::declared();
        for point in &mut vector.points {
            for alternative in &mut point.alternatives {
                alternative.family = historical_family.to_vec();
            }
            point.ordered();
        }
        let source = vector.encoded();
        let record = read_surface(&source, LIMITS).map_err(|_| ())?;
        drop(source);
        observe(&vector, &record)?;
    }
    let mut empty = SurfaceVector::declared();
    empty.points.clear();
    observe(
        &empty,
        &read_surface(&empty.encoded(), LIMITS).map_err(|_| ())?,
    )
}

fn live_surface(vector: &mut SurfaceVector) -> Result<EvaluationSurface, ()> {
    let family = EvaluationFamilyRef::named("évaluation", "family").map_err(|_| ())?;
    let claim = ClaimRef::named("owner", "law").map_err(|_| ())?;
    let operator = OperatorFamilyRef::of_slug("boolean-operators").ok_or(())?;
    let permission = MutationPermission::declared(claim, vec![operator]).map_err(|_| ())?;
    let policy = MutationPolicy::declared(family, vec![permission]).map_err(|_| ())?;
    let mut policy_bytes = Vec::new();
    name(vector.family, &mut policy_bytes);
    policy_bytes.extend_from_slice(&1u64.to_be_bytes());
    name((b"owner", b"law"), &mut policy_bytes);
    policy_bytes.extend_from_slice(&1u64.to_be_bytes());
    frame(b"boolean-operators", &mut policy_bytes);
    vector.policy = hash("mutation-policy/v1", &policy_bytes).to_vec();
    assert_eq!(
        policy.identity().address().as_bytes().as_slice(),
        vector.policy
    );
    let mut sites = Vec::new();
    for point in vector.points.iter().rev() {
        let alternatives = point
            .alternatives
            .iter()
            .rev()
            .map(|alternative| {
                AlternativeDeclaration::stated(operator, alternative.operation.clone())
            })
            .collect();
        sites.push(
            DiscoveredMutationSite::discovered(
                MutationPointRef::named(
                    "point",
                    core::str::from_utf8(point.name.1).map_err(|_| ())?,
                )
                .map_err(|_| ())?,
                OwnerClaimMapping::Mapped(claim),
                point.original.clone(),
                alternatives,
                ActivationSite::named("site", "fires").map_err(|_| ())?,
            )
            .map_err(|_| ())?,
        );
    }
    Ok(lower_discoveries(&policy, sites)
        .map_err(|_| ())?
        .into_parts()
        .1)
}

#[test]
fn live_lowering_retains_the_independent_preimage_and_exact_bounds() -> Result<(), ()> {
    for populated in [false, true] {
        let mut vector = SurfaceVector::declared();
        if !populated {
            vector.points.clear();
        }
        let surface = live_surface(&mut vector)?;
        let record = retain_surface(&surface, LIMITS).map_err(|_| ())?;
        assert_eq!(record.identity(), surface.identity().address());
        observe(&vector, &record)?;
        let exact = SurfaceArchiveLimits::declared(
            ArchiveLimits::declared(record.encoded().len(), record.canonical_bytes().len()),
            vector.points.len(),
            2,
        );
        assert_eq!(retain_surface(&surface, exact).map_err(|_| ())?, record);
        assert_eq!(
            read_surface(record.encoded(), exact).map_err(|_| ())?,
            record
        );
        let short = SurfaceArchiveLimits::declared(
            ArchiveLimits::declared(record.encoded().len().saturating_sub(1), 8192),
            4,
            4,
        );
        for refusal in [
            retain_surface(&surface, short),
            read_surface(record.encoded(), short),
        ] {
            assert_eq!(
                refusal,
                Err(SurfaceArchiveRefusal::Record(
                    ArchiveRefusal::EnvelopeTooLarge
                ))
            );
        }
        drop(surface);
        observe(&vector, &record)?;
    }
    Ok(())
}

#[test]
fn populations_and_canonical_order_are_not_normalized_during_loading() -> Result<(), ()> {
    let mut reordered = SurfaceVector::declared();
    reordered.points.reverse();
    assert_eq!(
        read_surface(&reordered.encoded(), LIMITS),
        Err(SurfaceArchiveRefusal::NonCanonicalPoints)
    );
    reordered.points.reverse();
    reordered
        .points
        .push(reordered.points.first().ok_or(())?.clone());
    assert_eq!(
        read_surface(&reordered.encoded(), LIMITS),
        Err(SurfaceArchiveRefusal::NonCanonicalPoints)
    );
    for duplicate in [false, true] {
        let mut vector = SurfaceVector::declared();
        let point = vector.points.first_mut().ok_or(())?;
        if duplicate {
            point
                .alternatives
                .push(point.alternatives.first().ok_or(())?.clone());
        } else {
            point.alternatives.reverse();
        }
        assert_eq!(
            read_surface(&vector.encoded(), LIMITS),
            Err(SurfaceArchiveRefusal::NonCanonicalAlternatives)
        );
    }
    for (point_count, alternative_count, expected) in [
        (1, 4, SurfaceArchiveRefusal::TooManyPoints),
        (4, 1, SurfaceArchiveRefusal::TooManyAlternatives),
    ] {
        let mut vector = SurfaceVector::declared();
        let live = live_surface(&mut vector)?;
        let limits = SurfaceArchiveLimits::declared(LIMITS.bytes(), point_count, alternative_count);
        assert_eq!(
            read_surface(&vector.encoded(), limits),
            Err(expected.clone())
        );
        assert_eq!(retain_surface(&live, limits), Err(expected));
    }
    Ok(())
}

#[test]
fn operation_invariants_and_independent_identity_joins_refuse() -> Result<(), ()> {
    for case in 0u8..6 {
        let mut vector = SurfaceVector::declared();
        let point = vector.points.first_mut().ok_or(())?;
        let expected = match case {
            0 => {
                point.original.clear();
                SurfaceArchiveRefusal::EmptyOperation
            }
            1 => {
                point.alternatives.clear();
                SurfaceArchiveRefusal::NonCanonicalAlternatives
            }
            2 => {
                point.alternatives.first_mut().ok_or(())?.operation.clear();
                SurfaceArchiveRefusal::EmptyOperation
            }
            3 => {
                point
                    .alternatives
                    .first_mut()
                    .ok_or(())?
                    .operation
                    .clone_from(&point.original);
                SurfaceArchiveRefusal::AlternativeIsOriginal
            }
            4 => {
                point.alternatives.first_mut().ok_or(())?.identity = Some(vec![0; 32]);
                SurfaceArchiveRefusal::AlternativeIdentityMismatch
            }
            _ => {
                vector.identity = Some(vec![0; 32]);
                SurfaceArchiveRefusal::SurfaceIdentityMismatch
            }
        };
        assert_eq!(read_surface(&vector.encoded(), LIMITS), Err(expected));
    }
    Ok(())
}

#[test]
fn canonical_preimages_and_envelopes_require_all_bytes_and_supported_headers() -> Result<(), ()> {
    let mut vector = SurfaceVector::declared();
    let canonical = vector.canonical();
    for length in 0..canonical.len() {
        assert!(
            read_surface(
                &envelope(&vector.body_with(canonical.get(..length).ok_or(())?)),
                LIMITS
            )
            .is_err()
        );
    }
    let body = vector.body();
    for length in 0..body.len() {
        assert!(read_surface(&envelope(body.get(..length).ok_or(())?), LIMITS).is_err());
    }
    let mut trailing = canonical;
    trailing.push(0);
    assert_eq!(
        read_surface(&envelope(&vector.body_with(&trailing)), LIMITS),
        Err(SurfaceArchiveRefusal::Record(ArchiveRefusal::TrailingBytes))
    );
    let mut trailing_body = body;
    trailing_body.push(0);
    assert_eq!(
        read_surface(&envelope(&trailing_body), LIMITS),
        Err(SurfaceArchiveRefusal::Record(ArchiveRefusal::TrailingBytes))
    );
    for case in 0u8..3 {
        let expected = match case {
            0 => {
                vector.format = 2;
                ArchiveRefusal::UnsupportedFormat { found: 2 }
            }
            1 => {
                vector.format = 1;
                vector.kind = 2;
                ArchiveRefusal::WrongKind { found: 2 }
            }
            _ => {
                vector.kind = 1;
                vector.custody = 1;
                ArchiveRefusal::UnsupportedCustody { found: 1 }
            }
        };
        assert_eq!(
            read_surface(&vector.encoded(), LIMITS),
            Err(SurfaceArchiveRefusal::Record(expected))
        );
    }
    let mut corrupt = SurfaceVector::declared().encoded();
    *corrupt.first_mut().ok_or(())? ^= 1;
    assert_eq!(
        read_surface(&corrupt, LIMITS),
        Err(SurfaceArchiveRefusal::Record(
            ArchiveRefusal::AddressMismatch
        ))
    );
    Ok(())
}

#[test]
#[ignore = "driven by the surface process-boundary claim"]
fn child_loads_surface() -> Result<(), Box<dyn Error>> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take(16385)
        .read_to_end(&mut bytes)?;
    let record = read_surface(&bytes, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(bytes);
    process::publish(record.encoded())
}

#[test]
fn each_required_name_and_family_component_is_nonempty_utf8() -> Result<(), ()> {
    for invalid in [b"".as_slice(), &[255]] {
        for case in 0u8..9 {
            let mut vector = SurfaceVector::declared();
            let point = vector.points.first_mut().ok_or(())?;
            match case {
                0 => vector.family.0 = invalid,
                1 => vector.family.1 = invalid,
                2 => point.name.0 = invalid,
                3 => point.name.1 = invalid,
                4 => point.owner.0 = invalid,
                5 => point.owner.1 = invalid,
                6 => point.site.0 = invalid,
                7 => point.site.1 = invalid,
                _ => point.alternatives.first_mut().ok_or(())?.family = invalid.to_vec(),
            }
            assert_eq!(
                read_surface(&vector.encoded(), LIMITS),
                Err(SurfaceArchiveRefusal::Record(ArchiveRefusal::InvalidText))
            );
        }
    }
    Ok(())
}

#[test]
fn address_widths_and_untrusted_counts_cannot_borrow_integrity() -> Result<(), ()> {
    for width in [0usize, 31, 33] {
        for case in 0u8..3 {
            let mut vector = SurfaceVector::declared();
            match case {
                0 => vector.identity = Some(vec![7; width]),
                1 => vector.policy = vec![7; width],
                _ => {
                    vector
                        .points
                        .first_mut()
                        .ok_or(())?
                        .alternatives
                        .first_mut()
                        .ok_or(())?
                        .identity = Some(vec![7; width]);
                }
            }
            assert_eq!(
                read_surface(&vector.encoded(), LIMITS),
                Err(SurfaceArchiveRefusal::Record(
                    ArchiveRefusal::InvalidAddressWidth
                ))
            );
        }
    }
    let mut vector = SurfaceVector::declared();
    vector.count = Some(5);
    assert_eq!(
        read_surface(&vector.encoded(), LIMITS),
        Err(SurfaceArchiveRefusal::TooManyPoints)
    );
    vector.count = None;
    vector.points.first_mut().ok_or(())?.count = Some(5);
    assert_eq!(
        read_surface(&vector.encoded(), LIMITS),
        Err(SurfaceArchiveRefusal::TooManyAlternatives)
    );
    vector.points.first_mut().ok_or(())?.count = Some(u64::MAX);
    assert!(read_surface(&vector.encoded(), LIMITS).is_err());
    vector.count = Some(u64::MAX);
    assert!(read_surface(&vector.encoded(), LIMITS).is_err());
    Ok(())
}

#[test]
fn surface_and_alternative_identity_movement_keep_their_distinct_coordinates() -> Result<(), ()> {
    let baseline = SurfaceVector::declared();
    let original = read_surface(&baseline.encoded(), LIMITS).map_err(|_| ())?;
    let original_alternatives = original.points().first().ok_or(())?.alternatives();
    for case in 0u8..8 {
        let mut vector = baseline.clone();
        let point = vector.points.first_mut().ok_or(())?;
        match case {
            0 => vector.family.1 = b"other-family",
            1 => vector.policy = vec![8; 32],
            2 => point.owner.1 = b"other-law",
            3 => point.original = vec![254],
            4 => point.site.1 = b"other-site",
            5 => point.name.0 = b"0-point",
            6 => {
                point.alternatives.first_mut().ok_or(())?.family =
                    b"another-historical-family".to_vec();
            }
            _ => point.alternatives.first_mut().ok_or(())?.operation = vec![2],
        }
        point.ordered();
        let moved = read_surface(&vector.encoded(), LIMITS).map_err(|_| ())?;
        assert_ne!(moved.identity(), original.identity());
        assert_eq!(
            moved.identity().as_bytes(),
            &hash("evaluation-surface/v1", &vector.canonical())
        );
        let alternatives = moved.points().first().ok_or(())?.alternatives();
        if case < 5 {
            assert_eq!(alternatives, original_alternatives);
        } else {
            assert_ne!(alternatives, original_alternatives);
        }
        for (actual, expected) in alternatives
            .iter()
            .zip(&vector.points.first().ok_or(())?.alternatives)
        {
            assert_eq!(
                actual.identity().as_bytes(),
                &expected.address(vector.points.first().ok_or(())?.name)
            );
        }
    }
    Ok(())
}

#[test]
fn nested_surface_field_bounds_match_retention_before_preimage_allocation() -> Result<(), ()> {
    let mut vector = SurfaceVector::declared();
    let surface = live_surface(&mut vector)?;
    let encoded = vector.encoded();
    for field in [31usize, vector.canonical().len().saturating_sub(1)] {
        let limits = SurfaceArchiveLimits::declared(ArchiveLimits::declared(16384, field), 4, 4);
        assert_eq!(
            retain_surface(&surface, limits),
            Err(SurfaceArchiveRefusal::Record(ArchiveRefusal::FieldTooLarge))
        );
        assert_eq!(
            read_surface(&encoded, limits),
            Err(SurfaceArchiveRefusal::Record(ArchiveRefusal::FieldTooLarge))
        );
    }
    let mut two_families = SurfaceVector::declared();
    let point = two_families.points.first_mut().ok_or(())?;
    let mut alternate = point.alternatives.first().ok_or(())?.clone();
    alternate.family = b"separate-historical-family".to_vec();
    point.alternatives.push(alternate);
    point.ordered();
    assert!(read_surface(&two_families.encoded(), LIMITS).is_ok());
    Ok(())
}

#[test]
fn complete_surface_readback_survives_a_fresh_process() -> Result<(), Box<dyn Error>> {
    let mut vector = SurfaceVector::declared();
    let surface =
        live_surface(&mut vector).map_err(|()| std::io::Error::other("fixture refused"))?;
    let record = retain_surface(&surface, LIMITS)
        .map_err(|refusal| std::io::Error::other(format!("{refusal:?}")))?;
    drop(surface);
    assert_eq!(
        process::round_trip(record.encoded(), "archive::surfaces::child_loads_surface")?,
        record.encoded()
    );
    vector.points.clear();
    let encoded = vector.encoded();
    assert_eq!(
        process::round_trip(&encoded, "archive::surfaces::child_loads_surface")?,
        encoded
    );
    Ok(())
}
