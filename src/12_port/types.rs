//! The port contract algebra: port families, the operation contract, outbound
//! external operations, the one-shot response-binding refusal, foreign claims,
//! and host-obligation shapes.
//!
//! # The four-way law
//!
//! Capability AUTHORIZES a port operation; the port or authority-backend
//! contract PROVES its physical postconditions; Bvisor MEDIATES AND OBSERVES
//! one Attempt; runtime INTERPRETS the evidence. Four owners; none substitutes
//! for another.
//!
//! # Least authority
//!
//! The port receives only the authority and data that request needs — no
//! ambient access to the application, runtime, grant table, host filesystem,
//! network, or unrelated Attempt state. Host libraries, OS APIs, browser
//! APIs, services, and devices are qualified mechanisms behind these
//! contracts, never the contract itself.
//!
//! # "Unsupported" is only ever an answer
//!
//! Request vocabularies express only real guarantee levels: "unsupported",
//! "none", and silent best-effort are ANSWER values, unrepresentable in any
//! request type — a request can never pre-weaken itself to pass, and the
//! mechanism that cannot provide the requested level refuses or answers with
//! its weaker established fact. A mechanism-specific success flag is evidence
//! input, never automatic proof of the stronger postcondition.

use crate::identity::{Commitment, CreationLaw, IdentityClass, IdentityRole, Occurrence};
use crate::refusal::{FamilyShape, RefusalFamily};
use crate::schema::SchemaSemanticCommitment;
use crate::types::{Bounded, ConstLimit, DeclaredMagnitude, EvidenceRef, Limit};

// ---------------------------------------------------------------------------
// Port family identity.
// ---------------------------------------------------------------------------

/// The identity role marker for port families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortFamilyRole;

/// One port family — Class D, fresh. A port family defines ONE semantic
/// boundary contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortFamilyId(Occurrence<PortFamilyRole>);

impl IdentityRole for PortFamilyId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

crate::scope_guard_version! {
    /// One version of a port family — Class C, scoped to its family: versions of
    /// different port families are incomparable by type.
    pub struct PortFamilyVersion over PortFamilyId, seated in mod port_family_version;
}

// ---------------------------------------------------------------------------
// The port role inventory.
// ---------------------------------------------------------------------------

/// The thirteen port roles — descriptions of one semantic boundary each,
/// never one port per line. Split where authority or recovery differs;
/// composed ONLY where one contract proves a shared boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortRole {
    /// Accepted-history source inspection.
    AcceptedHistoryInspection,
    /// Event / local-authority publication.
    EventPublication,
    /// Durable-checkpoint authority.
    DurableCheckpointAuthority,
    /// Genuine mutable authority, where admitted.
    MutableAuthority,
    /// Artifact retrieval / publication.
    ArtifactRetrievalPublication,
    /// Protected-payload extent access / publication.
    ProtectedPayloadExtent,
    /// Secret-authority operations.
    SecretAuthorityOperations,
    /// Wall-clock / chronology observation.
    WallClockChronologyObservation,
    /// Absolute monotonic progress.
    AbsoluteMonotonicProgress,
    /// Entropy / generated-identity material.
    EntropyGeneratedIdentity,
    /// Transport / external effects.
    TransportExternalEffects,
    /// Namespace publication.
    NamespacePublication,
    /// Device / qualified-external-tool effects.
    DeviceExternalToolEffects,
}

// ---------------------------------------------------------------------------
// The operation contract.
// ---------------------------------------------------------------------------

/// The operation's semantic effect posture — AUTHORED two-value carrier: the
/// restricted-query law derives effect refusal from THIS declared posture,
/// never from a hand-maintained blacklist of operation names, so the posture
/// must be data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortEffectPosture {
    /// Observes only; performs no effect.
    Observation,
    /// Performs a declared effect.
    Effectful,
}

/// The physical postconditions a mechanism may establish — seated HERE so the
/// admission boundary imports one vocabulary instead of minting a twin. A
/// requested postcondition the admitted profile cannot establish refuses
/// before a live Attempt, never as a post-hoc observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortPostcondition {
    /// A durability guarantee.
    Durability,
    /// An atomic-boundary guarantee.
    AtomicBoundary,
    /// A cancellation posture the mechanism honors.
    CancellationPosture,
}

