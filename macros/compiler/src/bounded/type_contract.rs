//! The bounded home's trait surface: how each refusal reads and preserves its concrete cause.
//!
//! A construction refusal here is an ordinary error and nothing more.

use super::{
    Empty, KeyedRosterAssignmentError, KeyedRosterError, KeyedRosterRowsError, NonEmptyError,
    Overflow, RepeatedRelationPairs,
};
use core::error::Error;
use core::fmt::{self, Display, Formatter};

impl Display for Overflow {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} items offered where at most {} fit",
            self.offered, self.capacity
        )
    }
}

impl Error for Overflow {}

impl Display for Empty {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("no item offered where at least one is required")
    }
}

impl Error for Empty {}

impl Display for NonEmptyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty(empty) => Display::fmt(empty, formatter),
            Self::Overflow(overflow) => Display::fmt(overflow, formatter),
        }
    }
}

impl Error for NonEmptyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Empty(empty) => Some(empty),
            Self::Overflow(overflow) => Some(overflow),
        }
    }
}

impl From<Empty> for NonEmptyError {
    fn from(empty: Empty) -> Self {
        Self::Empty(empty)
    }
}

impl From<Overflow> for NonEmptyError {
    fn from(overflow: Overflow) -> Self {
        Self::Overflow(overflow)
    }
}

impl<K, const N: usize> Display for KeyedRosterError<K, N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty(empty) => Display::fmt(empty, formatter),
            Self::Overflow(overflow) => Display::fmt(overflow, formatter),
            Self::DuplicateKeys(duplicates) if duplicates.count() == 1 => {
                formatter.write_str("one caller-declared key occurred more than once")
            }
            Self::DuplicateKeys(duplicates) => write!(
                formatter,
                "{} caller-declared keys occurred more than once",
                duplicates.count()
            ),
        }
    }
}

impl<K: fmt::Debug, const N: usize> Error for KeyedRosterError<K, N> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Empty(empty) => Some(empty),
            Self::Overflow(overflow) => Some(overflow),
            Self::DuplicateKeys(_) => None,
        }
    }
}

impl<K, S, const N: usize> Display for KeyedRosterAssignmentError<K, S, N> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty(empty) => Display::fmt(empty, formatter),
            Self::Overflow(overflow) => Display::fmt(overflow, formatter),
            Self::ForeignReferences(foreign) if foreign.count() == 1 => {
                formatter.write_str("one offered payload references a key outside the denominator")
            }
            Self::ForeignReferences(foreign) => write!(
                formatter,
                "{} offered payloads reference keys outside the denominator",
                foreign.count()
            ),
            Self::DuplicateReferences(duplicates) if duplicates.count() == 1 => formatter
                .write_str("one denominator key is referenced by more than one offered payload"),
            Self::DuplicateReferences(duplicates) => write!(
                formatter,
                "{} denominator keys are referenced by more than one offered payload",
                duplicates.count()
            ),
            Self::ReusedPayloadSeats(duplicates) if duplicates.count() == 1 => {
                formatter.write_str("one caller-declared payload-seat key is used more than once")
            }
            Self::ReusedPayloadSeats(duplicates) => write!(
                formatter,
                "{} caller-declared payload-seat keys are used more than once",
                duplicates.count()
            ),
            Self::MissingMembers(missing) if missing.count() == 1 => {
                formatter.write_str("one denominator member has no offered payload")
            }
            Self::MissingMembers(missing) => write!(
                formatter,
                "{} denominator members have no offered payload",
                missing.count()
            ),
        }
    }
}

impl<K: fmt::Debug, S: fmt::Debug, const N: usize> Error for KeyedRosterAssignmentError<K, S, N> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Empty(empty) => Some(empty),
            Self::Overflow(overflow) => Some(overflow),
            Self::ForeignReferences(_)
            | Self::DuplicateReferences(_)
            | Self::ReusedPayloadSeats(_)
            | Self::MissingMembers(_) => None,
        }
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
