//! The descriptor vocabulary's declarations: what one test states about itself,
//! the references it names, what makes a row executable, the two tables that
//! hold executable rows, and the producer-facing schema every crossing is
//! pinned to.
//!
//! Declarations only. Every road that builds one of these values — the name
//! parsers, the row and binding nuclei, the table uniqueness law, the posture
//! meet, and the one schema-identity mint — lives in this file's own child,
//! `type_guard.rs`, so a value of this vocabulary is born in one place. The
//! closed tables the encoder and the admitted arms read are `type_contract.rs`;
//! the canonical bytes are `encode.rs`.
//!
//! # The identity seam
//!
//! A content-addressed reference here carries [`crate::identity::ContentAddress`]
//! as its payload and mints none of it: a proposal identity, a replay
//! reference, and a revision identity are minted by the acts that author them
//! and arrive already made. The one derivation this home performs is the
//! generated-support schema identity, over bytes this home encodes.

use crate::identity::ContentAddress;
use std::collections::BTreeSet;

#[path = "type_guard.rs"]
mod guard;

/// A namespaced name: the owner that declares a spelling, and the spelling.
///
/// Every open reference in this vocabulary is one of these under its own
/// newtype, so a name always states who declared it and two owners never
/// collide by spelling alone.
///
/// # Construction
///
/// Both parts are refused empty, so a reference that names nothing is not a
/// value anybody can hold.
///
/// The road is CHECKED, and there is deliberately no total one beside it. A
/// `const fn` that refused an empty part by panicking would refuse at COMPILE
/// TIME only where it is evaluated in a `const` context, and the roads that
/// spell names evaluate inside ordinary function bodies — a stamped row
/// expression above all — where the same call is a runtime panic, which the
/// stamped road admits nowhere. Nothing forces the evaluation into a `const`
/// context either: a `&'static str` is not a const generic argument here, so
/// there is no seat a spelling could be declared in. A total constructor is
/// therefore not a smaller road but a dishonest one, and the honest road is the
/// refusal: a name that would not parse travels as [`NameRefusal`], and the
/// roads that build values out of names discharge it — [`TrialTableRefusal`]
/// carries the discharge the stamped road stands on.
///
/// # Bounds
///
/// A name is `'static` text. This vocabulary is AUTHORED — by a hand, by a
/// stamp expansion at its invocation site, or by a depot const — rather than
/// parsed out of a run's data, so no name is ever minted from input a subject
/// under test supplied.
///
/// # Ordering
///
/// The order is the storage order sets and maps need to iterate the same way
/// every run, over the namespace and then the stem. It ranks nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamespacedName {
    namespace: &'static str,
    stem: &'static str,
}

/// Why one namespaced name was refused.
///
/// Dependent checks in a declared order — the namespace is read before the
/// stem — so exactly one cause is true of any refused name.
///
/// # Nonclaims
///
/// The cause names the first part that failed, and says nothing about the part
/// that was never read.
#[must_use = "a refusal is the reason a name was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameRefusal {
    /// The namespace is empty, so the name states no owner.
    EmptyNamespace,
    /// The stem is empty, so the name states no spelling.
    EmptyStem,
}

/// The claim one row serves — the behavior the test exists to hold.
///
/// A claim names behavior in inputs, outputs, and laws, so a lawful refactor of
/// the subject cannot break the row that serves it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClaimRef(NamespacedName);

/// The typed selection of what is under test.
///
/// A route is answered by this crate's type and no other, so reaching a new
/// mechanism is structurally a law change rather than a new string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubjectRoute(NamespacedName);

/// The check that judges the subject — which property suite or oracle lane
/// renders the verdict.
///
/// A row REFERENCES its check and never carries one. The callable arrives with
/// [`ExecutableAttachment`], so no hidden row-to-function registry can exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CheckRef(NamespacedName);

/// The generated population that supplies one row's inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PopulationRef(NamespacedName);

