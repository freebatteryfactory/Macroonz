//! The duplicate scan, and the refusal an established issue list amounts to.
//!
//! The declared set is the quantifier. Every provider is examined against the
//! ones before and after it, so "every declared provider was examined" is a fact
//! about the loop rather than a claim about it, and only the FIRST occurrence of
//! a doubled identity raises the issue — a caller repairing one duplicate does
//! not get told about it twice.
//!
//! Nothing here reaches a private field: the scan reads a supplied provider list
//! before any root exists. The roads that consume this pass live in
//! `type_guard.rs`, because building a root and building the refusal body are
//! both what must stay unreachable.

use super::{CompositionRootIssue, DescriptorProvider};

/// Every provider identity declared more than once, reported at its first
/// occurrence.
pub(super) fn duplicate_issues(declared: &[DescriptorProvider]) -> Vec<CompositionRootIssue> {
    let mut issues: Vec<CompositionRootIssue> = Vec::new();
    for (position, provider) in declared.iter().enumerate() {
        let earlier = declared
            .iter()
            .take(position)
            .any(|other| other.provider == provider.provider);
        let repeated = declared
            .iter()
            .skip(position.saturating_add(1))
            .any(|other| other.provider == provider.provider);
        if repeated && !earlier {
            issues.push(CompositionRootIssue::DuplicateProvider {
                provider: provider.provider,
            });
        }
    }
    issues
}
