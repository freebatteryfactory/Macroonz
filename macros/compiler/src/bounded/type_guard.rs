//! The bounded home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's claims structural.
//! A list longer than its ceiling and a non-empty list with nothing in it are values nobody can build, rather than shapes something downstream has to check for.

use super::{Bounded, Capped, Capping, Empty, NonEmpty, NonEmptyError, Overflow};

impl<T, const N: usize> Bounded<T, N> {
    /// An empty collection under this ceiling.
    #[must_use]
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    /// Admits one complete ordered offering under this ceiling.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] when more than `N` items are offered.
    pub fn new(items: Vec<T>) -> Result<Self, Overflow> {
        if items.len() <= N {
            Ok(Self(items))
        } else {
            Err(Overflow {
                capacity: N,
                offered: items.len(),
            })
        }
    }

    /// Admits a fixed-arity offering whose fit is settled at compile time.
    #[must_use]
    pub fn from_array<const M: usize>(items: [T; M]) -> Self {
        const {
            assert!(
                M <= N,
                "a fixed list longer than the ceiling it is declared under"
            );
        }
        Self(Vec::from(items))
    }

    /// The held items.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    /// Reads the held items in order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    /// How many items are held.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether nothing is held.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Appends one item where the resulting collection fits under this ceiling.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] without changing the list where the appended item would exceed `N`.
    pub fn try_push(&mut self, item: T) -> Result<(), Overflow> {
        let offered = self.0.len().saturating_add(1);
        if offered > N {
            return Err(Overflow {
                capacity: N,
                offered,
            });
        }
        self.0.push(item);
        Ok(())
    }
}

impl<T, const N: usize> NonEmpty<T, N> {
    /// A non-empty collection holding exactly one item.
    #[must_use]
    pub const fn one(value: T) -> Self {
        const {
            assert!(
                N >= 1,
                "a non-empty list under a ceiling that admits no item"
            );
        }
        Self {
            head: value,
            tail: Vec::new(),
        }
    }

    /// Admits one complete ordered offering that is non-empty and under this ceiling.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] when nothing is offered, and [`Overflow`] when more than `N` items are.
    pub fn new(items: Vec<T>) -> Result<Self, NonEmptyError> {
        let offered = items.len();
        let mut rest = items.into_iter();
        let Some(head) = rest.next() else {
            return Err(NonEmptyError::Empty(Empty));
        };
        if offered <= N {
            Ok(Self {
                head,
                tail: rest.collect(),
            })
        } else {
            Err(NonEmptyError::Overflow(Overflow {
                capacity: N,
                offered,
            }))
        }
    }

    /// The first item, which this list always has.
    #[must_use]
    pub const fn first(&self) -> &T {
        &self.head
    }

    /// The first item and the rest, in order.
    #[must_use]
    pub fn split(&self) -> (&T, &[T]) {
        (&self.head, self.tail.as_slice())
    }

    /// Reads the held items in order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.into_iter()
    }

    /// How many items are held, which is never zero.
    #[must_use]
    pub fn count(&self) -> usize {
        self.tail.len().saturating_add(1)
    }
}

impl<'held, T, const N: usize> IntoIterator for &'held NonEmpty<T, N> {
    type Item = &'held T;
    type IntoIter = core::iter::Chain<core::iter::Once<&'held T>, core::slice::Iter<'held, T>>;

    fn into_iter(self) -> Self::IntoIter {
        core::iter::once(&self.head).chain(self.tail.iter())
    }
}

impl<T, const N: usize> Capped<T, N> {
    /// A capped collection that kept its complete lawful offering.
    #[must_use]
    pub const fn all(items: NonEmpty<T, N>) -> Self {
        Self {
            items,
            capping: Capping::Complete,
        }
    }

    /// Keeps the first item and the ordered prefix of the rest that fits, then records the exact omitted count.
    #[must_use]
    pub fn first_n(first: T, rest: impl Iterator<Item = T>) -> Self {
        const {
            assert!(N >= 1, "a capped list under a ceiling that admits no item");
        }
        let mut tail = Vec::new();
        let mut omitted = 0_usize;
        for item in rest {
            if tail.len() < N.saturating_sub(1) {
                tail.push(item);
            } else {
                omitted = omitted.saturating_add(1);
            }
        }
        Self {
            items: NonEmpty { head: first, tail },
            capping: capping_over(omitted),
        }
    }

    /// The items the list kept.
    #[must_use]
    pub const fn items(&self) -> &NonEmpty<T, N> {
        &self.items
    }

    /// Whether the list kept everything offered to it.
    #[must_use]
    pub const fn capping(&self) -> Capping {
        self.capping
    }
}

/// Reads the capping off the exact count of what was dropped.
const fn capping_over(omitted: usize) -> Capping {
    if omitted == 0 {
        Capping::Complete
    } else {
        Capping::Truncated { omitted }
    }
}
