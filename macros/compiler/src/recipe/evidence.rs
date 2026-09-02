//! The neutral capability a composition root fills with prepared evidence projections.

use super::{EVIDENCE_LIMIT, PreparedEvidence, RecipeRole};
use crate::token::GeneratedTree;

impl PreparedEvidence {
    /// Build the neutral prepared-output account from the composition root's complete role array.
    pub(crate) const fn assembled(trees: [Option<GeneratedTree>; EVIDENCE_LIMIT]) -> Self {
        Self { trees }
    }

    /// Reads the sealed output for one descriptor-native role.
    pub(crate) fn tree(&self, role: RecipeRole) -> Option<&GeneratedTree> {
        role.profile()
            .evidence_position
            .and_then(|position| self.trees.get(position))
            .and_then(Option::as_ref)
    }
}

pub(crate) fn evidence_position(role: RecipeRole) -> Option<usize> {
    role.profile().evidence_position
}
