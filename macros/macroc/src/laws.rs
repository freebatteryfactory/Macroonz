//! The one compile-time proof surface for the metaprogramming services,
//! sectioned by module. Green laws only, in the module's declaration order, so
//! this file reads down the dependency line exactly as `lib.rs` declares it.
//!
//! A law that cannot fail is not a law: these compile (and trivially run) only
//! while the shapes hold; reversing the shape breaks the named law. Each law's
//! doc line states the reversal it is owed, and the reversal lands in testpak.
//!
//! The proof surface reaches every module and is reached by none. It is the
//! only module in this crate that may look in every direction, which is why it
//! is declared last, is not public, and is excluded from the
//! `tooling-module-order` check by that non-public declaration.

mod plane {
    use crate::plane::{
        ExactIdentity, HumanProjection, HumanTextLimit, OwnerFactSubject, OwnerHomeSubject,
        ProfileVersion, RefusalReason,
    };
    use threadpak::types::{BoundedConstruction, ConstLimit};

    /// law: plane.subjects-do-not-unify — a reference naming one subject is a
    /// different type than a reference naming another, whatever the bytes.
    /// Owed reversal: erasing the subject parameter must break this law.
    #[test]
    fn subjects_do_not_unify() {
        let home: fn(ExactIdentity<OwnerHomeSubject>) = drop;
        let fact: fn(ExactIdentity<OwnerFactSubject>) = drop;
        assert!((home as usize) != 0 && (fact as usize) != 0);
        let same_bytes_different_subject = ExactIdentity::<OwnerHomeSubject>::decoded([3; 32]);
        assert_eq!(same_bytes_different_subject.as_bytes(), &[3_u8; 32]);
    }

    /// law: plane.reason-projection-preserves-bytes — projecting a registered
    /// reason adapts nothing; a projection may change presentation, never
    /// identity.
    /// Owed reversal: a projection that rewrote the bytes must break this law.
    #[test]
    fn reason_projection_preserves_bytes() {
        let declared = ExactIdentity::<RefusalReason>::decoded([9; 32]);
        assert_eq!(declared.as_bytes(), &[9_u8; 32]);
    }

    /// law: plane.human-projections-are-bounded — a rendering that does not fit
    /// its declared bound refuses rather than truncating.
    /// Owed reversal (red twin): a constructor that truncated must break this
    /// law.
    #[test]
    fn human_projections_are_bounded() {
        let fits = HumanProjection::<HumanTextLimit>::projected("the owner declared this repair");
        assert!(fits.is_ok_and(|projection| !projection.is_empty() && projection.len() == 30));
        let oversized = "x".repeat(HumanTextLimit::MAX.saturating_add(1));
        let refused = HumanProjection::<HumanTextLimit>::projected(&oversized);
        assert!(matches!(refused, Err(BoundedConstruction::OverLimit)));
    }

    /// law: plane.profile-versions-are-not-ranked — a profile version carries a
    /// position and admits no ordering operator across profiles.
    /// Owed reversal (red twin): deriving `Ord` and comparing two versions must
    /// not compile.
    #[test]
    fn profile_versions_are_not_ranked() {
        let first = ProfileVersion::declared(1);
        let second = ProfileVersion::declared(2);
        assert_ne!(first, second);
        assert_eq!(second.position(), 2);
    }
}

mod refusal {
    use crate::plane::{ExactIdentity, PlanningIssueLimit};
    use crate::refusal::{
        BOUND_AXES, BoundAxis, PlanSeat, ProjectionPlanning, ProjectionPlanningIssue,
    };
    use threadpak::refusal::{CompletionPosture, FamilyShape, RefusalFamily, StopBound};
    use threadpak::types::ConstLimit;

    /// The closed bound-axis roster, proven closed by an exhaustive match: a new
    /// axis stops compiling here until it is placed.
    const fn axis_index(axis: BoundAxis) -> usize {
        match axis {
            BoundAxis::Declarations => 0,
            BoundAxis::Outputs => 1,
            BoundAxis::TraceEntries => 2,
            BoundAxis::Diagnostics => 3,
            BoundAxis::OriginEdges => 4,
            BoundAxis::Bytes => 5,
        }
    }

