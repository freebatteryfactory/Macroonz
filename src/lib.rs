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
//! material, and belongs to no band.

pub mod types;

// ---------------------------------------------------------------------------
// The closed-register stamp: the root's own composition mechanism. Instantiated
// by band 13's authoring algebra, by the services crate's eight rosters, and by
// this crate's proof surface, on identical terms.
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
/// The consequence runs the other way too, and it is a law on the caller. A
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
/// # The row ceiling is sixty-four, and the stamp is what says so
///
/// The stamp carries a DECLARED SUPPLY of sixty-four positions, written out as
/// sixty-four literals, and the walk over a declaration's rows pairs each row
/// with exactly one of them. A sixty-fifth row finds the supply spent, and the
/// rule that matches that state is a `compile_error!` naming the ceiling and
/// this mechanism. Nothing is counted and nothing is added: the ceiling IS the
/// length of the supply, so there is no arithmetic anywhere in the expansion to
/// overflow, to saturate, or to disagree with a number written down elsewhere.
///
/// The supply is also what bounds the recursion, and that is why the refusal is
/// reachable at all. The walk stops at the sixty-fifth step whatever the
/// declaration's length, so a two-hundred-row roster refuses with the same one
/// sentence a sixty-five-row roster does rather than dying against the
/// compiler's recursion limit with a diagnostic about the stamp's internals.
/// The position type is a `u8` and the widest lawful slot is 63, so the byte a
/// roster's position is written as has room the ceiling can never spend.
///
/// **Sixty-four is the current authoring-profile ceiling of this stamp
/// implementation, not a semantic cap on any vocabulary.** No roster in the
/// machine is closed because sixty-four is meaningful; each is closed because
/// its vocabulary is closed. A future mechanism, or a wider declared profile,
/// may raise this ceiling once it is qualified — raising it means extending the
/// supply, and the number moves in exactly one place.
///
/// The proven altitude of that claim is the DECLARATION.
/// `laws.rs root::a_stamped_roster_declares_its_own_ceiling` compiles a
/// sixty-four-row roster and reads exact positions off it, and
/// `testpak/tests/compile-fail/a-roster-past-the-stamp-ceiling.rs` is the
/// sixty-fifth row refusing with the sentence above. The same boundary read
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
/// 13's authoring algebra reaches for it, the services crate stamps eight
/// rosters with it, and this crate's proof surface stamps its own. Seating it in
/// a band made every other consumer reach across a band edge for a mechanism
/// that band did not own.
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
/// The `@`-prefixed rules below are the stamp's own walk over its rows and are
/// not an invocation form.
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
                $crate::closed_register!(
                    @position self,
                    (),
                    [
                        0u8  1u8  2u8  3u8  4u8  5u8  6u8  7u8
                        8u8  9u8  10u8 11u8 12u8 13u8 14u8 15u8
                        16u8 17u8 18u8 19u8 20u8 21u8 22u8 23u8
                        24u8 25u8 26u8 27u8 28u8 29u8 30u8 31u8
                        32u8 33u8 34u8 35u8 36u8 37u8 38u8 39u8
                        40u8 41u8 42u8 43u8 44u8 45u8 46u8 47u8
                        48u8 49u8 50u8 51u8 52u8 53u8 54u8 55u8
                        56u8 57u8 58u8 59u8 60u8 61u8 62u8 63u8
                    ],
                    $($variant)+
                )
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
             of sixty-four positions. Sixty-four is the current authoring-profile ceiling of \
             this stamp implementation, not a semantic cap on the vocabulary: raising it means \
             extending the supply of positions the stamp declares."
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
