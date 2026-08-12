//! The coverage pass, the admission answer, and the refusal an established
//! issue list amounts to.
//!
//! The kind's applicable roster is the quantifier. Every question the kind
//! admits is examined, in roster order, and then every supplied answer is
//! examined against the roster — so an unanswered seat, a doubled seat, and an
//! inadmissible answer are three findings reported together rather than a caller
//! repairing a view one question per attempt.
//!
//! Nothing here reaches a private field: the pass reads each explanation's
//! question through the same answer any caller gets. The road that consumes this
//! pass lives in `type_guard.rs`, because completing a view is what must stay
//! unreachable.

use super::{ExplanationCoverage, ExplanationCoverageIssue, ProjectionExplanation};
use crate::plane::AuthoringLimitProfile;
use crate::planning::{ProjectionKind, ProjectionPlan};
use crate::question::{ExplanationQuestion, QuestionApplicability};
use threadpak::refusal::{CompletionPosture, StopBound};
use threadpak::types::{NonEmptyBounded, NonEmptyBoundedConstruction, PositiveLimit};

/// Whether one kind admits one question.
#[must_use]
pub fn kind_admits<K: ProjectionKind>(question: ExplanationQuestion) -> QuestionApplicability {
    if ProjectionPlan::<K>::applicable_questions().contains(&question) {
        QuestionApplicability::Applicable
    } else {
        QuestionApplicability::NotApplicableToKind
    }
}

/// Every way one answer set fails to cover one kind's applicable roster: the
/// questions nobody answered, the questions answered twice, and the answers the
/// kind does not admit at all.
pub(super) fn coverage_issues<K: ProjectionKind>(
    answers: &[ProjectionExplanation],
) -> Vec<ExplanationCoverageIssue> {
    let applicable = ProjectionPlan::<K>::applicable_questions();
    let mut issues: Vec<ExplanationCoverageIssue> = Vec::new();
    for question in &applicable {
        let count = answers
            .iter()
            .filter(|answer| answer.question() == *question)
            .count();
        if count == 0 {
            issues.push(ExplanationCoverageIssue::QuestionUnanswered(*question));
        } else if count > 1 {
            issues.push(ExplanationCoverageIssue::QuestionAnsweredTwice(*question));
        }
    }
    for answer in answers {
        if !applicable.contains(&answer.question()) {
            issues.push(ExplanationCoverageIssue::QuestionNotApplicableToKind(
                answer.question(),
            ));
        }
    }
    issues
}

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
///
/// One road for every pass in
/// [`ProjectionExplanationView::complete`](super::ProjectionExplanationView::complete),
/// so no pass can establish issues and then walk on past them.
pub(super) fn refused(issues: Vec<ExplanationCoverageIssue>) -> Option<ExplanationCoverage> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(ExplanationCoverage::established(
        first,
        established.collect(),
    ))
}

impl ExplanationCoverage {
    /// The body a coverage check refuses with. When the issues outrun the
    /// declared bound the body keeps the first and reports that examination
    /// stopped there.
    pub(super) fn established(
        first: ExplanationCoverageIssue,
        rest: Vec<ExplanationCoverageIssue>,
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
