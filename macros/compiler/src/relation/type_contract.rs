//! The relation home's trait surface: how each refusal reads and preserves its concrete cause.

use super::{
    KeyedRosterRowsError, ReachabilityError, RepeatedRelationPairs, SameRosterRequired,
    StructuralMismatch,
};
use core::error::Error;
use core::fmt::{self, Display, Formatter};

impl<const N: usize> fmt::Debug for super::RepeatedRelationPair<N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RepeatedRelationPair")
            .field("left_position", &self.left_position())
            .field("right_position", &self.right_position())
            .field("first", &self.first_position())
            .field("repeated", self.repeated_positions())
            .finish()
    }
}

impl<LeftKey, RightKey, const N: usize> Display for KeyedRosterRowsError<LeftKey, RightKey, N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow(overflow) => Display::fmt(overflow, formatter),
            Self::ForeignLeft(foreign) if foreign.count() == 1 => {
                formatter.write_str("one relation row references a key outside the left roster")
            }
            Self::ForeignLeft(foreign) => write!(
                formatter,
                "{} relation rows reference keys outside the left roster",
                foreign.count()
            ),
            Self::ForeignRight(foreign) if foreign.count() == 1 => {
                formatter.write_str("one relation row references a key outside the right roster")
            }
            Self::ForeignRight(foreign) => write!(
                formatter,
                "{} relation rows reference keys outside the right roster",
                foreign.count()
            ),
        }
    }
}

impl<LeftKey: fmt::Debug, RightKey: fmt::Debug, const N: usize> Error
    for KeyedRosterRowsError<LeftKey, RightKey, N>
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Overflow(overflow) => Some(overflow),
            Self::ForeignLeft(_) | Self::ForeignRight(_) => None,
        }
    }
}

impl<const N: usize> Display for RepeatedRelationPairs<N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if self.count() == 1 {
            formatter.write_str("one relation endpoint pair occurs more than once")
        } else {
            write!(
                formatter,
                "{} relation endpoint pairs occur more than once",
                self.count()
            )
        }
    }
}

impl<const N: usize> Error for RepeatedRelationPairs<N> {}

impl<Answer> Display for StructuralMismatch<Answer> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .write_str("the computed structural answer differs from the caller-required answer")
    }
}

impl<Answer: fmt::Debug> Error for StructuralMismatch<Answer> {}

impl Display for SameRosterRequired {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            "the structural question requires both relation sides to borrow one roster instance",
        )
    }
}

impl Error for SameRosterRequired {}

impl<Key> Display for ReachabilityError<Key> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DifferentRosters(cause) => Display::fmt(cause, formatter),
            Self::RootOutsideRoster { .. } => {
                formatter.write_str("the reachability root is outside the shared roster")
            }
        }
    }
}

impl<Key: fmt::Debug> Error for ReachabilityError<Key> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DifferentRosters(cause) => Some(cause),
            Self::RootOutsideRoster { .. } => None,
        }
    }
}
