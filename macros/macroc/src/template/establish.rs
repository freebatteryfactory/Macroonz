//! The three construction passes.
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
//! template, a ceiling, an application, or the refusal body itself is what must
//! stay unreachable.

use super::{
    AxisCeiling, DeclarationTemplate, META_BOUND_AXES, TemplateBinding, TemplateConstructionIssue,
    TemplateParameter,
};

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
        let supplied: Vec<&TemplateBinding> = bindings
            .iter()
            .filter(|binding| binding.parameter().parameter == declared.parameter)
            .collect();
        // Two independent questions about one declared hole, asked separately
        // because they co-establish. "How many bindings name this hole" and
        // "does a binding that names it disagree with its declared category"
        // are true or false of each other in every combination, so asking the
        // second only where the first answered exactly one would report the
        // doubling and swallow the disagreement — the one-defect-per-attempt
        // road this home exists to close.
        let arity_issue = match supplied.len() {
            0 => Some(TemplateConstructionIssue::MissingBinding {
                parameter: declared.parameter,
            }),
            1 => None,
            _ => Some(TemplateConstructionIssue::DuplicateBinding {
                parameter: declared.parameter,
            }),
        };
        // The disagreement is reported at the FIRST binding that disagrees, and
        // the category it carries is that binding's own. Naming a category off
        // some other binding would be a finding about a value the caller did
        // not write.
        let category_issue = supplied
            .iter()
            .find(|binding| binding.category() != declared.category)
            .map(
                |binding| TemplateConstructionIssue::DeclaredCategoryDisagreement {
                    parameter: declared.parameter,
                    declared: declared.category,
                    bound: binding.category(),
                },
            );
        issues.extend(arity_issue);
        issues.extend(category_issue);
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
