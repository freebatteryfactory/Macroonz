//! The authority value algebra: claims, grants, attenuation, the meet law's
//! carriers, `KeyScope`, protected resolution, release-contract shapes, and the
//! postcondition matrix. Lifecycle machinery (leases, revocation distribution,
//! secrets, mechanism standing, firewall enforcement) lives at the security
//! home, which imports this one.
//!
//! # The non-collapsing chain
//!
//! possession of bytes ≠ decoding ≠ structural validity ≠ semantic validity ≠
//! identity claim ≠ authentication ≠ proof of possession ≠ capability claim ≠
//! capability grant ≠ operation admission ≠ durable publication ≠ physical
//! attempt ≠ physical observation ≠ external completion ≠ receipt ≠ freshness ≠
//! rollback resistance. No ambient authority: a path, slot, name, connection,
//! process identity, browser origin, session, signature, token string, image
//! digest, or successful transport grants no operation by itself.
//!
//! # The meet
//!
//! Authority values form a partial order with a canonical normal form, shared
//! by capability and `KeyScope` composition. Composition is the meet: greatest
//! lower bound where defined, typed refusal where not — never generic set
//! intersection. Noncommuting purposes have no meet; stale delegation against a
//! newer generation refuses loudly; a disjoint-region meet is a typed,
//! explainable empty distinct from "no grant"; every algebra refusal names its
//! [`ConstraintSourcePair`]. Widening is a new grant — a named, receipted
//! morphism, never an algebra outcome. The boundary supervisor's effective-grant
//! evaluation IS this meet: one evaluator, two homes.
//!
//! # Typestate discipline
//!
//! claim → grant and live → shredded are fresh authority/evidence objects,
//! never same-object typestate; only untrusted-bytes → validated-view and
//! open → sealed are genuine typestate strengthenings.

use crate::identity::{ApplicationScope, CreationLaw, IdentityClass, IdentityRole, Occurrence};
use crate::logic::Decision;
use crate::refusal::{CompletionPosture, FamilyShape, RefusalFamily};
use crate::types::{Bounded, ConstLimit, EvidenceRef, Limit, NonEmptyBounded};
use crate::value::BoundedText;
use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// Foreign surfaces and threat postures.
// ---------------------------------------------------------------------------

/// The four foreign-surface roles — distinct, composing, never new authority
/// sources. Constructing one grants nothing: parsing establishes parse success,
/// authentication its exact claim, a signature signer-authenticity for its
/// declared preimage, rendering a presentation, transport carriage — and none
/// alone grants semantic identity, capability, freshness, completeness, proof,
/// durable acceptance, or effect authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForeignSurface {
    /// Artifact extraction — never makes contents executable or authoritative.
    ArtifactInterchange,
    /// Parser/compiler frontend — a familiar syntax tree confers no trust.
    ProgramAuthoring,
    /// Presentation — cannot strengthen the underlying claim.
    Rendering,
    /// External request, model suggestion, webhook, tool result — a typed
    /// effect candidate under the ordinary admission checks.
    EffectIngress,
}

/// One trust posture from the closed profile language: per named claim, each
/// participant is exactly one of these.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrustPosture {
    /// Trusted for this claim.
    Trusted,
    /// Honest but possibly faulty.
    HonestButFaulty,
    /// Potentially malicious.
    PotentiallyMalicious,
    /// Possibly unavailable.
    Unavailable,
    /// Outside this profile's scope.
    OutOfProfile,
}

/// Limit family for threat-profile rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreatProfileLimit;
impl Limit for ThreatProfileLimit {}

/// Limit family for threat-subject designations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreatSubjectLimit;
impl Limit for ThreatSubjectLimit {}

/// One row of a threat profile: a named subject bound to its posture (authored
/// v1 shape; the profile axes roster is the old book's long list, carried in
/// this home's README).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThreatProfileRow {
    /// The named subject (caller, host, backend, carrier, dependency…).
    pub subject: BoundedText<ThreatSubjectLimit>,
    /// The subject's posture for the named claim.
    pub posture: TrustPosture,
}

