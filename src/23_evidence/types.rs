//! Evidence: the vocabulary of distinct kinds, the receipt-family matrix,
//! the verification tuple, routes and independence, denominators, the
//! diagnostic epistemic posture, the calibration pair, and the four evidence
//! lifecycles.
//!
//! # The root supersession, recorded deliberately
//!
//! The freshness, proof-disposition, and completeness AXIS SHAPES live at the
//! root calculus (generic, non-erasable parameters); this home keeps the
//! machinery and the semantics of "complete over what". The warning that
//! travels with that split: a flattened anything-freshness enum is the
//! opposite collapse and is equally refused — the root shape is safe only
//! while the cut parameter stays non-erasable and claim-family-specific.
//!
//! # The reason-identity law (the seat band 00 has cited all along)
//!
//! Every refusal family's stable reason identity is minted under THIS home's
//! identity law: family-level, never per-issue, derived under the admitted
//! digest mechanism (pending the repository owner's mechanism admission).
//! This home owns the identity law and the diagnostic epistemic posture; it
//! never co-owns any family's meanings.
//!
//! # Append-only evidence
//!
//! Evidence is append-only with respect to what was observed and concluded
//! at the time. Later evidence may corroborate, contradict, reconcile,
//! establish a stronger claim, establish nonexecution, mark applicability
//! stale, supersede an artifact for future use, or preserve continuing
//! uncertainty — but never rewrites the prior record into a claim the prior
//! boundary never made, and later green evidence never erases earlier red
//! evidence. Freshness invalidation changes only present admissibility.
//! Sealed constructors: verified and committed constructors sit behind the
//! owning verifier so public callers cannot forge success by populating
//! fields. The producer is never the only decoder, preimage implementation,
//! or verdict path for a high-value claim — and successful parsing does not
//! imply successful verification.

use crate::identity::Commitment;
use crate::types::{Bounded, Completeness, EvidenceRef, Limit};

// ---------------------------------------------------------------------------
// The non-collapse law and the receipt-family matrix.
// ---------------------------------------------------------------------------

/// The root-level non-collapse roster — a value may REFER to several of
/// these but cannot claim their combined authority merely by containing
/// their fields. This is the constraint that forbids a universal result
/// wrapper anywhere in the crate.
pub const EVIDENCE_NON_COLLAPSE: [&str; 15] = [
    "result",
    "refusal",
    "event",
    "receipt",
    "explanation",
    "observation",
    "domain-report",
    "admitted-observation",
    "estimate",
    "proof",
    "diagnostic",
    "log-trace",
    "metric",
    "capability",
    "release-promise",
];

/// The twenty-five receipt families, organized BY SEMANTIC BOUNDARY — never
/// by package, transport, executor, or subsystem. Only the owning boundary
/// constructs a successful receipt for its claim; an unknown operation,
/// unknown family, missing owner, rejected evidence sink, or failed
/// publication cannot fabricate success. Rows 18 (image invocation) and 23
/// (proposal and adoption) are STRUCTURALLY TWO-RECORD families related by
/// typed reference — their one-record collapse is the refusal. For an
/// ordered admitted batch, command and receipt positions correspond exactly.
pub const RECEIPT_FAMILIES: [&str; 25] = [
    "operation-admission",
    "accepted-publication-and-durability",
    "named-materialization-guarantee",
    "logical-invocation-and-turn",
    "effect-intent-admission",
    "physical-attempt",
    "external-outcome-or-completion",
    "reconciliation",
    "process-or-subscription-progress",
    "carrier-delivery",
    "store-open-and-recovery",
    "namespace-publication",
    "compaction-or-rebuild",
    "protected-payload-publication",
    "secret-destruction-and-shred",
    "partition-handoff-and-epoch",
    "image-validation",
    "image-invocation-two-record",
    "specialization-or-qualified-realization",
    "query-or-decision-result",
    "numeric-information-loss-crossing",
    "qualification-and-mutation",
    "proposal-and-adoption-two-record",
    "generated-publication",
    "release-and-support",
];

/// Per-item evidence carriage — elemental per item; a profile with several
/// items mixes them through a bounded role-to-carriage plan, never a global
/// hybrid value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvidenceCarriage {
    /// Carried inline.
    Inline,
    /// Referenced by immutable content identity.
    ImmutableReference,
}

/// Commitment-layer domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommitmentLayerDomain;

/// Limit family for commitment layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommitmentLayerLimit;
impl Limit for CommitmentLayerLimit {}

