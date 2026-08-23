//! The test-descriptor home's declarations: the generated support shell every
//! crossing of the wall rides, the four syntax-facing identifiers a trial
//! declaration spells, the descriptor row vocabulary in the harness's own field
//! shape, what the shell declares into the gate's TRIALS seat and what an
//! expansion defers into its DEFERRED seat, the two rename twins a rendered path
//! is rooted at, and the magnitudes and refusal families this home answers with.
//!
//! Declarations only.
//! Every road that reaches a private field — a name's two parts, a path's segments, an identifier's spelling, a row's rosters, a group's rows, a payload's groups, a deferred cargo's tokens, the shell's mangled name and tree, and the refusal body's one seat — lives in `type_guard.rs`, this file's own child.
//!
//! # What an author states, and what a producer states
//!
//! The vocabulary below is cut along that line and the cut is structural rather
//! than documented. A declaration author states descriptor MEANING: the support
//! name a consumption target invokes, the module the stamp writes, the table's
//! name, each aggregate seat and the suite it selects on, and each row's lens,
//! claim, roles, tags, subject, check, and population. Nothing else has a seat.
//!
//! The producer's own act — the door, the projection, the producer's name, and
//! the schema identity a produced table pins against — is fixed by construction
//! or composed inside the rendering. The consumption target's host facts — the
//! two revision commitments, the callable, the invocation budgets, the target
//! binding, and the clock — have no seat here at all: they arrive as expressions
//! at the carrier's own invocation, inside the test target that owns them.
//!
//! # Nothing of the harness is imported
//!
//! Not one type here is the harness's. What crosses the wall is conforming DATA
//! in the harness's declared field shape, and the constructor-calling
//! expressions the shell renders name the harness through the caller-supplied
//! binding rather than through a dependency edge. The producer writes letters to
//! an address; it does not own the mailbox.
//!
//! # This home owns the carrier
//!
//! The wall declares ONE physical carrier for its three crossings — the
//! generated support shell — and the carrier is declared here, with the first
//! crossing, because the second and third are declared later in the module order
//! and a carrier declared twice is two carriers. The benchmark home reads
//! [`ShellName`], [`WallName`], [`BoundPath`], [`CrateFacing`] and the shell's own
//! roads from here and declares none of them again.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    GeneratedUnitSubject, GeneratorVersionSubject, PlanId, ProfileVersion, ProjectionIdentity,
    ProjectionProfileSubject, SoleRenderedUnit,
};
use crate::planning::{ContentAddressing, ObligationAnchoring};
use crate::token::{GeneratedTree, SpanHandle};
use macroonz::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitudes.
//
// This home's own rows, stamped by the plane's magnitude stamp. The stamp is the
// plane's mechanism; the meaning, the number, and the reason on every row below
// are this home's, declared beside the capacities they govern.
// ---------------------------------------------------------------------------

crate::plane::limits! {
    /// The magnitude governing how many segments one rendered path may carry
    /// after the crate binding it is rooted at.
    ///
    /// # Bounds
    ///
    /// Eight. A path reaching deeper than eight segments past a crate root has
    /// stopped naming an item and started describing a tree, and the repair is a
    /// re-export at the address rather than a longer spelling at this end.
    PathSegmentLimit = 8,
    /// The magnitude governing how many roles one descriptor row may carry.
    ///
    /// # Bounds
    ///
    /// Sixteen. A role is open classification and a row carrying more than
    /// sixteen of them has stopped classifying and started describing; the
    /// repair is a second row, not a wider roster.
    RoleLimit = 16,
    /// The magnitude governing how many tags one descriptor row may carry.
    ///
    /// # Bounds
    ///
    /// Sixteen, and DECLARED separately from [`RoleLimit`] rather than aliased
    /// to it: roles and tags are two capacities the harness declares as two
    /// rosters, and one family standing for both would be one authority
    /// answering two questions.
    TagLimit = 16,
    /// The magnitude governing how many rows one aggregate seat's group may
    /// declare.
    ///
    /// # Bounds
    ///
    /// Two hundred and fifty-six. Every row is one stamped lens function and one
    /// entry in the table the seat runs, so the group's size is what a
    /// consumer's test binary pays for; past this the repair is a second stamped
    /// module.
    RowLimit = 256,
    /// The magnitude governing how many aggregate seats one stamped module
    /// declares.
    ///
    /// # Bounds
    ///
    /// Thirty-two. A seat is one ordinary test function selecting on one
    /// execution suite, and a module declaring more suites than this is a module
    /// whose rows belong to more than one world.
    SuiteGroupLimit = 32,
    /// The magnitude governing how many issues one shell-rendering refusal body
    /// may carry.
    ///
    /// # Bounds
    ///
    /// Sixteen. The rendering's issues are facts about the TOKEN VOCABULARY and
    /// about the harness's own refusal composition rather than about the rows,
    /// so their count is bounded by how many distinct spellings one shell needs
    /// and not by how many rows it carries — a shell of one row and a shell of
    /// two hundred establish the same set.
    ///
    /// Written as the number rather than as a product of the row magnitude
    /// beside it: a magnitude derived from another magnitude reads as a fact
    /// when it is a choice.
    ShellIssueLimit = 16,
}

