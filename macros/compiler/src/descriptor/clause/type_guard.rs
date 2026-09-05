//! Construction and read-only access for informed descriptor clauses.

use super::{Assignment, Clause};
use crate::token::{CapturedTokenTree, SpanHandle};

impl<'trees> Assignment<'trees> {
    /// Admit one assignment after its key, separator, value, and grammar roster have been read.
    pub(crate) fn admitted(
        key: &'trees str,
        value: Vec<&'trees CapturedTokenTree>,
        at: SpanHandle,
    ) -> Self {
        Self { key, value, at }
    }

    /// The admitted key.
    pub(crate) const fn key(&self) -> &'trees str {
        self.key
    }

    /// The exact captured value tokens.
    pub(crate) fn value(&self) -> &[&'trees CapturedTokenTree] {
        &self.value
    }

    /// The authored key's capture coordinate.
    pub(crate) const fn at(&self) -> SpanHandle {
        self.at
    }
}

impl<'trees, Nested> Clause<'trees, Nested> {
    /// Admit one mechanically read assignment.
    pub(crate) const fn assigned(assignment: Assignment<'trees>) -> Self {
        Self::Assigned(assignment)
    }

    /// Carry one grammar-owned nested clause.
    pub(crate) const fn nested(nested: Nested) -> Self {
        Self::Nested(nested)
    }

    /// Read the assignment where this clause is one.
    pub(crate) const fn assignment(&self) -> Option<&Assignment<'trees>> {
        match self {
            Self::Assigned(assignment) => Some(assignment),
            Self::Nested(_) => None,
        }
    }

    /// Read the grammar-owned nested material where this clause carries one.
    pub(crate) const fn nested_value(&self) -> Option<&Nested> {
        match self {
            Self::Assigned(_) => None,
            Self::Nested(nested) => Some(nested),
        }
    }
}
