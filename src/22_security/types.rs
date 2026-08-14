//! Security machinery: the lease, revocation distribution, shred, secrets,
//! the mechanism-standing fact families, trust-boundary disclosure, and the
//! supply-chain law. The value algebra (grants, claims, the meet, `KeyScope`,
//! protected resolution, release contracts) lives at the authority home;
//! this band collects the lifecycle machinery band math forced upward.
//!
//! # The safe-Rust floor
//!
//! Safe Rust is repository policy, enforced by the workspace lint wall — not
//! a claim this home makes. What matters here: safe Rust narrows
//! memory-corruption risk WITHOUT proving semantic correctness, bounded work,
//! determinism, constant-time cryptography, honest receipts, supply-chain
//! integrity, or safety inside transitive dependencies.
//!
//! # Release surfaces
//!
//! Logs, traces, metrics, diagnostics, panics, `Debug`, `Display`, source
//! maps, receipts, and explanations are information-release surfaces. In
//! byte-role terms, protected bytes have no morphism to diagnostics — a log
//! call taking protected bytes does not typecheck. This is the law behind
//! the wall's deliberate exclusion of mandatory `Debug` derives. A refusal's
//! release posture inherits the typed-redacted-diagnostic default (the
//! ingress decision's shape), never a second answer per family.
//!
//! # Two supply-chain claims, neither substituting
//!
//! Source-to-artifact continuity and public semantic equivalence (a public
//! independent checker that can reject a wrong-but-compiling implementation
//! without executing unpublished code) are different claims; neither
//! substitutes for the other. Any source-generation process outside the
//! published boundary is OUTSIDE the published trust boundary and is never a
//! supported trust claim. Release mechanics — reproducible-build posture,
//! actor scoping, artifact eligibility — are repository policy.

use crate::authority::CapabilityGrantId;
use crate::identity::Commitment;
use crate::types::{EvidenceRef, EvidenceSelectedMagnitude, Limit, NonEmptyBounded};

// ---------------------------------------------------------------------------
// The lease — the band-forced seat from the authority home, collected.
// ---------------------------------------------------------------------------

/// Lease-scope domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LeaseScopeDomain;
/// Lease-generation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LeaseGenerationDomain;
/// Deadline-policy claim marker (the time home's policy, by reference).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LeaseDeadlineClaim;
/// Renewal-authority domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenewalAuthorityDomain;

/// The role-qualified renewal authority — carries EXACTLY the renewal job,
/// nothing broader. Renewal is a named authority-bearing morphism (a
/// rebase-style morphism over the consumed deadline policy) — never a date
/// edit or silent extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LeaseRenewalAuthority(pub Commitment<RenewalAuthorityDomain>);

/// The capability lease — THE PAVED ROAD: binds one admitted grant to its
/// temporal validity law. Grant validity is answered through the canonical
/// three-valued truth — never a second three-valued enum, never a revoked
/// flag — and `Pending` narrows fail-closed for safety-relevant authority.
/// (Seated here by band math: the lease consumes the time home's deadline
/// policy, three bands above the grant algebra.)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityLease {
    /// The admitted grant.
    pub grant: CapabilityGrantId,
    /// The lease's scope.
    pub scope: Commitment<LeaseScopeDomain>,
    /// The generation.
    pub generation: Commitment<LeaseGenerationDomain>,
    /// The consumed deadline policy.
    pub deadline_policy: EvidenceRef<LeaseDeadlineClaim>,
    /// The renewal authority.
    pub renewal: LeaseRenewalAuthority,
}

/// The paved revocation default per authority class — each an asymmetric
/// paved-default-plus-explicit-override; online-check everywhere is
/// expressible, never assumed; stale-window profiles are selected, never
/// implicit.
pub const REVOCATION_DEFAULTS: [&str; 4] = [
    "protected-data-grants-bounded-lease",
    "long-lived-effect-capabilities-online-check-or-short-lease",
    "local-session-authority-generation-bump",
    "registered-participant-flows-acknowledgement-with-denominator",
];

// ---------------------------------------------------------------------------
// Revocation is a distributed-time problem.
// ---------------------------------------------------------------------------

/// The observation fact — whether this participant has OBSERVED the
/// revocation within its bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RevocationObservation {
    /// Observed.
    Observed,
    /// Not yet observed, within the declared bound.
    UnobservedWithinBound,
}

