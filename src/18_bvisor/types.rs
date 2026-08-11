//! The boundary supervisor: physical admission, the Attempt lifecycle, the
//! reservation contract, physical observations, witnesses, port-request
//! custody, and containment.
//!
//! # The inversion rule
//!
//! An enforcer may always deny or narrow more than asked — fail-closed is
//! lawful in every mode, including degraded ones — but may NEVER report less
//! danger than occurred. Under-reporting an observed violation is illegal in
//! every mode, and the qualification ground truth exists to catch exactly
//! that lie.
//!
//! # What the supervisor is NOT
//!
//! Not an operating system, a hypervisor abstraction, an application
//! compositor, the logical runtime, the owner of Turn identity or checkpoint
//! meaning, the owner of effect causality or semantic retry legality, the
//! owner of compensation or reconciliation conclusions, or proof that a
//! backend operation is atomic or durable. It enforces the authority algebra;
//! it never authors it — and it cannot mint rights from an observed path,
//! integer slot, display name, process identity, connection, bearer string,
//! signature, caller identity, matching image digest, or asserted claim.
//!
//! # Containment is semantic first
//!
//! Primary containment is semantic and VM-native: the guest is bounded by the
//! machine's own capability model, typed crossings, and reserved budgets —
//! the only way out is a typed request this boundary must admit, and that
//! holds with no operating system beneath it: in-process, in a browser
//! worker, and under the `no_std`-plus-`alloc` posture, the membrane still
//! contains. Physical isolation is an OPTIONAL second layer of
//! defense-in-depth, never the source of the guarantee.

use crate::authority::ConstraintSourcePair;
use crate::bounds::DimensionId;
use crate::identity::{Commitment, CreationLaw, IdentityClass, IdentityRole, Occurrence};
use crate::port::{PortFamilyVersion, PortPostcondition};
use crate::refusal::{CompletionPosture, FamilyShape, RefusalFamily};
use crate::semantic::BoundDimensionRow;
use crate::types::{Bounded, EvidenceRef, Limit, NonEmptyBounded};
use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// Attempt identity — band-forced seat: the minting site declares.
// ---------------------------------------------------------------------------

/// The identity role marker for Attempts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttemptRole;

/// One physical effort — Class D, fresh per attempt, NEVER reused. Seated
/// here by band math and by the minting law: admission is the ONLY minting
/// site (the runtime home, one band up, imports it for lineage). A replay
/// may preserve the logical operation, Turn, and effect intent while
/// creating a new Attempt identity; no physical identity is reused because
/// the program, request, connection, worker, process, or external
/// idempotency key happens to match. A response, observation, cancellation,
/// reservation, completion, or receipt from one Attempt can never satisfy
/// another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttemptId(Occurrence<AttemptRole>);

impl IdentityRole for AttemptId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl AttemptId {
    /// In-crate mint for laws. Test-gated until the admission seam exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<AttemptRole>) -> Self {
        Self(occurrence)
    }
}

// ---------------------------------------------------------------------------
// Admission: inputs, order, the fourteen-issue family.
// ---------------------------------------------------------------------------

/// The eleven physical admission inputs, in the stated order. A directly
/// invoked image admits WITHOUT an application image or instance;
/// composition bindings are present only when the invocation actually
/// belongs to persistent composition.
pub const ADMISSION_INPUTS: [&str; 11] = [
    "invocation-and-entrypoint-identity",
    "turn-and-logical-operation-relationship",
    "application-instance-and-generation-where-composition-exists",
    "source-store-partition-authority-generations-where-applicable",
    "required-operations-against-live-grants",
    "requested-bounds-against-all-limits",
    "deadline-allowance-and-cancellation-posture",
    "port-kernel-host-profile-availability",
    "required-postconditions-establishable",
    "physical-reservation",
    "fresh-attempt-creation",
];

