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
//! A view's PARENTAGE is taken here too, off the plan and the proof themselves,
//! and its own identity is minted over the three — so a view that names a plan
//! or a closure it was not answered over is a value nobody can build either, and
//! the terminal that binds one has a name to compare rather than a claim to
//! trust.
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

use super::super::encode::answered_seats;
use super::super::establish::coverage_issues;
use super::super::project::human_line;
use super::{
    ClosureProofSeal, ExplanationAnswer, ExplanationCoverageIssue, ExplanationSeatLimit,
    ProjectionExplanation, ProjectionExplanationView, ProvedClosure,
};
use crate::plane::{
    AuthoringLimitProfile, ClosureId, ExplanationId, HumanProjection, HumanTextLimit, PlanId,
    ProjectionProvenance, ProjectionRole, ProjectionTranscript, encode_bytes,
};
use crate::planning::{ProjectionKind, ProjectionPlan};
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
    use super::super::{ExplanationCoverageIssue, ExplanationIssueLimit};
    use crate::plane::AuthoringLimitProfile;
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

impl ClosureProofSeal {
    /// The seal, admitted only within the services.
    pub(crate) const fn admitted() -> Self {
        Self(())
    }
}

/// The supplied answers, restated in the kind's own declared question order.
///
/// Reached only after the coverage pass agreed, which is what makes it TOTAL:
/// every applicable question has exactly one answer and no answer stands outside
/// the roster, so the walk below places every supplied answer exactly once and
/// leaves none behind.
///
/// The roster is the quantifier, so the result's order is the protocol's rather
/// than the caller's — which is what lets one set of answers be one explanation
/// however a call site assembled it.
fn in_declared_order<K: ProjectionKind>(
    answers: Vec<ProjectionExplanation>,
) -> Vec<ProjectionExplanation> {
    let mut supplied: Vec<Option<ProjectionExplanation>> = answers.into_iter().map(Some).collect();
    let mut ordered: Vec<ProjectionExplanation> = Vec::with_capacity(supplied.len());
    for question in ProjectionPlan::<K>::applicable_questions() {
        let seated = supplied.iter_mut().find(|held| {
            held.as_ref()
                .is_some_and(|answer| answer.question() == question)
        });
        if let Some(answer) = seated.and_then(Option::take) {
            ordered.push(answer);
        }
    }
    ordered
}

impl<K: ProjectionKind> ProjectionExplanationView<K> {
    /// Complete the view over the kind's applicable questions, answered over one
    /// plan and one proved closure.
    ///
    /// # The parentage is taken, never supplied
    ///
    /// The plan arrives as the PLAN and the closure as the PROOF, and their
    /// identities are read off them here. A road that took two identities beside
    /// the answers would take two values any caller can spell, and the view it
    /// built would name a parentage it was never written over — which is a
    /// complete, well-formed explanation about a different expansion.
    ///
    /// [`ProvedClosure`] is sealed, so the only value that satisfies the closure
    /// end is a proof: a caller reaching this road has proved a rendering
    /// against a plan, or it has nothing to hand in.
    ///
    /// # The explanation transcript
    ///
    /// This is a mint site, so its content grammar is stated here in full, the
    /// way [`ProjectionTranscript`] requires of every mint site. The identity is
    /// derived under [`ProjectionRole::Explanation`], anchored on the CLOSURE's
    /// identity — an explanation is written after a closure and over it — at
    /// roster position zero, over
    ///
    /// ```text
    /// content = bytes(plan_identity) || u64be(seats) || seat*
    /// ```
    ///
    /// where each `seat` is the question's roster slot, the answer's own
    /// discriminant, and the answer's typed material length-framed, in the
    /// kind's DECLARED question order. The seats' spelling is
    /// `explanation_protocol::encode`, which writes each typed value through the
    /// road its own home declares.
    ///
    /// Human prose is not a member: a rendered line is a projection of a typed
    /// answer, composed when it is asked for, so a preimage carrying one would
    /// commit to a rendering rather than to what was answered. The full
    /// statement, including the one posture written narrower than it reads,
    /// is [`EXPLANATION_IDENTITY_PROFILE`](crate::plane::EXPLANATION_IDENTITY_PROFILE).
    ///
    /// # Errors
    ///
    /// Returns [`ExplanationCoverage`] naming every unanswered question, every
    /// doubled question, and every question the kind does not admit.
    /// All of them are reported together: a caller repairing a view one
    /// question per attempt is a caller the protocol failed.
    pub fn complete<C>(
        plan: &ProjectionPlan<K>,
        closure: &C,
        answers: Vec<ProjectionExplanation>,
    ) -> Result<Self, ExplanationCoverage>
    where
        C: ProvedClosure<Rendered = K::Rendered>,
    {
        if let Some(refusal) = refused(coverage_issues::<K>(&answers)) {
            return Err(refusal);
        }
        let observed = answers.len();
        let ordered = in_declared_order::<K>(answers);

        let plan_identity = plan.identity();
        let closure_identity = closure.identity();
        let mut content = Vec::new();
        encode_bytes(plan_identity.as_bytes(), &mut content);
        answered_seats(&ordered, &mut content);
        let (identity, provenance) =
            ExplanationId::derived_with_provenance(ProjectionTranscript::under_projection(
                ProjectionRole::Explanation,
                &closure_identity,
                &content,
                0,
            ));

        Bounded::admitted_const(
            ordered,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map(|admitted| Self {
            plan: plan_identity,
            closure: closure_identity,
            answers: admitted,
            identity,
            provenance,
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

    /// This view's own identity — the name a terminal binds it under and
    /// commits to.
    #[must_use]
    pub const fn identity(&self) -> ExplanationId {
        self.identity
    }

    /// How that identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &ProjectionProvenance {
        &self.provenance
    }

    /// The plan this view was answered over.
    ///
    /// Read back so a terminal can establish that the plan it is binding is the
    /// plan the answers are about, rather than assuming it.
    #[must_use]
    pub const fn plan(&self) -> PlanId {
        self.plan
    }

    /// The proved closure this view was answered over, on the same terms.
    #[must_use]
    pub const fn closure(&self) -> ClosureId {
        self.closure
    }

    /// The answered seats, in the kind's declared question order.
    ///
    /// # Ordering
    ///
    /// The protocol's own order and never a caller's: it is what the identity
    /// was derived over, so a reader walking these seats walks exactly what the
    /// name commits to.
    pub fn answers(&self) -> impl Iterator<Item = &ProjectionExplanation> {
        self.answers.iter()
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