/// A closed typed threat profile: posture rows per named claim. An application
/// selects a complete policy set; it never invents a free-form security
/// adjective.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThreatProfile {
    /// The posture rows.
    pub rows: Bounded<ThreatProfileRow, ThreatProfileLimit>,
}

// ---------------------------------------------------------------------------
// The authentication kinds — opaque, method-bound, never collapsing.
// ---------------------------------------------------------------------------

/// A marker for one authentication method. Methods are declared by the security
/// home's machinery; the kinds here stay method-generic.
pub trait AuthMethod {}

/// Limit family for credential bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CredentialLimit;
impl Limit for CredentialLimit {}

/// An opaque credential supporting one authentication method. Not identity, not
/// possession, not a grant.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Credential<M: AuthMethod> {
    bytes: Bounded<u8, CredentialLimit>,
    _method: PhantomData<M>,
}

impl<M: AuthMethod> Credential<M> {
    /// Byte length of the opaque credential.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the credential is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

/// An established authentication: exactly that method's identity claim, nothing
/// more. A valid signature grants no capability.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Authentication<M: AuthMethod> {
    evidence: EvidenceRef<M>,
}

impl<M: AuthMethod> Authentication<M> {
    /// The evidence backing this method's exact claim.
    #[must_use]
    pub fn evidence(&self) -> &EvidenceRef<M> {
        &self.evidence
    }
}

/// An established proof of possession — separate from authentication.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProofOfPossession<M: AuthMethod> {
    evidence: EvidenceRef<M>,
}

impl<M: AuthMethod> ProofOfPossession<M> {
    /// The evidence backing the possession claim.
    #[must_use]
    pub fn evidence(&self) -> &EvidenceRef<M> {
        &self.evidence
    }
}

// ---------------------------------------------------------------------------
// Capability claims and their collection-shaped construction family.
// ---------------------------------------------------------------------------

/// Limit family for claim member text and bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClaimMemberLimit;
impl Limit for ClaimMemberLimit {}

/// Limit family for delegation chains (bounded depth by law).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DelegationLimit;
impl Limit for DelegationLimit {}

/// One link of a delegation chain: names its parent and carries its generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DelegationLink {
    /// The named parent.
    pub parent: BoundedText<ClaimMemberLimit>,
    /// The link's authority generation.
    pub generation: u64,
}

/// A capability claim — a **request** for authority; success means `requested`,
/// and nothing more. Claim members are untrusted claim content (bounded text
/// and bytes), which is exactly what a claim is: on the typed authored route
/// every required member is bound by type, so none of the nine absences is
/// representable; they are reachable only on the decoded route. The validity
/// member stays opaque bytes here — its interpretation belongs to the boundary
/// that rechecks it (the band graph forbids importing the time home).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityClaim {
    issuer: BoundedText<ClaimMemberLimit>,
    audience: BoundedText<ClaimMemberLimit>,
    subject: BoundedText<ClaimMemberLimit>,
    rights: Bounded<BoundedText<ClaimMemberLimit>, ClaimMemberLimit>,
    resources: Bounded<BoundedText<ClaimMemberLimit>, ClaimMemberLimit>,
    validity: Bounded<u8, ClaimMemberLimit>,
    generation: u64,
    possession_binding: Bounded<u8, ClaimMemberLimit>,
    purpose: BoundedText<ClaimMemberLimit>,
    delegation: Option<Bounded<DelegationLink, DelegationLimit>>,
}

impl CapabilityClaim {
    /// The claimed issuer.
    #[must_use]
    pub fn issuer(&self) -> &BoundedText<ClaimMemberLimit> {
        &self.issuer
    }

    /// The claimed audience.
    #[must_use]
    pub fn audience(&self) -> &BoundedText<ClaimMemberLimit> {
        &self.audience
    }

