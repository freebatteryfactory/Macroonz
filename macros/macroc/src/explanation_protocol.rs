//! The explanation protocol's machinery: the typed answers that carry the
//! questions, and the coverage check that admits no incomplete view.
//!
//! The question roster itself is a leaf vocabulary and lives in
//! [`crate::question`], which both this module and the planning module import.
//! Nothing here restates it.
//!
//! # The protocol is mandatory, and the shape enforces it
//!
//! A projection kind declares which questions its plans answer, and every kind
//! carries the universal roster whether it lists it or not. A
//! [`ProjectionExplanationView`] is complete only when every applicable question
//! has exactly one answer — an unanswered seat, a doubled seat, and an answer
//! to a question the kind does not admit are all refused, each naming the
//! question. No kind ducks the protocol by answering fewer questions than its
//! roster, because the roster is what the view is checked against.
//!
//! # Answers reference identities, not prose
//!
//! Every [`ExplanationAnswer`] carries typed values and exact identities. The
//! human projection riding alongside is for a person to read and is derived
//! from those values; nothing reads it back. The question an answer belongs to
//! is derived from the answer itself, so a mismatched question-and-answer pair
//! is unrepresentable rather than validated.

use crate::diagnostics::RepairAction;
use crate::origin_graph::DecisionTrace;
use crate::plane::{
    AssumptionLimit, ExplanationIssueLimit, ExplanationSeatLimit, GeneratedUnitSubject,
    HumanProjection, HumanTextLimit, MembershipLimit, OutputBytesSubject, OwnerFactRef,
    OwnerIdentityRef, PatternInstanceSubject, PatternSubject, ProfileVersion, ProjectionIdentity,
    ProjectionKindSubject, ProjectionProfileSubject, RepairLimit, RuntimeTraceSubject,
    TraceEntryLimit,
};
use crate::planning::{
    CauseAnchoring, GraphAnchoring, InvalidationSet, PlannedOutput, ProjectionDisposition,
    ProjectionKind, ProjectionPlan,
};
use crate::question::{ExplanationQuestion, QuestionApplicability};
use core::marker::PhantomData;
use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
use threadpak::types::{Bounded, ConstLimit, NonEmptyBounded, NonEmptyBoundedConstruction};

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

impl ExplanationAnswer {
    /// The question this answer answers. Total, and the only road there is: a
    /// pairing between a question and an answer that does not fit it cannot be
    /// built, because the pairing is derived rather than supplied.
    #[must_use]
    pub const fn question(&self) -> ExplanationQuestion {
        match self {
            Self::Kind { .. } => ExplanationQuestion::WhatAreYou,
            Self::Owner { .. } => ExplanationQuestion::WhichOwnerRequired,
            Self::CausingDeclarations { .. } => ExplanationQuestion::WhichDeclarationCaused,
            Self::PatternInstance { .. } => ExplanationQuestion::WhichTemplateOrPatternInstance,
            Self::GraphAndProfile { .. } => ExplanationQuestion::WhichGraphAndProfile,
            Self::SelectedWrappers { .. } => ExplanationQuestion::WhichCapabilitiesSelectedWrappers,
            Self::AssumptionsAndSpecializations { .. } => {
                ExplanationQuestion::WhichAssumptionsAndSpecializations
            }
            Self::OutputAndDigest { .. } => ExplanationQuestion::WhichOutputIdentityAndDigest,
            Self::ChallengingTests { .. } => ExplanationQuestion::WhichTestsChallenge,
            Self::MeasuringBenchmarks { .. } => ExplanationQuestion::WhichBenchmarksMeasure,
            Self::CorrespondingRuntimeTraces { .. } => {
                ExplanationQuestion::WhichRuntimeTracesCorrespond
            }
            Self::Invalidators { .. } => ExplanationQuestion::WhatInvalidates,
            Self::RelatedProjectionDisposition { .. } => {
                ExplanationQuestion::WhyWasRelatedProjectionNotGenerated
            }
            Self::Repairs { .. } => ExplanationQuestion::WhatRepairsARefusal,
        }
    }
}

/// One answered question: the typed answer, the question it answers, and one
/// bounded rendering of it for a person.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionExplanation {
    question: ExplanationQuestion,
    answer: ExplanationAnswer,
    human: HumanProjection<HumanTextLimit>,
}

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

/// The explanation-coverage refusal family body.
///
/// Independent members: several questions may be unanswered while another is
/// doubled, and reporting one of them would leave a caller repairing the view
/// one question per attempt.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExplanationCoverage {
    /// The established issues — at least one, at most the declared bound.
    pub issues: NonEmptyBounded<ExplanationCoverageIssue, ExplanationIssueLimit>,
    /// Whether every applicable question was examined.
    pub posture: CompletionPosture,
}

impl RefusalFamily for ExplanationCoverage {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}

impl ExplanationCoverage {
    /// The body a coverage check refuses with. When the issues outrun the
    /// declared bound the body keeps the first and reports that examination
    /// stopped there.
    #[must_use]
    fn established(first: ExplanationCoverageIssue, rest: Vec<ExplanationCoverageIssue>) -> Self {
        match NonEmptyBounded::admitted_const(first, rest) {
            Ok(issues) => Self {
                issues,
                posture: CompletionPosture::Complete,
            },
            Err(NonEmptyBoundedConstruction::OverLimit) => Self {
                issues: NonEmptyBounded::singleton(first),
                posture: CompletionPosture::EarlyStopped {
                    stopped_at: StopBound::DeclaredIssueBound,
                },
            },
        }
    }
}

/// A complete explanation view over one kind's plans.
///
/// Holding one is the proof: every applicable question has exactly one answer,
/// and no question outside the kind's roster was answered. There is no partial
/// view — a view that could not be completed is a refusal instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionExplanationView<K: ProjectionKind> {
    answers: Bounded<ProjectionExplanation, ExplanationSeatLimit>,
    _kind: PhantomData<K>,
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
        let applicable = ProjectionPlan::<K>::applicable_questions();
        let mut issues: Vec<ExplanationCoverageIssue> = Vec::new();
        for question in &applicable {
            let count = answers
                .iter()
                .filter(|answer| answer.question() == *question)
                .count();
            if count == 0 {
                issues.push(ExplanationCoverageIssue::QuestionUnanswered(*question));
            } else if count > 1 {
                issues.push(ExplanationCoverageIssue::QuestionAnsweredTwice(*question));
            }
        }
        for answer in &answers {
            if !applicable.contains(&answer.question()) {
                issues.push(ExplanationCoverageIssue::QuestionNotApplicableToKind(
                    answer.question(),
                ));
            }
        }
        let mut established = issues.into_iter();
        if let Some(first) = established.next() {
            return Err(ExplanationCoverage::established(
                first,
                established.collect(),
            ));
        }
        let observed = answers.len();
        Bounded::admitted_const(answers)
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

/// Whether one kind admits one question.
#[must_use]
pub fn kind_admits<K: ProjectionKind>(question: ExplanationQuestion) -> QuestionApplicability {
    if ProjectionPlan::<K>::applicable_questions().contains(&question) {
        QuestionApplicability::Applicable
    } else {
        QuestionApplicability::NotApplicableToKind
    }
}
