//! `ThreadPak` is a host-neutral semantic machine written in safe Rust. Programs are
//! typed data, not text: a builder constructs typed declarations, and the machine
//! validates, seals, executes, and remembers them. Any frontend enters through the
//! same public declaration path.
//!
//! The spine:
//!
//! ```text
//! typed declarations → Semantic Form → Execution Form → ProgramImage (.tpk)
//! → PakVM → runtime (the Stitch) → Bvisor → accepted history (.tlog)
//! ```
//!
//! Hosts live in other repositories and pin an exact `ThreadPak` revision. The machine
//! never knows which host is running it.
//!
//! The crate is a numbered waterfall of semantic homes (see the repository README's
//! band map); each home is declared here as it materializes, in dependency order.
//! The repository is in architecture closure: declaration surfaces and compile-time
//! laws are real code; machine runtime algorithms remain unopened until authorized.
//!
//! Two things sit at the root itself, ahead of the waterfall. [`types`] holds the
//! shape calculus every home instantiates. [`closed_register!`] is the composition
//! mechanism every home, the services crate, and this crate's own proof surface
//! stamp their closed rosters with; it declares no type, reaches no band's
//! material, and belongs to no band. [`CLOSED_REGISTER_ROW_CEILING`] is the one
//! value that mechanism projects, and it sits beside the stamp because it is read
//! out of the stamp's own expansion.

pub mod types;

// ---------------------------------------------------------------------------
// The closed-register stamp: the root's own composition mechanism. Instantiated
// by band 13's authoring algebra, by the services crate's rosters, and by this
// crate's proof surface, on identical terms.
// ---------------------------------------------------------------------------