    /// The claimed subject.
    #[must_use]
    pub fn subject(&self) -> &BoundedText<ClaimMemberLimit> {
        &self.subject
    }

    /// The claimed rights.
    #[must_use]
    pub fn rights(&self) -> &Bounded<BoundedText<ClaimMemberLimit>, ClaimMemberLimit> {
        &self.rights
    }

    /// The claimed resources.
    #[must_use]
    pub fn resources(&self) -> &Bounded<BoundedText<ClaimMemberLimit>, ClaimMemberLimit> {
        &self.resources
    }

    /// The opaque validity member, rechecked by the receiving boundary.
    #[must_use]
    pub fn validity(&self) -> &Bounded<u8, ClaimMemberLimit> {
        &self.validity
    }

    /// The claimed authority generation.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// The declared possession binding.
    #[must_use]
    pub fn possession_binding(&self) -> &Bounded<u8, ClaimMemberLimit> {
        &self.possession_binding
    }

    /// The claimed purpose.
    #[must_use]
    pub fn purpose(&self) -> &BoundedText<ClaimMemberLimit> {
        &self.purpose
    }

    /// The delegation chain — lawfully absent on a direct claim.
    #[must_use]
    pub fn delegation(&self) -> Option<&Bounded<DelegationLink, DelegationLimit>> {
        self.delegation.as_ref()
    }
}

/// The compile-time bound of the claim-construction issue collection: the
/// roster's own cardinality — one issue per issue kind, ten at most.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClaimIssueLimit;
impl Limit for ClaimIssueLimit {}
impl ConstLimit for ClaimIssueLimit {
    const MAX: usize = 10;
}

/// The ten claim-construction issues — **flat by recorded decision**: a
/// parameterized `MemberMissing(Member)` shape would make the four killed
/// causes (revocation posture, context, operation, scope-as-member)
/// representable again; shape makes the wrong move unrepresentable first,
/// anti-drift second. The nine absences are reachable only on the decoded
/// route; `DelegationChainMalformed` is the one issue an authored construction
/// can produce.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapabilityClaimConstructionIssue {
    /// Issuer absent (decoded route only).
    IssuerMissing,
    /// Audience absent (decoded route only).
    AudienceMissing,
    /// Subject absent (decoded route only).
    SubjectMissing,
    /// Rights absent (decoded route only).
    RightsMissing,
    /// Resources absent (decoded route only).
    ResourcesMissing,
    /// Validity absent (decoded route only).
    ValidityMissing,
    /// Generation absent (decoded route only).
    GenerationMissing,
    /// A declared possession binding absent (decoded route only) — a
    /// bearer-shaped claim is not authorable, so this observes decoded material
    /// and exists for nothing else.
    PossessionMissing,
    /// Purpose absent (decoded route only).
    PurposeMissing,
    /// A relation defect among the chain's links: each link names its parent,
    /// no cycles, bounded depth, each link carries its generation. The one
    /// authored-route issue.
    DelegationChainMalformed,
}

/// The claim-construction family: the machine's first collection-shaped
/// refusal. Independent members, no ladder, family-level reason, no primary
/// issue ever elected, posture carried as an instance value. For a decoded
/// claim the detailed issue list is route disclosure — under a profile
/// requiring grouped handling the boundary withholds it, never rewrites it.
/// It reads only the facts the claim itself carries; success mints nothing on
/// the authority path; no authority-algebra refusal is expressible here; a
/// broad claim is not malformed (over-grant is an admission finding).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityClaimConstruction {
    /// The established issues — at least one, at most the roster.
    pub issues: NonEmptyBounded<CapabilityClaimConstructionIssue, ClaimIssueLimit>,
    /// Whether every applicable check ran.
    pub posture: CompletionPosture,
}

impl RefusalFamily for CapabilityClaimConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

// ---------------------------------------------------------------------------
// Grants, handles, attenuation, the meet's carriers.
// ---------------------------------------------------------------------------

/// The identity role marker for capability grants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityGrantRole;

