//! The relation home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child so foreign endpoint references and repeated relation pairs are values a caller cannot forge.

use super::{
    CanonicalRelationPosition, KeyedRosterRelation, KeyedRosterRows, KeyedRosterRowsError,
    ReferencedRosterRow, RelationPair, RepeatedRelationPair, RepeatedRelationPairs,
    ResolvedRosterMember, RowResolutionError,
};
use crate::bounded::{Bounded, ForeignRosterReference, KeyedRoster, NonEmpty, NonEmptyError};
use core::borrow::Borrow;

impl<
    'rosters,
    Left,
    LeftKey,
    Right,
    RightKey,
    Payload,
    const LEFT: usize,
    const RIGHT: usize,
    const ROWS: usize,
> KeyedRosterRows<'rosters, Left, LeftKey, Right, RightKey, Payload, LEFT, RIGHT, ROWS>
{
    /// The left roster every row was resolved against.
    #[must_use]
    pub const fn left(&self) -> &'rosters KeyedRoster<Left, LeftKey, LEFT> {
        self.left
    }

    /// The right roster every row was resolved against.
    #[must_use]
    pub const fn right(&self) -> &'rosters KeyedRoster<Right, RightKey, RIGHT> {
        self.right
    }

    /// How many foreign-free rows are held.
    #[must_use]
    pub fn count(&self) -> usize {
        self.rows.len()
    }

    /// Whether no relation row was declared.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Reads every authored row with both resolved roster members and its caller-owned payload.
    pub fn indexed(
        &self,
    ) -> impl Iterator<Item = (usize, &LeftKey, &Left, &RightKey, &Right, &Payload)> {
        self.rows.iter().enumerate().map(|(index, row)| {
            (
                index,
                row.left_key,
                row.left_member,
                row.right_key,
                row.right_member,
                &row.payload,
            )
        })
    }

    /// Reads one authored row at a checked position.
    #[must_use]
    pub fn at(&self, index: usize) -> Option<(&LeftKey, &Left, &RightKey, &Right, &Payload)> {
        self.rows.as_slice().get(index).map(|row| {
            (
                row.left_key,
                row.left_member,
                row.right_key,
                row.right_member,
                &row.payload,
            )
        })
    }

    /// The authored row indices in canonical left-position then right-position order.
    ///
    /// Equal endpoint pairs retain authored order until duplicate posture is settled.
    #[must_use]
    pub fn canonical_indices(&self) -> &[usize] {
        self.canonical_indices.as_slice()
    }

    /// Reads one row by its canonical-order position.
    #[must_use]
    pub fn canonical_at(
        &self,
        index: usize,
    ) -> Option<(&LeftKey, &Left, &RightKey, &Right, &Payload)> {
        self.canonical_indices()
            .get(index)
            .and_then(|authored| self.at(*authored))
    }

    /// Reads every payload under one pair of borrowed roster keys.
    pub fn payloads_for<'reading, LeftQuery, RightQuery>(
        &'reading self,
        left: &LeftQuery,
        right: &RightQuery,
    ) -> impl Iterator<Item = &'reading Payload>
    where
        LeftKey: Borrow<LeftQuery>,
        LeftQuery: Eq + ?Sized,
        RightKey: Borrow<RightQuery>,
        RightQuery: Eq + ?Sized,
    {
        let left_position = self.left.index_of(left);
        let right_position = self.right.index_of(right);
        self.rows.iter().filter_map(move |row| {
            (Some(row.left_position) == left_position && Some(row.right_position) == right_position)
                .then_some(&row.payload)
        })
    }

    /// Promote these foreign-free rows into a relation where every endpoint pair occurs once.
    ///
    /// # Errors
    ///
    /// Returns every distinct repeated endpoint pair with its first and later authored positions.
    pub fn distinct(
        self,
    ) -> Result<
        KeyedRosterRelation<'rosters, Left, LeftKey, Right, RightKey, Payload, LEFT, RIGHT, ROWS>,
        RepeatedRelationPairs<ROWS>,
    > {
        match repeated_relation_pairs(&self) {
            Some(repeated) => Err(repeated),
            None => Ok(KeyedRosterRelation { rows: self }),
        }
    }
}

impl<
    'rosters,
    Left,
    LeftKey: Eq,
    Right,
    RightKey: Eq,
    Payload,
    const LEFT: usize,
    const RIGHT: usize,
    const ROWS: usize,
