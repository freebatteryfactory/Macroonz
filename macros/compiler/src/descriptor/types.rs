//! The descriptor home's shared declarations: the names a descriptor grammar spells, the crate bindings a rendered path is rooted at, how a declaration's values refuse, how a helper body is not read, and the composition root.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child.
//!
//! The three kinds themselves are declared beside their own grammars, in `trial/`, `bench/`, and `mutation/`.

use crate::bounded::{Capped, NonEmpty};
use crate::identity::{OwnerFact, OwnerIdentity};
use crate::token::SpanHandle;

#[path = "type_guard.rs"]
mod guard;

/// The one alphabet every spelling any grammar here renders in identifier position is admitted by, published from the nucleus every road already reads it through.
pub use guard::{rendered_identifier, rendered_name};

/// Segments one rendered path may carry after the crate binding it is rooted at.
///
/// A path reaching deeper than this has stopped naming an item and started describing a tree, and the repair is a re-export at the address rather than a longer spelling here.
pub const PATH_SEGMENT_LIMIT: usize = 8;

/// Providers one composition may declare.
///
/// The root is a list a reader audits in one sitting, which is the whole reason it is a declaration rather than a scan.
pub const PROVIDER_LIMIT: usize = 64;

/// Issues one composition refusal carries before it counts the rest.
pub const COMPOSITION_ISSUE_LIMIT: usize = 64;

/// The fact a grammar refusal cites as its repair.
pub const DESCRIPTOR_MEANING_FACT: OwnerFact = OwnerFact {
    home: "descriptor",
    name: "a-descriptor-declaration-states-descriptor-meaning-alone",
};

/// The fact a vocabulary refusal cites as its repair.
pub const RENDERED_SPELLING_FACT: OwnerFact = OwnerFact {
    home: "descriptor",
    name: "a-rendered-spelling-is-one-rust-identifier",
};

/// The helper attribute one descriptor grammar is written in.
///
/// The name is the caller's: a door registers the attribute it wants and hands the same value to the reading, so a refusal names the word an author actually wrote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Grammar {
    /// The attribute's spelling, without its brackets.
    pub attribute: &'static str,
}

/// Who emitted a generated table, in the words the emission writes.
///
/// Every name these services declare about their own act — the namespace, the producer, and the door — is stated here rather than composed inside a rendering, so an authored declaration cannot sign an act it did not perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Emitter {
    /// The owner the producer and the door are spelled under.
    pub namespace: &'static str,
    /// The producer that emits the table.
    pub producer: &'static str,
    /// The door the declaration was authored through.
    pub door: &'static str,
}

/// A namespaced name: the owner that declares a spelling, and the spelling.
///
/// # Construction
///
/// Both parts are refused empty, so a name that names nothing is not a value anybody can hold.
///
/// # Ordering
///
/// The order is a storage order, over the namespace and then the stem. It ranks nothing.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name {
    namespace: String,
    stem: String,
}

/// The exported name a consumption target invokes one declaration's carrier by.
///
/// The author chooses it and the consumer's own compiler collision-checks it: the physical carrier is exported under a spelling nobody can know before expansion, so a declaration whose carrier nobody can address is a declaration whose rows nobody can run.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupportName(String);

/// The module a stamped payload is written into, at the target that invokes the carrier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleName(String);

/// A type a rendered module declares.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeName(String);

/// A function a rendered module declares.
///
/// One type for every function a stamped module writes, because they all land in ONE namespace: an aggregate seat colliding with a row lens is the same defect as two lenses colliding, and one type is what lets a single pass say so.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionName(String);

/// One physical dependency path a direct descriptor projection is rooted at.
///
/// Every segment is an ordinary Rust item name, so a renamed dependency and a facade re-export are both stated through the same shape: `renamed_harness` and `renamed_facade::harness` differ only in how many segments they carry.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DirectBinding {
    segments: NonEmpty<String, PATH_SEGMENT_LIMIT>,
}

