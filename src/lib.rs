//! `ThreadPak` is an embedded, sync-first, event-native database and runtime —
//! an opinionated Rust library of semantic primitives, named for the logical
//! thread it preserves from intent through accepted facts, Turns, Attempts,
//! effects, receipts, replay, and reconciliation.
//! Programs enter as typed declarations, not text; accepted history is the
//! authority; everything else is derived and rebuildable.
//! The repository README carries the machine in one view and the band map;
//! `src/README.md` carries the root calculus and the crate-wide laws.
//!
//! The crate is a numbered waterfall of semantic homes, declared below in
//! dependency order as each materializes.
//! Three things sit at the root itself: [`types`] holds the shape calculus
//! every home instantiates, [`depot`] is the bank of data-shaped truth every
//! crate on the machine can read, and [`closed_register!`] is the stamp every
//! closed roster is declared through, with [`CLOSED_REGISTER_ROW_CEILING`]
//! the one value it projects.

pub mod types;

pub mod depot;

// ---------------------------------------------------------------------------
// The closed-register stamp: the root's own composition mechanism. Instantiated
// by band 13's authoring algebra, by the services crate's rosters, and by this
// crate's proof surface, on identical terms.
// ---------------------------------------------------------------------------

/// Stamps one closed, fieldless roster from a single declaration of its rows.
///
/// A row is a variant, its documentation, a declared stable name, and the
/// prose a person is shown.
/// From that one statement the stamp writes the enum, the roster constant
/// `ALL` in declared order, the position `slot`, the declared `stable_name`,
/// and the human `described` — authored once, read five ways, with no second
/// place for a position to drift.
/// A row that is not in the declaration does not exist: the enum has exactly
/// one declaration site, and the stamp is it.
///
/// # Ordering
///
/// A row's slot IS its place in the declaration, so reordering rows renumbers
/// every slot from the first move onward — and where a slot is written into
/// canonical bytes, that renumbering renames every identity derived under it.
/// A row is appended. It is never inserted, and never moved.
///
/// # Construction
///
/// The caller states everything: the stable name is declared rather than
/// taken from the Rust spelling, so renaming a variant moves the spelling and
/// moves nothing derived under the name, and the prose is declared for the
/// same reason.
/// The derive set is the stamp's, not the caller's: `Debug`, `Clone`, `Copy`,
/// `PartialEq`, `Eq`, `Hash`.
///
/// # Bounds
///
/// The stamp pairs each row with one position from its own declared supply;
/// a roster past the supply refuses at compile time with the stamp's own
/// sentence.
/// The supply's length is [`CLOSED_REGISTER_ROW_CEILING`] — an authoring
/// ceiling of this implementation, never a semantic cap on any vocabulary.
///
/// The `@`-prefixed rules below are the stamp's internals, not invocation
/// forms.
///
/// # Examples
///
/// ```
/// threadpak::closed_register! {
///     /// The demo roster.
///     pub enum DemoRow {
///         /// The first row.
///         First = "first", "the first row";
///         /// The second row.
///         Second = "second", "the second row";
///     }
/// }
///
/// assert_eq!(DemoRow::ALL, [DemoRow::First, DemoRow::Second]);
/// assert_eq!(DemoRow::Second.slot(), 1);
/// assert_eq!(DemoRow::Second.stable_name(), "second");
/// assert_eq!(DemoRow::Second.described(), "the second row");
/// ```
#[macro_export]
macro_rules! closed_register {
    (
        $(#[$note:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$row:meta])*
                $variant:ident = $stable:literal, $described:literal;
            )+
        }
    ) => {
        $(#[$note])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $name {
            $(
                $(#[$row])*
                $variant,
            )+
        }

        impl $name {
            /// The declared roster, in the order the declaration states it.
            ///
            /// This constant and [`slot`](Self::slot) are written from one row
            /// list in one expansion, so a position read here and a position
            /// answered there are the same fact rather than two facts that
            /// agree.
            $vis const ALL: [Self; [$(Self::$variant),+].len()] = [$(Self::$variant),+];

            /// This row's position in the declared roster, counted from the
            /// first row.
            ///
            /// It is the roster's own layout read back, never a number anybody
            /// wrote: see [`ALL`](Self::ALL). The stamp pairs each row with one
            /// position from its own declared supply, so the answer is a
            /// literal rather than a running total, and the supply's length is
            /// the ceiling a longer roster refuses against.
            #[must_use]
            $vis const fn slot(self) -> u8 {
                $crate::closed_register!(@supply pairing self, $($variant)+)
            }

            /// This row's declared stable name.
            ///
            /// Declared rather than taken from the Rust spelling, so renaming
            /// the variant moves the spelling and moves nothing derived under
            /// this name.
            #[must_use]
            $vis const fn stable_name(self) -> &'static str {
                match self {
                    $( Self::$variant => $stable, )+
                }
            }

            /// This row rendered for a person.
            ///
            /// A projection of the typed value: nothing reads it back, and no
            /// decision consults it.
            #[must_use]
            $vis const fn described(self) -> &'static str {
                match self {
                    $( Self::$variant => $described, )+
                }
            }
        }
    };

    // THE DECLARED SUPPLY OF POSITIONS. It is written out here and nowhere
    // else in this repository: everything that needs the supply, or needs how
    // long it is, arrives through this arm and continues into the rule it named.
    // The ceiling therefore cannot be raised in one reading and left standing in
    // the other, which is the drift this whole stamp exists to remove.
    (@supply $($continuation:tt)*) => {
        $crate::closed_register!(
            @supplied
            [
                0u8  1u8  2u8  3u8  4u8  5u8  6u8  7u8
                8u8  9u8  10u8 11u8 12u8 13u8 14u8 15u8
                16u8 17u8 18u8 19u8 20u8 21u8 22u8 23u8
                24u8 25u8 26u8 27u8 28u8 29u8 30u8 31u8
                32u8 33u8 34u8 35u8 36u8 37u8 38u8 39u8
                40u8 41u8 42u8 43u8 44u8 45u8 46u8 47u8
                48u8 49u8 50u8 51u8 52u8 53u8 54u8 55u8
                56u8 57u8 58u8 59u8 60u8 61u8 62u8 63u8
            ]
            $($continuation)*
        )
    };

    // The supply's LENGTH, as a value. Const-evaluable, and the array is a
    // temporary the compiler measures rather than material anything keeps.
    (@supplied [$($position:literal)*] length) => {
        [$($position),*].len()
    };

    // The supply, HANDED TO THE PAIRING WALK below.
    (@supplied [$($position:literal)*] pairing $subject:expr, $($variant:ident)+) => {
        $crate::closed_register!(
            @position $subject,
            (),
            [$($position)*],
            $($variant)+
        )
    };

    // One row, one position off the declared supply. Neither side is counted:
    // the pairing is the whole arithmetic.
    (
        @position $subject:expr,
        ($($arms:tt)*),
        [$position:literal $($unspent:literal)*],
        $head:ident $($rest:ident)*
    ) => {
        $crate::closed_register!(
            @position $subject,
            ($($arms)* Self::$head => $position,),
            [$($unspent)*],
            $($rest)*
        )
    };

    // Rows remain and the supply is spent: the stamp's own refusal, reached
    // before any recursion limit is, whatever the declaration's length.
    //
    // The sentence names the exported constant and no crate path. The expansion
    // reaches the constant through `$crate` and always resolves; a SENTENCE
    // cannot, because a consumer may rename this dependency and a compiler
    // message has no way to learn the name it was renamed to. Spelling one would
    // send that reader to a path their crate does not have.
    (
        @position $subject:expr,
        ($($arms:tt)*),
        [],
        $head:ident $($rest:ident)*
    ) => {
        ::core::compile_error!(
            "closed_register!: this roster declares more rows than the stamp's declared supply \
             of positions. The supply's length is this implementation's current \
             authoring-profile ceiling, exported as `CLOSED_REGISTER_ROW_CEILING`, and it is \
             not a semantic cap on the vocabulary: raising it means extending the stamp's \
             declared supply and requalifying it."
        )
    };

    // Every row paired. Whatever supply is unspent is simply unspent.
    (
        @position $subject:expr,
        ($($arms:tt)*),
        [$($unspent:literal)*],
    ) => {
        match $subject {
            $($arms)*
        }
    };
}

