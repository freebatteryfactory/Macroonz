//! The pattern-stamp home's declarations: the owner facts a scope-guard stamp
//! cites, the exact identities one stamp is planned against, the facts one
//! coupled seat is declared from, and the published artifact the coupled-seat
//! stamp is.
//!
//! Declarations only.
//! Every road that reaches a private field — a path's segments, a seat's two
//! spellings, one seat declaration, the exported stamp name, a coverage's one
//! namespace, the publication record's seats, and the artifact's rendered trees
//! — lives in `type_guard.rs`, this file's own child.
//!
//! # The receipt's field shape, and none of the machine's seats
//!
//! The machine's evidence home owns the publication receipt and every value on
//! it. What this home holds is its OWN record of what it rendered, in the
//! receipt's field shape, so the publication operation compares two
//! independently produced statements rather than one value with itself. No
//! machine receipt is constructed here, and no seat of one is stood in for.

use crate::origin_graph::OriginTrail;
use crate::plane::{
    ByteRoleSubject, CapturedDeclarationSubject, GeneratedUnitSubject, GeneratorVersionSubject,
    OriginNodeSubject, OwnerFactRef, OwnerIdentityRef, PatternArgumentSubject,
    PatternInstanceSubject, PatternSubject, ProfileVersion, ProjectionIdentity,
    ProjectionProfileSubject, SoleRenderedUnit, TracedSubject,
};
use crate::planning::{CauseAnchoring, DigestContract, ProjectionContext};
use crate::token::GeneratedTree;
use threadpak::types::NonEmptyBounded;

#[path = "type_guard.rs"]
mod guard;

// ---------------------------------------------------------------------------
// The scope-guard stamp's anchors.
// ---------------------------------------------------------------------------

/// The owner facts one scope-guard stamp cites.
///
/// Both belong to the machine's identity home. The stamp writes nothing they do
/// not already declare, and the plan's trace says so by naming them rather than
/// by asserting that a rule was followed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeGuardOwnerFacts {
    /// The identity home's fact that a Class-C position carries no ordering
    /// operator of its own.
    pub class_c_carries_no_ordering: OwnerFactRef,
    /// The identity home's fact that comparison is total within one scope and
    /// refuses across scopes.
    pub comparison_is_scope_guarded: OwnerFactRef,
}

/// The exact identities one scope-guard stamp is planned against.
///
/// There is no constructor and no default: every seat is required, because a
/// stamp plan that could omit its content, its pattern, its instantiation, or
/// its arguments would be an account that sometimes says less than it knows.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeGuardStampAnchors {
    /// The captured declaration this stamp's invocation IS — the ONE address
    /// the owner content walked in the door carrying.
    ///
    /// # Bounds
    ///
    /// It is a CAPTURE and not a declaration fragment, because a stamp is
    /// planned while an expansion is holding token material and nothing has
    /// been linked. What the content stands on is not a seat here: content that
    /// stands on nothing is a stated fact, and the entry account this seat
    /// opens is the one holder of that answer.
    pub content: ProjectionIdentity<CapturedDeclarationSubject>,
    /// The shared plan context: closed graph, profile and version, cause set,
    /// generator version, and target binding.
    pub context: ProjectionContext,
    /// The authored pattern — the machine's scope-guard version pattern.
    pub pattern: OwnerIdentityRef<PatternSubject>,
    /// This instantiation of it.
    pub instance: OwnerIdentityRef<PatternInstanceSubject>,
    /// The first typed argument: the guard type the caller named.
    pub guard_name: OwnerIdentityRef<PatternArgumentSubject>,
    /// The second typed argument: the scope type the caller named. A string
    /// never becomes an argument here — the caller states a type.
    pub scope_type: OwnerIdentityRef<PatternArgumentSubject>,
    /// The authored declaration the invocation sits in.
    pub authored_node: ProjectionIdentity<OriginNodeSubject>,
    /// The instantiated pattern as an origin node.
    pub instantiated_node: ProjectionIdentity<OriginNodeSubject>,
    /// The rendered guard as an origin node.
    pub rendered_node: ProjectionIdentity<OriginNodeSubject>,
    /// The generated unit the stamp materializes.
    pub stamped_unit: ProjectionIdentity<GeneratedUnitSubject>,
    /// The subject the plan's decisions are recorded about.
    pub traced: ProjectionIdentity<TracedSubject>,
    /// The owner facts the stamp rests on.
    pub owner_facts: ScopeGuardOwnerFacts,
}

