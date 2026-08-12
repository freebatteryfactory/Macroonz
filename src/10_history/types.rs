//! Accepted history: the four-object split, exact local order, lineage,
//! partitions and handoff, federation cuts, commit knowledge, durability,
//! the storage port, authenticated history, authorized removal, and the
//! `.tlog` recovery law.
//!
//! # The four-object split
//!
//! An event's immutable commitment cannot contain a durable cut that exists
//! only after its own batch publishes — that is a circular pre-claim. Four
//! objects, four times: the semantic body (authoring), the accepted record
//! (admission — where exact local order attaches), the publication record (the
//! batch crossing), and the commit point (the receipt). The event commitment
//! binds the first two stages only. An event record never grows a "committed?"
//! field; the knowledge axis carries it.
//!
//! # The committed-boundary recovery law
//!
//! Recovery is committed-boundary-bounded, not position-bounded: the boundary
//! is the last valid durably committed publication boundary — the store's own
//! commit-point receipt IN THE BYTES, never caller knowledge. Beyond it,
//! discard-with-receipt (nothing published was deleted, because the boundary
//! never became durable); at or before it, invalid bytes are a broken
//! durability promise and refuse-and-hold, never "recovered past". Caller
//! acknowledgement is evidence about caller knowledge — committed-but-
//! unacknowledged data may never be discarded merely because the
//! acknowledgement was lost in transit. Recovery ends at a committed prefix,
//! lawful rollback, or typed refusal — no fourth ending.

use crate::identity::{
    AuthorityPosition, Commitment, CreationLaw, IdentityClass, IdentityRole, Occurrence,
};
use crate::refusal::{CompletionPosture, FamilyShape, RefusalFamily};
use crate::schema::SchemaSemanticCommitment;
use crate::types::{Bounded, Completeness, EvidenceCut, EvidenceRef, Freshness, Limit};
use crate::value::BoundedText;

// ---------------------------------------------------------------------------
// Identity instantiations.
// ---------------------------------------------------------------------------

/// The identity role marker for stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreRole;

/// One physical store — Class D, fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreId(Occurrence<StoreRole>);

impl IdentityRole for StoreId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl StoreId {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<StoreRole>) -> Self {
        Self(occurrence)
    }
}

/// The identity role marker for store lineages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreLineageRole;

/// One store lineage — Class D, fresh: a lineage is a happening, not content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoreLineageId(Occurrence<StoreLineageRole>);

impl IdentityRole for StoreLineageId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

impl StoreLineageId {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(occurrence: Occurrence<StoreLineageRole>) -> Self {
        Self(occurrence)
    }
}

/// One authority generation — Class C, scoped to its lineage. The ordering
/// contract is fixed per generation: any change that would make old and new
/// sequence values incomparable mints a new generation, so the generation a
/// value carries names its ordering contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthorityGeneration(AuthorityPosition<StoreLineageId>);

impl AuthorityGeneration {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) fn for_laws(seed: u8) -> Self {
        Self(AuthorityPosition::assigned(
            StoreLineageId::for_laws(Occurrence::for_laws(
                crate::identity::OccurrenceForm::Fresh([seed; 16]),
            )),
            u64::from(seed),
        ))
    }
}

/// The identity role marker for partitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PartitionRole;

/// One logical partition — Class D, fresh, NO preimage by law: names and
/// ranges are mutable configuration, and a preimage over reusable facts could
/// resurrect a retired partition's identity. It survives handoff because it is
/// carried, not because it is re-derivable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PartitionId(Occurrence<PartitionRole>);

impl IdentityRole for PartitionId {
    const CLASS: IdentityClass = IdentityClass::Occurrence;
    const CREATION: CreationLaw = CreationLaw::FreshOpaque;
}

/// One write-authority epoch — Class C, scoped to its partition. No state
/// admits both epochs accepting writes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WriteAuthorityEpoch(AuthorityPosition<PartitionId>);

/// The identity role marker for events. `EventId` deliberately declares NO
/// fixed creation law: the register's one delegated row — the creation law is
/// selected per event contract (fresh 16-byte generated-opaque, or derived
/// 32-byte where the derived-seat law's two seats are earned), and the
/// contract row names its law with no ambient default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventRole;

/// One accepted event — Class D; creation law per event contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId(Occurrence<EventRole>);

/// The domain marker for event commitments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventCommitmentDomain;

/// The event's semantic commitment — Class A, keyed when protected. Its
/// preimage binds the semantic body and admission relationships (the first two
/// stages of the four-object split ONLY) — never the publication or the cut.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventCommitment(Commitment<EventCommitmentDomain>);

impl IdentityRole for EventCommitment {
    const CLASS: IdentityClass = IdentityClass::SemanticCommitment;
    const CREATION: CreationLaw = CreationLaw::DomainTaggedDigestOfMeaning;
}

