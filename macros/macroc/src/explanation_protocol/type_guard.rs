//! The explanation-protocol home's invariant nucleus: every road that reaches a
//! private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes the
//! protocol's central claims structural. An explanation's question and its human
//! rendering are both taken HERE, from the answer itself, so a true answer filed
//! under the wrong question and a sentence that contradicts its answer are
//! values nobody can build. A view is completed HERE, after the coverage pass
//! agreed, so there is no partial view for a reader to mistake for a complete
//! one. The refusal BODY is built here for the same reason and by the same
//! permission: its seat is private, so this file is the only module in the
//! workspace that can spell the literal, and every refusal that exists came off
//! the coverage pass.
//!
//! # Why the body is DECLARED here and not in `types.rs`
//!
//! Rust's privacy is MODULE-scoped, so a seat declared in `types.rs` puts every
//! other item in that file inside the wall and leaves "did anybody write a road
//! out?" as a whole-file audit. The body is therefore declared in the `seat`
//! module below, whose entire content is that record and inherent
//! implementations of it — held to exactly that by `cargo xtask check`'s
//! `seat-modules-carry-nothing-else`.
//!
//! # What a private seat does and does not exclude
//!
//! It excludes every SIBLING: the rest of this file, `types.rs` above it,
//! `establish.rs` beside it, anywhere else in the services, and any crate
//! downstream cannot write the literal, and the compiler says so with `E0451`.
//! It does not exclude DESCENDANTS — a module declared inside the seat would
//! construct as freely as these roads do, which is why the reversals for this
//! seat are testpak's compile-fail fixtures and why the law above refuses a
//! nested module in a `seat` module outright.

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
        /// established or names how many stand outside that bound. One seat
        /// rather than two, because a coverage claim seated beside its body is a
        /// claim that can be swapped for another body's. The pass itself always
        /// covers every applicable question, so the completion here never
        /// reports a halted examination.
        ///
        /// Private, and that is the second half of the same claim. The coupled
        /// seat keeps a carry and its posture together; a PUBLIC seat on a
        /// one-field record hands the whole record back as a literal, so any
        /// holder of a body built for one pass could write it into another
        /// pass's refusal. Read back through [`ExplanationCoverage::body`].
        body: AdmittedPrefix<ExplanationCoverageIssue, ExplanationIssueLimit>,
    }

    impl ExplanationCoverage {
        /// The body a coverage check refuses with.
        ///
        /// The coverage pass walks the kind's whole applicable roster and then
        /// every supplied answer before a body exists, so the posture here is
        /// about the REPORT rather than the pass. Where every established issue
        /// fits the declared bound the body carries all of them; where it does
        /// not, the body carries what the bound holds and names how many
        /// established issues stand outside it.
        ///
        /// Reaches the guard file and no further — `pub(super)` from inside the
        /// seat is exactly the module-private reach this road had before the
        /// declaration moved, and the pass that raises it is beside it.
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
