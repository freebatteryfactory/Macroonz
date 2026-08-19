//! The explanation-protocol home's declarations: the typed answers, one
//! answered question, how a view fails to be complete, the sealed proof
//! contract a view is answered over, and the complete view itself.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `type_guard.rs`, this
//! file's own child, which is what makes "the question is derived from the
//! answer" and "the parentage is taken rather than supplied" structural rather
//! than reviewed.

use crate::diagnostics::RepairAction;
use crate::origin_graph::DecisionTrace;
use crate::plane::{
    AssumptionLimit, ClosureId, ExplanationId, ExplanationSeatLimit, GeneratedUnitSubject,
    MembershipLimit, OutputBytesSubject, OwnerFactRef, OwnerIdentityRef, PatternInstanceSubject,
    PatternSubject, PlanId, ProfileVersion, ProjectionIdentity, ProjectionKindSubject,
    ProjectionProfileSubject, ProjectionProvenance, RenderedRole, RepairLimit, RuntimeTraceSubject,
    TraceEntryLimit,
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

/// The seal on the proved-closure contract.
///
/// A value of this type is producible only inside the services, so nothing
/// declared anywhere else can satisfy [`ProvedClosure`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClosureProofSeal(());

/// What a complete view is answered OVER at its closure end: a proof that a
/// rendering is what a plan declared, read for its own name.
///
/// # Why a contract rather than the value
///
/// The proof lives in the closure home, and the closure home is declared AFTER
/// this one because its terminal binds a complete view — so this home cannot
/// name [`ProjectionClosure`](crate::closure::ProjectionClosure) without the
/// module order carrying a backward edge, which is the one thing that order
/// exists to make unwritable.
///
/// The contract closes that gap without loosening anything. It is SEALED, so the
/// only implementation there can be is the proof itself: a caller handing a view
/// its closure is handing over a value it could only have obtained by proving
/// one, and there is no second type in the workspace that satisfies this
/// contract.
///
/// # Bounds
///
/// It answers with the proof's own NAME and nothing else. What a closure proved,
/// what it partitioned, and what it hands out are the closure's own surface, and
/// a view reads none of them — a view is written over a proof, not out of one.
pub trait ProvedClosure {
    /// The seal. Only the services can produce a value of this type.
    const SEAL: ClosureProofSeal;

    /// The closed roster of rendered roles the proof stands over.
    ///
    /// Carried so a view over one kind can only be answered over a proof of that
    /// kind's own rendering: a kind is not an expansion, and a proof of another
    /// kind's roster is a proof of something else.
    type Rendered: RenderedRole;

    /// The proof's own identity.
    fn identity(&self) -> ClosureId;
}

/// One typed answer.
///
/// Each variant carries the exact values that answer its question — identities,
/// typed rosters, and typed dispositions, never a sentence standing in for a
/// fact.
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
        /// The one anchored cause address.
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
    /// Two values, because they come from two places: the planned member is
    /// what the plan declared, and the digest is what the closure proved over
    /// bytes that exist.
    /// An answer carrying only the first would be answering half the question;
    /// an answer carrying a digest the plan supplied would be answering it with
    /// a value nobody computed.
    OutputAndDigest {
        /// The planned member.
        /// Boxed because one answer must not set the size of every other.
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
/// There is no seat here for a rendering.
/// The line a person reads is a function of the answer, composed when it is
/// asked for rather than carried, so an explanation whose sentence contradicts
/// its typed content is not a value anybody can build.
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
/// declared in `type_guard.rs`'s `seat` module, beside the only roads that
/// reach its seat.
pub use guard::ExplanationCoverage;

/// A complete explanation view over one kind's plans, answered over one plan and
/// one proved closure.
///
/// Holding one is the proof: every applicable question has exactly one answer,
/// no question outside the kind's roster was answered, and the seats stand in
/// the kind's own declared question order.
/// There is no partial view — a view that could not be completed is a refusal
/// instead.
///
/// # Authority
///
/// **A view carries the parentage it was answered over, and its identity commits
/// to it.** The plan and the closure are not decoration beside the answers: they
/// are what the answers are ABOUT. A view that carried coverage alone was a
/// value a terminal could bind under plan A and closure A while it had been
/// written over another expansion of the same kind — every question answered
/// correctly, about something else — and the type parameter could not catch it,
/// because a kind is not an expansion.
///
/// So the seats below arrive together and none is supplied: the constructor is
/// handed the ACTUAL plan and the ACTUAL closure and reads their identities off
/// them, which is why a caller cannot name a parentage the view was not written
/// over.
///
/// # Ordering
///
/// The answers are stored in the kind's DECLARED question order and never in the
/// order a caller supplied them. That order is what the identity is derived
/// over, so one set of answers is one explanation however it was assembled — and
/// a reader walking the seats reads the protocol's own order rather than a
/// call site's.
#[must_use = "a complete view is the proof every applicable question has exactly one answer, over the plan and closure it names"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionExplanationView<K: ProjectionKind> {
    plan: PlanId,
    closure: ClosureId,
    answers: Bounded<ProjectionExplanation, ExplanationSeatLimit>,
    identity: ExplanationId,
    provenance: ProjectionProvenance,
    _kind: PhantomData<K>,
}
