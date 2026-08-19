//! The documentation home's declarations: the owner's own doc text, the closed
//! roster of typed facts a rendered sentence may trace to, the earned section, the
//! documented item a caller supplies, what a plan decided, the anchors the
//! explanation station's four unheld facts arrive as, the rendered surface, and
//! the magnitudes and refusal families this home answers through.
//!
//! Declarations only.
//! Every road that reaches a private field — a sentence's text, a line's text, a
//! section's lines, an item's sections, the surface's composition, and the
//! refusal body's one seat — lives in `type_guard.rs`, this file's own child.
//! That is what makes the one-plain-sentence law STRUCTURAL: the only road to a
//! summary refuses an empty one, a multi-line one, an unfinished one, and one
//! past the declared magnitude, so a documented item carrying a paragraph where
//! its summary belongs is a value nobody can write.
//!
//! # Never invented prose
//!
//! [`SectionLine`] is a sum of exactly two arms — the owner's own authored text,
//! and one typed fact this home renders from typed values. A line tracing to
//! neither is unrepresentable rather than reviewed, which is the whole of what
//! this home's prose discipline is.
//!
//! # The prose is not in the plan, and that is why it arrives from the caller
//!
//! The plan's kind content names a SUBJECT, an AUDIENCE, and the FACETS covered.
//! It carries no sentence, no heading, and no line — so [`DocumentedItem`]
//! arrives from the caller, exactly as the descriptor rows do at the wall, and
//! `plan.rs` reads only what the plan actually decided.

use crate::diagnostics::RepairAction;
use crate::origin_graph::OriginTrail;
use crate::plane::{
    DocumentedSubject, FacetLimit, GeneratedUnitSubject, GeneratorVersionSubject,
    OutputBytesSubject, OwnerFactRef, OwnerIdentityRef, ProfileVersion, ProjectionIdentity,
    ProjectionKindSubject, ProjectionProfileSubject, RepairLimit, SoleRenderedUnit,
};
use crate::planning::{CauseAnchoring, GraphAnchoring, ProjectionDisposition};
use crate::token::GeneratedTree;
use threadpak::declaration::Facet;
use threadpak::declaration::types::ProjectionAudienceDomain;
use threadpak::types::{Bounded, NonEmptyBounded};

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
    /// The magnitude governing how many earned sections one documented item may
    /// carry.
    ///
    /// # Bounds
    ///
    /// Six — the machine's facet roster's own cardinality, because a section is
    /// EARNED by one facet and one facet earns at most one section. It is not a
    /// number this home chose out of taste: a seventh section would have to be
    /// earned by a seventh facet, and the machine declares six.
    ///
    /// # Nonclaims
    ///
    /// It is not the plane's [`FacetLimit`], which governs how many facets a
    /// PLAN may cover. This one governs how many sections an ITEM may carry, and
    /// one family standing for both would be one authority answering two
    /// questions — even where the two numbers agree today. The plan's question is
    /// asked by more than one home and lives on the plane's rows; an item's
    /// sections are this home's question alone.
    DocumentationSectionLimit = 6,
    /// The magnitude governing how many lines one earned section may carry.
    ///
    /// # Bounds
    ///
    /// Thirty-two. A section past thirty-two lines has stopped explaining one
    /// facet and started being a document, and the repair is prose the owner
    /// keeps somewhere a reader can navigate rather than a longer run of doc
    /// attributes on one item.
    DocumentationLineLimit = 32,
    /// The magnitude governing how many bytes one piece of owner doc text may
    /// carry.
    ///
    /// # Bounds
    ///
    /// Five hundred and twelve. One summary sentence and one section line are
    /// both single LINES, and a line past five hundred and twelve bytes is a
    /// paragraph that lost its line breaks — which the door already refuses for a
    /// different reason. The two checks stand together: no line breaks, and not
    /// this long.
    DocumentationTextLimit = 512,
    /// The magnitude governing how many issues one documentation-coverage
    /// refusal body may carry.
    ///
    /// # Bounds
    ///
    /// Eighteen. The coverage pass asks two independent questions of every
    /// covered facet — whether a section was written for it, and whether two
    /// were — and one question of every declared section, whether the plan
    /// covers the facet it names. The facet roster is six and the section roster
    /// is bounded by the same six, so eighteen issues can hold at once and no
    /// more.
    ///
    /// The plan pass does not add to it: reading the plan and covering its
    /// facets are dependent, and there is nothing to cover until the plan has
    /// been read.
    DocumentationIssueLimit = 18,
}

