//! The canonical preimages of the interpreted mutation lane's policy, alternative, and evaluation-surface identities.

use super::{
    AdmittedAlternative, DiscoveryEntry, EvaluationFamilyRef, MutationPermission, MutationPoint,
    MutationPolicyId, OwnerClaimMapping,
};
use crate::descriptor::NamespacedName;
use crate::identity::ContentAddress;
use crate::report::{encode_bytes, encode_length};

/// Append one namespaced name as its two length-framed authored parts.
fn push_name(into: &mut Vec<u8>, name: NamespacedName) {
    encode_bytes(name.namespace().written().as_bytes(), into);
    encode_bytes(name.stem().written().as_bytes(), into);
}

/// The complete preimage of one owner-authored mutation policy.
///
/// Members are the evaluation-family name followed by the permission count and each permission in claim order. A permission is its owner-claim name followed by its family count and each operator-family slug in lexical order.
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
/// Members are the point's namespaced identity, the operator-family slug, and the canonical mutation bytes. Roster position is absent, so reordering alternatives cannot rename them.
pub(super) fn alternative_preimage(
    point: crate::descriptor::MutationPointRef,
    family: super::OperatorFamilyRef,
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
/// Members are the evaluation-family name, policy address, point count, and each point in point-identity order. A point contributes its identity, policy membership claim, original bytes, activation-site name, alternative count, and each admitted alternative in alternative-identity order.
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
/// Members are the evaluation-family name, owner-policy address, discovery count, then every site in producer order. A site contributes its point identity, owner-mapping posture and mapped claim where present, unchanged operation, every candidate alternative's operator-family slug and exact meaning bytes in producer order, and its activation site. Admission disposition is derived from these facts and the addressed policy rather than encoded a second time.
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

/// Append one admitted alternative at its canonical surface width.
fn push_alternative(into: &mut Vec<u8>, alternative: &AdmittedAlternative) {
    encode_bytes(alternative.identity().address().as_bytes(), into);
    encode_bytes(alternative.family().slug().as_bytes(), into);
    encode_bytes(alternative.operation(), into);
}

/// Derive one content address over a preimage under the caller's family tag.
pub(super) fn address(tag: crate::identity::DomainTag, preimage: &[u8]) -> ContentAddress {
    ContentAddress::derived(tag, preimage)
}