/// The cheapest safe dependency order — ten stations. Coherence is FIRST and
/// consults no host fact; reservation happens only after every preceding
/// condition passes. Ordering is itself security-sensitive: a cheap refusal
/// may precede an expensive check only when doing so creates no undeclared
/// identity, existence, capability, secret, freshness, or workload oracle.
pub const ADMISSION_DEPENDENCY_ORDER: [&str; 10] = [
    "validated-executable-and-invocation-role",
    "exact-identity-lineage-generation-bindings",
    "principal-delegation-authenticity-policy-where-required",
    "capability-requirements-against-live-grants",
    "interface-schema-port-kernel-compatibility",
    "required-evidence-and-freshness",
    "logical-authorization-supplied-by-the-runtime",
    "complete-bound-intersection",
    "physical-capacity-reservation",
    "fresh-attempt-creation",
];

/// The consumed executable-verdict subjects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsumedVerdictSubject {
    /// The verdict is absent.
    VerdictAbsent,
    /// The invocation role mismatches.
    InvocationRoleMismatch,
    /// Composition bindings do not belong to this invocation.
    CompositionBindingsNotBelonging,
}

/// The identity/lineage binding subjects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BindingSubject {
    /// An identity binding.
    Identity,
    /// A lineage binding.
    Lineage,
}

/// The generation axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerationAxis {
    /// The source generation.
    Source,
    /// The store generation.
    Store,
    /// The partition epoch.
    Partition,
    /// The authority generation.
    Authority,
    /// The application generation.
    Application,
}

/// A STALE generation is not a WRONG one — two postures, never fused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerationPosture {
    /// Stale.
    Stale,
    /// Wrong.
    Wrong,
}

/// The principal/delegation/authenticity subjects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthenticitySubject {
    /// The principal.
    Principal,
    /// The delegation.
    Delegation,
    /// The authenticity policy.
    AuthenticityPolicy,
}

/// The meet's failure modes — the security home's meet, never generic set
/// intersection: greatest lower bound where defined, typed refusal where
/// not; missing authority is never filled by an adapter, default, parent
/// process, ambient host permission, or successful prior Attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeetFailure {
    /// The intersection is empty — a typed, explainable empty.
    EmptyIntersection,
    /// The intersection is contradictory.
    ContradictoryIntersection,
    /// Noncommuting purposes have no meet.
    NoMeetNoncommutingPurposes,
    /// A stale generation refuses loudly rather than silently shrinking.
    StaleGeneration,
    /// A revoked grant.
    Revoked,
}

/// The compatibility subjects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilitySubject {
    /// The interface.
    Interface,
    /// The schema.
    Schema,
    /// The port.
    Port,
    /// The kernel.
    Kernel,
}

/// The admission-time availability subjects — admission-time only, never
/// dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AvailabilitySubject {
    /// A port.
    Port,
    /// A kernel.
    Kernel,
    /// The host profile.
    HostProfile,
}

/// Missing and stale are two conditions with two repairs — NEVER fused into
/// one token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequiredEvidencePosture {
    /// The evidence is missing.
    Missing,
    /// The evidence is stale.
    Stale,
}

/// The narrowing inputs of the bound intersection — the effective bound never
/// exceeds any narrowing input, and no adapter, optimization, retry,
/// process move, or fresh Attempt resets or enlarges it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NarrowingInput {
    /// The image's declared maxima.
    ProgramImageMaximum,
    /// The invocation or application request.
    InvocationOrApplicationRequest,
    /// The grant's resource scope.
    CapabilityGrantScope,
    /// The runtime's remaining bound.
    RuntimeRemainingBound,
    /// The deployment or host profile.
    DeploymentOrHostProfile,
}

/// The derived-floor breach: the floor derived from the request's COMPLETE
/// JUDGMENT — never a hand-kept registry entry — plus the declared value that
/// cannot meet it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloorDomain;
/// The derived-floor breach carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DerivedFloorBreach(pub Commitment<FloorDomain>);

/// Declared-evidence-requirement domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvidenceRequirementDomain;
/// Which evidence requirement the admitted profile declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeclaredEvidenceRequirement(pub Commitment<EvidenceRequirementDomain>);

