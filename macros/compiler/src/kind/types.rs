//! The kind home's declarations: the open semantic traits a consumer implements, the rosters the compiler owns, the disposition vocabulary, and the complete-set witness.
//!
//! Declarations only, with every road that reaches a private field in `type_guard.rs`, this file's own child.

use super::type_contract::slot_in;
use crate::identity::{GeneratedUnit, Identity, OwnerFact, Profile};
use core::marker::PhantomData;

#[path = "type_guard.rs"]
mod guard;

/// What one request produces.
///
/// A kind is a marker type in the crate that declares it, and the compiler is generic over it from the first step of the road to the last.
/// Nothing seals this trait and nothing registers an implementation of it.
///
/// # Examples
///
/// ```rust
/// use macroonz_compiler::{Kind, NoQuestions, SoleRole};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// struct GreetImpl;
///
/// impl Kind for GreetImpl {
///     const NAME: &'static str = "greet.impl";
///     type Content = ();
///     type Role = SoleRole;
///     type Question = NoQuestions;
/// }
///
/// assert_eq!(GreetImpl::NAME, "greet.impl");
/// ```
pub trait Kind: 'static {
    /// The name this kind is spelled by wherever a name is written down.
    ///
    /// Declared rather than read off the Rust spelling, so renaming the marker renames no identity.
    const NAME: &'static str;

    /// The facts a request of this kind carries beyond its captured tokens.
    ///
    /// Its canonical encoding is the content commitment's material, so changing any fact a renderer may read changes the commitment before a plan exists.
    type Content: CanonicalContent;

    /// The seats this kind's rendering fills.
    type Role: Role;

    /// The questions this kind owes beyond the universal ones.
    type Question: Question;
}

/// Kind-specific facts with one complete canonical encoding.
///
/// The encoding is semantic material rather than a rendering for a person.
/// A kind owns the implementation for its content, and the compiler frames the complete result before deriving the content commitment.
pub trait CanonicalContent: Clone + Eq + core::fmt::Debug {
    /// Append every fact this content carries in its declared order.
    fn encode_content_into(&self, into: &mut Vec<u8>);

    /// The complete canonical bytes of this content.
    #[must_use]
    fn canonical_content_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        self.encode_content_into(&mut bytes);
        bytes
    }
}

/// One seat a kind's rendering fills.
///
/// A rendered unit is matched to a planned one by role, so a rendering that produced the right number of units in the wrong seats is caught by the seat rather than by a count.
pub trait Role: Copy + Eq + core::fmt::Debug + 'static {
    /// The complete roster, in the order the kind states it.
    ///
    /// Every walk over a rendering quantifies over this, and membership admission refuses a member whose role is absent from it — so a lawful value the roster omits cannot become a planned member a walk would never look at.
    const ALL: &'static [Self];

    /// This role's declared name.
    #[must_use]
    fn name(self) -> &'static str;

    /// Where the unit rendered under this role lands.
    ///
    /// A property of the seat, so two plans of one kind cannot disagree about which build compiles their units.
    #[must_use]
    fn destination(self) -> Destination;

    /// This role's position in the roster, which a rendered unit's transcript carries.
    ///
    /// A role the roster does not carry has no position and reads as the roster's length.
    #[must_use]
    fn slot(self) -> u16 {
        slot_in(Self::ALL, self)
    }
}

/// One question a kind owes an answer to, beyond the questions every kind owes.
pub trait Question: Copy + Eq + core::fmt::Debug + 'static {
    /// The complete roster, in the order the kind states it.
    const ALL: &'static [Self];

    /// The typed answer to a question of this roster.
    type Answer: Answer<Question = Self>;

    /// This question's declared name.
    #[must_use]
    fn name(self) -> &'static str;

    /// This question's position in the roster, which an explanation's preimage carries.
    #[must_use]
    fn slot(self) -> u16 {
        slot_in(Self::ALL, self)
    }
}

/// One typed answer, and the question it answers.
pub trait Answer: Clone + Eq + core::fmt::Debug {
    /// The roster this answer belongs to.
    type Question: Question;

    /// The question this answer answers.
    #[must_use]
    fn question(&self) -> Self::Question;

    /// Append this answer's canonical bytes.
    fn encode_into(&self, into: &mut Vec<u8>);

