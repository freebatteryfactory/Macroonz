//! The explanation home's declarations: the universal roster and its typed answers, the seat a related kind is accounted at, one complete view, and how coverage refuses.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this file's own child, which is what makes a view's parentage taken rather than supplied.

use crate::bounded::{Bounded, Capped};
use crate::diagnostic::{REPAIR_LIMIT, Repair};
use crate::identity::{
    self, ClosureId, ExplanationId, Identity, OwnerFact, PlanId, Profile, Provenance,
};
use crate::kind::{Disposition, Kind, Question};
use crate::plan::{DEPENDENCY_LIMIT, InvalidationSet, MEMBERSHIP_LIMIT, PlannedOutput};
use core::marker::PhantomData;

#[path = "type_guard.rs"]
mod guard;

/// How many questions every kind owes, whatever it is.
///
/// The width of a complete view's universal half, checked against the roster itself where the roster is written down.
pub const UNIVERSAL_QUESTION_COUNT: usize = 9;

/// Owner facts one answer may carry as the assumptions a projection rests on.
pub const ASSUMPTION_LIMIT: usize = 16;

/// Related kinds one answer may account for.
pub const RELATED_KIND_LIMIT: usize = 16;

/// Questions one kind may declare beyond the universal roster.
///
/// A kind whose roster outgrows this refuses: an answer sheet cut to fit is byte for byte the shape of a complete one.
pub const DECLARED_QUESTION_LIMIT: usize = 32;

/// Issues one coverage refusal carries before it begins counting the rest.
///
/// One per universal seat, one per declared seat at the widest roster, and room for the answers that stand outside a roster entirely.
pub const EXPLANATION_ISSUE_LIMIT: usize = 48;

/// One question every generated thing answers, whatever kind it is.
///
/// A kind narrows nothing here — its own questions are a second roster answered beside this one — so no kind can owe less by declaring less.
/// A row's position is what a complete view's preimage carries for it, so a row is appended and never renumbered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UniversalQuestion {
    /// What are you?
    WhatAreYou,
    /// Which owner required you?
    WhichOwnerRequired,
    /// Which declaration caused you?
    WhichDeclarationCaused,
    /// Which profile were you decided under?
    WhichProfile,
    /// Which output identity and digest are you?
    WhichOutputAndDigest,
    /// Which assumptions do you rest on?
    WhichAssumptions,
    /// What invalidates you?
    WhatInvalidates,
    /// Why was a related projection not generated?
    WhyRelatedNotGenerated,
    /// What repairs a refusal?
    WhatRepairsARefusal,
}

/// One kind a projection is related to, and what happened to that kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelatedDisposition {
    /// The related kind's declared name.
    pub kind: &'static str,
    /// What happened to it.
    pub disposition: Disposition,
}

/// One seat's half of the output-and-digest answer: the member the plan declared there, and the digest the closure proved over its rendered bytes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnsweredOutput {
    /// The planned member, boxed so one row does not set the width of the roster.
    pub output: Box<PlannedOutput>,
    /// The digest proved over the rendered bytes.
    pub digest: Identity<identity::OutputBytes>,
}

/// One typed answer to a universal question.
///
/// Every arm carries the exact values that answer its row — identities, typed rosters, typed dispositions — and never a sentence standing in for a fact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniversalAnswer {
    /// The kind this output is.
    Kind {
        /// The kind's declared name.
        name: &'static str,
    },
    /// The owner fact that required it.
    Owner {
        /// The requiring fact.
        owner: OwnerFact,
    },
    /// The declarations it was derived from.
    CausingDeclarations {
        /// The content commitment the request walked in with.
        commitment: Identity<identity::CapturedDeclaration>,
        /// The captures that content declares it stands on.
        dependencies: Bounded<Identity<identity::CapturedDeclaration>, DEPENDENCY_LIMIT>,
    },
    /// The profile it was decided under.
    Profile {
        /// The profile, at the version it was decided at.
        profile: Profile,
    },
    /// Every member it is, and the digest proved over each one's rendered bytes.
    ///
    /// The complete set in roster order, never a chosen row: a kind's roster may fill several seats, and an answer naming one of them would be coverage-complete syntax over a flattened denominator — the second output's identity and digest simply absent from a view that claims the whole expansion.
    /// Each row is two values because they come from two places: the member is what the plan declared, and the digest is what the closure proved.
    OutputAndDigest {
        /// One row per rendered seat, in roster order; never empty in a lawful expansion, because a rendering is structurally non-empty — and completion refuses a set that does not restate the proof's own roster, so a shortened or reordered answer cannot ride a coverage-complete view.
        outputs: Bounded<AnsweredOutput, MEMBERSHIP_LIMIT>,
    },
    /// The owner facts it rests on.
    Assumptions {
        /// The assumed facts.
        assumptions: Bounded<OwnerFact, ASSUMPTION_LIMIT>,
    },
    /// The triggers whose change makes it stale.
    Invalidators {
        /// The watch set.
        triggers: InvalidationSet,
    },
    /// What happened to every kind it is related to.
    RelatedDispositions {
        /// The accounted kinds.
        related: Bounded<RelatedDisposition, RELATED_KIND_LIMIT>,
    },
    /// The owner-declared repairs that apply.
    Repairs {
        /// The declared repairs.
        repairs: Bounded<Repair, REPAIR_LIMIT>,
    },
}

