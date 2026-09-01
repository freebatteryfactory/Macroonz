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
        evidence_position(role)
            .and_then(|position| self.trees.get(position))
            .and_then(Option::as_ref)
    }
}

pub(crate) const fn evidence_position(role: RecipeRole) -> Option<usize> {
    match role {
        RecipeRole::Trials => Some(0),
        RecipeRole::Mutation => Some(1),
        RecipeRole::Benchmarks => Some(2),
        RecipeRole::Network => Some(3),
        RecipeRole::Concurrency => Some(4),
        RecipeRole::Companions
        | RecipeRole::Dispatch
        | RecipeRole::CompileContract
        | RecipeRole::Property
        | RecipeRole::Typestate => None,
    }
}