// ---------------------------------------------------------------------------
// The declaration refusal family.
// ---------------------------------------------------------------------------

macroonz::closed_register! {
    /// How one declaration of this home's vocabulary refuses.
    ///
    /// Dependent checks in a declared order, so exactly one cause is true of any
    /// refused declaration: a name's parts are read before a path's segments, a
    /// path's segments before a row's rosters, and a row's rosters before a
    /// group's rows.
    /// Every one of them refuses before a partial value exists — a row holding
    /// some of its roles is a row classified as something nobody declared.
    #[must_use = "a declaration refusal names the exact seat the declaration did not fill"]
    pub enum ShellDeclarationRefusal {
        /// The name states no owner.
        EmptyNamespace = "empty-namespace",
            "a wall name states no owner";
        /// The name states no spelling.
        EmptyStem = "empty-stem",
            "a wall name states no spelling";
        /// A spelling the rendering writes as a Rust identifier is not one, so
        /// the emission would write tokens the consumer's compiler reads as
        /// something else.
        SpellingNotAnIdentifier = "spelling-not-an-identifier",
            "a rendered spelling is not one Rust identifier";
        /// The path names no segment past the crate binding it is rooted at, so
        /// it names the crate and nothing in it.
        PathSegmentsAbsent = "path-segments-absent",
            "a rendered path names no segment past its crate binding";
        /// The path carries more segments than the declared magnitude.
        PathSegmentsUnbounded = "path-segments-unbounded",
            "a rendered path carries more segments than the declared magnitude";
        /// Two of one row's roles carry one spelling, so the roster states a
        /// classification twice and the harness would refuse the row it is
        /// emitted into.
        RoleDoubled = "role-doubled",
            "two roles of one descriptor row carry one spelling";
        /// The row carries more roles than the declared magnitude.
        RolesUnbounded = "roles-unbounded",
            "a descriptor row carries more roles than the declared magnitude";
        /// Two of one row's tags carry one spelling.
        TagDoubled = "tag-doubled",
            "two tags of one descriptor row carry one spelling";
        /// The row carries more tags than the declared magnitude.
        TagsUnbounded = "tags-unbounded",
            "a descriptor row carries more tags than the declared magnitude";
        /// The group declares no row at all, and a seat over no row is a seat
        /// that measures nothing.
        RowsAbsent = "rows-absent",
            "an aggregate seat's group declares no row";
        /// The group declares more rows than the declared magnitude.
        RowsUnbounded = "rows-unbounded",
            "an aggregate seat's group declares more rows than the declared magnitude";
        /// Two rows of one group carry one lens spelling, so the stamped module
        /// would declare one function twice.
        LensSpellingDoubled = "lens-spelling-doubled",
            "two rows of one group carry one lens spelling";
        /// The payload declares no aggregate seat at all.
        SuiteGroupsAbsent = "suite-groups-absent",
            "a stamped payload declares no aggregate seat";
        /// The payload declares more aggregate seats than the declared
        /// magnitude.
        SuiteGroupsUnbounded = "suite-groups-unbounded",
            "a stamped payload declares more aggregate seats than the declared magnitude";
        /// Two aggregate seats of one payload carry one spelling.
        SeatSpellingDoubled = "seat-spelling-doubled",
            "two aggregate seats of one payload carry one spelling";
    }
}

// ---------------------------------------------------------------------------
// The trial-declaration grammar's refusal family.
// ---------------------------------------------------------------------------

