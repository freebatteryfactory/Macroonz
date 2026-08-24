//! The coverage pass: every way one set of answers fails to cover the questions a kind owes.
//!
//! Two rosters are the quantifiers — the compiler's universal one and the kind's own — and each is walked in its declared order before every supplied answer is walked against it.
//! So an unanswered seat, a doubled seat, an answer standing outside its roster, and a roster too wide to seat are findings reported together rather than one per attempt.
//!
//! Nothing here reaches a private field: the pass reads each answer's question through the same road any caller has.
//! The road that consumes the pass lives in `type_guard.rs`, because completing a view is what must stay unreachable.

use super::{DECLARED_QUESTION_LIMIT, ExplanationIssue, UniversalAnswer, UniversalQuestion};
use crate::kind::{Answer, Kind, Question};

/// How many of the supplied answers answer one question.
fn answered<Q: Question>(answers: &[Q::Answer], question: Q) -> usize {
    answers
        .iter()
        .filter(|answer| answer.question() == question)
        .count()
}

/// Every way one answer set fails to cover the universal roster.
fn universal_coverage(answers: &[UniversalAnswer], issues: &mut Vec<ExplanationIssue>) {
    for question in <UniversalQuestion as Question>::ALL {
        let count = answered::<UniversalQuestion>(answers, *question);
        if count == 0 {
            issues.push(ExplanationIssue::UniversalUnanswered {
                question: *question,
            });
        } else if count > 1 {
            issues.push(ExplanationIssue::UniversalAnsweredTwice {
                question: *question,
            });
        }
    }
}

/// Every way one answer set fails to cover the kind's own roster, and the one way that roster is too wide to seat at all.
///
/// The width is examined first, because a roster past the declared bound is a refusal about the KIND rather than about the answers, and a caller reading only unanswered seats would repair the wrong thing.
fn declared_coverage<K: Kind>(
    answers: &[<K::Question as Question>::Answer],
    issues: &mut Vec<ExplanationIssue>,
) {
    let roster = <K::Question as Question>::ALL;
    if roster.len() > DECLARED_QUESTION_LIMIT {
        issues.push(ExplanationIssue::SeatBoundExceeded {
            bound: u64::try_from(DECLARED_QUESTION_LIMIT).unwrap_or(u64::MAX),
            observed: u64::try_from(roster.len()).unwrap_or(u64::MAX),
        });
    }
    for question in roster {
        let count = answered::<K::Question>(answers, *question);
        if count == 0 {
            issues.push(ExplanationIssue::DeclaredUnanswered {
                question: question.name(),
                slot: question.slot(),
            });
        } else if count > 1 {
            issues.push(ExplanationIssue::DeclaredAnsweredTwice {
                question: question.name(),
                slot: question.slot(),
            });
        }
    }
    for answer in answers {
        let question = answer.question();
        if !roster.contains(&question) {
            issues.push(ExplanationIssue::QuestionOutsideRoster {
                question: question.name(),
            });
        }
    }
}

/// Every way one set of answers fails to cover the questions one kind owes.
///
/// The universal findings ride ahead of the kind's, so the first issue a refusal states in full is about the roster every kind shares wherever both are uncovered.
pub(super) fn coverage_issues<K: Kind>(
    universal: &[UniversalAnswer],
    declared: &[<K::Question as Question>::Answer],
) -> Vec<ExplanationIssue> {
    let mut issues: Vec<ExplanationIssue> = Vec::new();
    universal_coverage(universal, &mut issues);
    declared_coverage::<K>(declared, &mut issues);
    issues
}