    /// law: refusal.bound-axes-are-six-and-closed — the plan's declared
    /// magnitudes are a closed roster, each distinct.
    /// Owed reversal: adding an axis without placing it must break this law.
    #[test]
    fn bound_axes_are_six_and_closed() {
        assert_eq!(BOUND_AXES.len(), 6);
        let indexes: Vec<usize> = BOUND_AXES.iter().copied().map(axis_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: refusal.family-is-an-issue-collection — the planning family declares
    /// the collection shape and elects no primary issue, so its selection order
    /// is empty by law rather than by omission.
    /// Owed reversal (red twin): declaring `SingleCause` with a non-empty
    /// collection body must break this law.
    #[test]
    fn family_is_an_issue_collection() {
        assert!(matches!(
            ProjectionPlanning::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(ProjectionPlanning::SELECTION_ORDER.is_empty());
    }

    /// law: refusal.one-issue-body-is-total — a seam that establishes one issue
    /// builds its refusal without an error road of its own, so refusing is never
    /// the place a caller reaches for a panic.
    /// Owed reversal: a fallible one-issue road must break this law.
    #[test]
    fn one_issue_body_is_total() {
        let refusal = ProjectionPlanning::established(ProjectionPlanningIssue::MissingOwnerFact {
            seat: PlanSeat::TargetBinding,
        });
        assert_eq!(refusal.issues.len(), 1);
        assert!(matches!(refusal.posture, CompletionPosture::Complete));
        assert!(matches!(
            refusal.issues.first(),
            ProjectionPlanningIssue::MissingOwnerFact {
                seat: PlanSeat::TargetBinding
            }
        ));
    }

    /// law: refusal.co-established-issues-stay-whole-or-say-they-stopped — a
    /// body carrying several issues either covers them all or reports the
    /// declared bound that stopped it.
    /// Owed reversal (red twin): a body that dropped the remainder silently must
    /// break this law.
    #[test]
    fn co_established_issues_stay_whole_or_say_they_stopped() {
        let node = ExactIdentity::decoded([1; 32]);
        let whole = ProjectionPlanning::co_established(
            ProjectionPlanningIssue::OrphanGeneratedNode { node },
            vec![ProjectionPlanningIssue::MembershipIncomplete { absent: node }],
        );
        assert_eq!(whole.issues.len(), 2);
        assert!(matches!(whole.posture, CompletionPosture::Complete));

        let overrun: Vec<ProjectionPlanningIssue> = core::iter::repeat_n(
            ProjectionPlanningIssue::OrphanGeneratedNode { node },
            PlanningIssueLimit::MAX,
        )
        .collect();
        let stopped = ProjectionPlanning::co_established(
            ProjectionPlanningIssue::MembershipIncomplete { absent: node },
            overrun,
        );
        assert_eq!(stopped.issues.len(), 1);
        assert!(matches!(
            stopped.posture,
            CompletionPosture::EarlyStopped {
                stopped_at: StopBound::DeclaredIssueBound
            }
        ));
    }

    /// law: refusal.bound-refusals-name-their-magnitude — a bound refusal states
    /// the axis, the declared bound, and the observed count.
    /// Owed reversal: a payload-free bound cause must break this law.
    #[test]
    fn bound_refusals_name_their_magnitude() {
        let refusal = ProjectionPlanning::bound_exceeded(BoundAxis::Outputs, 32, 33);
        assert!(matches!(
            refusal.issues.first(),
            ProjectionPlanningIssue::BoundExceeded {
                axis: BoundAxis::Outputs,
                bound: 32,
                observed: 33
            }
        ));
    }
}

mod diagnostics {
    use crate::diagnostics::{
        MACROC_PHASES, MacrocDiagnostic, MacrocPhase, ObservedClassification, ReleasePosture,
        RepairAction, ReproductionRoute,
    };
    use crate::plane::{ExactIdentity, HumanProjection, OwnerFactRef};
    use threadpak::declaration::{CoordinateRole, SourceCoordinate};
    use threadpak::evidence::CauseDisposition;
    use threadpak::types::Bounded;

    /// The closed phase roster, proven closed by an exhaustive match.
    const fn phase_index(phase: MacrocPhase) -> usize {
        match phase {
            MacrocPhase::Capture => 0,
            MacrocPhase::DeclarationConstruction => 1,
            MacrocPhase::Linking => 2,
            MacrocPhase::Planning => 3,
            MacrocPhase::Rendering => 4,
            MacrocPhase::Inspection => 5,
        }
    }

    /// law: diagnostics.phases-are-six-and-closed — the acts the services run
    /// are a closed roster in one declared order.
    /// Owed reversal: adding a phase without placing it must break this law.
    #[test]
    fn phases_are_six_and_closed() {
        assert_eq!(MACROC_PHASES.len(), 6);
        let indexes: Vec<usize> = MACROC_PHASES.iter().copied().map(phase_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: diagnostics.a-diagnostic-constructs-complete — every seat is
    /// furnished, including the reason, the family, the phase, the typed
    /// coordinate, the three identities, the expected contract, the observed
    /// classification, the cause posture, the repairs, the reproduction route,
    /// and the release posture.
    /// Owed reversal (red twin): omitting any seat must not compile.
    #[test]
    fn a_diagnostic_constructs_complete() {
        let declared_by = OwnerFactRef {
            home: ExactIdentity::decoded([40; 32]),
            fact: ExactIdentity::decoded([41; 32]),
        };
        let description = HumanProjection::projected("bind the declared host contract");
        let repairs = description.map_err(|_| ()).and_then(|description| {
            Bounded::admitted_const(vec![RepairAction {
                declared_by,
                description,
            }])
            .map_err(|_| ())
        });
        let built = repairs.map(|repairs| MacrocDiagnostic {
            reason: ExactIdentity::decoded([42; 32]),
            family: ExactIdentity::decoded([43; 32]),
            phase: MacrocPhase::Planning,
            coordinate: SourceCoordinate {
                role: CoordinateRole::SemanticOrigin,
                position: 17,
            },
            declaration: ExactIdentity::decoded([44; 32]),
            fragment: ExactIdentity::decoded([45; 32]),
            graph: ExactIdentity::decoded([46; 32]),
            expected: ExactIdentity::decoded([47; 32]),
            observed: ObservedClassification::SeatAbsent,
            cause: CauseDisposition::UnresolvedCause,
            related: Bounded::empty(),
            repairs,
            reproduction: ReproductionRoute::CallableServices {
                entry: ExactIdentity::decoded([48; 32]),
            },
            release: ReleasePosture::NoReleasePromise,
        });
        assert!(built.is_ok_and(|diagnostic| {
            diagnostic.repairs.len() == 1
                && diagnostic.related.is_empty()
                && diagnostic.coordinate.position == 17
                && matches!(diagnostic.cause, CauseDisposition::UnresolvedCause)
                && matches!(diagnostic.phase, MacrocPhase::Planning)
        }));
    }

    /// law: diagnostics.repairs-cite-their-owner — a repair carries the owner
    /// fact that declares it, so no rendering can present composed advice as
    /// declared authority.
    /// Owed reversal: a repair whose only member is text must break this law.
    #[test]
    fn repairs_cite_their_owner() {
        let declared_by = OwnerFactRef {
            home: ExactIdentity::decoded([49; 32]),
            fact: ExactIdentity::decoded([50; 32]),
        };
        let repair =
            HumanProjection::projected("declare the missing obligation").map(|description| {
                RepairAction {
                    declared_by,
                    description,
                }
            });
        assert!(repair.is_ok_and(|repair| repair.declared_by == declared_by));
    }

    /// law: diagnostics.reproduction-does-not-require-the-shell — the callable
    /// services are one reproduction route in their own right, so a diagnostic
    /// is reachable without a proc-macro anywhere in the picture.
    /// Owed reversal: a route roster with only the shell must break this law.
    #[test]
    fn reproduction_does_not_require_the_shell() {
        let route = ReproductionRoute::CallableServices {
            entry: ExactIdentity::decoded([51; 32]),
        };
        assert!(matches!(route, ReproductionRoute::CallableServices { .. }));
        let shell = ReproductionRoute::ExpansionShell {
            surface: ExactIdentity::decoded([52; 32]),
        };
        let fixture = ReproductionRoute::RecordedFixture {
            population: ExactIdentity::decoded([53; 32]),
        };
        assert_ne!(route, shell);
        assert_ne!(shell, fixture);
    }
}

mod question {
    use crate::question::{EXPLANATION_QUESTIONS, ExplanationQuestion};

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

    /// law: question.questions-are-fourteen-and-closed — the protocol's roster
    /// is closed at fourteen, each distinct, in one declared order.
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
}

mod origin_graph {
    use crate::origin_graph::{
        DecisionTrace, Nonclaim, ORIGIN_RELATIONS, OriginEdge, OriginRelation, OriginTrail,
        TraceDecision, TraceEntry,
    };
    use crate::plane::{ExactIdentity, OriginEdgeLimit, OwnerFactRef, TraceEntryLimit};
    use crate::refusal::{BoundAxis, ProjectionPlanningIssue};
    use threadpak::types::ConstLimit;

    /// The closed relation roster, proven closed by an exhaustive match.
    const fn relation_index(relation: OriginRelation) -> usize {
        match relation {
            OriginRelation::AuthoredDeclaration => 0,
            OriginRelation::PatternInstantiation => 1,
            OriginRelation::SemanticDerivation => 2,
            OriginRelation::FragmentConstruction => 3,
            OriginRelation::ExplicitLink => 4,
            OriginRelation::Normalization => 5,
            OriginRelation::ProfileSelection => 6,
            OriginRelation::ProjectionSelection => 7,
            OriginRelation::WrapperComposition => 8,
            OriginRelation::Rendering => 9,
            OriginRelation::HostBinding => 10,
            OriginRelation::TestDerivation => 11,
            OriginRelation::BenchmarkDerivation => 12,
            OriginRelation::DiagnosticDerivation => 13,
        }
    }

    /// One owner fact, for laws that need a citation.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([1; 32]),
            fact: ExactIdentity::decoded([2; 32]),
        }
    }

    /// One edge, for laws that need a trail.
    fn edge() -> OriginEdge {
        OriginEdge {
            from: ExactIdentity::decoded([3; 32]),
            relation: OriginRelation::AuthoredDeclaration,
            to: ExactIdentity::decoded([4; 32]),
        }
    }

    /// law: origin.relations-are-fourteen-and-closed — the ruled relation
    /// categories are a closed roster whose members are pairwise distinct and
    /// declared in one order.
    /// Owed reversal: adding a relation without placing it must break this law.
    #[test]
    fn relations_are_fourteen_and_closed() {
        assert_eq!(ORIGIN_RELATIONS.len(), 14);
        let indexes: Vec<usize> = ORIGIN_RELATIONS
            .iter()
            .copied()
            .map(relation_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: origin.a-generated-node-without-an-origin-is-unrepresentable — the
    /// trail seat is structurally non-empty, so the orphan case has no
    /// constructor to reach and no runtime check to pass.
    /// Owed reversal (red twin): a trail built from an empty edge list must not
    /// compile.
    #[test]
    fn a_generated_node_without_an_origin_is_unrepresentable() {
        let trail = OriginTrail::from_edge(edge());
        assert!(!trail.is_empty() && trail.len() == 1);
        assert!(matches!(
            trail.first().relation,
            OriginRelation::AuthoredDeclaration
        ));
    }

    /// law: origin.trails-refuse-rather-than-truncate — a walk past the declared
    /// bound refuses with the bound axis named, so an origin never quietly
    /// shortens into a span.
    /// Owed reversal: a constructor that truncated must break this law.
    #[test]
    fn trails_refuse_rather_than_truncate() {
        let overrun: Vec<OriginEdge> = core::iter::repeat_n(edge(), OriginEdgeLimit::MAX).collect();
        let refused = OriginTrail::drawn(edge(), overrun);
        assert!(refused.is_err_and(|planning| matches!(
            planning.issues.first(),
            ProjectionPlanningIssue::BoundExceeded {
                axis: BoundAxis::OriginEdges,
                ..
            }
        )));
        let fits = OriginTrail::drawn(edge(), vec![edge()]);
        assert!(fits.is_ok_and(|trail| trail.len() == 2));
    }

    /// law: origin.not-run-is-not-passed — a check that did not run is a
    /// distinct recorded decision, and a decision that ran cites the owner fact
    /// that caused it.
    /// Owed reversal (red twin): collapsing `NotRun` into an omission must break
    /// this law.
    #[test]
    fn not_run_is_not_passed() {
        let selected = TraceEntry {
            subject: ExactIdentity::decoded([5; 32]),
            decision: TraceDecision::SelectedBecause(owner_fact()),
        };
        let omitted = TraceEntry {
            subject: ExactIdentity::decoded([5; 32]),
            decision: TraceDecision::OmittedBecause(owner_fact()),
        };
        let not_run = TraceEntry {
            subject: ExactIdentity::decoded([5; 32]),
            decision: TraceDecision::NotRun,
        };
        assert_ne!(selected, omitted);
        assert_ne!(omitted, not_run);
        assert_ne!(selected, not_run);
    }

    /// law: origin.traces-keep-selection-order-and-a-declared-bound — the first
    /// entry recorded is the first entry held, and a trace past its bound
    /// refuses on the trace-entry axis.
    /// Owed reversal: a constructor that sorted entries must break this law.
    #[test]
    fn traces_keep_selection_order_and_a_declared_bound() {
        let first = TraceEntry {
            subject: ExactIdentity::decoded([6; 32]),
            decision: TraceDecision::NotRun,
        };
        let second = TraceEntry {
            subject: ExactIdentity::decoded([7; 32]),
            decision: TraceDecision::SelectedBecause(owner_fact()),
        };
        let recorded = DecisionTrace::recorded(first, vec![second]);
        assert!(recorded.is_ok_and(|trace| trace.len() == 2 && *trace.first() == first));

        let overrun: Vec<TraceEntry> = core::iter::repeat_n(second, TraceEntryLimit::MAX).collect();
        let refused = DecisionTrace::recorded(first, overrun);
        assert!(refused.is_err_and(|planning| matches!(
            planning.issues.first(),
            ProjectionPlanningIssue::BoundExceeded {
                axis: BoundAxis::TraceEntries,
                ..
            }
        )));
    }

    /// law: origin.nonclaims-cite-an-owner-fact — a stated nonclaim names the
    /// fact that leaves it unclaimed rather than standing as a bare disclaimer.
    /// Owed reversal: a nonclaim without a citation must break this law.
    #[test]
    fn nonclaims_cite_an_owner_fact() {
        let nonclaim = Nonclaim {
            unclaimed: ExactIdentity::decoded([8; 32]),
            because: owner_fact(),
        };
        assert_eq!(nonclaim.because, owner_fact());
    }
}

mod planning {
    use crate::origin_graph::{
        DecisionTrace, Nonclaim, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
    };
    use crate::plane::{ExactIdentity, OwnerFactRef, ProfileVersion};
    use crate::planning::{
        BenchmarkDescriptorProjection, CodecProjection, DeriveImplContent, DeriveImplProjection,
        DocumentationProjection, HostWrapperContent, HostWrapperProjection, InvalidationTrigger,
        OutputIdentity, PatternStampProjection, PlannedMembership, ProjectionBundlePlan,
        ProjectionContext, ProjectionDisposition, ProjectionKind, ProjectionPlan,
        RemoteSurfaceProjection, TargetBinding, TargetRequirement, TestDescriptorProjection,
        UNIVERSAL_QUESTIONS, WRAPPER_COMPONENTS, WrapperComponent,
    };
    use crate::question::{EXPLANATION_QUESTIONS, ExplanationQuestion};
    use crate::refusal::{PlanSeat, ProjectionPlanning, ProjectionPlanningIssue};
    use threadpak::types::{Bounded, NonEmptyBounded};

    /// One owner fact, for laws that need a citation.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([10; 32]),
            fact: ExactIdentity::decoded([11; 32]),
        }
    }

    /// One origin trail, for laws that need a generated unit.
    fn trail() -> OriginTrail {
        OriginTrail::from_edge(OriginEdge {
            from: ExactIdentity::decoded([12; 32]),
            relation: OriginRelation::SemanticDerivation,
            to: ExactIdentity::decoded([13; 32]),
        })
    }

    /// One declared output.
    fn output() -> OutputIdentity {
        OutputIdentity {
            unit: ExactIdentity::decoded([14; 32]),
            digest: ExactIdentity::decoded([15; 32]),
            origin: trail(),
        }
    }

    /// One shared context, under the binding the caller names.
    fn context(target: TargetBinding) -> ProjectionContext {
        ProjectionContext {
            graph: ExactIdentity::decoded([16; 32]),
            profile: ExactIdentity::decoded([17; 32]),
            profile_version: ProfileVersion::declared(3),
            sources: ProjectionContext::one_source(ExactIdentity::decoded([18; 32])),
            generator: ExactIdentity::decoded([19; 32]),
            target,
        }
    }

    /// The implementation-projection content, for the complete-plan law.
    fn derive_content() -> DeriveImplContent {
        DeriveImplContent {
            derived_type: ExactIdentity::decoded([20; 32]),
            contract: ExactIdentity::decoded([21; 32]),
            assumptions: Bounded::empty(),
        }
    }

    /// The trace the complete-plan law records.
    fn trace() -> DecisionTrace {
        DecisionTrace::from_entry(TraceEntry {
            subject: ExactIdentity::decoded([22; 32]),
            decision: TraceDecision::SelectedBecause(owner_fact()),
        })
    }

    /// law: planning.a-complete-plan-constructs-through-checked-seams — every
    /// seat is furnished through the plane's own seams, and the resulting plan
    /// carries its cause set, output set, watch set, trace, and trail.
    /// Owed reversal (red twin): omitting any seat must not compile.
    #[test]
    fn a_complete_plan_constructs_through_checked_seams() {
        let planned = ProjectionPlan::<DeriveImplProjection>::planned(
            context(TargetBinding::TargetFree),
            derive_content(),
            PlannedMembership::from_output(output()),
            InvalidationTrigger::one_watched(InvalidationTrigger::GraphIdentityChanged {
                watched: ExactIdentity::decoded([16; 32]),
            }),
            trace(),
            trail(),
            Bounded::empty(),
        );
        assert!(planned.is_ok_and(|plan| {
            plan.membership().len() == 1
                && !plan.membership().is_empty()
                && plan.invalidation().len() == 1
                && plan.trace().len() == 1
                && plan.origin().len() == 1
                && plan.nonclaims().is_empty()
                && plan.context().profile_version.position() == 3
                && !plan.membership().first().origin.is_empty()
        }));
    }

    /// law: planning.several-outputs-and-nonclaims-ride-the-same-plan — a plan
    /// may declare several outputs and state what it does not claim, and both
    /// bounded seats hold what was put in them.
    /// Owed reversal: a membership seam that dropped a sibling must break this
    /// law.
    #[test]
    fn several_outputs_and_nonclaims_ride_the_same_plan() {
        let nonclaims = Bounded::admitted_const(vec![Nonclaim {
            unclaimed: ExactIdentity::decoded([23; 32]),
            because: owner_fact(),
        }])
        .map_err(|_| ());
        let membership = PlannedMembership::declared(output(), vec![output()]).map_err(|_| ());
        let built = nonclaims.and_then(|nonclaims| {
            membership.and_then(|membership| {
                ProjectionPlan::<DeriveImplProjection>::planned(
                    context(TargetBinding::TargetFree),
                    derive_content(),
                    membership,
                    InvalidationTrigger::one_watched(
                        InvalidationTrigger::GeneratorVersionChanged {
                            watched: ExactIdentity::decoded([19; 32]),
                        },
                    ),
                    trace(),
                    trail(),
                    nonclaims,
                )
                .map_err(|_| ())
            })
        });
        assert!(
            built.is_ok_and(|plan| plan.membership().len() == 2 && plan.nonclaims().len() == 1)
        );
    }

    /// law: planning.a-declared-output-set-reads-back-whole — the membership
    /// seam holds every sibling put into it and hands them all back on a
    /// read-only pass: two distinct outputs go in, two distinct outputs come
    /// out, and the membership is unconsumed — the second read sees the same
    /// set as the first.
    ///
    /// The order law this read carries: the declared output set is
    /// order-insensitive, so nothing identity-bearing is derived from the order
    /// observed here; identity-bearing generation canonicalizes by an
    /// owner-declared order or key first. testpak owes the permutation hostile.
    ///
    /// Owed reversal: a membership seam that dropped or aliased a sibling must
    /// break this law.
    #[test]
    fn a_declared_output_set_reads_back_whole() {
        let sibling = OutputIdentity {
            unit: ExactIdentity::decoded([31; 32]),
            digest: ExactIdentity::decoded([32; 32]),
            origin: trail(),
        };
        let membership = PlannedMembership::declared(output(), vec![sibling]);
        assert!(membership.is_ok_and(|membership| {
            let units: Vec<[u8; 32]> = membership.iter().map(|out| *out.unit.as_bytes()).collect();
            units == vec![[14_u8; 32], [31_u8; 32]]
                && membership.iter().count() == 2
                && membership.len() == 2
                && !membership.is_empty()
        }));
    }

    /// law: planning.a-host-bound-kind-refuses-a-target-free-context — a kind
    /// whose plans are meaningless without a host contract refuses rather than
    /// defaulting to one, and names the seat.
    /// Owed reversal: defaulting the binding must break this law.
    #[test]
    fn a_host_bound_kind_refuses_a_target_free_context() {
        assert!(matches!(
            HostWrapperProjection::TARGET_REQUIREMENT,
            TargetRequirement::BoundHostContract
        ));
        let refused = ProjectionPlan::<HostWrapperProjection>::planned(
            context(TargetBinding::TargetFree),
            HostWrapperContent {
                host_contract: ExactIdentity::decoded([24; 32]),
                components: NonEmptyBounded::singleton(WrapperComponent::Admission),
                capability_basis: owner_fact(),
            },
            PlannedMembership::from_output(output()),
            InvalidationTrigger::one_watched(InvalidationTrigger::TargetContractChanged {
                watched: ExactIdentity::decoded([24; 32]),
            }),
            trace(),
            trail(),
            Bounded::empty(),
        );
        assert!(refused.is_err_and(|planning| matches!(
            planning.issues.first(),
            ProjectionPlanningIssue::MissingOwnerFact {
                seat: PlanSeat::TargetBinding
            }
        )));
    }

    /// The closed trigger roster, proven closed by an exhaustive match.
    const fn trigger_index(trigger: &InvalidationTrigger) -> usize {
        match trigger {
            InvalidationTrigger::SourceDeclarationChanged { .. } => 0,
            InvalidationTrigger::GraphIdentityChanged { .. } => 1,
            InvalidationTrigger::ProjectionProfileChanged { .. } => 2,
            InvalidationTrigger::TargetContractChanged { .. } => 3,
            InvalidationTrigger::GeneratorVersionChanged { .. } => 4,
            InvalidationTrigger::MechanismProfileChanged { .. } => 5,
            InvalidationTrigger::WorkFormulaChanged { .. } => 6,
            InvalidationTrigger::FixturePopulationChanged { .. } => 7,
        }
    }

    /// law: planning.invalidation-triggers-are-eight-and-each-watches-an-identity
    /// — the roster is closed at eight, its members are pairwise distinct, and
    /// each names the exact identity whose change invalidates.
    /// Owed reversal: a payload-free trigger must break this law.
    #[test]
    fn invalidation_triggers_are_eight_and_each_watches_an_identity() {
        let triggers = [
            InvalidationTrigger::SourceDeclarationChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::GraphIdentityChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::ProjectionProfileChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::TargetContractChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::GeneratorVersionChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::MechanismProfileChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::WorkFormulaChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
            InvalidationTrigger::FixturePopulationChanged {
                watched: ExactIdentity::decoded([25; 32]),
            },
        ];
        assert_eq!(triggers.len(), 8);
        let indexes: Vec<usize> = triggers.iter().map(trigger_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// The closed wrapper-component roster, proven closed by an exhaustive
    /// match: a new component stops compiling here until it is placed.
    const fn component_index(component: WrapperComponent) -> usize {
        match component {
            WrapperComponent::Admission => 0,
            WrapperComponent::Decode => 1,
            WrapperComponent::Encode => 2,
            WrapperComponent::Cancellation => 3,
            WrapperComponent::Receipt => 4,
            WrapperComponent::EffectDispatch => 5,
            WrapperComponent::Observation => 6,
            WrapperComponent::Explanation => 7,
        }
    }

    /// law: planning.wrapper-components-are-eight-and-closed — the components a
    /// host wrapper may compose are a closed roster in one declared order, and
    /// the roster is the denominator every exhaustive disposition is checked
    /// against.
    /// Owed reversal: adding a component without placing it must break this
    /// law.
    #[test]
    fn wrapper_components_are_eight_and_closed() {
        assert_eq!(WRAPPER_COMPONENTS.len(), 8);
        let indexes: Vec<usize> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .map(component_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// The closed disposition roster, proven closed by an exhaustive match.
    fn disposition_index(disposition: &ProjectionDisposition) -> usize {
        match disposition {
            ProjectionDisposition::Generated { .. } => 0,
            ProjectionDisposition::NotApplicable { .. } => 1,
            ProjectionDisposition::Refused { .. } => 2,
            ProjectionDisposition::UnavailableUnderProfile { .. } => 3,
            ProjectionDisposition::NotRequested => 4,
            ProjectionDisposition::ExcludedByConfiguration { .. } => 5,
        }
    }

    /// law: planning.every-absence-has-a-named-disposition — all six
    /// dispositions are constructible and pairwise distinct, and none of them
    /// is silence.
    /// Owed reversal: dropping a disposition must break this law.
    #[test]
    fn every_absence_has_a_named_disposition() {
        let dispositions = [
            ProjectionDisposition::Generated { output: output() },
            ProjectionDisposition::NotApplicable {
                because: owner_fact(),
            },
            ProjectionDisposition::Refused {
                refusal: ProjectionPlanning::established(
                    ProjectionPlanningIssue::MissingOwnerFact {
                        seat: PlanSeat::TargetBinding,
                    },
                ),
            },
            ProjectionDisposition::UnavailableUnderProfile {
                profile: ExactIdentity::decoded([26; 32]),
                version: ProfileVersion::declared(1),
            },
            ProjectionDisposition::NotRequested,
            ProjectionDisposition::ExcludedByConfiguration {
                configuration: ExactIdentity::decoded([27; 32]),
            },
        ];
        assert_eq!(dispositions.len(), 6);
        let indexes: Vec<usize> = dispositions.iter().map(disposition_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: planning.a-bundle-names-its-members-and-refuses-a-partial-set — a
    /// bundle holds at least one member by shape and refuses past its declared
    /// bound rather than publishing part of a set.
    /// Owed reversal (red twin): an empty bundle must not compile.
    #[test]
    fn a_bundle_names_its_members_and_refuses_a_partial_set() {
        let bundle = ProjectionBundlePlan::materialized(
            ExactIdentity::decoded([28; 32]),
            ExactIdentity::decoded([29; 32]),
            vec![ExactIdentity::decoded([30; 32])],
        );
        assert!(bundle.is_ok_and(|plan| plan.len() == 2 && !plan.is_empty()));
        let single = ProjectionBundlePlan::of_one(
            ExactIdentity::decoded([28; 32]),
            ExactIdentity::decoded([29; 32]),
        );
        assert_eq!(single.bundle(), ExactIdentity::decoded([28; 32]));
    }

    /// law: planning.no-kind-ducks-the-explanation-protocol — every kind names
    /// every universal question, states its own questions without repeating one,
    /// and the eight kinds together reach all fourteen questions.
    /// Owed reversal: a kind declaring an empty applicable set must break this
    /// law.
    #[test]
    fn no_kind_ducks_the_explanation_protocol() {
        let rosters: [Vec<ExplanationQuestion>; 8] = [
            ProjectionPlan::<CodecProjection>::applicable_questions(),
            ProjectionPlan::<HostWrapperProjection>::applicable_questions(),
            ProjectionPlan::<RemoteSurfaceProjection>::applicable_questions(),
            ProjectionPlan::<TestDescriptorProjection>::applicable_questions(),
            ProjectionPlan::<BenchmarkDescriptorProjection>::applicable_questions(),
            ProjectionPlan::<DocumentationProjection>::applicable_questions(),
            ProjectionPlan::<DeriveImplProjection>::applicable_questions(),
            ProjectionPlan::<PatternStampProjection>::applicable_questions(),
        ];
        for roster in &rosters {
            assert!(
                UNIVERSAL_QUESTIONS
                    .iter()
                    .all(|question| roster.contains(question))
            );
            assert!(roster.iter().enumerate().all(|(position, question)| {
                roster
                    .iter()
                    .skip(position.saturating_add(1))
                    .all(|other| other != question)
            }));
        }
        assert!(
            EXPLANATION_QUESTIONS
                .iter()
                .all(|question| rosters.iter().any(|roster| roster.contains(question)))
        );
    }
}

mod explanation_protocol {
    use crate::explanation_protocol::{
        ExplanationAnswer, ExplanationCoverageIssue, ProjectionExplanation,
        ProjectionExplanationView, kind_admits,
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
    use crate::question::{ExplanationQuestion, QuestionApplicability};
    use threadpak::types::Bounded;

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

mod template {
    use crate::origin_graph::Nonclaim;
    use crate::plane::{ExactIdentity, OwnerFactRef, ProfileVersion};
    use crate::template::{
        ApplicativeDistinctness, AxisCeiling, CheckedMeterPosture, DeclarationTemplate,
        ForbiddenKeyFact, INVOCATION_KEY_NEVER, META_BOUND_AXES, MetaBoundAxis, ProfileCeiling,
        SPLICE_CATEGORIES, SpliceCategory, SymbolicBoundFormula, TemplateApplication,
        TemplateArgument, TemplateBinding, TemplateBindingIssue, TemplateConstruction,
        TemplateConstructionIssue, TemplateInvocationKey, TemplateParameter, TemplateSeat,
        VersionedProfile,
    };
    use threadpak::declaration::Stage;
    use threadpak::refusal::{FamilyShape, RefusalFamily};
    use threadpak::types::{Bounded, NonEmptyBounded};

    /// The closed splice-category roster, proven closed by an exhaustive match:
    /// a new category stops compiling here until it is placed.
    const fn category_index(category: SpliceCategory) -> usize {
        match category {
            SpliceCategory::Expression => 0,
            SpliceCategory::Type => 1,
            SpliceCategory::Pattern => 2,
            SpliceCategory::Declaration => 3,
            SpliceCategory::Fragment => 4,
            SpliceCategory::IdentifierBinding => 5,
        }
    }

    /// The closed meta bound-axis roster, proven closed by an exhaustive match.
    const fn axis_index(axis: MetaBoundAxis) -> usize {
        match axis {
            MetaBoundAxis::InputDescriptors => 0,
            MetaBoundAxis::Work => 1,
            MetaBoundAxis::Memory => 2,
            MetaBoundAxis::Recursion => 3,
            MetaBoundAxis::Declarations => 4,
            MetaBoundAxis::Symbols => 5,
            MetaBoundAxis::Diagnostics => 6,
            MetaBoundAxis::OutputBytes => 7,
        }
    }

    /// The closed forbidden-fact roster, proven closed by an exhaustive match.
    const fn forbidden_index(fact: ForbiddenKeyFact) -> usize {
        match fact {
            ForbiddenKeyFact::CheckoutPath => 0,
            ForbiddenKeyFact::CurrentDirectory => 1,
            ForbiddenKeyFact::ModificationTime => 2,
            ForbiddenKeyFact::ProcessIdentity => 3,
            ForbiddenKeyFact::AmbientEnvironment => 4,
            ForbiddenKeyFact::WallTime => 5,
            ForbiddenKeyFact::Entropy => 6,
            ForbiddenKeyFact::HostAddress => 7,
            ForbiddenKeyFact::MapIterationOrder => 8,
        }
    }

    /// The closed template-seat roster, proven closed by an exhaustive match.
    const fn seat_index(seat: TemplateSeat) -> usize {
        match seat {
            TemplateSeat::DeclaredParameters => 0,
            TemplateSeat::SuppliedBindings => 1,
            TemplateSeat::AxisCeilings => 2,
        }
    }

    /// One owner fact, for laws that need a citation.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([80; 32]),
            fact: ExactIdentity::decoded([81; 32]),
        }
    }

    /// One declared hole under the category and identity byte the caller names.
    fn parameter(category: SpliceCategory, tag: u8) -> TemplateParameter {
        TemplateParameter {
            category,
            parameter: ExactIdentity::decoded([tag; 32]),
        }
    }

    /// One offered commitment under the category and identity byte named.
    fn argument(category: SpliceCategory, tag: u8) -> TemplateArgument {
        TemplateArgument {
            category,
            commitment: ExactIdentity::decoded([tag; 32]),
        }
    }

    /// The complete ceiling: every axis bounded exactly once.
    fn complete_ceiling() -> Result<ProfileCeiling, TemplateConstruction> {
        ProfileCeiling::declared(
            META_BOUND_AXES
                .iter()
                .copied()
                .map(|axis| AxisCeiling {
                    axis,
                    magnitude: 64,
                    declared_by: owner_fact(),
                })
                .collect(),
        )
    }

    /// The first lock, over one validated input.
    fn formula() -> SymbolicBoundFormula {
        SymbolicBoundFormula {
            formula: ExactIdentity::decoded([82; 32]),
            declared_by: owner_fact(),
            over_inputs: NonEmptyBounded::singleton(ExactIdentity::decoded([83; 32])),
        }
    }

    /// The third lock, as an obligation and a stated nonclaim.
    fn meter() -> CheckedMeterPosture {
        CheckedMeterPosture {
            obliged_by: owner_fact(),
            unmeasured: Nonclaim {
                unclaimed: ExactIdentity::decoded([84; 32]),
                because: owner_fact(),
            },
        }
    }

    /// One template over the holes the caller names.
    fn template(
        first: TemplateParameter,
        rest: Vec<TemplateParameter>,
    ) -> Result<DeclarationTemplate, TemplateConstruction> {
        complete_ceiling().and_then(|ceiling| {
            DeclarationTemplate::declared(
                ExactIdentity::decoded([85; 32]),
                first,
                rest,
                formula(),
                ceiling,
                meter(),
                Stage::Meta,
            )
        })
    }

    /// The language profile, at a declared version.
    fn language() -> VersionedProfile<crate::plane::LanguageProfileSubject> {
        VersionedProfile {
            profile: ExactIdentity::decoded([86; 32]),
            version: ProfileVersion::declared(4),
        }
    }

    /// The meta profile, at a declared version.
    fn meta() -> VersionedProfile<crate::plane::MetaProfileSubject> {
        VersionedProfile {
            profile: ExactIdentity::decoded([87; 32]),
            version: ProfileVersion::declared(5),
        }
    }

    /// law: template.splice-categories-are-six-and-closed — the hole categories
    /// are a closed roster whose members are pairwise distinct and declared in
    /// one order.
    /// Owed reversal: adding a category without placing it must break this law.
    #[test]
    fn splice_categories_are_six_and_closed() {
        assert_eq!(SPLICE_CATEGORIES.len(), 6);
        let indexes: Vec<usize> = SPLICE_CATEGORIES
            .iter()
            .copied()
            .map(category_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: template.a-binding-agrees-on-category-or-refuses — an argument
    /// enters a hole only when both ends name the same category, and the
    /// refusal names both categories rather than saying "wrong kind".
    /// Owed reversal (red twin): a constructor that coerced the argument's
    /// category must break this law.
    #[test]
    fn a_binding_agrees_on_category_or_refuses() {
        let bound = TemplateBinding::bound(
            parameter(SpliceCategory::IdentifierBinding, 1),
            argument(SpliceCategory::IdentifierBinding, 2),
        );
        assert!(bound.is_ok_and(|binding| {
            matches!(binding.category(), SpliceCategory::IdentifierBinding)
                && binding.argument().commitment == ExactIdentity::decoded([2; 32])
                && binding.parameter().parameter == ExactIdentity::decoded([1; 32])
        }));

        let refused = TemplateBinding::bound(
            parameter(SpliceCategory::IdentifierBinding, 1),
            argument(SpliceCategory::Expression, 2),
        );
        assert!(refused.is_err_and(|issue| matches!(
            issue,
            TemplateBindingIssue::CategoryMismatch {
                expected: SpliceCategory::IdentifierBinding,
                found: SpliceCategory::Expression
            }
        )));
    }

    /// law: template.the-two-families-declare-their-shapes — the binding seam
    /// runs one check and takes the single-cause shape with a declared
    /// selection order; the construction seam co-establishes and takes the
    /// collection shape, electing no primary issue.
    /// Owed reversal (red twin): swapping the two shapes must break this law.
    #[test]
    fn the_two_families_declare_their_shapes() {
        assert!(matches!(
            TemplateBindingIssue::SHAPE,
            FamilyShape::SingleCause
        ));
        assert_eq!(TemplateBindingIssue::SELECTION_ORDER, &["CategoryMismatch"]);
        assert!(matches!(
            TemplateConstruction::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(TemplateConstruction::SELECTION_ORDER.is_empty());
    }

    /// law: template.a-ceiling-covers-every-meta-bound-axis — the axis roster is
    /// closed at eight, a complete ceiling reads back one magnitude per axis,
    /// and a ceiling missing or doubling an axis refuses naming that axis.
    /// Owed reversal (red twin): a ceiling admitting a subset of the axes must
    /// break this law.
    #[test]
    fn a_ceiling_covers_every_meta_bound_axis() {
        assert_eq!(META_BOUND_AXES.len(), 8);
        let indexes: Vec<usize> = META_BOUND_AXES.iter().copied().map(axis_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );

        assert!(complete_ceiling().is_ok_and(|ceiling| {
            ceiling.len() == 8
                && !ceiling.is_empty()
                && ceiling.iter().count() == 8
                && ceiling
                    .iter()
                    .all(|held| held.magnitude == 64 && held.declared_by == owner_fact())
                && META_BOUND_AXES
                    .iter()
                    .all(|axis| ceiling.iter().any(|held| held.axis == *axis))
        }));

        let short = ProfileCeiling::declared(
            META_BOUND_AXES
                .iter()
                .copied()
                .filter(|axis| *axis != MetaBoundAxis::Memory)
                .map(|axis| AxisCeiling {
                    axis,
                    magnitude: 8,
                    declared_by: owner_fact(),
                })
                .collect(),
        );
        assert!(short.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::CeilingAxisAbsent {
                axis: MetaBoundAxis::Memory
            }
        )));

        let doubled = ProfileCeiling::declared(
            META_BOUND_AXES
                .iter()
                .copied()
                .chain(core::iter::once(MetaBoundAxis::Work))
                .map(|axis| AxisCeiling {
                    axis,
                    magnitude: 8,
                    declared_by: owner_fact(),
                })
                .collect(),
        );
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::CeilingAxisDoubled {
                axis: MetaBoundAxis::Work
            }
        )));
    }

    /// law: template.a-template-carries-its-three-locks-and-its-stage — a
    /// declared template holds the symbolic formula over validated inputs, the
    /// complete ceiling, the checked-meter obligation with its stated nonclaim,
    /// and the stage its owner declared; two holes under one identity refuse.
    /// Owed reversal (red twin): omitting any lock seat must not compile.
    #[test]
    fn a_template_carries_its_three_locks_and_its_stage() {
        let declared = template(
            parameter(SpliceCategory::Type, 10),
            vec![parameter(SpliceCategory::Expression, 11)],
        );
        assert!(declared.is_ok_and(|template| {
            template.arity() == 2
                && template.parameters().count() == 2
                && template.identity() == ExactIdentity::decoded([85; 32])
                && template.formula().over_inputs.len() == 1
                && template.formula().declared_by == owner_fact()
                && template.ceiling().len() == 8
                && template.meter().obliged_by == owner_fact()
                && template.meter().unmeasured.because == owner_fact()
                && matches!(template.stage(), Stage::Meta)
                && matches!(template.first_parameter().category, SpliceCategory::Type)
        }));

        let doubled = template(
            parameter(SpliceCategory::Type, 10),
            vec![parameter(SpliceCategory::Expression, 10)],
        );
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::DuplicateParameter { .. }
        )));
    }

    /// The two-hole template the application laws range over: a type hole at
    /// parameter identity 20 and an expression hole at parameter identity 21.
    fn two_hole_template() -> Result<DeclarationTemplate, TemplateConstruction> {
        template(
            parameter(SpliceCategory::Type, 20),
            vec![parameter(SpliceCategory::Expression, 21)],
        )
    }

    /// The bindings for the holes named, each built through the checked binding
    /// seam. A category disagreement at that seam yields no binding at all, so
    /// a law that expected one fails on the count it asserts rather than on a
    /// road this helper invented.
    fn bindings(named: &[(SpliceCategory, u8, u8)]) -> Vec<TemplateBinding> {
        named
            .iter()
            .filter_map(|(category, hole, commitment)| {
                TemplateBinding::bound(
                    parameter(*category, *hole),
                    argument(*category, *commitment),
                )
                .ok()
            })
            .collect()
    }

    /// Apply the two-hole template to the bindings supplied.
    fn apply(supplied: Vec<TemplateBinding>) -> Result<TemplateApplication, TemplateConstruction> {
        two_hole_template().and_then(|template| {
            TemplateApplication::applied(
                &template,
                supplied,
                language(),
                meta(),
                ApplicativeDistinctness::Applicative,
            )
        })
    }

    /// law: template.an-application-binds-every-hole-exactly-once — a complete
    /// application reads its bindings back whole under both profiles, an
    /// unbound hole refuses, and a doubly bound hole refuses.
    /// Owed reversal: an application seam that accepted a partial binding set
    /// must break this law.
    #[test]
    fn an_application_binds_every_hole_exactly_once() {
        let supplied = bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Expression, 21, 31),
        ]);
        assert_eq!(supplied.len(), 2);
        let applied = apply(supplied);
        assert!(applied.is_ok_and(|application| {
            application.arity() == 2
                && application.bindings().count() == 2
                && application.template() == ExactIdentity::decoded([85; 32])
                && application.language_profile().version.position() == 4
                && application.meta_profile().version.position() == 5
                && matches!(
                    application.distinctness(),
                    ApplicativeDistinctness::Applicative
                )
        }));

        let unbound = apply(bindings(&[(SpliceCategory::Type, 20, 30)]));
        assert!(unbound.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::MissingBinding { .. }
        )));

        let doubled = apply(bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Type, 20, 33),
            (SpliceCategory::Expression, 21, 31),
        ]));
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::DuplicateBinding { .. }
        )));
    }

    /// law: template.an-application-refuses-a-stranger-or-a-recategorized-hole —
    /// a binding naming a hole this template does not declare refuses, and a
    /// binding naming a declared hole under another category refuses naming both
    /// the declared category and the bound one.
    /// Owed reversal: an application seam that ignored an unknown binding, or
    /// one that trusted the binding's own category over the template's, must
    /// break this law.
    #[test]
    fn an_application_refuses_a_stranger_or_a_recategorized_hole() {
        let stranger = apply(bindings(&[
            (SpliceCategory::Type, 20, 30),
            (SpliceCategory::Expression, 21, 31),
            (SpliceCategory::Pattern, 99, 98),
        ]));
        assert!(stranger.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::UnknownParameter { .. }
        )));

        let recategorized = apply(bindings(&[
            (SpliceCategory::Pattern, 20, 30),
            (SpliceCategory::Expression, 21, 31),
        ]));
        assert!(recategorized.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::DeclaredCategoryDisagreement {
                declared: SpliceCategory::Type,
                bound: SpliceCategory::Pattern,
                ..
            }
        )));
    }

    /// law: template.deliberate-distinctness-is-identity-bearing — two
    /// applications of one template over the same bindings and profiles differ
    /// only when a distinctness identity says so; the applicative posture and a
    /// declared distinctness never read the same.
    /// Owed reversal (red twin): a boolean distinctness flag must break this
    /// law.
    #[test]
    fn deliberate_distinctness_is_identity_bearing() {
        let holes = template(parameter(SpliceCategory::Fragment, 40), Vec::new());
        assert!(holes.is_ok_and(|template| {
            let binding = TemplateBinding::bound(
                parameter(SpliceCategory::Fragment, 40),
                argument(SpliceCategory::Fragment, 41),
            );
            let pair = binding.map_err(|_| ()).and_then(|binding| {
                let applicative = TemplateApplication::applied(
                    &template,
                    vec![binding],
                    language(),
                    meta(),
                    ApplicativeDistinctness::Applicative,
                );
                let twin = TemplateApplication::applied(
                    &template,
                    vec![binding],
                    language(),
                    meta(),
                    ApplicativeDistinctness::Applicative,
                );
                let distinct = TemplateApplication::applied(
                    &template,
                    vec![binding],
                    language(),
                    meta(),
                    ApplicativeDistinctness::DeliberatelyDistinct(ExactIdentity::decoded([42; 32])),
                );
                applicative
                    .and_then(|applicative| {
                        twin.and_then(|twin| distinct.map(|distinct| (applicative, twin, distinct)))
                    })
                    .map_err(|_| ())
            });
            pair.is_ok_and(|(applicative, twin, distinct)| {
                applicative == twin && applicative != distinct
            })
        }));
    }

    /// law: template.the-invocation-key-names-seven-lawful-inputs — the key
    /// carries the template identity, the validated inputs, the source
    /// snapshot, the fragment dependencies, both profile versions, and the
    /// configuration commitment, and two keys differing only in a lawful input
    /// are different keys.
    /// Owed reversal: a key that dropped the configuration commitment must
    /// break this law.
    #[test]
    fn the_invocation_key_names_seven_lawful_inputs() {
        let key = TemplateInvocationKey {
            template: ExactIdentity::decoded([50; 32]),
            inputs: Bounded::empty(),
            source_snapshot: ExactIdentity::decoded([51; 32]),
            fragment_dependencies: Bounded::empty(),
            language_profile: language(),
            meta_profile: meta(),
            configuration: ExactIdentity::decoded([52; 32]),
        };
        let reconfigured = TemplateInvocationKey {
            configuration: ExactIdentity::decoded([53; 32]),
            ..key.clone()
        };
        assert_ne!(key, reconfigured);
        assert_eq!(key, key.clone());
        assert!(key.inputs.is_empty() && key.fragment_dependencies.is_empty());
        assert_eq!(key.language_profile.version.position(), 4);
        assert_eq!(key.meta_profile.version.position(), 5);
        assert_eq!(key.source_snapshot.as_bytes(), &[51_u8; 32]);
    }

    /// law: template.forbidden-key-facts-are-nine-and-closed — the never-roster
    /// is closed at nine, each member distinct, and none of them is a member of
    /// the key record.
    /// Owed reversal: adding a forbidden fact without placing it must break
    /// this law.
    #[test]
    fn forbidden_key_facts_are_nine_and_closed() {
        assert_eq!(INVOCATION_KEY_NEVER.len(), 9);
        let indexes: Vec<usize> = INVOCATION_KEY_NEVER
            .iter()
            .copied()
            .map(forbidden_index)
            .collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: template.seat-bounds-name-the-seat-that-overran — a bound refusal
    /// names which seat exceeded its magnitude, the declared bound, and the
    /// observed count, and the seat roster is closed.
    /// Owed reversal: a payload-free bound issue must break this law.
    #[test]
    fn seat_bounds_name_the_seat_that_overran() {
        let seats = [
            TemplateSeat::DeclaredParameters,
            TemplateSeat::SuppliedBindings,
            TemplateSeat::AxisCeilings,
        ];
        let indexes: Vec<usize> = seats.iter().copied().map(seat_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );

        let overrun: Vec<TemplateParameter> = (0..40_u8)
            .map(|tag| parameter(SpliceCategory::Expression, tag.saturating_add(100)))
            .collect();
        let refused = template(parameter(SpliceCategory::Expression, 99), overrun);
        assert!(refused.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            TemplateConstructionIssue::SeatBoundExceeded {
                seat: TemplateSeat::DeclaredParameters,
                bound: 32,
                observed: 41
            }
        )));
    }
}