impl EventCommitment {
    /// In-crate mint for laws. Test-gated until digest derivation exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(commitment: Commitment<EventCommitmentDomain>) -> Self {
        Self(commitment)
    }
}

// ---------------------------------------------------------------------------
// Exact local order and durable cuts.
// ---------------------------------------------------------------------------

/// The one scope both order roles share: lineage + generation (+ partition
/// where applicable). LANE IS NEVER A COMPONENT: lanes organize order inside
/// one writer authority, and a lane-scoped ordinal would make the complete
/// cross-lane writer order incomparable. A scoped ordinal carries scope and
/// order only; auxiliary facts ride typed relations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WriterOrderScope {
    /// The store lineage.
    pub lineage: StoreLineageId,
    /// The authority generation.
    pub generation: AuthorityGeneration,
    /// The partition, where applicable.
    pub partition: Option<PartitionId>,
}

/// Exact order under ONE local writer authority — never global. Role-distinct
/// from the cut; no conversion bridges them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthoritySequence {
    scope: WriterOrderScope,
    order: u64,
}

impl AuthoritySequence {
    /// In-crate mint for laws. Test-gated until admission minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(scope: WriterOrderScope, order: u64) -> Self {
        Self { scope, order }
    }

    /// The writer-order scope.
    #[must_use]
    pub fn scope(&self) -> &WriterOrderScope {
        &self.scope
    }

    /// The order position within the scope.
    #[must_use]
    pub fn order(&self) -> u64 {
        self.order
    }
}

/// An exact durable historical cut: the same writer-order scope under its own
/// role-distinct domain tag, plus the cut's ordinal ceiling (the visible
/// ceiling of published history — the watermark-family word is banned
/// vocabulary; the substance is unchanged). Never a wall-clock
/// time, HLC, page cursor, item count, route, or best-known observation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommitPoint {
    scope: WriterOrderScope,
    ceiling: u64,
}

impl CommitPoint {
    /// In-crate mint for laws. Test-gated until publication minting exists.
    #[cfg(test)]
    pub(crate) const fn for_laws(scope: WriterOrderScope, ceiling: u64) -> Self {
        Self { scope, ceiling }
    }

    /// The writer-order scope.
    #[must_use]
    pub fn scope(&self) -> &WriterOrderScope {
        &self.scope
    }

    /// The cut's ordinal ceiling.
    #[must_use]
    pub fn ceiling(&self) -> u64 {
        self.ceiling
    }
}

/// The domain marker for applied-cut operation bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CutOperationDomain;

/// Exact local-order progress for one declared scope and operation. It proves
/// only the named projection's progress; it does not make it event authority,
/// and it can never be replaced by an HLC summary or a generic completeness
/// ceiling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeAppliedCut {
    /// The writer-order scope.
    pub scope: WriterOrderScope,
    /// The bound operation.
    pub operation: Commitment<CutOperationDomain>,
    /// The progress ceiling.
    pub ceiling: u64,
}

/// What the Turn read — frozen without advancing a checkpoint. Shares the
/// layout law under a role-distinct domain tag: wrong role is wrong tag is
/// refusal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TurnInputCut {
    /// The writer-order scope.
    pub scope: WriterOrderScope,
    /// The read ceiling.
    pub ceiling: u64,
}

// ---------------------------------------------------------------------------
// Lineage transitions — the machine's first composite-pair refusal family.
// ---------------------------------------------------------------------------

/// The claim marker for lineage provenance references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineageProvenanceClaim;

/// Claim-specific evidence for a foreign-lineage import: identity, the
/// relationship established, the source cut, provenance. Never a universal
/// evidence record.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForeignLineageEvidence {
    /// The foreign lineage.
    pub lineage: StoreLineageId,
    /// The source cut the relationship was established at.
    pub source_cut: CommitPoint,
    /// The provenance reference.
    pub provenance: EvidenceRef<LineageProvenanceClaim>,
}

/// A lineage transition — a SUCCESS value stating an established relationship,
/// carrying the cut it proves. Inability to prove is a refusal, not a variant.
/// Restore, import, fork, snapshot, compaction, authority replacement, and
/// reattachment cannot silently preserve or change lineage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LineageTransition {
    /// Same lineage, same generation.
    ContinueLineage {
        /// The continuing generation.
        generation: AuthorityGeneration,
    },
    /// A new generation within the lineage.
    NewGeneration {
        /// The cut the new generation starts from.
        from_cut: CommitPoint,
    },
    /// A new lineage derived from a named cut.
    NewLineageFromCut {
        /// The source lineage.
        source: StoreLineageId,
        /// The deriving cut.
        from_cut: CommitPoint,
    },
    /// A foreign lineage imported as immutable evidence.
    ImportForeignLineage {
        /// The import evidence.
        evidence: ForeignLineageEvidence,
    },
}

