//! The relation home's rows, postures, standings, and refusals.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs` or `questions.rs`, this file's own children.

use crate::bounded::{Bounded, ForeignRosterReference, KeyedRoster, NonEmpty};

#[path = "type_guard.rs"]
mod guard;

#[path = "questions.rs"]
mod questions;

/// Foreign-free rows referencing one left and one right caller-keyed roster.
///
/// Rows retain authored order and may be empty or repeated until a caller-selected posture informs those questions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyedRosterRows<
    'rosters,
    Left,
    LeftKey,
    Right,
    RightKey,
    Payload,
    const LEFT: usize,
    const RIGHT: usize,
    const ROWS: usize,
> {
    left: &'rosters KeyedRoster<Left, LeftKey, LEFT>,
    right: &'rosters KeyedRoster<Right, RightKey, RIGHT>,
    rows: Bounded<ReferencedRosterRow<'rosters, Left, LeftKey, Right, RightKey, Payload>, ROWS>,
    canonical_indices: Bounded<usize, ROWS>,
}

/// One duplicate-free relation promoted from foreign-free keyed-roster rows.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyedRosterRelation<
    'rosters,
    Left,
    LeftKey,
    Right,
    RightKey,
    Payload,
    const LEFT: usize,
    const RIGHT: usize,
    const ROWS: usize,
> {
    rows: KeyedRosterRows<'rosters, Left, LeftKey, Right, RightKey, Payload, LEFT, RIGHT, ROWS>,
}

crate::roster! {
    /// Which stable relation-row reading a caller selects.
    pub enum RowOrder {
        /// The order in which the caller declared the rows.
        Authored = "authored",
        /// Left-roster position, then right-roster position, with authored order breaking equal-pair ties.
        Canonical = "canonical",
    }
}

crate::roster! {
    /// Which structural question one recipe relation may answer.
    pub(crate) enum RelationQuestion {
        /// Whether the relation has any row.
        Empty = "empty",
        /// Whether an endpoint pair occurs more than once.
        Repetition = "repetition",
        /// Whether each endpoint roster is open or closed.
        Membership = "membership",
        /// Whether each endpoint roster is completely covered.
        Completeness = "completeness",
        /// Whether every possible endpoint pair occurs.
        Density = "density",
        /// What a generated operation does when no row applies.
        Absence = "absence",
        /// Whether a same-roster row may relate one member to itself.
        SelfRelation = "self_relation",
        /// Whether a same-roster directed cycle remains lawful.
        Cycle = "cycle",
    }
}

crate::roster! {
    /// Whether repeated endpoint pairs remain lawful.
    pub enum RepetitionPosture {
        /// Repeated pairs are retained as caller-owned rows.
        Allowed = "allowed",
        /// Every endpoint pair must occur once.
        Refusal = "refused",
    }
}

crate::roster! {
    /// Whether a relation with no row remains lawful.
    pub enum EmptyPosture {
        /// An empty relation remains lawful.
        Allowed = "allowed",
        /// At least one relation row is required.
        Refusal = "refused",
    }
}

crate::roster! {
    /// Whether a declared roster is the complete membership vocabulary.
    pub enum MembershipPosture {
        /// Values outside the declared roster may exist, while every enumerated operation remains bounded by the stated roster.
        Open = "open",
        /// The declared roster is the complete membership vocabulary.
        Closed = "closed",
    }
}

crate::roster! {
    /// Whether incomplete structural coverage remains lawful.
    pub enum CompletenessPosture {
        /// Partial coverage remains lawful.
        Partial = "partial",
        /// Every member under the selected question must be covered.
        Total = "total",
    }
}

crate::roster! {
    /// Whether a relation may omit endpoint pairs from the full cross product.
    pub enum DensityPosture {
        /// Omitted endpoint pairs remain lawful.
        Sparse = "sparse",
        /// Every left and right endpoint pair must occur.
        Dense = "dense",
    }
}

crate::roster! {
    /// What a generated operation does where no relation row applies.
    pub enum AbsencePosture {
        /// The caller supplies or accepts an ordinary absent case.
        Allowed = "allowed",
        /// An absent case is a typed refusal.
        Refusal = "refused",
    }
}

crate::roster! {
    /// Whether a member may relate to itself in a same-roster relation.
    pub enum SelfRelationPosture {
        /// Self relations remain lawful.
        Allowed = "allowed",
        /// Every self relation is refused.
        Refusal = "refused",
    }
}

crate::roster! {
    /// Whether a same-roster directed cycle remains lawful.
    pub enum CyclePosture {
        /// Directed cycles remain lawful.
        Allowed = "allowed",
        /// The relation must be acyclic.
        Refusal = "refused",
    }
}