    /// This answer rendered for a person.
    ///
    /// A projection: no identity, decision, or refusal reads one back.
    #[must_use]
    fn human(&self) -> String;
}

/// The question roster of a kind that owes nothing beyond the universal questions.
///
/// Uninhabited, so it is its own answer as well as its own roster: there is no value here to ask about or to answer for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoQuestions {}

/// The role roster of a kind that renders exactly one unit, at the declaration site.
///
/// Not a placeholder and not an absence: a kind whose rendering is one unit says so with a roster of one, and a kind whose one unit lands elsewhere declares its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoleRole {
    /// The kind's one rendered unit.
    Sole,
}

crate::roster! {
    /// Where a rendered unit lands.
    ///
    /// Four deliveries, and a role names exactly one of them.
    pub enum Destination {
        /// The tokens the consumer's normal build compiles where the declaration stands.
        DeclarationSite = "declaration-site",
        /// The deferred cargo the consumer's test target invokes; the normal build compiles none of it.
        TestCarrier = "test-carrier",
        /// The deferred cargo the consumer's bench target invokes, on the same terms and through the same shell.
        BenchCarrier = "bench-carrier",
        /// A standalone artifact a publication step writes to its own address.
        PublicationArtifact = "publication-artifact",
    }
}

/// What happened to one kind that could have been generated.
///
/// Silence is not a variant: where a projection is absent, the absence has a name and cites the fact that caused it.
/// There is no refused answer either, because a request that fails a step of the road is refused whole and produces a diagnostic rather than a set.
#[must_use = "a disposition is what happened to a kind, and silence is not a variant"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Disposition {
    /// It was generated, and this is the unit it produced.
    Generated {
        /// The generated unit's semantic key.
        unit: Identity<GeneratedUnit>,
    },
    /// It does not apply here, and this is the fact that makes it inapplicable.
    NotApplicable {
        /// The fact the answer rests on.
        because: OwnerFact,
    },
    /// Nobody asked for it, and this is the fact that says so.
    NotRequested {
        /// The fact the answer rests on.
        because: OwnerFact,
    },
    /// The profile the request ran under does not offer it.
    UnavailableUnderProfile {
        /// The profile that does not offer it.
        profile: Profile,
        /// The fact naming what that profile could not furnish.
        because: OwnerFact,
    },
}

/// A consumer-owned record that can surrender its named dispositions in kind declaration order.
///
/// Implementations state rows, not completeness.
/// [`DispositionSet::complete`] compares every surrendered name and the whole row count with the owning [`KindSet`] before the record can become the witness an account seats.
pub trait DispositionRecord: Clone + Eq + core::fmt::Debug {
    /// Surrender every stated kind name and disposition, in the set's declaration order.
    fn into_dispositions(self) -> impl Iterator<Item = (&'static str, Disposition)>;
}

/// One declared set of kinds and the record from which its complete disposition witness is built.
///
/// The trait remains open, but naming a record here does not certify its completeness.
/// Only [`DispositionSet::complete`] can turn the record into the private-field witness [`Accounted`](crate::Accounted) accepts.
pub trait KindSet {
    /// The consumer-owned disposition record for this set.
    type Dispositions: DispositionRecord;

    /// Every kind's declared name, in the order the set states them.
    const NAMES: &'static [&'static str];
}

/// A disposition for every declared kind of one set, in declaration order.
///
/// The rows are private and the only public constructor checks every name and the complete row count against [`KindSet::NAMES`], so an omitted, doubled, foreign, or reordered seat cannot become this value and cannot be seated beside an expansion.
#[must_use = "a complete disposition set is the witness an accounted expansion requires"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispositionSet<Set: KindSet> {
    dispositions: Vec<Disposition>,
    kind_set: PhantomData<fn() -> Set>,
}

/// How a disposition record refuses to become a complete set witness.
#[must_use = "a disposition-set refusal names the count or kind-name disagreement"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DispositionSetError {
    /// The record surrendered a different number of rows than the kind set declares.
    CountMismatch {
        /// How many kind names the set declares.
        expected: usize,
        /// How many disposition rows the record surrendered.
        observed: usize,
    },
    /// One surrendered row names a kind other than the kind declared at that position.
    KindMismatch {
        /// The kind name the set declares at this position.
        expected: &'static str,
        /// The kind name the record surrendered at this position.
        observed: &'static str,
    },
}