/// The reason member of the lineage refusal — closed single-cause enum with
/// the declared selection order: positive contradiction outranks every weaker
/// reading; a source cut must resolve before the commitment at it can be
/// compared; the residual holds exactly when nothing stronger was established.
/// Reasons are unit: the record already owns one evidence home, and a
/// per-reason carrier would store one fact twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineageRefusalReason {
    /// Evidence positively contradicts the claimed relationship — never
    /// down-reported as unproven.
    ContradictoryLineageClaim,
    /// The named source cut does not resolve under the claimed lineage.
    WrongSourceCut,
    /// The commitment at the named cut is not the one the transition claims.
    IncompatibleCommitment,
    /// Nothing stronger was established.
    RelationshipUnproven,
}

/// The claim marker for partial lineage-refusal evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineagePartialClaim;

/// The partial relationship evidence a failed transition attempt produced —
/// claim-specific, and never the success carrier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineageRefusalEvidence {
    /// The partial evidence reference.
    pub partial: EvidenceRef<LineagePartialClaim>,
}

/// The machine's first composite-pair refusal family: two members, neither
/// droppable — the reason answers WHAT was established against the claim, the
/// evidence answers WHAT PARTIAL RELATIONSHIP the attempt produced; neither
/// answer means anything alone. It establishes no relationship, changes no
/// accepted material, and owns neither fork detection nor equivocation
/// handling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineageRefusal {
    /// The established reason.
    pub reason: LineageRefusalReason,
    /// The partial evidence.
    pub evidence: LineageRefusalEvidence,
}

impl RefusalFamily for LineageRefusal {
    const SHAPE: FamilyShape = FamilyShape::InseparablePair;
    const SELECTION_ORDER: &'static [&'static str] = &[
        "ContradictoryLineageClaim",
        "WrongSourceCut",
        "IncompatibleCommitment",
        "RelationshipUnproven",
    ];
}

// ---------------------------------------------------------------------------
// Partitions, handoff, federation.
// ---------------------------------------------------------------------------

/// The persisted/recovery handoff state — NOT the live handle. The live
/// discipline is affine typestate: each phase consumes the prior handle, a
/// later phase cannot be forged or deserialized into existence, and routing is
/// published LAST because routing reports authority — it does not grant it. A
/// decoded record re-enters live custody through validation, never by
/// construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandoffState {
    /// The source is sealed at a cut.
    SealedSource,
    /// State preserved or transferred.
    StatePreservedOrTransferred,
    /// The target imported the state.
    TargetImported,
    /// The new epoch is activated.
    EpochActivated,
    /// The route is published.
    RoutePublished,
}

/// Limit family for successor sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SuccessorLimit;
impl Limit for SuccessorLimit {}

/// The split/merge coverage witness, proving as its own claims: successors
/// pairwise disjoint; their union equals the sealed predecessor coverage; each
/// successor receives a fresh epoch; inherited accepted events retain their
/// identities and original accepted positions. A predecessor cut and a
/// successor cut are joined only by an explicit succession or cut-translation
/// witness — never by matching integer components.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoverageWitness {
    /// The sealed predecessor.
    pub predecessor: PartitionId,
    /// The seal cut.
    pub sealed_at: CommitPoint,
    /// The successor partitions.
    pub successors: Bounded<PartitionId, SuccessorLimit>,
}

/// The epoch-validation refusal: every write validates the epoch under which
/// it was admitted. Routing reports authority; it does not grant it — a
/// reachable old writer, valid connection, authenticated host, nearby shard,
/// or later HLC cannot admit work under a stale epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EpochValidation {
    /// The write's epoch has been superseded.
    StaleEpoch,
}

impl RefusalFamily for EpochValidation {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["StaleEpoch"];
}

/// The claim marker for succession witnesses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SuccessionClaim;

/// The claim marker for cut-translation witnesses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CutTranslationClaim;

/// The explicit succession witness joining a predecessor cut to a successor
/// cut — the ONLY lawful join; matching integer components never join cuts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuccessionWitness {
    /// The predecessor's sealed cut.
    pub predecessor: CommitPoint,
    /// The successor's cut.
    pub successor: CommitPoint,
    /// The succession evidence.
    pub evidence: EvidenceRef<SuccessionClaim>,
}

/// The explicit cut-translation witness carrying one cut's meaning across a
/// partition transition.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CutTranslationWitness {
    /// The cut translated from.
    pub source: CommitPoint,
    /// The cut translated to.
    pub translated: CommitPoint,
    /// The translation evidence.
    pub evidence: EvidenceRef<CutTranslationClaim>,
}

/// Limit family for federation entry sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FederationLimit;
impl Limit for FederationLimit {}