macroonz::closed_register! {
    /// How one authored trial declaration is not read.
    ///
    /// Dependent checks in a declared order, so exactly one cause is true of any
    /// refused declaration: the attribute is found before its body is read, the
    /// body's clauses before their values, and a value's shape before the
    /// vocabulary that value states.
    ///
    /// # Authority
    ///
    /// **The grammar is CLOSED and states descriptor meaning alone.** Every seat
    /// a producer performs — the origin, the producer's name, the projection, the
    /// schema a produced table pins against — and every seat a consumption target
    /// supplies — the two revision commitments, the callable, the declared
    /// budgets, the target binding, the clock — reaches
    /// [`TrialDeclarationCause::NotADeclarableClause`], because none of them is a
    /// thing the declaration author is the one to state.
    #[must_use = "a trial-declaration refusal names the exact seat the declaration did not fill"]
    #[expect(
        clippy::enum_variant_names,
        reason = "the shared word is the axis: this is a single-cause family whose every row names a thing the reading did NOT establish, and a roster without it would read as a roster of clauses rather than of what was missing from them"
    )]
    pub enum TrialDeclarationCause {
        /// The declaration carries the trial attribute more than once, so two
        /// declarations of one carrier's rows stand beside each other and neither
        /// is the one.
        NotDeclaredOnce = "not-declared-once",
            "the declaration carries the trial attribute more than once";
        /// The trial attribute states no parenthesized body, so it declares no
        /// rows at all.
        NotBodied = "not-bodied",
            "the trial attribute states no body";
        /// A clause of the body is not `<key> = <value>` shaped.
        NotAClause = "not-a-clause",
            "a clause of the trial declaration is not one key and one value";
        /// A clause's key is not one this grammar declares.
        NotADeclarableClause = "not-a-declarable-clause",
            "the clause is not one the trial grammar declares";
        /// One clause key is stated twice, so the declaration says two things
        /// where the grammar admits one.
        NotDistinct = "not-distinct",
            "one clause of the trial declaration is stated twice";
        /// A required clause is absent.
        NotCovered = "not-covered",
            "a required clause of the trial declaration is absent";
        /// A value written where `named(<namespace>, <stem>)` is required is not
        /// one.
        NotANamedReference = "not-a-named-reference",
            "a value is not one `named(<namespace>, <stem>)` reference";
        /// A value written where a bracketed roster is required is not one.
        NotARoster = "not-a-roster",
            "a value is not one bracketed roster";
        /// A suite clause is not `suite <seat> = named(<namespace>, <stem>) {
        /// <rows> }` shaped.
        NotASuiteGroup = "not-a-suite-group",
            "a suite clause is not one seat, one suite reference, and one row body";
        /// A row is not `<lens> { <clauses> }` shaped.
        NotARow = "not-a-row",
            "a row is not one lens and one clause body";
    }
}

/// How one authored trial declaration was not read: which of the two homes
/// refused, and the token it was established at.
///
/// # Authority
///
/// **Two homes answer at this seam and each answer is carried whole.** Whether
/// the tokens SAY a trial declaration is this grammar's question; whether the
/// values they say are a lawful carrier declaration is the carrier vocabulary's,
/// and its constructors answer in their own family. A single roster covering both
/// would give a malformed clause and a doubled role one shape and one
/// related-identity tag, and two homes' facts under one tag derive one related
/// identity for two unrelated observations.
///
/// # Bounds
///
/// Both arms name a TOKEN. A trial declaration is read out of an attribute a
/// person wrote, so every refusal on this road is a fact about one clause of it
/// and the reader is sent to that clause rather than to the declaration's
/// opening.
#[must_use = "a trial-declaration refusal names which home refused and the token it was established at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrialDeclarationRefusal {
    /// The authored grammar refused: this home's own cause, at the token the
    /// clause it was established at sits at.
    Grammar {
        /// The established cause.
        cause: TrialDeclarationCause,
        /// The token it was established at.
        at: SpanHandle,
    },
    /// The carrier's own declaration vocabulary refused a value the grammar read:
    /// that family's own cause, carried exactly as its constructor returned it,
    /// at the token the clause the value was read from sits at.
    Carrier {
        /// The carrier vocabulary's own refusal.
        refusal: ShellDeclarationRefusal,
        /// The token the clause it was read from sits at.
        at: SpanHandle,
    },
}

// ---------------------------------------------------------------------------
// The two rename twins.
// ---------------------------------------------------------------------------

