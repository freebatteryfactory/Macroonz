//! Outside claims over owner policy, producer discovery, lowering, selection, and identity bytes.

use super::support::{
    MutationRoadFailure, OWNER, claim, discovered_point, family, operator, policy, surface_with,
};
use macroonz_harness::descriptor::{ClaimRef, MutationPointRef, NamespacedName};
use macroonz_harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use macroonz_harness::muterprater::discover::lower_discoveries;
use macroonz_harness::muterprater::{
    ActivationSite, AdmittedAlternative, AlternativeDeclaration, AlternativeId,
    DiscoveredMutationSite, DiscoveryDisposition, DiscoveryLoweringRefusal, DiscoveryRefusal,
    MappedUnpermittedCause, MutationDiscoveryReading, MutationPermission, MutationPoint,
    MutationPolicy, OperatorFamilyRef, OwnerClaimMapping, PermissionRefusal, PointCatalogPosture,
    PolicyRefusal, SelectionRefusal,
};
use macroonz_harness::report::{encode_bytes, encode_length};

const POLICY_READING_TAG: DomainTag =
    DomainTag::declared("mutation-policy", IdentityProfileVersion::declared(1));
const ALTERNATIVE_READING_TAG: DomainTag =
    DomainTag::declared("mutation-alternative", IdentityProfileVersion::declared(1));
const SURFACE_READING_TAG: DomainTag =
    DomainTag::declared("evaluation-surface", IdentityProfileVersion::declared(1));
const DISCOVERY_READING_TAG: DomainTag =
    DomainTag::declared("mutation-discovery", IdentityProfileVersion::declared(1));

fn push_name(into: &mut Vec<u8>, name: NamespacedName) {
    encode_bytes(name.namespace().written().as_bytes(), into);
    encode_bytes(name.stem().written().as_bytes(), into);
}

fn independently_frame_discovery(reading: &MutationDiscoveryReading) -> Vec<u8> {
    let mut bytes = Vec::new();
    push_name(&mut bytes, reading.family().name());
    encode_bytes(reading.policy().address().as_bytes(), &mut bytes);
    encode_length(reading.entries().len(), &mut bytes);
    for entry in reading.entries() {
        let site = entry.site();
        push_name(&mut bytes, site.identity().name());
        match site.mapping() {
            OwnerClaimMapping::Mapped(owner_claim) => {
                bytes.push(1);
                push_name(&mut bytes, owner_claim.name());
            }
            OwnerClaimMapping::OwnerUnmapped => bytes.push(0),
        }
        encode_bytes(site.original_operation(), &mut bytes);
        encode_length(site.alternatives().len(), &mut bytes);
        for alternative in site.alternatives() {
            encode_bytes(alternative.family().slug().as_bytes(), &mut bytes);
            encode_bytes(alternative.operation(), &mut bytes);
        }
        push_name(&mut bytes, site.activation_site().name());
    }
    bytes
}

fn point(
    policy: &MutationPolicy,
    stem: &'static str,
    alternatives: Vec<&'static [u8]>,
) -> Result<MutationPoint, MutationRoadFailure> {
    let discovered = discovered_point(stem, OwnerClaimMapping::Mapped(claim()?), alternatives)?;
    let lowered = lower_discoveries(policy, vec![discovered])?;
    lowered
        .surface()
        .points()
        .first()
        .cloned()
        .ok_or(MutationRoadFailure::MissingAlternative)
}

/// Claim: Policy identity and its readable rosters are canonical across authored permission and family order.
/// Subject: One two-claim policy carrying two operator families.
/// Population: Both authored permission orders and both authored family orders.
/// Hostile control: Reversing both input rosters must preserve the canonical reading and identity.
/// Denominator: Every permission and operator family in both policy constructions.
/// Evidence ceiling: This outside test establishes canonical policy values and identity only, not discovery admission.
/// Retained regression: Authored roster order becoming identity-significant remains a permanent owner regression.
#[test]
fn policy_identity_is_canonical_across_authored_roster_order() -> Result<(), MutationRoadFailure> {
    let comparison_claim = claim()?;
    let boolean_claim =
        ClaimRef::named(OWNER, "boolean-behaviour").map_err(|_| MutationRoadFailure::Name)?;
    let comparison_family = operator()?;
    let boolean_family = OperatorFamilyRef::of_slug("boolean-operators")
        .ok_or(MutationRoadFailure::MissingFamily)?;
    let comparison_permission =
        MutationPermission::declared(comparison_claim, vec![comparison_family, boolean_family])?;
    let comparison_permission_reversed =
        MutationPermission::declared(comparison_claim, vec![boolean_family, comparison_family])?;
    let boolean_permission = MutationPermission::declared(boolean_claim, vec![boolean_family])?;
    let forward = MutationPolicy::declared(
        family("canonical-policy-family")?,
        vec![comparison_permission, boolean_permission.clone()],
    )?;
    let reversed = MutationPolicy::declared(
        family("canonical-policy-family")?,
        vec![boolean_permission, comparison_permission_reversed],
    )?;
    assert_eq!(forward.identity(), reversed.identity());
    assert_eq!(forward.permissions(), reversed.permissions());
    let Some(comparison) = forward.permission(comparison_claim) else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(
        comparison.admitted_families(),
        &[boolean_family, comparison_family]
    );
    Ok(())
}