// ---------------------------------------------------------------------------
// The declaration refusal family.
// ---------------------------------------------------------------------------

threadpak::closed_register! {
    /// How one declaration of this home's vocabulary refuses.
    ///
    /// Dependent checks in a declared order, so exactly one cause is true of any
    /// refused declaration: a text's characters are read before a sentence's
    /// ending, a sentence's ending before a section's lines, and a section's
    /// lines before an item's sections.
    /// Every one of them refuses before a partial value exists — a section
    /// holding some of its lines is prose the owner did not write.
    #[must_use = "a declaration refusal names the exact seat the declaration did not fill"]
    pub enum DocumentationDeclarationRefusal {
        /// The text carries no character at all, so it says nothing.
        EmptyText = "empty-text",
            "a piece of owner doc text carries no character";
        /// The text carries a line break.
        ///
        /// A summary and a section line are both single LINES: each is rendered
        /// as one doc attribute, and a break inside one would silently become a
        /// paragraph boundary in the rendered documentation that nobody wrote.
        TextCarriesLineBreak = "text-carries-line-break",
            "a piece of owner doc text carries a line break";
        /// The text carries more bytes than the declared magnitude.
        TextUnbounded = "text-unbounded",
            "a piece of owner doc text carries more bytes than the declared magnitude";
        /// The summary does not end in a full stop, so it is a fragment rather
        /// than the one plain SENTENCE the law asks for.
        SentenceNotEnded = "sentence-not-ended",
            "a summary does not end in a full stop";
        /// The section carries no line at all, and a heading over nothing
        /// explains nothing.
        LinesAbsent = "lines-absent",
            "an earned section carries no line";
        /// The section carries more lines than the declared magnitude.
        LinesUnbounded = "lines-unbounded",
            "an earned section carries more lines than the declared magnitude";
        /// The item carries more sections than the declared magnitude.
        SectionsUnbounded = "sections-unbounded",
            "a documented item carries more sections than the declared magnitude";
    }
}

// ---------------------------------------------------------------------------
// The owner's own doc text.
// ---------------------------------------------------------------------------

/// The ONE plain sentence a documented item opens with.
///
/// # Authority
///
/// **It is the owner's text, carried, and never composed here.** The only road to
/// one takes a string the owner wrote and refuses it four ways; there is no road
/// that builds a sentence out of typed values, because a summary this home
/// composed would be a claim about the owner's item that the owner did not make.
///
/// # Bounds
///
/// Not empty, no line break, ending in a full stop, and inside the declared
/// magnitude — the whole of the one-plain-sentence law, settled at the door and
/// never re-checked downstream.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlainSentence {
    text: String,
}

/// One line of the owner's own doc text, inside an earned section or standing as
/// its heading.
///
/// # Bounds
///
/// Not empty, no line break, and inside the declared magnitude. It carries no
/// full-stop law, because a line inside a section may lawfully be a list item, a
/// table row, or a fenced code line — none of which is a sentence, and all of
/// which the owner is entitled to write.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthoredLine {
    text: String,
}

// ---------------------------------------------------------------------------
// The typed facts a rendered sentence may trace to.
// ---------------------------------------------------------------------------

/// One typed fact a rendered line may stand for.
///
/// A closed roster of exactly five, and every arm carries the typed values its
/// sentence is composed from — never a sentence somebody supplied. What each arm
/// traces to is stated once, as `FACT_ROSTER` in `type_contract.rs`.
///
/// # Authority
///
/// **A fact is rendered from values and never carried as text.** The rendering is
/// composed at the moment it is asked for and is never stored, so a documented
/// item whose sentence contradicts its typed fact is not a value anybody can
/// build — the same shape the explanation station's own answers stand under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentedFact {
    /// The projection kind's declared stable name.
    ProjectionKindName {
        /// The kind's declared name, as the kind itself declares it.
        name: &'static str,
    },
    /// The ONE address the entry account walked in the door carrying.
    CausingDeclaration {
        /// The anchored cause address.
        anchoring: CauseAnchoring,
    },
    /// The semantic key the plan declared for the member this documents.
    OutputIdentity {
        /// The planned member's semantic key.
        key: ProjectionIdentity<GeneratedUnitSubject>,
    },
    /// One owner fact this documentation rests on.
    Assumption {
        /// The cited fact.
        fact: OwnerFactRef,
    },
    /// One facet the plan covers.
    ///
    /// # Bounds
    ///
    /// This arm REFUSES at rendering time, and the refusal is the honest answer
    /// rather than a gap: the machine's facet roster declares no stable name and
    /// no described projection, so a sentence naming a facet would be these
    /// services legislating a spelling inside a vocabulary the machine owns. The
    /// seat that closes it is a declared name on the machine's own roster.
    ///
    /// The coverage law is untouched by that, and deliberately: it matches facets
    /// by identity and never by spelling.
    CoveredFacet {
        /// The facet this line was to be about.
        facet: Facet,
    },
}

