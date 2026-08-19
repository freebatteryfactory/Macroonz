//! The test-descriptor home's declarations: the generated support shell every
//! crossing of the wall rides, the descriptor row vocabulary in the harness's
//! own field shape, the two rename twins a rendered path is rooted at, and the
//! magnitudes and refusal families this home answers with.
//!
//! Declarations only.
//! Every road that reaches a private field — a name's two parts, a path's
//! segments, a row's rosters, a group's rows, a payload's groups, the shell's
//! mangled name and its tree, and the refusal body's one seat — lives in
//! `type_guard.rs`, this file's own child.
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
    GeneratedUnitSubject, GeneratorVersionSubject, ObligationSubject, OwnerIdentityRef,
    ProfileVersion, ProjectionIdentity, ProjectionProfileSubject, SoleRenderedUnit,
};
use crate::planning::CauseAnchoring;
use crate::token::GeneratedTree;
use threadpak::evidence::Method;
use threadpak::types::{Bounded, NonEmptyBounded};

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The magnitudes.
// ---------------------------------------------------------------------------

/// The magnitude governing how many segments one rendered path may carry after
/// the crate binding it is rooted at.
///
/// # Bounds
///
/// Eight. A path reaching deeper than eight segments past a crate root has
/// stopped naming an item and started describing a tree, and the repair is a
/// re-export at the address rather than a longer spelling at this end.
///
/// The authority and the number are written together in `type_contract.rs`, one
/// row per family, so a family cannot stand on the compile-time ladder while
/// wearing another road's authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathSegmentLimit;

/// The magnitude governing how many roles one descriptor row may carry.
///
/// # Bounds
///
/// Sixteen. A role is open classification and a row carrying more than sixteen
/// of them has stopped classifying and started describing; the repair is a
/// second row, not a wider roster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoleLimit;

/// The magnitude governing how many tags one descriptor row may carry.
///
/// # Bounds
///
/// Sixteen, and DECLARED separately from [`RoleLimit`] rather than aliased to
/// it: roles and tags are two capacities the harness declares as two rosters,
/// and one family standing for both would be one authority answering two
/// questions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TagLimit;

/// The magnitude governing how many rows one aggregate seat's group may declare.
///
/// # Bounds
///
/// Two hundred and fifty-six. Every row is one stamped lens function and one
/// entry in the table the seat runs, so the group's size is what a consumer's
/// test binary pays for; past this the repair is a second stamped module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RowLimit;

/// The magnitude governing how many aggregate seats one stamped module declares.
///
/// # Bounds
///
/// Thirty-two. A seat is one ordinary test function selecting on one execution
/// suite, and a module declaring more suites than this is a module whose rows
/// belong to more than one world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SuiteGroupLimit;

/// The magnitude governing how many issues one shell-rendering refusal body may
/// carry.
///
/// # Bounds
///
/// Sixteen. The rendering's issues are facts about the TOKEN VOCABULARY and
/// about the harness's own refusal composition rather than about the rows, so
/// their count is bounded by how many distinct spellings one shell needs and not
/// by how many rows it carries — a shell of one row and a shell of two hundred
/// establish the same set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShellIssueLimit;

// ---------------------------------------------------------------------------
// The declaration refusal family.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
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
// The two rename twins.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// Which of the two rename twins a rendered path is rooted at.
    ///
    /// A closed roster of exactly two, because the wall declares exactly two
    /// crates a consumer may rename and a generated expression names no third:
    /// the MACHINE, whose operations a row measures or challenges, and the
    /// HARNESS, whose vocabulary the row is spelled in and whose lanes judge it.
    ///
    /// # Bounds
    ///
    /// A callable living in the consumer's OWN crate has no arm here, and that
    /// absence is stated rather than worked around: the shell's argument list is
    /// what the wall declared, and admitting a third root is a change to the
    /// wall rather than a change to this roster.
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
// The descriptor row vocabulary, in the harness's field shape.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// What one revision identity is worth, stated by the party that bound it.
    ///
    /// The harness's own posture roster, mirrored as data. A producer states
    /// which posture the identity it names is held under; it never decides what a
    /// posture MEANS for a cache or for replay, which is the report instrument's
    /// one statement.
    pub enum RevisionStanding {
        /// Generated from an owned declaration.
        Derived = "derived",
            "generated from an owned declaration";
        /// A hand author's explicit commitment.
        Declared = "declared",
            "a hand author's explicit commitment";
        /// No stable commitment at all, and lawful.
        Untracked = "untracked",
            "no stable commitment";
    }
}