mod trigger_view {
    use crate::plane::{ExactIdentity, OwnerFactRef};
    use crate::planning::{WRAPPER_COMPONENTS, WrapperComponent};
    use crate::trigger_view::{
        TriggerOmission, TriggerSelection, TriggerViewComposition, TriggerViewIssue,
        WrapperTriggerView,
    };
    use threadpak::refusal::{FamilyShape, RefusalFamily};
    use threadpak::types::NonEmptyBounded;

    /// One owner fact, for laws that need a citation.
    fn owner_fact(tag: u8) -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([tag; 32]),
            fact: ExactIdentity::decoded([tag.saturating_add(1); 32]),
        }
    }

    /// One selection of the named component, citing one owner fact.
    fn selection(component: WrapperComponent) -> TriggerSelection {
        TriggerSelection {
            component,
            because: NonEmptyBounded::singleton(owner_fact(90)),
        }
    }

    /// One omission of the named component, citing one owner fact.
    fn omission(component: WrapperComponent) -> TriggerOmission {
        TriggerOmission {
            component,
            because: NonEmptyBounded::singleton(owner_fact(92)),
        }
    }

    /// law: trigger.a-disposition-always-cites-an-owner-fact — a selection and
    /// an omission each carry at least one citation by shape, so a bare
    /// selection is unrepresentable rather than refused, and the citations read
    /// back whole.
    /// Owed reversal (red twin): a citation-free selection must not compile.
    #[test]
    fn a_disposition_always_cites_an_owner_fact() {
        let selected = selection(WrapperComponent::Admission);
        assert_eq!(selected.because.len(), 1);
        assert_eq!(*selected.because.first(), owner_fact(90));
        assert!(!selected.because.is_empty());

        let paired = TriggerSelection {
            component: WrapperComponent::Receipt,
            because: NonEmptyBounded::admitted_const(owner_fact(94), vec![owner_fact(96)])
                .unwrap_or_else(|_| NonEmptyBounded::singleton(owner_fact(94))),
        };
        assert_eq!(paired.because.iter().count(), 2);

        let left_out = omission(WrapperComponent::Explanation);
        assert_eq!(left_out.because.len(), 1);
        assert_eq!(*left_out.because.first(), owner_fact(92));
    }

    /// law: trigger.every-component-is-disposed-exactly-once — a composed view
    /// covers the whole component roster, an undecided component refuses under
    /// its own issue naming it, and a component disposed of twice refuses too.
    /// Owed reversal: a seam that treated an undecided component as omitted
    /// must break this law.
    #[test]
    fn every_component_is_disposed_exactly_once() {
        let plan = ExactIdentity::decoded([88; 32]);
        let selections: Vec<TriggerSelection> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .take(5)
            .map(selection)
            .collect();
        let omissions: Vec<TriggerOmission> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .skip(5)
            .map(omission)
            .collect();
        let composed = WrapperTriggerView::composed(plan, selections, omissions);
        assert!(composed.is_ok_and(|view| {
            view.len() == 8
                && !view.is_empty()
                && view.plan() == plan
                && view.selections().count() == 5
                && view.omissions().count() == 3
                && view
                    .selections()
                    .all(|selection| !selection.because.is_empty())
                && view
                    .omissions()
                    .all(|omission| !omission.because.is_empty())
        }));

        let undecided: Vec<TriggerSelection> = WRAPPER_COMPONENTS
            .iter()
            .copied()
            .filter(|component| *component != WrapperComponent::Cancellation)
            .map(selection)
            .collect();
        let refused = WrapperTriggerView::composed(plan, undecided, Vec::new());
        assert!(refused.is_err_and(|composition| matches!(
            composition.issues.first(),
            TriggerViewIssue::MissingComponentDisposition {
                component: WrapperComponent::Cancellation
            }
        )));

        let doubled: Vec<TriggerSelection> =
            WRAPPER_COMPONENTS.iter().copied().map(selection).collect();
        let twice = WrapperTriggerView::composed(
            plan,
            doubled,
            vec![omission(WrapperComponent::Observation)],
        );
        assert!(twice.is_err_and(|composition| matches!(
            composition.issues.first(),
            TriggerViewIssue::DoubledComponent {
                component: WrapperComponent::Observation
            }
        )));
    }

    /// law: trigger.the-view-family-is-an-issue-collection — the composition
    /// family declares the collection shape and elects no primary issue, and a
    /// view missing several dispositions reports all of them at once.
    /// Owed reversal (red twin): reporting only the first undecided component
    /// must break this law.
    #[test]
    fn the_view_family_is_an_issue_collection() {
        assert!(matches!(
            TriggerViewComposition::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(TriggerViewComposition::SELECTION_ORDER.is_empty());

        let refused = WrapperTriggerView::composed(
            ExactIdentity::decoded([89; 32]),
            vec![selection(WrapperComponent::Admission)],
            Vec::new(),
        );
        assert!(refused.is_err_and(|composition| composition.issues.len() == 7));
    }
}

mod composition {
    use crate::composition::{
        CompositionRoot, CompositionRootDeclaration, CompositionRootIssue, DESCRIPTOR_KINDS,
        DescriptorKind, DescriptorProvider,
    };
    use crate::plane::{ExactIdentity, OwnerFactRef};
    use threadpak::refusal::{FamilyShape, RefusalFamily};

    /// The closed descriptor-kind roster, proven closed by an exhaustive match:
    /// a new kind stops compiling here until it is placed.
    const fn kind_index(kind: DescriptorKind) -> usize {
        match kind {
            DescriptorKind::TestDescriptor => 0,
            DescriptorKind::BenchmarkDescriptor => 1,
            DescriptorKind::HostBindingDescriptor => 2,
            DescriptorKind::DocumentationIndexEntry => 3,
            DescriptorKind::ApiInventoryRow => 4,
            DescriptorKind::RemoteSurfaceEntry => 5,
        }
    }

    /// One owner fact, for laws that need a home citation.
    fn owner_fact(tag: u8) -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([tag; 32]),
            fact: ExactIdentity::decoded([tag.saturating_add(1); 32]),
        }
    }

    /// One provider of the named kind under the identity byte named.
    fn provider(kind: DescriptorKind, tag: u8) -> DescriptorProvider {
        DescriptorProvider {
            provider: ExactIdentity::decoded([tag; 32]),
            home: owner_fact(tag.saturating_add(50)),
            kind,
        }
    }

    /// law: composition.descriptor-kinds-are-six-and-closed — the kinds a
    /// provider may compose are a closed roster whose members are pairwise
    /// distinct and declared in one order.
    /// Owed reversal: adding a kind without placing it must break this law.
    #[test]
    fn descriptor_kinds_are_six_and_closed() {
        assert_eq!(DESCRIPTOR_KINDS.len(), 6);
        let indexes: Vec<usize> = DESCRIPTOR_KINDS.iter().copied().map(kind_index).collect();
        assert!(
            indexes
                .iter()
                .enumerate()
                .all(|(position, index)| *index == position)
        );
    }

    /// law: composition.a-provider-names-its-home-and-its-kind — a declared
    /// provider carries the owning home its facts come from and the kind it
    /// composes, and the root hands both back on a read-only pass.
    /// Owed reversal: a provider standing on its own authority must break this
    /// law.
    #[test]
    fn a_provider_names_its_home_and_its_kind() {
        let root = CompositionRoot::declared(
            provider(DescriptorKind::TestDescriptor, 1),
            vec![provider(DescriptorKind::ApiInventoryRow, 2)],
        );
        assert!(root.is_ok_and(|root| {
            let kinds: Vec<DescriptorKind> = root.iter().map(|held| held.kind).collect();
            let homes: Vec<OwnerFactRef> = root.iter().map(|held| held.home).collect();
            kinds
                == vec![
                    DescriptorKind::TestDescriptor,
                    DescriptorKind::ApiInventoryRow,
                ]
                && homes == vec![owner_fact(51), owner_fact(52)]
                && root.len() == 2
                && !root.is_empty()
                && root.first().provider == ExactIdentity::decoded([1; 32])
        }));
    }

    /// law: composition.a-root-refuses-a-duplicate-provider — one provider
    /// identity declared twice refuses naming that provider, and a root past
    /// its declared magnitude refuses naming the seat. Neither is deduplicated
    /// and neither is trimmed.
    /// Owed reversal (red twin): a root that silently kept one of two entries
    /// must break this law.
    #[test]
    fn a_root_refuses_a_duplicate_provider() {
        let doubled = CompositionRoot::declared(
            provider(DescriptorKind::TestDescriptor, 3),
            vec![provider(DescriptorKind::BenchmarkDescriptor, 3)],
        );
        assert!(doubled.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            CompositionRootIssue::DuplicateProvider { .. }
        )));

        let overrun: Vec<DescriptorProvider> = (0..70_u8)
            .map(|tag| {
                provider(
                    DescriptorKind::DocumentationIndexEntry,
                    tag.saturating_add(100),
                )
            })
            .collect();
        let refused =
            CompositionRoot::declared(provider(DescriptorKind::RemoteSurfaceEntry, 4), overrun);
        assert!(refused.is_err_and(|refusal| matches!(
            refusal.issues.first(),
            CompositionRootIssue::SeatBoundExceeded {
                bound: 64,
                observed: 71
            }
        )));
    }

    /// law: composition.the-root-family-is-an-issue-collection — the
    /// declaration family declares the collection shape, elects no primary
    /// issue, and reports every doubled provider at once.
    /// Owed reversal (red twin): declaring `SingleCause` with a collection body
    /// must break this law.
    #[test]
    fn the_root_family_is_an_issue_collection() {
        assert!(matches!(
            CompositionRootDeclaration::SHAPE,
            FamilyShape::IssueCollection
        ));
        assert!(CompositionRootDeclaration::SELECTION_ORDER.is_empty());

        let refused = CompositionRoot::declared(
            provider(DescriptorKind::TestDescriptor, 5),
            vec![
                provider(DescriptorKind::TestDescriptor, 5),
                provider(DescriptorKind::ApiInventoryRow, 6),
                provider(DescriptorKind::ApiInventoryRow, 6),
            ],
        );
        assert!(refused.is_err_and(|refusal| refusal.issues.len() == 2));
    }
}

