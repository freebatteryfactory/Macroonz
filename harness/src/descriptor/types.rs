//! What one trial states about itself, what makes it executable, the two tables that hold executable rows, and the producer-facing schema every crossing is pinned to.
//!
//! Declarations only.
//! Every road that builds one of these values lives in `type_guard.rs`, declared at the foot of this file as its own child so that it sees private fields.
//! Canonical bytes live in `encode.rs`; trait realizations live in `type_contract.rs`.
//!
//! A content-addressed reference here carries a [`ContentAddress`] and mints none of it: a proposal identity, a replay reference, and a revision identity arrive already made.
//! The one derivation this home performs is the generated-support schema identity, over bytes this home encodes.

use crate::identity::ContentAddress;
use std::collections::BTreeSet;

/// A namespaced name: the owner that declares a spelling, and the spelling.
///
/// Every open reference in this vocabulary is one of these under its own newtype, so a name always states who declared it and two owners never collide by spelling alone.
/// Both parts are refused empty, and the road is checked rather than total because names are spelled inside ordinary function bodies, where a panicking `const fn` would be a runtime panic rather than a refusal.
///
/// The order is storage order, over the namespace and then the stem; it ranks nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamespacedName {
    namespace: Namespace,
    stem: Stem,
}

/// The owner half of a namespaced name: who declares a spelling.
///
/// Its own type rather than a string, so a road that wants an owner cannot be handed a spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Namespace(&'static str);

/// The spelling half of a namespaced name: what the owner calls it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stem(&'static str);

/// Why one namespaced name was refused.
///
/// The namespace is read before the stem, so exactly one cause is true of any refused name and it names the part that failed.
#[must_use = "a refusal is the reason a name was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameRefusal {
    /// The namespace is empty, so the name states no owner.
    EmptyNamespace,
    /// The stem is empty, so the name states no spelling.
    EmptyStem,
}

/// The claim one row serves — the behavior the test exists to hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClaimRef(NamespacedName);

/// The typed selection of what is under test.
///
/// A route is answered by this crate's type and no other, so reaching a new mechanism is a law change rather than a new string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubjectRoute(NamespacedName);

/// The check that judges the subject — which property suite or oracle lane renders the verdict.
///
/// A row references its check and never carries one; the callable arrives with an [`ExecutableAttachment`], so no hidden row-to-function registry can exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CheckRef(NamespacedName);

/// The generated population that supplies one row's inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PopulationRef(NamespacedName);

/// One open, namespaced classification a row carries.
///
/// A role is a label, never an execution roster: nothing selects a mechanism by reading one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Role(NamespacedName);

/// One open, namespaced label a row carries beside its roles.
///
/// A tag carries no vocabulary convention at all: it exists so a row can be selected on a distinction nobody has named a role yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag(NamespacedName);

/// The aggregate seat a row runs under by default — exactly one per row.
///
/// One suite per row is what keeps a row from running through two default aggregates and being counted twice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionSuite(NamespacedName);

/// The name one authored table is known by.
///
/// A staged view names its parent by this, so a report can say which world it was overlaid on without holding the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthoredTableName(NamespacedName);

/// The declaration door a generated row was authored through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DoorRef(NamespacedName);

/// The projection of a declaration that emitted one generated row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectionRef(NamespacedName);

/// One mutation point on the evaluation surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutationPointRef(NamespacedName);

/// The producer that emitted a binding against a published schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProducerName(NamespacedName);

/// A proposal's content identity — permanent provenance for a row a human admitted.
///
/// Not a storage location: the review artifact a sink stored is mortal and may be deleted after any ruling, so the admitted origin cites this identity and nothing dangles when the artifact dies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProposalId(ContentAddress);

/// The depot capsule entry one admitted row replays from.
///
/// The entry it points at is authored by the admission act itself; runtime evidence never writes the bank.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayRef(ContentAddress);

/// How a row's roles and tags are carried: two open, multi-valued rosters parsed into sets.
///
/// A repeated label is refused rather than folded away, because collapsing a duplicate silently would normalize an authoring defect out of sight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classification {
    roles: BTreeSet<Role>,
    tags: BTreeSet<Tag>,
}