/// One open, namespaced classification a row carries.
///
/// A role is a label, never an execution roster: nothing selects a mechanism by
/// reading one. The initial vocabulary is convention rather than a closed set —
/// anomaly, boundary, malformed-input, regression, metamorphic, fault,
/// crash-recovery, mutation, smoke, end-to-end, performance — and a new role is
/// a label, while a new mechanism is a law change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Role(NamespacedName);

/// One open, namespaced label a row carries beside its roles.
///
/// A tag carries no vocabulary convention at all: it exists so a row can be
/// selected and reported on by a distinction nobody has named a role yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(NamespacedName);

/// The aggregate seat a row runs under by default — exactly one per row.
///
/// One suite per row is what keeps a row from running through two default
/// aggregates and being counted twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionSuite(NamespacedName);

/// The name one authored table is known by.
///
/// A staged view names its parent by this, so a report can say which world a
/// staged run was overlaid on without holding the world itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthoredTableName(NamespacedName);

/// The declaration door a generated row was authored through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DoorRef(NamespacedName);

/// The projection of a declaration that emitted one generated row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectionRef(NamespacedName);

/// One mutation point on the evaluation surface.
///
/// A candidate synthesized against a survivor names the point that survived.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutationPointRef(NamespacedName);

/// The producer that emitted a binding against a published schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProducerName(NamespacedName);

/// A proposal's content identity — permanent provenance for a row a human
/// admitted.
///
/// # Nonclaims
///
/// It is not a storage location. The review artifact a sink stored is mortal
/// and may be deleted after any ruling; this identity is what the admitted
/// origin cites, so nothing dangles when the artifact dies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProposalId(ContentAddress);

/// The depot capsule entry one admitted row replays from.
///
/// The entry it points at is authored by the admission act itself; runtime
/// evidence never writes the bank.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayRef(ContentAddress);

/// How a row's roles and tags are carried: two open, multi-valued rosters
/// parsed into sets.
///
/// # Construction
///
/// The rosters are taken as authored and a repeated label is REFUSED rather
/// than folded away, because collapsing a duplicate silently would be the
/// harness normalizing an authoring defect out of sight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classification {
    roles: BTreeSet<Role>,
    tags: BTreeSet<Tag>,
}

/// Why one classification was refused.
///
/// Dependent checks in a declared order — the roles are read before the tags —
/// so the cause names the first repeat found and the roster it was found in.
#[must_use = "a refusal is the reason a classification was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassificationRefusal {
    /// The role roster states this role more than once.
    DuplicateRole(Role),
    /// The tag roster states this tag more than once.
    DuplicateTag(Tag),
}

/// What a producer's own act contributed to one generated row.
///
/// The door and the projection are the producer-side identity facts a row
/// earns. The producer's NAME and the generated-support schema identity it
/// emitted against are not here: they ride [`Provenance`] on the binding, so a
/// hand-written row never touches a schema identity and a row's meaning never
/// churns when a producer-facing schema changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProducerFacts {
    door: DoorRef,
    projection: ProjectionRef,
}

/// What a candidate row was synthesized to serve.
///
/// # Nonclaims
///
/// A synthesis fact states the opening the candidate was cut for, never that
/// the candidate closes it. Whether it does is executed evidence, and the
/// executing happens in a staged view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthesisFacts {
    /// A mutation point survived, and this candidate was cut to kill it.
    Survivor(MutationPointRef),
    /// Claim coverage read a claim with no proof behind it, and this candidate
    /// was cut to close that gap. The claim is the row's own claim; it is not
    /// restated here.
    ProofGap,
}

/// The structural ground a human's admission stood on.
///
/// One of exactly three, per the proposal road: a proposal earns admission by
/// killing a real mutant, by pinning a named claim, or by discharging a claim
/// declared owed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdmissionGround {
    /// The proposal killed a real mutant.
    MutantKilled,
    /// The proposal pinned a named claim.
    ClaimPinned,
    /// The proposal discharged a claim declared owed.
    ObligationDischarged,
}

/// Whether a ground brings a replay capsule with it.
///
/// The two answers are the reason the admitted origin has two arms: a
/// replay-bearing ground authors a depot capsule entry the row then points at,
/// and a discharge authors none, because the admitted row IS the discharge's
/// permanent record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapsulePosture {
    /// The admission act authors a capsule entry, and the row references it.
    ReplayBearing,
    /// The admission act authors no capsule entry at all.
    NoCapsule,
}