macroonz::closed_register! {
    /// Which of the two rename twins a rendered path is rooted at.
    ///
    /// A closed roster of exactly two, because the wall declares exactly two
    /// crates a consumer may rename and a generated expression names no third:
    /// the MACHINE, whose operations a row measures or challenges, and the
    /// HARNESS, whose vocabulary the row is spelled in and whose lanes judge it.
    ///
    /// # Bounds
    ///
    /// A callable living in the consumer's OWN crate has no arm here, and it
    /// needs none. A generated trial row points at a check function the
    /// CONSUMPTION target owns, and that value arrives as an EXPRESSION at the
    /// carrier's own invocation — where the target's own hygiene reaches its own
    /// items — rather than as a path this side would have to root somewhere. A
    /// third twin would be the wall admitting a crate it cannot name; supplying
    /// the expression is the target naming its own.
    ///
    /// Which twins a delivery ASKS for is the carrier's own fact and not this
    /// roster's. The trial crossing spells only the harness, because nothing it
    /// renders is machine-rooted; the bench crossing spells both, because its
    /// rows point at machine callables.
    pub enum CrateFacing {
        /// The machine — the crate whose declarations the projection was planned
        /// over.
        Machine = "machine",
            "the machine, under the name the consumer reached it by";
        /// The harness — the crate that owns the descriptor vocabulary and the
        /// gate.
        Harness = "harness",
            "the harness, under the name the consumer reached it by";
    }
}

/// A namespaced name: the owner that declares a spelling, and the spelling.
///
/// The FIELD SHAPE is the harness's, mirrored here as data. Nothing of the
/// harness is imported and no harness type is named: what crosses the wall is a
/// conforming pair of parts rather than a borrowed type.
///
/// # Construction
///
/// Both parts are refused empty, so a name that names nothing is not a value
/// anybody can hold — which is also what makes the emission's own name
/// constructors unable to refuse at the consumer's site for a reason this side
/// could have seen.
///
/// # Bounds
///
/// The parts are OWNED text, where the harness's own are `'static`. That
/// difference is the side of the wall each one is on: a name here is cut from the
/// token material one expansion was handed, and it becomes static text only once
/// the shell splices it into the consumer's own target.
///
/// # Ordering
///
/// The order is the storage order a set needs to iterate the same way every run,
/// over the namespace and then the stem. It ranks nothing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WallName {
    namespace: String,
    stem: String,
}

/// One path a rendered expression names, rooted at one of the two rename twins.
///
/// # Authority
///
/// **A path here never spells a crate name.** It states which TWIN it is rooted
/// at, and the rendering writes the shell's own metavariable for that twin — so a
/// consumer that renamed either crate gets its own name back without this home
/// ever learning what the name is.
///
/// # Bounds
///
/// The segments are structurally non-empty: a path naming a crate and nothing in
/// it names no item.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoundPath {
    facing: CrateFacing,
    segments: NonEmptyBounded<String, PathSegmentLimit>,
}

// ---------------------------------------------------------------------------
// The four syntax-facing identifiers a trial declaration spells.
//
// Four types rather than one, because a support macro is not a row lens and the
// compiler is the seat that should say so: a road that wants the name a
// consumption target invokes cannot be handed the name a lens is declared under,
// and no call site is told the two apart by argument position.
// ---------------------------------------------------------------------------

/// The exported name a consumption target invokes one declaration's support
/// carrier by.
///
/// # Authority
///
/// **The author chooses it, and rustc collision-checks it.** The physical
/// carrier is exported under a plan-keyed spelling nobody can know before
/// expansion ([`ShellName`]), so a declaration whose support nobody can address
/// is a declaration whose rows nobody can run. This is the address, and it is the
/// author's because a name a producer composed would be a name the author has to
/// discover.
///
/// A second declaration in one crate choosing this spelling is an ordinary
/// duplicate-macro refusal at the consumer's own compiler. Nothing here keeps a
/// register of what it has already exported.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupportMacroName(String);

/// The module the stamp writes one declaration's trial table into, at the
/// consumption target.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrialModuleName(String);

/// The aggregate seat one suite group is declared under — the ordinary test
/// function that runs by default.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrialSeatName(String);

/// The named lens one row is declared under — the ignored-by-default test
/// function a person runs by name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrialLensName(String);

// ---------------------------------------------------------------------------
// The descriptor row vocabulary, in the harness's field shape.
// ---------------------------------------------------------------------------

macroonz::closed_register! {
    /// The declaration doors a generated descriptor row may be authored through.
    ///
    /// # Authority
    ///
    /// **A door is the PRODUCER's fact and this roster is closed, so a
    /// declaration author cannot state one.** Which door a row came through is
    /// something these services performed; a seat that took a spelling would let
    /// an authored declaration sign an act it did not perform.
    ///
    /// A second door is a law change here, and one the compiler collects: every
    /// reading over this roster is a `match` that stops compiling until the new
    /// row says what it is called.
    pub enum DeclarationDoor {
        /// The refusal-family derive.
        RefusalFamilyDerive = "refusal-family-derive",
            "the refusal-family derive";
    }
}

