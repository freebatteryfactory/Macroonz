//! The explanation-protocol home's invariant nucleus: every road that reaches a
//! private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes the
//! protocol's central claims structural. An explanation's question and its human
//! rendering are both taken HERE, from the answer itself, so a true answer filed
//! under the wrong question and a sentence that contradicts its answer are
//! values nobody can build. A view is completed HERE, after the coverage pass
//! agreed, so there is no partial view for a reader to mistake for a complete
//! one.

use super::super::establish::{coverage_issues, refused};
use super::super::project::human_line;
use super::{
    ExplanationAnswer, ExplanationCoverage, ExplanationCoverageIssue, ProjectionExplanation,
    ProjectionExplanationView,
};
use crate::plane::{AuthoringLimitProfile, ExplanationSeatLimit, HumanProjection, HumanTextLimit};
use crate::planning::ProjectionKind;
use crate::question::ExplanationQuestion;
use core::marker::PhantomData;
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit};

impl ProjectionExplanation {
    /// Answer one question.
    ///
    /// The answer is the whole input, and that is the claim. The question is
    /// taken from the answer, so no explanation can file a true answer under the
    /// wrong question; the rendering is taken from the answer too, so no
    /// explanation can carry a sentence its typed content does not support. Both
    /// used to be seats a caller could fill independently — the question was
    /// closed first, and this road closes the second.
    #[must_use]
    pub fn answered(answer: ExplanationAnswer) -> Self {
        Self {
            question: answer.question(),
            answer,
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

    /// The rendering for a person, composed from the answer at the moment it is
    /// asked for. Never stored, never read back, and never supplied.
    #[must_use]
    pub fn human(&self) -> HumanProjection<HumanTextLimit> {
        human_line(&self.answer)
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
        Bounded::admitted_const(
            answers,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
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
