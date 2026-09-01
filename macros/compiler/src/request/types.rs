//! The request home's declarations: the front door itself, who is asking, the crate rendered paths are rooted at, and the two facts a request stands on.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, so a request is whatever one of those roads built.

use crate::identity::{self, Identity, MACROONZ_STEM, OwnerFact, OwnerIdentity, Profile, Version};
use crate::kind::{Kind, Question, Role};
use crate::token::CapturedInput;

#[path = "type_guard.rs"]
mod guard;

/// The profile a request runs under where it selects no other.
///
/// Everything this compiler renders is Rust written at a declaration, and that is what the name says.
pub const RUST_DECLARATION_PROFILE: Profile =
    Profile::declared(MACROONZ_STEM, "rust-declaration", Version::declared(1));

/// This home's own declared fact: the seats one request selects are its complete output set.
///
/// The selection rule every plan built here cites, and the first entry of every decision trace it records.
pub const SELECTION_FACT: OwnerFact = OwnerFact {
    home: "request",
    name: "a-requests-selected-seats-are-its-complete-output-set",
};

/// Who is asking, for whatever a door's expansions are stamped into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Producer {
    /// The namespace the producer's generated names stand under.
    pub namespace: &'static str,
    /// The producer's own declared name.
    pub name: &'static str,
}

/// The crate a path rendered through one door is rooted at.
///
/// A consumer may rename its dependencies, so the word a rendered path opens with is the consumer's to declare and no spelling of it is written down in this compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CrateBinding {
    spelling: &'static str,
}

/// The one value that says who is asking.
///
/// The two declared names are spellings rather than identities so a door is a `const` a consumer writes down once; the identities they stand for are derived on read, under the declared-name grammar at the two positions this compiler assigns.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Door {
    prefix: &'static str,
    grammar: &'static str,
    entry: &'static str,
    binding: CrateBinding,
    producer: Producer,
}

/// One request: the material it stands over, what that material means, who is asking, and the seats the caller states.
///
/// A builder, and the value the whole road is walked from.
/// Stating a builder fact again replaces its earlier statement; a publication address replaces only the earlier address for that same seat.
/// The material is held rather than committed to at the door, because a commitment beside the bytes it was taken from is one fact in two seats; the commitment is derived once, where the plan that carries it is built.
///
/// # Nonclaims
///
/// It answers nothing about the material it holds.
/// Reading a declaration grammar out of captured tokens is the consumer's, and a request is what it hands over afterwards.
#[must_use = "a request is a road nobody walked until it is rendered"]
pub struct Request<'door, K: Kind> {
    capture: CapturedInput,
    content: K::Content,
    door: &'door Door,
    dependencies: Vec<Identity<identity::CapturedDeclaration>>,
    profile: Profile,
    assumptions: Vec<OwnerFact>,
    addresses: Vec<(K::Role, OwnerIdentity)>,
    answers: Vec<<K::Question as Question>::Answer>,
    selection: Selection<K::Role>,
}

/// Whether one request selects its kind's complete role roster or one explicitly stated nonempty subset.
pub(super) enum Selection<R: Role> {
    /// Every role the kind declares, preserving the established default road.
    All,
    /// One explicitly selected role followed by the rest of the selected subset.
    Declared { first: R, rest: Vec<R> },
}

/// The already stated request facts planning reads together.
pub(super) struct Statements<'request, R: Role> {
    pub(super) assumptions: &'request [OwnerFact],
    pub(super) addresses: &'request [(R, OwnerIdentity)],
    pub(super) selection: &'request Selection<R>,
}
