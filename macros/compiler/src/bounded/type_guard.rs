//! The bounded home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's claims structural.
//! A list longer than its ceiling and a non-empty list with nothing in it are values nobody can build, rather than shapes something downstream has to check for.

use super::{
    Bounded, Capped, Capping, DuplicateKey, Empty, KeyedRoster, KeyedRosterError, NonEmpty,
    NonEmptyError, Overflow,
};
use core::borrow::Borrow;

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

impl<T, K, const N: usize> KeyedRoster<T, K, N> {
    /// A roster containing one member under its caller-declared key.
    #[must_use]
    pub const fn one(member: T, key: K) -> Self {
        Self {
            members: NonEmpty::one(member),
            keys: NonEmpty::one(key),
        }
    }

    /// The first member, which this roster always has.
    #[must_use]
    pub const fn first(&self) -> &T {
        self.members.first()
    }

    /// The key retained for the first member.
    #[must_use]
    pub const fn first_key(&self) -> &K {
        self.keys.first()
    }

    /// How many members are held, which is never zero.
    #[must_use]
    pub fn count(&self) -> usize {
        self.members.count()
    }

    /// Reads the members in declaration order.
    pub fn members(&self) -> impl Iterator<Item = &T> {
        self.members.iter()
    }

    /// Reads the retained keys in declaration order.
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.iter()
    }

    /// Reads every declaration index, retained key, and member together.
    pub fn indexed(&self) -> impl Iterator<Item = (usize, &K, &T)> {
        self.keys()
            .zip(self.members())
            .enumerate()
            .map(|(index, (key, member))| (index, key, member))
    }

    /// Reads the key and member at one checked declaration index.
    #[must_use]
    pub fn at(&self, index: usize) -> Option<(&K, &T)> {
        self.keys().zip(self.members()).nth(index)
    }

    /// Finds the declaration index of one borrowed key.
    #[must_use]
    pub fn index_of<Q>(&self, sought: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.keys().position(|key| key.borrow() == sought)
    }

    /// Finds the member held under one borrowed key.
    #[must_use]
    pub fn get<Q>(&self, sought: &Q) -> Option<&T>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        self.keys()
            .zip(self.members())
            .find_map(|(key, member)| (key.borrow() == sought).then_some(member))
    }
}

impl<T, K: Eq, const N: usize> KeyedRoster<T, K, N> {
    /// Admits one complete ordered offering under caller-declared unique keys.
    ///
    /// The offering's nonempty bounded magnitude is settled before the key projection runs.
    ///
    /// # Errors
    ///
    /// Returns [`Empty`] when nothing is offered, [`Overflow`] when more than `N` items are offered, and [`DuplicateKey`] coordinates for every distinct key that occurs more than once.
    pub fn new(
        members: Vec<T>,
        mut key_of: impl FnMut(&T) -> K,
    ) -> Result<Self, KeyedRosterError<K, N>> {
        let members = NonEmpty::new(members).map_err(keyed_magnitude_refusal)?;
        let mut groups = KeyGroups {
            head: KeyGroup {
                key: key_of(members.first()),
                first: 0,
                repeated: Vec::new(),
            },
            tail: Vec::new(),
        };
        for (index, member) in members.iter().enumerate().skip(1) {
            groups.insert(key_of(member), index);
        }
        match groups.admit() {
            KeyAdmission::Unique(keys) => Ok(Self { members, keys }),
            KeyAdmission::Duplicated(duplicates) => {
                Err(KeyedRosterError::DuplicateKeys(duplicates))
            }
        }
    }
}

impl<K, const N: usize> DuplicateKey<K, N> {
    /// The caller-declared key that occurred more than once.
    #[must_use]
    pub const fn key(&self) -> &K {
        &self.key
    }

    /// The zero-based declaration position of the first occurrence.
    #[must_use]
    pub const fn first_position(&self) -> usize {
        self.first
    }

    /// The zero-based declaration positions of every later occurrence.
    #[must_use]
    pub const fn repeated_positions(&self) -> &NonEmpty<usize, N> {
        &self.repeated
    }
}

fn keyed_magnitude_refusal<K, const N: usize>(error: NonEmptyError) -> KeyedRosterError<K, N> {
    match error {
        NonEmptyError::Empty(empty) => KeyedRosterError::Empty(empty),
        NonEmptyError::Overflow(overflow) => KeyedRosterError::Overflow(overflow),
    }
}

struct KeyGroup<K> {
    key: K,
    first: usize,
    repeated: Vec<usize>,
}

struct KeyGroups<K> {
    head: KeyGroup<K>,
    tail: Vec<KeyGroup<K>>,
}

enum KeyAdmission<K, const N: usize> {
    Unique(NonEmpty<K, N>),
    Duplicated(NonEmpty<DuplicateKey<K, N>, N>),
}

impl<K: Eq> KeyGroups<K> {
    fn insert(&mut self, key: K, index: usize) {
        if self.head.key == key {
            self.head.repeated.push(index);
            return;
        }
        if let Some(group) = self.tail.iter_mut().find(|group| group.key == key) {
            group.repeated.push(index);
            return;
        }
        self.tail.push(KeyGroup {
            key,
            first: index,
            repeated: Vec::new(),
        });
    }

    fn admit<const N: usize>(self) -> KeyAdmission<K, N> {
        let mut tail = self.tail.into_iter();
        let head = match admitted_group(self.head) {
            Ok(key) => key,
            Err(duplicate) => {
                return KeyAdmission::Duplicated(NonEmpty {
                    head: duplicate,
                    tail: tail
                        .filter_map(|group| admitted_group(group).err())
                        .collect(),
                });
            }
        };
        let mut unique = Vec::new();
        while let Some(group) = tail.next() {
            match admitted_group(group) {
                Ok(key) => unique.push(key),
                Err(duplicate) => {
                    return KeyAdmission::Duplicated(NonEmpty {
                        head: duplicate,
                        tail: tail
                            .filter_map(|remaining| admitted_group(remaining).err())
                            .collect(),
                    });
                }
            }
        }
        KeyAdmission::Unique(NonEmpty { head, tail: unique })
    }
}

fn admitted_group<K, const N: usize>(group: KeyGroup<K>) -> Result<K, DuplicateKey<K, N>> {
    let KeyGroup {
        key,
        first,
        repeated,
    } = group;
    let mut repeated = repeated.into_iter();
    if let Some(repeated_head) = repeated.next() {
        return Err(DuplicateKey {
            key,
            first,
            repeated: NonEmpty {
                head: repeated_head,
                tail: repeated.collect(),
            },
        });
    }
    Ok(key)
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
