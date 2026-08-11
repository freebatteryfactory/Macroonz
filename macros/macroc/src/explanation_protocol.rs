//! The explanation protocol: the fourteen questions every generated thing must
//! be able to answer, and the typed answers that carry them.
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
    AssumptionLimit, ExactIdentity, ExplanationIssueLimit, ExplanationSeatLimit,
    GeneratedUnitSubject, HumanProjection, HumanTextLimit, MembershipLimit, OwnerFactRef,
    PatternInstanceSubject, PatternSubject, ProfileVersion, ProjectionKindSubject,
    ProjectionProfileSubject, RepairLimit, RuntimeTraceSubject, TraceEntryLimit,
};
use crate::planning::{
    InvalidationSet, OutputIdentity, ProjectionDisposition, ProjectionKind, ProjectionPlan,
    SourceDeclarations,
};
use core::marker::PhantomData;
use threadpak::declaration::types::LinkedGraphDomain;
use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
use threadpak::types::{Bounded, ConstLimit, NonEmptyBounded, NonEmptyBoundedConstruction};

/// The fourteen questions. A generated thing that cannot answer one of these is
/// a generated thing nobody can hold to account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExplanationQuestion {
    /// What are you?
    WhatAreYou,
    /// Which owner required you?
    WhichOwnerRequired,
    /// Which declaration caused you?
    WhichDeclarationCaused,
    /// Which template or pattern instance produced you?
    WhichTemplateOrPatternInstance,
    /// Which graph and profile were you decided under?
    WhichGraphAndProfile,
    /// Which capabilities selected your wrappers?
    WhichCapabilitiesSelectedWrappers,
    /// Which assumptions and specializations do you rest on?
    WhichAssumptionsAndSpecializations,
    /// Which output identity and digest are you?
    WhichOutputIdentityAndDigest,
    /// Which tests challenge you?
    WhichTestsChallenge,
    /// Which benchmarks measure you?
    WhichBenchmarksMeasure,
    /// Which runtime traces correspond to you?
    WhichRuntimeTracesCorrespond,
    /// What invalidates you?
    WhatInvalidates,
    /// Why was a related projection not generated?
    WhyWasRelatedProjectionNotGenerated,
    /// What repairs a refusal?
    WhatRepairsARefusal,
}

/// The declared question roster, in the order the protocol states it.
pub const EXPLANATION_QUESTIONS: [ExplanationQuestion; 14] = [
    ExplanationQuestion::WhatAreYou,
    ExplanationQuestion::WhichOwnerRequired,
    ExplanationQuestion::WhichDeclarationCaused,
    ExplanationQuestion::WhichTemplateOrPatternInstance,
    ExplanationQuestion::WhichGraphAndProfile,
    ExplanationQuestion::WhichCapabilitiesSelectedWrappers,
    ExplanationQuestion::WhichAssumptionsAndSpecializations,
    ExplanationQuestion::WhichOutputIdentityAndDigest,
    ExplanationQuestion::WhichTestsChallenge,
    ExplanationQuestion::WhichBenchmarksMeasure,
    ExplanationQuestion::WhichRuntimeTracesCorrespond,
    ExplanationQuestion::WhatInvalidates,
    ExplanationQuestion::WhyWasRelatedProjectionNotGenerated,
    ExplanationQuestion::WhatRepairsARefusal,
];

/// Whether one kind's plans admit one question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuestionApplicability {
    /// The kind's plans answer this question.
    Applicable,
    /// The kind's plans do not admit this question at all.
    NotApplicableToKind,
}