/// How many rows one [`closed_register!`] roster may declare: the length of
/// the stamp's own declared supply of positions, read out as a value.
///
/// The pairing walk that answers `slot` and this constant are two readings of
/// one list, so extending the supply moves this value with it and cannot
/// leave it standing.
/// It is the current authoring profile of this stamp implementation, never a
/// semantic cap: no roster is closed because the supply is the length it is —
/// each is closed because its own vocabulary is closed.
///
/// # Bounds
///
/// A position is answered as a `u8`, and that width tells two acts apart.
/// A supply extended within `u8` is an append: every answered position keeps
/// its value, no slot moves, and no identity derived under a slot is renamed.
/// A supply extended past `u8` is not a longer supply at all — it is a new
/// versioned encoding profile for positions and the identities written under
/// them, arriving with its own version, migration, and qualification.
///
/// # Examples
///
/// The recorded value is deliberate: extending the supply fails here, once,
/// where a reader of the diff sees the profile move.
///
/// ```
/// assert_eq!(threadpak::CLOSED_REGISTER_ROW_CEILING, 64);
/// ```
pub const CLOSED_REGISTER_ROW_CEILING: usize = crate::closed_register!(@supply length);

#[path = "00_refusal/mod.rs"]
pub mod refusal;

#[path = "01_logic/mod.rs"]
pub mod logic;

#[path = "02_identity/mod.rs"]
pub mod identity;

#[path = "03_value/mod.rs"]
pub mod value;

#[path = "04_numeric/mod.rs"]
pub mod numeric;

#[path = "05_bounds/mod.rs"]
pub mod bounds;

#[path = "06_authority/mod.rs"]
pub mod authority;

#[path = "07_bytes/mod.rs"]
pub mod bytes;

#[path = "08_schema/mod.rs"]
pub mod schema;

#[path = "09_time/mod.rs"]
pub mod time;

#[path = "10_history/mod.rs"]
pub mod history;

#[path = "11_navigation/mod.rs"]
pub mod navigation;

#[path = "12_port/mod.rs"]
pub mod port;

#[path = "13_declaration/mod.rs"]
pub mod declaration;

#[path = "14_semantic/mod.rs"]
pub mod semantic;

#[path = "15_execution/mod.rs"]
pub mod execution;

#[path = "16_image/mod.rs"]
pub mod image;

#[path = "17_pakvm/mod.rs"]
pub mod pakvm;

#[path = "18_bvisor/mod.rs"]
pub mod bvisor;

#[path = "19_runtime/mod.rs"]
pub mod runtime;

#[path = "20_derived/mod.rs"]
pub mod derived;

#[path = "21_application/mod.rs"]
pub mod application;

#[path = "22_security/mod.rs"]
pub mod security;

#[path = "23_evidence/mod.rs"]
pub mod evidence;

#[cfg(test)]
mod laws;