/// The acknowledgement fact — a DISTINCT participant act: observing a
/// revocation is not acknowledging it. (AUTHORED variant names; the axis
/// separation is the law.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RevocationAcknowledgement {
    /// The participant performed its acknowledgement act.
    Acknowledged,
    /// Not yet performed.
    NotYetAcknowledged,
}

/// Revocation-participant domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RevocationParticipantDomain;
/// Revocation-evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RevocationEvidenceClaim;

/// Per-participant revocation evidence keeping THREE facts separate:
/// observation, acknowledgement, and evidence freshness (the root axis on
/// the evidence itself — stale revocation evidence is stale EVIDENCE, never
/// a third observation state). A participant's non-observation never proves
/// universal cessation; an old grant honored during the lag window is either
/// a declared bounded-risk profile or a profile violation — never silently
/// acceptable. "Revocation complete" binds its explicit participant
/// denominator, never bare done; the stale window is a declared, bounded,
/// priced uncertainty interval. No-bearer-tokens is already law, so the
/// escaped bearer claim structurally does not exist.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RevocationEvidence {
    /// The participant.
    pub participant: Commitment<RevocationParticipantDomain>,
    /// The observation fact.
    pub observation: RevocationObservation,
    /// The acknowledgement fact.
    pub acknowledgement: RevocationAcknowledgement,
    /// The evidence itself (freshness rides the root axis on this).
    pub evidence: EvidenceRef<RevocationEvidenceClaim>,
}

// ---------------------------------------------------------------------------
// The firewall's act table and the label algebra.
// ---------------------------------------------------------------------------

/// What each act establishes AND NOTHING MORE — none alone grants semantic
/// identity, capability, freshness, completeness, proof, durable acceptance,
/// or effect authority. Outer routing fields stay untrusted hints until
/// reconciled with the authenticated inner claim; a conflict refuses.
pub const FIREWALL_ACT_TABLE: [&str; 5] = [
    "parsing-establishes-parse-success",
    "authentication-establishes-its-exact-claim",
    "signature-establishes-signer-authenticity-for-its-preimage",
    "rendering-establishes-a-presentation",
    "transport-establishes-carriage",
];

/// The label algebra's three arrows — closed. Labels ride schema fields and
/// byte roles: no parallel taxonomy. Every operation declares its label
/// transform beside its result axes (one more generated column — the macro
/// role absorbs the ceremony). No single severity tier decides
/// authorization, filtering, logging, retention, export, and egress at once;
/// a consumer with no fact for a field refuses to release it rather than
/// defaulting to open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LabelArrow {
    /// Joins combine restrictions — the output label is the join of the
    /// inputs; the most restrictive wins.
    Join,
    /// Aggregates DECLARE their leakage per operator family — aggregation
    /// never launders.
    AggregateWithDeclaredLeakage,
    /// The ONLY label-loosening arrow: a named, authority-bearing, receipted
    /// morphism owned here.
    Declassification,
}

// ---------------------------------------------------------------------------
// Secrets.
// ---------------------------------------------------------------------------

/// Secret-backend contract domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SecretBackendDomain;

/// The secret-authority backend contract: identity and profile, generation
/// bindings, key identity/scope/generation, bounded creation and import,
/// authorized resolution to a live handle, rotation/rewrap/revocation/shred,
/// metadata enumeration WITHOUT raw-key leakage, durability, stale/foreign-
/// restore refusal, and evidence. No ambient secret authority exists:
/// no process-wide environment variables, inherited descriptors, global
/// registries, static mutable slots, command-line arguments, or implicit
/// paths — an adapter importing from an environment or keychain stays an
/// explicit typed boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SecretAuthorityBackend(pub Commitment<SecretBackendDomain>);

/// Live secret-use authority: capability-bound, operation-scoped,
/// lifetime-bounded, nonportable. Deliberately NO `Clone`, NO `Copy`, NO
/// `Debug`, NO `Display`, no serialization — `Debug` and `Display` are named
/// release surfaces and this handle refuses the morphism by not having it.
/// Structurally `!Send`/`!Sync` via the raw-pointer phantom; a lawful thread
/// transfer, if an owner ever needs one, is a named consuming operation
/// minting a fresh handle — never ambient. Raw secret material is exposed
/// only as a borrowed view inside an admitted closure, never an owned raw
/// return; zeroization is best-effort hygiene, never a durable-destruction
/// claim. Key derivation REALIZES attenuation — it does not prove it: the
/// child scope's narrowing is established by the authority algebra and the
/// derivation policy, which the derivation then realizes.
pub struct SecretUseHandle {
    _execution_context_local: core::marker::PhantomData<*const ()>,
}