/// The owner every name these services declare about their OWN act is spelled
/// under.
///
/// The producer, the door, and the projection a generated row carries are the
/// services' facts, so they are spelled under the machine's own owner and never
/// under the declaration author's.
pub const PRODUCER_NAMESPACE: &str = "threadpak";

/// The producer that emits a generated trial table, by its declared spelling.
pub const GENERATED_TABLE_PRODUCER: &str = "macroc";

/// The projection that emits a generated descriptor row, by its declared
/// spelling.
pub const GENERATED_ROW_PROJECTION: &str = "test-descriptor-projection";

/// The four namespaced references one descriptor row states about itself.
///
/// Every seat is public and required, because a row that could omit its claim,
/// its subject, its check, or its population is a row the harness's closed field
/// set would refuse — and a shape that can express the refused row is a shape
/// that defers the refusal to somebody else's compiler.
///
/// # Bounds
///
/// The EXECUTION SUITE is not among them, and its absence is the whole of what
/// keeps one suite from being authored twice. A row runs under exactly one
/// aggregate seat, the seat is what a suite group declares, and a seat carrying
/// rows whose own suite is a different name is a seat that selects none of them —
/// a disagreement no constructor could have refused, because both spellings were
/// lawful. The suite is stated once, at [`SuiteGroup`], and every row under it
/// inherits that one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RowReferences {
    /// The claim this row serves.
    pub claim: WallName,
    /// What this row exercises.
    pub subject: WallName,
    /// The check that judges the subject.
    pub check: WallName,
    /// The population that supplies this row's inputs.
    pub population: WallName,
}

/// One descriptor row, in the harness's closed field shape, plus the lens name
/// the stamp will declare it under.
///
/// # Bounds
///
/// The lens spelling is not a row field — the harness's roster has no seat for
/// it — and it is carried here because the stamp's grammar demands one: a row
/// arrives at `trial_table!` as `<lens>: <expression>`, and a producer that did
/// not name its lens would be handing the stamp an unnamable row.
///
/// **There is no attachment seat.** A harness `Row` and an `ExecutableAttachment` meet at a `Binding` and nowhere earlier, and the attachment's three parts — the two revision commitments and the callable that reaches the conclusion — are the consumption target's: the check functions a generated row points at live in the test target that invokes the carrier, which is not the crate the declaration sits in and has no crate binding a rendered path could be rooted at.
/// The attachment therefore arrives as expressions at the carrier's invocation, where the test target's own hygiene reaches its own items, and this side declares descriptor meaning only.
///
/// **There is no origin seat either.** The harness's origin roster carries five
/// arms and four of them are somebody else's act — a hand wrote it, a synthesis
/// cut it, a human admitted it — so a producer able to express one of those would
/// be claiming an act it did not perform. What the rendering writes is the
/// generated arm and its two producer facts, composed from the payload's own
/// [`DeclarationDoor`] and this home's declared projection spelling; a row that
/// cannot express an origin at all cannot express the wrong one.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorRow {
    lens: TrialLensName,
    references: RowReferences,
    roles: Bounded<WallName, RoleLimit>,
    tags: Bounded<WallName, TagLimit>,
}

/// One aggregate seat's group: the seat the stamp declares, the execution suite
/// that seat selects on, and the rows declared under it.
///
/// # Authority
///
/// **The suite is stated HERE and inherited by every row under it.** One seat
/// selects on one suite and every row it runs carries that suite, so the two
/// spellings a row and its seat used to carry were one fact written twice — and
/// two lawful spellings that disagree produce a seat that runs nothing while
/// every constructor on the road admits them. A row grouped here IS a row of this
/// suite, structurally.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuiteGroup {
    seat: TrialSeatName,
    suite: WallName,
    rows: NonEmptyBounded<DescriptorRow, RowLimit>,
}

