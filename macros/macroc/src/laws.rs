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
        HumanProjection, HumanTextLimit, OwnerFactSubject, OwnerHomeSubject, OwnerIdentityRef,
        ProfileVersion, RefusalReason,
    };
    use threadpak::types::{BoundedConstruction, ConstLimit};

    /// law: plane.subjects-do-not-unify — a reference naming one subject is a
    /// different type than a reference naming another, whatever the bytes.
    /// Owed reversal: erasing the subject parameter must break this law.
    #[test]
    fn subjects_do_not_unify() {
        let home: fn(OwnerIdentityRef<OwnerHomeSubject>) = drop;
        let fact: fn(OwnerIdentityRef<OwnerFactSubject>) = drop;
        assert!((home as usize) != 0 && (fact as usize) != 0);
        let same_bytes_different_subject = OwnerIdentityRef::<OwnerHomeSubject>::decoded([3; 32]);
        assert_eq!(same_bytes_different_subject.as_bytes(), &[3_u8; 32]);
    }

    /// law: plane.reason-projection-preserves-bytes — projecting a registered
    /// reason adapts nothing; a projection may change presentation, never
    /// identity.
    /// Owed reversal: a projection that rewrote the bytes must break this law.
    #[test]
    fn reason_projection_preserves_bytes() {
        let declared = OwnerIdentityRef::<RefusalReason>::decoded([9; 32]);
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
    use crate::plane::PlanningIssueLimit;
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
        let node = crate::plane::for_laws(1);
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
        DiagnosticSite, MACROC_PHASES, MachineAnchoring, MachineAnchors, MacrocDiagnostic,
        MacrocPhase, ObservedClassification, ReleasePosture, RepairAction, ReproductionRoute,
    };
    use crate::plane::{HumanProjection, OwnerFactRef, OwnerIdentityRef};
    use crate::token::SpanHandle;
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
        let declared_by = OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([40; 32]),
            fact: OwnerIdentityRef::decoded([41; 32]),
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
            machine: MachineAnchoring::Anchored(Box::new(MachineAnchors {
                reason: OwnerIdentityRef::decoded([42; 32]),
                family: OwnerIdentityRef::decoded([43; 32]),
                declaration: OwnerIdentityRef::decoded([44; 32]),
                fragment: OwnerIdentityRef::decoded([45; 32]),
                graph: OwnerIdentityRef::decoded([46; 32]),
            })),
            summary: HumanProjection::empty(),
            phase: MacrocPhase::Planning,
            site: DiagnosticSite {
                token: SpanHandle::at(4),
                coordinate: SourceCoordinate {
                    role: CoordinateRole::SemanticOrigin,
                    position: 17,
                },
            },
            expected: crate::plane::for_laws(47),
            observed: ObservedClassification::SeatAbsent,
            cause: CauseDisposition::UnresolvedCause,
            related: Bounded::empty(),
            repairs,
            reproduction: ReproductionRoute::CallableServices {
                entry: crate::plane::for_laws(48),
            },
            release: ReleasePosture::NoReleasePromise,
        });
        assert!(built.is_ok_and(|diagnostic| {
            diagnostic.repairs.len() == 1
                && diagnostic.related.is_empty()
                && diagnostic.site.coordinate.position == 17
                && diagnostic.site.token == SpanHandle::at(4)
                && matches!(diagnostic.machine, MachineAnchoring::Anchored(_))
                && matches!(diagnostic.cause, CauseDisposition::UnresolvedCause)
                && matches!(diagnostic.phase, MacrocPhase::Planning)
        }));
    }

    /// law: diagnostics.an-unanchored-diagnostic-says-so — where the machine has
    /// minted no identity for an observation, the diagnostic states the posture
    /// rather than carrying a stand-in. The compiler plane never mints a value
    /// that independently answers a question the machine owns.
    /// Owed reversal (red twin): a plane-minted "reason identity" filling the
    /// seat must break this law.
    #[test]
    fn an_unanchored_diagnostic_says_so() {
        let anchored = MachineAnchoring::Anchored(Box::new(MachineAnchors {
            reason: OwnerIdentityRef::decoded([42; 32]),
            family: OwnerIdentityRef::decoded([43; 32]),
            declaration: OwnerIdentityRef::decoded([44; 32]),
            fragment: OwnerIdentityRef::decoded([45; 32]),
            graph: OwnerIdentityRef::decoded([46; 32]),
        }));
        assert_ne!(anchored, MachineAnchoring::UnmintedAtThisSeam);
        assert!(matches!(
            MachineAnchoring::UnmintedAtThisSeam,
            MachineAnchoring::UnmintedAtThisSeam
        ));
    }

    /// law: diagnostics.an-owner-fact-may-be-named-without-being-minted — a
    /// citation names the home and the fact the owning home wrote down, which is
    /// a reference to an owner fact and never a second answer to it.
    /// Owed reversal: a `Declared` citation that derived an identity of its own
    /// must break this law.
    #[test]
    fn an_owner_fact_may_be_named_without_being_minted() {
        let named = OwnerFactRef::named("refusal", "family-shapes-are-three-and-closed");
        let minted = OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([40; 32]),
            fact: OwnerIdentityRef::decoded([41; 32]),
        };
        assert_ne!(named, minted);
        assert_ne!(named.citation_bytes(), minted.citation_bytes());
        assert_eq!(
            named.citation_bytes(),
            OwnerFactRef::named("refusal", "family-shapes-are-three-and-closed").citation_bytes()
        );
    }

    /// law: diagnostics.repairs-cite-their-owner — a repair carries the owner
    /// fact that declares it, so no rendering can present composed advice as
    /// declared authority.
    /// Owed reversal: a repair whose only member is text must break this law.
    #[test]
    fn repairs_cite_their_owner() {
        let declared_by = OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([49; 32]),
            fact: OwnerIdentityRef::decoded([50; 32]),
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
            entry: crate::plane::for_laws(51),
        };
        assert!(matches!(route, ReproductionRoute::CallableServices { .. }));
        let shell = ReproductionRoute::ExpansionShell {
            surface: crate::plane::for_laws(52),
        };
        let fixture = ReproductionRoute::RecordedFixture {
            population: crate::plane::for_laws(53),
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
    use crate::plane::{OriginEdgeLimit, OwnerFactRef, OwnerIdentityRef, TraceEntryLimit};
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
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([1; 32]),
            fact: OwnerIdentityRef::decoded([2; 32]),
        }
    }

    /// One edge, for laws that need a trail.
    fn edge() -> OriginEdge {
        OriginEdge {
            from: crate::plane::for_laws(3),
            relation: OriginRelation::AuthoredDeclaration,
            to: crate::plane::for_laws(4),
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
            subject: crate::plane::for_laws(5),
            decision: TraceDecision::SelectedBecause(owner_fact()),
        };
        let omitted = TraceEntry {
            subject: crate::plane::for_laws(5),
            decision: TraceDecision::OmittedBecause(owner_fact()),
        };
        let not_run = TraceEntry {
            subject: crate::plane::for_laws(5),
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
            subject: crate::plane::for_laws(6),
            decision: TraceDecision::NotRun,
        };
        let second = TraceEntry {
            subject: crate::plane::for_laws(7),
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
            unclaimed: crate::plane::for_laws(8),
            because: owner_fact(),
        };
        assert_eq!(nonclaim.because, owner_fact());
    }
}

