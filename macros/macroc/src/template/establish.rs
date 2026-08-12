//! The three construction passes, and the refusal an established issue list
//! amounts to.
//!
//! Each pass has a roster for its quantifier — the declared holes, the meta
//! bound axes, the template's own parameters — so "every one was examined" is a
//! fact about a loop rather than a claim about it. A hole doubled, an axis
//! unbounded, a binding naming a hole nobody declared: each is its own finding,
//! and all of them are reported together, because a caller repairing a template
//! one hole per attempt is a caller this home failed.
//!
//! Nothing here reaches a private field: every pass reads supplied material, or
//! reads a template through the same public answers any caller gets. The roads
//! that consume these passes live in `type_guard.rs`, because building a
//! template, a ceiling, or an application is what must stay unreachable.

use super::{
    AxisCeiling, DeclarationTemplate, META_BOUND_AXES, TemplateBinding, TemplateConstruction,
    TemplateConstructionIssue, TemplateParameter,
};
use crate::plane::AuthoringLimitProfile;
use threadpak::refusal::{CompletionPosture, StopBound};
use threadpak::types::{NonEmptyBounded, NonEmptyBoundedConstruction, PositiveLimit};

/// Every parameter identity a hole set declares more than once, reported at its
/// first occurrence.
pub(super) fn parameter_issues(declared: &[TemplateParameter]) -> Vec<TemplateConstructionIssue> {
    let mut issues: Vec<TemplateConstructionIssue> = Vec::new();
    for (position, parameter) in declared.iter().enumerate() {
        let earlier = declared
            .iter()
            .take(position)
            .any(|other| other.parameter == parameter.parameter);
        let repeated = declared
            .iter()
            .skip(position.saturating_add(1))
            .any(|other| other.parameter == parameter.parameter);
        if repeated && !earlier {
            issues.push(TemplateConstructionIssue::DuplicateParameter {
                parameter: parameter.parameter,
            });
        }
    }
    issues
}

/// Every axis the supplied ceilings leave unbounded or bound twice, in roster
/// order.
pub(super) fn ceiling_issues(axes: &[AxisCeiling]) -> Vec<TemplateConstructionIssue> {
    let mut issues: Vec<TemplateConstructionIssue> = Vec::new();
    for axis in META_BOUND_AXES {
        let stated = axes.iter().filter(|held| held.axis == axis).count();
        if stated == 0 {
            issues.push(TemplateConstructionIssue::CeilingAxisAbsent { axis });
        } else if stated > 1 {
            issues.push(TemplateConstructionIssue::CeilingAxisDoubled { axis });
        }
    }
    issues
}

/// Every way one binding set fails to fill one template's holes: a hole left
/// unbound, a hole bound twice, a bound hole under the wrong category, and a
/// binding naming a hole the template does not declare.
pub(super) fn binding_issues(
    template: &DeclarationTemplate,
    bindings: &[TemplateBinding],
) -> Vec<TemplateConstructionIssue> {
    let mut issues: Vec<TemplateConstructionIssue> = Vec::new();
    for declared in template.parameters() {
        let mut supplied = bindings
            .iter()
            .filter(|binding| binding.parameter().parameter == declared.parameter);
        // The dispatch answers one question — what issue, if any, does this
        // declared parameter establish — and every arm answers it. An arm
        // that pushes a side effect at its own depth says nothing about the
        // arms beside it; an arm that yields the answer is comparable with
        // them, and the exhaustive shape is what proves nothing was missed.
        let established_issue = match (supplied.next(), supplied.next()) {
            (None, _) => Some(TemplateConstructionIssue::MissingBinding {
                parameter: declared.parameter,
            }),
            (Some(bound), None) if bound.category() != declared.category => {
                Some(TemplateConstructionIssue::DeclaredCategoryDisagreement {
                    parameter: declared.parameter,
                    declared: declared.category,
                    bound: bound.category(),
                })
            }
            (Some(_), None) => None,
            (Some(_), Some(_)) => Some(TemplateConstructionIssue::DuplicateBinding {
                parameter: declared.parameter,
            }),
        };
        issues.extend(established_issue);
    }
    for binding in bindings {
        let known = template
            .parameters()
            .any(|declared| declared.parameter == binding.parameter().parameter);
        if !known {
            issues.push(TemplateConstructionIssue::UnknownParameter {
                parameter: binding.parameter().parameter,
            });
        }
    }
    issues
}

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
pub(super) fn refused(issues: Vec<TemplateConstructionIssue>) -> Option<TemplateConstruction> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(TemplateConstruction::co_established(
        first,
        established.collect(),
    ))
}

impl TemplateConstruction {
    /// The one-issue body. Total: the declared bound admits an item by
    /// compile-time proof, so refusing never needs an error road of its own.
    pub fn established(issue: TemplateConstructionIssue) -> Self {
        Self {
            issues: NonEmptyBounded::singleton(issue),
            posture: CompletionPosture::Complete,
        }
    }

    /// The several-issue body. When the supplied issues outrun the declared
    /// bound the body keeps the first and reports that enumeration stopped
    /// there — never a silent drop, never an unearned claim of completeness.
    pub fn co_established(
        first: TemplateConstructionIssue,
        rest: Vec<TemplateConstructionIssue>,
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
