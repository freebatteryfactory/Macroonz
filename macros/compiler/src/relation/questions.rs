//! Pure structural questions over already informed relation values.
//!
//! These operations compute answers.
//! The caller separately states which answer is lawful.

use super::{
    CompletenessPosture, CompletenessStanding, CyclePosture, CycleStanding, DensityPosture,
    DensityStanding, EmptyPosture, KeyedRosterRows, OccupancyStanding, Reachability,
    ReachabilityError, ReferencedRosterRow, RepetitionPosture, RepetitionStanding,
    RosterRelationStanding, RowOrder, SameRosterRequired, SelfRelationPosture,
    SelfRelationStanding, StructuralMismatch, StructuralRequirement,
};
use crate::bounded::{Bounded, NonEmpty};

impl<Answer> StructuralRequirement<Answer> {
    /// States the exact answer one caller requires from one structural question.
    pub const fn stated(required: Answer) -> Self {
        Self { required }
    }

    /// The answer the caller requires.
    #[must_use]
    pub const fn required(&self) -> &Answer {
        &self.required
    }
}

impl<Answer: Eq> StructuralRequirement<Answer> {
    /// Settles one computed answer against this caller-declared requirement.
    ///
    /// # Errors
    ///
    /// Returns both answers when the computed answer differs from the required answer.
    pub fn settle(self, observed: Answer) -> Result<Answer, StructuralMismatch<Answer>> {
        if self.required == observed {
            Ok(observed)
        } else {
            Err(StructuralMismatch {
                required: self.required,
                observed,
            })
        }
    }
}

impl<Answer> StructuralMismatch<Answer> {
    /// The answer the caller required.
    #[must_use]
    pub const fn required(&self) -> &Answer {
        &self.required
    }

    /// The answer the structural question computed.
    #[must_use]
    pub const fn observed(&self) -> &Answer {
        &self.observed
    }
}

impl EmptyPosture {
    /// The occupancy requirement expressed by this posture, when it constrains occupancy.
    #[must_use]
    pub const fn requirement(self) -> Option<StructuralRequirement<OccupancyStanding>> {
        match self {
            Self::Allowed => None,
            Self::Refusal => Some(StructuralRequirement::stated(OccupancyStanding::Populated)),
        }
    }
}

impl RepetitionPosture {
    /// The repetition requirement expressed by this posture, when it constrains repetition.
    #[must_use]
    pub const fn requirement(self) -> Option<StructuralRequirement<RepetitionStanding>> {
        match self {
            Self::Allowed => None,
            Self::Refusal => Some(StructuralRequirement::stated(RepetitionStanding::Distinct)),
        }
    }
}

impl CompletenessPosture {
    /// The completeness requirement expressed by this posture, when it constrains coverage.
    #[must_use]
    pub const fn requirement(self) -> Option<StructuralRequirement<CompletenessStanding>> {
        match self {
            Self::Partial => None,
            Self::Total => Some(StructuralRequirement::stated(
                CompletenessStanding::Complete,
            )),
        }
    }
}

impl DensityPosture {
    /// The density requirement expressed by this posture, when it constrains pair coverage.
    #[must_use]
    pub const fn requirement(self) -> Option<StructuralRequirement<DensityStanding>> {
        match self {
            Self::Sparse => None,
            Self::Dense => Some(StructuralRequirement::stated(DensityStanding::Dense)),
        }
    }
}

impl SelfRelationPosture {
    /// The self-relation requirement expressed by this posture, when it constrains self relations.
    #[must_use]
    pub const fn requirement(self) -> Option<StructuralRequirement<SelfRelationStanding>> {
        match self {
            Self::Allowed => None,
            Self::Refusal => Some(StructuralRequirement::stated(SelfRelationStanding::Absent)),
        }
    }
}

impl CyclePosture {
    /// The cycle requirement expressed by this posture, when it constrains cycles.
    #[must_use]
    pub const fn requirement(self) -> Option<StructuralRequirement<CycleStanding>> {
        match self {
            Self::Allowed => None,
            Self::Refusal => Some(StructuralRequirement::stated(CycleStanding::Acyclic)),
        }
    }
}

