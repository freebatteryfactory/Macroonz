//! The explanation-protocol home's invariant nucleus: every road that reaches a
//! private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes the
//! protocol's two central claims structural. An explanation's question is taken
//! HERE, from the answer itself, so a true answer filed under the wrong question
//! is a value nobody can build. A view is completed HERE, after the coverage
//! pass agreed, so there is no partial view for a reader to mistake for a
//! complete one.

use super::super::establish::{coverage_issues, refused};
use super::{
    ExplanationAnswer, ExplanationCoverage, ExplanationCoverageIssue, ProjectionExplanation,
    ProjectionExplanationView,
};
use crate::plane::{ExplanationSeatLimit, HumanProjection, HumanTextLimit};
use crate::planning::ProjectionKind;
use crate::question::ExplanationQuestion;
use core::marker::PhantomData;
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit};

impl ProjectionExplanation {
    /// Answer one question. The question is taken from the answer, never from
    /// the caller, so no explanation can file a true answer under the wrong
    /// question.
    #[must_use]
    pub fn answered(answer: ExplanationAnswer, human: HumanProjection<HumanTextLimit>) -> Self {
        Self {
            question: answer.question(),
            answer,
            human,
        }
    }

    /// The question answered.
    #[must_use]
    pub const fn question(&self) -> ExplanationQuestion {
        self.question
    }

    /// The typed answer.
    #[must_use]
    pub const fn answer(&self) -> &ExplanationAnswer {
        &self.answer
    }

    /// The rendering for a person. Derived from the answer; never read back.
    #[must_use]
    pub const fn human(&self) -> &HumanProjection<HumanTextLimit> {
        &self.human
    }
}

impl<K: ProjectionKind> ProjectionExplanationView<K> {
    /// Complete the view over the kind's applicable questions.
    ///
    /// # Errors
    ///
    /// Returns [`ExplanationCoverage`] naming every unanswered question, every
    /// doubled question, and every question the kind does not admit. All three
    /// are reported together: a caller repairing a view one question per
    /// attempt is a caller the protocol failed.
    pub fn complete(answers: Vec<ProjectionExplanation>) -> Result<Self, ExplanationCoverage> {
        if let Some(refusal) = refused(coverage_issues::<K>(&answers)) {
            return Err(refusal);
        }
        let observed = answers.len();
        Bounded::admitted_const(answers, &AdmittedLimit::under_ceiling())
            .map(|answers| Self {
                answers,
                _kind: PhantomData,
            })
            .map_err(|_| {
                ExplanationCoverage::established(
                    ExplanationCoverageIssue::SeatBoundExceeded {
                        bound: u64::try_from(ExplanationSeatLimit::MAX).unwrap_or(u64::MAX),
                        observed: u64::try_from(observed).unwrap_or(u64::MAX),
                    },
                    Vec::new(),
                )
            })
    }

    /// The number of seats filled.
    #[must_use]
    pub fn len(&self) -> usize {
        self.answers.len()
    }

    /// Whether the view holds no answer. Always `false` in practice: a kind's
    /// applicable roster is never empty, so a complete view always has seats.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.answers.is_empty()
    }
}