/// The complete payload one stamped trial table is declared from: the exported
/// name a consumption target invokes, the module the stamp writes, the table's
/// own name, the producer that emitted it, and the aggregate seats with their
/// rows.
///
/// # Authority
///
/// **The author states descriptor MEANING and the producer states its own act.**
/// The support name, the module, the table's name, each seat's name and suite,
/// and every row's lens, claim, roles, tags, subject, check, and population are
/// the declaration's. The door is a row of a closed roster no declaration can
/// reach; the producer's own name and the projection's are this home's declared
/// spellings; and the schema a produced table pins against is derived inside the
/// rendering. None of the four has a clause an author could fill.
///
/// # Bounds
///
/// The provenance is fixed to the PRODUCED form and there is no unproduced seat:
/// a table this home rendered was emitted by a producer by construction, and a
/// shell claiming otherwise would be a producer denying its own act.
///
/// The invocation profile, the target binding, and the clock are deliberately
/// absent. They are the consumer's own host facts, declared at the carrier's
/// invocation inside its own test target — so they travel as the shell's
/// arguments and this home neither invents them nor names their parts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrialTablePayload {
    support: SupportMacroName,
    module: TrialModuleName,
    table: WallName,
    door: DeclarationDoor,
    groups: NonEmptyBounded<SuiteGroup, SuiteGroupLimit>,
}

// ---------------------------------------------------------------------------
// The carrier.
// ---------------------------------------------------------------------------

/// The mangled, collision-free name one generated support shell is exported
/// under.
///
/// # Authority
///
/// **The spelling is DERIVED from the PLAN's own identity, at full width, and
/// from nothing else.** The shell is a `#[macro_export]` item, so it lands at
/// the root of whatever crate the declaration site sits in and shares one
/// namespace with every other exported macro there — a name a producer chose, or
/// one taken from the owner's own spelling, would collide the first time two
/// declarations in one crate wanted a shell.
///
/// The key is the PLAN's and not the planned member's, because a member's
/// semantic key is a value the planning caller supplies while a plan's identity
/// is one these services derive: two plans handed one semantic key would mint
/// one exported name for two declarations, and nothing in the types would say
/// so. A plan identity cannot be handed in at all.
///
/// It is also the identity that separates two DOORS over one declaration. A
/// plan's transcript commits to the origin trail its members walk and to the
/// trail the plan itself walks, and distinct doors are required to carry
/// distinct origins — which is exactly why door equivalence compares the INTENT
/// identity instead ([`ProjectionIntentId`](crate::planning::ProjectionIntentId)).
/// So two textually identical declarations expanded at two sites plan two
/// identities and mint two names, where an intent-scoped or content-scoped key
/// would mint one name twice.
///
/// # Bounds
///
/// The spelling carries the identity at FULL width — thirty-two bytes as
/// sixty-four lowercase hexadecimal characters — so "collision-free" is true as
/// written rather than true of a prefix. A shortened key is a different claim:
/// it says two distinct plans are unlikely to collide, and the sentence beside
/// it said they cannot.
///
/// The spelling is a Rust identifier by construction: a fixed prefix and
/// lowercase hexadecimal, which is exactly the alphabet an identifier admits
/// after its first character.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShellName {
    spelling: String,
}

/// The proved tokens one consumption target receives in the opaque deferred seat.
///
/// # Where the tokens come from
///
/// They are ONE closed expansion's proved cargo, read off that TERMINAL by the
/// caller that holds one, and handed over whole. Nothing here joins anything,
/// nothing here decides which units belong to a carrier, and nothing here holds
/// a second copy of them: the split by delivery is inside the proof, the closed
/// expansion is what the split is read from, and this seat is the vehicle's end
/// of the same tokens.
///
/// The word "receipt" is not the one for it and never was: a receipt is a
/// human-committed evidence or publication crossing, and a closed expansion
/// states in the open that it has made none
/// ([`DeliveryAddressing`](crate::closure::DeliveryAddressing)).
///
#[must_use = "a deferred cargo is one emission's proved tokens and what they stand over"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeferredCargo {
    tokens: GeneratedTree,
}

/// What one shell declares into the gate's TRIALS seat.
///
/// Two postures, and they are different facts rather than one with a missing
/// half. A crossing whose caller supplied row material declares the stamped
/// payload; a crossing whose caller supplied none declares nothing, and the
/// trials seat is rendered EMPTY rather than left out.
///
/// # Authority
///
/// **An empty trials seat is lawful and is what the rendering must admit.** The
/// rows a descriptor states about itself are the harness's declarations and
/// arrive whole from the caller
/// ([`RowMaterialPosture`](crate::planning::RowMaterialPosture)), so a door that
/// holds no caller-supplied payload has no rows to declare.
/// A carrier that still has a generated mutation module to defer is a MUTATION-ONLY delivery, which is exactly the shape the derive door produces.
/// A renderer that required a payload would make that delivery unwritable and would push the deferred cargo back outside the gate to reach a consumption target at all.
///
/// The seat is still rendered under the empty posture, because both seats are
/// always present in the published grammar: a gate arm that had to match two
/// shapes would be two arms, and one pin would open two doors.
///
/// # Bounds
///
/// A borrowed VIEW of the assembly's own seat, and crate-internal. The payload
/// lives in the assembly for as long as the rendering reads it, so a view that
/// owned a copy would clone the largest thing this home carries to hand it
/// straight back as a reference — and would make every posture of this roster
/// as large as the payload, including the one that declares nothing.
#[must_use = "a trial delivery either declares a stamped payload or states that no rows were declared"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TrialDelivery<'payload> {
    /// The crossing declared no rows into this carrier.
    NothingDeclared,
    /// The payload the trials seat carries.
    Declared(&'payload TrialTablePayload),
}