/// One revision identity a rendered attachment names, and the posture it is held
/// under.
///
/// # Authority
///
/// **The identity is NAMED and never minted here.** A revision identity is
/// minted by the act that authored it and arrives already made, so what this seat
/// carries is the path the consumer's target reaches that identity by — never
/// thirty-two bytes this home invented for it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RevisionReference {
    /// The posture the identity is held under.
    pub standing: RevisionStanding,
    /// The path the identity is reached by.
    pub address: BoundPath,
}

/// What makes one generated row executable: the two revision bindings and the
/// callable, each named as a path rather than carried as a value.
///
/// # Bounds
///
/// The callable is a path to a function item, and the harness's attachment takes
/// a function POINTER — so what the emission writes is the item's name and the
/// coercion is the consumer's compiler's. A closure is unwritable here for the
/// reason it is unwritable there: a closure carries captured state, and nothing
/// ambient rides into a trial.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RowAttachment {
    /// The subject's revision binding.
    pub subject_revision: RevisionReference,
    /// The check's revision binding.
    pub check_revision: RevisionReference,
    /// The callable that reaches this row's conclusion.
    pub call: BoundPath,
}

/// What a producer's own act contributed to one generated row: the declaration
/// door it was authored through, and the projection that emitted it.
///
/// # Authority
///
/// **This is the ONLY origin arm this home renders.** The harness's origin roster
/// carries five arms and four of them are somebody else's fact — a hand wrote it,
/// a synthesis cut it, a human admitted it — so a producer that could emit one of
/// those would be claiming an act it did not perform. The arm is fixed by the
/// TYPE rather than chosen at a call site, which is why there is no origin
/// selector anywhere in this vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProducerOrigin {
    /// The declaration door this row was authored through.
    pub door: WallName,
    /// The projection that emitted it.
    pub projection: WallName,
}

/// The five namespaced references one descriptor row states about itself.
///
/// Every seat is public and required, because a row that could omit its claim,
/// its suite, its subject, its check, or its population is a row the harness's
/// closed field set would refuse — and a shape that can express the refused row
/// is a shape that defers the refusal to somebody else's compiler.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RowReferences {
    /// The claim this row serves.
    pub claim: WallName,
    /// The one aggregate seat this row runs under by default.
    pub execution_suite: WallName,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorRow {
    lens: String,
    references: RowReferences,
    roles: Bounded<WallName, RoleLimit>,
    tags: Bounded<WallName, TagLimit>,
    origin: ProducerOrigin,
    attachment: RowAttachment,
}

/// One aggregate seat's group: the seat the stamp declares, the execution suite
/// that seat selects on, and the rows declared under it.
///
/// # Nonclaims
///
/// The grouping decides which aggregate seat EXISTS; it never decides which rows
/// the world holds, and it makes no claim that a row grouped here carries this
/// suite. The selection reads each ROW's own suite, so a row grouped under a
/// seat whose suite is not the row's own is simply not selected by that seat —
/// and the run's census says so in the open.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuiteGroup {
    seat: String,
    suite: WallName,
    rows: NonEmptyBounded<DescriptorRow, RowLimit>,
}