crate::roster! {
    /// What a declaration was stating when its values refused.
    ///
    /// One roster over every bounded and doubled seat the three grammars declare, so a seat admitted later is one row here rather than three rows of a refusal.
    pub enum Seat {
        /// A segment of a path rooted at a crate binding.
        PathSegment = "path-segment",
        /// An open classification a row carries.
        Role = "role",
        /// An open label a row carries beside its roles.
        Tag = "tag",
        /// One row of a declared table.
        Row = "row",
        /// One aggregate seat's group of rows.
        SuiteGroup = "suite-group",
        /// The function an aggregate seat is declared under.
        Aggregate = "aggregate",
        /// The function one row is declared under.
        Lens = "lens",
        /// One item emitted into a generated target namespace.
        GeneratedItem = "generated-item",
        /// One point of a declared input-size axis.
        AxisSize = "axis-size",
        /// One byte of a declared work formula.
        WorkFormulaByte = "work-formula-byte",
        /// One counted quantity a bench row reads against its formula.
        WorkObservation = "work-observation",
        /// One owner fact mapped to the claim that permits pressure on it.
        FactMapping = "fact-mapping",
        /// One claim's permission over a roster of operator families.
        Permission = "permission",
        /// One operator family a permission names.
        OperatorFamily = "operator-family",
        /// One declared alternative at a mutation point.
        Alternative = "alternative",
        /// One declared provider of descriptor material.
        Provider = "provider",
    }
}

/// How the values one descriptor declaration states are not a lawful declaration.
///
/// Seven shapes over one seat roster rather than one row per seat: what refuses is the SHAPE of the disagreement, and which seat it was about is the other half of the same sentence.
#[must_use = "a declaration refusal names the seat the declaration did not fill"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclarationError {
    /// A namespaced name states no owner.
    NamespaceEmpty,
    /// A namespaced name states no spelling.
    StemEmpty,
    /// A spelling written where the rendering needs an item name cannot be one: not one Rust identifier, or a keyword the language already took.
    NotAnIdentifier,
    /// A seat that admits no emptiness was stated empty.
    Absent {
        /// What was being stated.
        seat: Seat,
    },
    /// Two of one roster's members carry one spelling.
    Doubled {
        /// What was being stated.
        seat: Seat,
    },
    /// A roster states more members than its declared magnitude admits.
    Unbounded {
        /// What was being stated.
        seat: Seat,
        /// The declared magnitude.
        bound: u64,
        /// How many were stated.
        observed: u64,
    },
    /// An input-size axis states too few sizes for a growth class to be read off it.
    NotACurve {
        /// How many sizes were stated.
        observed: u64,
    },
}

