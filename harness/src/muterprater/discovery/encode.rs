//! The canonical preimages behind the interpreted lane's four content identities.
//!
//! Every member is length-framed through the record vocabulary's one framing law, so no two different readings can share a preimage by accident.

use super::types::{
    AdmittedAlternative, DiscoveryEntry, EvaluationFamilyRef, MutationPermission, MutationPoint,
    MutationPolicyId, OwnerClaimMapping,
};
use crate::descriptor::{MutationPointRef, NamespacedName};
use crate::identity::{ContentAddress, DomainTag};
use crate::muterprater::OperatorFamilyRef;
use crate::report::{encode_bytes, encode_length};

/// Append one namespaced name, through the type's own seated spelling.
fn push_name(into: &mut Vec<u8>, name: NamespacedName) {
    name.encode_into(into);
}

/// Append one admitted alternative at its canonical surface width.
fn push_alternative(into: &mut Vec<u8>, alternative: &AdmittedAlternative) {
    encode_bytes(alternative.identity().address().as_bytes(), into);
    encode_bytes(alternative.family().slug().as_bytes(), into);
    encode_bytes(alternative.operation(), into);
}

/// The complete preimage of one owner-authored mutation policy.
///
/// The evaluation-family name, then the permission count and each permission in claim order.
/// A permission is its owner-claim name, then its family count and each operator-family slug in lexical order.
pub(super) fn policy_preimage(
    family: EvaluationFamilyRef,
    permissions: &[MutationPermission],
) -> Vec<u8> {
    let mut bytes = Vec::new();
    push_name(&mut bytes, family.name());
    encode_length(permissions.len(), &mut bytes);
    for permission in permissions {
        push_name(&mut bytes, permission.owner_claim().name());
        encode_length(permission.admitted_families().len(), &mut bytes);
        for admitted in permission.admitted_families() {
            encode_bytes(admitted.slug().as_bytes(), &mut bytes);
        }
    }
    bytes
}

/// The complete preimage of one admitted alternative.
///
/// The point's namespaced identity, the operator-family slug, and the canonical mutation bytes.
/// Roster position is absent, so reordering alternatives cannot rename them.
pub(super) fn alternative_preimage(
    point: MutationPointRef,
    family: OperatorFamilyRef,
    operation: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::new();
    push_name(&mut bytes, point.name());
    encode_bytes(family.slug().as_bytes(), &mut bytes);
    encode_bytes(operation, &mut bytes);
    bytes
}

/// The complete preimage of one evaluation surface.
///
/// The evaluation-family name, the policy address, the point count, and each point in point-identity order.
/// A point contributes its identity, its membership claim, its original bytes, its activation-site name, and its alternatives in alternative-identity order.
pub(super) fn surface_preimage(
    family: EvaluationFamilyRef,
    policy: MutationPolicyId,
    points: &[MutationPoint],
) -> Vec<u8> {
    let mut bytes = Vec::new();
    push_name(&mut bytes, family.name());
    encode_bytes(policy.address().as_bytes(), &mut bytes);
    encode_length(points.len(), &mut bytes);
    for point in points {
        push_name(&mut bytes, point.identity().name());
        push_name(&mut bytes, point.owner_claim().name());
        encode_bytes(point.original_operation(), &mut bytes);
        push_name(&mut bytes, point.activation_site().name());
        encode_length(point.admitted_alternatives().len(), &mut bytes);
        for alternative in point.admitted_alternatives() {
            push_alternative(&mut bytes, alternative);
        }
    }
    bytes
}

/// The complete preimage of one producer discovery reading.
///
/// The evaluation-family name, the owner-policy address, the discovery count, then every site in producer order.
/// A site contributes its point identity, its mapping posture and mapped claim where it has one, its unchanged operation, each candidate alternative's family slug and meaning bytes in producer order, and its activation site.
/// Admission disposition is derived from those facts and the addressed policy rather than encoded a second time.
pub(super) fn discovery_preimage(
    family: EvaluationFamilyRef,
    policy: MutationPolicyId,
    entries: &[DiscoveryEntry],
) -> Vec<u8> {
    let mut bytes = Vec::new();
    push_name(&mut bytes, family.name());
    encode_bytes(policy.address().as_bytes(), &mut bytes);
    encode_length(entries.len(), &mut bytes);
    for entry in entries {
        let site = entry.site();
        push_name(&mut bytes, site.identity().name());
        match site.mapping() {
            OwnerClaimMapping::Mapped(claim) => {
                bytes.push(1u8);
                push_name(&mut bytes, claim.name());
            }
            OwnerClaimMapping::OwnerUnmapped => bytes.push(0u8),
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

/// Derive one content address over a preimage, under the caller's domain tag.
pub(super) fn address(tag: DomainTag, preimage: &[u8]) -> ContentAddress {
    ContentAddress::derived(tag, preimage)
}
