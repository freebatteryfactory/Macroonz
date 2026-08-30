//! The bounded home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's claims structural.
//! A list longer than its ceiling and a non-empty list with nothing in it are values nobody can build, rather than shapes something downstream has to check for.

use super::{
    Bounded, Capped, Capping, DuplicateKey, Empty, ForeignRosterReference, KeyedRoster,
    KeyedRosterAssignment, KeyedRosterAssignmentError, KeyedRosterError, NonEmpty, NonEmptyError,
    Overflow, UnassignedRosterMember,
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
        key_of: impl FnMut(&T) -> K,
    ) -> Result<Self, KeyedRosterError<K, N>> {
        let members = NonEmpty::new(members).map_err(keyed_magnitude_refusal)?;
        let keys = project_keys(&members, key_of);
        match admit_keys(keys) {
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

fn project_keys<T, K, const N: usize>(
    members: &NonEmpty<T, N>,
    mut key_of: impl FnMut(&T) -> K,
) -> NonEmpty<K, N> {
    let (head, tail) = members.split();
    NonEmpty {
        head: key_of(head),
        tail: tail.iter().map(key_of).collect(),
    }
}

fn admit_keys<K: Eq, const N: usize>(keys: NonEmpty<K, N>) -> KeyAdmission<K, N> {
    let NonEmpty { head, tail } = keys;
    let mut groups = KeyGroups {
        head: KeyGroup {
            key: head,
            first: 0,
            repeated: Vec::new(),
        },
        tail: Vec::new(),
    };
    for (index, key) in tail.into_iter().enumerate() {
        groups.insert(key, index.saturating_add(1));
    }
    groups.admit()
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

impl<D, K, P, S, const N: usize> KeyedRosterAssignment<D, K, P, S, N> {
    /// The complete caller-keyed denominator retained by this assignment.
    #[must_use]
    pub const fn denominator(&self) -> &KeyedRoster<D, K, N> {
        &self.denominator
    }

    /// The payload roster aligned with the denominator and keyed by caller-declared seats.
    #[must_use]
    pub const fn payloads(&self) -> &KeyedRoster<P, S, N> {
        &self.payloads
    }

    /// How many denominator members and aligned payloads are held.
    #[must_use]
    pub fn count(&self) -> usize {
        self.denominator.count()
    }

    /// The first denominator member and its assigned payload.
    #[must_use]
    pub const fn first(&self) -> (&K, &D, &S, &P) {
        (
            self.denominator.first_key(),
            self.denominator.first(),
            self.payloads.first_key(),
            self.payloads.first(),
        )
    }

    /// Reads every denominator member and aligned payload in denominator order.
    pub fn indexed(&self) -> impl Iterator<Item = (usize, &K, &D, &S, &P)> {
        self.denominator
            .indexed()
            .zip(self.payloads.keys().zip(self.payloads.members()))
            .map(|((index, key, member), (seat, payload))| (index, key, member, seat, payload))
    }

    /// Reads one denominator member and aligned payload at a checked index.
    #[must_use]
    pub fn at(&self, index: usize) -> Option<(&K, &D, &S, &P)> {
        self.denominator
            .at(index)
            .zip(self.payloads.at(index))
            .map(|((key, member), (seat, payload))| (key, member, seat, payload))
    }

    /// Finds one denominator member and aligned payload through a borrowed key.
    #[must_use]
    pub fn get<Q>(&self, sought: &Q) -> Option<(&D, &S, &P)>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let index = self.denominator.index_of(sought)?;
        self.at(index)
            .map(|(_key, member, seat, payload)| (member, seat, payload))
    }
}

impl<D, K: Eq, P, S: Eq, const N: usize> KeyedRosterAssignment<D, K, P, S, N> {
    /// Completes one payload assignment over an existing caller-keyed denominator.
    ///
    /// Payload magnitude is settled before either key projection runs.
    /// Reference membership and uniqueness are settled before payload-seat keys are projected.
    ///
    /// # Errors
    ///
    /// Returns [`KeyedRosterAssignmentError`] with the first structural refusal class reached by the declared construction order.
    pub fn complete(
        denominator: KeyedRoster<D, K, N>,
        payloads: Vec<P>,
        reference_of: impl FnMut(&P) -> K,
        seat_of: impl FnMut(&P) -> S,
    ) -> Result<Self, KeyedRosterAssignmentError<K, S, N>> {
        let payloads = NonEmpty::new(payloads).map_err(assignment_magnitude_refusal::<K, S, N>)?;
        let references = project_keys(&payloads, reference_of);
        let (references, positions) = match admit_references(&denominator, references) {
            ReferenceAdmission::Lawful { keys, positions } => (keys, positions),
            ReferenceAdmission::Foreign(foreign) => {
                return Err(KeyedRosterAssignmentError::ForeignReferences(foreign));
            }
        };
        if let KeyAdmission::Duplicated(duplicates) = admit_keys(references) {
            return Err(KeyedRosterAssignmentError::DuplicateReferences(duplicates));
        }
        let seats = project_keys(&payloads, seat_of);
        let seats = match admit_keys(seats) {
            KeyAdmission::Unique(seats) => seats,
            KeyAdmission::Duplicated(duplicates) => {
                return Err(KeyedRosterAssignmentError::ReusedPayloadSeats(duplicates));
            }
        };
        let denominator = match settle_completeness(denominator, &positions) {
            AssignmentCompleteness::Complete(denominator) => denominator,
            AssignmentCompleteness::Missing(missing) => {
                return Err(KeyedRosterAssignmentError::MissingMembers(missing));
            }
        };
        let payloads = align_payloads(payloads, seats, positions);
        Ok(Self {
            denominator,
            payloads,
        })
    }
}

impl<K> ForeignRosterReference<K> {
    /// The denominator key named by the offered payload.
    #[must_use]
    pub const fn key(&self) -> &K {
        &self.key
    }

    /// The zero-based offered-payload position carrying the foreign reference.
    #[must_use]
    pub const fn offered_position(&self) -> usize {
        self.offered_position
    }
}

impl<K> UnassignedRosterMember<K> {
    /// The denominator key for which no payload was offered.
    #[must_use]
    pub const fn key(&self) -> &K {
        &self.key
    }

    /// The zero-based denominator position for which no payload was offered.
    #[must_use]
    pub const fn denominator_position(&self) -> usize {
        self.denominator_position
    }
}

enum ReferenceAdmission<K, const N: usize> {
    Lawful {
        keys: NonEmpty<K, N>,
        positions: NonEmpty<usize, N>,
    },
    Foreign(NonEmpty<ForeignRosterReference<K>, N>),
}

fn admit_references<D, K: Eq, const N: usize>(
    denominator: &KeyedRoster<D, K, N>,
    references: NonEmpty<K, N>,
) -> ReferenceAdmission<K, N> {
    let NonEmpty { head, tail } = references;
    let Some(head_position) = denominator.index_of(&head) else {
        let foreign_tail = tail
            .into_iter()
            .enumerate()
            .filter_map(|(offset, key)| {
                denominator
                    .index_of(&key)
                    .is_none()
                    .then_some(ForeignRosterReference {
                        key,
                        offered_position: offset.saturating_add(1),
                    })
            })
            .collect();
        return ReferenceAdmission::Foreign(NonEmpty {
            head: ForeignRosterReference {
                key: head,
                offered_position: 0,
            },
            tail: foreign_tail,
        });
    };
    admit_references_after_lawful_head(denominator, head, head_position, tail)
}

fn admit_references_after_lawful_head<D, K: Eq, const N: usize>(
    denominator: &KeyedRoster<D, K, N>,
    head: K,
    head_position: usize,
    tail: Vec<K>,
) -> ReferenceAdmission<K, N> {
    let mut lawful_keys = Vec::new();
    let mut lawful_positions = Vec::new();
    let mut foreign_head = None;
    let mut foreign_tail = Vec::new();
    for (offset, key) in tail.into_iter().enumerate() {
        let offered_position = offset.saturating_add(1);
        if let Some(position) = denominator.index_of(&key) {
            lawful_keys.push(key);
            lawful_positions.push(position);
        } else {
            let foreign = ForeignRosterReference {
                key,
                offered_position,
            };
            if foreign_head.is_none() {
                foreign_head = Some(foreign);
            } else {
                foreign_tail.push(foreign);
            }
        }
    }
    if let Some(foreign) = foreign_head {
        ReferenceAdmission::Foreign(NonEmpty {
            head: foreign,
            tail: foreign_tail,
        })
    } else {
        ReferenceAdmission::Lawful {
            keys: NonEmpty {
                head,
                tail: lawful_keys,
            },
            positions: NonEmpty {
                head: head_position,
                tail: lawful_positions,
            },
        }
    }
}

enum AssignmentCompleteness<D, K, const N: usize> {
    Complete(KeyedRoster<D, K, N>),
    Missing(NonEmpty<UnassignedRosterMember<K>, N>),
}

enum KeyCompleteness<K, const N: usize> {
    Complete(NonEmpty<K, N>),
    Missing(NonEmpty<UnassignedRosterMember<K>, N>),
}

enum AssignmentStanding {
    Assigned,
    Missing,
}

fn settle_completeness<D, K, const N: usize>(
    denominator: KeyedRoster<D, K, N>,
    positions: &NonEmpty<usize, N>,
) -> AssignmentCompleteness<D, K, N> {
    let KeyedRoster { members, keys } = denominator;
    let NonEmpty { head, tail } = keys;
    let first_assigned = positions.iter().any(|position| *position == 0);
    let mut completeness = if first_assigned {
        KeyCompleteness::Complete(NonEmpty {
            head,
            tail: Vec::new(),
        })
    } else {
        KeyCompleteness::Missing(NonEmpty {
            head: UnassignedRosterMember {
                key: head,
                denominator_position: 0,
            },
            tail: Vec::new(),
        })
    };
    for (offset, key) in tail.into_iter().enumerate() {
        let denominator_position = offset.saturating_add(1);
        let standing = if positions
            .iter()
            .any(|position| *position == denominator_position)
        {
            AssignmentStanding::Assigned
        } else {
            AssignmentStanding::Missing
        };
        completeness = completeness.push(key, denominator_position, standing);
    }
    match completeness {
        KeyCompleteness::Complete(complete_keys) => AssignmentCompleteness::Complete(KeyedRoster {
            members,
            keys: complete_keys,
        }),
        KeyCompleteness::Missing(missing) => AssignmentCompleteness::Missing(missing),
    }
}

impl<K, const N: usize> KeyCompleteness<K, N> {
    fn push(self, key: K, denominator_position: usize, standing: AssignmentStanding) -> Self {
        match (self, standing) {
            (Self::Complete(mut keys), AssignmentStanding::Assigned) => {
                keys.tail.push(key);
                Self::Complete(keys)
            }
            (Self::Complete(_keys), AssignmentStanding::Missing) => Self::Missing(NonEmpty {
                head: UnassignedRosterMember {
                    key,
                    denominator_position,
                },
                tail: Vec::new(),
            }),
            (Self::Missing(missing), AssignmentStanding::Assigned) => Self::Missing(missing),
            (Self::Missing(mut missing), AssignmentStanding::Missing) => {
                missing.tail.push(UnassignedRosterMember {
                    key,
                    denominator_position,
                });
                Self::Missing(missing)
            }
        }
    }
}

struct PendingAssignment<P, S> {
    denominator_position: usize,
    payload: P,
    seat: S,
}

fn align_payloads<P, S, const N: usize>(
    payloads: NonEmpty<P, N>,
    seats: NonEmpty<S, N>,
    positions: NonEmpty<usize, N>,
) -> KeyedRoster<P, S, N> {
    let NonEmpty {
        head: payload_head,
        tail: payload_tail,
    } = payloads;
    let NonEmpty {
        head: seat_head,
        tail: seat_tail,
    } = seats;
    let NonEmpty {
        head: position_head,
        tail: position_tail,
    } = positions;
    let mut head = PendingAssignment {
        denominator_position: position_head,
        payload: payload_head,
        seat: seat_head,
    };
    let mut tail = position_tail
        .into_iter()
        .zip(payload_tail)
        .zip(seat_tail)
        .map(
            |((denominator_position, payload), seat)| PendingAssignment {
                denominator_position,
                payload,
                seat,
            },
        )
        .collect::<Vec<_>>();
    for assignment in &mut tail {
        if assignment.denominator_position < head.denominator_position {
            core::mem::swap(&mut head, assignment);
        }
    }
    tail.sort_by_key(|assignment| assignment.denominator_position);
    let mut ordered_payload_tail = Vec::with_capacity(tail.len());
    let mut ordered_seat_tail = Vec::with_capacity(tail.len());
    for assignment in tail {
        ordered_payload_tail.push(assignment.payload);
        ordered_seat_tail.push(assignment.seat);
    }
    KeyedRoster {
        members: NonEmpty {
            head: head.payload,
            tail: ordered_payload_tail,
        },
        keys: NonEmpty {
            head: head.seat,
            tail: ordered_seat_tail,
        },
    }
}

fn assignment_magnitude_refusal<K, S, const N: usize>(
    error: NonEmptyError,
) -> KeyedRosterAssignmentError<K, S, N> {
    match error {
        NonEmptyError::Empty(empty) => KeyedRosterAssignmentError::Empty(empty),
        NonEmptyError::Overflow(overflow) => KeyedRosterAssignmentError::Overflow(overflow),
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