/// Why one classification was refused.
///
/// The roles are read before the tags, so the cause names the first repeat found and the roster it was found in.
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
/// The producer's name and the schema identity it emitted against are not here: they ride [`Provenance`] on the binding, so a hand-written row never touches a schema identity and a row's meaning never churns when a producer-facing schema changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProducerFacts {
    door: DoorRef,
    projection: ProjectionRef,
}

/// What a candidate row was synthesized to serve.
///
/// It states the opening the candidate was cut for, never that the candidate closes it; whether it does is executed evidence, and the executing happens in a staged view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthesisFacts {
    /// A mutation point survived, and this candidate was cut to kill it.
    Survivor(MutationPointRef),
    /// Claim coverage read a claim with no proof behind it, and this candidate was cut to close that gap.
    ProofGap,
}

/// The structural ground a human's admission stood on.
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
/// The two answers are why the admitted origin has two arms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapsulePosture {
    /// The admission act authors a capsule entry, and the row references it.
    ReplayBearing,
    /// The admission act authors no capsule entry at all.
    NoCapsule,
}

/// The grounds a replay-bearing admission stands on.
///
/// Exactly the grounds whose [`AdmissionGround::capsule_posture`] is [`CapsulePosture::ReplayBearing`]: a ground that authors no capsule has no spelling here, so a mismatched pair is not a value anybody can write.
/// It is not a second ground vocabulary — [`AdmissionGround`] is what these widen back to, once, in `type_contract.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplayBearingGround {
    /// The proposal killed a real mutant.
    MutantKilled,
    /// The proposal pinned a named claim.
    ClaimPinned,
}

/// What one admission act stated about itself, at summary width.
///
/// A reading rather than a seat an origin holds: each admitted arm answers with one, so every admitted row states its admission at one width while the arms keep their own shapes.
/// There is no admitter seat and no timestamp seat, because a clock is an ambient fact this vocabulary refuses to carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmissionFacts {
    ground: AdmissionGround,
    destination: ExecutionSuite,
}

/// What a human's admission on a replay-bearing ground earned one row.
///
/// The ground seat is [`ReplayBearingGround`] rather than the wide vocabulary, so a capsule-bearing arm carrying a discharge ground is unrepresentable rather than refused.
/// The replay reference is unconditional for the same reason: the ground that opens this arm is exactly a ground that authored a capsule entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayAdmission {
    proposal: ProposalId,
    ground: ReplayBearingGround,
    destination: ExecutionSuite,
    replay: ReplayRef,
}

/// What a human's admission on a discharge ground earned one row.
///
/// A discharge stands on exactly one ground, so the ground is forced and the constructor does not ask for it.
/// There is no replay seat at all: the admitted row is the discharge's permanent record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DischargeAdmission {
    proposal: ProposalId,
    destination: ExecutionSuite,
}

/// Where one row came from, with each arm carrying exactly what it earns.
///
/// Every arm carries its own payload type with exactly one lawful constructor, so an incoherent origin — a replay-bearing arm under a discharge ground, a discharge arm under a ground that authors a capsule — is not a value that can be written.
/// The hand-written arm's payload is the arm itself, because there is nothing a hand earns beyond having written the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// A hand wrote this row.
    HandWritten,
    /// A producer emitted this row through the named door and projection.
    Generated(ProducerFacts),
    /// A synthesis cut this row for an opening; lawful in a staged view only.
    Candidate(SynthesisFacts),
    /// A human admitted this row on a replay-bearing ground.
    AdmittedReplay(ReplayAdmission),
    /// A human admitted this row on a discharge ground.
    AdmittedDischarge(DischargeAdmission),
}

/// Where one trial sits: the claim it serves, the subject it exercises, the check that judges it, and the population that supplies its inputs.
///
/// The readable account, each coordinate still its own envelope, with the execution suite deliberately outside because two rows differing only by suite are one trial run under two seats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrialCoordinates {
    claim: ClaimRef,
    subject: SubjectRoute,
    check: CheckRef,
    population: PopulationRef,
}

/// The compact identity of one trial's coordinates.
///
/// The comparable account, and the only one that travels: a table compares these thirty-two bytes to decide whether two bindings state one trial.
/// It carries none of the four coordinates and has no road back to them — a caller that wants them reads them off the row that holds both, and a reverse lookup would be the hidden registry this vocabulary refuses everywhere else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrialKey(ContentAddress);