/// Federation composition refusal: checked composition of already-established
/// cuts — explicit authority entries, closed source-set membership,
/// deterministic ordering; duplicate and omitted authorities refuse; no
/// raw-map constructor exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FederationComposition {
    /// The same authority appears twice.
    DuplicateAuthority,
    /// A declared authority is missing from the entries.
    OmittedAuthority,
}

impl RefusalFamily for FederationComposition {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["DuplicateAuthority", "OmittedAuthority"];
}

/// The closed, store-id-sorted federation entries — role-distinct; a generic
/// pair-vector is never the public meaning.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FederationCutEntries {
    entries: Vec<(StoreId, CommitPoint)>,
}

impl FederationCutEntries {
    /// Checked composition: sorts deterministically by store identity, refuses
    /// duplicates and omissions against the declared source set. Composing the
    /// vector asserts no federation-wide transaction or authority, and never
    /// fabricates one cross-store order.
    ///
    /// # Errors
    ///
    /// Returns the family body on a duplicate or omitted authority.
    pub fn composed(
        declared: &[StoreId],
        mut entries: Vec<(StoreId, CommitPoint)>,
    ) -> Result<Self, FederationComposition> {
        fn storage_key(id: &StoreId) -> (u8, &[u8]) {
            match id.0.form() {
                crate::identity::OccurrenceForm::Derived(bytes) => (0, bytes.as_slice()),
                crate::identity::OccurrenceForm::Fresh(bytes) => (1, bytes.as_slice()),
            }
        }
        entries.sort_by(|left, right| storage_key(&left.0).cmp(&storage_key(&right.0)));
        let mut previous: Option<&StoreId> = None;
        for (store, _) in &entries {
            if previous.is_some_and(|last| last == store) {
                return Err(FederationComposition::DuplicateAuthority);
            }
            previous = Some(store);
        }
        for required in declared {
            if !entries.iter().any(|(store, _)| store == required) {
                return Err(FederationComposition::OmittedAuthority);
            }
        }
        Ok(Self { entries })
    }

    /// Number of participating authorities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the vector is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// One exact durable cut per participating authority — federation never
/// invents one federation-wide order. Componentwise comparison reports what
/// each store has durably accepted; it proves no distributed transaction,
/// simultaneous visibility, causal dependency, or shared writer authority.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FederationCutVector {
    /// The per-authority entries.
    pub entries: FederationCutEntries,
}

// ---------------------------------------------------------------------------
// Predecessor, causation, commit knowledge.
// ---------------------------------------------------------------------------

/// The domain marker for predecessor commitments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PredecessorDomain;

/// One exact accepted-history integrity relation, protecting against deletion,
/// reorder, splice, foreign lineage, and unauthorized genesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImmediateHistoryPredecessor(pub Commitment<PredecessorDomain>);

/// A registered causation edge kind — an application registry row; the
/// machine owns acyclicity, the finite fan-in bound, and the commitment role
/// that makes a join verifiable; applications own which kinds exist and what
/// they mean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CausationEdgeKindId(pub u16);

/// Limit family for causation fan-in (bound value evidence-selected).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FanInLimit;
impl Limit for FanInLimit {}

/// One bounded typed multi-parent causation edge. Correlation is grouping;
/// chronology is ordering evidence; store adjacency is integrity structure;
/// deterministic display order is presentation — none is a causal edge.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CausationEdge {
    /// The application-declared edge kind.
    pub kind: CausationEdgeKindId,
    /// The parent commitments.
    pub parents: Bounded<EventCommitment, FanInLimit>,
}

// ---------------------------------------------------------------------------
// The four-object split's stage values.
// ---------------------------------------------------------------------------

/// Stage 1 — authoring: payload meaning only; no order, no cut, no
/// predecessor (authored v1 core: the schema binding).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventSemanticBody {
    /// The payload's schema binding.
    pub schema: SchemaSemanticCommitment,
}

/// Stage 2 — admission: where exact local order attaches. Never carries a
/// commit point.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AcceptedEventRecord {
    /// The body's commitment.
    pub body: EventCommitment,
    /// The exact local order, assigned at admission.
    pub sequence: AuthoritySequence,
    /// The integrity predecessor.
    pub predecessor: ImmediateHistoryPredecessor,
}

/// Limit family for publication batches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BatchLimit;
impl Limit for BatchLimit {}

/// Stage 3 — the batch crossing: which accepted records crossed the local
/// durability boundary — membership and order of the batch, not a cut. A
/// batch publishes whole or not at all.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicationRecord {
    /// The ordered member commitments.
    pub members: Bounded<EventCommitment, BatchLimit>,
}

/// Do we know it committed? The only owner of that question — an event record
/// never grows a committed field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommitKnowledge {
    /// Known not to have committed.
    KnownAbsent,
    /// Known committed.
    KnownCommitted,
    /// Genuinely unknown. Not `KnownAbsent`.
    Unknown,
}