/// The four consumer-selected secret capability configurations — portability
/// and recovery trade directly against nonextractability and honest-shred
/// claims, so the machine selects no single posture on the consumer's
/// behalf. THE DEFAULT IS FAIL-CLOSED (nonextractable-only, no raw export);
/// every additional capability is an explicit, evidence-bearing opt-in,
/// individually qualified. Recovery never resurrects a shredded generation.
pub const SECRET_CAPABILITIES: [&str; 4] = [
    "raw-export",
    "backup-escrow-profile",
    "password-derived-protection",
    "recovery",
];

// ---------------------------------------------------------------------------
// Shred and anti-resurrection.
// ---------------------------------------------------------------------------

/// The shred-progress facts — distinct, never collapsing into each other or
/// into the resolution outcome: destruction requested, attempted, and
/// acknowledged are different facts, and physical ciphertext retirement is a
/// fourth. Key destruction, physical retirement, federated completion, and
/// regulatory compliance remain different claims; secure ciphertext deletion
/// is NOT claimed merely because key authority was shredded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShredProgress {
    /// Destruction requested.
    DestructionRequested,
    /// Destruction attempted.
    DestructionAttempted,
    /// Destruction acknowledged by the backend.
    DestructionAcknowledged,
    /// The physical ciphertext was retired.
    PhysicalCiphertextRetired,
}

/// Shred-evidence domain markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShredGenerationDomain;
/// Shred key-scope domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShredScopeDomain;
/// Shred backend domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShredBackendDomain;
/// Shred durability claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShredDurabilityClaim;
/// Protected-index invalidation claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexInvalidationClaim;
/// Resulting-resolution domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResultingResolutionDomain;

/// Limit family for shred participants. A denominator's participant set is as
/// wide as the generation it is about, so the magnitude is selected by the
/// owner's evidence rather than declared here — see
/// [`crate::types::EvidenceSelectedLimit`]. The only family in this crate on
/// that ladder whose seat is not a refusal body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShredParticipantLimit;
impl Limit for ShredParticipantLimit {
    type Authority = EvidenceSelectedMagnitude;
}
impl crate::types::EvidenceSelectedLimit for ShredParticipantLimit {}

/// Shred is acknowledged only after every required backend has durably
/// destroyed the relevant key authority and produced THIS evidence. Shred
/// changes key authority and resolution; it NEVER rewrites accepted history,
/// immutable event frames, or public derived bytes. Anti-resurrection: a
/// stale, foreign, restored, or pre-shred keyset cannot restore readability
/// to a later generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShredEvidence {
    /// The generations destroyed.
    pub generations: Commitment<ShredGenerationDomain>,
    /// The key scope.
    pub scope: Commitment<ShredScopeDomain>,
    /// The backends.
    pub backend: Commitment<ShredBackendDomain>,
    /// The participant rows — the completion denominator names every
    /// protected derivative and copy.
    pub participants: NonEmptyBounded<ShredDenominatorRow, ShredParticipantLimit>,
    /// The durability evidence.
    pub durability: EvidenceRef<ShredDurabilityClaim>,
    /// The protected-index invalidation evidence.
    pub index_invalidation: EvidenceRef<IndexInvalidationClaim>,
    /// The resulting resolution.
    pub resulting_resolution: Commitment<ResultingResolutionDomain>,
}

/// One row of the shred completion denominator — every protected derivative
/// and copy (indexes, embeddings, caches, logs, corpora, exported bundles,
/// projections, external processors), with the honest statuses VISIBLE.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShredDenominatorRow {
    /// The derivative or copy.
    pub subject: Commitment<ShredScopeDomain>,
    /// Its status.
    pub status: ShredRowStatus,
}