/// One typed answer. Each variant carries the exact values that answer its
/// question — identities, typed rosters, and typed dispositions, never a
/// sentence standing in for a fact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplanationAnswer {
    /// What are you: the projection kind.
    Kind {
        /// The kind's identity.
        kind: ExactIdentity<ProjectionKindSubject>,
    },
    /// Which owner required you.
    Owner {
        /// The requiring owner fact.
        owner: OwnerFactRef,
    },
    /// Which declarations caused you.
    CausingDeclarations {
        /// The cause set.
        sources: SourceDeclarations,
    },
    /// Which pattern instance produced you.
    PatternInstance {
        /// The authored pattern.
        pattern: ExactIdentity<PatternSubject>,
        /// This instantiation of it.
        instance: ExactIdentity<PatternInstanceSubject>,
    },
    /// Which graph and profile you were decided under.
    GraphAndProfile {
        /// The closed graph.
        graph: ExactIdentity<LinkedGraphDomain>,
        /// The profile.
        profile: ExactIdentity<ProjectionProfileSubject>,
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
    OutputAndDigest {
        /// The output.
        output: OutputIdentity,
    },
    /// Which tests challenge you.
    ChallengingTests {
        /// The test descriptors.
        descriptors: Bounded<ExactIdentity<GeneratedUnitSubject>, MembershipLimit>,
    },
    /// Which benchmarks measure you.
    MeasuringBenchmarks {
        /// The benchmark descriptors.
        descriptors: Bounded<ExactIdentity<GeneratedUnitSubject>, MembershipLimit>,
    },
    /// Which runtime traces correspond to you.
    CorrespondingRuntimeTraces {
        /// The corresponding traces.
        traces: Bounded<ExactIdentity<RuntimeTraceSubject>, TraceEntryLimit>,
    },
    /// What invalidates you.
    Invalidators {
        /// The watch set.
        triggers: InvalidationSet,
    },
    /// Why a related projection was not generated.
    RelatedProjectionDisposition {
        /// The related kind.
        related: ExactIdentity<ProjectionKindSubject>,
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

#[cfg(test)]
mod laws {
    use super::{
        EXPLANATION_QUESTIONS, ExplanationAnswer, ExplanationCoverageIssue, ExplanationQuestion,
        ProjectionExplanation, ProjectionExplanationView, QuestionApplicability, kind_admits,
    };
    use crate::origin_graph::{
        DecisionTrace, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
    };
    use crate::plane::{
        ExactIdentity, HumanProjection, HumanTextLimit, OwnerFactRef, ProfileVersion,
    };
    use crate::planning::{
        DeriveImplProjection, HostWrapperProjection, InvalidationTrigger, OutputIdentity,
        ProjectionContext, ProjectionDisposition,
    };
    use threadpak::types::Bounded;

    /// The closed question roster, proven closed by an exhaustive match.
    const fn question_index(question: ExplanationQuestion) -> usize {
        match question {
            ExplanationQuestion::WhatAreYou => 0,
            ExplanationQuestion::WhichOwnerRequired => 1,
            ExplanationQuestion::WhichDeclarationCaused => 2,
            ExplanationQuestion::WhichTemplateOrPatternInstance => 3,
            ExplanationQuestion::WhichGraphAndProfile => 4,
            ExplanationQuestion::WhichCapabilitiesSelectedWrappers => 5,
            ExplanationQuestion::WhichAssumptionsAndSpecializations => 6,
            ExplanationQuestion::WhichOutputIdentityAndDigest => 7,
            ExplanationQuestion::WhichTestsChallenge => 8,
            ExplanationQuestion::WhichBenchmarksMeasure => 9,
            ExplanationQuestion::WhichRuntimeTracesCorrespond => 10,
            ExplanationQuestion::WhatInvalidates => 11,
            ExplanationQuestion::WhyWasRelatedProjectionNotGenerated => 12,
            ExplanationQuestion::WhatRepairsARefusal => 13,
        }
    }

    /// One owner fact.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([60; 32]),
            fact: ExactIdentity::decoded([61; 32]),
        }
    }

    /// One rendering, for laws that need a human projection. The empty
    /// rendering is total, so this helper needs no panic road.
    fn human() -> HumanProjection<HumanTextLimit> {
        HumanProjection::projected("derived from the declared contract")
            .unwrap_or_else(|_| HumanProjection::empty())
    }

    /// The eight universal answers every kind owes.
    fn universal_answers() -> Vec<ProjectionExplanation> {
        let trail = OriginTrail::from_edge(OriginEdge {
            from: ExactIdentity::decoded([62; 32]),
            relation: OriginRelation::Rendering,
            to: ExactIdentity::decoded([63; 32]),
        });
        vec![
            ProjectionExplanation::answered(
                ExplanationAnswer::Kind {
                    kind: ExactIdentity::decoded([64; 32]),
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::Owner {
                    owner: owner_fact(),
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::CausingDeclarations {
                    sources: ProjectionContext::one_source(ExactIdentity::decoded([65; 32])),
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::GraphAndProfile {
                    graph: ExactIdentity::decoded([66; 32]),
                    profile: ExactIdentity::decoded([67; 32]),
                    version: ProfileVersion::declared(2),
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::OutputAndDigest {
                    output: OutputIdentity {
                        unit: ExactIdentity::decoded([68; 32]),
                        digest: ExactIdentity::decoded([69; 32]),
                        origin: trail,
                    },
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::Invalidators {
                    triggers: InvalidationTrigger::one_watched(
                        InvalidationTrigger::GraphIdentityChanged {
                            watched: ExactIdentity::decoded([66; 32]),
                        },
                    ),
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::RelatedProjectionDisposition {
                    related: ExactIdentity::decoded([70; 32]),
                    disposition: ProjectionDisposition::NotRequested,
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::Repairs {
                    repairs: Bounded::empty(),
                },
                human(),
            ),
        ]
    }

    /// law: explanation.questions-are-fourteen-and-closed — the protocol's
    /// roster is closed at fourteen, each distinct, in one declared order.
    /// Owed reversal: adding a question without placing it must break this law.
    #[test]
    fn questions_are_fourteen_and_closed() {
        assert_eq!(EXPLANATION_QUESTIONS.len(), 14);
        let indexes: Vec<usize> = EXPLANATION_QUESTIONS
            .iter()
            .copied()
            .map(question_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: explanation.an-answer-names-its-own-question — the pairing is
    /// derived from the answer, so filing a true answer under the wrong question
    /// is unrepresentable, and every question has an answer variant.
    /// Owed reversal (red twin): a constructor taking the question from the
    /// caller must break this law.
    #[test]
    fn an_answer_names_its_own_question() {
        let explanation = ProjectionExplanation::answered(
            ExplanationAnswer::Owner {
                owner: owner_fact(),
            },
            human(),
        );
        assert!(matches!(
            explanation.question(),
            ExplanationQuestion::WhichOwnerRequired
        ));
        let answers: Vec<ExplanationQuestion> = universal_answers()
            .iter()
            .map(ProjectionExplanation::question)
            .collect();
        assert_eq!(answers.len(), 8);
        assert!(answers.iter().enumerate().all(|(position, question)| {
            answers
                .iter()
                .skip(position.saturating_add(1))
                .all(|other| other != question)
        }));
    }

    /// law: explanation.a-complete-view-fills-every-applicable-seat — a view
    /// completes exactly when every applicable question has one answer.
    /// Owed reversal: a view accepting a subset must break this law.
    #[test]
    fn a_complete_view_fills_every_applicable_seat() {
        let mut answers = universal_answers();
        answers.push(ProjectionExplanation::answered(
            ExplanationAnswer::AssumptionsAndSpecializations {
                assumptions: Bounded::empty(),
            },
            human(),
        ));
        let view = ProjectionExplanationView::<DeriveImplProjection>::complete(answers);
        assert!(view.is_ok_and(|view| view.len() == 9 && !view.is_empty()));
    }

    /// law: explanation.an-incomplete-view-names-every-missing-seat — a view
    /// missing seats refuses and reports all of them at once, never one per
    /// attempt.
    /// Owed reversal: reporting only the first unanswered question must break
    /// this law.
    #[test]
    fn an_incomplete_view_names_every_missing_seat() {
        let refused = ProjectionExplanationView::<HostWrapperProjection>::complete(Vec::new());
        assert!(refused.is_err_and(|coverage| {
            coverage.issues.len() == 10
                && matches!(
                    coverage.issues.first(),
                    ExplanationCoverageIssue::QuestionUnanswered(ExplanationQuestion::WhatAreYou)
                )
        }));
    }

    /// law: explanation.a-doubled-or-foreign-seat-refuses — answering one
    /// question twice, or answering a question the kind does not admit, each
    /// refuses under its own issue.
    /// Owed reversal: silently keeping the last answer must break this law.
    #[test]
    fn a_doubled_or_foreign_seat_refuses() {
        let mut doubled = universal_answers();
        doubled.push(ProjectionExplanation::answered(
            ExplanationAnswer::AssumptionsAndSpecializations {
                assumptions: Bounded::empty(),
            },
            human(),
        ));
        doubled.push(ProjectionExplanation::answered(
            ExplanationAnswer::Owner {
                owner: owner_fact(),
            },
            human(),
        ));
        let refused = ProjectionExplanationView::<DeriveImplProjection>::complete(doubled);
        assert!(refused.is_err_and(|coverage| matches!(
            coverage.issues.first(),
            ExplanationCoverageIssue::QuestionAnsweredTwice(
                ExplanationQuestion::WhichOwnerRequired
            )
        )));

        let mut foreign = universal_answers();
        foreign.push(ProjectionExplanation::answered(
            ExplanationAnswer::AssumptionsAndSpecializations {
                assumptions: Bounded::empty(),
            },
            human(),
        ));
        foreign.push(ProjectionExplanation::answered(
            ExplanationAnswer::SelectedWrappers {
                trace: DecisionTrace::from_entry(TraceEntry {
                    subject: ExactIdentity::decoded([71; 32]),
                    decision: TraceDecision::SelectedBecause(owner_fact()),
                }),
            },
            human(),
        ));
        let rejected = ProjectionExplanationView::<DeriveImplProjection>::complete(foreign);
        assert!(rejected.is_err_and(|coverage| matches!(
            coverage.issues.first(),
            ExplanationCoverageIssue::QuestionNotApplicableToKind(
                ExplanationQuestion::WhichCapabilitiesSelectedWrappers
            )
        )));
    }

    /// law: explanation.applicability-is-answered-typed — whether a kind admits
    /// a question is a typed answer, not a bare boolean the caller reinterprets.
    /// Owed reversal: returning a boolean must break this law.
    #[test]
    fn applicability_is_answered_typed() {
        assert!(matches!(
            kind_admits::<HostWrapperProjection>(
                ExplanationQuestion::WhichCapabilitiesSelectedWrappers
            ),
            QuestionApplicability::Applicable
        ));
        assert!(matches!(
            kind_admits::<DeriveImplProjection>(
                ExplanationQuestion::WhichCapabilitiesSelectedWrappers
            ),
            QuestionApplicability::NotApplicableToKind
        ));
    }
}
