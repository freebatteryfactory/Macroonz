//! The invariant nucleus of owner policy, discovery, surfaces, and directives.

use super::super::encode;
use super::{
    ActivationSite, ActiveSelection, AdmittedAlternative, AlternativeDeclaration, AlternativeId,
    ClaimRef, ContentAddress, DiscoveredMutationSite, DiscoveryDisposition, DiscoveryEntry,
    DiscoveryRefusal, EVALUATION_SURFACE_TAG, EvaluationDirective, EvaluationFamilyRef,
    EvaluationSurface, EvaluationSurfaceId, MUTATION_ALTERNATIVE_TAG, MUTATION_DISCOVERY_TAG,
    MUTATION_POLICY_TAG, MutationDiscoveryId, MutationDiscoveryReading, MutationPermission,
    MutationPoint, MutationPointRef, MutationPolicy, MutationPolicyId, MutationSurfaceLowering,
    OperatorFamilyRef, OwnerClaimMapping, PermissionRefusal, PointCatalogPosture, PolicyMembership,
    PolicyRefusal, ResolvedMutation, SelectionRefusal,
};
use crate::descriptor::namespaced_reference;

namespaced_reference!(EvaluationFamilyRef, ActivationSite);

impl MutationPolicyId {
    /// The policy's derived content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
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

impl PolicyMembership {
    /// The policy that issued this membership.
    #[must_use]
    pub const fn policy(self) -> MutationPolicyId {
        self.policy
    }

    /// The owner claim this membership is scoped to.
    #[must_use]
    pub const fn owner_claim(self) -> ClaimRef {
        self.owner_claim
    }
}

// ---------------------------------------------------------------------------
// Producer discovery.
// ---------------------------------------------------------------------------

impl AlternativeDeclaration {
    /// One discovered operator family and canonical mutation meaning, before policy admission.
    #[must_use]
    pub fn stated(family: OperatorFamilyRef, operation: Vec<u8>) -> Self {
        Self { family, operation }
    }

    /// The operator family the producer attributes this meaning to.
    #[must_use]
    pub const fn family(&self) -> OperatorFamilyRef {
        self.family
    }

    /// The canonical mutation meaning supplied for admission.
    #[must_use]
    pub fn operation(&self) -> &[u8] {
        &self.operation
    }
}

impl DiscoveredMutationSite {
    /// Read one complete discovered site, before owner-policy admission.
    ///
    /// # Errors
    ///
    /// Refuses an empty unchanged operation, an empty alternative roster, then each alternative whose bytes are empty, equal the unchanged operation, or duplicate an earlier family and meaning.
    pub fn discovered(
        identity: MutationPointRef,
        mapping: OwnerClaimMapping,
        original_operation: Vec<u8>,
        alternatives: Vec<AlternativeDeclaration>,
        activation_site: ActivationSite,
    ) -> Result<Self, DiscoveryRefusal> {
        if original_operation.is_empty() {
            return Err(DiscoveryRefusal::EmptyOriginalOperation);
        }
        if alternatives.is_empty() {
            return Err(DiscoveryRefusal::NoAlternative);
        }
        for (at, alternative) in alternatives.iter().enumerate() {
            if alternative.operation().is_empty() {
                return Err(DiscoveryRefusal::EmptyAlternative { at });
            }
            if alternative.operation() == original_operation {
                return Err(DiscoveryRefusal::AlternativeIsOriginal { at });
            }
            if alternatives.iter().take(at).any(|earlier| {
                earlier.family() == alternative.family()
                    && earlier.operation() == alternative.operation()
            }) {
                return Err(DiscoveryRefusal::DuplicateAlternativeMeaning { at });
            }
        }
        Ok(Self {
            identity,
            mapping,
            original_operation,
            alternatives,
            activation_site,
        })
    }

    /// The stable point identity the producer discovered.
    #[must_use]
    pub const fn identity(&self) -> MutationPointRef {
        self.identity
    }

    /// The origin reading's owner-claim mapping posture.
    #[must_use]
    pub const fn mapping(&self) -> OwnerClaimMapping {
        self.mapping
    }

