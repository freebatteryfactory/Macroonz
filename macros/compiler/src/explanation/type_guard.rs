//! The explanation home's invariant nucleus: every road that reaches a private field.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's two central claims structural.
//! A view is completed here, after the coverage pass agreed, so there is no partial view for a reader to mistake for a complete one.
//! Its parentage is taken here too — off the plan and the proof themselves — and its own identity is minted over the three, so a view naming a plan or a closure it was not answered over is a value nobody can build.

use super::super::encode::seats_into;
use super::super::establish::coverage_issues;
use super::{
    AnsweredOutput, EXPLANATION_ISSUE_LIMIT, ExplanationError, ExplanationIssue, UniversalAnswer,
    UniversalQuestion, View,
};
use crate::bounded::{Bounded, Capped, Capping, NonEmpty, Overflow};
use crate::closure::Closure;
use crate::identity::{
    self, ClosureId, ExplanationId, PlanId, Provenance, Transcript, encode_bytes,
};
use crate::kind::{Answer, Kind, Question, Role};
use crate::plan::Plan;
use crate::render::RenderedProjection;
use core::marker::PhantomData;

/// The refusal one established issue list amounts to, or nothing where the pass established none.
///
/// One road for every pass in [`View::complete`](super::View::complete), so no pass can establish issues and then walk on past them.
fn refused(issues: Vec<ExplanationIssue>) -> Option<ExplanationError> {
    let mut established = issues.into_iter();
    let first = established.next()?;
    Some(ExplanationError::over(first, established.collect()))
}

impl AnsweredOutput {
    /// Every seat's half of the output-and-digest answer, in roster order, read off the proof itself.
    ///
    /// Roster order and never rendering order, so the answer does not turn on the sequence a renderer happened to write its units in.
    /// The whole roster and never a chosen row: a kind may fill several seats, and an answer naming fewer than all of them would flatten the expansion's denominator to whichever row was picked.
    /// Seated once: the request road composes its answer through this walk, and [`View::complete`] rebuilds the same walk to compare — one derivation, so the claim and its check cannot drift apart.
    pub(crate) fn roster<R: Role>(rendered: &RenderedProjection<R>) -> Vec<Self> {
        R::ALL
            .iter()
            .copied()
            .filter_map(|role| rendered.under(role))
            .map(|unit| Self {
                output: Box::new(unit.reconstructed().output),
                digest: unit.digest(),
            })
            .collect()
    }
}

/// The issue the output answer establishes against the proof's own rendered roster, or nothing where it restates it exactly.
///
/// The lawful rows are derivable from the closure, so this pass rebuilds them and compares whole — count, order, members, and digests in one equality.
/// An absent output answer is the coverage pass's finding and establishes nothing here.
fn outputs_beside_proof<R: Role>(
    universal: &[UniversalAnswer],
    closure: &Closure<R>,
) -> Vec<ExplanationIssue> {
    let supplied = universal.iter().find_map(|answer| match answer {
        UniversalAnswer::OutputAndDigest { outputs } => Some(outputs),
        UniversalAnswer::Kind { .. }
        | UniversalAnswer::Owner { .. }
        | UniversalAnswer::CausingDeclarations { .. }
        | UniversalAnswer::Profile { .. }
        | UniversalAnswer::Assumptions { .. }
        | UniversalAnswer::Invalidators { .. }
        | UniversalAnswer::RelatedDispositions { .. }
        | UniversalAnswer::Repairs { .. } => None,
    });
    let Some(supplied) = supplied else {
        return Vec::new();
    };
    let lawful = AnsweredOutput::roster(closure.rendered());
    if supplied.len() == lawful.len() && supplied.iter().eq(lawful.iter()) {
        return Vec::new();
    }
    vec![ExplanationIssue::OutputsBesideTheProof {
        expected: u16::try_from(lawful.len()).unwrap_or(u16::MAX),
        observed: u16::try_from(supplied.len()).unwrap_or(u16::MAX),
    }]
}

/// The supplied answers, restated in their roster's own declared order.
///
/// Reached only after the coverage pass agreed, which is what makes it total: every row of the roster has exactly one answer and no answer stands outside it, so the walk seats each answer once and leaves none behind.
/// The roster is the quantifier, so the result's order is the protocol's rather than a call site's.
fn in_roster_order<Q: Question>(answers: Vec<Q::Answer>) -> Vec<Q::Answer> {
    let mut supplied: Vec<Option<Q::Answer>> = answers.into_iter().map(Some).collect();
    let mut ordered: Vec<Q::Answer> = Vec::with_capacity(supplied.len());
    for question in Q::ALL {
        let seated = supplied.iter_mut().find(|held| {
            held.as_ref()
                .is_some_and(|answer| answer.question() == *question)
        });
        if let Some(answer) = seated.and_then(Option::take) {
            ordered.push(answer);
        }
    }
    ordered
}

impl ExplanationError {
    /// The refusal one established issue makes.
    pub fn of(issue: ExplanationIssue) -> Self {
        Self {
            body: Capped::all(NonEmpty::one(issue)),
        }
    }

    /// The refusal a pass whose checks co-establish makes.
    ///
    /// The caller arrives holding every issue its pass established, so the posture the body writes is about the REPORT and never about the pass: where the issues fit it carries all of them, and where they do not it carries what fits and counts the rest.
    pub fn over(first: ExplanationIssue, rest: Vec<ExplanationIssue>) -> Self {
        Self {
            body: Capped::first_n(first, rest.into_iter()),
        }
    }