> KeyedRosterRows<'rosters, Left, LeftKey, Right, RightKey, Payload, LEFT, RIGHT, ROWS>
{
    /// Resolve one complete row offering against two existing keyed rosters.
    ///
    /// Row magnitude is settled before either endpoint projection runs.
    /// Every left reference is settled before the right projection begins.
    ///
    /// # Errors
    ///
    /// Returns row overflow, every foreign left reference, or every foreign right reference under that precedence.
    pub fn referenced(
        left: &'rosters KeyedRoster<Left, LeftKey, LEFT>,
        right: &'rosters KeyedRoster<Right, RightKey, RIGHT>,
        payloads: Vec<Payload>,
        left_key_of: impl FnMut(&Payload) -> LeftKey,
        right_key_of: impl FnMut(&Payload) -> RightKey,
    ) -> Result<Self, KeyedRosterRowsError<LeftKey, RightKey, ROWS>> {
        let payloads = Bounded::new(payloads).map_err(KeyedRosterRowsError::Overflow)?;
        let left_rows = resolve_rows(left, &payloads, left_key_of).map_err(left_refusal)?;
        let right_rows = resolve_rows(right, &payloads, right_key_of).map_err(right_refusal)?;
        let rows = payloads
            .into_vec()
            .into_iter()
            .zip(left_rows)
            .zip(right_rows)
            .map(row_from_resolved)
            .collect::<Vec<_>>();
        let rows = Bounded::new(rows).map_err(KeyedRosterRowsError::Overflow)?;
        let canonical_indices = canonical_indices(&rows)?;
        Ok(Self {
            left,
            right,
            rows,
            canonical_indices,
        })
    }
}

impl<
    'rosters,
    Left,
    LeftKey,
    Right,
    RightKey,
    Payload,
    const LEFT: usize,
    const RIGHT: usize,
    const ROWS: usize,
> KeyedRosterRelation<'rosters, Left, LeftKey, Right, RightKey, Payload, LEFT, RIGHT, ROWS>
{
    /// The foreign-free authored rows whose endpoint pairs this relation proves distinct.
    #[must_use]
    pub const fn rows(
        &self,
    ) -> &KeyedRosterRows<'rosters, Left, LeftKey, Right, RightKey, Payload, LEFT, RIGHT, ROWS>
    {
        &self.rows
    }

    /// Recover the foreign-free row value where a later caller posture permits repetition.
    #[must_use]
    pub fn into_rows(
        self,
    ) -> KeyedRosterRows<'rosters, Left, LeftKey, Right, RightKey, Payload, LEFT, RIGHT, ROWS> {
        self.rows
    }
}

impl<const N: usize> RepeatedRelationPair<N> {
    /// The resolved left-roster position of the repeated pair.
    #[must_use]
    pub const fn left_position(&self) -> usize {
        self.duplicate.key().left
    }

    /// The resolved right-roster position of the repeated pair.
    #[must_use]
    pub const fn right_position(&self) -> usize {
        self.duplicate.key().right
    }

    /// The pair's first authored row position.
    #[must_use]
    pub const fn first_position(&self) -> usize {
        self.duplicate.first_position()
    }

    /// Every later authored row position carrying the same pair.
    #[must_use]
    pub const fn repeated_positions(&self) -> &NonEmpty<usize, N> {
        self.duplicate.repeated_positions()
    }
}

impl<const N: usize> RepeatedRelationPairs<N> {
    /// Every distinct repeated pair in first-occurrence order.
    pub fn iter(&self) -> impl Iterator<Item = &RepeatedRelationPair<N>> {
        self.pairs.iter()
    }

    /// How many distinct endpoint pairs repeated.
    #[must_use]
    pub fn count(&self) -> usize {
        self.pairs.count()
    }
}