/// Claim: Permission and policy constructors refuse their malformed inputs in documented order.
/// Subject: One claim's operator-family permission and one evaluation-family policy.
/// Population: Empty, duplicate-family, and duplicate-claim authored rosters.
/// Hostile control: Each fixture reverses exactly one structural constructor clause.
/// Denominator: Every public permission and policy refusal variant.
/// Evidence ceiling: This outside test establishes constructor refusal values and order only, not compile-time unwritability.
/// Retained regression: Reordered or skipped structural refusals remain permanent owner regressions.
#[test]
fn permission_and_policy_refusal_order_is_observable() -> Result<(), MutationRoadFailure> {
    let owner_claim = claim()?;
    let admitted_family = operator()?;
    assert_eq!(
        MutationPermission::declared(owner_claim, Vec::new()),
        Err(PermissionRefusal::NoOperatorFamily)
    );
    assert_eq!(
        MutationPermission::declared(owner_claim, vec![admitted_family, admitted_family]),
        Err(PermissionRefusal::DuplicateOperatorFamily(admitted_family))
    );
    let permission = MutationPermission::declared(owner_claim, vec![admitted_family])?;
    assert_eq!(
        MutationPolicy::declared(
            family("policy-refusal-family")?,
            vec![permission.clone(), permission],
        ),
        Err(PolicyRefusal::DuplicateClaim(owner_claim))
    );
    Ok(())
}

/// Claim: A discovered site refuses malformed structural inputs in documented order.
/// Subject: One producer-authored mutation site before policy admission.
/// Population: Empty original bytes, no alternatives, empty alternative bytes, unchanged bytes, and a duplicate meaning.
/// Hostile control: Each fixture reverses exactly one constructor clause while keeping the remaining inputs readable.
/// Denominator: Every public discovery refusal variant and its producer position.
/// Evidence ceiling: This outside test establishes constructor refusal values and order only, not lowering or compile-time unwritability.
/// Retained regression: Reordered, skipped, or position-erasing discovery refusals remain permanent owner regressions.
#[test]
fn discovered_site_refusal_order_is_observable() -> Result<(), MutationRoadFailure> {
    let identity = MutationPointRef::named(OWNER, "discovery-refusal")
        .map_err(|_| MutationRoadFailure::Name)?;
    let activation =
        ActivationSite::named(OWNER, "discovery-refusal").map_err(|_| MutationRoadFailure::Name)?;
    let mapping = OwnerClaimMapping::Mapped(claim()?);
    let admitted_family = operator()?;
    let replacement = AlternativeDeclaration::stated(admitted_family, b"a == b".to_vec());
    assert_eq!(
        DiscoveredMutationSite::discovered(
            identity,
            mapping,
            Vec::new(),
            vec![replacement.clone()],
            activation,
        ),
        Err(DiscoveryRefusal::EmptyOriginalOperation)
    );
    assert_eq!(
        DiscoveredMutationSite::discovered(
            identity,
            mapping,
            b"a != b".to_vec(),
            Vec::new(),
            activation,
        ),
        Err(DiscoveryRefusal::NoAlternative)
    );
    assert_eq!(
        DiscoveredMutationSite::discovered(
            identity,
            mapping,
            b"a != b".to_vec(),
            vec![AlternativeDeclaration::stated(admitted_family, Vec::new())],
            activation,
        ),
        Err(DiscoveryRefusal::EmptyAlternative { at: 0 })
    );
    assert_eq!(
        DiscoveredMutationSite::discovered(
            identity,
            mapping,
            b"a != b".to_vec(),
            vec![AlternativeDeclaration::stated(
                admitted_family,
                b"a != b".to_vec(),
            )],
            activation,
        ),
        Err(DiscoveryRefusal::AlternativeIsOriginal { at: 0 })
    );
    assert_eq!(
        DiscoveredMutationSite::discovered(
            identity,
            mapping,
            b"a != b".to_vec(),
            vec![replacement.clone(), replacement],
            activation,
        ),
        Err(DiscoveryRefusal::DuplicateAlternativeMeaning { at: 1 })
    );
    Ok(())
}