/// Commitment layers COEXIST — a digest AND a signature AND a freshness
/// witness prove different claims, so a pick-one enum is the refusal. Each
/// entry references one commitment role (the bytes home's neutral sum) and
/// carries its verifier, profile, subject, key/witness relationship, and
/// exact claim and nonclaim. Only stable structured fields participate in a
/// commitment; evidence travels as the exact canonical bytes its commitment
/// covers — a re-encoded typed mirror is a different byte role with no
/// morphism back to the commitment.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommitmentLayers {
    /// The coexisting layers.
    pub layers: Bounded<Commitment<CommitmentLayerDomain>, CommitmentLayerLimit>,
}

// ---------------------------------------------------------------------------
// The verification tuple — a tuple, not a ladder.
// ---------------------------------------------------------------------------

/// The verdict basis. A contract projection (generated from the same owner)
/// is never sufficient as the sole judge; runtime observation may
/// corroborate but is never the sole verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Basis {
    /// A projection generated from the owner's own contract.
    ContractProjection,
    /// An independent reference implementation.
    IndependentReference,
    /// A direct hostile boundary.
    DirectBoundary,
    /// Runtime observation.
    RuntimeObservation,
}

/// The verification methods — A METHOD IS NOT A RANK, and selecting the
/// formal check confers none: fuzzing, mutation, formal proof, simulation,
/// differential execution, runtime observation, and independent
/// reconstruction answer different claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    /// A structural rule.
    StructuralRule,
    /// A compile refusal (an unrepresentability witness).
    CompileRefusal,
    /// A property sequence.
    PropertySequence,
    /// Bounded state exploration.
    BoundedStateExploration,
    /// Schedule exploration.
    ScheduleExploration,
    /// Deterministic simulation.
    DeterministicSimulation,
    /// Differential execution.
    DifferentialExecution,
    /// Translation validation — mismatch invalidates the image; matching
    /// digests supply no missing comparison.
    TranslationValidation,
    /// Fault injection.
    FaultInjection,
    /// Crash recovery.
    CrashRecovery,
    /// Fuzzing.
    Fuzzing,
    /// Mutation.
    Mutation,
    /// A complexity contract — measured in a named work currency, never
    /// elapsed time; anti-vacuous only with a planted worse-class
    /// realization the same procedure rejects.
    ComplexityContract,
    /// A benchmark envelope — evidence about one realization, never a
    /// specification; performance never compensates for semantic
    /// disagreement.
    BenchmarkEnvelope,
    /// History replay.
    HistoryReplay,
    /// A formal check — refuses when any binding (kind, exact claim,
    /// assumptions, the disclosed trust boundary, model/extraction seam,
    /// scope, bounds, checker identity, nonclaims) is absent.
    FormalCheck,
}

/// The verified claim kinds — a passing safety result establishes no
/// liveness; a benchmark establishes no refinement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerifiedClaim {
    /// Safety.
    Safety,
    /// Liveness.
    Liveness,
    /// Bounded response.
    BoundedResponse,
    /// Convergence.
    Convergence,
    /// Stability.
    Stability,
    /// Non-oscillation.
    NonOscillation,
    /// Determinism.
    Determinism,
    /// Refinement.
    Refinement,
    /// Conformance.
    Conformance,
    /// A resource envelope.
    ResourceEnvelope,
}

/// Coverage — EXPLICITLY UNORDERED (no `Ord`, deliberately): exhaustive
/// coverage of a small model does not outrank observed history, and observed
/// history says nothing about unobserved paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Coverage {
    /// Sampled.
    Sampled,
    /// Bounded.
    Bounded,
    /// Exhaustive within the declared model.
    ExhaustiveWithinDeclaredModel,
    /// Observed history — bounded to what was captured.
    ObservedHistory,
}

/// Enforcement — AUTHORED POLICY, not something an observation grants
/// itself; the evidence result stays separate from the enforcement
/// consequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Enforcement {
    /// Blocking.
    Blocking,
    /// Quarantine.
    Quarantine,
    /// Advisory.
    Advisory,
}

/// Lane-identity domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LaneDomain;

/// A verification lane — the typed axis, with the roster owner-declared. The
/// LAW lives here and binds every declared lane: passing a faster lane never
/// discharges a requirement assigned to a broader one, and every required
/// lane stays visible in the expected denominator with its own disposition.
/// WHICH lanes exist is the qualifying owner's declaration (testpak and its
/// adapters), never a roster in core — a lane roster spelled here would be a
/// second vocabulary competing with the owner's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lane(pub Commitment<LaneDomain>);

/// The verification lifecycle terminal — `Falsified` is a CONCLUDED run,
/// not a different terminal; an aborted run's causes live in the gaps record
/// and never shrink the denominator. Terminal algebras are lifecycle-owned;
/// no universal terminal status exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationTerminal {
    /// The run concluded (established, falsified, corroborated, narrowed —
    /// the proof axis says which).
    Concluded,
    /// The run aborted — gaps visible.
    Aborted,
}