mod pattern_stamp {
    use crate::origin_graph::{OriginRelation, TraceDecision};
    use crate::pattern_stamp::{
        ScopeGuardOwnerFacts, ScopeGuardStampAnchors, plan_scope_guard_stamp,
    };
    use crate::plane::{ExactIdentity, OwnerFactRef, ProfileVersion};
    use crate::planning::{ProjectionContext, TargetBinding};

    /// One owner fact, distinguished by its fact identity.
    fn owner_fact(fact: u8) -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([100; 32]),
            fact: ExactIdentity::decoded([fact; 32]),
        }
    }

    /// The anchors one demo stamp is planned against.
    fn anchors() -> ScopeGuardStampAnchors {
        ScopeGuardStampAnchors {
            context: ProjectionContext {
                graph: ExactIdentity::decoded([101; 32]),
                profile: ExactIdentity::decoded([102; 32]),
                profile_version: ProfileVersion::declared(1),
                sources: ProjectionContext::one_source(ExactIdentity::decoded([103; 32])),
                generator: ExactIdentity::decoded([104; 32]),
                target: TargetBinding::TargetFree,
            },
            pattern: ExactIdentity::decoded([105; 32]),
            instance: ExactIdentity::decoded([106; 32]),
            guard_name: ExactIdentity::decoded([107; 32]),
            scope_type: ExactIdentity::decoded([108; 32]),
            authored_node: ExactIdentity::decoded([109; 32]),
            instantiated_node: ExactIdentity::decoded([110; 32]),
            rendered_node: ExactIdentity::decoded([111; 32]),
            stamped_unit: ExactIdentity::decoded([112; 32]),
            stamped_digest: ExactIdentity::decoded([113; 32]),
            traced: ExactIdentity::decoded([114; 32]),
            owner_facts: ScopeGuardOwnerFacts {
                class_c_carries_no_ordering: owner_fact(115),
                comparison_is_scope_guarded: owner_fact(116),
            },
        }
    }

    /// law: pattern-stamp.a-declarative-stamp-carries-a-complete-plan — the plan
    /// family carries a declarative stamp: one output, a trail that walks back
    /// through the pattern-instantiation edge to the authored declaration, two
    /// decisions in selection order each citing an identity-home fact, three
    /// watched identities, and the two typed arguments the caller stated.
    /// Owed reversal: a stamp planned without its instantiation edge, or with a
    /// string where a typed argument belongs, must break this law.
    #[test]
    fn a_declarative_stamp_carries_a_complete_plan() {
        let planned = plan_scope_guard_stamp(&anchors());
        assert!(planned.is_ok_and(|plan| {
            plan.membership().len() == 1
                && plan.origin().len() == 2
                && matches!(
                    plan.origin().first().relation,
                    OriginRelation::PatternInstantiation
                )
                && plan.trace().len() == 2
                && matches!(
                    plan.trace().first().decision,
                    TraceDecision::SelectedBecause(_)
                )
                && plan.invalidation().len() == 3
                && plan.content().arguments.len() == 2
                && plan.nonclaims().is_empty()
                && !plan.membership().first().origin.is_empty()
        }));
    }

    /// law: pattern-stamp.the-stamp-cites-the-identity-home-and-never-itself —
    /// both decisions cite owner facts of one home, and the two facts are
    /// distinct: a stamp that cited itself would be its own oracle.
    /// Owed reversal: collapsing the two facts into one citation must break this
    /// law.
    #[test]
    fn the_stamp_cites_the_identity_home_and_never_itself() {
        let facts = anchors().owner_facts;
        assert_eq!(
            facts.class_c_carries_no_ordering.home,
            facts.comparison_is_scope_guarded.home
        );
        assert_ne!(
            facts.class_c_carries_no_ordering.fact,
            facts.comparison_is_scope_guarded.fact
        );
        let planned = plan_scope_guard_stamp(&anchors());
        assert!(planned.is_ok_and(|plan| {
            matches!(
                plan.trace().first().decision,
                TraceDecision::SelectedBecause(cited) if cited == facts.class_c_carries_no_ordering
            )
        }));
    }
}