/// What one admission act stated about itself.
///
/// # Authority
///
/// The ground summary and the destination suite are the whole of it. There is
/// no admitter seat and no timestamp seat: who admitted is not identity-bearing
/// here, and a clock is an ambient fact this vocabulary refuses to carry.
///
/// # Nonclaims
///
/// The ground is the admission's STATED ground, at summary width. The typed
/// ground with its evidence is the proposal's, and the proposal is cited by
/// identity rather than copied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissionFacts {
    ground: AdmissionGround,
    destination: ExecutionSuite,
}

/// Where one row came from, with each arm carrying exactly what it earns.
///
/// # The arms and their ceilings
///
/// `HandWritten` claims a hand wrote the row and nothing else; it is a lawful
/// producer, not a lesser one.
///
/// `Generated` claims a producer emitted the row through the named door and
/// projection. It claims nothing about which schema the producer emitted
/// against — that pin rides [`Provenance`].
///
/// `Candidate` claims a synthesis cut the row for the named opening. It is
/// lawful only in a staged view: [`AuthoredTable`] refuses this arm outright,
/// so a candidate cannot become authored by any road except a human's
/// admission.
///
/// `AdmittedReplay` claims a human admitted the cited proposal on a
/// replay-bearing ground, and the replay reference points at the depot capsule
/// entry that admission act authored.
///
/// `AdmittedDischarge` claims a human admitted the cited proposal on a
/// discharge ground, which authors no capsule at all — so there is no replay
/// seat to leave empty.
///
/// # Construction
///
/// The two admitted arms and the ground stated in their admission facts must
/// agree, and [`Row::declared`] is where that agreement is checked: the arms
/// are open shapes, so the coherence law is enforced where a row is born.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// A hand wrote this row.
    HandWritten,
    /// A producer emitted this row.
    Generated(ProducerFacts),
    /// A synthesis cut this row for an opening; lawful in a staged view only.
    Candidate(SynthesisFacts),
    /// A human admitted this row on a replay-bearing ground.
    AdmittedReplay {
        /// The admitted proposal's content identity.
        proposal: ProposalId,
        /// What the admission stated.
        admission: AdmissionFacts,
        /// The depot capsule entry the admission act authored.
        replay: ReplayRef,
    },
    /// A human admitted this row on a discharge ground.
    AdmittedDischarge {
        /// The admitted proposal's content identity.
        proposal: ProposalId,
        /// What the admission stated.
        admission: AdmissionFacts,
    },
}

/// One row of the harness's denominator: one test, stated as data.
///
/// The fields are closed — the claim served, the one execution suite, the open
/// classification, the subject route, the check reference, the population, and
/// the origin. A schema identity is not among them.
///
/// # Authority
///
/// A row is pure data and cannot execute. It names its check rather than
/// carrying one, and the callable arrives only with [`Binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    claim: ClaimRef,
    execution_suite: ExecutionSuite,
    classification: Classification,
    subject: SubjectRoute,
    check: CheckRef,
    population: PopulationRef,
    origin: Origin,
}

/// Why one row was refused.
///
/// # Nonclaims
///
/// This family is narrow on purpose: an empty claim, an empty suite name, and a
/// repeated role are refused upstream, by the name parsers and by
/// [`Classification`], so a row constructor that received typed values has one
/// thing left to establish.
#[must_use = "a refusal is the reason a row was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowRefusal {
    /// The origin's admitted arm and the ground its admission facts state
    /// disagree: a replay-bearing arm under a discharge ground, or a discharge
    /// arm under a ground that authors a capsule.
    AdmissionGroundMismatch(AdmissionGround),
}