/// Requested-reservation-semantics domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReservationSemanticsDomain;
/// The reservation semantics requested and not providable — DISTINCT from
/// capacity: the capacity exists and the semantics do not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RequestedReservationSemantics(pub Commitment<ReservationSemanticsDomain>);

/// Constraint-source domain marker for the meet's receipt pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstraintSourceDomain;

/// The fourteen admission issues — exact, ordered by the issue vocabulary's
/// canonical order, NEVER evaluation order (evaluation order is the admitted
/// profile's and is security-sensitive; rendering it would republish it).
/// Fresh Attempt creation is the success terminal and is NO issue at all.
/// Plural-subject stations carry a closed subcause AND one instance per
/// established subject — a one-token-per-station vocabulary would recompress
/// several established facts and report less danger than occurred. Details
/// are typed classifications only; an unmeasured dimension reports a typed
/// non-measurement, never a fabricated zero. Issue 6 is THE ONLY issue that
/// names a constraint-source pair.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttemptAdmissionIssue {
    /// The request's own declared facts are mutually incoherent — judged
    /// from the complete judgment's derived floors and NO host fact; refuses
    /// identically on every host.
    RequestIncoherent {
        /// The derived-floor breach.
        breach: DerivedFloorBreach,
    },
    /// Consumption of the executable verdict — never a re-interpretation of
    /// semantic law.
    ExecutableOrInvocationRoleUnadmitted {
        /// The subject.
        subject: ConsumedVerdictSubject,
    },
    /// An identity or lineage binding mismatches.
    IdentityOrLineageBindingMismatch {
        /// The subject.
        subject: BindingSubject,
    },
    /// A generation mismatches.
    GenerationMismatch {
        /// The axis.
        axis: GenerationAxis,
        /// The posture — stale is not wrong.
        posture: GenerationPosture,
    },
    /// Principal, delegation, or authenticity policy unsatisfied.
    PrincipalDelegationOrAuthenticityUnsatisfied {
        /// The subject.
        subject: AuthenticitySubject,
    },
    /// The capability meet is unsatisfied.
    CapabilityMeetUnsatisfied {
        /// The failure mode.
        mode: MeetFailure,
        /// The constraint-source pair that produced it.
        sources: ConstraintSourcePair<Commitment<ConstraintSourceDomain>>,
    },
    /// Interface, schema, port, or kernel incompatible.
    InterfaceSchemaPortOrKernelIncompatible {
        /// The subject.
        subject: CompatibilitySubject,
    },
    /// A port, kernel, or host profile is unavailable at admission.
    PortKernelOrHostUnavailable {
        /// The subject.
        subject: AvailabilitySubject,
    },
    /// Required evidence is unsatisfied.
    RequiredEvidenceUnsatisfied {
        /// The declared requirement.
        requirement: DeclaredEvidenceRequirement,
        /// Missing or stale — never fused.
        posture: RequiredEvidencePosture,
    },
    /// Logical authorization was not supplied — names the ABSENCE, never
    /// decides the semantic conclusion.
    LogicalAuthorizationNotSupplied,
    /// The bound intersection is unsatisfied — includes the
    /// deadline-allowance dimension.
    BoundIntersectionUnsatisfied {
        /// The dimension.
        dimension: DimensionId,
        /// The narrowing input.
        narrowing: NarrowingInput,
    },
    /// A required postcondition is unsupported — refused BEFORE a live
    /// Attempt, never a post-hoc observation; the vocabulary is the port
    /// home's.
    RequiredPostconditionUnsupported {
        /// The unsupported postcondition.
        postcondition: PortPostcondition,
    },
    /// The host could not satisfy an otherwise lawful bounded request —
    /// binds the physical record that established it.
    CapacityUnavailable {
        /// The observation.
        observation: ReservationObservation,
    },
    /// The capacity EXISTS and the requested reservation semantics do not.
    ReservationSemanticsUnavailable {
        /// The requested semantics.
        semantics: RequestedReservationSemantics,
    },
}

/// Limit family for admission issues — a declared finite bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdmissionIssueLimit;
impl Limit for AdmissionIssueLimit {}