mod derive_refusal {
    use crate::derive_refusal::{
        CaptureDiagnosticAnchors, CapturedCause, CauseOrderStanding, DerivationAnchors,
        DerivedItem, DerivedMembership, PlantedDefect, RefusalDeriveCapture,
        RefusalDeriveDisposition, RefusalDeriveSurface, RefusalOwnerFacts, captured, disposed,
    };
    use crate::diagnostics::MacrocPhase;
    use crate::plane::{ExactIdentity, OwnerFactRef, ProfileVersion};
    use crate::planning::{ProjectionContext, ProjectionDisposition, TargetBinding};
    use threadpak::declaration::CoordinateRole;
    use threadpak::evidence::CauseDisposition;
    use threadpak::refusal::{CauseOrderDeclaration, FamilyShape, RefusalFamily};

    /// The lawful single-cause declaration, as a token stream renders it.
    const SINGLE_CAUSE: &str = "#[refusal(shape = single_cause, order(NotCanonical = \
        \"demo.not-canonical\", NotAdmitted = \"demo.not-admitted\"))] enum DemoFamily { \
        NotCanonical, NotAdmitted, }";

    /// The lawful collection declaration: no order clause, and none admitted.
    const ISSUE_COLLECTION: &str = "#[refusal(shape = issue_collection)] enum DemoIssues { \
        NotCanonical, NotAdmitted, }";