/// The semantic content two rows would have to share to be one trial: the
/// claim, the subject, the check, and the population.
///
/// # Nonclaims
///
/// This is NOT the trial identity. The report instrument derives `TrialId` from
/// semantic meaning — the claim, the subject, the mechanism the check reference
/// names, the population, and the profile — and this home cannot compute it
/// without importing that vocabulary. This key is the descriptor-side shadow of
/// it: the structural fact a table constructor can establish on its own, so a
/// duplicated trial cannot exist in a constructed table. The execution suite is
/// deliberately outside the key, because two rows differing only by suite are
/// one trial run under two seats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrialKey {
    claim: ClaimRef,
    subject: SubjectRoute,
    check: CheckRef,
    population: PopulationRef,
}

/// What a revision identity is worth, stated by the party that bound it.
///
/// # The ceilings
///
/// `Derived` was generated from an owned declaration: the identity moves when
/// the declaration moves, and nobody had to remember to update it.
///
/// `Declared` is a hand author's explicit commitment. The ceiling is the
/// author's word — the identity moves when the author says it moved.
///
/// `Untracked` is no stable commitment at all, and it is lawful. It claims
/// nothing about whether the thing it names has changed.
///
/// # The order
///
/// Derived is stronger than Declared, and Declared is stronger than Untracked.
/// The meet of two postures is the weaker of the pair — see
/// [`RevisionPosture::meet`]. What a posture means for the cache and for replay
/// is the report instrument's one statement ([`crate::report`]), and it is not
/// restated here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RevisionPosture {
    /// Generated from an owned declaration.
    Derived,
    /// A hand author's explicit commitment.
    Declared,
    /// No stable commitment.
    Untracked,
}

/// One revision identity and the posture it is held under.
///
/// # Nonclaims
///
/// Under [`RevisionPosture::Untracked`] the payload identifies the revision
/// value that was recorded and carries no claim that it moves when the thing it
/// names moves. Reading an untracked binding as a commitment reads a claim
/// nobody made.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevisionBinding {
    revision: ContentAddress,
    posture: RevisionPosture,
}

/// What makes one row executable: the typed subject and check references, a
/// posture-bearing revision binding for each, and the callable.
///
/// # The callable
///
/// The callable is a pure map — invocation facts in, one conclusion out — and
/// it is a function pointer rather than a closure, so it carries no captured
/// state and nothing ambient can ride in with it.
///
/// # The generic seam
///
/// The invocation facts and the conclusion belong to the report instrument's
/// vocabulary ([`crate::report`]), and this home sits below it: a descriptor
/// value may never import a record type. So both ends of the callable are type
/// parameters, and the runner — which sees both vocabularies — is what
/// instantiates them.
///
/// # Bounds
///
/// The parameters carry no bounds, deliberately. This home cannot state a
/// contract over vocabulary it does not own, and a bound invented here would be
/// a second authority over a type another home declares.
///
/// # Nonclaims
///
/// The two revision bindings say what their postures say and no more. The
/// weaker of the pair governs — [`ExecutableAttachment::posture`] is the meet —
/// so a mixed attachment never inherits the stronger half's ceiling.
pub struct ExecutableAttachment<Invocation, Conclusion> {
    subject: SubjectRoute,
    check: CheckRef,
    subject_revision: RevisionBinding,
    check_revision: RevisionBinding,
    call: fn(&Invocation) -> Conclusion,
}

/// Whether a producer stands behind one binding or one table, and which schema
/// it emitted against.
///
/// This is where the generated-support schema identity rides — on the binding
/// and on the table a producer emitted. It is not a row field, so hand-written
/// rows never touch it and row identity never churns when a producer-facing
/// schema changes. Each seat states its own emitter: a produced table holding a
/// hand-written row is an ordinary thing, and neither seat speaks for the other.
///
/// # Nonclaims
///
/// A pin records which schema identity a producer emitted against. It is not
/// evidence that the pin is current: a jointly stale pair — a schema that moved
/// while publication never ran — agrees with itself and says nothing. Pair
/// currency is the conformance trial's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provenance {
    /// No producer stands behind this binding.
    Unproduced,
    /// A producer emitted this binding against a published schema.
    Produced {
        /// The producer that emitted it.
        producer: ProducerName,
        /// The generated-support schema identity it emitted against.
        schema: GeneratedSupportSchemaId,
    },
}