/// Compile-time bound for declared postcondition sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortPostconditionLimit;
impl Limit for PortPostconditionLimit {
    type Authority = DeclaredMagnitude;
}
impl ConstLimit for PortPostconditionLimit {
    const MAX: usize = 3;
}

/// Domain markers for the operation contract's committed facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortOperationDomain;
/// Capability-scope domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityScopeDomain;
/// Resource-scope domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceScopeDomain;
/// Subject/principal/delegation/audience/purpose binding domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubjectBindingDomain;
/// Admitted-destination/peer-expectation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerDestinationDomain;
/// Information classification/release/response-validation contract domain
/// marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReleaseContractDomain;
/// Generation-binding domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GenerationBindingDomain;
/// Idempotency/retry/reconciliation/cancellation posture domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecoveryPostureDomain;
/// Information-label transform domain marker (the security flow law's
/// generated column).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InformationLabelDomain;
/// Deadline-allowance behavior domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadlineBehaviorDomain;
/// Evidence-family binding domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvidenceFamilyDomain;
/// Refusal-family binding domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefusalFamilyDomain;
/// Qualification/target/compatibility/support claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QualificationClaim;

/// The declared bound categories of one port operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortBoundsDeclaration {
    /// Portable-work bound.
    pub portable_work: u64,
    /// Byte bound.
    pub bytes: u64,
    /// Memory bound.
    pub memory: u64,
    /// Concurrency bound.
    pub concurrency: u64,
    /// Output bound.
    pub output: u64,
}

/// One admitted port operation — the seventeen-fact contract, every fact its
/// own typed member. A port contract implies NO universal request envelope,
/// response enum, host trait, or dynamic dispatcher — role-specific types
/// stay visible to the compiler; this record declares the contract, it does
/// not carry live requests.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PortOperation {
    /// The family and version.
    pub family: PortFamilyVersion,
    /// The operation's identity.
    pub operation: Commitment<PortOperationDomain>,
    /// The typed request value's schema.
    pub request_schema: SchemaSemanticCommitment,
    /// The typed response value's schema.
    pub response_schema: SchemaSemanticCommitment,
    /// The semantic effect posture.
    pub effect_posture: PortEffectPosture,
    /// The required capability scope.
    pub capability_scope: Commitment<CapabilityScopeDomain>,
    /// The required resource scope.
    pub resource_scope: Commitment<ResourceScopeDomain>,
    /// Subject / acting-principal / delegation / audience / purpose bindings,
    /// where required.
    pub subject_bindings: Option<Commitment<SubjectBindingDomain>>,
    /// Admitted destination and peer expectations, for external operations.
    pub destination: Option<Commitment<PeerDestinationDomain>>,
    /// Information classification / release / response-validation contracts.
    pub release_contracts: Commitment<ReleaseContractDomain>,
    /// Source/authority/partition/secret/application generations, where
    /// applicable.
    pub generations: Option<Commitment<GenerationBindingDomain>>,
    /// Idempotency / retry / reconciliation / cancellation posture.
    pub recovery_posture: Commitment<RecoveryPostureDomain>,
    /// The information-label transform: what classification the output
    /// carries, per the flow law — declared beside the result axes.
    pub information_label: Commitment<InformationLabelDomain>,
    /// The declared bounds.
    pub bounds: PortBoundsDeclaration,
    /// The deadline-allowance behavior: every lower mechanism enforces a
    /// derived allowance in its own clock domain — no monotonic value is
    /// transplanted across a carrier.
    pub deadline_allowance: Commitment<DeadlineBehaviorDomain>,
    /// The physical postconditions the mechanism may establish.
    pub postconditions: Bounded<PortPostcondition, PortPostconditionLimit>,
    /// The bound evidence families.
    pub evidence_families: Commitment<EvidenceFamilyDomain>,
    /// The bound refusal families.
    pub refusal_families: Commitment<RefusalFamilyDomain>,
    /// Qualification / target / compatibility / support requirements.
    pub qualification: EvidenceRef<QualificationClaim>,
}

// ---------------------------------------------------------------------------
// Outbound external operations.
// ---------------------------------------------------------------------------