/// Whether the operation's receipt is complete. A lost acknowledgement lowers
/// this and never rewrites commit knowledge as noncommit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReceiptCompleteness {
    /// The receipt is complete.
    Complete,
    /// The receipt is incomplete.
    Incomplete,
}

/// The history-owned reconciliation posture — `Outstanding` is a lifecycle
/// posture (owed but not yet performed), never a second "pending" grown inside
/// another axis. Effect-outcome reconciliation is a different, runtime-owned
/// family; one name never spans both owners. Reconciliation appends or
/// references new evidence and never edits the accepted event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommitReconciliation {
    /// No reconciliation is required.
    NotRequired,
    /// Reconciliation is owed but not yet performed.
    Outstanding,
    /// Reconciled: committed.
    ReconciledCommitted,
    /// Reconciled: not committed.
    ReconciledNotCommitted,
}

// ---------------------------------------------------------------------------
// Durability and the storage port.
// ---------------------------------------------------------------------------

/// The twelve claim axes of a durability profile — a typed set of explicit
/// postcondition claims, never one global adjective. Content durability is not
/// namespace durability: a staged object with durable bytes is not published,
/// and a successful replacement call is not itself proof of durable namespace
/// publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DurabilityClaimAxis {
    /// Bytes visible within the writing execution context.
    ExecutionContextVisibleBytes,
    /// Bytes visible to the host storage service.
    HostServiceVisibleBytes,
    /// Durable content.
    DurableContent,
    /// Durable extent/segment metadata.
    DurableExtentMetadata,
    /// Durable namespace entry.
    DurableNamespaceEntry,
    /// Durable replacement of an existing entry.
    DurableReplacement,
    /// Enclosing-namespace publication.
    EnclosingNamespacePublication,
    /// Cross-object ordering.
    CrossObjectOrdering,
    /// Acknowledged batch membership.
    AcknowledgedBatchMembership,
    /// Reopen behavior.
    ReopenBehavior,
    /// Crash/power-loss model.
    CrashModel,
    /// Independent evidence.
    IndependentEvidence,
}

/// Limit family for durability profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DurabilityProfileLimit;
impl Limit for DurabilityProfileLimit {}

/// One durability profile: the claims it makes. An operation requests exactly
/// one admitted profile; an adapter proves it or refuses it — no weaker
/// profile is silently substituted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DurabilityProfile {
    /// The claimed axes.
    pub claims: Bounded<DurabilityClaimAxis, DurabilityProfileLimit>,
}

/// A read-only storage root: it exposes no writer method, and the absence is
/// the point — nothing on this type mints a writer, appends a lifecycle fact,
/// advances mutable authority, or performs an ambient-route side effect.
///
/// That absence stands today and is not owed: what makes it hold is that
/// [`OpenReadOnly`] and [`OpenWritable`] are two types rather than one type with
/// a flag, so a read-only root cannot be passed where a writable one is
/// required. What IS owed is everything that would populate the pair — the open
/// road that mints either root, and the writer roster on the second — and it
/// lands with the storage adapter contract when implementation opens for this
/// home on explicit authorization. Until then neither root is inhabitable, so
/// the separation is proven by shape and exercised by nothing.
#[derive(Debug)]
pub struct OpenReadOnly {
    lineage: StoreLineageId,
}

impl OpenReadOnly {
    /// The lineage this root serves.
    #[must_use]
    pub fn lineage(&self) -> StoreLineageId {
        self.lineage
    }
}

/// A writable storage root: declared to prove live write authority, with
/// failure to establish or close it reported rather than hidden by
/// construction, and with an injected adapter as the ONLY physical route for the
/// operation using it — no ambient host call compiles into a port operation.
///
/// Every one of those claims is about a road that is owed. The port trait's
/// method roster — create/open, bounded reads/writes, append, content durability
/// handoff, namespace durability handoff, atomic publication, enumeration,
/// single-writer exclusion, crash/reload, compaction,
/// snapshot/fork/import/export/restore, lineage validation — and its
/// postcondition rows land with the storage adapter contract in host space, when
/// implementation opens for this home on explicit authorization. What this type
/// carries today is the lineage it serves and its distinctness from
/// [`OpenReadOnly`].
#[derive(Debug)]
pub struct OpenWritable {
    lineage: StoreLineageId,
}

impl OpenWritable {
    /// The lineage this root serves.
    #[must_use]
    pub fn lineage(&self) -> StoreLineageId {
        self.lineage
    }
}

// ---------------------------------------------------------------------------
// Authenticated history — three orthogonal witnesses, never a ladder.
// ---------------------------------------------------------------------------

/// The domain marker for accumulator commitments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccumulatorDomain;

/// The lineage's append-order accumulator commitment. The law: appending a
/// batch yields exactly the root sequential appends yield — batch and
/// sequential construction are one identity, never two. Only the lineage-wide
/// commitment structure can testify that two holders of one scope are the same
/// world (a scope guard verifies scope identity, not world identity).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HistoryAccumulatorRoot(pub Commitment<AccumulatorDomain>);