// ---------------------------------------------------------------------------
// The magnitudes.
// ---------------------------------------------------------------------------

/// The magnitude governing how many segments one rendered type path may carry.
///
/// # Bounds
///
/// Eight. A path reaching deeper than eight segments has stopped naming an item
/// and started describing a tree, and the repair is a re-export at the address
/// rather than a longer spelling at this end.
///
/// The authority and the number are written together in `type_contract.rs`, one
/// row per family, so a family cannot stand on the compile-time ladder while
/// wearing another road's authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeatPathSegmentLimit;

/// The magnitude governing how many coupled seats one published stamp covers.
///
/// # Bounds
///
/// Sixty-four. One publication unit is one artifact landed under one receipt,
/// and the seats it covers are the homes that will invoke it; past sixty-four
/// the unit has stopped being one migration and become two, which is a decision
/// the publication road's own admission rule takes rather than a wider bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeatDeclarationLimit;

// ---------------------------------------------------------------------------
// The declaration refusal family.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// How one declaration of this home's seat vocabulary refuses.
    ///
    /// Dependent checks in a declared order, so exactly one cause is true of any
    /// refused declaration: a path's segments are read before a seat's
    /// spellings, a seat's spellings before a coverage's seats, and a
    /// coverage's seats before its one namespace.
    /// Every one of them refuses before a partial value exists — a coverage
    /// holding some of its seats is a publication unit nobody declared.
    #[must_use = "a declaration refusal names the exact seat the declaration did not fill"]
    pub enum SeatDeclarationRefusal {
        /// The path names no segment at all, so it names no type.
        PathSegmentsAbsent = "path-segments-absent",
            "a rendered type path names no segment";
        /// A spelling the rendering writes as a Rust identifier is not one, so
        /// the emission would write tokens the machine's compiler reads as
        /// something else.
        SpellingNotAnIdentifier = "spelling-not-an-identifier",
            "a rendered spelling is not one Rust identifier";
        /// The path carries more segments than the declared magnitude.
        PathSegmentsUnbounded = "path-segments-unbounded",
            "a rendered type path carries more segments than the declared magnitude";
        /// The coverage declares no seat at all, and a published stamp nobody
        /// invokes is an artifact with no reader.
        SeatsAbsent = "seats-absent",
            "a published stamp's coverage declares no coupled seat";
        /// The coverage declares more seats than the declared magnitude.
        SeatsUnbounded = "seats-unbounded",
            "a published stamp's coverage declares more seats than the declared magnitude";
        /// Two seats of one coverage name one refusal family, so the migration
        /// would seat one family twice.
        FamilySpellingDoubled = "family-spelling-doubled",
            "two seats of one coverage name one refusal family";
        /// Two seats of one coverage name one seat module, so two families
        /// would collide inside a single home as a duplicate definition.
        HomeSpellingDoubled = "home-spelling-doubled",
            "two seats of one coverage name one seat module";
    }
}