    /// The unchanged operation at this site.
    #[must_use]
    pub fn original_operation(&self) -> &[u8] {
        &self.original_operation
    }

    /// Every discovered alternative, in producer order.
    #[must_use]
    pub fn alternatives(&self) -> &[AlternativeDeclaration] {
        &self.alternatives
    }

    /// The named activation site the producer discovered.
    #[must_use]
    pub const fn activation_site(&self) -> ActivationSite {
        self.activation_site
    }
}

impl DiscoveryEntry {
    /// Retain one discovered site and its derived owner-policy disposition.
    pub(in crate::muterprater) fn recorded(
        site: DiscoveredMutationSite,
        disposition: DiscoveryDisposition,
    ) -> Self {
        Self { site, disposition }
    }

    /// The complete discovered site.
    #[must_use]
    pub const fn site(&self) -> &DiscoveredMutationSite {
        &self.site
    }

    /// Whether and why this site entered the executable surface.
    #[must_use]
    pub const fn disposition(&self) -> DiscoveryDisposition {
        self.disposition
    }
}

impl MutationDiscoveryId {
    /// The discovery reading's content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl MutationDiscoveryReading {
    /// Retain one complete discovery denominator and derive its content identity.
    pub(in crate::muterprater) fn recorded(
        policy: &MutationPolicy,
        entries: Vec<DiscoveryEntry>,
    ) -> Self {
        let preimage = encode::discovery_preimage(policy.family(), policy.identity(), &entries);
        Self {
            family: policy.family(),
            policy: policy.identity(),
            identity: MutationDiscoveryId(encode::address(MUTATION_DISCOVERY_TAG, &preimage)),
            entries,
        }
    }

    /// The evaluation family whose discovery was read.
    #[must_use]
    pub const fn family(&self) -> EvaluationFamilyRef {
        self.family
    }

    /// The owner policy the discovered sites were admitted against.
    #[must_use]
    pub const fn policy(&self) -> MutationPolicyId {
        self.policy
    }

    /// The content identity of the complete discovery denominator.
    #[must_use]
    pub const fn identity(&self) -> MutationDiscoveryId {
        self.identity
    }

    /// Every discovered site and disposition, in producer order.
    #[must_use]
    pub fn entries(&self) -> &[DiscoveryEntry] {
        &self.entries
    }
}

impl MutationSurfaceLowering {
    /// Bind one complete discovery reading to the executable surface derived from it.
    pub(in crate::muterprater) fn lowered(
        discovery: MutationDiscoveryReading,
        surface: EvaluationSurface,
    ) -> Self {
        Self { discovery, surface }
    }

    /// The complete discovery denominator.
    #[must_use]
    pub const fn discovery(&self) -> &MutationDiscoveryReading {
        &self.discovery
    }

    /// The executable subset admitted by owner policy.
    #[must_use]
    pub const fn surface(&self) -> &EvaluationSurface {
        &self.surface
    }

    /// Consume the closed lowering into its reading and its executable surface.
    #[must_use]
    pub fn into_parts(self) -> (MutationDiscoveryReading, EvaluationSurface) {
        (self.discovery, self.surface)
    }
}

// ---------------------------------------------------------------------------
// The evaluation surface.
// ---------------------------------------------------------------------------

impl AlternativeId {
    /// The alternative's derived content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl AdmittedAlternative {
    /// The alternative's stable identity.
    #[must_use]
    pub const fn identity(&self) -> AlternativeId {
        self.identity
    }

    /// The owner-permitted operator family this alternative realizes.
    #[must_use]
    pub const fn family(&self) -> OperatorFamilyRef {
        self.family
    }