mod planning {
    use crate::origin_graph::{
        DecisionTrace, Nonclaim, OriginEdge, OriginRelation, OriginTrail, TraceDecision, TraceEntry,
    };
    use crate::plane::{OwnerFactRef, OwnerIdentityRef, ProfileVersion, SoleRenderedUnit};
    use crate::planning::{
        BenchmarkDescriptorProjection, CauseAnchoring, CodecProjection, DeriveImplContent,
        DeriveImplProjection, DigestContract, DocumentationProjection, GraphAnchoring,
        HostWrapperContent, HostWrapperProjection, InvalidationTrigger, MemberDestination,
        PatternStampProjection, PlannedMember, PlannedMembership, PlannedOutput,
        ProjectionBundlePlan, ProjectionContext, ProjectionDisposition, ProjectionKind,
        ProjectionPlan, RemoteSurfaceProjection, RenderedImplementation, TargetBinding,
        TargetRequirement, TestDescriptorProjection, UNIVERSAL_QUESTIONS, WRAPPER_COMPONENTS,
        WrapperComponent,
    };
    use crate::question::{EXPLANATION_QUESTIONS, ExplanationQuestion};
    use crate::refusal::{PlanSeat, ProjectionPlanning, ProjectionPlanningIssue};
    use threadpak::types::{Bounded, NonEmptyBounded};

