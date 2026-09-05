//! One closed-seat storage and saturating increment mechanism for harness censuses.

macro_rules! declare_census {
    (
        $(#[$census_meta:meta])*
        $visibility:vis struct $census:ident {
            count: $count:ty,
            seat: $seat:ident,
            context { $( $context:ident: $context_type:ty, )* }
            array $counts:ident [$seat_count:expr] {
                $( $variant:ident => $field:ident, )+
            }
        }
    ) => {
        $(#[$census_meta])*
        $visibility struct $census {
            $( $context: $context_type, )*
            $counts: [$count; $seat_count],
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub(crate) enum $seat {
            $( $variant, )+
        }
    };
    (
        $(#[$census_meta:meta])*
        $visibility:vis struct $census:ident {
            count: $count:ty,
            seat: $seat:ident,
            context { $( $context:ident: $context_type:ty, )* }
            fields {
                $( $variant:ident => $field:ident, )+
            }
        }
    ) => {
        $(#[$census_meta])*
        $visibility struct $census {
            $( $context: $context_type, )*
            $( $field: $count, )+
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub(crate) enum $seat {
            $( $variant, )+
        }
    };
}

macro_rules! implement_census {
    (
        impl $census:ident {
            count: $count:ty,
            zero: $zero:expr,
            seat: $seat:ident,
            context { $( $context:ident: $context_type:ty, )* }
            array $counts:ident [$seat_count:expr] {
                $( $variant:ident => $field:ident, )+
            }
        }
    ) => {
        impl $census {
            pub(crate) const fn empty($( $context: $context_type, )*) -> Self {
                Self {
                    $( $context, )*
                    $counts: [$zero; $seat_count],
                }
            }

            pub(crate) fn increment(&mut self, seat: $seat, amount: $count) {
                let [$($field),+] = &mut self.$counts;
                match seat {
                    $(
                        $seat::$variant => *$field = $field.saturating_add(amount),
                    )+
                }
            }

            pub(crate) const fn count_at(&self, seat: $seat) -> $count {
                let [$($field),+] = &self.$counts;
                match seat {
                    $(
                        $seat::$variant => *$field,
                    )+
                }
            }
        }
    };
    (
        impl $census:ident {
            count: $count:ty,
            zero: $zero:expr,
            seat: $seat:ident,
            context { $( $context:ident: $context_type:ty, )* }
            fields {
                $( $variant:ident => $field:ident, )+
            }
        }
    ) => {
        impl $census {
            pub(crate) const fn empty($( $context: $context_type, )*) -> Self {
                Self {
                    $( $context, )*
                    $( $field: $zero, )+
                }
            }

            pub(crate) fn increment(&mut self, seat: $seat, amount: $count) {
                match seat {
                    $(
                        $seat::$variant => {
                            self.$field = self.$field.saturating_add(amount);
                        }
                    )+
                }
            }

            pub(crate) const fn count_at(&self, seat: $seat) -> $count {
                match seat {
                    $(
                        $seat::$variant => self.$field,
                    )+
                }
            }
        }
    };
}

pub(crate) use {declare_census, implement_census};
