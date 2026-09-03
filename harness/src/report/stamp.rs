//! The private stamp for change readings that retain one before-and-after pair.

macro_rules! declare_change_pair {
    (
        $(#[$meta:meta])*
        $visibility:vis struct $name:ident {
            context { $( $context:ident: $context_type:ty, )* }
            value: $value:ty,
        }
    ) => {
        $(#[$meta])*
        $visibility struct $name {
            $( $context: $context_type, )*
            before: $value,
            after: $value,
        }
    };
}

macro_rules! implement_copy_change_pair {
    (
        $name:ident {
            context { $( $context:ident: $context_type:ty => $context_doc:literal, )* }
            value: $value:ty,
            construction: $construction_doc:literal,
            before: $before_doc:literal,
            after: $after_doc:literal,
        }
    ) => {
        impl $name {
            #[doc = $construction_doc]
            #[must_use]
            pub(in crate::report) const fn between(
                $( $context: $context_type, )*
                before: $value,
                after: $value,
            ) -> Self {
                Self {
                    $( $context, )*
                    before,
                    after,
                }
            }

            $(
                #[doc = $context_doc]
                #[must_use]
                pub const fn $context(self) -> $context_type {
                    self.$context
                }
            )*

            #[doc = $before_doc]
            #[must_use]
            pub const fn before(self) -> $value {
                self.before
            }

            #[doc = $after_doc]
            #[must_use]
            pub const fn after(self) -> $value {
                self.after
            }
        }
    };
}

macro_rules! implement_borrowed_change_pair {
    (
        $name:ident {
            context { $( $context:ident: $context_type:ty => $context_doc:literal, )* }
            value: $value:ty,
            construction: $construction_doc:literal,
            before: $before_doc:literal,
            after: $after_doc:literal,
        }
    ) => {
        impl $name {
            #[doc = $construction_doc]
            #[must_use]
            pub(in crate::report) const fn between(
                $( $context: $context_type, )*
                before: $value,
                after: $value,
            ) -> Self {
                Self {
                    $( $context, )*
                    before,
                    after,
                }
            }

            $(
                #[doc = $context_doc]
                #[must_use]
                pub const fn $context(&self) -> &$context_type {
                    &self.$context
                }
            )*

            #[doc = $before_doc]
            #[must_use]
            pub const fn before(&self) -> &$value {
                &self.before
            }

            #[doc = $after_doc]
            #[must_use]
            pub const fn after(&self) -> &$value {
                &self.after
            }
        }
    };
}

pub(crate) use {declare_change_pair, implement_borrowed_change_pair, implement_copy_change_pair};
