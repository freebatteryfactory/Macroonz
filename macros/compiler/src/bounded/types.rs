//! The bounded home's declarations: three collections, the record of how one was capped, and the two ways construction refuses.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is what makes each ceiling structural rather than remembered.

#[path = "type_guard.rs"]
mod guard;

/// A list of at most `N` items.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bounded<T, const N: usize>(Vec<T>);

/// A list of at least one and at most `N` items.
///
/// The first item is a field, so non-emptiness is the shape of the value rather than a property a road checks.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonEmpty<T, const N: usize> {
    head: T,
    tail: Vec<T>,
}

/// A non-empty list together with how it was capped.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Capped<T, const N: usize> {
    items: NonEmpty<T, N>,
    capping: Capping,
}

/// Whether a capped list holds everything that was offered to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capping {
    /// Everything offered fit.
    Complete,
    /// The list filled and the rest was dropped.
    Truncated {
        /// How many offered items the list did not keep.
        omitted: usize,
    },
}

/// More items were offered than a bound admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Overflow {
    /// The most items the bound admits.
    pub capacity: usize,
    /// How many items were offered.
    pub offered: usize,
}

/// No item was offered where at least one is required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Empty;

/// How construction of a non-empty list refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonEmptyError {
    /// Nothing was offered.
    Empty(Empty),
    /// Too much was offered.
    Overflow(Overflow),
}