/// Claim: Policy admission and stable alternative identities are structural rather than roster-positional.
/// Subject: One owner policy, its admitted point, and the point-free posture.
/// Population: Two permutations of one alternative roster, one empty roster, and two operator families carrying equal bytes.
/// Hostile control: Reordering inputs, omitting alternatives, and changing only the operator family reverse the asserted clauses.
/// Denominator: Every public policy, lowering, point, alternative, selection, and posture value used by this fixture.
/// Evidence ceiling: This outside test establishes value and byte behavior only, not producer coverage or runtime execution.
/// Retained regression: The claim remains here after extraction from the former monolithic integration target.
#[test]
fn policy_admission_and_identity_are_structural() -> Result<(), MutationRoadFailure> {
    let evaluation_family = family("comparison-family")?;
    let first = surface_with(evaluation_family, vec![b"a <= b", b"a > b"])?;
    let reordered = surface_with(evaluation_family, vec![b"a > b", b"a <= b"])?;
    assert_eq!(first.identity(), reordered.identity());
    let first_point = first
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let reordered_point = reordered
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let first_ids = first_point
        .admitted_alternatives()
        .iter()
        .map(AdmittedAlternative::identity)
        .collect::<Vec<AlternativeId>>();
    let reordered_ids = reordered_point
        .admitted_alternatives()
        .iter()
        .map(AdmittedAlternative::identity)
        .collect::<Vec<AlternativeId>>();
    assert_eq!(first_ids, reordered_ids);

    let policy = policy(evaluation_family)?;
    let point_free = lower_discoveries(&policy, Vec::new())?.into_parts().1;
    assert_eq!(
        point_free.catalog_posture(),
        PointCatalogPosture::NoAdmittedPoints
    );
    assert!(point_free.selections().is_empty());
    assert_eq!(
        discovered_point(
            "empty-point",
            OwnerClaimMapping::Mapped(claim()?),
            Vec::new(),
        ),
        Err(MutationRoadFailure::Discovery(
            DiscoveryRefusal::NoAlternative
        ))
    );

    let boolean_family = OperatorFamilyRef::of_slug("boolean-operators")
        .ok_or(MutationRoadFailure::MissingFamily)?;
    let two_family_policy = MutationPolicy::declared(
        family("two-operator-family")?,
        vec![MutationPermission::declared(
            claim()?,
            vec![operator()?, boolean_family],
        )?],
    )?;
    let discovered = DiscoveredMutationSite::discovered(
        MutationPointRef::named(OWNER, "operator-identity")
            .map_err(|_| MutationRoadFailure::Name)?,
        OwnerClaimMapping::Mapped(claim()?),
        b"a < b".to_vec(),
        vec![
            AlternativeDeclaration::stated(operator()?, b"a > b".to_vec()),
            AlternativeDeclaration::stated(boolean_family, b"a > b".to_vec()),
        ],
        ActivationSite::named(OWNER, "operator-identity").map_err(|_| MutationRoadFailure::Name)?,
    )?;
    let lowering = lower_discoveries(&two_family_policy, vec![discovered])?;
    let same_bytes_under_two_operators = lowering
        .surface()
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let first_alternative = same_bytes_under_two_operators
        .admitted_alternatives()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let last_alternative = same_bytes_under_two_operators
        .admitted_alternatives()
        .last()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    assert_ne!(first_alternative.identity(), last_alternative.identity());
    Ok(())
}