crate::roster! {
    /// How one helper body's tokens do not say a declaration its grammar admits.
    ///
    /// Dependent checks in a declared order, so exactly one cause is true of any refused body: the attribute is found before its body is read, the body's clauses before their values, and a value's shape before the vocabulary that value states.
    pub enum CaptureCause {
        /// The declaration carries the helper more than once.
        HelperDoubled = "helper-doubled",
        /// The helper states no parenthesized body.
        BodyAbsent = "body-absent",
        /// A clause is not one key and one value.
        ClauseUnread = "clause-unread",
        /// A clause's key is not one this grammar declares.
        ClauseUndeclared = "clause-undeclared",
        /// One clause is stated twice.
        ClauseDoubled = "clause-doubled",
        /// A required clause is absent.
        ClauseAbsent = "clause-absent",
        /// A value written where a namespaced reference is required is not one.
        ReferenceUnread = "reference-unread",
        /// A value written where a bracketed roster is required is not one.
        RosterUnread = "roster-unread",
        /// A value written where a named group is required is not one.
        GroupUnread = "group-unread",
        /// A row is not one name and one clause body.
        RowUnread = "row-unread",
        /// A mapping is not one fact and one claim.
        MappingUnread = "mapping-unread",
        /// A permission is not one claim and one family roster.
        PermissionUnread = "permission-unread",
        /// A value written where a declared Rust item path is required is not one.
        PathUnread = "path-unread",
        /// The item the helper sits on does not state a declared order this grammar can read.
        ItemUnread = "item-unread",
        /// The declared order carries fewer than two members, so no transposition of it exists.
        OrderUnpressable = "order-unpressable",
        /// A choice is not one bare name.
        ChoiceUnread = "choice-unread",
        /// One name is chosen twice.
        ChoiceDoubled = "choice-doubled",
        /// A chosen name is not one the shadow roster covers.
        NameUnshadowed = "name-unshadowed",
        /// The declaration chooses no name at all.
        NothingChosen = "nothing-chosen",
        /// A fault phrase is not one this grammar reads.
        PhraseUnread = "phrase-unread",
        /// A phrase or a link names a node or link the declaration never declared.
        EndpointUnknown = "endpoint-unknown",
        /// An authored number outruns the width of the seat it is written for.
        ///
        /// Every numeric seat has the width its harness value declares, and the capture parses at exactly that width — because generated code cannot outsource range safety to rustc: the overflowing-literal diagnostic is suppressed inside a foreign macro expansion, and an out-of-range literal wraps silently where source-authored Rust would refuse.
        NumberBeyondSeat = "number-beyond-seat",
        /// A declared name is one the language or the generated module already owns.
        ///
        /// A Rust keyword, or a name the rendering itself writes beside the authored ones — either would collide in the adopter's build, inside an expansion whose lints rustc has silenced, so the name refuses here at its own token instead.
        NameReserved = "name-reserved",
        /// A separator stands where no clause does.
        ///
        /// A leading or doubled comma is a phrase the author wrote, and a reader that dropped the empty group it makes would read straight past it — so the dangling separator refuses at its own token. A trailing comma after the last clause is ordinary Rust and stays lawful.
        SeparatorDangling = "separator-dangling",
    }
}

/// Which of the two readings refused, carrying that reading's own answer whole.
///
/// Whether the tokens SAY a declaration is the grammar's question; whether the values they say are lawful is the vocabulary's, and each answers in its own words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaptureIssue {
    /// The authored grammar refused.
    Grammar {
        /// The established cause.
        cause: CaptureCause,
    },
    /// The vocabulary refused a value the grammar read.
    Vocabulary {
        /// The vocabulary's own refusal.
        refusal: DeclarationError,
    },
}

/// How one helper body was not read: which grammar was reading, what it established, and the token.
///
/// Shared by every helper grammar this home declares.
/// The TYPE a refusal travels in is the grammar's own, because a diagnostic's family tag is a fact about the type and two helper readings of one declaration must never derive one related identity.
#[must_use = "a helper refusal names the grammar, the cause, and the token it was established at"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HelperRefusal {
    grammar: Grammar,
    issue: CaptureIssue,
    at: SpanHandle,
}

/// One declared provider of descriptor material.
///
/// The owning-home seat is what keeps a composition a declaration rather than a registry: a provider does not stand on its own authority, it stands on the owner fact it derives from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Provider {
    /// The provider's own identity.
    pub identity: OwnerIdentity,
    /// The owning home whose fact this provider derives from.
    pub home: OwnerFact,
    /// The kind it composes, by that kind's declared name.
    pub composes: &'static str,
}

/// How a composition fails to be declarable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompositionIssue {
    /// One provider identity is declared more than once.
    ProviderDoubled {
        /// The doubled provider.
        provider: OwnerIdentity,
    },
    /// The shared declaration vocabulary refused the provider seat.
    Declaration {
        /// The vocabulary's own refusal.
        refusal: DeclarationError,
    },
}

/// How one composition refused, with the complete admitted declaration finding or every bounded duplicate finding.
#[must_use = "a composition refusal carries the issues its declaration pass established"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompositionError {
    body: Capped<CompositionIssue, COMPOSITION_ISSUE_LIMIT>,
}

/// The one composition of descriptor-material providers: every provider that participates, named once.
///
/// Structurally non-empty — a composition with no provider is not a composition, it is silence — and duplicate-free by construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Composition {
    providers: NonEmpty<Provider, PROVIDER_LIMIT>,
}
