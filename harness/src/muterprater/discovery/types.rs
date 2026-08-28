//! Owner policy, producer discovery, executable surfaces, and resolved directives.

use crate::descriptor::{ClaimRef, MutationPointRef, NameRefusal, NamespacedName};
use crate::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::muterprater::OperatorFamilyRef;
#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// Owner policy and producer discovery.
// ---------------------------------------------------------------------------

/// The domain tag of an owner-authored mutation policy.
pub const MUTATION_POLICY_TAG: DomainTag =
    DomainTag::declared("mutation-policy", IdentityProfileVersion::declared(1));

/// The domain tag of one admitted alternative's stable identity.
pub const MUTATION_ALTERNATIVE_TAG: DomainTag =
    DomainTag::declared("mutation-alternative", IdentityProfileVersion::declared(1));

/// The domain tag of one complete evaluation surface.
pub const EVALUATION_SURFACE_TAG: DomainTag =
    DomainTag::declared("evaluation-surface", IdentityProfileVersion::declared(1));

/// The domain tag of one complete producer discovery reading.
pub const MUTATION_DISCOVERY_TAG: DomainTag =
    DomainTag::declared("mutation-discovery", IdentityProfileVersion::declared(1));

/// The owner-declared family that binds one production road, evaluation callable, policy, and evidence chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluationFamilyRef(NamespacedName);

/// The content identity of one owner-authored mutation policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutationPolicyId(ContentAddress);

/// One claim's permission to use a nonempty roster of operator families.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationPermission {
    owner_claim: ClaimRef,
    admitted_families: Vec<OperatorFamilyRef>,
}

/// Why one mutation permission was refused.
#[must_use = "a refusal is the reason a mutation permission was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionRefusal {
    /// The permission names no operator family, so it permits no executable damage.
    NoOperatorFamily,
    /// One operator family appears twice in the permission.
    DuplicateOperatorFamily(OperatorFamilyRef),
}

/// One evaluation family's owner-authored mutation policy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationPolicy {
    family: EvaluationFamilyRef,
    permissions: Vec<MutationPermission>,
    identity: MutationPolicyId,
}

/// Why one mutation policy was refused.
#[must_use = "a refusal is the reason a mutation policy was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyRefusal {
    /// Two permission rows name one owner claim.
    DuplicateClaim(ClaimRef),
}

/// The policy-issued membership carried by one admitted mutation point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PolicyMembership {
    policy: MutationPolicyId,
    owner_claim: ClaimRef,
}

/// Where a selected alternative fires, named rather than path-spelled.
///
/// A file move must rename nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActivationSite(NamespacedName);

/// One operator family and canonical mutation meaning a producer found at a site, before owner policy admits it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlternativeDeclaration {
    family: OperatorFamilyRef,
    operation: Vec<u8>,
}

/// Whether the producer's origin reading maps one discovered site to an owner claim.
///
/// The unmapped arm stays a first-class discovery fact and can acquire no policy membership or executable point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwnerClaimMapping {
    /// The origin reading mapped this site to the exact owner claim.
    Mapped(ClaimRef),
    /// The origin reading established no owner claim for this site.
    OwnerUnmapped,
}

/// One producer-discovered mutation site, before owner policy admits it.
///
/// A discovery states the site, its unchanged operation, its candidate alternative meanings, its activation site, and its owner mapping.
/// It grants no permission and is not executable: [`lower_discoveries`](crate::muterprater::discover::lower_discoveries) is the only road from a discovery roster to executable points.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscoveredMutationSite {
    identity: MutationPointRef,
    mapping: OwnerClaimMapping,
    original_operation: Vec<u8>,
    alternatives: Vec<AlternativeDeclaration>,
    activation_site: ActivationSite,
}

/// Why one producer-discovered mutation site was not structurally readable.
#[must_use = "a refusal is the reason one discovered mutation site was not read"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiscoveryRefusal {
    /// The site states no unchanged operation.
    EmptyOriginalOperation,
    /// The site carries no candidate alternative meaning.
    NoAlternative,
    /// One candidate alternative states no mutation meaning.
    EmptyAlternative {
        /// The alternative's position in producer order.
        at: usize,
    },
    /// One candidate alternative is byte-identical to the unchanged operation.
    AlternativeIsOriginal {
        /// The alternative's position in producer order.
        at: usize,
    },
    /// Two candidate alternatives state one operator family and mutation meaning.
    DuplicateAlternativeMeaning {
        /// The duplicate alternative's position in producer order.
        at: usize,
    },
}

/// Why one owner-mapped discovered site did not become executable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MappedUnpermittedCause {
    /// The policy carries no permission row for the mapped claim.
    Claim(ClaimRef),
    /// One candidate alternative uses a family outside the mapped claim's permission.
    Family {
        /// The alternative's position in producer order.
        at: usize,
        /// The family outside the mapped claim's permission.
        family: OperatorFamilyRef,
    },
}

