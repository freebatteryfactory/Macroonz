//! The private stamp that keeps one destination variant beside its one emitted spelling.

macro_rules! vocabulary {
    (
        $(#[$note:meta])*
        pub enum $name:ident {
            $( $(#[$row:meta])* $variant:ident = $spelling:literal ),+ $(,)?
        }
        spelling = $spelling_note:literal;
    ) => {
        $(#[$note])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $( $(#[$row])* $variant, )+
        }

        impl $name {
            #[doc = $spelling_note]
            #[must_use]
            pub const fn spelling(self) -> &'static str {
                match self {
                    $( Self::$variant => $spelling, )+
                }
            }
        }
    };
}

pub(super) use vocabulary;