/// One row married to one executable attachment.
///
/// # Construction
///
/// The constructor structurally verifies that the row's subject route and check
/// reference are the attachment's, so a binding cannot pair a row with a
/// callable that judges something else.
pub struct Binding<Invocation, Conclusion> {
    row: Row,
    attachment: ExecutableAttachment<Invocation, Conclusion>,
    provenance: Provenance,
}

/// Why one binding was refused.
///
/// Dependent checks in a declared order: the subject, then the check, then the
/// provenance the row's origin demands.
#[must_use = "a refusal is the reason a binding was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingRefusal {
    /// The row's subject route and the attachment's are different routes.
    SubjectMismatch {
        /// What the row states.
        row: SubjectRoute,
        /// What the attachment states.
        attachment: SubjectRoute,
    },
    /// The row's check reference and the attachment's are different checks.
    CheckMismatch {
        /// What the row states.
        row: CheckRef,
        /// What the attachment states.
        attachment: CheckRef,
    },
    /// The row carries producer facts, so a producer emitted it, but the
    /// binding names no schema the producer emitted against.
    GeneratedWithoutSchemaPin,
}

/// The complete authored world: every binding a run's denominator is stated
/// over.
///
/// # Construction
///
/// Two structural refusals stand at the door. The candidate origin arm is
/// refused outright, so a candidate becomes authored only through a human's
/// admission. And two rows with one trial identity are refused, so a
/// denominator can never read two where one thing is measured.
///
/// # Authority
///
/// One authored world, ever. A selection narrows a run; it never narrows this.
pub struct AuthoredTable<Invocation, Conclusion> {
    name: AuthoredTableName,
    provenance: Provenance,
    bindings: Vec<Binding<Invocation, Conclusion>>,
}

/// Why one authored table was refused.
///
/// Each cause names the offending row by its trial key, so a caller reads which
/// row rather than which position.
#[must_use = "a refusal is the reason a table was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoredTableRefusal {
    /// A binding carries the candidate origin arm, which is lawful in a staged
    /// view and never in the authored world.
    CandidateOrigin(TrialKey),
    /// Two bindings state one trial.
    DuplicateTrial(TrialKey),
}

/// A complete authored world with candidate bindings overlaid on it, for
/// proving a candidate against the world it would join.
///
/// # Construction
///
/// The parent is borrowed rather than copied, so nothing here can grow into a
/// second authored world. Every overlaid binding must carry the candidate
/// origin arm, and trial uniqueness is enforced across parent and overlay
/// together.
///
/// # Nonclaims
///
/// A staged run is not an authored run. Claim coverage admits authored-posture
/// reports only; the posture this view presents is what a report carries so
/// that refusal, rather than declaration, is what keeps a staged run out.
pub struct StagedTableView<'parent, Invocation, Conclusion> {
    parent: &'parent AuthoredTable<Invocation, Conclusion>,
    candidates: Vec<Binding<Invocation, Conclusion>>,
}

/// Why one staged view was refused.
#[must_use = "a refusal is the reason a staged view was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedTableRefusal {
    /// An overlaid binding does not carry the candidate origin arm, so it is an
    /// authored row entering by the staging door.
    NotACandidate(TrialKey),
    /// A candidate states a trial the parent or another candidate already
    /// states.
    DuplicateTrial(TrialKey),
}

/// Which world a view presents, and — when it is staged — the authored parent
/// it was overlaid on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TablePosture {
    /// The authored world itself.
    Authored,
    /// An authored world with candidates overlaid.
    Staged {
        /// The authored parent the candidates were overlaid on.
        parent: AuthoredTableName,
    },
}

/// The one read surface an authored table and a staged view both present.
///
/// # The sealed shape
///
/// This is an enum rather than a trait, and that is the sealing: the arms are
/// the two worlds this crate declares, and no outside crate can add a third by
/// implementing anything. "One authored world, ever" is therefore structural
/// rather than a rule somebody follows.
pub enum TableView<'view, Invocation, Conclusion> {
    /// The authored world.
    Authored(&'view AuthoredTable<Invocation, Conclusion>),
    /// An authored world with candidates overlaid.
    Staged(&'view StagedTableView<'view, Invocation, Conclusion>),
}