/// One way a set of answers fails to cover the questions a kind owes.
///
/// No row is payload-free: an issue names the question it is about, because a bare row makes the reader guess which seat to repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplanationIssue {
    /// A universal question has no answer.
    UniversalUnanswered {
        /// The unanswered question.
        question: UniversalQuestion,
    },
    /// A universal question was answered more than once.
    UniversalAnsweredTwice {
        /// The doubled question.
        question: UniversalQuestion,
    },
    /// A question the kind declared has no answer.
    DeclaredUnanswered {
        /// The question's declared name.
        question: &'static str,
        /// Its position in the kind's roster.
        slot: u16,
    },
    /// A question the kind declared was answered more than once.
    DeclaredAnsweredTwice {
        /// The question's declared name.
        question: &'static str,
        /// Its position in the kind's roster.
        slot: u16,
    },
    /// An answer names a question its own roster does not carry.
    QuestionOutsideRoster {
        /// The question's declared name.
        question: &'static str,
    },
    /// More seats were offered than a declared bound admits.
    SeatBoundExceeded {
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
    /// The output answer does not restate the proof's own rendered roster.
    ///
    /// The lawful rows are derivable from the closure a view is completed over, so the pass rebuilds them and compares whole — a missing seat, an extra row, a reordered roster, and a digest that is not the proof's all land here rather than riding a coverage-complete view.
    OutputsBesideTheProof {
        /// Rows the proof's rendered roster carries.
        expected: u16,
        /// Rows the supplied answer carried.
        observed: u16,
    },
}

/// How the explanation protocol says no.
///
/// Coverage issues are independent and co-establishable — several questions may stand unanswered while another is doubled — so the body carries every issue the pass established rather than electing a primary one, and says so where it kept only what fits.
#[must_use = "a coverage refusal carries every uncovered, doubled, and inadmissible question"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExplanationError {
    body: Capped<ExplanationIssue, EXPLANATION_ISSUE_LIMIT>,
}

/// One complete explanation: every question a kind owes, answered exactly once, over the plan and the proof the answers are ABOUT.
///
/// Holding one is the coverage proof, and there is no partial view — a set of answers that could not be completed is a refusal instead.
///
/// # Authority
///
/// **The parentage is taken and never supplied.**
/// A view assembled from two identities beside the answers would name a plan and a closure it was never written over: every question answered correctly, about a different expansion of the same kind, and the type parameter cannot catch that because a kind is not an expansion.
///
/// # Ordering
///
/// The universal seats stand in the compiler's roster order and the declared seats in the kind's, never in the order a caller supplied them.
/// That order is what the identity is derived over, so one set of answers is one explanation however it was assembled.
#[must_use = "a complete view is the proof every question has exactly one answer, over the plan and closure it names"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct View<K: Kind> {
    plan: PlanId,
    closure: ClosureId,
    universal: Bounded<UniversalAnswer, UNIVERSAL_QUESTION_COUNT>,
    declared: Bounded<<K::Question as Question>::Answer, DECLARED_QUESTION_LIMIT>,
    identity: ExplanationId,
    provenance: Provenance,
    kind: PhantomData<K>,
}