/// Audience domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudienceDomain;
/// Redirect/retry/failover/destination-change law domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedirectLawDomain;
/// Response schema/size/identity/freshness/trust posture domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResponsePostureDomain;
/// Credential/grant-scope claim marker — evidence about scope, NEVER reusable
/// authority material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CredentialScopeClaim;

/// The declared outbound bound categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutboundBounds {
    /// Rate bound.
    pub rate: u64,
    /// Concurrency bound.
    pub concurrency: u64,
    /// Fan-out bound.
    pub fan_out: u64,
    /// Byte bound.
    pub bytes: u64,
    /// Cost bound.
    pub cost: u64,
    /// Effect-count bound.
    pub effects: u64,
    /// Deadline bound.
    pub deadline: u64,
}

/// One outbound transport, service, device, or external-tool operation — ONE
/// typed effect. DNS resolution, routing, connection success, and carrier
/// encryption are physical evidence only; a host status code or a familiar
/// response shape is claim evidence only. The admitted port operation
/// determines which destination and business effect are lawful; the
/// foreign-content firewall and the response schema determine what returned
/// material may mean.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OutboundExternalOperation {
    /// The exact destination.
    pub destination: Commitment<PeerDestinationDomain>,
    /// The admitted audience.
    pub audience: Commitment<AudienceDomain>,
    /// Represented subject / principal / delegate / purpose bindings.
    pub subject: Option<Commitment<SubjectBindingDomain>>,
    /// The request schema and permitted information release.
    pub request_schema: SchemaSemanticCommitment,
    /// The release contract.
    pub release: Commitment<ReleaseContractDomain>,
    /// Credential or grant scope, WITHOUT exposing reusable authority.
    pub credential_scope: EvidenceRef<CredentialScopeClaim>,
    /// The redirect / retry / failover / destination-change law.
    pub redirect_law: Commitment<RedirectLawDomain>,
    /// The response schema / size / identity / freshness / trust posture.
    pub response_posture: Commitment<ResponsePostureDomain>,
    /// The declared bounds.
    pub bounds: OutboundBounds,
    /// Idempotency / cancellation / uncertainty / reconciliation.
    pub recovery: Commitment<RecoveryPostureDomain>,
}

// ---------------------------------------------------------------------------
// Foreign claims — the firewall-admission seam.
// ---------------------------------------------------------------------------

/// The claim marker for firewall admissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FirewallAdmissionClaim;

/// A typed foreign claim: external results, port responses, tool artifacts,
/// and carrier material enter as claims — wrapping is free, unwrapping is
/// not. The ONLY route to the inner value is [`ForeignClaim::admitted`],
/// which consumes the claim against firewall-admission evidence and yields
/// the value still carrying that evidence. Origin, signature, transport
/// security, successful parsing, a familiar field name, or a driver success
/// code cannot upgrade foreign material — no other unwrap exists.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForeignClaim<T> {
    claim: T,
}

impl<T> ForeignClaim<T> {
    /// Wrap foreign material as the claim it is.
    #[must_use]
    pub const fn foreign(claim: T) -> Self {
        Self { claim }
    }

    /// The one lawful unwrap: consume this claim against firewall-admission
    /// evidence. The admitted value carries its admission.
    #[must_use]
    pub fn admitted(self, admission: EvidenceRef<FirewallAdmissionClaim>) -> AdmittedForeign<T> {
        AdmittedForeign {
            value: self.claim,
            admission,
        }
    }
}

/// A foreign value that crossed the firewall — it carries the admission
/// evidence that let it in.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdmittedForeign<T> {
    /// The admitted value.
    pub value: T,
    /// The admission evidence.
    pub admission: EvidenceRef<FirewallAdmissionClaim>,
}

// ---------------------------------------------------------------------------
// The one-shot response-binding refusal.
// ---------------------------------------------------------------------------