/// What one shell defers into its consumption target.
///
/// Two postures, and they are different facts rather than one with a missing
/// half. An expansion that planned members into this carrier defers their proved
/// cargo; an expansion that planned none defers nothing, and the shell splices
/// no module into the deferred seat at all.
/// A cargo of no tokens would be a module declaring a subject nothing implements
/// and constants nothing reads, which is a different thing from an expansion
/// that never sent this carrier anything — so the absence is a posture rather
/// than an empty tree.
///
/// # Bounds
///
/// A borrowed view and crate-internal, on exactly [`TrialDelivery`]'s terms.
#[must_use = "a deferred delivery either carries proved cargo or states that nothing was planned"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum DeferredDelivery<'cargo> {
    /// The expansion planned no member into this carrier.
    NothingDeferred,
    /// The cargo the carrier receives.
    Carried(&'cargo DeferredCargo),
}

/// Whether one generated-support carrier has a public invocation address.
#[must_use = "support delivery states whether the carrier has one authored public address"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum SupportDelivery<'name> {
    /// No helper declared cargo a person invokes.
    Unaddressed,
    /// The one helper-owned public support spelling.
    Addressed(&'name SupportMacroName),
}

/// The generated support shell: the ONE physical carrier every crossing of the
/// wall rides.
///
/// A rendered `macro_rules!` definition, exported hidden under its mangled name,
/// whose body is one invocation of the harness's own gate. It holds its cargo
/// INERT — the constructor-calling expressions name no harness type until the
/// consumer's target expands it — and it executes nothing in a normal build,
/// because a macro definition nobody invokes is a definition nobody compiles.
///
/// # Authority
///
/// **Nothing here is written to disk.** The shell is rendered tokens the door
/// places at the declaration site, and the consumption target invokes it. A
/// carrier that wrote a file would be a second delivery road nobody planned, and
/// the plan's own destination seat says the member lands at the declaration site.
///
/// # Bounds
///
/// The seats are exactly what a rendered unit is rebuilt from — role, semantic
/// key, profile at its version, origin trail, and the tree — plus the exported
/// name a consumer's target invokes it by, which is a fact about THIS rendering
/// and is therefore read back rather than recomputed by a caller.
#[must_use = "a generated support shell is the carrier the consumption target invokes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GeneratedSupportShell {
    role: SoleRenderedUnit,
    semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    profile: ProjectionIdentity<ProjectionProfileSubject>,
    profile_version: ProfileVersion,
    origin: OriginTrail,
    name: ShellName,
    tree: GeneratedTree,
}