    /// One owner fact, distinguished by its fact identity.
    fn owner_fact(fact: u8) -> OwnerFactRef {
        OwnerFactRef {
            home: ExactIdentity::decoded([200; 32]),
            fact: ExactIdentity::decoded([fact; 32]),
        }
    }

    /// The anchors one demo derivation is projected against.
    fn anchors() -> DerivationAnchors {
        DerivationAnchors {
            context: ProjectionContext {
                graph: ExactIdentity::decoded([201; 32]),
                profile: ExactIdentity::decoded([202; 32]),
                profile_version: ProfileVersion::declared(1),
                sources: ProjectionContext::one_source(ExactIdentity::decoded([203; 32])),
                generator: ExactIdentity::decoded([204; 32]),
                target: TargetBinding::TargetFree,
            },
            kind: ExactIdentity::decoded([205; 32]),
            derived_type: ExactIdentity::decoded([206; 32]),
            family_contract: ExactIdentity::decoded([207; 32]),
            authored_node: ExactIdentity::decoded([208; 32]),
            family_node: ExactIdentity::decoded([209; 32]),
            family_unit: ExactIdentity::decoded([210; 32]),
            family_digest: ExactIdentity::decoded([211; 32]),
            order_node: ExactIdentity::decoded([212; 32]),
            order_unit: ExactIdentity::decoded([213; 32]),
            order_digest: ExactIdentity::decoded([214; 32]),
            traced: ExactIdentity::decoded([215; 32]),
            owner_facts: RefusalOwnerFacts {
                body_shapes: owner_fact(216),
                canonical_order_is_shape_ruled: owner_fact(217),
            },
        }
    }