/// How many values one schema field carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldCardinality {
    /// Exactly one value, always.
    ExactlyOne,
    /// One value or none.
    ZeroOrOne,
    /// A roster of any size, including empty.
    ZeroOrMore,
}

/// What shape one schema field's values take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldShape {
    /// A namespaced name: an owner and a spelling.
    NamespacedName,
    /// A thirty-two byte content address.
    ContentAddress,
    /// One arm of a closed roster, named by the arm spellings the field admits,
    /// in declared order.
    ClosedChoice(&'static [&'static str]),
    /// An unbounded byte string.
    Bytes,
    /// A count.
    Count,
}

/// One field of one producer-facing vocabulary, as the schema declares it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaField {
    name: &'static str,
    shape: FieldShape,
    cardinality: FieldCardinality,
}

/// The descriptor vocabulary's canonical field roster, as the schema declares
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorSchema {
    fields: &'static [SchemaField],
}

/// The mutation-point vocabulary's canonical field roster, as the schema
/// declares it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MutationPointSchema {
    fields: &'static [SchemaField],
}

/// The bench-row vocabulary's canonical field roster, as the schema declares
/// it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchSchema {
    fields: &'static [SchemaField],
}

/// Why one schema member was refused.
///
/// Dependent checks in a declared order: an empty roster is read first, then
/// each field's name, then whether a name repeats.
#[must_use = "a refusal is the reason a schema member was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaRefusal {
    /// The roster is empty, so the member declares no vocabulary at all.
    EmptyRoster,
    /// A field states an empty name.
    EmptyFieldName,
    /// Two fields state one name.
    DuplicateFieldName(&'static str),
}

/// The root declaration every producer-facing vocabulary is pinned through: the
/// descriptor, mutation-point, and bench field rosters, together.
///
/// # Authority
///
/// One declaration governs all three crossings. A change to ANY member moves
/// the derived identity, so one pin is mechanically enough.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedSupportSchema {
    descriptor: DescriptorSchema,
    mutation_point: MutationPointSchema,
    bench: BenchSchema,
}

/// The identity derived from one root schema declaration's canonical bytes.
///
/// # Authority
///
/// The bytes are the PREIMAGE and this is DERIVED from them. It is never
/// hand-bumped, never a hash of source text, and the bytes are never "the id".
///
/// # Nonclaims
///
/// Two of these being equal says the two declarations encode the same rosters.
/// It says nothing about whether either side is current: a pair that agrees
/// because publication never ran is exactly what this comparison cannot see.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedSupportSchemaId(ContentAddress);

/// Why one schema encoding was refused.
///
/// # Nonclaims
///
/// The single cause is unreachable on every target this crate is built for. It
/// exists because the encoder states its widths and never guesses at one.
#[must_use = "a refusal is the reason canonical bytes were not produced"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeRefusal {
    /// A length does not fit the sixty-four bit width the encoding declares.
    LengthPastEncodingWidth,
}