/// Claim: Complete discovery retains every offered site while admitting only the exact owner-mapped and policy-permitted subset.
/// Subject: One four-site producer roster lowered against one owner policy.
/// Population: A mapped site, an owner-unmapped site, an unpermitted-family site, and an unpermitted-claim site.
/// Hostile control: Each non-admitted posture reverses one mapping or permission clause and is refused by surface selection.
/// Denominator: All four sites in producer order and the complete executable surface derived from them.
/// Evidence ceiling: This outside test establishes lowering and selection behavior only, not producer completeness.
/// Retained regression: The four-way disposition and withheld-selection checks remain permanent owner regressions.
#[test]
fn mutation_constructor_and_selection_boundaries_refuse_crossed_joins()
-> Result<(), MutationRoadFailure> {
    let first_family = family("constructor-family")?;
    let first_policy = policy(first_family)?;
    let boolean_family = OperatorFamilyRef::of_slug("boolean-operators")
        .ok_or(MutationRoadFailure::MissingFamily)?;
    let mapped = discovered_point(
        "first-point",
        OwnerClaimMapping::Mapped(claim()?),
        vec![b"a <= b"],
    )?;
    let owner_unmapped = discovered_point(
        "owner-unmapped-point",
        OwnerClaimMapping::OwnerUnmapped,
        vec![b"a >= b"],
    )?;
    let unpermitted_family = DiscoveredMutationSite::discovered(
        MutationPointRef::named(OWNER, "foreign-family-point")
            .map_err(|_| MutationRoadFailure::Name)?,
        OwnerClaimMapping::Mapped(claim()?),
        b"a < b".to_vec(),
        vec![AlternativeDeclaration::stated(
            boolean_family,
            b"true".to_vec(),
        )],
        ActivationSite::named(OWNER, "foreign-family-point")
            .map_err(|_| MutationRoadFailure::Name)?,
    )?;
    let another_claim =
        ClaimRef::named(OWNER, "unpermitted-claim").map_err(|_| MutationRoadFailure::Name)?;
    let unpermitted_claim = discovered_point(
        "foreign-claim-point",
        OwnerClaimMapping::Mapped(another_claim),
        vec![b"a == b"],
    )?;
    let mapped_ref = mapped.identity();
    let unmapped_ref = owner_unmapped.identity();
    let family_ref = unpermitted_family.identity();
    let claim_ref = unpermitted_claim.identity();
    let lowering = lower_discoveries(
        &first_policy,
        vec![
            mapped.clone(),
            owner_unmapped,
            unpermitted_family,
            unpermitted_claim,
        ],
    )?;
    let entries = lowering.discovery().entries();
    let [mapped_entry, unmapped_entry, family_entry, claim_entry] = entries else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    assert_eq!(
        mapped_entry.disposition(),
        DiscoveryDisposition::Mapped { point: mapped_ref }
    );
    assert_eq!(
        unmapped_entry.disposition(),
        DiscoveryDisposition::OwnerUnmapped
    );
    assert_eq!(
        family_entry.disposition(),
        DiscoveryDisposition::MappedUnpermitted {
            cause: MappedUnpermittedCause::Family {
                at: 0,
                family: boolean_family,
            },
        }
    );
    assert_eq!(
        claim_entry.disposition(),
        DiscoveryDisposition::MappedUnpermitted {
            cause: MappedUnpermittedCause::Claim(another_claim),
        }
    );
    let first_surface = lowering.surface();
    let [admitted_point] = first_surface.points() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    let [admitted] = admitted_point.admitted_alternatives() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    let admitted_alternative = admitted.identity();
    assert!(matches!(
        first_surface.select(unmapped_ref, admitted_alternative),
        Err(SelectionRefusal::NoSuchPoint(found))
            if found == unmapped_ref
    ));
    assert!(matches!(
        first_surface.select(family_ref, admitted_alternative),
        Err(SelectionRefusal::NoSuchPoint(found))
            if found == family_ref
    ));
    assert!(matches!(
        first_surface.select(claim_ref, admitted_alternative),
        Err(SelectionRefusal::NoSuchPoint(found))
            if found == claim_ref
    ));
    Ok(())
}