/// One line of an earned section: the owner's own text, or one typed fact.
///
/// # Authority
///
/// **There is no third arm, and that absence is the prose discipline.** Every
/// rendered sentence traces to the owner's own words or to a typed fact rendered
/// from typed values, so a line tracing to neither is unrepresentable rather than
/// caught in review.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SectionLine {
    /// The owner's own line, written out unchanged.
    Authored(AuthoredLine),
    /// One typed fact, rendered from its values when the line is asked for.
    Fact(DocumentedFact),
}

/// One earned section: the facet that earns it, the owner's own heading, and the
/// lines under it.
///
/// # Bounds
///
/// The lines are structurally non-empty: a heading over nothing explains nothing,
/// and a section that could be empty is a section a coverage pass would count as
/// written while a reader saw nothing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentedSection {
    facet: Facet,
    heading: AuthoredLine,
    lines: NonEmptyBounded<SectionLine, DocumentationLineLimit>,
}

/// The complete documentation material one item carries.
///
/// # Bounds
///
/// The sections may be EMPTY, and an empty roster is a stated fact rather than a
/// missing one: an item whose plan covers no facet owes exactly its one plain
/// sentence, and that item is fully documented.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentedItem {
    summary: PlainSentence,
    sections: Bounded<DocumentedSection, DocumentationSectionLimit>,
}

// ---------------------------------------------------------------------------
// What the plan decided, and what it does not hold.
// ---------------------------------------------------------------------------

/// What a documentation plan decided, read off the plan's own public surface.
///
/// Every seat is public and required, because a statement that could omit its
/// engine, its declaration, or its audience would be an account that sometimes
/// says less than it knows. There is no private field here and this home's
/// invariant nucleus holds nothing of it.
///
/// # Nonclaims
///
/// Holding one claims that these are the facts the plan carries under its kind's
/// one rendered role, and nothing about whether anything was rendered.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentationPlan {
    /// The rendered role the material stands for.
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
    /// What the plan was decided against.
    ///
    /// Carried because the explanation station asks for it beside the profile,
    /// and the two are one answer.
    pub graph: GraphAnchoring,
    /// The rendering engine the material is written by.
    pub engine: ProjectionIdentity<GeneratorVersionSubject>,
    /// The subject documented.
    ///
    /// # Bounds
    ///
    /// It reaches no rendered token. The subject names WHAT is documented and the
    /// item carries what is SAID about it, and a rendering that spelled the
    /// subject's identity into prose would be answering the reader's question with
    /// thirty-two bytes. It travels for a caller joining the material back to the
    /// declaration it answers to — and one typed fact arm does render an identity,
    /// on the owner's own request, which is a different act.
    pub subject: OwnerIdentityRef<DocumentedSubject>,
    /// The audience the projection is written for.
    ///
    /// # Bounds
    ///
    /// It reaches no rendered token either. Which audience a piece of prose is
    /// pitched at is a fact about the prose the OWNER wrote, and this home neither
    /// checks that the prose suits the audience nor renders the audience's name.
    pub audience: OwnerIdentityRef<ProjectionAudienceDomain>,
    /// The facets covered — the quantifier the earned sections are checked
    /// against, in both directions.
    pub facets: Bounded<Facet, FacetLimit>,
}

/// The four facts the explanation station asks for that a plan does not hold.
///
/// # Authority
///
/// **Every seat is required and none is minted here.** The kind's own identity is
/// the machine's; the requiring owner fact is the owner's; the digest is what the
/// closure proved over bytes that exist, and deriving one here would be a fact
/// about bytes nobody has produced; and the related kind's disposition beside the
/// repairs a refusal offers are decided where dispositions and repairs are
/// decided. A road that invented any of them would be answering the station with
/// a value nobody computed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentationExplanationAnchors {
    /// This projection kind's own identity.
    pub kind: ProjectionIdentity<ProjectionKindSubject>,
    /// The owner fact that required this projection.
    pub owner: OwnerFactRef,
    /// The digest the closure proved over the bytes actually rendered.
    pub digest: ProjectionIdentity<OutputBytesSubject>,
    /// The related kind the disposition is about.
    pub related: ProjectionIdentity<ProjectionKindSubject>,
    /// What happened to that related projection.
    pub disposition: ProjectionDisposition,
    /// The owner-declared repairs a refusal offers.
    pub repairs: Bounded<RepairAction, RepairLimit>,
}