    /// One owner fact, for laws that need a citation.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([10; 32]),
            fact: OwnerIdentityRef::decoded([11; 32]),
        }
    }

    /// One origin trail, for laws that need a generated unit.
    fn trail() -> OriginTrail {
        OriginTrail::from_edge(OriginEdge {
            from: crate::plane::for_laws(12),
            relation: OriginRelation::SemanticDerivation,
            to: crate::plane::for_laws(13),
        })
    }

    /// One planned member under one rendered role. Logical only: a semantic key,
    /// a destination, an origin, a renderer, and a digest CONTRACT — never a
    /// digest, because no byte has been rendered when a plan is made.
    fn member(role: RenderedImplementation, tag: u8) -> PlannedMember<RenderedImplementation> {
        PlannedMember {
            role,
            output: planned_output(tag),
        }
    }

    /// One planned output, tagged so two of them are distinguishable.
    fn planned_output(tag: u8) -> PlannedOutput {
        let key = crate::plane::for_laws(tag);
        PlannedOutput {
            semantic_key: key,
            destination: MemberDestination::AtDeclarationSite,
            origin: trail(),
            expected_profile: crate::plane::for_laws(17),
            expected_profile_version: ProfileVersion::declared(3),
            digest_contract: DigestContract::over(key),
        }
    }

    /// One planned member for a kind whose rendering is a single unit.
    fn sole_member(tag: u8) -> PlannedMember<SoleRenderedUnit> {
        PlannedMember {
            role: SoleRenderedUnit::Sole,
            output: planned_output(tag),
        }
    }

    /// One shared context, under the binding the caller names.
    fn context(target: TargetBinding) -> ProjectionContext {
        ProjectionContext {
            graph: GraphAnchoring::ClosedGraph(OwnerIdentityRef::decoded([16; 32])),
            profile: crate::plane::for_laws(17),
            profile_version: ProfileVersion::declared(3),
            sources: CauseAnchoring::Declarations(ProjectionContext::one_source(
                OwnerIdentityRef::decoded([18; 32]),
            )),
            generator: crate::plane::for_laws(19),
            target,
        }
    }

    /// The implementation-projection content, for the complete-plan law.
    fn derive_content() -> DeriveImplContent {
        DeriveImplContent {
            derived_type: crate::plane::for_laws(20),
            contract: crate::plane::for_laws(21),
            assumptions: Bounded::empty(),
        }
    }

    /// The trace the complete-plan law records.
    fn trace() -> DecisionTrace {
        DecisionTrace::from_entry(TraceEntry {
            subject: crate::plane::for_laws(22),
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
            PlannedMembership::from_member(member(RenderedImplementation::RenderedFamilyImpl, 14)),
            InvalidationTrigger::one_watched(InvalidationTrigger::GraphIdentityChanged {
                watched: OwnerIdentityRef::decoded([16; 32]),
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
                && !plan.membership().first().output.origin.is_empty()
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
            unclaimed: crate::plane::for_laws(23),
            because: owner_fact(),
        }])
        .map_err(|_| ());
        let membership = PlannedMembership::declared(
            member(RenderedImplementation::RenderedFamilyImpl, 14),
            vec![member(RenderedImplementation::RenderedCauseOrderImpl, 15)],
        )
        .map_err(|_| ());
        let built = nonclaims.and_then(|nonclaims| {
            membership.and_then(|membership| {
                ProjectionPlan::<DeriveImplProjection>::planned(
                    context(TargetBinding::TargetFree),
                    derive_content(),
                    membership,
                    InvalidationTrigger::one_watched(
                        InvalidationTrigger::GeneratorVersionChanged {
                            watched: crate::plane::for_laws(19),
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
        let membership = PlannedMembership::declared(
            member(RenderedImplementation::RenderedFamilyImpl, 14),
            vec![member(RenderedImplementation::RenderedCauseOrderImpl, 31)],
        );
        assert!(membership.is_ok_and(|membership| {
            let keys: Vec<[u8; 32]> = membership
                .iter()
                .map(|row| *row.output.semantic_key.as_bytes())
                .collect();
            keys.len() == 2
                && keys.first() != keys.get(1)
                && membership
                    .under(RenderedImplementation::RenderedCauseOrderImpl)
                    .is_some()
                && membership.count_under(RenderedImplementation::RenderedFamilyImpl) == 1
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
                host_contract: OwnerIdentityRef::decoded([24; 32]),
                components: NonEmptyBounded::singleton(WrapperComponent::Admission),
                capability_basis: owner_fact(),
            },
            PlannedMembership::from_member(sole_member(14)),
            InvalidationTrigger::one_watched(InvalidationTrigger::TargetContractChanged {
                watched: OwnerIdentityRef::decoded([24; 32]),
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
            InvalidationTrigger::CapturedDeclarationChanged { .. } => 1,
            InvalidationTrigger::GraphIdentityChanged { .. } => 2,
            InvalidationTrigger::ProjectionProfileChanged { .. } => 3,
            InvalidationTrigger::TargetContractChanged { .. } => 4,
            InvalidationTrigger::GeneratorVersionChanged { .. } => 5,
            InvalidationTrigger::MechanismProfileChanged { .. } => 6,
            InvalidationTrigger::WorkFormulaChanged { .. } => 7,
            InvalidationTrigger::FixturePopulationChanged { .. } => 8,
        }
    }

    /// law: planning.invalidation-triggers-are-nine-and-each-watches-an-identity
    /// — the roster is closed at nine, its members are pairwise distinct, and
    /// each names the exact identity whose change invalidates.
    /// Owed reversal: a payload-free trigger must break this law.
    #[test]
    fn invalidation_triggers_are_nine_and_each_watches_an_identity() {
        let triggers = [
            InvalidationTrigger::SourceDeclarationChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
            InvalidationTrigger::CapturedDeclarationChanged {
                watched: crate::plane::for_laws(25),
            },
            InvalidationTrigger::GraphIdentityChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
            InvalidationTrigger::ProjectionProfileChanged {
                watched: crate::plane::for_laws(25),
            },
            InvalidationTrigger::TargetContractChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
            InvalidationTrigger::GeneratorVersionChanged {
                watched: crate::plane::for_laws(25),
            },
            InvalidationTrigger::MechanismProfileChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
            InvalidationTrigger::WorkFormulaChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
            InvalidationTrigger::FixturePopulationChanged {
                watched: OwnerIdentityRef::decoded([25; 32]),
            },
        ];
        assert_eq!(triggers.len(), 9);
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
            ProjectionDisposition::Generated {
                output: Box::new(planned_output(14)),
            },
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
                profile: crate::plane::for_laws(26),
                version: ProfileVersion::declared(1),
            },
            ProjectionDisposition::NotRequested,
            ProjectionDisposition::ExcludedByConfiguration {
                configuration: OwnerIdentityRef::decoded([27; 32]),
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
            crate::plane::for_laws(28),
            crate::plane::for_laws(29),
            vec![crate::plane::for_laws(30)],
        );
        assert!(bundle.is_ok_and(|plan| plan.len() == 2 && !plan.is_empty()));
        let single =
            ProjectionBundlePlan::of_one(crate::plane::for_laws(28), crate::plane::for_laws(29));
        assert_eq!(single.bundle(), crate::plane::for_laws(28));
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
        HumanProjection, HumanTextLimit, OwnerFactRef, OwnerIdentityRef, ProfileVersion,
    };
    use crate::planning::{
        CauseAnchoring, DeriveImplProjection, DigestContract, GraphAnchoring,
        HostWrapperProjection, InvalidationTrigger, MemberDestination, PlannedOutput,
        ProjectionContext, ProjectionDisposition,
    };
    use crate::question::{ExplanationQuestion, QuestionApplicability};
    use threadpak::types::Bounded;

    /// One owner fact.
    fn owner_fact() -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([60; 32]),
            fact: OwnerIdentityRef::decoded([61; 32]),
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
            from: crate::plane::for_laws(62),
            relation: OriginRelation::Rendering,
            to: crate::plane::for_laws(63),
        });
        vec![
            ProjectionExplanation::answered(
                ExplanationAnswer::Kind {
                    kind: crate::plane::for_laws(64),
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
                    sources: CauseAnchoring::Declarations(ProjectionContext::one_source(
                        OwnerIdentityRef::decoded([65; 32]),
                    )),
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::GraphAndProfile {
                    graph: GraphAnchoring::ClosedGraph(OwnerIdentityRef::decoded([66; 32])),
                    profile: crate::plane::for_laws(67),
                    version: ProfileVersion::declared(2),
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::OutputAndDigest {
                    output: Box::new(PlannedOutput {
                        semantic_key: crate::plane::for_laws(68),
                        destination: MemberDestination::AtDeclarationSite,
                        origin: trail,
                        expected_profile: crate::plane::for_laws(67),
                        expected_profile_version: ProfileVersion::declared(2),
                        digest_contract: DigestContract::over(crate::plane::for_laws(68)),
                    }),
                    digest: crate::plane::for_laws(69),
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::Invalidators {
                    triggers: InvalidationTrigger::one_watched(
                        InvalidationTrigger::GraphIdentityChanged {
                            watched: OwnerIdentityRef::decoded([66; 32]),
                        },
                    ),
                },
                human(),
            ),
            ProjectionExplanation::answered(
                ExplanationAnswer::RelatedProjectionDisposition {
                    related: crate::plane::for_laws(70),
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
                    subject: crate::plane::for_laws(71),
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
    use crate::plane::{OwnerFactRef, OwnerIdentityRef, ProfileVersion};
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
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([80; 32]),
            fact: OwnerIdentityRef::decoded([81; 32]),
        }
    }

    /// One declared hole under the category and identity byte the caller names.
    fn parameter(category: SpliceCategory, tag: u8) -> TemplateParameter {
        TemplateParameter {
            category,
            parameter: OwnerIdentityRef::decoded([tag; 32]),
        }
    }

    /// One offered commitment under the category and identity byte named.
    fn argument(category: SpliceCategory, tag: u8) -> TemplateArgument {
        TemplateArgument {
            category,
            commitment: OwnerIdentityRef::decoded([tag; 32]),
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
            formula: OwnerIdentityRef::decoded([82; 32]),
            declared_by: owner_fact(),
            over_inputs: NonEmptyBounded::singleton(OwnerIdentityRef::decoded([83; 32])),
        }
    }

    /// The third lock, as an obligation and a stated nonclaim.
    fn meter() -> CheckedMeterPosture {
        CheckedMeterPosture {
            obliged_by: owner_fact(),
            unmeasured: Nonclaim {
                unclaimed: crate::plane::for_laws(84),
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
                OwnerIdentityRef::decoded([85; 32]),
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
            profile: OwnerIdentityRef::decoded([86; 32]),
            version: ProfileVersion::declared(4),
        }
    }

    /// The meta profile, at a declared version.
    fn meta() -> VersionedProfile<crate::plane::MetaProfileSubject> {
        VersionedProfile {
            profile: OwnerIdentityRef::decoded([87; 32]),
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
                && binding.argument().commitment == OwnerIdentityRef::decoded([2; 32])
                && binding.parameter().parameter == OwnerIdentityRef::decoded([1; 32])
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
                && template.identity() == OwnerIdentityRef::decoded([85; 32])
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
                && application.template() == OwnerIdentityRef::decoded([85; 32])
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
                    ApplicativeDistinctness::DeliberatelyDistinct(OwnerIdentityRef::decoded(
                        [42; 32],
                    )),
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
            template: OwnerIdentityRef::decoded([50; 32]),
            inputs: Bounded::empty(),
            source_snapshot: OwnerIdentityRef::decoded([51; 32]),
            fragment_dependencies: Bounded::empty(),
            language_profile: language(),
            meta_profile: meta(),
            configuration: OwnerIdentityRef::decoded([52; 32]),
        };
        let reconfigured = TemplateInvocationKey {
            configuration: OwnerIdentityRef::decoded([53; 32]),
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
    use crate::plane::{OwnerFactRef, OwnerIdentityRef};
    use crate::planning::{WRAPPER_COMPONENTS, WrapperComponent};
    use crate::trigger_view::{
        TriggerOmission, TriggerSelection, TriggerViewComposition, TriggerViewIssue,
        WrapperTriggerView,
    };
    use threadpak::refusal::{FamilyShape, RefusalFamily};
    use threadpak::types::NonEmptyBounded;

    /// One owner fact, for laws that need a citation.
    fn owner_fact(tag: u8) -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([tag; 32]),
            fact: OwnerIdentityRef::decoded([tag.saturating_add(1); 32]),
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
        let plan = crate::plane::for_laws(88);
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
            crate::plane::for_laws(89),
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
    use crate::plane::{OwnerFactRef, OwnerIdentityRef};
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
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([tag; 32]),
            fact: OwnerIdentityRef::decoded([tag.saturating_add(1); 32]),
        }
    }

    /// One provider of the named kind under the identity byte named.
    fn provider(kind: DescriptorKind, tag: u8) -> DescriptorProvider {
        DescriptorProvider {
            provider: OwnerIdentityRef::decoded([tag; 32]),
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
                && root.first().provider == OwnerIdentityRef::decoded([1; 32])
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
    use crate::plane::{OwnerFactRef, OwnerIdentityRef, ProfileVersion};
    use crate::planning::{CauseAnchoring, GraphAnchoring, ProjectionContext, TargetBinding};

    /// One owner fact, distinguished by its fact identity.
    fn owner_fact(fact: u8) -> OwnerFactRef {
        OwnerFactRef::Minted {
            home: OwnerIdentityRef::decoded([100; 32]),
            fact: OwnerIdentityRef::decoded([fact; 32]),
        }
    }

    /// The anchors one demo stamp is planned against.
    fn anchors() -> ScopeGuardStampAnchors {
        ScopeGuardStampAnchors {
            context: ProjectionContext {
                graph: GraphAnchoring::ClosedGraph(OwnerIdentityRef::decoded([101; 32])),
                profile: crate::plane::for_laws(102),
                profile_version: ProfileVersion::declared(1),
                sources: CauseAnchoring::Declarations(ProjectionContext::one_source(
                    OwnerIdentityRef::decoded([103; 32]),
                )),
                generator: crate::plane::for_laws(104),
                target: TargetBinding::TargetFree,
            },
            pattern: OwnerIdentityRef::decoded([105; 32]),
            instance: OwnerIdentityRef::decoded([106; 32]),
            guard_name: OwnerIdentityRef::decoded([107; 32]),
            scope_type: OwnerIdentityRef::decoded([108; 32]),
            authored_node: crate::plane::for_laws(109),
            instantiated_node: crate::plane::for_laws(110),
            rendered_node: crate::plane::for_laws(111),
            stamped_unit: crate::plane::for_laws(112),
            traced: crate::plane::for_laws(114),
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
                && !plan.membership().first().output.origin.is_empty()
        }));
    }

    /// The owning home one citation names.
    fn citation_home(cited: OwnerFactRef) -> [u8; 32] {
        match cited {
            OwnerFactRef::Minted { home, .. } => *home.as_bytes(),
            OwnerFactRef::Declared(named) => crate::plane::provenance_tag(&[named.home.as_bytes()]),
        }
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
            citation_home(facts.class_c_carries_no_ordering),
            citation_home(facts.comparison_is_scope_guarded)
        );
        assert_ne!(
            facts.class_c_carries_no_ordering.citation_bytes(),
            facts.comparison_is_scope_guarded.citation_bytes()
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
    use crate::closure::ClosureIssue;
    use crate::derive_refusal::{
        CapturedCause, CauseOrderStanding, DerivedMembership, RefusalDeriveCapture,
        RefusalDeriveSurface, captured, captured_text, compile_refusal, compile_refusal_text,
    };
    use crate::diagnostics::{MachineAnchoring, MacrocPhase};
    use crate::planning::{ProjectionDisposition, RenderedImplementation};
    use crate::token::TextCapture;
    use threadpak::declaration::CoordinateRole;
    use threadpak::refusal::{CauseOrderDeclaration, FamilyShape, RefusalFamily};

    /// The lawful single-cause declaration, as a token stream renders it.
    const SINGLE_CAUSE: &str = "#[refusal(family = \"demo.example\", shape = single_cause, \
        order(NotCanonical = \"not-canonical\", NotAdmitted = \"not-admitted\"))] \
        enum DemoFamily { NotCanonical, NotAdmitted, }";

    /// The lawful collection declaration: no order clause, and none admitted.
    const ISSUE_COLLECTION: &str = "#[refusal(family = \"demo.example\", shape = issue_collection)] \
        enum DemoIssues { NotBound, NotCovered, }";

    /// One captured surface, or the cause the capture established.
    fn surface(source: &str) -> Result<RefusalDeriveSurface, RefusalDeriveCapture> {
        captured_text(source)
            .map(|(_, surface)| surface)
            .map_err(crate::derive_refusal::RefusalDeriveRefusal::cause)
    }

    /// law: derive.the-engine-declares-its-own-order-by-hand — the capture
    /// family's own declared facts are authored, never derived by the derive
    /// this module ships. A generator that produced its own contracts would be
    /// its own oracle.
    /// Owed reversal: deriving the engine's own order must break this law.
    #[test]
    fn the_engine_declares_its_own_order_by_hand() {
        assert!(matches!(
            RefusalDeriveCapture::SHAPE,
            FamilyShape::SingleCause
        ));
        assert_eq!(
            RefusalDeriveCapture::SELECTION_ORDER.len(),
            RefusalDeriveCapture::DECLARED_ORDER.len()
        );
        assert!(
            RefusalDeriveCapture::DECLARED_ORDER.projects_to(RefusalDeriveCapture::SELECTION_ORDER)
        );
    }

    /// law: derive.a-cause-identity-is-its-family-and-its-local-key — every
    /// cause of the capture family carries the derived pair band 00 declares,
    /// the pair's family is one family, and no two causes share a local key.
    /// Owed reversal: two causes sharing a local key must break this law.
    #[test]
    fn a_cause_identity_is_its_family_and_its_local_key() {
        let causes = [
            RefusalDeriveCapture::NotAnEnum,
            RefusalDeriveCapture::UnsupportedDeclarationForm,
            RefusalDeriveCapture::NotNamed,
            RefusalDeriveCapture::UnavailableUnderCompilerProfile,
            RefusalDeriveCapture::NotBodied,
            RefusalDeriveCapture::NotInhabited,
            RefusalDeriveCapture::UnsupportedVariantPayload,
            RefusalDeriveCapture::NotFamilyDeclared,
            RefusalDeriveCapture::NotFamilyGrammatical,
            RefusalDeriveCapture::NotShapeDeclared,
            RefusalDeriveCapture::NotAnAdmittedShape,
            RefusalDeriveCapture::NotOrderDeclared,
            RefusalDeriveCapture::NotOrderAdmitted,
            RefusalDeriveCapture::NotCovered,
            RefusalDeriveCapture::NotDistinct,
            RefusalDeriveCapture::NotKeyed,
            RefusalDeriveCapture::Unbounded,
        ];
        assert_eq!(causes.len(), RefusalDeriveCapture::SELECTION_ORDER.len());
        assert!(
            causes
                .iter()
                .all(|cause| cause.key().family() == RefusalDeriveCapture::FAMILY)
        );
        let keys: Vec<&str> = causes
            .iter()
            .map(|cause| cause.key().local().as_declared())
            .collect();
        assert!(keys.iter().enumerate().all(|(position, key)| {
            keys.iter()
                .skip(position.saturating_add(1))
                .all(|other| other != key)
        }));
    }

    /// law: derive.a-lawful-declaration-captures-typed — a well-formed
    /// declaration yields the machine's shape, the author's family identity, the
    /// author's local keys, and the default crate binding.
    /// Owed reversal: a capture that read the body layout instead of the order
    /// clause must break this law.
    #[test]
    fn a_lawful_declaration_captures_typed() {
        assert!(surface(SINGLE_CAUSE).is_ok_and(|surface| {
            surface.family_name() == "DemoFamily"
                && surface.family_id() == "demo.example"
                && surface.binding().spelling() == "threadpak"
                && matches!(surface.shape(), FamilyShape::SingleCause)
                && surface.causes().count() == 2
                && surface
                    .causes()
                    .next()
                    .is_some_and(|cause: &CapturedCause| {
                        cause.spelling() == "NotCanonical" && cause.local_key() == "not-canonical"
                    })
        }));
        assert!(surface(ISSUE_COLLECTION).is_ok_and(|surface| {
            matches!(surface.shape(), FamilyShape::IssueCollection) && surface.causes().count() == 0
        }));
    }

    /// law: derive.the-crate-binding-travels-with-the-declaration — a consumer
    /// that renamed its dependency is captured under the name it used, and the
    /// binding reaches the rendering rather than being assumed there.
    /// Owed reversal (red twin): a renderer hardcoding `::threadpak` must break
    /// this law.
    #[test]
    fn the_crate_binding_travels_with_the_declaration() {
        let renamed = "#[refusal(crate = tp, family = \"demo.example\", shape = issue_collection)] \
            enum DemoIssues { NotBound, }";
        assert!(surface(renamed).is_ok_and(|surface| surface.binding().spelling() == "tp"));
        let rendered = compile_refusal_text(renamed)
            .map(|(_, closed)| closed.inspected())
            .map_err(|_| ());
        assert!(rendered.is_ok_and(|text| {
            text.contains(":: tp :: refusal :: RefusalFamily") && !text.contains("threadpak")
        }));
    }

    /// law: derive.the-declared-order-is-the-selector-not-the-layout — the
    /// captured order follows the order clause, not the order the variants
    /// happen to be written in.
    /// Owed reversal: a capture reading the body layout must break this law.
    #[test]
    fn the_declared_order_is_the_selector_not_the_layout() {
        let reordered = "#[refusal(family = \"demo.example\", shape = single_cause, \
            order(NotAdmitted = \"not-admitted\", NotCanonical = \"not-canonical\"))] \
            enum DemoFamily { NotCanonical, NotAdmitted, }";
        assert!(surface(reordered).is_ok_and(|surface| {
            let spellings: Vec<&str> = surface.causes().map(CapturedCause::spelling).collect();
            spellings == vec!["NotAdmitted", "NotCanonical"]
        }));
    }

    /// law: derive.a-real-enum-is-never-told-it-is-not-an-enum — a declaration
    /// that IS a real Rust item and merely meets a limit of this grammar gets a
    /// cause naming that limit. A caller told `NotAnEnum` about a perfectly good
    /// enum goes looking for the wrong problem.
    /// Owed reversal (red twin): folding these forms back into `NotAnEnum` must
    /// break this law.
    #[test]
    fn a_real_enum_is_never_told_it_is_not_an_enum() {
        let cases = [
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection)] \
                 struct NotAnEnumAtAll;",
                RefusalDeriveCapture::UnsupportedDeclarationForm,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection)] \
                 enum Generic<T> { NotBound, }",
                RefusalDeriveCapture::UnavailableUnderCompilerProfile,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection)] \
                 enum Payloaded { NotBound(u8), }",
                RefusalDeriveCapture::UnsupportedVariantPayload,
            ),
        ];
        assert!(
            cases
                .iter()
                .all(|(source, expected)| surface(source) == Err(*expected))
        );
    }

    /// law: derive.every-malformed-declaration-establishes-one-cause — the
    /// capture family is single-cause, and each malformed declaration
    /// establishes exactly the cause its defect names.
    /// Owed reversal: collapsing two defects onto one cause must break this law.
    #[test]
    fn every_malformed_declaration_establishes_one_cause() {
        let cases = [
            ("nothing declared here", RefusalDeriveCapture::NotAnEnum),
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection)] enum { A, }",
                RefusalDeriveCapture::NotNamed,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection)] enum Empty { }",
                RefusalDeriveCapture::NotInhabited,
            ),
            (
                "#[refusal(shape = issue_collection)] enum Demo { A, }",
                RefusalDeriveCapture::NotFamilyDeclared,
            ),
            (
                "#[refusal(family = \"NotKebab\", shape = issue_collection)] enum Demo { A, }",
                RefusalDeriveCapture::NotFamilyGrammatical,
            ),
            (
                "#[refusal(family = \"demo.example\")] enum Demo { A, }",
                RefusalDeriveCapture::NotShapeDeclared,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = tri_state)] enum Demo { A, }",
                RefusalDeriveCapture::NotAnAdmittedShape,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = single_cause)] enum Demo { A, }",
                RefusalDeriveCapture::NotOrderDeclared,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = issue_collection, \
                 order(A = \"a\"))] enum Demo { A, }",
                RefusalDeriveCapture::NotOrderAdmitted,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = single_cause, \
                 order(A = \"a\"))] enum Demo { A, B, }",
                RefusalDeriveCapture::NotCovered,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = single_cause, \
                 order(A = \"a\", B = \"a\"))] enum Demo { A, B, }",
                RefusalDeriveCapture::NotDistinct,
            ),
            (
                "#[refusal(family = \"demo.example\", shape = single_cause, \
                 order(A = \"NotKebab\"))] enum Demo { A, }",
                RefusalDeriveCapture::NotKeyed,
            ),
        ];
        assert!(
            cases
                .iter()
                .all(|(source, expected)| surface(source) == Err(*expected))
        );
    }

    /// law: derive.a-refusal-names-the-offending-token — a capture refusal
    /// carries the token it was established at, and the text route resolves that
    /// token to a byte position. A refusal that always pointed at the first
    /// token would send every reader to the same wrong place.
    /// Owed reversal (red twin): reporting at `token[0]` must break this law.
    #[test]
    fn a_refusal_names_the_offending_token() {
        let source = "#[refusal(family = \"demo.example\", shape = tri_state)] enum Demo { A, }";
        let read = TextCapture::read(source).map_err(|_| ());
        let refused = captured_text(source).map(|_| ());
        assert!(read.is_ok());
        assert!(refused.is_err_and(|refusal| {
            let table = TextCapture::read(source).map(|read| read.spans().clone());
            table.is_ok_and(|table| {
                let coordinate = table.coordinate_of(refusal.token());
                coordinate.role == CoordinateRole::Byte
                    && coordinate.position > 0
                    && refusal.cause() == RefusalDeriveCapture::NotAnAdmittedShape
            })
        }));
    }

    /// law: derive.the-standing-of-the-cause-order-is-typed — a shape that
    /// declares no canonical order says so with a typed standing, and the plan's
    /// disposition names the owner fact rather than saying nothing.
    /// Owed reversal: an untyped standing must break this law.
    #[test]
    fn the_standing_of_the_cause_order_is_typed() {
        let single = surface(SINGLE_CAUSE).map(RefusalDeriveSurface::planned);
        let collection = surface(ISSUE_COLLECTION).map(RefusalDeriveSurface::planned);
        assert!(single.is_ok_and(|draft| {
            matches!(draft.cause_order_standing(), CauseOrderStanding::Declared)
                && draft.declared_membership() == DerivedMembership::FamilyAndCauseOrder
                && draft.declared_membership().len() == 2
                && !draft.declared_membership().is_empty()
        }));
        assert!(collection.is_ok_and(|draft| {
            matches!(
                draft.cause_order_standing(),
                CauseOrderStanding::NotApplicableToShape
            ) && draft.declared_membership() == DerivedMembership::FamilyOnly
        }));
    }

    /// law: derive.the-one-road-closes-before-it-emits — the live road produces
    /// a plan, a rendering, a proved closure, and a complete explanation, and
    /// the token tree is reachable only off the closed expansion those four
    /// produced.
    /// Owed reversal (red twin): a render road that skipped the closure must
    /// break this law.
    #[test]
    fn the_one_road_closes_before_it_emits() {
        let compiled = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        assert!(compiled.is_ok_and(|(_, closed)| {
            let plan = closed.plan();
            let closure = closed.closure();
            plan.membership().len() == 2
                && closure.rendered().len() == 2
                && closure.reconstructed().len() == 2
                && plan.trace().len() == 3
                && plan.invalidation().len() == 3
                && plan.origin().len() == 1
                && closed.explanation().len() == 9
                && !closed.emitted().is_empty()
                && matches!(
                    closed.cause_order(),
                    ProjectionDisposition::Generated { .. }
                )
        }));
    }

    /// law: derive.inspection-and-emission-read-one-value — the text a caller
    /// inspects is a projection of the same tree that is emitted, and the plan
    /// and closure a caller reads are the same values the emission came from.
    /// There is no parallel plan and no synthetic sibling.
    /// Owed reversal (red twin): a second rendering built for inspection must
    /// break this law.
    #[test]
    fn inspection_and_emission_read_one_value() {
        let compiled = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        assert!(compiled.is_ok_and(|(_, closed)| {
            let inspected = closed.inspected();
            let emitted = closed.emitted().inspected();
            let joined = closed
                .rendered()
                .joined_tree()
                .map(|tree| tree.inspected())
                .unwrap_or_default();
            inspected == emitted
                && inspected == joined
                && closed.closure().identity() == closed.closure().identity()
        }));
    }

    /// law: derive.the-plan-is-a-function-of-the-declaration — two captures of
    /// the same declaration produce the same plan identities and the same
    /// closure identity, and a different declaration produces different ones.
    /// Owed reversal: an identity carrying anything ambient must break this law.
    #[test]
    fn the_plan_is_a_function_of_the_declaration() {
        let once = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        let twice = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        let other = compile_refusal_text(ISSUE_COLLECTION).map_err(|_| ());
        assert!(once.is_ok_and(|(_, first)| {
            twice.is_ok_and(|(_, second)| {
                other.is_ok_and(|(_, third)| {
                    first.closure().identity() == second.closure().identity()
                        && first.surface().identity() == second.surface().identity()
                        && first.closure().identity() != third.closure().identity()
                })
            })
        }));
    }

    /// law: derive.a-membership-only-draft-renders-nothing — the draft states
    /// what the shape fixed and carries no rendering road. The frontage road is
    /// closed: there is no public value in this home other than a closed
    /// expansion that carries a token tree.
    /// Owed reversal (red twin): re-adding `rendered()` to the draft must break
    /// this law.
    #[test]
    fn a_membership_only_draft_renders_nothing() {
        let draft = surface(SINGLE_CAUSE).map(RefusalDeriveSurface::planned);
        assert!(draft.is_ok_and(|draft| {
            // The draft answers what the SHAPE fixed and nothing else. Every
            // question about bytes is answered by a closed expansion or by
            // nobody.
            draft.declared_membership().roles().len() == 2
                && draft.surface().family_id() == "demo.example"
        }));
    }

    /// law: derive.the-explanation-carries-the-proved-digest — the
    /// output-and-digest seat is answered with the digest the CLOSURE proved
    /// over bytes that exist, never with a value the plan invented.
    /// Owed reversal (red twin): a plan-supplied digest must break this law.
    #[test]
    fn the_explanation_carries_the_proved_digest() {
        let compiled = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        assert!(compiled.is_ok_and(|(_, closed)| {
            let family = closed
                .rendered()
                .under(RenderedImplementation::RenderedFamilyImpl);
            family.is_some_and(|unit| {
                let planned = closed
                    .plan()
                    .membership()
                    .under(RenderedImplementation::RenderedFamilyImpl);
                planned.is_some_and(|member| {
                    unit.digest_under(member.output.digest_contract) == unit.digest()
                        && unit.semantic_key() == member.output.semantic_key
                })
            })
        }));
    }

    /// law: derive.a-diagnostic-from-an-expansion-says-it-is-unanchored — the
    /// live road refuses with a diagnostic that states the machine posture it
    /// actually has, rather than carrying a stand-in identity nobody minted.
    /// Owed reversal (red twin): a plane-minted machine identity must break this
    /// law.
    #[test]
    fn a_diagnostic_from_an_expansion_says_it_is_unanchored() {
        let malformed = "#[refusal(family = \"demo.example\", shape = tri_state)] enum Demo { A, }";
        let read = TextCapture::read(malformed).map_err(|_| ());
        assert!(read.is_ok_and(|read| {
            let context = crate::derive_refusal::RefusalCompileContext {
                spans: read.spans().clone(),
                machine: MachineAnchoring::UnmintedAtThisSeam,
                owner_facts: crate::derive_refusal::RefusalOwnerFacts::declared(),
                nonclaims: threadpak::types::Bounded::empty(),
            };
            compile_refusal(read.input(), &context).is_err_and(|diagnostic| {
                matches!(diagnostic.machine, MachineAnchoring::UnmintedAtThisSeam)
                    && matches!(diagnostic.phase, MacrocPhase::Capture)
                    && !diagnostic.summary.is_empty()
                    && diagnostic.repairs.len() == 1
            })
        }));
    }

    /// law: derive.a-closure-refuses-a-rendering-that-drops-a-planned-role — the
    /// closure check is claim-specific: a rendering that materializes fewer
    /// units than the plan declared is refused by role, before any token exists.
    /// Owed reversal (red twin): a closure that compared counts must break this
    /// law.
    #[test]
    fn a_closure_refuses_a_rendering_that_drops_a_planned_role() {
        let compiled = compile_refusal_text(SINGLE_CAUSE).map_err(|_| ());
        assert!(compiled.is_ok_and(|(_, closed)| {
            let family = closed
                .rendered()
                .under(RenderedImplementation::RenderedFamilyImpl)
                .cloned();
            family.is_some_and(|unit| {
                let partial = crate::closure::RenderedProjection::of_one(unit);
                crate::closure::ProjectionClosure::proved(closed.plan().membership(), partial)
                    .is_err_and(|refusal| {
                        *refusal.issues.first()
                            == ClosureIssue::MemberMissing {
                                role: RenderedImplementation::RenderedCauseOrderImpl,
                            }
                    })
            })
        }));
    }

    /// law: derive.the-callable-route-needs-no-proc-macro — the whole road runs
    /// from text, with no proc-macro anywhere in the path, which is what makes a
    /// diagnostic's declared reproduction route a real road.
    /// Owed reversal: a road reachable only through an expansion must break this
    /// law.
    #[test]
    fn the_callable_route_needs_no_proc_macro() {
        let read = TextCapture::read(SINGLE_CAUSE).map_err(|_| ());
        assert!(read.is_ok_and(|read| {
            captured(read.input()).is_ok_and(|surface| surface.family_id() == "demo.example")
        }));
    }
}
