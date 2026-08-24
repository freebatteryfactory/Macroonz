//! The stamp home's declarations: the pattern a caller authors, the sites that adopt it, the artifact both are rendered into, and how stamping refuses.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is what makes the identifier alphabet and the two closed namespaces structural rather than remembered.

use crate::bounded::{Bounded, NonEmpty, Overflow};
use crate::identity::{self, Identity};
use crate::plan::DigestContract;
use crate::token::GeneratedTree;

#[path = "type_guard.rs"]
mod guard;

/// Segments one spelled path may carry.
///
/// A path reaching deeper than eight segments has stopped naming an item and started describing a tree, and the repair is a re-export at the address rather than a longer spelling at this end.
pub const PATH_SEGMENT_LIMIT: usize = 8;

/// Parts one pattern may declare, and therefore the most arguments one site can carry.
pub const PART_LIMIT: usize = 64;

/// Sites one published stamp may cover.
///
/// One publication unit is one artifact landed at one address, and past sixty-four the unit has stopped being one migration and become two.
pub const SITE_LIMIT: usize = 64;

crate::roster! {
    /// Which fragment of Rust one seat matches.
    ///
    /// The language's own roster, minus a visibility: a reach is declared at a coordinate rather than captured as a fragment, because a captured one cannot be transported one module deeper.
    pub enum Fragment {
        /// A whole item.
        Item = "item",
        /// A braced block.
        Block = "block",
        /// One statement.
        Statement = "stmt",
        /// One expression.
        Expression = "expr",
        /// One match pattern.
        MatchPattern = "pat",
        /// One type.
        Type = "ty",
        /// One identifier.
        Identifier = "ident",
        /// One path.
        Path = "path",
        /// One lifetime.
        Lifetime = "lifetime",
        /// One literal.
        Literal = "literal",
        /// The body of one attribute.
        Attribute = "meta",
        /// One token tree, whatever it is.
        Tokens = "tt",
    }
}

crate::roster! {
    /// The reach one stamped item is written at, as the site spells it.
    ///
    /// The five non-parameterized forms and no more: a front door spells every reach it admits as literal syntax, so a parameterized reach is a change to that grammar rather than a value this roster can carry.
    pub enum Visibility {
        /// No visibility token: private to the module the site sits in.
        Private = "private",
        /// `pub(self)` — the same reach, spelled.
        Module = "module",
        /// `pub(super)`.
        Parent = "parent",
        /// `pub(crate)`.
        Crate = "crate",
        /// `pub`.
        Public = "public",
    }
}

crate::roster! {
    /// The reach one stamped item carries inside the module a pattern seats it in.
    ///
    /// A separate roster from [`Visibility`] because it answers a different question at a different coordinate: one is what the site wrote, one module out, and this is what the stamped item wears one module in.
    /// Nothing transports to a private reach — an item that landed private inside the seat module could not be re-exported out of it, and the site's own coordinate would name nothing.
    pub enum TransportedReach {
        /// `pub(super)` — out of the seat module to the module the site sits in.
        Enclosing = "enclosing",
        /// `pub(in super::super)` — out to the parent of the module the site sits in.
        Ancestor = "ancestor",
        /// `pub(crate)` — absolute, and unmoved by the extra module.
        Crate = "crate",
        /// `pub` — absolute, and unmoved by the extra module.
        Public = "public",
    }
}

crate::roster! {
    /// Why neither a splice at the declaration nor a hand-written definition expresses one requested output.
    ///
    /// A publication road is lawful only where the output requires one of these two, and a rendering that names neither has not earned the road.
    pub enum PublicationGround {
        /// The output is one definition several files must reach.
        CrossFileArtifact = "cross-file-artifact",
        /// The output mints identifiers other files name.
        IdentifierMinting = "identifier-minting",
    }
}

