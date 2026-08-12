//! The duplicate scan, and the refusal an established issue list amounts to.
//!
//! The declared set is the quantifier. Every provider is examined against the
//! ones before and after it, so "every declared provider was examined" is a fact
//! about the loop rather than a claim about it, and only the FIRST occurrence of
//! a doubled identity raises the issue — a caller repairing one duplicate does
//! not get told about it twice.
//!
//! Nothing here reaches a private field: the scan reads a supplied provider list
//! before any root exists. The road that consumes this pass lives in
//! `type_guard.rs`, because building a root is what must stay unreachable.

use super::{CompositionRootDeclaration, CompositionRootIssue, DescriptorProvider};
use crate::plane::AuthoringLimitProfile;
use threadpak::refusal::{CompletionPosture, StopBound};
use threadpak::types::{NonEmptyBounded, NonEmptyBoundedConstruction, PositiveLimit};

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

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
pub(super) fn refused(issues: Vec<CompositionRootIssue>) -> Option<CompositionRootDeclaration> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(CompositionRootDeclaration::established(
        first,
        established.collect(),
    ))
}

impl CompositionRootDeclaration {
    /// The body a declaration check refuses with. When the issues outrun the
    /// declared bound the body keeps the first and reports that examination
    /// stopped there.
    pub(super) fn established(
        first: CompositionRootIssue,
        rest: Vec<CompositionRootIssue>,
    ) -> Self {
        match NonEmptyBounded::admitted_const(
            first,
            rest,
            &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
        ) {
            Ok(issues) => Self {
                issues,
                posture: CompletionPosture::Complete,
            },
            Err(NonEmptyBoundedConstruction::OverLimit) => Self {
                issues: NonEmptyBounded::singleton(first),
                posture: CompletionPosture::EarlyStopped {
                    stopped_at: StopBound::DeclaredIssueBound,
                },
            },
        }
    }
}
