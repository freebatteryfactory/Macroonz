//! The bounded home's trait surface: how each refusal reads and preserves its concrete cause.
//!
//! A construction refusal here is an ordinary error and nothing more.

use super::{Empty, KeyedRosterError, NonEmptyError, Overflow};
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