/// The response-binding refusal — a response must match the exact outstanding
/// request AND live Attempt, and resumes it at most once. Seated here: the
/// port contract owns its boundary law; the physical membrane enforces it
/// (the same authored-by/applied-by split as the firewall — its enforcement
/// checklist at the membrane factors these same causes and is never a second
/// family). The selection order is AUTHORED (causes coexist and the source
/// declares no order): a dead Attempt has no outstanding request, so no
/// further question exists; a spent one-shot outranks every content check;
/// request identity before contract shape; contract before authority;
/// declared bounds before temporal facts; `Late` ranks last because lateness
/// only means anything against an otherwise-matching response.
#[must_use = "a binding refusal carries the lawful reason the response was not bound"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResponseBinding {
    /// The Attempt is dead or foreign.
    DeadAttempt,
    /// The one-shot continuation was already spent.
    SecondResume,
    /// Wrong request identity.
    WrongRequest,
    /// Duplicate request identity.
    Duplicate,
    /// Wrong port family.
    WrongFamily,
    /// Wrong response type.
    WrongType,
    /// Wrong capability or grant generation.
    WrongCapability,
    /// Wrong source.
    WrongSource,
    /// Wrong generation.
    WrongGeneration,
    /// The response exceeds remaining bounds.
    OverBound,
    /// The applicable deadline expired.
    Expired,
    /// The response arrived after terminal disposition.
    Late,
}

impl RefusalFamily for ResponseBinding {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "DeadAttempt",
        "SecondResume",
        "WrongRequest",
        "Duplicate",
        "WrongFamily",
        "WrongType",
        "WrongCapability",
        "WrongSource",
        "WrongGeneration",
        "OverBound",
        "Expired",
        "Late",
    ];
}

// ---------------------------------------------------------------------------
// Deadline expiry, secret-authority verbs, and the declared rosters.
// ---------------------------------------------------------------------------

/// Expiry before admission versus after durable admission retain DIFFERENT
/// commit, cancellation, retry, and reconciliation meanings — two typed
/// outcomes, never one generic timeout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeadlineExpiryPosture {
    /// Expired before admission — nothing durable exists.
    BeforeAdmission,
    /// Expired after durable admission — expiry is never proof of noncommit.
    AfterDurableAdmission,
}

/// The nine bounded secret-authority port verbs — the port never exposes
/// reusable raw secrets to any image, artifact, log, receipt, explanation, or
/// unrelated port; unwrap yields an opaque use handle, never key material.
/// Protected-payload resolution reuses the authority home's canonical
/// eight-outcome resolution — never a local copy, never collapsed into empty
/// bytes, null, false, or one generic failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SecretAuthorityVerb {
    /// Authorize.
    Authorize,
    /// Select.
    Select,
    /// Wrap.
    Wrap,
    /// Unwrap into an opaque use handle.
    UnwrapIntoOpaqueUseHandle,
    /// Rotate.
    Rotate,
    /// Rewrap.
    Rewrap,
    /// Revoke.
    Revoke,
    /// Shred.
    Shred,
    /// Evidence.
    Evidence,
}

/// The eleven role-specific projections every port result carries. A
/// transport or physical success never proves operation success; outcome
/// uncertainty stays uncertainty.
pub const RESULT_PROJECTION_AXES: [&str; 11] = [
    "semantic-value",
    "admission",
    "exact-cut",
    "completeness",
    "freshness",
    "explanation",
    "work-and-bounds",
    "publication",
    "attempt-observation",
    "outcome-uncertainty",
    "checkpoint-and-reconciliation-posture",
];

/// The five statements every typed port refusal makes — self-describing,
/// never one generic error.
pub const SELF_DESCRIBING_REFUSAL_STATEMENTS: [&str; 5] = [
    "what-was-established",
    "what-was-not-established",
    "whether-durable-admission-could-have-occurred",
    "which-recovery-operation-is-lawful",
    "which-evidence-supports-the-conclusion",
];

/// The sixteen host-obligation axes the machine fixes per configuration
/// value — mechanisms qualify against them; a Cargo feature, target
/// compilation, an available dependency, or a passing unit test cannot by
/// itself prove support.
pub const HOST_OBLIGATION_AXES: [&str; 16] = [
    "required-apis",
    "capabilities",
    "bounds",
    "atomicity",
    "ordering",
    "visibility",
    "durability",
    "quota",
    "crash-reload",
    "ownership",
    "locking",
    "liveness",
    "cancellation",
    "evidence",
    "compatibility",
    "refusal",
];
