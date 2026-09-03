//! Private stamps that keep one recipe vocabulary row beside its declared reading.

macro_rules! named_vocabulary {
    (
        $(#[$note:meta])*
        pub enum $name:ident {
            $( $(#[$row:meta])* $variant:ident = $declared:literal ),+ $(,)?
        }
    ) => {
        $(#[$note])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $( $(#[$row])* $variant, )+
        }

        impl $name {
            /// Reads the stable declared name.
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $( Self::$variant => $declared, )+
                }
            }
        }
    };
}

pub(super) use named_vocabulary;