/// The admission refusal family. The INVERSION RULE fixes its shape: a
/// canonical body that discards a second established violation reports less
/// danger than occurred. Posture is `EarlyStopped` by default — the
/// dependency order halts at the first station whose successors would
/// require facts the profile may not yet consult, and that halt IS the
/// stated reason; `Complete` only when every applicable check ran. THE
/// CANONICAL REFUSAL IS NOT THE RELEASED REFUSAL: the canonical body is
/// supplied to the runtime by typed reference; under a hostile threat
/// profile the released projection is grouped or constant-shape with a
/// CONSTANT CARDINALITY (the number of tripped conditions is itself an
/// oracle), and a grouped projection may not authorize retry, recovery,
/// disclosure, or authority a hidden canonical issue would forbid. A refusal
/// creates no Attempt, no reservation residue, no partial authority, and no
/// Attempt-shaped report — in every projection alike.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttemptAdmission {
    /// The established issues, in the vocabulary's canonical order.
    pub issues: NonEmptyBounded<AttemptAdmissionIssue, AdmissionIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for AttemptAdmission {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

// ---------------------------------------------------------------------------
// The Attempt lifecycle — affine live handles, persisted state, the report.
// ---------------------------------------------------------------------------

/// A pre-Attempt value — no Attempt exists yet, no postcondition. Admission
/// refusal means no Attempt EVER existed: no Attempt-shaped report, no
/// reservation residue, nothing to reconcile.
#[derive(Debug)]
pub struct PlannedInvocation {
    _process_local: PhantomData<*const ()>,
}

/// The FIRST Attempt state: grants, bounds, generations, capacity closed.
/// Non-`Clone`, non-serializable — only it can start, drive, or terminate
/// the Attempt; never a bare identity.
#[derive(Debug)]
pub struct AdmittedAttempt {
    attempt: AttemptId,
    _process_local: PhantomData<*const ()>,
}

impl AdmittedAttempt {
    /// In-crate mint for laws. Test-gated until the admission seam exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(attempt: AttemptId) -> Self {
        Self {
            attempt,
            _process_local: PhantomData,
        }
    }

    /// The Attempt's identity.
    #[must_use]
    pub fn attempt(&self) -> AttemptId {
        self.attempt
    }
}

/// The executor or one admitted host mechanism is advancing.
#[derive(Debug)]
pub struct RunningAttempt {
    attempt: AttemptId,
    _process_local: PhantomData<*const ()>,
}

impl RunningAttempt {
    /// The Attempt's identity.
    #[must_use]
    pub fn attempt(&self) -> AttemptId {
        self.attempt
    }
}

/// A bounded one-shot continuation awaits exactly one typed response.
/// Suspension returns to running only for the same live Attempt and only
/// once for the expected request.
#[derive(Debug)]
pub struct LiveSuspendedAttempt {
    attempt: AttemptId,
    _process_local: PhantomData<*const ()>,
}

impl LiveSuspendedAttempt {
    /// The Attempt's identity.
    #[must_use]
    pub fn attempt(&self) -> AttemptId {
        self.attempt
    }
}

/// The TERMINAL state: no further execution or response is legal; it cannot
/// return to running.
#[derive(Debug)]
pub struct TerminalAttempt {
    attempt: AttemptId,
    _process_local: PhantomData<*const ()>,
}

/// Attempt-evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttemptEvidenceClaim;

impl TerminalAttempt {
    /// In-crate mint for laws. Test-gated until the lifecycle seams exist.
    #[cfg(test)]
    pub(crate) const fn for_laws(attempt: AttemptId) -> Self {
        Self {
            attempt,
            _process_local: PhantomData,
        }
    }

    /// Sealing CONSUMES the terminal Attempt and mints the immutable report —
    /// evidence produced at the boundary, never a later identity or phase of
    /// the Attempt it observed. Reconciliation is never an Attempt phase.
    #[must_use]
    pub fn seal(self, evidence: EvidenceRef<AttemptEvidenceClaim>) -> AttemptReport {
        AttemptReport {
            attempt: self.attempt,
            evidence,
        }
    }
}