    /// The refusal a seat bound makes, out of the two counts the overflow already carries.
    pub fn bounded(overflow: Overflow) -> Self {
        Self::of(ExplanationIssue::SeatBoundExceeded {
            bound: u64::try_from(overflow.capacity).unwrap_or(u64::MAX),
            observed: u64::try_from(overflow.offered).unwrap_or(u64::MAX),
        })
    }

    /// The first issue the pass established, which every refusal has.
    #[must_use]
    pub fn first_issue(&self) -> &ExplanationIssue {
        self.body.items().first()
    }

    /// Every issue this refusal carries, in the order the pass established them; structurally at least one.
    #[must_use]
    pub fn issues(&self) -> &NonEmpty<ExplanationIssue, EXPLANATION_ISSUE_LIMIT> {
        self.body.items()
    }

    /// Whether this refusal carries every issue its pass established.
    #[must_use]
    pub const fn capping(&self) -> Capping {
        self.body.capping()
    }
}

impl<K: Kind> View<K> {
    /// Complete one view over the universal questions and the kind's own, answered over one plan and the proof of its rendering.
    ///
    /// # The parentage is taken, never supplied
    ///
    /// The plan arrives as the PLAN and the closure as the PROOF, and both identities are read off them here.
    /// A road that took two identities beside the answers would take two values any caller can spell, and the view it built would name a parentage it was never written over — which is a complete, well-formed explanation about something else.
    /// A [`Closure`] is reachable only by proving a rendering against a plan, so a caller standing here has done that or has nothing to hand in, and its role roster is the kind's own.
    ///
    /// # The explanation transcript
    ///
    /// This is a mint site, so its content grammar is stated in full.
    /// The identity is derived under [`Role::Explanation`](crate::identity::Role::Explanation), anchored on the CLOSURE's identity at full width — an explanation is written after a closure and over it — at position zero, over
    ///
    /// ```text
    /// content = bytes(plan) || u64be(universal seats) || seat* || u64be(declared seats) || seat*
    /// ```
    ///
    /// where each `seat` is the question's roster position in two big-endian bytes followed by the answer's own canonical bytes, in that roster's declared order.
    /// The two rosters are written behind two counts, so the split between them is framed rather than inferred: a universal seat and a declared seat may share a position.
    /// Human prose is not a member — a rendered line is a projection of a typed answer, so a preimage carrying one would commit to a rendering rather than to what was answered.
    ///
    /// # Errors
    ///
    /// Returns [`ExplanationError`] naming every unanswered question, every doubled question, every answer standing outside its own roster, the seat bound where a kind's roster outgrows it, and an output answer that does not restate the proof's own rendered roster.
    /// All of them together: a caller repairing a view one question per attempt is a caller the protocol failed.
    pub fn complete(
        plan: &Plan<K>,
        closure: &Closure<K::Role>,
        universal: Vec<UniversalAnswer>,
        declared: Vec<<K::Question as Question>::Answer>,
    ) -> Result<Self, ExplanationError> {
        if let Some(refusal) = refused(coverage_issues::<K>(&universal, &declared)) {
            return Err(refusal);
        }
        if let Some(refusal) = refused(outputs_beside_proof(&universal, closure)) {
            return Err(refusal);
        }
        let seated_universal = in_roster_order::<UniversalQuestion>(universal);
        let seated_declared = in_roster_order::<K::Question>(declared);

        let plan_identity = plan.identity();
        let closure_identity = closure.identity();
        let mut content = Vec::new();
        encode_bytes(plan_identity.as_bytes(), &mut content);
        seats_into(&seated_universal, &mut content);
        seats_into(&seated_declared, &mut content);
        let (derived, provenance) =
            ExplanationId::derived_with_provenance(Transcript::under_projection(
                identity::Role::Explanation,
                &closure_identity,
                &content,
                0,
            ));

        let held_universal = Bounded::new(seated_universal).map_err(ExplanationError::bounded)?;
        let held_declared = Bounded::new(seated_declared).map_err(ExplanationError::bounded)?;
        Ok(Self {
            plan: plan_identity,
            closure: closure_identity,
            universal: held_universal,
            declared: held_declared,
            identity: derived,
            provenance,
            kind: PhantomData,
        })
    }

    /// This view's own identity — the name a binding commits to.
    #[must_use]
    pub const fn identity(&self) -> ExplanationId {
        self.identity
    }

    /// The record of how that identity was derived.
    #[must_use]
    pub const fn provenance(&self) -> &Provenance {
        &self.provenance
    }

    /// The plan this view was answered over.
    ///
    /// Read back so a binding establishes that the plan it seals is the plan the answers are about, rather than assuming it.
    #[must_use]
    pub const fn plan(&self) -> PlanId {
        self.plan
    }

    /// The proved closure this view was answered over, on the same terms.
    #[must_use]
    pub const fn closure(&self) -> ClosureId {
        self.closure
    }

    /// The universal answers, in the compiler's roster order.
    #[must_use]
    pub fn universal(&self) -> &[UniversalAnswer] {
        self.universal.as_slice()
    }

    /// The kind's own answers, in the kind's declared roster order.
    #[must_use]
    pub fn declared(&self) -> &[<K::Question as Question>::Answer] {
        self.declared.as_slice()
    }

    /// How many seats this view fills, across both rosters.
    #[must_use]
    pub fn seats(&self) -> usize {
        self.universal.len().saturating_add(self.declared.len())
    }
}