/// Why one stamped trial table was not built.
///
/// # Authority
///
/// This is the complete refusal family of the road
/// [`trial_table!`](crate::trial_table) expands, and it is the one family a
/// declared ROW EXPRESSION refuses in. Every constructor the stamped module
/// calls, and every construction a row expression performs on its own way to a
/// binding, answers with one of these causes — so a seat that could not be built
/// states WHICH construction refused rather than a single flattened failure.
/// Each arm carries the owning constructor's own typed refusal unchanged —
/// nothing is re-spelled here, so there is no second vocabulary for a cause that
/// already has one.
///
/// # Construction
///
/// Every family a row expression's own `?` can raise has exactly one lawful
/// discharge into this one, declared once as a [`From`] realization. That is
/// what makes the `?` a generated expression writes total, and it is why a
/// producer's expression never names an arm of this enum: the expression builds
/// its parts through this home's public constructors, each of them refuses in a
/// family this one already admits, and the language's own conversion carries the
/// refusal across. An arm named in a rendering would be a producer legislating
/// inside a vocabulary it does not own; the arms are named only by the roads
/// this home writes for itself, where the target type is stated on the same
/// line.
///
/// The authored-table refusal has no such road, and its absence is the same law
/// read the other way: the only construction that raises it is the stamp's own
/// final one, which stands in tail position and names the arm where it stands,
/// so a conversion beside it would be that mapping written twice.
///
/// # Nonclaims
///
/// It says nothing about what a trial CONCLUDED. A table that was never built
/// ran nothing, and a run's verdict is the report instrument's
/// ([`crate::report`]) to state.
#[must_use = "a refusal is the reason a trial table was not stamped"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialTableRefusal {
    /// A name the road declares — the table's, a suite's, a producer's, or one
    /// a row expression spells for itself — was refused by the name parser.
    NameNotParsed(NameRefusal),
    /// A row's two open rosters were refused by the classification
    /// constructor: a role or a tag was stated twice.
    ClassificationNotAuthored(ClassificationRefusal),
    /// A row's own constructor refused the values it was given.
    RowNotDeclared(RowRefusal),
    /// The root schema declaration a produced table or a produced row pins
    /// against was refused by the roster law.
    SchemaNotDeclared(SchemaRefusal),
    /// The root schema declaration's canonical bytes were refused, so no
    /// identity could be derived to pin against.
    SchemaNotEncoded(EncodeRefusal),
    /// A row's binding constructor refused the row and attachment it was given.
    BindingNotBound(BindingRefusal),
    /// The authored world refused the bindings it was given.
    TableNotAuthored(AuthoredTableRefusal),
}

/// The descriptor vocabulary's canonical field roster: the closed field set one
/// row states, in reading order.
///
/// # Authority
///
/// This roster is the descriptor member's preimage material, and it is the same
/// field set [`Row`] declares. The two are held together by the conformance
/// trial, which derives the current schema's identity and checks the published
/// literal against it — a roster that drifts from the row is what that trial
/// exists to catch.
pub const DESCRIPTOR_FIELDS: &[SchemaField] = &[
    SchemaField::declared("claim", FieldShape::NamespacedName, FieldCardinality::ExactlyOne),
    SchemaField::declared(
        "execution_suite",
        FieldShape::NamespacedName,
        FieldCardinality::ExactlyOne,
    ),
    SchemaField::declared("roles", FieldShape::NamespacedName, FieldCardinality::ZeroOrMore),
    SchemaField::declared("tags", FieldShape::NamespacedName, FieldCardinality::ZeroOrMore),
    SchemaField::declared("subject", FieldShape::NamespacedName, FieldCardinality::ExactlyOne),
    SchemaField::declared("check", FieldShape::NamespacedName, FieldCardinality::ExactlyOne),
    SchemaField::declared("population", FieldShape::NamespacedName, FieldCardinality::ExactlyOne),
    SchemaField::declared(
        "origin",
        FieldShape::ClosedChoice(&[
            "hand-written",
            "generated",
            "candidate",
            "admitted-replay",
            "admitted-discharge",
        ]),
        FieldCardinality::ExactlyOne,
    ),
];