/// The PERSISTED state for recovery and evidence — not the live handle. A
/// decoded record re-enters live custody through validation, never by
/// construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttemptState {
    /// Admitted.
    Admitted,
    /// Running.
    Running,
    /// Live-suspended.
    LiveSuspended,
    /// Terminal.
    Terminal,
}

/// The immutable sealed report — owned here, consumed by the runtime BY
/// TYPED REFERENCE. It cannot by itself prove semantic retry legality,
/// checkpoint advancement, accepted-event truth, backend atomicity, external
/// completion, or a reconciliation conclusion — the Attempt-report route is
/// never its own only judge.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttemptReport {
    /// The Attempt this report seals.
    pub attempt: AttemptId,
    /// The sealed physical evidence.
    pub evidence: EvidenceRef<AttemptEvidenceClaim>,
}

/// The admission outcome — the ONLY minting site of an Attempt identity. A
/// refusal creates no Attempt at all (the seam returns the family body per
/// the canonical≠released law).
#[derive(Debug)]
pub enum AdmissionOutcome {
    /// Admitted: one fresh Attempt under live custody.
    Admitted(AdmittedAttempt),
    /// Refused: no Attempt ever existed.
    Refused(AttemptAdmission),
}

// ---------------------------------------------------------------------------
// Reservation and physical observations.
// ---------------------------------------------------------------------------

/// The Attempt-bound admitted physical envelope — AFFINE: created only after
/// prior checks pass, bound to the admitted plan and Attempt, released
/// EXACTLY ONCE, never exceeding the effective bound, surviving suspension
/// only as the suspension contract requires, never reused by another Attempt
/// and never reset or enlarged by retry, rescheduling, adapter conversion,
/// or containment movement. Partial reservation is legal only under an
/// explicit all-or-release protocol that exposes no admitted Attempt or
/// partial authority. `requested ≠ reserved ≠ consumed ≠ released`.
#[derive(Debug)]
pub struct ResourceReservation {
    _process_local: PhantomData<*const ()>,
}

/// Host-guarantee domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HostGuaranteeDomain;

/// The ONE home of "the host could not satisfy an otherwise lawful bounded
/// request" — a role-specific physical-observation record, a peer of the
/// storage-durability, port-response, and process-exit records, never a
/// universal payload. Evidence, never authority, never a bound. Two
/// bindings, two questions, NO conversion (the Attempt-existence line): the
/// runtime's resource-exhausted outcome binds it to answer how an ADMITTED
/// operation ended; the admission issue binds it to answer why NO Attempt
/// was admitted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReservationObservation {
    /// The requested capacity.
    pub requested: u64,
    /// The granted capacity.
    pub granted: u64,
    /// The unavailable capacity.
    pub unavailable: u64,
    /// Weaker host guarantees, classified.
    pub guarantees: Commitment<HostGuaranteeDomain>,
    /// The measurement uncertainty.
    pub uncertainty: u64,
}

/// Cost-model domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CostModelDomain;

/// A mechanism's PREDICTED cost under a NAMED model — the fifth resource
/// fact beside requested/reserved/consumed/released; never a bound and never
/// an observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalEstimate {
    /// The named model.
    pub model: Commitment<CostModelDomain>,
    /// The predicted cost.
    pub predicted: u64,
}

/// Reservation-evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReservationHeldClaim;

/// Proof a reservation was granted, held, and released exactly once —
/// evidence that a reservation existed, which is NOT the live reservation
/// itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReservationEvidence(pub EvidenceRef<ReservationHeldClaim>);

/// Storage-durability observation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StorageDurabilityDomain;
/// A storage-durability observation row (physical-observation family — the
/// family is a NAME, not a type: role-specific records sharing at most a
/// private common header; no public universal observation payload exists,
/// so a socket acknowledgement can never satisfy a durable-file witness).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StorageDurabilityObservation(pub Commitment<StorageDurabilityDomain>);

/// Port-response observation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortResponseDomain;
/// A port-response observation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortResponseObservation(pub Commitment<PortResponseDomain>);

