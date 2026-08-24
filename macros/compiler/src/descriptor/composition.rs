//! The duplicate scan one composition is declared through.
//!
//! The declared list is the quantifier: every provider is examined against the ones before and after it, so "every declared provider was examined" is a fact about the loop rather than a claim about it.
//! Only the FIRST occurrence of a doubled identity raises an issue, so a caller repairing one duplicate is not told about it twice.
//!
//! Nothing here reaches a private field. The road that consumes this pass builds the composition, and that road is in `type_guard.rs`, because building the value is what must stay unreachable.

use super::{CompositionIssue, Provider};

/// Every provider identity declared more than once, reported at its first occurrence.
pub(super) fn doubled_providers(declared: &[Provider]) -> Vec<CompositionIssue> {
    let mut issues: Vec<CompositionIssue> = Vec::new();
    for (position, provider) in declared.iter().enumerate() {
        let earlier = declared
            .iter()
            .take(position)
            .any(|other| other.identity == provider.identity);
        let repeated = declared
            .iter()
            .skip(position.saturating_add(1))
            .any(|other| other.identity == provider.identity);
        if repeated && !earlier {
            issues.push(CompositionIssue::ProviderDoubled {
                provider: provider.identity,
            });
        }
    }
    issues
}