/// The claim marker for segment seals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SegmentSealClaim;

/// A reference to a sealed segment's seal for a bound prefix.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistorySegmentSealRef(pub EvidenceRef<SegmentSealClaim>);

/// The accumulator binding — ONE closed carrier, never two nullable fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HistoryAccumulatorBinding {
    /// The live accumulator root.
    Root(HistoryAccumulatorRoot),
    /// A sealed prefix's seal.
    SealedPrefix(HistorySegmentSealRef),
}

/// The exact history-prefix subject every witness binds.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoryPrefixBinding {
    /// The lineage.
    pub lineage: StoreLineageId,
    /// The generation.
    pub generation: AuthorityGeneration,
    /// The prefix ceiling.
    pub through: CommitPoint,
    /// The accumulator binding.
    pub accumulator: HistoryAccumulatorBinding,
}

/// The claim markers for the three witness families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConsistencyClaim;
/// Authorship witness claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthorshipClaim;
/// Freshness witness claim marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FreshnessWitnessClaim;

/// Internal-consistency witness: validates frame/segment/predecessor/
/// generation/accumulator relations. Orthogonal claims, never a ladder — a
/// signature adds authorship without adding freshness; a fresh witness proves
/// nothing about authorship; no claim implies another merely by sounding
/// stronger. A shared subject reference is lawful; a shared universal evidence
/// envelope is not.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalConsistency {
    /// The bound subject.
    pub subject: HistoryPrefixBinding,
    /// The integrity evidence.
    pub evidence: EvidenceRef<ConsistencyClaim>,
}

/// Authorship witness: an admitted signer produced the preimage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthenticatedAuthorship {
    /// The bound subject.
    pub subject: HistoryPrefixBinding,
    /// The signer evidence.
    pub evidence: EvidenceRef<AuthorshipClaim>,
}

/// External-freshness witness: an independent monotonic or anti-rollback
/// witness. Two sealed accumulator roots for one lineage position is a
/// detected equivocation, never silently reconciled; restored material first
/// enters as inspectable evidence and earns live standing only after the
/// witness comparison; where a deployment declares the witness Required, that
/// is safety-relevant configuration — witness absence refuses rather than
/// weakens.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternallyWitnessedFreshness {
    /// The bound subject.
    pub subject: HistoryPrefixBinding,
    /// The external-witness evidence.
    pub evidence: EvidenceRef<FreshnessWitnessClaim>,
}

// ---------------------------------------------------------------------------
// Reading history: the three-axis reading and authorized removal.
// ---------------------------------------------------------------------------

/// Limit family for source-region sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionLimit;
impl Limit for RegionLimit {}

/// The declared source regions a closure claim covers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceRegions {
    /// The region cuts.
    pub regions: Bounded<ScopeAppliedCut, RegionLimit>,
}

/// The typed completeness axis over declared source regions at named cuts —
/// seated HERE by band math (navigation, one band above, imports it for
/// `Fix`); one owner, with navigation a consumer.
/// An instantiation of the root's non-erasable completeness shape.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceClosure(pub Completeness<SourceRegions>);

/// The history cut marker for freshness — the first production instantiation
/// of the claim-family cut contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoryCut(pub CommitPoint);

impl EvidenceCut for HistoryCut {}

/// The claim marker for integrity disagreements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntegrityClaim;

/// Claim-specific corruption evidence: asserting corruption OWES the claim —
/// never a bare token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoryIntegrityEvidence {
    /// The disagreement evidence.
    pub evidence: EvidenceRef<IntegrityClaim>,
}

/// What the read found. Authorized semantic unavailability is never proof that
/// an event never existed: `AuthorizedlyRemoved` never collapses into
/// `HistoricalAbsence` or a generic not-found, and no reader may down-report
/// it to hide that a removal occurred.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HistoryDisposition<T> {
    /// The material is present.
    Present(T),
    /// Never admitted — the event does not exist.
    HistoricalAbsence,
    /// Authorizedly removed; the commitment proves the removal was lawful.
    AuthorizedlyRemoved(RemovalCommitment),
    /// Integrity disagreement — the claim is owed.
    Corrupt {
        /// The owed evidence.
        evidence: HistoryIntegrityEvidence,
    },
}

/// The read refusal: a single unit cause. Absence, removal, and corruption
/// resolve on the disposition; closure and freshness ride their own axes;
/// protected access resolves through the protected-resolution axis. One
/// inhabited cause, so no cause-selection rule is owed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HistoryReadRefusal {
    /// The route declines this operation.
    UnsupportedAccess,
}

impl RefusalFamily for HistoryReadRefusal {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["UnsupportedAccess"];
}