/// Process-exit observation domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessExitDomain;
/// A process-exit observation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessExitObservation(pub Commitment<ProcessExitDomain>);

/// The twelve physical-observation kinds the model must distinguish —
/// different port and authority contracts expose only the observations they
/// can actually establish; each records its Attempt, request, mechanism,
/// generations, bounds, deadline, order bindings, source evidence, and
/// UNCERTAINTY, and claims no more than its source proves.
pub const PHYSICAL_OBSERVATION_KINDS: [&str; 12] = [
    "operation-requested",
    "operation-admitted",
    "physical-attempt-created",
    "mechanism-invocation-started",
    "request-submitted-beyond-local-process",
    "acknowledgement-observed",
    "completion-observed",
    "durability-evidence-observed",
    "cancellation-requested-or-observed",
    "deadline-expired",
    "mechanism-refused-or-failed",
    "outcome-remains-unknown",
];

/// The thirteen pairwise non-substitutions — pairwise, NOT an ordering; a
/// stronger witness exists only after its postcondition is established, and
/// nothing strengthens by position in a list, only through a declared
/// implication edge in the owning profile's implication graph.
pub const PAIRWISE_NON_SUBSTITUTION: [&str; 13] = [
    "capability-claim-vs-capability-grant",
    "requested-capability-vs-granted-capability",
    "requested-budget-vs-reserved-budget",
    "reserved-budget-vs-consumed-budget",
    "effect-requested-vs-effect-admitted",
    "effect-admitted-vs-effect-attempted",
    "effect-attempted-vs-effect-completed",
    "bytes-prepared-vs-bytes-written",
    "bytes-written-vs-bytes-durable",
    "file-content-durable-vs-namespace-publication-durable",
    "digest-matched-vs-authority-verified",
    "request-sent-vs-response-received",
    "response-received-vs-semantic-reconciliation",
];

/// Capability-witness claim marker (claim vs grant).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CapabilityWitnessClaim;
/// The capability witness family.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityWitness(pub EvidenceRef<CapabilityWitnessClaim>);

/// Budget-witness claim marker (requested/reserved/consumed/released).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BudgetWitnessClaim;
/// The budget witness family.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BudgetWitness(pub EvidenceRef<BudgetWitnessClaim>);

/// Effect-progress-witness claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EffectProgressWitnessClaim;
/// The effect-progress witness family.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectProgressWitness(pub EvidenceRef<EffectProgressWitnessClaim>);

/// Durability-witness claim marker (content vs namespace stay distinct).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DurabilityWitnessClaim;
/// The durability witness family.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DurabilityWitness(pub EvidenceRef<DurabilityWitnessClaim>);

/// Authenticity-witness claim marker (digest-matched vs authority-verified).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthenticityWitnessClaim;
/// The authenticity witness family.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthenticityWitness(pub EvidenceRef<AuthenticityWitnessClaim>);

/// Carrier-witness claim marker (request-sent vs response-received).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CarrierWitnessClaim;
/// The carrier witness family. Semantic reconciliation is a runtime
/// conclusion — never a physical witness at all.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CarrierWitness(pub EvidenceRef<CarrierWitnessClaim>);

// ---------------------------------------------------------------------------
// The port crossing.
// ---------------------------------------------------------------------------

/// The identity role marker for port requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortRequestRole;

/// One VM/port crossing's identity — DISTINCT from the remote face's
/// transport correlation identity; they relate only through typed carriage,
/// and matching raw bits never lets one satisfy the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortRequestId(Occurrence<PortRequestRole>);

impl IdentityRole for PortRequestId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl PortRequestId {
    /// In-crate mint for laws. Test-gated until the crossing seam exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<PortRequestRole>) -> Self {
        Self(occurrence)
    }
}

/// Request-payload domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortRequestPayloadDomain;

/// Limit family for a request's bound rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortRequestBoundLimit;
impl Limit for PortRequestBoundLimit {}

