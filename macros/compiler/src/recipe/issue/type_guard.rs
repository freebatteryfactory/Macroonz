//! The one road that builds a recipe refusal and the readers it hands back.

use super::{RecipeError, RecipeIssue};
use crate::token::SpanHandle;

impl RecipeError {
    pub(in crate::recipe) const fn at(issue: RecipeIssue, at: Option<SpanHandle>) -> Self {
        Self { issue, at }
    }

    /// Reads the exact recipe issue.
    #[must_use]
    pub(in crate::recipe) const fn issue(&self) -> &RecipeIssue {
        &self.issue
    }

    /// Reads the captured producer span available for this issue.
    #[must_use]
    pub(in crate::recipe) const fn token(&self) -> Option<SpanHandle> {
        self.at
    }
}