    /// law: derive-refusal.the-engine-declares-its-own-order-by-hand — the
    /// capture family is single-cause, its typed order carries ten distinct
    /// stable identities, and its textual selection order is exactly that
    /// order's projection. The services never derive their own contracts.
    /// Owed reversal: a projection that permuted, dropped, or added a spelling
    /// must break this law.
    #[test]
    fn the_engine_declares_its_own_order_by_hand() {
        assert_eq!(RefusalDeriveCapture::SHAPE, FamilyShape::SingleCause);
        assert_eq!(RefusalDeriveCapture::DECLARED_ORDER.len(), 10);
        assert!(
            RefusalDeriveCapture::DECLARED_ORDER.projects_to(RefusalDeriveCapture::SELECTION_ORDER)
        );
        let identities: Vec<&str> = RefusalDeriveCapture::DECLARED_ORDER
            .iter()
            .map(|cause| cause.id().as_declared())
            .collect();
        assert!(identities.iter().enumerate().all(|(position, identity)| {
            identities
                .iter()
                .skip(position.saturating_add(1))
                .all(|other| other != identity)
        }));
    }

    /// law: derive-refusal.a-lawful-declaration-captures-typed — the captured
    /// surface carries the family name, the machine's own body shape, and the
    /// causes in the declared canonical order with their stable identities,
    /// and the caller wrote no selection-order string anywhere.
    /// Owed reversal: a capture that read the order from the body layout rather
    /// than from the order clause must break this law.
    #[test]
    fn a_lawful_declaration_captures_typed() {
        let captured = captured(SINGLE_CAUSE);
        assert!(captured.is_ok_and(|surface| {
            let causes: Vec<&CapturedCause> = surface.causes().collect();
            surface.family_name() == "DemoFamily"
                && surface.shape() == FamilyShape::SingleCause
                && causes.len() == 2
                && causes.first().is_some_and(|cause| {
                    cause.spelling() == "NotCanonical" && cause.stable_id() == "demo.not-canonical"
                })
                && causes
                    .get(1)
                    .is_some_and(|cause| cause.spelling() == "NotAdmitted")
        }));
    }

    /// law: derive-refusal.the-declared-order-is-the-selector-not-the-layout —
    /// an order clause naming the same causes in another order captures that
    /// order, because the canonical order is a selector over established
    /// conditions and not the order the variants happen to be written in.
    /// Owed reversal: a capture that sorted the causes, or took them from the
    /// body, must break this law.
    #[test]
    fn the_declared_order_is_the_selector_not_the_layout() {
        let reordered = "#[refusal(shape = single_cause, order(NotAdmitted = \
            \"demo.not-admitted\", NotCanonical = \"demo.not-canonical\"))] enum DemoFamily { \
            NotCanonical, NotAdmitted, }";
        let captured = captured(reordered);
        assert!(captured.is_ok_and(|surface| {
            surface
                .causes()
                .next()
                .is_some_and(|cause| cause.spelling() == "NotAdmitted")
        }));
    }

    /// law: derive-refusal.every-malformed-declaration-establishes-one-cause —
    /// each grammar failure establishes its own cause at a byte coordinate, and
    /// no two of them collapse into one.
    /// Owed reversal (red twin): collapsing two causes into one, or returning a
    /// bare unit error, must break this law.
    #[test]
    fn every_malformed_declaration_establishes_one_cause() {
        let cases: [(&str, RefusalDeriveCapture); 7] = [
            ("struct NotAnEnumAtAll;", RefusalDeriveCapture::NotAnEnum),
            ("enum { A, }", RefusalDeriveCapture::NotNamed),
            (
                "#[refusal(shape = single_cause, order())] enum DemoFamily { }",
                RefusalDeriveCapture::NotInhabited,
            ),
            (
                "enum DemoFamily { A, }",
                RefusalDeriveCapture::NotShapeDeclared,
            ),
            (
                "#[refusal(shape = tri_state)] enum DemoFamily { A, }",
                RefusalDeriveCapture::NotAnAdmittedShape,
            ),
            (
                "#[refusal(shape = single_cause)] enum DemoFamily { A, }",
                RefusalDeriveCapture::NotOrderDeclared,
            ),
            (
                "#[refusal(shape = issue_collection, order(A = \"x\"))] enum DemoFamily { A, }",
                RefusalDeriveCapture::NotOrderAdmitted,
            ),
        ];
        for (source, expected) in cases {
            let refused = captured(source);
            assert!(
                refused.is_err_and(|refusal| {
                    refusal.cause() == expected && refusal.coordinate().role == CoordinateRole::Byte
                }),
                "{source}"
            );
        }
    }

