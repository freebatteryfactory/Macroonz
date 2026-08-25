//! The kind home's declarations: the four open traits a consumer implements, the two rosters the compiler owns, the destination, the disposition, the set contract, and the two stamps that write a roster down.
//!
//! Declarations only.
//! Nothing here holds a private field, so the home has no invariant nucleus and no `type_guard.rs`.

use crate::identity::{GeneratedUnit, Identity, OwnerFact, Profile};

/// What one request produces.
///
/// A kind is a marker type in the crate that declares it, and the compiler is generic over it from the first step of the road to the last.
/// Nothing seals this trait and nothing registers an implementation of it.
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

/// One declared set of kinds, and the record that says what happened to every one of them.
///
/// The record has one required seat per kind, so a set that leaves a kind unanswered is a struct nobody can finish writing.
pub trait KindSet {
    /// One required seat per kind of the set.
    type Dispositions: Clone + Eq + core::fmt::Debug;

    /// Every kind's declared name, in the order the set states them.
    const NAMES: &'static [&'static str];
}

/// One row's position in its roster, or the roster's length where the roster does not carry it.
fn slot_in<T: Copy + Eq>(roster: &[T], row: T) -> u16 {
    let position = roster
        .iter()
        .position(|other| *other == row)
        .unwrap_or(roster.len());
    u16::try_from(position).unwrap_or(u16::MAX)
}

/// Declares one closed vocabulary: the enum, its complete roster, and one declared name per row.
///
/// For a list of names and nothing else: a role is written by hand instead, because a role also names a destination and an implementation says that better than a stamp with an extra column.
///
/// # Examples
///
/// ```rust
/// macroonz_compiler::roster! {
///     /// Which direction a codec covers.
///     pub enum Direction {
///         /// Typed value to canonical bytes.
///         Encode = "encode",
///         /// Canonical bytes to typed value.
///         Decode = "decode",
///     }
/// }
///
/// assert_eq!(Direction::ALL, &[Direction::Encode, Direction::Decode]);
/// assert_eq!(Direction::Decode.name(), "decode");
/// ```
#[macro_export]
macro_rules! roster {
    (
        $(#[$note:meta])*
        $vis:vis enum $name:ident {
            $( $(#[$row:meta])* $variant:ident = $declared:literal ),+ $(,)?
        }
    ) => {
        $(#[$note])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $name {
            $( $(#[$row])* $variant, )+
        }

        impl $name {
            /// The complete roster, in declaration order.
            $vis const ALL: &'static [Self] = &[$( Self::$variant ),+];

            /// This row's declared name.
            #[must_use]
            $vis const fn name(self) -> &'static str {
                match self {
                    $( Self::$variant => $declared, )+
                }
            }
        }
    };
}

/// Declares one set of kinds: a marker type and its [`Kind`] implementation per row, the enumerated set, its [`KindSet`] implementation, and the disposition record.
///
/// One declaration, so the marker, the set, and the record cannot drift apart.
/// A kind added to a declaration grows all three together and stops the compiler at every construction of the record until somebody says what happens to it.
///
/// The seat is the field name the record carries a row's answer under, declared beside the kind rather than composed from the marker's spelling, for the same reason the declared name beside it is: a field renamed by every refactor of a Rust identifier is a field nobody can rely on.
///
/// # Examples
///
/// ```rust
/// pub type Greeting = &'static str;
///
/// macroonz_compiler::kinds! {
///     set = GreetKinds;
///     dispositions = GreetDispositions;
///
///     /// Projects a declaration into the implementation that greets.
///     GreetImpl = "greet.impl", greet_impl => Greeting, SoleRole, NoQuestions;
/// }
///
/// use macroonz_compiler::{KindSet, NoQuestions, SoleRole};
///
/// assert_eq!(<GreetKinds as KindSet>::NAMES, &["greet.impl"]);
/// assert_eq!(GreetKinds::GreetImpl.name(), "greet.impl");
/// ```
#[macro_export]
macro_rules! kinds {
    (
        set = $set:ident;
        dispositions = $record:ident;
        $(
            $(#[$note:meta])*
            $kind:ident = $declared:literal, $seat:ident => $content:ty, $role:ty, $question:ty
        );+ $(;)?
    ) => {
        $(
            $(#[$note])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $kind;

            impl $crate::kind::Kind for $kind {
                const NAME: &'static str = $declared;
                type Content = $content;
                type Role = $role;
                type Question = $question;
            }
        )+

        /// The kinds this set names, one row each.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $set {
            $( $(#[$note])* $kind ),+
        }

        impl $set {
            /// The complete set, in declaration order.
            pub const ALL: &'static [Self] = &[$( Self::$kind ),+];

            /// This row's kind's declared name, read off the kind itself.
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $( Self::$kind => <$kind as $crate::kind::Kind>::NAME ),+
                }
            }
        }

        impl $crate::kind::KindSet for $set {
            type Dispositions = $record;

            const NAMES: &'static [&'static str] =
                &[$( <$kind as $crate::kind::Kind>::NAME ),+];
        }

        /// What happened to every kind of the set: one required seat per row.
        #[must_use = "a disposition record is what happened to every kind of the set"]
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $record {
            $(
                #[doc = concat!("What happened to the `", $declared, "` kind.")]
                pub $seat: $crate::kind::Disposition
            ),+
        }

        impl $record {
            /// What happened to one kind of the set.
            ///
            /// Total: every row reads to exactly one seat, and a row admitted later stops the compiler here until somebody says which seat carries it.
            #[must_use]
            pub const fn under(&self, row: $set) -> &$crate::kind::Disposition {
                match row {
                    $( $set::$kind => &self.$seat ),+
                }
            }
        }
    };
}