/// The mutation-point vocabulary's canonical field roster: what a producer
/// states about one point on an evaluation surface.
///
/// # Authority
///
/// This roster is the mutation-point member's preimage material and the whole
/// of what the second crossing admits. The RUNTIME types that carry these
/// values belong to the lane that owns them ([`crate::muterprater`]); what is
/// declared here is the producer-facing VOCABULARY, so a producer emits against
/// this roster rather than against another crate's shape.
///
/// # The fields
///
/// The identity names the point itself, and the owner claim is what makes a
/// survivor explainable: a point that survived leads to the claim that owns the
/// behaviour, and from there to the check reference that would close it.
///
/// The original operation is the unmutated reading — the no-mutation mutant
/// every evaluation surface contains — carried as the declaration's own
/// rendered bytes rather than as a name, because two different operations a
/// producer happened to name alike would otherwise encode identically. The
/// admitted alternatives are the damages this point may be selected into, a
/// roster because a point admitting none is a lawful point that only ever reads
/// as no-mutation.
///
/// The activation site is where a selected alternative fires. It is NAMED
/// rather than path-spelled, for the reason a trial's identity is not its site:
/// a file move must rename nothing.
///
/// # Nonclaims
///
/// A roster of admitted alternatives states which damages the point ADMITS, and
/// never that any of them was materialized, activated, or killed. Those are
/// executed facts and they live in the mutation lane's own record.
pub const MUTATION_POINT_FIELDS: &[SchemaField] = &[
    SchemaField::declared("identity", FieldShape::NamespacedName, FieldCardinality::ExactlyOne),
    SchemaField::declared("owner_claim", FieldShape::NamespacedName, FieldCardinality::ExactlyOne),
    SchemaField::declared("original_operation", FieldShape::Bytes, FieldCardinality::ExactlyOne),
    SchemaField::declared(
        "admitted_alternatives",
        FieldShape::Bytes,
        FieldCardinality::ZeroOrMore,
    ),
    SchemaField::declared(
        "activation_site",
        FieldShape::NamespacedName,
        FieldCardinality::ExactlyOne,
    ),
];

/// The bench-row vocabulary's canonical field roster: what a producer states
/// about one measured workload.
///
/// # Authority
///
/// This roster is the bench member's preimage material and the whole of what
/// the third crossing admits. The bench seat's own contract is where these
/// fields are argued; this is the vocabulary they cross in.
///
/// # The fields
///
/// The workload identity names what is measured. The input-size axis is the
/// declared roster of sizes it is measured across, because a growth class is
/// read off a curve and never off one point.
///
/// The correctness preflight and the planted-worse falsifier are the two gates
/// the host order runs before any backend is invoked: a failing operation is
/// never benchmarked, and a measurement that cannot separate a deliberately
/// worse implementation from the real one has not been shown to measure
/// anything. Both are REFERENCES — the callables that stand behind them ride
/// the bench binding, exactly as a descriptor row references its check rather
/// than carrying one.
///
/// The declared budgets are the gate's own tolerances — sample counts, warmup,
/// the ratio threshold — declared beside the row so a threshold is spec rather
/// than a number somebody tuned. The contention posture is a closed pair,
/// stated always: a measurement taken with the host to itself and a measurement
/// taken with declared competing work present are different measurements, and
/// an undeclared posture is inadmissible — which is why there is no third arm
/// standing for "unstated".
///
/// The work formula is optional because only some operations declare one; where
/// one is declared, the gate reads WORK COUNTS against it and wall time is the
/// secondary human observation. The complexity claim is a neutral reference: a
/// standalone public vocabulary never names a product type, so a machine maps
/// its own complexity contract into this seat from the product side.
///
/// # Nonclaims
///
/// A roster of declared budgets carries counts and states nothing about what a
/// run spent. Adding an arm to the contention posture moves the derived schema
/// identity — which is the mechanism, not an accident of it.
pub const BENCH_FIELDS: &[SchemaField] = &[
    SchemaField::declared(
        "workload_identity",
        FieldShape::NamespacedName,
        FieldCardinality::ExactlyOne,
    ),
    SchemaField::declared("input_size_axis", FieldShape::Count, FieldCardinality::ZeroOrMore),
    SchemaField::declared(
        "correctness_preflight",
        FieldShape::NamespacedName,
        FieldCardinality::ExactlyOne,
    ),
    SchemaField::declared(
        "planted_worse_falsifier",
        FieldShape::NamespacedName,
        FieldCardinality::ExactlyOne,
    ),
    SchemaField::declared("declared_budgets", FieldShape::Count, FieldCardinality::ZeroOrMore),
    SchemaField::declared(
        "contention_posture",
        FieldShape::ClosedChoice(&["uncontended", "contended"]),
        FieldCardinality::ExactlyOne,
    ),
    SchemaField::declared("work_formula", FieldShape::Bytes, FieldCardinality::ZeroOrOne),
    SchemaField::declared(
        "complexity_claim",
        FieldShape::NamespacedName,
        FieldCardinality::ExactlyOne,
    ),
];