// ---------------------------------------------------------------------------
// The seat's declared facts.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// The reach a stamped seat is re-exported at, as the stamp's front door
    /// spells it.
    ///
    /// # Bounds
    ///
    /// The five non-parameterized visibility forms, and no more. A `macro_rules!`
    /// front door cannot transport an opaque `vis` fragment one module deeper, so
    /// every reach it admits is literal front syntax it spells itself — and a
    /// parameterized reach (`pub(in some::path)`) is a change to the front
    /// grammar rather than a value this roster can carry.
    pub enum SeatVisibility {
        /// No visibility token: private to the module the invocation sits in.
        Private = "private",
            "private to the module the invocation sits in";
        /// `pub(self)` — the same reach, spelled.
        SelfReach = "self-reach",
            "private to the module the invocation sits in, spelled";
        /// `pub(super)`.
        SuperReach = "super-reach",
            "visible to the parent of the module the invocation sits in";
        /// `pub(crate)`.
        CrateReach = "crate-reach",
            "visible throughout the machine's own crate";
        /// `pub`.
        PublicReach = "public-reach",
            "visible to every consumer of the machine";
    }
}

threadpak::closed_register! {
    /// The reach a stamped item carries INSIDE the module the stamp seats it in.
    ///
    /// A separate roster from [`SeatVisibility`] because it answers a different
    /// question at a different coordinate: [`SeatVisibility`] is what the caller
    /// WROTE, one module out, and this is what the stamped item wears one module
    /// in. Two rosters rather than one, because the transported set is not the
    /// declared set — nothing lands private, and the parent-facing reach lands as
    /// a parameterized path the declared roster deliberately does not carry.
    pub enum TransportedReach {
        /// `pub(super)` — the reach a private or `pub(self)` declaration becomes.
        SuperReach = "super-reach",
            "reaching out of the seat module to the module the invocation sits in";
        /// `pub(in super::super)` — the reach a `pub(super)` declaration becomes.
        AncestorReach = "ancestor-reach",
            "reaching out of the seat module to the parent of the module the invocation sits in";
        /// `pub(crate)` — absolute, and unchanged by the extra module.
        CrateReach = "crate-reach",
            "visible throughout the machine's own crate";
        /// `pub` — absolute, and unchanged by the extra module.
        PublicReach = "public-reach",
            "visible to every consumer of the machine";
    }
}

threadpak::closed_register! {
    /// Which mint road a stamped seat carries.
    ///
    /// The FORM alone, without the admission profile a minting seat names, so
    /// the stamp definition — which is written once and covers every seat — can
    /// be rendered from it while one seat's own profile stays that seat's fact.
    pub enum SeatMintForm {
        /// The seat carries its two readers and no road that builds a body.
        ReadersOnly = "readers-only",
            "the seat carries its readers and no mint";
        /// The seat carries a mint road under a declared admission profile.
        Minting = "minting",
            "the seat carries a mint road under a declared admission profile";
    }
}

threadpak::closed_register! {
    /// Why neither declaration-site generation nor a core-local stamp can
    /// express one requested output.
    ///
    /// The publication road's admission rule is structural, and this is the
    /// typed statement it reads: publication is lawful only where the output
    /// requires one of these two, and a rendering that names neither has not
    /// earned the road.
    #[must_use = "the ground is what the publication road's admission rule reads"]
    pub enum InsufficiencyGround {
        /// The output is ONE definition read by several files at once.
        /// Declaration-site generation writes tokens where the declaration
        /// stands and nowhere else, so a definition several homes invoke cannot
        /// be expressed by it, and a core-local stamp cannot write itself.
        CrossFileArtifact = "cross-file-artifact",
            "the output is one definition several files must reach";
        /// The output mints identifiers other files name.
        /// A module spelling and an exported macro spelling are names other
        /// homes write down, and neither road can mint a name a second file
        /// then resolves.
        IdentifierMinting = "identifier-minting",
            "the output mints identifiers other files name";
    }
}

/// One type path a rendered seat names, spelled as segments.
///
/// # Bounds
///
/// Structurally non-empty: a path naming no segment names no type. Every
/// segment is a Rust identifier by construction, so a spelling the machine's
/// compiler would read as something else is not a value anybody can hold.
///
/// The path carries no crate binding of its own. A stamped seat is invoked from
/// inside the machine, over types the invoking home declared, so what a segment
/// names is resolved at the invocation the way the home's own source resolves
/// it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeatPath {
    segments: NonEmptyBounded<String, SeatPathSegmentLimit>,
}