/// The complete payload one stamped trial table is declared from: the module the
/// stamp writes, the table's own name, the producer that emitted it, and the
/// aggregate seats with their rows.
///
/// # Bounds
///
/// The provenance is fixed to the PRODUCED form and there is no unproduced seat:
/// a table this home rendered was emitted by a producer by construction, and a
/// shell claiming otherwise would be a producer denying its own act.
///
/// The invocation profile is deliberately absent. It is a `const` item at the
/// consumer's site carrying declared budgets, and budgets are the consumer's
/// declaration rather than the producer's — so it travels as one of the shell's
/// arguments and this home neither invents one nor names its parts.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrialTablePayload {
    module: String,
    table: WallName,
    producer: WallName,
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
/// **The spelling is DERIVED from the planned member's semantic key and from
/// nothing else.** The shell is a `#[macro_export]` item, so it lands at the
/// root of whatever crate the declaration site sits in and shares one namespace
/// with every other exported macro there — a name a producer chose, or one taken
/// from the owner's own spelling, would collide the first time two declarations
/// in one crate wanted a shell. The semantic key is content-addressed, so two
/// distinct planned members reach two distinct spellings without this home
/// keeping a register of what it has already emitted.
///
/// # Bounds
///
/// The spelling is a Rust identifier by construction: a fixed prefix and
/// lowercase hexadecimal, which is exactly the alphabet an identifier admits
/// after its first character.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShellName {
    spelling: String,
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
/// # Nonclaims
///
/// Holding one claims that these are the facts the plan carries under its one
/// rendered role, and nothing about whether anything was rendered.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DescriptorPlan {
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
    /// The ONE address the entry account walked in the door carrying.
    pub declaration: CauseAnchoring,
    /// The rendering engine the shell is written by.
    pub engine: ProjectionIdentity<GeneratorVersionSubject>,
    /// The obligation the descriptor challenges.
    pub obligation: OwnerIdentityRef<ObligationSubject>,
    /// The method the challenge is made by.
    ///
    /// # Bounds
    ///
    /// It reaches no seat of the emitted row. The harness's closed descriptor
    /// field set has no method seat — a row names its CHECK, and which mechanism
    /// that check runs under is the check's own fact — so this travels for the
    /// explanation station and for a caller deciding which check reference the row
    /// should name, and the rendering reads it nowhere.
    pub challenge: Method,
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
    /// The shell is emitted at the declaration site as deferred tokens and
    /// invoked by the consumption target; a member written as a standalone
    /// artifact is a different delivery and is not this one.
    DestinationNotDeclarationSite {
        /// The role whose planned destination disagreed.
        role_slot: u32,
    },
}

// ---------------------------------------------------------------------------
// The rendering refusal family.
// ---------------------------------------------------------------------------

/// How rendering the shell disagrees with what the token vocabulary can spell or
/// with what the harness's own refusal composition admits.
///
/// # Authority
///
/// **Every issue here is a fact about a VOCABULARY and never about a row.** A row
/// that could not be declared was refused at the door by
/// [`ShellDeclarationRefusal`]; what reaches this family is a spelling the
/// rendering needs and cannot write, which is a gap in the seam rather than a
/// defect in the caller's material — so each issue names the exact seat that
/// closes it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShellRenderIssue {
    /// The gate's `expected:` clause takes a BYTE STRING literal, and the
    /// generated-token vocabulary has no arm that spells one: its literal arm is
    /// a text literal, which the gate reads as a different literal and refuses
    /// against.
    ///
    /// The seat that closes it is a byte-string arm on the generated token
    /// roster, carried through the expansion shell as the compiler's own
    /// byte-string literal. Rendering the two tokens `b` and `"…"` instead is
    /// NOT the repair: the gate's pattern matches one literal token, so the pair
    /// reaches the refusing arm and a reader is told the published pair is
    /// incoherent when the truth is that the producer could not spell its own
    /// expectation.
    PinLiteralNotSpellable {
        /// How many bytes the expectation carries, so a reader knows the width
        /// the missing arm has to spell.
        width: u64,
    },
    /// A field the schema declares as a COUNT has no literal arm in the
    /// generated-token vocabulary, so the rendering cannot write the number.
    ///
    /// The seat that closes it is a numeric arm on the generated token roster —
    /// the captured side already carries one, and the generated side does not.
    CountLiteralNotSpellable {
        /// The declared field the count belongs to.
        field: &'static str,
        /// The count itself, so a reader sees the value that could not be
        /// written rather than only the seat it belonged in.
        value: u64,
    },
    /// A field the schema declares as BYTES has no literal arm in the
    /// generated-token vocabulary either, for the same reason the gate's
    /// expectation does: the roster's one literal arm is text.
    ///
    /// It is a separate issue from the expectation's because the CONSEQUENCE
    /// differs — an unspellable expectation misroutes the gate and reports pair
    /// incoherence, while an unspellable declared field simply cannot be written —
    /// and a caller repairing them reads two different sentences.
    ByteLiteralNotSpellable {
        /// The declared field the bytes belong to.
        field: &'static str,
        /// How many bytes the field carries, so a reader knows the width the
        /// missing arm has to spell.
        width: u64,
    },
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