fn resolve_rows<'roster, Member, Key: Eq, Payload, const MEMBERS: usize, const ROWS: usize>(
    roster: &'roster KeyedRoster<Member, Key, MEMBERS>,
    payloads: &Bounded<Payload, ROWS>,
    mut key_of: impl FnMut(&Payload) -> Key,
) -> Result<Vec<ResolvedRosterMember<'roster, Member, Key>>, RowResolutionError<Key, ROWS>> {
    let mut resolved = Vec::with_capacity(payloads.len());
    let mut foreign = Vec::new();
    for (offered_position, payload) in payloads.iter().enumerate() {
        let key = key_of(payload);
        match roster.indexed_get(&key) {
            Some((position, retained, member)) => resolved.push(ResolvedRosterMember {
                position,
                key: retained,
                member,
            }),
            None => foreign.push(ForeignRosterReference::at(key, offered_position)),
        }
    }
    settle_resolved_rows(resolved, foreign)
}

fn settle_resolved_rows<Member, Key, const ROWS: usize>(
    resolved: Vec<ResolvedRosterMember<'_, Member, Key>>,
    foreign: Vec<ForeignRosterReference<Key>>,
) -> Result<Vec<ResolvedRosterMember<'_, Member, Key>>, RowResolutionError<Key, ROWS>> {
    match NonEmpty::new(foreign) {
        Ok(foreign) => Err(RowResolutionError::Foreign(foreign)),
        Err(NonEmptyError::Empty(_)) => Ok(resolved),
        Err(NonEmptyError::Overflow(overflow)) => Err(RowResolutionError::Overflow(overflow)),
    }
}

fn left_refusal<LeftKey, RightKey, const N: usize>(
    refusal: RowResolutionError<LeftKey, N>,
) -> KeyedRosterRowsError<LeftKey, RightKey, N> {
    match refusal {
        RowResolutionError::Overflow(overflow) => KeyedRosterRowsError::Overflow(overflow),
        RowResolutionError::Foreign(foreign) => KeyedRosterRowsError::ForeignLeft(foreign),
    }
}

fn right_refusal<LeftKey, RightKey, const N: usize>(
    refusal: RowResolutionError<RightKey, N>,
) -> KeyedRosterRowsError<LeftKey, RightKey, N> {
    match refusal {
        RowResolutionError::Overflow(overflow) => KeyedRosterRowsError::Overflow(overflow),
        RowResolutionError::Foreign(foreign) => KeyedRosterRowsError::ForeignRight(foreign),
    }
}

fn row_from_resolved<'rosters, Left, LeftKey, Right, RightKey, Payload>(
    ((payload, left), right): (
        (Payload, ResolvedRosterMember<'rosters, Left, LeftKey>),
        ResolvedRosterMember<'rosters, Right, RightKey>,
    ),
) -> ReferencedRosterRow<'rosters, Left, LeftKey, Right, RightKey, Payload> {
    ReferencedRosterRow {
        left_position: left.position,
        left_key: left.key,
        left_member: left.member,
        right_position: right.position,
        right_key: right.key,
        right_member: right.member,
        payload,
    }
}

fn canonical_indices<Left, LeftKey, Right, RightKey, Payload, const N: usize>(
    rows: &Bounded<ReferencedRosterRow<'_, Left, LeftKey, Right, RightKey, Payload>, N>,
) -> Result<Bounded<usize, N>, KeyedRosterRowsError<LeftKey, RightKey, N>> {
    let mut canonical = rows
        .iter()
        .enumerate()
        .map(|(authored, row)| CanonicalRelationPosition {
            authored,
            left: row.left_position,
            right: row.right_position,
        })
        .collect::<Vec<_>>();
    canonical.sort_by_key(|position| (position.left, position.right, position.authored));
    Bounded::new(
        canonical
            .into_iter()
            .map(|position| position.authored)
            .collect(),
    )
    .map_err(KeyedRosterRowsError::Overflow)
}

fn repeated_relation_pairs<
    Left,
    LeftKey,
    Right,
    RightKey,
    Payload,
    const LEFT: usize,
    const RIGHT: usize,
    const ROWS: usize,
>(
    rows: &KeyedRosterRows<'_, Left, LeftKey, Right, RightKey, Payload, LEFT, RIGHT, ROWS>,
) -> Option<RepeatedRelationPairs<ROWS>> {
    let pairs = rows.rows.mapped(|row| RelationPair {
        left: row.left_position,
        right: row.right_position,
    });
    let pairs = NonEmpty::from_bounded(pairs).ok()?;
    let duplicates = pairs.duplicate_keys()?;
    Some(RepeatedRelationPairs {
        pairs: duplicates.mapped(|duplicate| RepeatedRelationPair { duplicate }),
    })
}
