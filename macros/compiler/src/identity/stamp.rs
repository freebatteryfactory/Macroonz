//! The identity home's declaration stamps: the subject roster, and the static human projection with the const seam it builds on.

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

/// One static rendering's bytes, at the fixed width the caller declared.
///
/// Written for the `const` item [`human_projection!`] builds: a width other than the rendering's own length stops the compiler rather than handing a reader a padded or cut projection.
#[must_use]
pub(crate) const fn static_bytes<const N: usize>(text: &str) -> [u8; N] {
    assert!(
        text.len() == N,
        "a declared width that is not the rendering's own length"
    );
    let mut rendered = [0u8; N];
    let mut source = text.as_bytes();
    let mut sink: &mut [u8] = &mut rendered;
    while let Some((seat, open)) = sink.split_first_mut() {
        let Some((byte, remaining)) = source.split_first() else {
            break;
        };
        *seat = *byte;
        sink = open;
        source = remaining;
    }
    rendered
}

/// Projects one STATIC rendering, proving at COMPILE TIME that it fits.
///
/// [`HumanProjection::projected`](crate::identity::HumanProjection::projected) reads a runtime length and may refuse, and a caller that swallowed that refusal with an empty fallback would be silently deleting an explanation.
/// Where the material is static the length is a compile-time fact instead, and no refusal road appears anywhere between the literal and the projection.
macro_rules! human_projection {
    ($text:literal) => {{
        const RENDERED: [u8; $text.len()] = $crate::identity::static_bytes($text);
        $crate::identity::HumanProjection::proven(RENDERED)
    }};
}

pub(crate) use human_projection;