/// Claim: Admission is all-or-nothing for each discovered site's candidate roster.
/// Subject: One mapped site carrying one permitted and one unpermitted operator family.
/// Population: The complete two-alternative producer roster and its derived surface.
/// Hostile control: The second alternative reverses family permission while the first remains permitted.
/// Denominator: The sole discovered site, both candidates, its disposition, and the complete surface.
/// Evidence ceiling: This outside test establishes admission behavior only, not runtime activation or mutation verdicts.
/// Retained regression: Silent narrowing of a mixed roster remains a permanent owner regression.
#[test]
fn mixed_discovery_rosters_are_admitted_all_or_nothing() -> Result<(), MutationRoadFailure> {
    let policy = policy(family("mixed-roster-family")?)?;
    let boolean_family = OperatorFamilyRef::of_slug("boolean-operators")
        .ok_or(MutationRoadFailure::MissingFamily)?;
    let site = DiscoveredMutationSite::discovered(
        MutationPointRef::named(OWNER, "mixed-family-point")
            .map_err(|_| MutationRoadFailure::Name)?,
        OwnerClaimMapping::Mapped(claim()?),
        b"a != b".to_vec(),
        vec![
            AlternativeDeclaration::stated(operator()?, b"a == b".to_vec()),
            AlternativeDeclaration::stated(boolean_family, b"true".to_vec()),
        ],
        ActivationSite::named(OWNER, "mixed-family-point")
            .map_err(|_| MutationRoadFailure::Name)?,
    )?;
    let point = site.identity();
    let lowering = lower_discoveries(&policy, vec![site])?;
    assert!(matches!(
        lowering.discovery().entries(),
        [entry] if entry.disposition() == DiscoveryDisposition::MappedUnpermitted {
            cause: MappedUnpermittedCause::Family {
                at: 1,
                family: boolean_family,
            },
        }
    ));
    assert!(lowering.surface().points().is_empty());
    assert!(
        lowering
            .surface()
            .points()
            .iter()
            .all(|found| found.identity() != point)
    );
    Ok(())
}

/// Claim: Policy, alternative, and surface identities are derived from their documented framed owner facts.
/// Subject: One admitted policy, point, alternative roster, and evaluation surface.
/// Population: Every field in the policy, alternative, and surface preimages built by this fixture.
/// Hostile control: Each expected address is derived independently rather than read back from the owning encoder.
/// Denominator: The complete policy roster, all admitted alternatives, and the complete surface point roster.
/// Evidence ceiling: This outside test establishes exact identity preimages and bytes only, not collision resistance.
/// Retained regression: Framing, field-order, and domain-tag drift remain permanent owner regressions.
#[test]
fn mutation_identity_preimages_are_independently_read() -> Result<(), MutationRoadFailure> {
    let family = family("identity-family")?;
    let policy = policy(family)?;
    let point = point(&policy, "identity-point", vec![b"a <= b", b"a > b"])?;

    let mut policy_preimage = Vec::new();
    push_name(&mut policy_preimage, family.name());
    encode_length(policy.permissions().len(), &mut policy_preimage);
    for permission in policy.permissions() {
        push_name(&mut policy_preimage, permission.owner_claim().name());
        encode_length(permission.admitted_families().len(), &mut policy_preimage);
        for admitted in permission.admitted_families() {
            encode_bytes(admitted.slug().as_bytes(), &mut policy_preimage);
        }
    }
    assert_eq!(
        policy.identity().address(),
        ContentAddress::derived(POLICY_READING_TAG, &policy_preimage)
    );

    for alternative in point.admitted_alternatives() {
        let mut alternative_preimage = Vec::new();
        push_name(&mut alternative_preimage, point.identity().name());
        encode_bytes(
            alternative.family().slug().as_bytes(),
            &mut alternative_preimage,
        );
        encode_bytes(alternative.operation(), &mut alternative_preimage);
        assert_eq!(
            alternative.identity().address(),
            ContentAddress::derived(ALTERNATIVE_READING_TAG, &alternative_preimage)
        );
    }

    let surface = lower_discoveries(
        &policy,
        vec![discovered_point(
            "identity-point",
            OwnerClaimMapping::Mapped(claim()?),
            vec![b"a <= b", b"a > b"],
        )?],
    )?
    .into_parts()
    .1;
    let mut surface_preimage = Vec::new();
    push_name(&mut surface_preimage, family.name());
    encode_bytes(
        policy.identity().address().as_bytes(),
        &mut surface_preimage,
    );
    encode_length(surface.points().len(), &mut surface_preimage);
    for surface_point in surface.points() {
        push_name(&mut surface_preimage, surface_point.identity().name());
        push_name(&mut surface_preimage, surface_point.owner_claim().name());
        encode_bytes(surface_point.original_operation(), &mut surface_preimage);
        push_name(
            &mut surface_preimage,
            surface_point.activation_site().name(),
        );
        encode_length(
            surface_point.admitted_alternatives().len(),
            &mut surface_preimage,
        );
        for alternative in surface_point.admitted_alternatives() {
            encode_bytes(
                alternative.identity().address().as_bytes(),
                &mut surface_preimage,
            );
            encode_bytes(
                alternative.family().slug().as_bytes(),
                &mut surface_preimage,
            );
            encode_bytes(alternative.operation(), &mut surface_preimage);
        }
    }
    assert_eq!(
        surface.identity().address(),
        ContentAddress::derived(SURFACE_READING_TAG, &surface_preimage)
    );

    Ok(())
}

