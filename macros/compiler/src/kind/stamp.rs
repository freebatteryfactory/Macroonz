//! The kind home's declaration stamps: one closed vocabulary from one declaration, and one related kind set with its complete disposition record.

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

/// Declares one set of kinds: a marker type and its [`Kind`](crate::kind::Kind) implementation per row, the enumerated set, its [`KindSet`](crate::kind::KindSet) implementation, and its [`DispositionRecord`](crate::kind::DispositionRecord).
///
/// One declaration, so the marker, the set, and the record cannot drift apart.
/// A kind added to a declaration grows all three together and stops the compiler at every construction of the record until somebody says what happens to it.
/// The record then becomes a [`DispositionSet`](crate::kind::DispositionSet) only after the compiler independently checks every surrendered name and the whole row count against the set's declaration.
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
/// use macroonz_compiler::{Disposition, DispositionSet, KindSet, NoQuestions, OwnerFact, SoleRole};
///
/// assert_eq!(<GreetKinds as KindSet>::NAMES, &["greet.impl"]);
/// assert_eq!(GreetKinds::GreetImpl.name(), "greet.impl");
///
/// let record = GreetDispositions {
///     greet_impl: Disposition::NotApplicable {
///         because: OwnerFact { home: "greet", name: "not-applicable" },
///     },
/// };
/// assert!(DispositionSet::<GreetKinds>::complete(record).is_ok());
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

        impl $crate::kind::DispositionRecord for $record {
            fn into_dispositions(
                self,
            ) -> impl Iterator<Item = (&'static str, $crate::kind::Disposition)> {
                [$( (<$kind as $crate::kind::Kind>::NAME, self.$seat) ),+].into_iter()
            }
        }
    };
}