/// The canonical byte string one row commits to.
///
/// Written once, where the row is born, and carried for the row's whole life; it is a preimage and never an identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalRowBytes(Vec<u8>);

/// One row of the harness's denominator: one test, stated as data.
///
/// A row is pure data and cannot execute: it names its check rather than carrying one, and the callable arrives only with a [`Binding`].
///
/// Beside the fields it declares, a row owns three readings computed at construction — its [`CanonicalRowBytes`], its [`TrialCoordinates`], and the [`TrialKey`] derived over them.
/// None of them is a declared field, which is why the producer-facing roster does not name them and why a revision identity is a reading rather than a recomputation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    coordinates: TrialCoordinates,
    trial_key: TrialKey,
    execution_suite: ExecutionSuite,
    classification: Classification,
    origin: Origin,
    canonical: CanonicalRowBytes,
}

/// Why one row was refused.
///
/// The family is narrow because everything else was spent upstream: an empty name is refused by the name parsers, a repeated label by [`Classification`], and an origin whose arm and ground disagree never reaches a constructor.
#[must_use = "a refusal is the reason a row was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowRefusal {
    /// The row's canonical bytes could not be written, so the row has no preimage to commit to.
    NotEncoded(EncodeRefusal),
}

/// What a revision identity is worth, stated by the party that bound it.
///
/// Derived outranks Declared, and Declared outranks Untracked; the meet of two postures is the weaker of the pair.
/// What a posture buys the cache and lets a reproduction claim is the record home's statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RevisionPosture {
    /// Generated from an owned declaration, so the identity moves when the declaration moves.
    Derived,
    /// A hand author's explicit commitment, whose ceiling is the author's word.
    Declared,
    /// No stable commitment at all, and lawful.
    Untracked,
}

/// One revision identity and the posture it is held under.
///
/// Under [`RevisionPosture::Untracked`] the payload identifies the revision that was recorded and carries no claim that it moves when the thing it names moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevisionBinding {
    revision: ContentAddress,
    posture: RevisionPosture,
}

/// What makes one row executable: the typed subject and check references, a posture-bearing revision binding for each, and the callable.
///
/// The callable is a function pointer — invocation facts in, one conclusion out — so it carries no captured state.
/// A function pointer may still read process-global state or perform effects; this type establishes the callable shape, not semantic purity.
///
/// Both ends are type parameters, carrying no bounds, because they belong to the record home's vocabulary and a bound invented here would be a second authority over a type another home declares.
/// The runner instantiates them, because it is what sees both vocabularies.
///
/// The weaker of the two revision postures governs, so a mixed attachment never inherits the stronger half's ceiling.
pub struct ExecutableAttachment<Invocation, Conclusion> {
    subject: SubjectRoute,
    check: CheckRef,
    subject_revision: RevisionBinding,
    check_revision: RevisionBinding,
    call: fn(&Invocation) -> Conclusion,
}

/// Whether a producer stands behind one binding or one table, and which schema it emitted against.
///
/// The schema identity rides here rather than on the row, so hand-written rows never touch one and row identity never churns when a producer-facing schema changes.
/// Each seat states its own emitter, so a produced table holding a hand-written row is an ordinary thing and neither seat speaks for the other.
///
/// A pin records which schema identity a producer emitted against; it is not evidence that the pin is current.
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
/// The constructor verifies that the row's subject route and check reference are the attachment's, so a binding cannot pair a row with a callable that judges something else.
pub struct Binding<Invocation, Conclusion> {
    row: Row,
    attachment: ExecutableAttachment<Invocation, Conclusion>,
    provenance: Provenance,
}

/// Why one binding was refused.
///
/// Dependent checks in a declared order: the subject, then the check, then the provenance the row's origin demands.
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
    /// The row carries producer facts, but the binding names no schema the producer emitted against.
    GeneratedWithoutSchemaPin,
}

/// The complete authored world: every binding a run's denominator is stated over.
///
/// One authored world, ever: a selection narrows a run and never narrows this.
pub struct AuthoredTable<Invocation, Conclusion> {
    name: AuthoredTableName,
    provenance: Provenance,
    bindings: Vec<Binding<Invocation, Conclusion>>,
}