/// The rendered documentation surface's typed description.
///
/// # Bounds
///
/// There is no destination seat: a documentation projection's material is spliced
/// ahead of the owner's own item, so the answer is a constant
/// ([`DocumentedSurface::DESTINATION`]) rather than a seat that could say
/// something else.
///
/// The remaining seats are exactly what a rendered unit is rebuilt from — role,
/// semantic key, profile at its version, origin trail, and the tree.
///
/// # Nonclaims
///
/// The tree is the doc material an item CARRIES and never the item. A projection
/// that emitted the item would be a second declaration of it, and the plan
/// declares one member at the declaration site rather than two.
#[must_use = "a documented surface is the doc material one declared item carries"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentedSurface {
    role: SoleRenderedUnit,
    semantic_key: ProjectionIdentity<GeneratedUnitSubject>,
    profile: ProjectionIdentity<ProjectionProfileSubject>,
    profile_version: ProfileVersion,
    origin: OriginTrail,
    tree: GeneratedTree,
}

// ---------------------------------------------------------------------------
// The coverage refusal family.
// ---------------------------------------------------------------------------

/// How composing documentation disagrees with the plan, with the facet roster, or
/// with what this home can spell.
///
/// No issue is payload-free: an issue names the role, the facet, or the bound it
/// is about, because a caller told only that the composition failed has nothing to
/// repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentationIssue {
    /// The plan declares no member under its kind's one rendered role, so there
    /// is no material to render.
    RoleNotPlanned {
        /// The role's position in its kind's declared roster.
        role_slot: u32,
    },
    /// The planned member lands somewhere other than the declaration site.
    ///
    /// Doc material is an attribute run spliced ahead of the owner's own item, so
    /// it belongs in the tokens the consumer's normal build compiles.
    /// The destination roster names four deliveries, and a member that is not at
    /// the declaration site declared one of the other three: a standalone
    /// artifact a publication writes to its own address, the deferred cargo a
    /// test target invokes, or the deferred cargo a bench target invokes. Each
    /// of the three is a different delivery and each establishes this issue.
    DestinationNotDeclarationSite {
        /// The role whose planned destination disagreed.
        role_slot: u32,
    },
    /// The plan covers this facet and the item writes no section for it.
    /// The facet roster is the quantifier: a covered facet with nothing said
    /// about it is a coverage claim the material does not keep.
    CoveredFacetNotWritten {
        /// The facet nobody wrote.
        facet: Facet,
    },
    /// The item writes a section for a facet the plan does not cover, so the
    /// section stands on a coverage claim nobody planned.
    SectionFacetNotCovered {
        /// The facet the section named.
        facet: Facet,
    },
    /// Two sections of one item are earned by one facet, so the material says two
    /// things under one coverage claim and a reader cannot tell which was meant.
    FacetSectionDoubled {
        /// The doubled facet.
        facet: Facet,
    },
    /// A line asked for the facet's NAME and the machine's roster declares none.
    ///
    /// The seat that closes it is a declared stable name on the machine's facet
    /// roster, the way every other roster this crate speaks declares one.
    /// Rendering the Rust spelling instead is NOT the repair: a spelling taken
    /// from a variant's name renames the prose whenever somebody refactors, which
    /// is exactly what a declared name exists to prevent.
    FacetNameNotDeclared {
        /// The facet whose name could not be written.
        facet: Facet,
    },
    /// The rendered material outgrows the declared token magnitude.
    DocumentationTreeUnbounded {
        /// The declared bound.
        bound: u64,
    },
}

/// The documentation-coverage refusal family body, published from this file and
/// DECLARED in `type_guard.rs`'s `seat` module, beside the only roads that reach
/// its seat.
///
/// The declaration is not here because Rust's privacy is MODULE-scoped: a seat
/// declared beside the rest of this home's declarations would put all of them
/// inside the same wall.
pub use guard::DocumentationCoverage;
