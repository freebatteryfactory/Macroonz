//! The explanation-protocol home's declarations: the typed answers, one
//! answered question, how a view fails to be complete, and the complete view.
//!
//! Declarations only. Every road that reaches a private field — an explanation's
//! question, answer, and human projection, the view's own seats, and the refusal
//! body's one seat — lives in `type_guard.rs`, this file's own child. That is
//! what makes "the question is derived from the answer" structural rather than
//! reviewed.

use crate::diagnostics::RepairAction;
use crate::origin_graph::DecisionTrace;
use crate::plane::{
    AssumptionLimit, ExplanationSeatLimit, GeneratedUnitSubject, MembershipLimit,
    OutputBytesSubject, OwnerFactRef, OwnerIdentityRef, PatternInstanceSubject, PatternSubject,
    ProfileVersion, ProjectionIdentity, ProjectionKindSubject, ProjectionProfileSubject,
    RepairLimit, RuntimeTraceSubject, TraceEntryLimit,
};
use crate::planning::{
    CauseAnchoring, GraphAnchoring, InvalidationSet, PlannedOutput, ProjectionDisposition,
    ProjectionKind,
};
use crate::question::ExplanationQuestion;
use core::marker::PhantomData;
use threadpak::types::Bounded;

#[path = "type_guard.rs"]
mod guard;

/// One typed answer. Each variant carries the exact values that answer its
/// question — identities, typed rosters, and typed dispositions, never a
/// sentence standing in for a fact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplanationAnswer {
    /// What are you: the projection kind.
    Kind {
        /// The kind's identity.
        kind: ProjectionIdentity<ProjectionKindSubject>,
    },
    /// Which owner required you.
    Owner {
        /// The requiring owner fact.
        owner: OwnerFactRef,
    },
    /// Which declarations caused you.
    CausingDeclarations {
        /// The cause set.
        sources: CauseAnchoring,
    },
    /// Which pattern instance produced you.
    PatternInstance {
        /// The authored pattern.
        pattern: OwnerIdentityRef<PatternSubject>,
        /// This instantiation of it.
        instance: OwnerIdentityRef<PatternInstanceSubject>,
    },
    /// Which graph and profile you were decided under.
    GraphAndProfile {
        /// What the plan was decided against.
        graph: GraphAnchoring,
        /// The profile.
        profile: ProjectionIdentity<ProjectionProfileSubject>,
        /// That profile's version.
        version: ProfileVersion,
    },
    /// Which capabilities selected your wrappers: the trace that decided them.
    SelectedWrappers {
        /// The selection trace.
        trace: DecisionTrace,
    },
    /// Which assumptions and specializations you rest on.
    AssumptionsAndSpecializations {
        /// The assumed owner facts.
        assumptions: Bounded<OwnerFactRef, AssumptionLimit>,
    },
    /// Which output identity and digest you are.
    ///
    /// Two values, because they come from two places and always did: the planned
    /// member is what the PLAN declared, and the digest is what the CLOSURE
    /// proved over bytes that exist. An answer carrying only the first would be
    /// answering half the question; an answer carrying a digest the plan
    /// supplied would be answering it with a value nobody computed.
    OutputAndDigest {
        /// The planned member. Boxed because one answer of fourteen must not set
        /// the size of the other thirteen.
        output: Box<PlannedOutput>,
        /// The digest the closure proved over the bytes actually rendered.
        digest: ProjectionIdentity<OutputBytesSubject>,
    },
    /// Which tests challenge you.
    ChallengingTests {
        /// The test descriptors.
        descriptors: Bounded<ProjectionIdentity<GeneratedUnitSubject>, MembershipLimit>,
    },
    /// Which benchmarks measure you.
    MeasuringBenchmarks {
        /// The benchmark descriptors.
        descriptors: Bounded<ProjectionIdentity<GeneratedUnitSubject>, MembershipLimit>,
    },
    /// Which runtime traces correspond to you.
    CorrespondingRuntimeTraces {
        /// The corresponding traces.
        traces: Bounded<OwnerIdentityRef<RuntimeTraceSubject>, TraceEntryLimit>,
    },
    /// What invalidates you.
    Invalidators {
        /// The watch set.
        triggers: InvalidationSet,
    },
    /// Why a related projection was not generated.
    RelatedProjectionDisposition {
        /// The related kind.
        related: ProjectionIdentity<ProjectionKindSubject>,
        /// What happened to it.
        disposition: ProjectionDisposition,
    },
    /// What repairs a refusal.
    Repairs {
        /// The owner-declared repairs.
        repairs: Bounded<RepairAction, RepairLimit>,
    },
}

/// One answered question: the typed answer, and the question it answers.
///
/// # Two seats, because the rendering is not one
///
/// There is no seat here for a rendering. The line a person reads is a function
/// of the answer — see `project.rs` — and it is composed when it is asked for
/// rather than carried, so an explanation whose sentence contradicts its typed
/// content is not a value anybody can build. A stored rendering would be a
/// second value answering a question the answer already answers, which is
/// exactly what a projection may never become.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionExplanation {
    question: ExplanationQuestion,
    answer: ExplanationAnswer,
}

/// How one explanation view fails to be complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplanationCoverageIssue {
    /// An applicable question has no answer.
    QuestionUnanswered(ExplanationQuestion),
    /// An applicable question was answered more than once.
    QuestionAnsweredTwice(ExplanationQuestion),
    /// A question the kind does not admit was answered anyway.
    QuestionNotApplicableToKind(ExplanationQuestion),
    /// More answers were supplied than the declared seat bound admits.
    SeatBoundExceeded {
        /// The declared bound.
        bound: u64,
        /// The observed count.
        observed: u64,
    },
}

/// The explanation-coverage refusal family body, published from this file and
/// DECLARED in `type_guard.rs`'s `seat` module, beside the only roads that reach
/// its seat.
///
/// The declaration is not here because Rust's privacy is MODULE-scoped: a
/// private field is private to the module the declaration lands in, and this
/// file declares much else that would have been inside that wall.
pub use guard::ExplanationCoverage;

/// A complete explanation view over one kind's plans.
///
/// Holding one is the proof: every applicable question has exactly one answer,
/// and no question outside the kind's roster was answered. There is no partial
/// view — a view that could not be completed is a refusal instead.
#[must_use = "a complete view is the proof every applicable question has exactly one answer"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionExplanationView<K: ProjectionKind> {
    answers: Bounded<ProjectionExplanation, ExplanationSeatLimit>,
    _kind: PhantomData<K>,
}