/// Why one authored table was refused.
///
/// Each cause names the offending row by its trial key, so a caller reads which row rather than which position.
#[must_use = "a refusal is the reason a table was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoredTableRefusal {
    /// A binding carries the candidate origin arm, which is lawful in a staged view and never in the authored world.
    CandidateOrigin(TrialKey),
    /// Two bindings state one trial.
    DuplicateTrial(TrialKey),
}

/// A complete authored world with candidate bindings overlaid on it, for proving a candidate against the world it would join.
///
/// The parent is borrowed rather than copied, so nothing here can grow into a second authored world.
/// A staged run is not an authored run: claim coverage admits authored-posture reports only, so refusal rather than declaration is what keeps a staged run out.
pub struct StagedTableView<'parent, Invocation, Conclusion> {
    parent: &'parent AuthoredTable<Invocation, Conclusion>,
    candidates: Vec<Binding<Invocation, Conclusion>>,
}

/// Why one staged view was refused.
#[must_use = "a refusal is the reason a staged view was not built"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedTableRefusal {
    /// An overlaid binding does not carry the candidate origin arm, so it is an authored row entering by the staging door.
    NotACandidate(TrialKey),
    /// A candidate states a trial the parent or another candidate already states.
    DuplicateTrial(TrialKey),
}

/// Which world a view presents, and — when it is staged — the authored parent it was overlaid on.
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
/// An enum rather than a trait, and that is the sealing: no outside crate can add a third world by implementing anything.
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
    /// A roster carrying at least one value.
    OneOrMore,
}

/// What shape one schema field's values take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldShape {
    /// A namespaced name: an owner and a spelling.
    NamespacedName,
    /// A thirty-two byte content address.
    ContentAddress,
    /// One arm of a closed roster, named by the arm spellings the field admits, in declared order.
    ClosedChoice(&'static [&'static str]),
    /// An unbounded byte string.
    Bytes,
    /// A count.
    Count,
    /// One producer-discovered mutation alternative: its operator family and canonical mutation meaning.
    MutationAlternative,
}

/// One field of one producer-facing vocabulary, as the schema declares it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaField {
    name: &'static str,
    shape: FieldShape,
    cardinality: FieldCardinality,
}

/// The descriptor vocabulary's canonical field roster, as the schema declares it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DescriptorSchema {
    fields: &'static [SchemaField],
}

/// The mutation-discovery vocabulary's canonical field roster, as the schema declares it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MutationDiscoverySchema {
    fields: &'static [SchemaField],
}

/// The bench-row vocabulary's canonical field roster, as the schema declares it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BenchSchema {
    fields: &'static [SchemaField],
}

/// Why one schema member was refused.
///
/// Dependent checks in a declared order: an empty roster, then each field's name, then a repeated name.
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

/// One generated-support schema identity, either freshly derived or reified from an address already derived.
///
/// [`GeneratedSupportSchema::identity`] derives one from a root declaration's canonical bytes.
/// [`GeneratedSupportSchemaId::over`] reifies an address whose derivation its caller already established, and neither derives nor verifies that address again.
///
/// Two of these being equal says the two declarations encode the same rosters.
/// It says nothing about whether either side is current: a pair that agrees because publication never ran is exactly what this comparison cannot see.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedSupportSchemaId(ContentAddress);

/// Why one encoding was refused.
///
/// One family for both preimages this home writes: the root schema declaration's, and one row's.
/// The single cause is unreachable on every target this crate is built for; it exists because the encoder states its widths rather than guessing at one.
#[must_use = "a refusal is the reason canonical bytes were not produced"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncodeRefusal {
    /// A length does not fit the sixty-four bit width the encoding declares.
    LengthPastEncodingWidth,
}