/// The identity of one grant, for receipts and evidence — it carries no
/// authority; the grant value carries the authority and the live handle its
/// use, never the id. Class D, fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityGrantId(Occurrence<CapabilityGrantRole>);

impl CapabilityGrantId {
    /// In-crate mint for laws. Test-gated until grant minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<CapabilityGrantRole>) -> Self {
        Self(occurrence)
    }
}

impl IdentityRole for CapabilityGrantId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

/// An admitted authority object, minted by the owning policy — a fresh object,
/// never a typestate of the claim that requested it (authored v1 content: the
/// scope; the full grant member roster rides the meet's normal form).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityGrant {
    scope: KeyScope,
}

impl CapabilityGrant {
    /// The granted scope.
    #[must_use]
    pub fn scope(&self) -> &KeyScope {
        &self.scope
    }
}

/// One process-local route to use one grant. Never a bearer token: the raw
/// pointer phantom makes the handle structurally `!Send` and `!Sync`, and no
/// serialization exists — live authority cannot be reconstructed from an
/// encoded object.
#[derive(Debug)]
pub struct GrantUseHandle {
    _process_local: PhantomData<*const ()>,
}

/// The six narrow-only attenuation axes. Widening is not expressible — no
/// operation exists; the reverse is a new grant, a named receipted morphism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttenuationAxis {
    /// Narrow the rights.
    Rights,
    /// Narrow the resources.
    Resources,
    /// Narrow the audience.
    Audience,
    /// Narrow the time validity.
    Time,
    /// Narrow the scope.
    Scope,
    /// Narrow the delegation depth.
    Delegation,
}

/// Limit family for attenuation axis sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttenuationLimit;
impl Limit for AttenuationLimit {}

/// One attenuation operand: which axes it narrows (authored v1 shape; each
/// axis's narrowing content rides the normal form).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attenuation {
    /// The axes narrowed.
    pub axes: Bounded<AttenuationAxis, AttenuationLimit>,
}

/// The receipt-grade carrier every algebra refusal must name: the pair of
/// constraint sources that produced it. The pair structure is the law; the
/// source type is the algebra's caller's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstraintSourcePair<Source> {
    left: Source,
    right: Source,
}

impl<Source> ConstraintSourcePair<Source> {
    /// In-crate mint for laws. Test-gated until the meet evaluator exists.
    #[cfg(test)]
    pub(crate) const fn named(left: Source, right: Source) -> Self {
        Self { left, right }
    }

    /// The first constraint source.
    #[must_use]
    pub fn left(&self) -> &Source {
        &self.left
    }

    /// The second constraint source.
    #[must_use]
    pub fn right(&self) -> &Source {
        &self.right
    }
}

// ---------------------------------------------------------------------------
// KeyScope — Class F's contract landed.
// ---------------------------------------------------------------------------

/// Limit family for key-scope components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyScopeLimit;
impl Limit for KeyScopeLimit {}

/// One application-declared scope component (tenant, subject, purpose, case,
/// record family, application-defined domain).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeComponent {
    /// The component designation.
    pub designation: BoundedText<KeyScopeLimit>,
}

/// The application-composed key scope — Class F, and the machine mints none.
/// Composition shares the one authority-algebra substrate with capability
/// composition. The canonical composition normal form (authored v1): components
/// in byte-lexicographic canonical order, one canonical normalization, never
/// free bytes — the normal form IS the byte form the secret authority
/// generation's scope binds.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyScope {
    /// The composed components in canonical order.
    pub components: Bounded<ScopeComponent, KeyScopeLimit>,
}

impl ApplicationScope for KeyScope {}

// ---------------------------------------------------------------------------
// Protected resolution, admission, release contracts, the matrix.
// ---------------------------------------------------------------------------