/// How one seat is matched, and how an expansion writes it back.
///
/// Three shapes, because three is what a matcher can express about one metavariable: one of something, any number of them, or any number of attributes written over the item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Seating {
    /// One fragment of the declared kind.
    One(Fragment),
    /// Any number of fragments of the declared kind, separated by commas.
    Many(Fragment),
    /// Any number of attributes, each written over the stamped item.
    Attributes,
}

/// One metavariable seat: the name material travels under, and the shape it travels in.
///
/// The name is the caller's and is never composed from another: a matcher cannot build an identifier out of an identifier, and a derived name would be this home deciding a spelling law nobody gave it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Seat {
    name: String,
    seating: Seating,
}

/// One part of a pattern's declared shape.
///
/// A pattern is a sequence of these, and both halves of the grammar are walks over it: a matcher reads a seat as a metavariable and a site writes its own material there, while literal parts are the same tokens on both sides.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Part {
    /// Token material every matcher reads and every site writes unchanged.
    Literal(GeneratedTree),
    /// One seat.
    Seat(Seat),
    /// The coordinate the site's visibility is written at, which a pattern seating nothing behind a module of its own declares none of.
    Reach,
}

/// One authored pattern: what the definition is documented with, the shape it is invoked in, and the body that shape expands into.
///
/// # Authority
///
/// **The body is the caller's token material and nothing here reads it.**
/// It names seats by the names the shape declares them under, and the two reaches by the names this home publishes.
/// A body naming anything else is a defect the consumer's own compiler reports at the site that adopted it, because a producer that checked it would be legislating a meaning it does not own.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pattern {
    note: String,
    parts: NonEmpty<Part, PART_LIMIT>,
    body: GeneratedTree,
}

/// The name one published stamp is exported under.
///
/// The spelling is the caller's and is never mangled: a published artifact is visible source a person commits and other files invoke by name, so the uniqueness an exported macro namespace needs is the caller's to keep.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StampName {
    spelling: String,
}

/// The path one site reaches its published stamp by.
///
/// Structurally non-empty and every segment an identifier, so a root the consumer's compiler would read as something else is not a value anybody can hold.
/// Usually one segment: the crate the stamp is published in names its own root, and a site elsewhere names that crate.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SiteRoot {
    segments: NonEmpty<String, PATH_SEGMENT_LIMIT>,
}

/// One site that adopts a stamp: what the manifest calls it, how it reaches the definition, the reach it writes, and one argument per declared seat.
///
/// The arguments are as many as the pattern declares seats, settled where the site meets the pattern rather than counted a second time here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Site {
    name: String,
    root: SiteRoot,
    reach: Visibility,
    arguments: Bounded<GeneratedTree, PART_LIMIT>,
}

/// The complete declared payload one published stamp is rendered from: the name it is exported under, the pattern it stamps, and every site that adopts it.
///
/// # Bounds
///
/// Structurally non-empty — a definition nobody invokes is an artifact with no reader — and the site namespace is closed at declaration, because two rows naming one site is a manifest that says one thing twice.
/// Every site's arguments are settled against the pattern here rather than left to the consumer's compiler, which would report a mismatch inside an expansion nobody wrote.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stamp {
    name: StampName,
    pattern: Pattern,
    sites: NonEmpty<Site, SITE_LIMIT>,
}

/// What a plan decided about the artifact one stamp publishes, read off the plan's own surface.
///
/// Both seats are required, and holding one says nothing about whether anything was rendered, staged, or landed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StampedPlan {
    /// The planned member's semantic key.
    pub unit: Identity<identity::GeneratedUnit>,
    /// What the eventual staged bytes' digest must satisfy.
    pub staged: DigestContract,
}

/// This side's record of one publication act.
///
/// # Authority
///
/// **It is a statement and never a receipt.**
/// It answers which unit the artifact materializes, what its staged bytes must satisfy, why the road is lawful at all, and what the unit contains — so a publication step compares its own independently built answers against these rather than a value with itself.
///
/// The manifest is read off the stamp the artifact was rendered from, never kept as a second list that agrees with it until it does not.
#[must_use = "the record is what a publication road's admission rule is satisfied from"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicationRecord {
    ground: PublicationGround,
    unit: Identity<identity::GeneratedUnit>,
    staged: DigestContract,
    stamp: Stamp,
}