/// Why one stamped trial table was not built.
///
/// The complete refusal family of the road [`trial_table!`](crate::trial_table) expands, and the one family a declared row expression refuses in.
/// Each arm carries the owning constructor's own typed refusal unchanged, so a seat that could not be built states which construction refused.
///
/// Every family a row expression's `?` can raise has exactly one lawful discharge into this one, declared as a [`From`] realization in `type_contract.rs`, which is why a producer's expression never names an arm here.
/// The authored-table refusal has no such discharge: the only construction that raises it stands in the stamp's tail position, where the arm is named where it stands.
#[must_use = "a refusal is the reason a trial table was not stamped"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialTableRefusal {
    /// A name the road declares was refused by the name parser.
    NameNotParsed(NameRefusal),
    /// A row's two open rosters stated a role or a tag twice.
    ClassificationNotAuthored(ClassificationRefusal),
    /// A row's own constructor refused the values it was given.
    RowNotDeclared(RowRefusal),
    /// The root schema declaration a produced table or row pins against was refused by the roster law.
    SchemaNotDeclared(SchemaRefusal),
    /// The root schema declaration's canonical bytes were refused, so no identity could be derived to pin against.
    SchemaNotEncoded(EncodeRefusal),
    /// A row's binding constructor refused the row and attachment it was given.
    BindingNotBound(BindingRefusal),
    /// The authored world refused the bindings it was given.
    TableNotAuthored(AuthoredTableRefusal),
}

/// One field of a row, as the canonical row traversal reaches it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DescriptorProjection {
    Claim,
    ExecutionSuite,
    Roles,
    Tags,
    Subject,
    Check,
    Population,
    Origin,
}

/// The root's accepted members, their order, and their canonical tags, in one place.
///
/// It projects the root's seats, its guard operations, and its canonical traversal, so a change to any member moves the derived identity through one authored membership fact.
macro_rules! generated_support_members {
    ($callback:ident $(, $argument:ident)*) => {
        $callback! {
            [$($argument),*];
            descriptor: DescriptorSchema => DESCRIPTOR_FIELDS => 1,
            mutation_discovery: MutationDiscoverySchema => MUTATION_DISCOVERY_FIELDS => 2,
            bench: BenchSchema => BENCH_FIELDS => 3,
        }
    };
}

pub(super) use generated_support_members;

macro_rules! declare_generated_support_schema {
    ([]; $( $member:ident: $member_type:ty => $fields:ident => $tag:literal, )+) => {
        /// The root declaration every producer-facing vocabulary is pinned through: the descriptor, mutation-discovery, and bench field rosters, together.
        ///
        /// A change to any member moves the derived identity, so one pin governs all three crossings.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct GeneratedSupportSchema {
            $(
                $member: $member_type,
            )+
        }
    };
}

generated_support_members!(declare_generated_support_schema);

/// The origin arms, their canonical spellings, and their canonical slots, in one place.
///
/// It projects the schema's closed-choice spellings and the encoder's slot match, so their order cannot drift independently.
macro_rules! origin_declarations {
    ($callback:ident) => {
        $callback! {
            HandWritten => "hand-written" => 1,
            Generated(_) => "generated" => 2,
            Candidate(_) => "candidate" => 3,
            AdmittedReplay(_) => "admitted-replay" => 4,
            AdmittedDischarge(_) => "admitted-discharge" => 5,
        }
    };
}

pub(super) use origin_declarations;

macro_rules! declare_origin_choices {
    ($( $variant:ident $(($payload:pat))? => $spelling:literal => $slot:literal, )+) => {
        const ORIGIN_CHOICES: &[&str] = &[
            $(
                $spelling,
            )+
        ];
    };
}

origin_declarations!(declare_origin_choices);

macro_rules! declare_descriptor_fields {
    ($( $projection:ident => $name:literal => $shape:expr => $cardinality:expr, )+) => {
        pub(super) const DESCRIPTOR_PROJECTIONS: &[DescriptorProjection] = &[
            $(
                DescriptorProjection::$projection,
            )+
        ];

        /// The descriptor vocabulary's canonical field roster: the closed field set one row states, in reading order.
        ///
        /// This public roster and the canonical row traversal are emitted from one local declaration, so neither can drift from the other.
        pub const DESCRIPTOR_FIELDS: &[SchemaField] = &[
            $(
                SchemaField::declared($name, $shape, $cardinality),
            )+
        ];
    };
}