impl<const N: usize> Reachability<N> {
    /// Reachable roster positions in roster order.
    pub fn reachable_positions(&self) -> impl Iterator<Item = usize> + '_ {
        self.reachable.iter().copied()
    }

    /// Unreachable roster positions in roster order.
    pub fn unreachable_positions(&self) -> impl Iterator<Item = usize> + '_ {
        self.unreachable.iter().copied()
    }

    /// Whether every member is reachable from the declared root.
    #[must_use]
    pub fn standing(&self) -> CompletenessStanding {
        if self.unreachable.is_empty() {
            CompletenessStanding::Complete
        } else {
            CompletenessStanding::Partial
        }
    }
}

impl<
    Left,
    LeftKey,
    Right,
    RightKey,
    Payload,
    const LEFT: usize,
    const RIGHT: usize,
    const ROWS: usize,
> KeyedRosterRows<'_, Left, LeftKey, Right, RightKey, Payload, LEFT, RIGHT, ROWS>
{
    /// Reads one row under the caller-selected stable order.
    #[must_use]
    pub fn at_in(
        &self,
        order: RowOrder,
        index: usize,
    ) -> Option<(&LeftKey, &Left, &RightKey, &Right, &Payload)> {
        match order {
            RowOrder::Authored => self.at(index),
            RowOrder::Canonical => self.canonical_at(index),
        }
    }

    /// Whether this relation holds no row or at least one.
    #[must_use]
    pub fn occupancy_standing(&self) -> OccupancyStanding {
        if self.is_empty() {
            OccupancyStanding::Empty
        } else {
            OccupancyStanding::Populated
        }
    }

    /// Whether every endpoint pair occurs once or at least one pair repeats.
    #[must_use]
    pub fn repetition_standing(&self) -> RepetitionStanding {
        let repeated = self.rows.iter().enumerate().any(|(position, row)| {
            self.rows
                .iter()
                .skip(position.saturating_add(1))
                .any(|later| {
                    later.left_position == row.left_position
                        && later.right_position == row.right_position
                })
        });
        if repeated {
            RepetitionStanding::Repeated
        } else {
            RepetitionStanding::Distinct
        }
    }

    /// Whether every left-roster member occurs in at least one row.
    #[must_use]
    pub fn left_completeness(&self) -> CompletenessStanding {
        completeness_over(self.left.count(), |position| {
            self.rows.iter().any(|row| row.left_position == position)
        })
    }

    /// Whether every right-roster member occurs in at least one row.
    #[must_use]
    pub fn right_completeness(&self) -> CompletenessStanding {
        completeness_over(self.right.count(), |position| {
            self.rows.iter().any(|row| row.right_position == position)
        })
    }

    /// Whether every pair in the left-by-right roster product occurs in at least one row.
    #[must_use]
    pub fn density_standing(&self) -> DensityStanding {
        let dense = (0..self.left.count()).all(|left_position| {
            (0..self.right.count())
                .all(|right_position| pair_is_present(&self.rows, left_position, right_position))
        });
        if dense {
            DensityStanding::Dense
        } else {
            DensityStanding::Sparse
        }
    }
}