/// Whether one discovered site was mapped and executable, owner-unmapped, or mapped but unpermitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiscoveryDisposition {
    /// Owner mapping and policy permission admitted this exact executable point.
    Mapped {
        /// The executable point issued from this discovery.
        point: MutationPointRef,
    },
    /// The producer found the site and its origin reading named no owner claim.
    OwnerUnmapped,
    /// The producer mapped the site, and owner policy did not admit it.
    MappedUnpermitted {
        /// The exact policy cause that withheld executable admission.
        cause: MappedUnpermittedCause,
    },
}

/// One complete producer discovery row and its owner-policy admission disposition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiscoveryEntry {
    site: DiscoveredMutationSite,
    disposition: DiscoveryDisposition,
}

/// The content identity of one complete producer discovery reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutationDiscoveryId(ContentAddress);

/// The complete producer discovery denominator, after owner-policy admission was read over it.
///
/// Every offered site appears exactly once in producer order with its disposition, so unmapped and unpermitted sites stay visible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationDiscoveryReading {
    family: EvaluationFamilyRef,
    policy: MutationPolicyId,
    identity: MutationDiscoveryId,
    entries: Vec<DiscoveryEntry>,
}

/// Why one complete discovery roster could not be lowered.
#[must_use = "a refusal is the reason no complete mutation discovery reading was lowered"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiscoveryLoweringRefusal {
    /// Two discovered sites state one point identity.
    DuplicateSite {
        /// The duplicate site's position in producer order.
        at: usize,
        /// The repeated point identity.
        point: MutationPointRef,
    },
}

/// One closed lowering: the complete discovery denominator beside the executable subset drawn from it.
pub struct MutationSurfaceLowering {
    discovery: MutationDiscoveryReading,
    surface: EvaluationSurface,
}

// ---------------------------------------------------------------------------
// The evaluation surface.
// ---------------------------------------------------------------------------

/// The stable identity of one point's admitted mutation meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlternativeId(ContentAddress);

/// One executable operator family and canonical mutation meaning admitted under a point's policy membership.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdmittedAlternative {
    identity: AlternativeId,
    family: OperatorFamilyRef,
    operation: Vec<u8>,
}

/// One owner-admitted executable mutation point on an evaluation surface.
///
/// Only [`lower_discoveries`](crate::muterprater::discover::lower_discoveries) mints this value, after retaining the complete discovery and checking owner mapping and policy permission.
/// A roster of admitted alternatives says which damages the point admits, and never that any of them was materialized, activated, or killed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutationPoint {
    identity: MutationPointRef,
    membership: PolicyMembership,
    original_operation: Vec<u8>,
    admitted_alternatives: Vec<AdmittedAlternative>,
    activation_site: ActivationSite,
}

/// The content identity of one complete evaluation surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvaluationSurfaceId(ContentAddress);

/// One evaluation surface's complete point table.
///
/// A hand author may supply discovery candidates and owner policy to the same closed lowering a producer targets, and only that lowering mints this surface.
/// Runtime is selection among these points, never interpretation of arbitrary source, which would mint a second meaning authority.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvaluationSurface {
    family: EvaluationFamilyRef,
    policy: MutationPolicyId,
    identity: EvaluationSurfaceId,
    points: Vec<MutationPoint>,
}

/// Whether a complete evaluation surface admits executable points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PointCatalogPosture {
    /// The surface is lawful and admits no active directive.
    NoAdmittedPoints,
    /// The surface admits at least one executable mutation point.
    Mutable,
}

/// One point selected into one of the damages it admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActiveSelection {
    surface: EvaluationSurfaceId,
    point: MutationPointRef,
    alternative: AlternativeId,
}

/// One surface-resolved mutation handed to an evaluation callable.
///
/// The value keeps the surface-issued selection and borrows the exact point and alternative it resolved to, so an evaluation callable never reconstructs an identity or consults a positional registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolvedMutation<'surface> {
    selection: ActiveSelection,
    point: &'surface MutationPoint,
    alternative: &'surface AdmittedAlternative,
}

/// What one evaluation call reads once the surface has resolved its authority.
///
/// The no-mutation posture is directly constructible through [`EvaluationDirective::no_mutation`]; an active directive is minted privately, and only from a selection its exact surface issued.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvaluationDirective<'surface> {
    resolved: Option<ResolvedMutation<'surface>>,
}

/// Why an evaluation callable could not execute one otherwise-lawful directive.
#[must_use = "a refusal is the reason an evaluation callable produced no observation"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvaluationCallRefusal {
    /// The evaluation callable contains no no-mutation branch.
    NoMutationNotImplemented,
    /// The surface admitted a selection the evaluation callable has no branch for.
    ActiveSelectionNotImplemented(ActiveSelection),
}

/// Why one active-mutant selection was refused.
#[must_use = "a refusal is the reason a mutant was not selected"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionRefusal {
    /// The selection was minted by another evaluation surface.
    SelectionFromAnotherSurface {
        /// The surface reading the selection.
        expected: EvaluationSurfaceId,
        /// The surface that issued the selection.
        found: EvaluationSurfaceId,
    },
    /// The surface states no point under this identity.
    NoSuchPoint(MutationPointRef),
    /// The point does not admit this mutation meaning.
    NoSuchAlternative {
        /// The point whose roster was read.
        point: MutationPointRef,
        /// The alternative identity absent from that roster.
        alternative: AlternativeId,
    },
}
