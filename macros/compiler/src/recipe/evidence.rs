//! The neutral capability a composition root fills with prepared evidence projections.

use super::{EVIDENCE_LIMIT, Recipe, RecipeRole};
use crate::diagnostic::Diagnostic;
use crate::request::Door;
use crate::token::{CapturedInput, GeneratedTree};

/// The already sealed output for each selected standard evidence projection.
pub(crate) struct PreparedEvidence {
    trees: [Option<GeneratedTree>; EVIDENCE_LIMIT],
}

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

/// The one crate-internal preparation capability the composition root supplies.
pub(crate) trait EvidenceCompiler {
    /// Prepare every selected standard evidence projection without giving the recipe home adapter vocabulary.
    fn prepared(
        capture: &CapturedInput,
        recipe: &Recipe,
        door: &Door,
        replaced: Option<RecipeRole>,
    ) -> Result<PreparedEvidence, Diagnostic>;
}

/// The sealed marker whose sole implementation lives at the crate composition root.
pub(crate) struct ConfiguredEvidence;

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