/// One history read: three orthogonal axes — the disposition never absorbs
/// source closure or freshness.
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HistoryReading<T> {
    /// What was found.
    pub disposition: HistoryDisposition<T>,
    /// The source-closure axis.
    pub closure: SourceClosure,
    /// The freshness axis.
    pub freshness: Freshness<T, HistoryCut>,
}

// ---------------------------------------------------------------------------
// Authorized removal — a distinct semantic operation: not a new generation,
// not compaction, not shred. Values whose role is authority are always
// minted, never authored.
// ---------------------------------------------------------------------------

/// Limit family for removal-plan collections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemovalPlanLimit;
impl Limit for RemovalPlanLimit {}

/// Limit family for removal text members.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemovalTextLimit;
impl Limit for RemovalTextLimit {}

/// The claim marker for removal evidence references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemovalClaim;

/// A caller-authored authorization claim: this principal, under this
/// authority — verified at admission, never authority by existing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemovalAuthorizationClaim {
    /// The asserted principal.
    pub principal: BoundedText<RemovalTextLimit>,
    /// The claimed authority.
    pub claimed_authority: BoundedText<RemovalTextLimit>,
}

/// The policy or legal basis presented FOR admission — evaluated there,
/// authority nowhere.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemovalPolicyBasis {
    /// The presented basis.
    pub basis: BoundedText<RemovalTextLimit>,
}

/// The caller-authored checked removal intent — twelve members in declared
/// order (the order IS the canonical issue order of its construction family).
/// Collections ride bounded carriers — a bare vector would contradict this
/// machine's closed-carrier law. Retained
/// commitments and removed material answer two different questions, so their
/// overlap is never a defect.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemovalPlan {
    /// The authorization claim.
    pub authorization_claim: RemovalAuthorizationClaim,
    /// The policy basis.
    pub policy_basis: RemovalPolicyBasis,
    /// The exact source cuts affected.
    pub affected_cuts: Bounded<CommitPoint, RemovalPlanLimit>,
    /// Commitments retained as existence proof.
    pub retained: Bounded<EventCommitment, RemovalPlanLimit>,
    /// The removed material set.
    pub removed_material: Bounded<EvidenceRef<RemovalClaim>, RemovalPlanLimit>,
    /// The declared visibility posture.
    pub visibility_posture: BoundedText<RemovalTextLimit>,
    /// The plan's completeness posture over the affected region.
    pub completeness_posture: BoundedText<RemovalTextLimit>,
    /// The restoration/rollback posture.
    pub restoration_posture: BoundedText<RemovalTextLimit>,
    /// The derived-materialization invalidation set — every derived row built
    /// on removed material must invalidate, never serve stale rows.
    pub invalidation_set: Bounded<EvidenceRef<RemovalClaim>, RemovalPlanLimit>,
    /// Obligations placed on external and backup participants.
    pub participant_obligations: Bounded<BoundedText<RemovalTextLimit>, RemovalPlanLimit>,
    /// The evidence the operation must produce.
    pub evidence_contract: EvidenceRef<RemovalClaim>,
    /// The bound lineage.
    pub lineage: StoreLineageId,
}

/// The plan-construction issues — one unit token per declared member, in the
/// roster's own order. Every check is a pure function over the value in hand,
/// so no lawful early stop exists; the collection answers WHICH ISSUES HOLD,
/// and no issue is elected primary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RemovalPlanConstructionIssue {
    /// The authorization claim is missing.
    AuthorizationClaimMissing,
    /// The policy basis is missing.
    PolicyBasisMissing,
    /// The affected scope is empty or invalid.
    AffectedScopeEmptyOrInvalid,
    /// A retained commitment names an event outside the affected scope.
    RetainedCommitmentOutOfAffectedScope,
    /// The removed material set is missing.
    RemovedMaterialSetMissing,
    /// The visibility posture is missing.
    VisibilityPostureMissing,
    /// The plan's completeness posture is missing — never the refusal's
    /// completion posture.
    CompletenessPostureMissing,
    /// The restoration posture is missing.
    RestorationPostureMissing,
    /// The invalidation set is incomplete.
    InvalidationSetIncomplete,
    /// Participant obligations are incomplete.
    ParticipantObligationsIncomplete,
    /// The evidence contract is missing.
    EvidenceContractMissing,
    /// The lineage does not match.
    LineageMismatch,
}

/// Compile-time bound for plan-construction issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemovalPlanIssueLimit;
impl Limit for RemovalPlanIssueLimit {}
impl crate::types::ConstLimit for RemovalPlanIssueLimit {
    const MAX: usize = 12;
}