/// A typed claim made by ONE live Attempt. The port receives least authority
/// and only the data that request needs — no ambient access to the runtime,
/// application, grant table, or unrelated Attempt state; the response
/// resumes exactly the outstanding request of the same live Attempt at most
/// once, and the guest cannot observe which driving strategy carried it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PortRequest {
    /// The requesting Attempt.
    pub attempt: AttemptId,
    /// The request identity.
    pub request: PortRequestId,
    /// The expected port family and version.
    pub family: PortFamilyVersion,
    /// The typed payload's commitment.
    pub payload: Commitment<PortRequestPayloadDomain>,
    /// The remaining bounds this crossing runs under.
    pub bounds: Bounded<BoundDimensionRow, PortRequestBoundLimit>,
}

/// The ten port-request validation inputs — validated against the port
/// home's contracts before dispatch; the derived allowance is
/// clock-domain-rebased, never a transported raw instant.
pub const PORT_REQUEST_VALIDATION: [&str; 10] = [
    "attempt-and-request-identity",
    "turn-and-effect-intent-where-applicable",
    "expected-port-family-and-operation",
    "typed-input-and-schema-contract",
    "live-capability-handle-and-admitted-grant",
    "remaining-semantic-and-physical-bounds",
    "derived-operation-and-attempt-allowance",
    "generations-where-applicable",
    "idempotency-and-recovery-binding-where-required",
    "selected-host-and-containment-profile",
];

/// The ten cancellation/abandonment/deadline facts kept distinct. The
/// supervisor MAY prevent an unadmitted Attempt, signal a conforming
/// mechanism, terminate local execution, release physical resources, and
/// report that cancellation could not establish the external outcome; it
/// CANNOT infer noncommit, noncompletion, retry safety, or logical rollback
/// from any of these — the runtime, not the supervisor, decides the semantic
/// conclusion.
pub const CANCELLATION_FACTS: [&str; 10] = [
    "caller-stopped-waiting",
    "cancellation-requested",
    "cancellation-before-physical-admission",
    "after-admission-before-host-crossing",
    "during-or-after-host-crossing",
    "attempt-deadline-expiry",
    "operation-deadline-expiry",
    "late-observation-after-abandonment-or-expiry",
    "mechanism-does-not-support-cancellation",
    "cancellation-accepted-outcome-unknown",
];

// ---------------------------------------------------------------------------
// Containment.
// ---------------------------------------------------------------------------

/// The five containment profiles. The first two are containment/execution
/// postures, NOT isolation; the last three add host-level defense-in-depth
/// against a defect in the engine itself. An in-process Attempt is ALREADY
/// contained; stronger isolation is an explicit deployment profile, never an
/// implied property of interpretation — and none turns the supervisor into
/// an operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContainmentProfile {
    /// Same thread, in process.
    SameThreadInProcess,
    /// Another thread, in process.
    OtherThreadInProcess,
    /// A worker process.
    WorkerProcess,
    /// A browser worker.
    BrowserWorker,
    /// A remote qualified boundary.
    RemoteQualifiedBoundary,
}

/// The interaction shape — a SECOND coordinate, never a containment row: an
/// external-tool effect composes an interaction shape with the host
/// containment profile it runs under. Artifact-shaped I/O alone establishes
/// NO host containment; a profile running the tool as an ordinary process
/// declares the weaker posture as an explicit nonclaim. This is the batch
/// shape only — interactive, streaming, and persistent external interaction
/// reaches the outside through ordinary ports as bounded effects over the
/// same membrane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractionShape {
    /// An executor guest.
    PakVmGuest,
    /// A batch foreign binary: artifact in, artifact out — output re-enters
    /// as untrusted foreign claims through the firewall.
    ArtifactInArtifactOutExternalTool,
}

/// What the supervisor is NOT — the closed exclusion list.
pub const BVISOR_IS_NOT: [&str; 8] = [
    "an-operating-system",
    "a-hypervisor-abstraction",
    "an-application-compositor",
    "the-logical-runtime",
    "the-owner-of-turn-or-checkpoint-meaning",
    "the-owner-of-effect-causality-or-retry-legality",
    "the-owner-of-compensation-or-reconciliation",
    "proof-of-backend-atomicity-or-durability",
];