    /// law: derive-refusal.coverage-and-distinctness-are-separate-causes — an
    /// order clause that names a cause the body does not, and one that reuses a
    /// stable identity, establish different causes.
    /// Owed reversal: folding distinctness into coverage must break this law.
    #[test]
    fn coverage_and_distinctness_are_separate_causes() {
        let uncovered = "#[refusal(shape = single_cause, order(NotCanonical = \"a\"))] \
            enum DemoFamily { NotCanonical, NotAdmitted, }";
        let repeated = "#[refusal(shape = single_cause, order(NotCanonical = \"a\", \
            NotAdmitted = \"a\"))] enum DemoFamily { NotCanonical, NotAdmitted, }";
        assert!(
            captured(uncovered)
                .is_err_and(|refusal| refusal.cause() == RefusalDeriveCapture::NotCovered)
        );
        assert!(
            captured(repeated)
                .is_err_and(|refusal| refusal.cause() == RefusalDeriveCapture::NotDistinct)
        );
    }

    /// law: derive-refusal.an-oversized-declaration-refuses-rather-than-reads —
    /// a declared input past the declared magnitude is refused before it is
    /// parsed at all.
    /// Owed reversal: an unbounded read must break this law.
    #[test]
    fn an_oversized_declaration_refuses_rather_than_reads() {
        let oversized = format!("enum DemoFamily {{ {} }}", "A, ".repeat(4096));
        assert!(
            captured(&oversized)
                .is_err_and(|refusal| refusal.cause() == RefusalDeriveCapture::Unbounded)
        );
    }

    /// law: derive-refusal.a-capture-refusal-projects-into-a-diagnostic — the
    /// typed cause projects into the services' structured diagnostic with the
    /// capture phase, the byte coordinate, the typed observed classification,
    /// the established cause, an owner-cited repair, and the callable
    /// reproduction route.
    /// Owed reversal: a diagnostic without a coordinate or without a cited
    /// repair must break this law.
    #[test]
    fn a_capture_refusal_projects_into_a_diagnostic() {
        let refused = captured("struct NotAnEnumAtAll;");
        assert!(refused.is_err_and(|refusal| {
            let diagnostic = refusal.diagnosed(&CaptureDiagnosticAnchors {
                reason: ExactIdentity::decoded([220; 32]),
                family: ExactIdentity::decoded([221; 32]),
                declaration: ExactIdentity::decoded([222; 32]),
                fragment: ExactIdentity::decoded([223; 32]),
                graph: ExactIdentity::decoded([224; 32]),
                expected: ExactIdentity::decoded([225; 32]),
                posture: CauseDisposition::UnresolvedCause,
                entry: ExactIdentity::decoded([227; 32]),
                repair_declared_by: owner_fact(228),
            });
            matches!(diagnostic.phase, MacrocPhase::Capture)
                && diagnostic.coordinate.role == CoordinateRole::Byte
                && diagnostic.repairs.len() == 1
                && matches!(diagnostic.cause, CauseDisposition::UnresolvedCause)
                && matches!(
                    diagnostic.observed,
                    crate::diagnostics::ObservedClassification::ContractDisagreement
                )
                && matches!(
                    diagnostic.reproduction,
                    crate::diagnostics::ReproductionRoute::CallableServices { .. }
                )
                && !refusal.compiler_message().is_empty()
        }));
    }

    /// law: derive-refusal.rendering-materializes-exactly-what-was-planned —
    /// the rendered membership is the planned membership, item for item, and
    /// the single-cause family renders both implementations while the
    /// collection family renders one.
    /// Owed reversal (red twin): a rendering that emitted a sibling the plan did
    /// not declare must break this law.
    #[test]
    fn rendering_materializes_exactly_what_was_planned() {
        let single = captured(SINGLE_CAUSE).map(RefusalDeriveSurface::planned);
        assert!(single.is_ok_and(|derivation| {
            let rendered = derivation.rendered();
            rendered.membership() == derivation.declared_membership()
                && derivation.declared_membership() == DerivedMembership::FamilyAndCauseOrder
                && derivation.declared_membership().len() == 2
                && !derivation.declared_membership().is_empty()
                && derivation.declared_membership().items()
                    == [
                        DerivedItem::FamilyImplementation,
                        DerivedItem::CauseOrderImplementation,
                    ]
                && rendered
                    .source()
                    .contains("impl ::threadpak::refusal::RefusalFamily")
                && rendered
                    .source()
                    .contains("impl ::threadpak::refusal::CauseOrderDeclaration")
        }));
        let collection = captured(ISSUE_COLLECTION).map(RefusalDeriveSurface::planned);
        assert!(collection.is_ok_and(|derivation| {
            let rendered = derivation.rendered();
            derivation.declared_membership() == DerivedMembership::FamilyOnly
                && rendered.membership() == derivation.declared_membership()
                && !rendered
                    .source()
                    .contains("impl ::threadpak::refusal::CauseOrderDeclaration")
        }));
    }

    /// law: derive-refusal.the-textual-order-is-emitted-from-the-typed-rows —
    /// the rendered selection order is the declared spellings in the declared
    /// positions, and the rendered typed order carries the declared identities.
    /// Owed reversal: emitting a caller-supplied string list must break this
    /// law.
    #[test]
    fn the_textual_order_is_emitted_from_the_typed_rows() {
        let rendered = captured(SINGLE_CAUSE).map(|surface| surface.planned().rendered());
        assert!(rendered.is_ok_and(|rendered| {
            rendered
                .source()
                .contains("&[\"NotCanonical\", \"NotAdmitted\"]")
                && rendered.source().contains("\"demo.not-canonical\"")
                && rendered.source().contains("\"demo.not-admitted\"")
        }));
    }

    /// law: derive-refusal.a-planted-defect-changes-the-rendering-and-not-the-declaration
    /// — both planted defects leave the captured surface untouched and change
    /// only what the rendering claims, which is what makes them catchable from
    /// outside.
    /// Owed reversal: a planted defect that also moved the typed order would be
    /// undetectable and must break this law.
    #[test]
    fn a_planted_defect_changes_the_rendering_and_not_the_declaration() {
        let derivation = captured(SINGLE_CAUSE).map(RefusalDeriveSurface::planned);
        assert!(derivation.is_ok_and(|derivation| {
            let lawful = derivation.rendered();
            let permuted =
                derivation.rendered_with_planted_defect(PlantedDefect::SelectionOrderPermuted);
            let recycled =
                derivation.rendered_with_planted_defect(PlantedDefect::CauseIdentityRecycled);
            lawful.source() != permuted.source()
                && lawful.source() != recycled.source()
                && permuted
                    .source()
                    .contains("&[\"NotAdmitted\", \"NotCanonical\"]")
                && !recycled.source().contains("\"demo.not-admitted\"")
                && lawful.membership() == permuted.membership()
                && derivation
                    .surface()
                    .causes()
                    .next()
                    .is_some_and(|cause| cause.stable_id() == "demo.not-canonical")
        }));
    }

    /// law: derive-refusal.the-plan-carries-the-derivation-and-its-absences —
    /// a single-cause derivation plans two outputs and disposes the cause-order
    /// projection as generated; a collection derivation plans one and disposes
    /// it as not applicable on the refusal home's fact. Silence is never the
    /// answer.
    /// Owed reversal: a shorter membership with no stated disposition must break
    /// this law.
    #[test]
    fn the_plan_carries_the_derivation_and_its_absences() {
        let anchors = anchors();
        let single = captured(SINGLE_CAUSE)
            .map(RefusalDeriveSurface::planned)
            .map_err(|_| ())
            .and_then(|derivation| derivation.projected(&anchors).map_err(|_| ()));
        assert!(single.is_ok_and(|projection| {
            projection.plan().membership().len() == 2
                && projection.plan().trace().len() == 2
                && projection.plan().invalidation().len() == 3
                && projection.plan().content().assumptions.len() == 2
                && matches!(
                    projection.cause_order(),
                    ProjectionDisposition::Generated { .. }
                )
        }));
        let collection = captured(ISSUE_COLLECTION)
            .map(RefusalDeriveSurface::planned)
            .map_err(|_| ())
            .and_then(|derivation| derivation.projected(&anchors).map_err(|_| ()));
        assert!(collection.is_ok_and(|projection| {
            projection.plan().membership().len() == 1
                && matches!(
                    projection.cause_order(),
                    ProjectionDisposition::NotApplicable { because }
                        if *because == anchors.owner_facts.canonical_order_is_shape_ruled
                )
        }));
    }

    /// law: derive-refusal.the-derivation-answers-every-applicable-question —
    /// the explanation view completes over the implementation kind's nine
    /// applicable questions, and the why-not-generated seat carries the
    /// cause-order disposition rather than a sentence.
    /// Owed reversal: a view missing the disposition seat must break this law.
    #[test]
    fn the_derivation_answers_every_applicable_question() {
        let anchors = anchors();
        let explained = captured(ISSUE_COLLECTION)
            .map(RefusalDeriveSurface::planned)
            .map_err(|_| ())
            .and_then(|derivation| {
                derivation
                    .projected(&anchors)
                    .map_err(|_| ())
                    .and_then(|projection| {
                        derivation
                            .explained(&projection, &anchors)
                            .map_err(|_| ())
                            .map(|view| view.len())
                    })
            });
        assert_eq!(explained, Ok(9));
    }

    /// law: derive-refusal.the-standing-of-the-cause-order-is-typed — whether a
    /// family carries the typed cause order is a typed answer read off the
    /// shape, never a boolean the caller reinterprets.
    /// Owed reversal: returning a boolean must break this law.
    #[test]
    fn the_standing_of_the_cause_order_is_typed() {
        let single = captured(SINGLE_CAUSE).map(RefusalDeriveSurface::planned);
        let collection = captured(ISSUE_COLLECTION).map(RefusalDeriveSurface::planned);
        assert!(single.is_ok_and(|derivation| matches!(
            derivation.cause_order_standing(),
            CauseOrderStanding::Declared
        )));
        assert!(collection.is_ok_and(|derivation| matches!(
            derivation.cause_order_standing(),
            CauseOrderStanding::NotApplicableToShape
        )));
    }

    /// law: derive-refusal.the-one-call-road-disposes-either-way — the road an
    /// expansion surface takes answers with a rendering or with an established
    /// refusal, and never with silence.
    /// Owed reversal: an empty expansion on a malformed input must break this
    /// law.
    #[test]
    fn the_one_call_road_disposes_either_way() {
        assert!(matches!(
            disposed(SINGLE_CAUSE),
            RefusalDeriveDisposition::Generated(_)
        ));
        assert!(matches!(
            disposed("struct NotAnEnumAtAll;"),
            RefusalDeriveDisposition::Refused(_)
        ));
    }
}
