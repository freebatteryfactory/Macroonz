#![doc = include_str!("../README.md")]

mod type_contract;
mod types;

pub use types::{
    AdmittedLimit, AdmittedPrefix, AdmittedRefusalFamily, Bounded, BoundedConstruction,
    CapacityAuthority, CauseId, CauseOrderDeclaration, CauseOrdinal, Commitment, CompletionPosture,
    ConstLimit, DeclaredCause, DeclaredCauseOrder, DeclaredMagnitude, FamilyAdmission,
    FamilyAdmissionCoverage, FamilyShape, FieldCardinality, Limit, LimitAdmissionProfile,
    LocalCauseKey, NonEmptyBounded, NonEmptyBoundedConstruction, OrderAdmission, OrderProjected,
    PositiveLimit, ReasonId, RefusalFamily, RefusalFamilyId, ReportTruncation, ShapeAdmission,
    ShapeCoherent, StopBound, TypedOrderCoherent, UnstatedMagnitude, admit_order,
    admit_order_projection, admit_shape,
};

/// Declares a closed enum together with its roster, stable names, descriptions, and positions.
///
/// The declaration supplies one row list. The expansion derives `ALL`, `slot`, `stable_name`, and `described` from that list, so none is maintained beside another.
///
/// # Bounds
///
/// The declared position supply admits [`CLOSED_REGISTER_ROW_CEILING`] rows. The ceiling belongs to this stamp implementation and is not a semantic limit on a vocabulary.
///
/// # Examples
///
/// ```
/// macroonz::closed_register! {
///     /// A small example roster.
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
            /// The declared roster, in declaration order.
            $vis const ALL: [Self; [$(Self::$variant),+].len()] = [$(Self::$variant),+];

            /// This row's zero-based position in the declared roster.
            #[must_use]
            $vis const fn slot(self) -> u8 {
                $crate::closed_register!(@supply pairing self, $($variant)+)
            }

            /// This row's declared stable name.
            #[must_use]
            $vis const fn stable_name(self) -> &'static str {
                match self {
                    $( Self::$variant => $stable, )+
                }
            }

            /// This row's declared description.
            #[must_use]
            $vis const fn described(self) -> &'static str {
                match self {
                    $( Self::$variant => $described, )+
                }
            }
        }
    };

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

    (@supplied [$($position:literal)*] length) => {
        [$($position),*].len()
    };

    (@supplied [$($position:literal)*] pairing $subject:expr, $($variant:ident)+) => {
        $crate::closed_register!(
            @position $subject,
            (),
            [$($position)*],
            $($variant)+
        )
    };

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

    (
        @position $subject:expr,
        ($($arms:tt)*),
        [],
        $head:ident $($rest:ident)*
    ) => {
        ::core::compile_error!(
            "closed_register!: this roster exceeds the stamp's declared position supply; the supply length is exported as `CLOSED_REGISTER_ROW_CEILING`"
        )
    };

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

/// The number of rows admitted by one [`closed_register!`] declaration.
///
/// The value is projected from the same position supply used by `slot`.
pub const CLOSED_REGISTER_ROW_CEILING: usize = crate::closed_register!(@supply length);