    /// The canonical mutation meaning selected at runtime.
    #[must_use]
    pub fn operation(&self) -> &[u8] {
        &self.operation
    }
}

impl MutationPoint {
    /// Admit one structurally read, mapped, and policy-permitted discovery.
    pub(in crate::muterprater) fn admitted(
        policy: &MutationPolicy,
        owner_claim: ClaimRef,
        discovered: DiscoveredMutationSite,
    ) -> Self {
        let identity = discovered.identity;
        let mut admitted = Vec::new();
        for alternative in discovered.alternatives {
            let preimage = encode::alternative_preimage(
                identity,
                alternative.family(),
                alternative.operation(),
            );
            admitted.push(AdmittedAlternative {
                identity: AlternativeId(encode::address(MUTATION_ALTERNATIVE_TAG, &preimage)),
                family: alternative.family(),
                operation: alternative.operation,
            });
        }
        admitted.sort_by_key(AdmittedAlternative::identity);
        Self {
            identity,
            membership: PolicyMembership {
                policy: policy.identity(),
                owner_claim,
            },
            original_operation: discovered.original_operation,
            admitted_alternatives: admitted,
            activation_site: discovered.activation_site,
        }
    }

    /// The reference this point is known by.
    #[must_use]
    pub const fn identity(&self) -> MutationPointRef {
        self.identity
    }

    /// The policy-issued membership this point carries.
    #[must_use]
    pub const fn membership(&self) -> PolicyMembership {
        self.membership
    }

    /// The claim that owns the behaviour at this point.
    #[must_use]
    pub const fn owner_claim(&self) -> ClaimRef {
        self.membership.owner_claim()
    }

    /// What the point reads as under no mutation.
    #[must_use]
    pub fn original_operation(&self) -> &[u8] {
        &self.original_operation
    }

    /// The damages this point may be selected into, in canonical alternative-identity order.
    #[must_use]
    pub fn admitted_alternatives(&self) -> &[AdmittedAlternative] {
        &self.admitted_alternatives
    }

    /// Where a selected alternative fires.
    #[must_use]
    pub const fn activation_site(&self) -> ActivationSite {
        self.activation_site
    }
}

impl EvaluationSurfaceId {
    /// The surface's derived content address.
    #[must_use]
    pub const fn address(self) -> ContentAddress {
        self.0
    }
}

impl EvaluationSurface {
    /// Assemble an already policy-issued, identity-distinct point roster.
    pub(in crate::muterprater) fn admitted(
        policy: &MutationPolicy,
        mut points: Vec<MutationPoint>,
    ) -> Self {
        points.sort_by_key(MutationPoint::identity);
        let preimage = encode::surface_preimage(policy.family(), policy.identity(), &points);
        let identity = EvaluationSurfaceId(encode::address(EVALUATION_SURFACE_TAG, &preimage));
        Self {
            family: policy.family(),
            policy: policy.identity(),
            identity,
            points,
        }
    }

    /// The evaluation family this surface belongs to.
    #[must_use]
    pub const fn family(&self) -> EvaluationFamilyRef {
        self.family
    }

    /// The owner policy this surface was admitted under.
    #[must_use]
    pub const fn policy(&self) -> MutationPolicyId {
        self.policy
    }

    /// The exact surface identity.
    #[must_use]
    pub const fn identity(&self) -> EvaluationSurfaceId {
        self.identity
    }

    /// Whether this surface admits an active directive.
    #[must_use]
    pub const fn catalog_posture(&self) -> PointCatalogPosture {
        if self.points.is_empty() {
            PointCatalogPosture::NoAdmittedPoints
        } else {
            PointCatalogPosture::Mutable
        }
    }

    /// Every point the table carries, in canonical point-identity order.
    #[must_use]
    pub fn points(&self) -> &[MutationPoint] {
        &self.points
    }

    /// The point this reference names, where the table carries one.
    #[must_use]
    pub fn point(&self, identity: MutationPointRef) -> Option<&MutationPoint> {
        self.points
            .iter()
            .find(|point| point.identity() == identity)
    }