/// The two spellings one coupled seat is written under: the refusal family, and
/// the module the seat is seated in.
///
/// # Bounds
///
/// The module spelling is written in `snake_case` by the caller and the stamp
/// never builds one from the other. `macro_rules!` cannot compose an identifier
/// out of another identifier on stable, and a name derived from another name
/// would be this home deciding a spelling law nobody gave it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeatNames {
    family: String,
    home: String,
}

/// The prose one stamped seat carries: the sentence a reader is shown, and the
/// obligation the family states about itself.
///
/// Both are the CALLER's words. The stamp writes the derive set, the seat, the
/// two readers, and the family declaration; what the family means and why
/// holding one obliges a reader are the owner's statements, and a producer that
/// wrote them would be documenting a meaning it does not own.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeatProse {
    /// The sentence the stamped family is documented with.
    pub note: String,
    /// The sentence the family's `must_use` obligation states.
    pub obligation: String,
}

/// Which mint road one declared seat asks for, and the profile a minting one
/// names.
///
/// Not an option: a seat that carries only its readers is a STATED posture — a
/// family whose issues no road in the machine can yet assemble — and a seat that
/// mints names the admission profile its mint stands under. The two never read
/// alike, and neither is a missing profile.
///
/// # Bounds
///
/// A minting seat's family must stand on the compile-time magnitude ladder,
/// because the road the mint calls is declared over it. A family whose capacity
/// its owner's evidence selects has no mint to ask for and declares
/// [`SeatMint::ReadersOnly`], which is the honest posture rather than a road
/// with no callable target.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeatMint {
    /// The seat carries its two readers and nothing that builds a body.
    ReadersOnly,
    /// The seat carries a mint road standing under this admission profile.
    EstablishedUnder(SeatPath),
}

/// Everything one coupled seat is declared from.
///
/// One seat is one collection-shaped refusal family: the family's own spelling,
/// the issue roster it carries, the magnitude that bounds the roster, the module
/// the seat is seated in, the reach it is re-exported at, the mint road it asks
/// for, and the prose it is documented with.
///
/// # Authority
///
/// **The private seat is what the declaration buys.** A body declared beside the
/// rest of a home's types is private to that whole file, so every other type,
/// function, and implementation beside it can write the body's field — and
/// whether one of them does is a whole-file audit. The stamped seat lands in a
/// module of its own whose entire content is the stamp's output, so the complete
/// set of roads to the body is the set the stamp writes, and it is the machine's
/// own compiler that establishes it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CoupledSeatDeclaration {
    names: SeatNames,
    issue: SeatPath,
    bound: SeatPath,
    reach: SeatVisibility,
    mint: SeatMint,
    prose: SeatProse,
}

/// The name one published stamp is exported under.
///
/// # Authority
///
/// **The spelling is the caller's and is never mangled.** A published artifact
/// is git-visible source a human commits under a receipt and other homes invoke
/// by name, so a content-addressed spelling would put a name nobody can read at
/// the root of the machine's own crate. The uniqueness the exported macro
/// namespace needs is the caller's to keep, exactly as it is for every other
/// stamp the machine already carries.
///
/// # Bounds
///
/// The spelling is one Rust identifier by construction, so a name the machine's
/// compiler would read as something else is not a value anybody can hold.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StampName {
    spelling: String,
}

/// The complete declared payload one published stamp is rendered from: the name
/// it is exported under, and every coupled seat it covers.
///
/// # Bounds
///
/// Structurally non-empty and bounded, and the seats' two namespaces are closed
/// at declaration: two seats naming one refusal family would seat that family
/// twice, and two seats naming one module would collide inside a home as a
/// duplicate definition. Both are refused here rather than left to the machine's
/// own compiler, which would report a collision inside an expansion nobody
/// wrote.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StampCoverage {
    stamp: StampName,
    seats: NonEmptyBounded<CoupledSeatDeclaration, SeatDeclarationLimit>,
}