/// The qualification lifecycle terminal — its verdict IS its lifecycle
/// conclusion; `Abandoned` ended with no verdict, gaps visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualificationTerminal {
    /// Qualified.
    Qualified,
    /// Failed.
    Failed,
    /// Abandoned — no verdict.
    Abandoned,
}

/// Denominator domain marker (the population an axis is complete over —
/// concrete shapes are per-owner).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DenominatorDomain;

/// The verification denominator — an instantiation of the root completeness
/// shape, deriving EXACTLY from expected versus executed; never a third
/// completeness claim. Nothing shrinks the denominator: feature selection,
/// runner filters, unsupported targets, stale evidence, infrastructure
/// failure, discovery drift, and tool limits leave every expected row with a
/// visible disposition; zero executed units are never green; retries
/// preserve every attempt and a later pass never erases an earlier failure.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VerificationDenominator(pub Completeness<Commitment<DenominatorDomain>>);

/// The verification tuple — each axis its own typed value, NEVER flattened
/// into one status enum, score, or level: a stronger-looking result in one
/// axis cannot launder weakness in another. Freshness rides the root axis on
/// the evidence; the proof axis is the root's disposition; only
/// lifecycle-specific terminal meaning remains in the terminal member.
#[must_use = "a verification result carries nine axes, and none of them survives being dropped"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VerificationResult {
    /// The basis.
    pub basis: Basis,
    /// The method.
    pub method: Method,
    /// The claim.
    pub claim: VerifiedClaim,
    /// The coverage.
    pub coverage: Coverage,
    /// The enforcement policy.
    pub enforcement: Enforcement,
    /// The lane.
    pub lane: Lane,
    /// The denominator axis.
    pub denominator: VerificationDenominator,
    /// The proof axis (the root's disposition).
    pub proof: crate::types::ProofDisposition,
    /// The lifecycle terminal.
    pub terminal: VerificationTerminal,
}

// ---------------------------------------------------------------------------
// Routes and independence.
// ---------------------------------------------------------------------------

/// The independent routes — BASIS-BOUND: differential implementation and
/// independent history replay require the independent-reference basis; the
/// hostile boundary requires the direct-boundary basis; invalid combinations
/// refuse structurally — a route label cannot acquire an independent
/// verdict. Independence is behavioral and claim-relative: two wrappers
/// around one implementation, two binaries from one verdict path, or two
/// projections generated from one source that agree with each other are ONE
/// route; agreement is evidence only across an independence boundary. Two
/// obligations, neither substituting: projection completeness AND oracle
/// independence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Route {
    /// A differential implementation.
    DifferentialImplementation,
    /// Independent history replay.
    IndependentHistoryReplay,
    /// A hostile boundary.
    HostileBoundary,
}

/// Substrate-disclosure domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubstrateDomain;

/// The mandatory per-route substrate disclosure — compiler, stdlib, linker,
/// build tools, dependencies, target support, generated inputs, runtime,
/// infrastructure. Sharing one disclosed substrate does not erase
/// independence for unrelated failure classes, but cannot establish
/// independence from a defect in that substrate: a claim exposed to
/// correlated failure either names another qualified route or records that
/// exact failure class as unproved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubstrateDisclosure(pub Commitment<SubstrateDomain>);

// ---------------------------------------------------------------------------
// The diagnostic epistemic posture.
// ---------------------------------------------------------------------------

/// One established diagnostic cause.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagnosticCauseDomain;
/// The established-cause carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagnosticCause(pub Commitment<DiagnosticCauseDomain>);

/// Limit family for narrowed suspect sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CauseSuspectLimit;
impl Limit for CauseSuspectLimit {}

/// The bounded narrowed-suspect set — its public meaning is "narrowed
/// suspects", never a raw bounded vector and never a bare universal cause.
/// Three causation vocabularies stay apart: the runtime home's Attempt
/// lineage set, the refusal home's handling chain, and this investigation
/// carrier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiagnosticCauseSuspects {
    /// The narrowed suspects.
    pub suspects: Bounded<DiagnosticCause, CauseSuspectLimit>,
}

/// The diagnostic epistemic posture — NARROWING IS PROGRESS, NOT A FORCED
/// VERDICT: identical admitted facts produce identical ordering,
/// investigation, and required action; projections cannot upgrade a
/// correlation into a root cause or a suggested edit into authority.
#[must_use = "a cause posture is what diagnosis established, and narrowing is progress"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CauseDisposition {
    /// One cause was established.
    EstablishedCause(DiagnosticCause),
    /// The suspects were narrowed.
    NarrowedCauseSuspects(DiagnosticCauseSuspects),
    /// Unresolved.
    UnresolvedCause,
}