crate::roster! {
    /// Whether the relation holds no row or at least one.
    pub enum OccupancyStanding {
        /// The relation holds no row.
        Empty = "empty",
        /// The relation holds at least one row.
        Populated = "populated",
    }
}

crate::roster! {
    /// Whether every endpoint pair occurs once or at least one pair repeats.
    pub enum RepetitionStanding {
        /// Every endpoint pair occurs once.
        Distinct = "distinct",
        /// At least one endpoint pair occurs more than once.
        Repeated = "repeated",
    }
}

crate::roster! {
    /// Whether every member under one structural question is covered.
    pub enum CompletenessStanding {
        /// At least one member is not covered.
        Partial = "partial",
        /// Every member is covered.
        Complete = "complete",
    }
}

crate::roster! {
    /// Whether every left and right endpoint pair occurs.
    pub enum DensityStanding {
        /// At least one endpoint pair is absent.
        Sparse = "sparse",
        /// Every endpoint pair occurs.
        Dense = "dense",
    }
}

crate::roster! {
    /// Whether two relation sides borrow the same roster instance.
    pub enum RosterRelationStanding {
        /// Both sides borrow the same roster instance.
        Same = "same",
        /// The sides borrow different roster instances.
        Cross = "cross",
    }
}

crate::roster! {
    /// Whether at least one same-roster row relates a member to itself.
    pub enum SelfRelationStanding {
        /// No row relates a member to itself.
        Absent = "absent",
        /// At least one row relates a member to itself.
        Present = "present",
    }
}

crate::roster! {
    /// Whether one same-roster directed relation contains a cycle.
    pub enum CycleStanding {
        /// No directed cycle exists.
        Acyclic = "acyclic",
        /// At least one directed cycle exists.
        Cyclic = "cyclic",
    }
}

/// One caller-declared answer required of one independently computed structural answer.
#[must_use = "a structural requirement has not been settled against an observed answer"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructuralRequirement<Answer> {
    required: Answer,
}

/// One disagreement between a caller-required answer and the answer a structural question computed.
#[must_use = "a structural mismatch names the required and observed answers"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructuralMismatch<Answer> {
    required: Answer,
    observed: Answer,
}

/// Which members one same-roster root can and cannot reach.
#[must_use = "a reachability reading retains the complete reachable and unreachable position partition"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reachability<const N: usize> {
    reachable: NonEmpty<usize, N>,
    unreachable: Bounded<usize, N>,
}

/// A same-roster structural question was asked of two distinct roster instances.
#[must_use = "a same-roster refusal means the question has no lawful subject"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SameRosterRequired;

/// How a same-roster reachability question refuses before traversal.
#[must_use = "a reachability refusal names whether its roster or root was unavailable"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReachabilityError<Key> {
    /// The two relation sides borrow different roster instances.
    DifferentRosters(SameRosterRequired),
    /// The declared root key is outside the shared roster.
    RootOutsideRoster {
        /// The caller-declared root key.
        root: Key,
    },
}

/// One internally resolved relation row.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ReferencedRosterRow<'rosters, Left, LeftKey, Right, RightKey, Payload> {
    left_position: usize,
    left_key: &'rosters LeftKey,
    left_member: &'rosters Left,
    right_position: usize,
    right_key: &'rosters RightKey,
    right_member: &'rosters Right,
    payload: Payload,
}

/// One repeated relation pair and every authored position at which it occurred.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepeatedRelationPair<const N: usize> {
    left_position: usize,
    right_position: usize,
    first: usize,
    repeated: NonEmpty<usize, N>,
}

/// Every distinct relation pair that occurred more than once.
#[must_use = "a repeated-pair refusal carries every duplicated relation coordinate"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepeatedRelationPairs<const N: usize> {
    pairs: NonEmpty<RepeatedRelationPair<N>, N>,
}

/// How offered rows refuse reference-safe admission over two keyed rosters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyedRosterRowsError<LeftKey, RightKey, const N: usize> {
    /// More rows were offered than the declared row ceiling admits.
    Overflow(crate::bounded::Overflow),
    /// One or more rows name a key outside the left roster.
    ForeignLeft(NonEmpty<ForeignRosterReference<LeftKey>, N>),
    /// One or more rows name a key outside the right roster.
    ForeignRight(NonEmpty<ForeignRosterReference<RightKey>, N>),
}

struct ResolvedRosterMember<'roster, Member, Key> {
    position: usize,
    key: &'roster Key,
    member: &'roster Member,
}

struct CanonicalRelationPosition {
    authored: usize,
    left: usize,
    right: usize,
}

enum RowResolutionError<Key, const N: usize> {
    Overflow(crate::bounded::Overflow),
    Foreign(NonEmpty<ForeignRosterReference<Key>, N>),
}
