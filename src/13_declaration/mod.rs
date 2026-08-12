//! Band 13 — declaration: the shared authoring algebra — phase roots, name
//! roles, the linker's families, the six facets, staged meta, frontend roles.

pub mod types;

pub use types::{
    AuthoredName, AuthoredNameConstruction, AuthoredNameConstructionIssue, AuthoringRole,
    CANONICAL_FACET_SEQUENCE, CONVERGENCE_ROUTES, ClaimKind, ClosureNamespace,
    ClosureNamespaceIssue, CoordinateRole, DeclarationFragment, DeclarationGraph, ExportAlias,
    ExportAliasDerivation, Facet, FacetForm, FrontendRole, HOW_FACET_CONTENT, HygieneClass,
    LINKER_CONTRACT, LinkResolution, LinkResolutionIssue, META_EVALUATION_LOCKS, MetaStageLaw,
    OriginGraph, ProjectionClaim, ProjectionContract, ProjectionContractConstruction,
    ProjectionContractConstructionIssue, ProjectionProfileId, ProjectionProfileVersion,
    SourceCoordinate, SourceForm, Stage, SymbolIdentity, TopLevelForm, WHAT_FACET_CONTENT,
    WHEN_FACET_CONTENT, WHERE_FACET_CONTENT, WHO_FACET_CONTENT, WHY_FACET_CONTENT,
};

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
/// # The position cannot drift from the order
///
/// `ALL` and `slot` are generated from the SAME row list in one expansion.
/// There is no second place to write a position, so a roster whose fourth row
/// answers `2` is not a defect this stamp can express — it is a value nobody
/// can write down. That is the whole reason the stamp exists: the hand-kept
/// form of this pattern is a roster array beside a `match` returning numbers,
/// and those two are two things to keep true.
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
/// # The position is a `u8`, and that is a stated ceiling
///
/// `slot` answers a byte, so the stamp is for rosters of at most 256 rows. Past
/// that the count saturates rather than wrapping, which is a wrong answer given
/// quietly — so the stamp is not the tool for an unbounded vocabulary, and a
/// roster that grew that far would be a roster that stopped being closed.
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
/// This home owns the shared authoring algebra, and a closed roster of declared
/// words is an authoring shape rather than an identity, a refusal, or a value.
/// So this home stamps it. Rust exports `macro_rules!` at the crate root; that
/// placement is Rust's rule about macro namespacing and is not a root admission
/// of a semantic noun — the stamp declares no type of its own, reaches no band's
/// material, and owns no meaning. `scope_guard_version!` in band 02 is the
/// worked precedent for both halves of that sentence.
///
/// The `@`-prefixed rules below are the stamp's own recursion over its rows and
/// are not an invocation form.
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
            /// wrote: see [`ALL`](Self::ALL).
            #[must_use]
            $vis const fn slot(self) -> u8 {
                $crate::closed_register!(@slot self, 0u8, (), $($variant)+)
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

    (@slot $subject:expr, $position:expr, ($($arms:tt)*), $head:ident $($rest:ident)*) => {
        $crate::closed_register!(
            @slot
            $subject,
            $position.saturating_add(1),
            ($($arms)* Self::$head => $position,),
            $($rest)*
        )
    };

    (@slot $subject:expr, $position:expr, ($($arms:tt)*),) => {
        match $subject {
            $($arms)*
        }
    };
}