/// Stamps one closed, fieldless roster from a single declaration of its rows.
///
/// # What one row states, and what follows from it
///
/// A row is a variant, its documentation, a declared stable name, and the prose
/// a person is shown. From that one statement the stamp writes the enum, the
/// roster constant `ALL` in declared order, the row's position as `slot`, the
/// declared stable name as `stable_name`, and the prose as `described` — so a
/// roster is authored once and read five ways.
///
/// # The position cannot drift from the order, and the order is semantic
///
/// `ALL` and `slot` are generated from the SAME row list in one expansion.
/// There is no second place to write a position, so a roster whose fourth row
/// answers `2` is not a defect this stamp can express — it is a value nobody
/// can write down. That is the whole reason the stamp exists: the hand-kept
/// form of this pattern is a roster array beside a `match` returning numbers,
/// and those two are two things to keep true.
///
/// The consequence runs the other way too, and it binds the caller. A
/// row's slot IS its place in the declaration, so reordering rows renumbers
/// every slot from the first move onward — and where a slot is written into
/// canonical bytes, that renumbering renames every identity ever derived under
/// it. A row is appended. It is never inserted, and never moved.
///
/// # What the stamp refuses to guess
///
/// The caller states everything. The stable name is DECLARED rather than taken
/// from the Rust spelling, on exactly the terms
/// [`CauseId`](crate::refusal::CauseId) is declared apart from its spelling:
/// renaming a variant must move the spelling and must move nothing derived
/// under the declared name. The prose is declared for the same reason — a
/// sentence built from an identifier is a sentence nobody wrote.
///
/// The derive set is the stamp's, not the caller's: `Debug`, `Clone`, `Copy`,
/// `PartialEq`, `Eq`, and `Hash`. A roster is a closed vocabulary of bare
/// words, and every one of those is true of every such roster; letting a caller
/// vary them would make the stamped pattern negotiable, which is the property
/// the stamp is for.
///
/// # The row ceiling is the supply's length, and the stamp is what says so
///
/// The stamp carries a DECLARED SUPPLY of positions, written out as literals in
/// ONE arm of the expansion, and the walk over a declaration's rows pairs each
/// row with exactly one of them. A row past the last of them finds the supply
/// spent, and the rule that matches that state is a `compile_error!` naming the
/// mechanism. Nothing is counted and nothing is added: the ceiling IS the
/// length of the supply, so there is no arithmetic anywhere in the expansion to
/// overflow, to saturate, or to disagree with a number written down elsewhere.
///
/// [`CLOSED_REGISTER_ROW_CEILING`] is that same supply read out as a value,
/// from the same arm the pairing walk spends, so the ceiling has one source and
/// two readings rather than a length and a number that agree. This
/// documentation names that constant and never the number, because a sentence
/// stating the number is a second place the ceiling would have to be moved.
///
/// The supply is also what bounds the recursion, and that is why the refusal is
/// reachable at all. The exhausted-supply rule MATCHES the rows that remain
/// without recursing over them, so expansion stops at the first row past the
/// supply whatever the declaration's length — rather than walking on until the
/// compiler's recursion limit ends it with a diagnostic about the stamp's own
/// internals at a boundary nobody declared.
///
/// **The ceiling is this stamp implementation's current authoring profile, not
/// a semantic cap on any vocabulary.** No roster in the machine is closed
/// because the supply is the length it is; each is closed because its
/// vocabulary is closed. A future mechanism, or a wider declared profile, may
/// raise the ceiling once it is qualified — raising it means extending the
/// supply, and what a longer supply costs is stated at
/// [`CLOSED_REGISTER_ROW_CEILING`].
///
/// The proven altitude of that claim is the DECLARATION.
/// `testpak/tests/stamp_row_ceiling.rs` stamps a roster that spends the supply
/// to its last position and reads exact positions off it — through the public
/// export, from a crate that is an ordinary consumer of this one — and
/// `testpak/tests/compile-fail/a-roster-past-the-stamp-ceiling.rs` is the row
/// past the supply refusing with the sentence above. The same boundary read
/// through the lifecycle facade is neither proven nor claimed here: that facade
/// does not exist, and its validation arrives with the lifecycle specimen.
///
/// # The closure claim, and its exact ceiling
///
/// A row that is not in the declaration does not exist: the enum has exactly
/// one declaration site and the stamp is it. That closure is a property of
/// macro output rather than a check anything performs, so it is stated as the
/// claim's ceiling and no fixture is manufactured to rehearse it. What IS
/// rehearsed is the drift the stamp removes —
/// `laws.rs declaration::a_stamped_roster_cannot_disagree_with_its_own_order`
/// exhibits a hand-kept roster whose position and slot disagree beside a
/// stamped one where the disagreement is unwritable.
///
/// # Where the stamp lives
///
/// The root owns it. A closed roster of declared words is a composition shape
/// the whole repository instantiates rather than any one band's material: band
/// 13's authoring algebra reaches for it, the services crate stamps its rosters
/// with it, and this crate's proof surface stamps its own. Seating it in a band
/// made every other consumer reach across a band edge for a mechanism that band
/// did not own.
///
/// It sits in `lib.rs` rather than in `types.rs` because that is where the
/// precedent puts it. [`scope_guard_version!`] sits in band 02's `mod.rs` — the
/// module surface that DECLARES its home's content — and not inside the
/// `types.rs` that holds the shapes. The root calculus has no `mod.rs`;
/// `lib.rs` is its module surface and `types.rs` is where its shapes live. The
/// stamp declares no type, so it has nothing to put in the latter.
///
/// Rust exports `macro_rules!` at the crate root whatever file it is written in.
/// That was always Rust's rule about macro namespacing rather than a root
/// admission of a semantic noun; it simply no longer has any explaining to do,
/// because the seat and the export now agree.
///
/// The `@`-prefixed rules below are the stamp's own internals — the declared
/// supply of positions, the two readings taken off it, and the walk that pairs
/// rows with it. None of them is an invocation form.
///
/// # The invocation
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
///
/// [`scope_guard_version!`]: crate::scope_guard_version
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
    (
        @position $subject:expr,
        ($($arms:tt)*),
        [],
        $head:ident $($rest:ident)*
    ) => {
        ::core::compile_error!(
            "closed_register!: this roster declares more rows than the stamp's declared supply \
             of positions. The supply's length is this implementation's current \
             authoring-profile ceiling, projected as `threadpak::CLOSED_REGISTER_ROW_CEILING`, \
             and it is not a semantic cap on the vocabulary: raising it means extending the \
             stamp's declared supply and requalifying it."
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

/// How many rows one [`closed_register!`] roster may declare: the length of the
/// stamp's own declared supply of positions, read out as a value.
///
/// # It is the supply, not a number kept beside it
///
/// The stamp writes its supply of positions out in one arm. The pairing walk
/// that answers `slot` and this constant are two readings of that ONE list, so
/// a supply that is extended moves this value with it and cannot leave it
/// standing. That is the same anti-drift shape the stamp applies to its
/// callers, applied to the stamp itself: the hand-kept form of this pattern is
/// a supply of literals beside a number stating how many there are, and those
/// are two things to keep true.
///
/// # The one place this profile's value is written down
///
/// The example below is that place, and it is deliberate rather than
/// decorative. It records what the supply is TODAY, so extending the supply
/// fails here and nowhere else in this repository's prose. That failure is the
/// requalification trigger: repairing it means restating the recorded value on
/// purpose, once, where a reader of the diff can see the profile move.
///
/// ```
/// assert_eq!(threadpak::CLOSED_REGISTER_ROW_CEILING, 64);
/// ```
///
/// # What it is a ceiling ON
///
/// This is the current authoring profile of this stamp implementation, and
/// nothing else. No roster in the machine is closed because the supply is the
/// length it is; each is closed because its own vocabulary is closed. A
/// vocabulary that genuinely holds more rows than the supply is a reason to
/// extend the supply, never a reason to read a meaning into this number.
///
/// # Extending the supply, and the width that decides what it costs
///
/// A row's position is answered as a `u8`, and that width is what tells two
/// different acts apart.
///
/// A supply extended WITHIN what a `u8` represents is an append and nothing
/// more. Every position already answered keeps the value it answered, no
/// existing row's slot moves, and no identity ever derived under a slot is
/// renamed — the same terms a roster's own rows stand under, where a row is
/// appended and never inserted or moved.
///
/// A supply extended PAST what a `u8` represents is not a longer supply at all.
/// It is a new versioned encoding profile for positions and for the identities
/// written under them, and it arrives as one: its own version, its own
/// migration of anything already written, and its own qualification. The
/// measured decision to answer positions as a `u8` rather than a wider integer
/// is stated here because this is the value that would have to move first, and
/// a reader who is about to move it is the reader who needs to know which of
/// the two acts they are performing.
///
/// # Where it is seated, and why
///
/// It is a projection of the stamp's own expansion, so it cannot be written
/// anywhere the stamp is not in scope. The stamp is seated in `lib.rs` — the
/// root calculus's module surface — on the precedent [`scope_guard_version!`]
/// set in band 02's `mod.rs`, and this constant seats beside it on exactly that
/// precedent. `types.rs` holds the root's shapes; a value read out of a
/// mechanism that file does not carry has nothing to do there.
///
/// [`scope_guard_version!`]: crate::scope_guard_version
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
