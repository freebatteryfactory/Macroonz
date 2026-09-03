//! The policy roads: one claim's permission, and the owner-authored policy with the identity it derives.

use crate::descriptor::ClaimRef;
use crate::muterprater::OperatorFamilyRef;
use crate::muterprater::discovery::encode;
use crate::muterprater::discovery::types::{
    EvaluationFamilyRef, MUTATION_POLICY_TAG, MutationPermission, MutationPolicy, MutationPolicyId,
    PermissionRefusal, PolicyRefusal,
};

crate::identity::content_address_reference! {
    /// The policy's derived content address.
    value MutationPolicyId;
}

impl MutationPermission {
    /// One owner claim's nonempty roster of admitted operator families.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, then a family stated twice.
    pub fn declared(
        owner_claim: ClaimRef,
        mut admitted_families: Vec<OperatorFamilyRef>,
    ) -> Result<Self, PermissionRefusal> {
        if admitted_families.is_empty() {
            return Err(PermissionRefusal::NoOperatorFamily);
        }
        admitted_families.sort_by_key(|family| family.slug());
        for pair in admitted_families.windows(2) {
            let [left, right] = pair else {
                continue;
            };
            if left == right {
                return Err(PermissionRefusal::DuplicateOperatorFamily(*right));
            }
        }
        Ok(Self {
            owner_claim,
            admitted_families,
        })
    }

    /// The owner claim this permission is scoped to.
    #[must_use]
    pub const fn owner_claim(&self) -> ClaimRef {
        self.owner_claim
    }

    /// The operator families the owner admits for this claim, in canonical slug order.
    #[must_use]
    pub fn admitted_families(&self) -> &[OperatorFamilyRef] {
        &self.admitted_families
    }

    /// Whether this permission admits one operator family.
    #[must_use]
    pub fn admits(&self, family: OperatorFamilyRef) -> bool {
        self.admitted_families.contains(&family)
    }
}

impl MutationPolicy {
    /// One evaluation family's owner-authored mutation policy.
    ///
    /// An empty permission roster is lawful and admits a point-free evaluation surface; it earns no parity or mutation evidence by existing.
    ///
    /// # Errors
    ///
    /// Refuses two permission rows naming one claim.
    pub fn declared(
        family: EvaluationFamilyRef,
        mut permissions: Vec<MutationPermission>,
    ) -> Result<Self, PolicyRefusal> {
        permissions.sort_by_key(MutationPermission::owner_claim);
        for pair in permissions.windows(2) {
            let [left, right] = pair else {
                continue;
            };
            if left.owner_claim() == right.owner_claim() {
                return Err(PolicyRefusal::DuplicateClaim(right.owner_claim()));
            }
        }
        let preimage = encode::policy_preimage(family, &permissions);
        let identity = MutationPolicyId(encode::address(MUTATION_POLICY_TAG, &preimage));
        Ok(Self {
            family,
            permissions,
            identity,
        })
    }

    /// The evaluation family this policy belongs to.
    #[must_use]
    pub const fn family(&self) -> EvaluationFamilyRef {
        self.family
    }

    /// The policy's derived identity.
    #[must_use]
    pub const fn identity(&self) -> MutationPolicyId {
        self.identity
    }

    /// The policy's permissions, in canonical claim order.
    #[must_use]
    pub fn permissions(&self) -> &[MutationPermission] {
        &self.permissions
    }

    /// The permission row for one owner claim, where this policy carries one.
    #[must_use]
    pub fn permission(&self, claim: ClaimRef) -> Option<&MutationPermission> {
        self.permissions
            .iter()
            .find(|permission| permission.owner_claim() == claim)
    }
}