impl<Member, Key, Payload, const MEMBERS: usize, const ROWS: usize>
    KeyedRosterRows<'_, Member, Key, Member, Key, Payload, MEMBERS, MEMBERS, ROWS>
{
    /// Whether both relation sides borrow the same roster instance.
    #[must_use]
    pub fn roster_relation_standing(&self) -> RosterRelationStanding {
        if core::ptr::eq(self.left, self.right) {
            RosterRelationStanding::Same
        } else {
            RosterRelationStanding::Cross
        }
    }

    /// Whether at least one row relates a member to itself.
    ///
    /// # Errors
    ///
    /// Returns a typed refusal when the relation sides borrow different roster instances.
    pub fn self_relation_standing(&self) -> Result<SelfRelationStanding, SameRosterRequired> {
        self.require_same_roster()?;
        if self
            .rows
            .iter()
            .any(|row| row.left_position == row.right_position)
        {
            Ok(SelfRelationStanding::Present)
        } else {
            Ok(SelfRelationStanding::Absent)
        }
    }

    /// Whether this same-roster directed relation contains a cycle.
    ///
    /// # Errors
    ///
    /// Returns a typed refusal when the relation sides borrow different roster instances.
    pub fn cycle_standing(&self) -> Result<CycleStanding, SameRosterRequired> {
        self.require_same_roster()?;
        if (0..self.left.count()).any(|root| self.path_returns_to(root)) {
            Ok(CycleStanding::Cyclic)
        } else {
            Ok(CycleStanding::Acyclic)
        }
    }

    fn require_same_roster(&self) -> Result<(), SameRosterRequired> {
        match self.roster_relation_standing() {
            RosterRelationStanding::Same => Ok(()),
            RosterRelationStanding::Cross => Err(SameRosterRequired),
        }
    }

    fn path_returns_to(&self, root: usize) -> bool {
        let mut discovered = vec![root];
        let mut cursor = 0_usize;
        while let Some(position) = discovered.get(cursor).copied() {
            if self.advance_cycle_search(position, root, &mut discovered) {
                return true;
            }
            cursor = cursor.saturating_add(1);
        }
        false
    }

    fn advance_cycle_search(
        &self,
        position: usize,
        root: usize,
        discovered: &mut Vec<usize>,
    ) -> bool {
        for destination in self.destinations_from(position) {
            if destination == root {
                return true;
            }
            retain_once(discovered, destination);
        }
        false
    }

    fn extend_reachability(&self, position: usize, discovered: &mut Vec<usize>) {
        for destination in self.destinations_from(position) {
            retain_once(discovered, destination);
        }
    }

    fn destinations_from(&self, position: usize) -> impl Iterator<Item = usize> + '_ {
        self.rows
            .iter()
            .filter(move |row| row.left_position == position)
            .map(|row| row.right_position)
    }
}

impl<Member, Key: Eq, Payload, const MEMBERS: usize, const ROWS: usize>
    KeyedRosterRows<'_, Member, Key, Member, Key, Payload, MEMBERS, MEMBERS, ROWS>
{
    /// Partitions one shared roster into members reachable and unreachable from a declared root.
    ///
    /// Both partitions follow roster order rather than traversal order.
    ///
    /// # Errors
    ///
    /// Returns a typed refusal when the relation sides borrow different roster instances or when the root is outside that roster.
    pub fn reachability_from(
        &self,
        root: Key,
    ) -> Result<Reachability<MEMBERS>, ReachabilityError<Key>> {
        self.require_same_roster()
            .map_err(ReachabilityError::DifferentRosters)?;
        let Some(root_position) = self.left.index_of(&root) else {
            return Err(ReachabilityError::RootOutsideRoster { root });
        };
        let discovered = self.discover_from(root_position);
        let reachable = self
            .left
            .positions_where(|position, _, _| discovered.contains(&position));
        let unreachable = self
            .left
            .positions_where(|position, _, _| !discovered.contains(&position));
        let reachable = NonEmpty::from_bounded(reachable)
            .map_err(|_| ReachabilityError::RootOutsideRoster { root })?;
        Ok(Reachability {
            reachable,
            unreachable,
        })
    }

    fn discover_from(&self, root: usize) -> Vec<usize> {
        let mut discovered = vec![root];
        let mut cursor = 0_usize;
        while let Some(position) = discovered.get(cursor).copied() {
            self.extend_reachability(position, &mut discovered);
            cursor = cursor.saturating_add(1);
        }
        discovered
    }
}

fn pair_is_present<Left, LeftKey, Right, RightKey, Payload, const N: usize>(
    rows: &Bounded<ReferencedRosterRow<'_, Left, LeftKey, Right, RightKey, Payload>, N>,
    left_position: usize,
    right_position: usize,
) -> bool {
    rows.iter()
        .any(|row| row.left_position == left_position && row.right_position == right_position)
}

fn retain_once(held: &mut Vec<usize>, position: usize) {
    if held.contains(&position) {
        return;
    }
    held.push(position);
}

fn completeness_over(
    positions: usize,
    mut contains: impl FnMut(usize) -> bool,
) -> CompletenessStanding {
    if (0..positions).all(&mut contains) {
        CompletenessStanding::Complete
    } else {
        CompletenessStanding::Partial
    }
}
