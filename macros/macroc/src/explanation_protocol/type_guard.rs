//! The explanation-protocol home's invariant nucleus: every road that reaches a
//! private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes the
//! protocol's central claims structural.
//! An explanation's question and its human rendering are both taken here, from
//! the answer itself, so a true answer filed under the wrong question and a
//! sentence that contradicts its answer are values nobody can build.
//! A view is completed here, after the coverage pass agreed, so there is no
//! partial view for a reader to mistake for a complete one.
//! The refusal body is built here by the same permission: its seat is private,
//! so this file is the only module in the workspace that can spell the literal,
//! and every refusal that exists came off the coverage pass.
//!
//! Rust's privacy is module-scoped, so a seat declared in `types.rs` would put
//! every other item in that file inside the wall.
//! The body is therefore declared in the `seat` module below, whose entire
//! content is that record and inherent implementations of it — the module is
//! the complete set of roads that can reach the private seat.
//!
//! # Nonclaims
//!
//! A private seat excludes every sibling — the rest of this file, `types.rs`
//! above it, `establish.rs` beside it, anywhere else in the services, and any
//! crate downstream — and the compiler says so with `E0451`.
//! It does not exclude descendants: a module declared inside the seat would
//! construct as freely as these roads do, so the reversal for this seat is a
//! compile-fail fixture testpak owns.

use super::super::establish::coverage_issues;
use super::super::project::human_line;
use super::{
    ExplanationAnswer, ExplanationCoverageIssue, ProjectionExplanation, ProjectionExplanationView,
};
use crate::plane::{AuthoringLimitProfile, ExplanationSeatLimit, HumanProjection, HumanTextLimit};
use crate::planning::ProjectionKind;
use crate::question::ExplanationQuestion;
use core::marker::PhantomData;
use threadpak::types::{AdmittedLimit, Bounded, ConstLimit};

/// The refusal one established issue list amounts to, or nothing where the list
/// is empty.
///
/// One road for every pass in
/// [`ProjectionExplanationView::complete`](super::ProjectionExplanationView::complete),
/// so no pass can establish issues and then walk on past them.
fn refused(issues: Vec<ExplanationCoverageIssue>) -> Option<ExplanationCoverage> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(ExplanationCoverage::established(
        first,
        established.collect(),
    ))
}

pub use seat::ExplanationCoverage;

mod seat {
    use super::super::ExplanationCoverageIssue;
    use crate::plane::{AuthoringLimitProfile, ExplanationIssueLimit};
    use threadpak::refusal::{AdmittedPrefix, StopBound};
    use threadpak::types::PositiveLimit;

    /// The explanation-coverage refusal family body.
    ///
    /// Independent members: several questions may be unanswered while another is
    /// doubled, and reporting one of them would leave a caller repairing the
    /// view one question per attempt.
    #[must_use = "a refusal family body carries every uncovered, doubled, or inadmissible question"]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct ExplanationCoverage {
        /// The established issues — at least one, at most the declared bound —
        /// together with whether the body carries every issue the coverage pass
        /// established or names how many stand outside that bound.
        /// One seat rather than two, because a coverage claim seated beside its
        /// body is a claim that can be swapped for another body's.
        ///
        /// Private for the second half of the same claim: a public seat on a
        /// one-field record hands the whole record back as a literal, so any
        /// holder of a body built for one pass could write it into another
        /// pass's refusal.
        /// Read back through [`ExplanationCoverage::body`].
        body: AdmittedPrefix<ExplanationCoverageIssue, ExplanationIssueLimit>,
    }

    impl ExplanationCoverage {
        /// The body a coverage check refuses with.
        ///
        /// The coverage pass walks the kind's whole applicable roster and then
        /// every supplied answer before a body exists, so the posture here is
        /// about the report rather than the pass: where every established issue
        /// fits the declared bound the body carries all of them, and where it
        /// does not, the body carries what the bound holds and names how many
        /// stand outside it.
        ///
        /// Reaches the guard file and no further.
        pub(super) fn established(
            first: ExplanationCoverageIssue,
            rest: Vec<ExplanationCoverageIssue>,
        ) -> Self {
            Self {
                body: AdmittedPrefix::examined_completely(
                    first,
                    rest,
                    &PositiveLimit::<_, AuthoringLimitProfile>::inhabited_under_profile(),
                    StopBound::DeclaredIssueBound,
                ),
            }
        }

        /// The established issues and what this refusal says about its own
        /// coverage of them.
        ///
        /// Borrowed and never owned, for the reason band 00 borrows its carry:
        /// an owned body is a value a caller can seat under another refusal,
        /// which is the pairing the coupled seat exists to end.
        pub const fn body(
            &self,
        ) -> &AdmittedPrefix<ExplanationCoverageIssue, ExplanationIssueLimit> {
            &self.body
        }
    }
}

impl ProjectionExplanation {
    /// Answer one question.
    ///
    /// The answer is the whole input, and that is the claim.
    /// The question is taken from the answer, so no explanation can file a true
    /// answer under the wrong question; the rendering is taken from the answer
    /// too, so no explanation can carry a sentence its typed content does not
    /// support.
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
    /// asked for.
    /// Never stored, never read back, and never supplied.
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
    /// doubled question, and every question the kind does not admit.
    /// All of them are reported together: a caller repairing a view one
    /// question per attempt is a caller the protocol failed.
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

    /// Whether the view holds no answer.
    /// A kind's applicable roster is never empty, so a complete view always has
    /// seats.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.answers.is_empty()
    }
}
