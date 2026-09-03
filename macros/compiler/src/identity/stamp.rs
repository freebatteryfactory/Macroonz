//! The identity-subject declaration stamp.

/// Declares one roster of identity subjects under one stem, as the home's README shows.
///
/// Each row becomes a marker type carrying its declared name, and the roster settles both ways it could fail to separate while it compiles: a name outside the context grammar, and a name two rows declare.
/// A declared name is lowercase ASCII letters and digits in `-`-joined segments, with no leading, trailing, or doubled separator.
#[macro_export]
macro_rules! subjects {
    (stem = $stem:expr; $( $(#[$note:meta])* $name:ident = $declared:literal ),+ $(,)?) => {
        $(
            $(#[$note])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;

            impl $crate::identity::Subject for $name {
                const NAME: &'static str = $declared;
                const STEM: &'static str = $stem;
            }
        )+

        const _: () = ::core::assert!(
            $crate::identity::names_are_separating(&[$($declared),+]),
            "a subject name outside the derive-key grammar, or one two subjects declare",
        );
    };
}