declare_descriptor_fields! {
    Claim => "claim" => FieldShape::NamespacedName => FieldCardinality::ExactlyOne,
    ExecutionSuite => "execution_suite" => FieldShape::NamespacedName => FieldCardinality::ExactlyOne,
    Roles => "roles" => FieldShape::NamespacedName => FieldCardinality::ZeroOrMore,
    Tags => "tags" => FieldShape::NamespacedName => FieldCardinality::ZeroOrMore,
    Subject => "subject" => FieldShape::NamespacedName => FieldCardinality::ExactlyOne,
    Check => "check" => FieldShape::NamespacedName => FieldCardinality::ExactlyOne,
    Population => "population" => FieldShape::NamespacedName => FieldCardinality::ExactlyOne,
    Origin => "origin" => FieldShape::ClosedChoice(ORIGIN_CHOICES) => FieldCardinality::ExactlyOne,
}

/// The mutation-discovery vocabulary's canonical field roster: what a producer states about one candidate site before admission.
///
/// The runtime types that carry these values belong to the lane that owns them ([`crate::muterprater`]); what is declared here is the producer-facing vocabulary.
///
/// The owner claim is optional because discovery must retain an unmapped site rather than invent an owner.
/// The original operation is carried as rendered bytes rather than as a name, because two different operations a producer happened to name alike would otherwise encode identically.
/// The alternatives are the damages the producer discovered, and the roster is nonempty because a site with no alternative states no candidate.
/// The activation site is named rather than path-spelled, for the reason a trial's identity is not its site: a file move must rename nothing.
///
/// Discovery grants no permission and makes no alternative executable.
pub const MUTATION_DISCOVERY_FIELDS: &[SchemaField] = &[
    SchemaField::declared(
        "identity",
        FieldShape::NamespacedName,
        FieldCardinality::ExactlyOne,
    ),
    SchemaField::declared(
        "owner_claim",
        FieldShape::NamespacedName,
        FieldCardinality::ZeroOrOne,
    ),
    SchemaField::declared(
        "original_operation",
        FieldShape::Bytes,
        FieldCardinality::ExactlyOne,
    ),
    SchemaField::declared(
        "candidate_alternatives",
        FieldShape::MutationAlternative,
        FieldCardinality::OneOrMore,
    ),
    SchemaField::declared(
        "activation_site",
        FieldShape::NamespacedName,
        FieldCardinality::ExactlyOne,
    ),
];

/// The bench-row vocabulary's canonical field roster: what a producer states about one measured workload.
///
/// The input-size axis is a roster rather than one size, because a growth class is read off a curve and never off one point.
///
/// The correctness preflight and the planted-worse falsifier are the two gates that run before any backend is invoked: a failing operation is never benchmarked, and a measurement that cannot separate a deliberately worse implementation from the real one has not been shown to measure anything.
/// Both are references, and the callables behind them ride the bench binding.
///
/// The declared budgets are the gate's own tolerances, stated beside the row so a threshold is spec rather than a number somebody tuned.
/// The contention posture is required, because a measurement under an undeclared posture is inadmissible; its closed choice carries one arm because one arm is all the declared facts support.
/// The work formula is optional because only some operations declare one, and where one is declared the gate reads work counts against it and wall time is the secondary observation.
/// The complexity claim is a neutral reference: a standalone public vocabulary never names a consumer's type, so a consumer maps its own complexity contract into this seat from its own side.
///
/// A roster of declared budgets carries counts and states nothing about what a run spent.
/// The posture's one arm claims no quiet host: it says nothing was declared beside the measurement, which is a fact about a declaration and never about a host.
pub const BENCH_FIELDS: &[SchemaField] = &[
    SchemaField::declared(
        "workload_identity",
        FieldShape::NamespacedName,
        FieldCardinality::ExactlyOne,
    ),
    SchemaField::declared(
        "input_size_axis",
        FieldShape::Count,
        FieldCardinality::ZeroOrMore,
    ),
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
    SchemaField::declared(
        "declared_budgets",
        FieldShape::Count,
        FieldCardinality::ZeroOrMore,
    ),
    SchemaField::declared(
        "contention_posture",
        FieldShape::ClosedChoice(&["no-declared-contention"]),
        FieldCardinality::ExactlyOne,
    ),
    SchemaField::declared(
        "work_formula",
        FieldShape::Bytes,
        FieldCardinality::ZeroOrOne,
    ),
    SchemaField::declared(
        "complexity_claim",
        FieldShape::NamespacedName,
        FieldCardinality::ExactlyOne,
    ),
];

#[path = "type_guard.rs"]
mod guard;