/// Claim: Discovery identity preserves producer order while surface identity preserves canonical point order.
/// Subject: One two-site roster lowered in forward and reversed producer order.
/// Population: Both complete discovery readings and both complete executable surfaces.
/// Hostile control: Reversing only producer order must move discovery identity and leave surface identity unchanged.
/// Denominator: Both producer sites, every discovery entry, and both canonical surfaces.
/// Evidence ceiling: This outside test establishes ordering and identity bytes only, not producer completeness.
/// Retained regression: Producer-order and canonical-order conflation remains a permanent owner regression.
#[test]
fn discovery_identity_and_surface_identity_keep_their_own_ordering()
-> Result<(), MutationRoadFailure> {
    let policy = policy(family("discovery-identity-family")?)?;
    let first = discovered_point(
        "producer-order-first",
        OwnerClaimMapping::Mapped(claim()?),
        vec![b"a <= b"],
    )?;
    let second = discovered_point(
        "producer-order-second",
        OwnerClaimMapping::Mapped(claim()?),
        vec![b"a >= b"],
    )?;
    let forward = lower_discoveries(&policy, vec![first.clone(), second.clone()])?;
    let reversed = lower_discoveries(&policy, vec![second, first])?;
    assert_eq!(
        forward.discovery().identity().address(),
        ContentAddress::derived(
            DISCOVERY_READING_TAG,
            &independently_frame_discovery(forward.discovery()),
        )
    );
    assert_ne!(
        forward.discovery().identity(),
        reversed.discovery().identity()
    );
    assert_eq!(forward.surface().identity(), reversed.surface().identity());
    Ok(())
}

/// Claim: a surface refuses absent points and alternatives borrowed from another point.
///
/// Subject: the public discovery lowering and surface selection roads.
/// Population: one duplicate input and one two-point admitted surface.
/// Hostile control: the selection attempts an absent point and a sibling point's alternative.
/// Denominator: every selection refusal coordinate exposed by the two-point fixture.
/// Evidence ceiling: this establishes discovery-owned selection boundaries for one outside fixture and says nothing about interpretation execution.
/// Retained regression: this discovery composition claim remains in the original integration target.
#[test]
fn surface_selection_refuses_absent_points_and_crossed_alternatives()
-> Result<(), MutationRoadFailure> {
    let first_family = family("constructor-family")?;
    let first_policy = policy(first_family)?;
    let duplicate = discovered_point(
        "duplicate-selection-point",
        OwnerClaimMapping::Mapped(claim()?),
        vec![b"a <= b"],
    )?;
    let duplicate_ref = duplicate.identity();
    assert!(matches!(
        lower_discoveries(&first_policy, vec![duplicate.clone(), duplicate]),
        Err(DiscoveryLoweringRefusal::DuplicateSite { at: 1, point }) if point == duplicate_ref
    ));
    let two = lower_discoveries(
        &first_policy,
        vec![
            discovered_point(
                "selection-first",
                OwnerClaimMapping::Mapped(claim()?),
                vec![b"a <= b"],
            )?,
            discovered_point(
                "selection-second",
                OwnerClaimMapping::Mapped(claim()?),
                vec![b"a >= b"],
            )?,
        ],
    )?;
    let [first_point, second_point] = two.surface().points() else {
        return Err(MutationRoadFailure::MissingAlternative);
    };
    let first_point_ref = first_point.identity();
    let second_alternative = second_point
        .admitted_alternatives()
        .first()
        .map(AdmittedAlternative::identity)
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let absent_point =
        MutationPointRef::named(OWNER, "absent-point").map_err(|_| MutationRoadFailure::Name)?;
    assert_eq!(
        two.surface().select(absent_point, second_alternative,),
        Err(SelectionRefusal::NoSuchPoint(absent_point))
    );
    assert_eq!(
        two.surface().select(first_point_ref, second_alternative),
        Err(SelectionRefusal::NoSuchAlternative {
            point: first_point_ref,
            alternative: second_alternative,
        })
    );

    Ok(())
}