/// One covered site's landing: the site, and the invocation written there.
///
/// The two travel together because they are one fact about one site.
/// An invocation without the site it stands for cannot be placed, and a site without its invocation is a file the published stamp never reaches.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Landing {
    site: String,
    invocation: GeneratedTree,
}

/// The published stamp: the definition a publication road lands as visible source, and every landing it is landed for.
///
/// # Authority
///
/// **What is emitted is declarative and calls nothing.**
/// The definition's body is the caller's self-contained token material and this compiler is named nowhere in it, which is why a stamp can stand in a crate that carries no edge back here.
///
/// # Bounds
///
/// Nothing here writes to disk: these are rendered trees and the record that pairs with them.
/// The landings are as many as the record's stamp declares sites, because the one road that builds them walks that stamp, and the exported name is read out of the record rather than kept beside it.
#[must_use = "a published stamp is the artifact a publication road lands under a record"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublishedStamp {
    definition: GeneratedTree,
    landings: Vec<Landing>,
    record: PublicationRecord,
}

/// How stamping says no.
///
/// One cause per refusal: every road here settles one question, and the road that settles several settles them in a declared order, so exactly one row is true of any refused value.
#[must_use = "a stamping refusal names the exact thing the declaration did not settle"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StampError {
    /// A spelling written as a Rust identifier is not one.
    NotAnIdentifier,
    /// A path names no segment, so it names nothing.
    PathEmpty,
    /// A path carries more segments than the declared magnitude.
    PathUnbounded {
        /// The magnitude and what was offered.
        overflow: Overflow,
    },
    /// A pattern declares no part, so it declares no shape.
    PatternEmpty,
    /// A pattern declares more parts than the declared magnitude.
    PatternUnbounded {
        /// The magnitude and what was offered.
        overflow: Overflow,
    },
    /// Two seats of one pattern carry one name, which binds one metavariable twice.
    SeatNameDoubled {
        /// The doubling part's position in the pattern.
        at: u32,
    },
    /// A stamp covers no site, and a definition nobody invokes has no reader.
    SitesAbsent,
    /// A stamp covers more sites than the declared magnitude.
    SitesUnbounded {
        /// The magnitude and what was offered.
        overflow: Overflow,
    },
    /// Two sites of one stamp carry one name, which is a manifest row written twice.
    SiteNameDoubled {
        /// The doubling site's position in the stamp.
        at: u32,
    },
    /// One site carries more arguments than the declared magnitude.
    ArgumentsUnbounded {
        /// The magnitude and what was offered.
        overflow: Overflow,
    },
    /// One site supplies a different number of arguments than the pattern declares seats.
    ArgumentsUnmatched {
        /// The site's position in the stamp.
        at: u32,
        /// How many seats the pattern declares.
        seats: u32,
        /// How many arguments the site supplied.
        supplied: u32,
    },
    /// One site declares a reach the pattern gives no coordinate to, which is a visibility with nowhere to be written rather than a narrower one.
    ReachUnseated {
        /// The site's position in the stamp.
        at: u32,
    },
    /// The plan declares no member under the seat this artifact stands for.
    SeatNotPlanned {
        /// The seat's position in its kind's roster.
        role_slot: u16,
    },
    /// The planned member lands somewhere other than a standalone artifact, and a published stamp is bytes at an address the other three deliveries do not name.
    DestinationNotArtifact {
        /// The seat's position in its kind's roster.
        role_slot: u16,
    },
    /// A rendered tree outgrows the declared token magnitude, and the artifact refuses whole rather than materializing the sites that happened to fit.
    TokensUnbounded {
        /// The magnitude and what was offered.
        overflow: Overflow,
    },
}