// ---------------------------------------------------------------------------
// What the plan decided, read off its own surface.
// ---------------------------------------------------------------------------

/// What a pattern-stamp plan decided about the artifact it will publish, read
/// off the plan's own public surface.
///
/// Every seat is public and required, because a statement that could omit its
/// engine, its declaration, or the byte role its artifact is written under would
/// be an account that sometimes says less than it knows. There is no private
/// field here and this home's invariant nucleus holds nothing of it.
///
/// # Nonclaims
///
/// Holding one claims that these are the facts the plan carries under its one
/// rendered role, and nothing about whether anything was rendered.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StampedUnitPlan {
    /// The rendered role the artifact stands for.
    pub role: SoleRenderedUnit,
    /// The planned member's semantic key, exactly as the plan declared it.
    pub semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    /// The byte role the artifact is written under.
    pub byte_role: OwnerIdentityRef<ByteRoleSubject>,
    /// The profile the plan expects to render it.
    pub profile: ProjectionIdentity<ProjectionProfileSubject>,
    /// That profile's version.
    pub profile_version: ProfileVersion,
    /// The member's origin trail, walked back to authored material.
    pub origin: OriginTrail,
    /// What the eventual staged bytes' digest must satisfy.
    pub digest_contract: DigestContract,
    /// The ONE address the entry account walked in the door carrying.
    pub declaration: CauseAnchoring,
    /// The rendering engine the artifact is written by.
    pub engine: ProjectionIdentity<GeneratorVersionSubject>,
    /// The authored pattern this artifact instantiates.
    pub pattern: OwnerIdentityRef<PatternSubject>,
    /// This instantiation of it.
    pub instance: OwnerIdentityRef<PatternInstanceSubject>,
}

/// How reading a plan into [`StampedUnitPlan`] disagrees with the plan.
///
/// No issue is payload-free: an issue names the role it is about, because a
/// caller told only that the reading failed has nothing to repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StampedUnitPlanIssue {
    /// The plan declares no member under its kind's one rendered role, so there
    /// is no artifact to render.
    RoleNotPlanned {
        /// The role's position in its kind's declared roster.
        role_slot: u32,
    },
    /// The planned member lands at the declaration site.
    /// A published stamp is a standalone artifact written under a byte role and
    /// committed by a human; a member spliced into the declaration it came from
    /// is a different delivery, and it is the delivery that needs no publication
    /// road at all.
    DestinationNotArtifact {
        /// The role whose planned destination disagreed.
        role_slot: u32,
    },
}

// ---------------------------------------------------------------------------
// The rendering refusal.
// ---------------------------------------------------------------------------

/// How rendering the published stamp disagrees with what the token vocabulary
/// can spell.
///
/// # Authority
///
/// **The one issue is a fact about the token MAGNITUDE and never about a seat.**
/// A seat that could not be declared was refused at the door by
/// [`SeatDeclarationRefusal`]; every spelling the emission needs — paths, words,
/// punctuation, groups, and the two prose literals — has an arm on the generated
/// token roster, so the only way a rendering fails is that the artifact outgrew
/// what one tree holds.
#[must_use = "a rendering refusal names the declared magnitude the artifact outgrew"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StampRenderIssue {
    /// The rendered tree outgrows the declared token magnitude.
    /// The artifact carries the stamp's whole front grammar and one invocation
    /// per covered seat, and it refuses rather than materializing part of one.
    StampTreeUnbounded {
        /// The declared bound.
        bound: u64,
    },
}

// ---------------------------------------------------------------------------
// The published artifact and the record it pairs with.
// ---------------------------------------------------------------------------