/// The eight protected-resolution outcomes — exactly these, never collapsed,
/// and no blank, zeroed, absent, null, default, generic-missing, or
/// decoder-error value substitutes for any of them. Resolution changes with
/// capability, key authority, generation, or availability without rewriting
/// event meaning or derived bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtectedResolution {
    /// The payload resolves.
    Resolved,
    /// No payload exists at this reference.
    NotPresent,
    /// Key authority was durably destroyed.
    Shredded,
    /// The caller lacks authority.
    Unauthorized,
    /// The key authority is missing.
    KeyAuthorityMissing,
    /// The payload binding is invalid.
    BindingInvalid,
    /// The payload bytes are corrupt.
    Corrupt,
    /// The payload is physically unavailable.
    PhysicallyUnavailable,
}

/// One operation admission: the composition of the domain owner's transition
/// judgment and security's authority judgment. The operation proceeds only when
/// both allow, and neither seat ever answers for the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperationAdmission {
    /// The domain owner's judgment of the business transition.
    pub domain: Decision,
    /// Security's judgment of grants and authority predicates.
    pub authority: Decision,
}

/// The exact source claim and subject of one release.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InformationReleaseSource {
    /// The source claim.
    pub claim: EvidenceRef<ReleaseClaim>,
}

/// The acting principal, sponsor, purpose, audience, and authorization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InformationReleaseAuthority {
    /// The authorizing grant's identity.
    pub grant: CapabilityGrantId,
}

/// Classification facts and permitted use.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InformationReleaseClassification {
    /// The classification designation.
    pub classification: BoundedText<ClaimMemberLimit>,
}

/// Fields and relationships exposed and withheld; transformations applied.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InformationReleaseProjection {
    /// The named projection.
    pub projection: BoundedText<ClaimMemberLimit>,
}

/// Cut/freshness/completeness posture and bounded work and output size.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InformationReleaseBounds {
    /// The declared output byte bound.
    pub output_bytes: u64,
}

/// Retention and deletion obligations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InformationReleaseRetention {
    /// The named retention obligation.
    pub obligation: BoundedText<ClaimMemberLimit>,
}

/// The evidence owed and the loss/reversibility/verifiability posture.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InformationReleaseEvidence {
    /// The owed evidence reference.
    pub owed: EvidenceRef<ReleaseClaim>,
}

/// The claim marker for release evidence references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReleaseClaim;

/// The release contract: one typed record of seven role-qualified subrecords —
/// never a flat scalar pile. Missing facts refuse release; no serializer,
/// formatter, adapter, route, or generated projection may infer them.
/// (Subrecord members are authored v1 cores; the full member rosters ride each
/// subrecord's docs from the primary.)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InformationReleaseContract {
    /// The source subrecord.
    pub source: InformationReleaseSource,
    /// The authority subrecord.
    pub authority: InformationReleaseAuthority,
    /// The classification subrecord.
    pub classification: InformationReleaseClassification,
    /// The projection subrecord.
    pub projection: InformationReleaseProjection,
    /// The bounds subrecord.
    pub bounds: InformationReleaseBounds,
    /// The retention subrecord.
    pub retention: InformationReleaseRetention,
    /// The evidence subrecord.
    pub evidence: InformationReleaseEvidence,
}

/// The postcondition honesty matrix — the fourteen non-substitutions, whose
/// only orderings are declared implication edges, never one ladder. Partial
/// evidence preserves successful and missing subclaims without rendering a
/// stronger combined success.
pub const POSTCONDITION_NON_SUBSTITUTIONS: [(&str, &str); 14] = [
    ("requested", "granted"),
    ("authenticated", "authorized"),
    ("authorized", "admitted"),
    ("admitted", "attempted"),
    ("attempted", "completed"),
    ("completed", "durable"),
    ("written", "namespace-published"),
    ("digest-matched", "authenticated"),
    ("signed", "fresh"),
    ("shred-requested", "shred-acknowledged"),
    ("key-unavailable", "shredded"),
    ("isolated", "semantically-correct"),
    ("safe-rust", "supply-chain-proof"),
    ("qualified-mechanism", "universal-support-promise"),
];