/// What a test-descriptor plan decided, read off the plan's own public surface.
///
/// Every seat is public and required, because a statement that could omit its
/// engine, its declaration, or its challenged obligation would be an account that
/// sometimes says less than it knows. There is no private field here and this
/// home's invariant nucleus holds nothing of it.
///
/// # Bounds
///
/// There is no challenge-METHOD seat, and the absence is the honest shape rather
/// than a dropped fact: the plan's kind content declares none
/// ([`TestDescriptorContent`](crate::planning::TestDescriptorContent)), because
/// the harness's closed descriptor field set has no method seat at all — a row
/// names its CHECK, and which mechanism that check runs under is the check's own
/// fact. A method carried here would reach no emitted seat of the crossing: a
/// value the plan decided and nothing read, which reads as a decision the plan
/// made about the rendering when the rendering never consults it.
///
/// # Nonclaims
///
/// Holding one claims that these are the facts the plan carries under its one
/// rendered role, and nothing about whether anything was rendered.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorPlan {
    /// The plan's own identity — the key the exported shell name is derived
    /// from.
    ///
    /// Read off the plan rather than composed here, and PLAN-scoped rather than
    /// member-scoped, for the reason [`ShellName`] states: a member's semantic
    /// key is the planning caller's value while this one is derived by these
    /// services over the whole plan, so two plans cannot be made to mint one
    /// exported name.
    pub plan: PlanId,
    /// The rendered role the shell stands for.
    pub role: SoleRenderedUnit,
    /// The planned member's semantic key, exactly as the plan declared it.
    pub semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    /// The profile the plan expects to render it.
    pub profile: ProjectionIdentity<ProjectionProfileSubject>,
    /// That profile's version.
    pub profile_version: ProfileVersion,
    /// The member's origin trail, walked back to authored material.
    pub origin: OriginTrail,
    /// The complete content account the carrier plan walked in carrying.
    pub addressing: ContentAddressing,
    /// The rendering engine the shell is written by.
    pub engine: ProjectionIdentity<GeneratorVersionSubject>,
    /// What the descriptor challenges, under the posture the planning caller
    /// could honestly state.
    ///
    /// An obligation identity is the MACHINE's mint and an expansion holds none,
    /// so the seat carries the anchoring rather than the identity — read exactly
    /// as the plan stated it, and never elected here.
    pub obligation: ObligationAnchoring,
}

/// How reading a plan into [`DescriptorPlan`] disagrees with the plan.
///
/// No issue is payload-free: an issue names the role it is about, because a
/// caller told only that the reading failed has nothing to repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DescriptorPlanIssue {
    /// The plan declares no member under its kind's one rendered role, so there
    /// is no shell to render.
    RoleNotPlanned {
        /// The role's position in its kind's declared roster.
        role_slot: u32,
    },
    /// The planned member lands somewhere other than the declaration site.
    ///
    /// The shell is DEFINED at the declaration site — that is what makes it
    /// reachable — and the cargo it carries rides into a consumption target as
    /// that cargo's own delivery and never as the shell's.
    /// The destination roster names four deliveries, and a member that is not at
    /// the declaration site declared one of the other three: a standalone
    /// artifact a publication writes to its own address, the deferred cargo a
    /// test target invokes, or the deferred cargo a bench target invokes. Each
    /// of the three is a different delivery and each establishes this issue —
    /// the two carriers included, because a shell declared into a carrier is a
    /// member declared into the thing it is the vehicle for.
    DestinationNotDeclarationSite {
        /// The role whose planned destination disagreed.
        role_slot: u32,
    },
}

// ---------------------------------------------------------------------------
// The rendering refusal family.
// ---------------------------------------------------------------------------

/// How rendering the shell disagrees with what the token vocabulary can carry.
///
/// # Authority
///
/// **Every issue here is a fact about a VOCABULARY and never about a row.** A row
/// that could not be declared was refused at the door by
/// [`ShellDeclarationRefusal`]; what reaches this family is a limit of the seam
/// rather than a defect in the caller's material — so the issue names the exact
/// magnitude that bit.
///
/// # Bounds
///
/// There is no unspellable-LITERAL arm on this roster, and its absence is the
/// current truth rather than a gap somebody closed by deleting the report. The
/// generated-token roster carries a byte-string arm and a numeric arm
/// ([`GeneratedToken::ByteText`](crate::token::GeneratedToken::ByteText),
/// [`GeneratedToken::Number`](crate::token::GeneratedToken::Number)), so the
/// gate's byte-string expectation, a declared count, and a declared byte string
/// are each ONE literal token this home writes directly. A refusal arm for a
/// spelling the roster can spell would be a decision nothing can reach: no road
/// constructs it, no caller repairs it, and a reader would be owed a story about
/// a seam that closed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShellRenderIssue {
    /// The rendered tree outgrows the declared token magnitude.
    /// A shell carrying every row's complete constructor expression is the widest
    /// tree this home writes, and it refuses rather than materializing part of
    /// one.
    ShellTreeUnbounded {
        /// The declared bound.
        bound: u64,
    },
}

/// The shell-rendering refusal family body, published from this file and DECLARED
/// in `type_guard.rs`'s `seat` module, beside the only roads that reach its seat.
///
/// The declaration is not here because Rust's privacy is MODULE-scoped: a seat
/// declared beside the rest of this home's declarations would put all of them
/// inside the same wall.
pub use guard::ShellRendering;

/// The one alphabet every spelling any crossing renders as a Rust identifier is
/// admitted by, published from the nucleus that every road here already reads it
/// through.
pub use guard::is_rendered_identifier;