/// The honest per-row statuses — none hidden. (AUTHORED enum for the
/// prose's six-status list.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShredRowStatus {
    /// Destroyed.
    Destroyed,
    /// Missing.
    Missing,
    /// Unreachable.
    Unreachable,
    /// Legally retained.
    LegallyRetained,
    /// Unsupported.
    Unsupported,
    /// Failed or not run.
    FailedOrNotRun,
}

// ---------------------------------------------------------------------------
// Crypto roles, witnesses, mechanism standing, foreign execution.
// ---------------------------------------------------------------------------

/// The seven-role non-substitution chain — a checksum does not authenticate;
/// a MAC is not publicly verifiable; a signature does not prove freshness;
/// inclusion does not prove completeness; a witness proves only what its
/// profile establishes. Mechanisms are selected by role-specific evidence —
/// no incumbent wins by familiarity, and no native replacement of a
/// cryptographic primitive is admitted merely to cut dependency count.
pub const CRYPTO_ROLES: [&str; 7] = [
    "checksum-corruption-triage",
    "content-digest-exact-byte-binding",
    "semantic-commitment-domain-separated-preimage",
    "mac-shared-key-one-trust-domain",
    "signature-signer-authenticity-one-key-policy",
    "inclusion-proof-membership-in-one-committed-structure",
    "external-witness-retained-outside-the-challenged-authority",
];

/// The one witness role the bytes home's neutral sum does not carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WitnessRole {
    /// A claim retained OUTSIDE the challenged authority — the anti-rollback
    /// facet requires one (no local signature or clock summary substitutes).
    ExternalWitness,
}

/// Mechanism admission — append-only fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanismAdmissionFact {
    /// Admitted.
    Admitted,
    /// Refused.
    Refused,
}

/// Mechanism qualification — append-only, HISTORICAL standing only:
/// evidence freshness is the separate root axis on the qualification
/// evidence itself, and a mechanism stays historically qualified while its
/// evidence goes stale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanismQualificationFact {
    /// A profile qualified.
    QualifiedProfile,
    /// Qualification failed.
    Failed,
}

/// Mechanism release support — append-only fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanismSupportFact {
    /// A supported release row.
    SupportedReleaseRow,
    /// Unsupported.
    Unsupported,
}

/// Mechanism retirement — append-only fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MechanismRetirementFact {
    /// Retired.
    Retired,
}

/// Standing-policy domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StandingPolicyDomain;

/// The read-only standing projection — composes the append-only fact
/// families under a named policy and OWNS NO AUTHORITY. The lifecycle runs
/// proposed → admitted → qualified profile → supported release row →
/// retired, and no state implies the next: familiarity does not admit,
/// admission does not qualify, qualification does not promise support, and
/// removal from one role does not prove removal from every graph (the old
/// first-state word is renamed: a PROPOSED mechanism carries no standing). A
/// mechanism upgrade is itself a new proposal whenever it changes a fact
/// relevant to a claim. A historical fact is never erased by a later one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MechanismStandingView(pub Commitment<StandingPolicyDomain>);

/// The two foreign-execution families — a neutral classification for
/// inspection only; the families share no operational type, and a release
/// supports each as its own qualified row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForeignExecution {
    /// The same executor semantics in another process for fault isolation —
    /// physical mechanics and evidence change; program meaning does not.
    IsolatedPakVmWorker,
    /// A bounded artifact-in/artifact-out effect running foreign code
    /// outside the executor, whose output re-enters as a foreign claim and
    /// never becomes a hidden instruction or membrane operation.
    ExternalToolEffect,
}

/// The trust boundary is CLAIM-LOCAL, not a permanent handwritten list, and
/// does not shrink because code is branded first-party. For one exact
/// profile and release claim it includes every component whose failure
/// could invalidate that claim; every supported profile DISCLOSES its
/// unsafe-containing mechanisms with their complete closure — the existence
/// of unsafe is not automatic rejection, and popularity is not automatic
/// admission.
pub const TRUST_BOUNDARY_MEMBERS: [&str; 8] = [
    "first-party-and-generated-source",
    "compiler-linker-stdlib-target-libraries-build-config",
    "proc-macros-build-scripts-generators-packaging-tools",
    "direct-and-transitive-runtime-dependencies",
    "selected-crypto-entropy-compression-storage-carrier-platform-mechanisms",
    "host-adapters-and-external-services",
    "keys-witnesses-configuration-deployment-assumptions",
    "independent-verifier-assumptions-where-relied-upon",
];
