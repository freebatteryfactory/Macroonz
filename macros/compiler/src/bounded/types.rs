//! The bounded home's collection shapes, capping posture, and construction refusals.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is what makes each ceiling structural rather than remembered.

#[path = "type_guard.rs"]
mod guard;

pub(crate) use guard::first_duplicate_position;

/// An ordered collection of at most `N` items.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bounded<T, const N: usize>(Vec<T>);

/// An ordered collection of at least one and at most `N` items.
///
/// The first item is a field, so non-emptiness is the shape of the value rather than a property a road checks.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonEmpty<T, const N: usize> {
    head: T,
    tail: Vec<T>,
}

/// A non-empty bounded roster whose caller-declared keys are unique.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyedRoster<T, K, const N: usize> {
    members: NonEmpty<T, N>,
    keys: NonEmpty<K, N>,
}

/// One payload assignment for every member of a caller-keyed denominator roster.
///
/// The denominator and payload rosters share denominator order by construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyedRosterAssignment<D, K, P, S, const N: usize> {
    denominator: KeyedRoster<D, K, N>,
    payloads: KeyedRoster<P, S, N>,
}

/// One duplicated key and every declaration position at which it occurred.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DuplicateKey<K, const N: usize> {
    key: K,
    first: usize,
    repeated: NonEmpty<usize, N>,
}

/// One offered item whose referenced roster key is foreign.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForeignRosterReference<K> {
    key: K,
    offered_position: usize,
}

/// One denominator member for which no payload was offered.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnassignedRosterMember<K> {
    key: K,
    denominator_position: usize,
}

/// A non-empty ordered collection together with its constructor-derived capping posture.
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

/// The exact magnitude refused because more items were offered than a ceiling admits.
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

/// How construction of a required non-empty collection refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonEmptyError {
    /// Nothing was offered.
    Empty(Empty),
    /// Too much was offered.
    Overflow(Overflow),
}

/// How construction of a caller-keyed roster refuses.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyedRosterError<K, const N: usize> {
    /// Nothing was offered.
    Empty(Empty),
    /// Too much was offered.
    Overflow(Overflow),
    /// One or more caller-declared keys occurred more than once.
    DuplicateKeys(NonEmpty<DuplicateKey<K, N>, N>),
}

/// How an offered payload roster refuses exact assignment to a keyed denominator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyedRosterAssignmentError<K, S, const N: usize> {
    /// Nothing was offered.
    Empty(Empty),
    /// Too much was offered.
    Overflow(Overflow),
    /// One or more offered payloads name a key outside the denominator.
    ForeignReferences(NonEmpty<ForeignRosterReference<K>, N>),
    /// One or more denominator keys were referenced more than once.
    DuplicateReferences(NonEmpty<DuplicateKey<K, N>, N>),
    /// One or more caller-declared payload-seat keys were reused.
    ReusedPayloadSeats(NonEmpty<DuplicateKey<S, N>, N>),
    /// One or more denominator members received no payload.
    MissingMembers(NonEmpty<UnassignedRosterMember<K>, N>),
}