/// One row of the coverage a published stamp states about itself: the refusal
/// family the stamp seats, and the module that seat lands in.
///
/// # Bounds
///
/// It is a projection of one [`CoupledSeatDeclaration`] and never a second
/// declaration: the spellings are read off the declaration the artifact was
/// rendered from, so a manifest row and the seat it is about cannot disagree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StampedSeat {
    family: String,
    home: String,
}

/// This side's record of one publication act, in the machine receipt's field
/// shape.
///
/// # Authority
///
/// **It is a statement and never a receipt.** The machine's evidence home owns
/// the publication receipt, its publication-unit commitment, its staged-output
/// evidence, and its independent manifest. This value answers the same three
/// questions in the vocabulary these services own — which unit, what its staged
/// bytes must satisfy, and what the unit contains — so the publication operation
/// compares its own independently built answers against these. A value minted
/// here for the other side would make that comparison compare a value with
/// itself, which detects nothing at any cost.
///
/// The ground rides beside the three because it is what the publication road's
/// structural admission rule reads: the road is lawful only where the requested
/// output requires a cross-file artifact or identifier minting, and the record
/// is where the rendering states which.
///
/// # Nonclaims
///
/// It claims nothing about whether the artifact was staged, checked, or
/// published, and nothing about whether a human committed it. Those are acts
/// this side does not perform and cannot witness.
/// # Bounds
///
/// The manifest is read off the COVERAGE the artifact was rendered from rather
/// than kept as a second list beside it. A second list is a value that agrees
/// with the coverage until it does not, and nothing downstream could tell which
/// of the two the artifact actually stamps.
#[must_use = "the record is what the publication road's admission rule is satisfied from"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StampPublicationRecord {
    ground: InsufficiencyGround,
    unit: ProjectionIdentity<GeneratedUnitSubject>,
    staged: DigestContract,
    coverage: StampCoverage,
}

/// One covered seat's landing: the seat, and the invocation the migration writes
/// at that seat's home.
///
/// The two travel together because they are one fact about one seat. An
/// invocation without the seat it stands for cannot be placed, and a seat
/// without its invocation is a home the published stamp never reaches.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeatSeating {
    seat: StampedSeat,
    invocation: GeneratedTree,
}

/// The published stamp: the engine-authored shell the publication road lands in
/// the machine as visible source, and every seating it is landed for.
///
/// A rendered `macro_rules!` definition whose expansion is the whole coupled-seat
/// module — the family's private seat, its readers, its mint road where one was
/// declared, and its family declaration — so the machine's own privacy wall is
/// what keeps the seat rather than a reader's audit of the file it landed in.
///
/// # Authority
///
/// **What is emitted is DECLARATIVE and calls nothing.** The definition's body
/// is self-contained tokens: it names the machine's own vocabulary through
/// `$crate` and names these services nowhere. A stamp inside the machine cannot
/// be a live caller of these services — the machine carries no dependency edge
/// to them — so what the road produces is a shell proven equivalent to the
/// engine's own entrance, never a call into it.
///
/// # Bounds
///
/// Nothing here writes to disk. These are rendered trees and the record that
/// pairs with them; the publication operation is what stages, checks, and lands
/// them, and a human is what commits them.
///
/// The seatings are exactly as many as the record's coverage declared, because
/// the one road that builds them walks that coverage — so the count is the
/// coverage's fact and not a second magnitude, and the exported name is read out
/// of the coverage rather than kept beside it.
#[must_use = "a published stamp is the artifact the publication road lands under a receipt"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublishedSeatStamp {
    role: SoleRenderedUnit,
    semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    byte_role: OwnerIdentityRef<ByteRoleSubject>,
    profile: ProjectionIdentity<ProjectionProfileSubject>,
    profile_version: ProfileVersion,
    origin: OriginTrail,
    definition: GeneratedTree,
    seatings: Vec<SeatSeating>,
    record: StampPublicationRecord,
}