// ---------------------------------------------------------------------------
// The calibration pair — this home's two work records.
// ---------------------------------------------------------------------------

/// Calibration-model domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CalibrationModelDomain;

/// Relates predicted to measured — and OWNS NEITHER: a calibration model
/// owns neither semantic work nor physical truth. One of the seven
/// role-specific work records; no universal work-observation envelope
/// exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CalibrationModel(pub Commitment<CalibrationModelDomain>);

/// Calibration-evidence claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CalibrationClaim;

/// Tests a calibration model against observations. Stale calibration is
/// never reused; a model score or scalar confidence cannot launder
/// uncertainty, incompleteness, staleness, or weak proof into authority.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CalibrationEvidence {
    /// The model under test.
    pub model: CalibrationModel,
    /// The evidence.
    pub evidence: EvidenceRef<CalibrationClaim>,
}

// ---------------------------------------------------------------------------
// The four evidence lifecycles — separate authorities, never one machine.
// ---------------------------------------------------------------------------

/// Qualification-reference claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QualificationClaim;
/// Principal domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrincipalDomain;
/// Frozen-semantic-target domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrozenTargetDomain;

/// The adoption decision — AUTHORED rename of the banned lifecycle noun: a
/// HUMAN authority act, separate from qualification and from release, that
/// confers implementation authority on qualified bytes. Proposal origin
/// grants no adoption; qualification creates no release promise; no self-
/// or automatic adoption exists. The three-authority separation (qualify ≠
/// adopt ≠ release) is law that survives the rename. (The full
/// qualification-evidence record is the testpak crate's — production never
/// depends on testpak.)
#[must_use = "a receipt records that a human authority adopted qualified bytes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdoptionDecisionReceipt {
    /// The qualification this adoption rests on.
    pub qualified: EvidenceRef<QualificationClaim>,
    /// The adopting principal — a human act by the configured authority.
    pub adopted_by: Commitment<PrincipalDomain>,
    /// The frozen semantic target.
    pub target: Commitment<FrozenTargetDomain>,
}

/// Publication-unit domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PublicationUnitDomain;
/// Staged-output claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StagedOutputClaim;
/// Independent-manifest claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndependentManifestClaim;

/// One declared publication unit was staged, independently checked, and
/// atomically published — partial publication is a refusal, never a partial
/// success; generated origin confers no semantic authority.
///
/// The narrow owner: this receipt exists ONLY where tooling materializes a
/// COMPLETE set of sibling projections across a publication boundary — staged
/// as a unit, independently checked as a unit, published atomically as a
/// unit. Ordinary token expansion mints no receipt: a proc-macro expanding
/// declarations in place crosses no publication boundary and publishes
/// nothing. Nothing here confers standing on generated bytes, and no
/// construction-lifecycle machinery hides behind it.
/// (Type name pending the repository owner's review.)
#[must_use = "a receipt records that a complete unit was staged, checked, and published"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratedPublicationReceipt {
    /// The publication unit.
    pub unit: Commitment<PublicationUnitDomain>,
    /// The staged outputs.
    pub staged: EvidenceRef<StagedOutputClaim>,
    /// The independent manifest.
    pub manifest: EvidenceRef<IndependentManifestClaim>,
}

/// Release-row claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReleaseRowClaim;
/// Release-artifact domain marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReleaseArtifactDomain;

/// Release evidence — a CONJUNCTION of role-specific claims, not one green
/// badge; a graph, not one universal record. Binds the exact qualified
/// artifact per target and build profile: a rebuild, byte-changing
/// repackaging, or mutable-tag substitution creates a NEW artifact claim. A
/// repository toolchain pin is not a support promise; expected and executed
/// denominators must agree. The decision to promise a supported row belongs
/// to the release owner — a fourth authority, separate from all three
/// above.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReleaseEvidence {
    /// The support rows.
    pub rows: EvidenceRef<ReleaseRowClaim>,
    /// The exact artifacts and digests.
    pub artifacts: Commitment<ReleaseArtifactDomain>,
    /// The expected-vs-executed denominator.
    pub denominator: VerificationDenominator,
}

/// The four explanation-ladder levels over ONE meaning — deeper but never
/// divergent; expansion bottoms out at admitted operations and qualified
/// kernels, never at natural-language words treated as opcodes. An
/// explanation comes from the SAME evaluation that produced the result —
/// never reconstructed afterward by a second evaluator or a model inventing
/// plausible prose; prose is not query identity, policy, or proof.
pub const EXPLANATION_LADDER: [&str; 4] = [
    "concise-human-description",
    "typed-semantic-signature",
    "structured-explanation-from-the-same-evaluation",
    "full-definitional-expansion",
];