/// Removal-plan construction: an issue collection whose posture is invariantly
/// complete (pure checks over the value in hand). Judges the shape and
/// coherence of authored intent only — never capability, policy sufficiency,
/// scope, generation, retention, or participant capability (those close at
/// admission); mints nothing; decides no removal lawfulness.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemovalPlanConstruction {
    /// The established issues.
    pub issues: crate::types::NonEmptyBounded<RemovalPlanConstructionIssue, RemovalPlanIssueLimit>,
    /// The enumeration posture — invariantly complete for this family.
    pub posture: CompletionPosture,
}

impl RefusalFamily for RemovalPlanConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// The claim-construction issues (two independent members — no intra-claim
/// contradiction is representable).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RemovalAuthorizationClaimConstructionIssue {
    /// The principal is missing.
    PrincipalMissing,
    /// The claimed authority is missing.
    ClaimedAuthorityMissing,
}

/// Compile-time bound for claim-construction issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemovalClaimIssueLimit;
impl Limit for RemovalClaimIssueLimit {}
impl crate::types::ConstLimit for RemovalClaimIssueLimit {
    const MAX: usize = 2;
}

/// Removal-authorization-claim construction: validates claim shape only —
/// never verifies the claimed authority; binds no scope, no generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemovalAuthorizationClaimConstruction {
    /// The established issues.
    pub issues: crate::types::NonEmptyBounded<
        RemovalAuthorizationClaimConstructionIssue,
        RemovalClaimIssueLimit,
    >,
    /// The enumeration posture — invariantly complete.
    pub posture: CompletionPosture,
}

impl RefusalFamily for RemovalAuthorizationClaimConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// The admission-act refusal issues — independent facts one admission can
/// establish together; under-reporting an established one would hide that a
/// removal was refused for more than the named reason. Typed classifications
/// only: none carries protected material, a participant's identity beyond its
/// typed role, or the retention policy's interior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RemovalRefusalIssue {
    /// The claimed authority was not proven.
    AuthorityUnproven,
    /// A retention obligation conflicts with the plan.
    RetentionConflict,
    /// An external participant cannot honor its obligation.
    ExternalParticipantCannotHonor,
}

/// Compile-time bound for removal-refusal issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemovalRefusalIssueLimit;
impl Limit for RemovalRefusalIssueLimit {}
impl crate::types::ConstLimit for RemovalRefusalIssueLimit {
    const MAX: usize = 3;
}

/// The removal-admission act's refusal. A refusal to remove changes no
/// accepted material.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RemovalRefusal {
    /// The established issues.
    pub issues: crate::types::NonEmptyBounded<RemovalRefusalIssue, RemovalRefusalIssueLimit>,
    /// The enumeration posture.
    pub posture: CompletionPosture,
}

impl RefusalFamily for RemovalRefusal {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

/// The domain marker for admitted removal plans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemovalPlanDomain;

/// The boundary-minted admission: authorizes exactly one admitted plan, minted
/// only after capability, policy, scope, generation, retention, and
/// external-participant obligations close.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemovalAdmission {
    /// The admitted plan's commitment.
    pub admitted_plan: Commitment<RemovalPlanDomain>,
}

/// The domain marker for removal commitments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemovalDomain;

/// The owner-minted commitment that the removal was lawful — postcondition
/// evidence binding the admitted plan and the evidence produced. Removal
/// appends its evidence; it never rewrites surviving accepted events, order,
/// causation, or the predecessor chain of events that remain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RemovalCommitment(pub Commitment<RemovalDomain>);

// ---------------------------------------------------------------------------
// The .tlog container and recovery.
// ---------------------------------------------------------------------------

/// The named body-frame kinds of the `.tlog` container (roster closure stays
/// open — the minting restriction implies more lifecycle kinds need
/// representation; flagged, not resolved).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TlogFrameKind {
    /// An accepted-event record.
    AcceptedEventRecord,
    /// A publication record.
    PublicationRecord,
    /// A commit-point receipt (group commit).
    CommitPointReceipt,
    /// A checkpoint frame.
    CheckpointFrame,
}

/// The recovery scan's declared steps, in order.
pub const RECOVERY_SCAN: [&str; 5] = [
    "locate-last-valid-commit-point-receipt",
    "classify-every-later-frame-k3",
    "discard-with-receipt-beyond-the-boundary",
    "refuse-and-hold-on-invalid-at-or-before",
    "admit-the-recovery-receipt-into-successor-history",
];

/// The recovery receipt: byte range, partial-record count, and the cut
/// recovered to — admitted into the successor history.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecoveryReceipt {
    /// The discarded byte range start.
    pub range_start: u64,
    /// The discarded byte range end.
    pub range_end: u64,
    /// The partial-record count.
    pub partial_records: u32,
    /// The cut recovered to.
    pub recovered_to: CommitPoint,
}

/// How recovery may end — no fourth ending exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryOutcome {
    /// A committed prefix stands.
    CommittedPrefix,
    /// A lawful rollback was performed.
    LawfulRollback,
    /// A typed refusal was produced.
    TypedRefusal,
}