    /// Resolve one surface-issued selection to the point and alternative it names.
    ///
    /// # Errors
    ///
    /// Refuses a selection issued by another surface, then a point or alternative this surface does not carry.
    pub(in crate::muterprater) fn selected_alternative(
        &self,
        selection: ActiveSelection,
    ) -> Result<(&MutationPoint, &AdmittedAlternative), SelectionRefusal> {
        if selection.surface() != self.identity() {
            return Err(SelectionRefusal::SelectionFromAnotherSurface {
                expected: self.identity(),
                found: selection.surface(),
            });
        }
        let Some(point) = self.point(selection.point()) else {
            return Err(SelectionRefusal::NoSuchPoint(selection.point()));
        };
        let Some(alternative) = point
            .admitted_alternatives()
            .iter()
            .find(|alternative| alternative.identity() == selection.alternative())
        else {
            return Err(SelectionRefusal::NoSuchAlternative {
                point: selection.point(),
                alternative: selection.alternative(),
            });
        };
        Ok((point, alternative))
    }

    /// Select one point into one admitted mutation meaning.
    ///
    /// Runtime is selection among admitted alternatives, never interpretation of arbitrary source, and alternative identity is independent of roster order.
    ///
    /// # Errors
    ///
    /// Refuses a point the table does not carry, then an alternative that point does not admit.
    pub fn select(
        &self,
        point: MutationPointRef,
        alternative: AlternativeId,
    ) -> Result<ActiveSelection, SelectionRefusal> {
        let Some(found) = self.point(point) else {
            return Err(SelectionRefusal::NoSuchPoint(point));
        };
        if !found
            .admitted_alternatives()
            .iter()
            .any(|admitted| admitted.identity() == alternative)
        {
            return Err(SelectionRefusal::NoSuchAlternative { point, alternative });
        }
        Ok(ActiveSelection {
            surface: self.identity,
            point,
            alternative,
        })
    }

    /// Every active selection this surface admits, in canonical point and alternative order.
    #[must_use]
    pub fn selections(&self) -> Vec<ActiveSelection> {
        self.points
            .iter()
            .flat_map(|point| {
                point
                    .admitted_alternatives()
                    .iter()
                    .map(|alternative| ActiveSelection {
                        surface: self.identity,
                        point: point.identity(),
                        alternative: alternative.identity(),
                    })
            })
            .collect()
    }
}

impl ActiveSelection {
    /// The evaluation surface that issued this selection.
    #[must_use]
    pub const fn surface(self) -> EvaluationSurfaceId {
        self.surface
    }

    /// The point that is damaged.
    #[must_use]
    pub const fn point(self) -> MutationPointRef {
        self.point
    }

    /// Which of its admitted alternatives is active.
    #[must_use]
    pub const fn alternative(self) -> AlternativeId {
        self.alternative
    }
}

impl<'surface> ResolvedMutation<'surface> {
    /// Bind one surface-issued selection to the exact point and alternative it resolved to.
    pub(in crate::muterprater) const fn resolved(
        selection: ActiveSelection,
        point: &'surface MutationPoint,
        alternative: &'surface AdmittedAlternative,
    ) -> Self {
        Self {
            selection,
            point,
            alternative,
        }
    }

    /// The exact surface-issued selection that was resolved.
    #[must_use]
    pub const fn selection(self) -> ActiveSelection {
        self.selection
    }

    /// The admitted point selected for this call.
    #[must_use]
    pub const fn point(self) -> &'surface MutationPoint {
        self.point
    }

    /// The admitted alternative selected for this call.
    #[must_use]
    pub const fn alternative(self) -> &'surface AdmittedAlternative {
        self.alternative
    }
}

impl<'surface> EvaluationDirective<'surface> {
    /// The directly representable no-mutation posture.
    #[must_use]
    pub const fn no_mutation() -> Self {
        Self { resolved: None }
    }

    /// One active directive, after its selection was resolved against the exact surface.
    pub(in crate::muterprater) const fn active(
        selection: ActiveSelection,
        point: &'surface MutationPoint,
        alternative: &'surface AdmittedAlternative,
    ) -> Self {
        Self {
            resolved: Some(ResolvedMutation::resolved(selection, point, alternative)),
        }
    }

    /// The exact resolved mutation, where this directive is active.
    #[must_use]
    pub const fn resolved(self) -> Option<ResolvedMutation<'surface>> {
        self.resolved
    }
}
