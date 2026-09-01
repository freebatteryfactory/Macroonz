//! The callable recipe host and the paved wrapper envelope over one informed structural slice.

mod refusal_contract;

use macroonz_compiler::recipe::{
    HarnessPosture, LoweringSource, ProjectionDisposition, ProjectionError, ProjectionOffered,
    ProjectionRequest, ProjectionSink, RecipeBake, RecipeProjector, RecipeRole, RecipeView,
};
use macroonz_compiler::{
    CanonicalContent, CrateBinding, Destination, Door, GeneratedDelimiter, GeneratedRowRefusal,
    GeneratedToken, GeneratedTree, NonEmptyError, Producer, TextCapture, absolute_path,
    associated_constant, associated_function, attribute, constant, decorated, documentation,
    enumeration, function_item, function_signature, group, implementation, inline_module,
    keyed_roster_items, match_arm, match_expression, result_type, trait_declaration, tuple_struct,
    typed_parameter, unit_struct, unit_variant, use_item,
};

const DOOR: Door = Door::declared(
    "recipe-crossing",
    "recipe-crossing.grammar",
    "recipe-crossing::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "recipe-crossing",
        name: "recipe",
    },
);

const COMPLETE_RECIPE: &str = r"
pub mod door {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State {
        Closed,
        Open,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
            dispatch(apply);
            compile_contract;
            property;
            typestate;
        };
        support(door_recipe_support);
    }
}
";

const COMPANION_RECIPE: &str = r"
pub mod door {
    pub enum State {
        Closed,
        Open,
    }

    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
        };
    }
}
";

const EXACT_DISPATCH_RECIPE: &str = r"
pub mod door {
    pub enum State {
        Closed,
        Open,
    }

    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            dispatch {
                #[inline]
                pub fn advance<'a>(
                    current: State,
                    stimulus: Event,
                ) -> Result<State, TransitionRefusal>
                where
                    State: 'a;
            };
        };
    }
}
";

const EVIDENCE_RECIPE: &str = r#"
pub mod door {
    pub enum State {
        Closed,
        Open,
        Locked,
    }

    pub enum Event {
        OpenDoor,
        CloseDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
            (Open, CloseDoor) => Closed with(crate::effects::close);
        };
        absence(refused);
        projections {
            companions;
        };
        evidence {
            trials {
                support = recipe_trials_support,
                module = recipe_trials,
                table = named("recipe", "trial-table"),
                suite checks = named("recipe", "unit") {
                    transition_answers {
                        claim = named("recipe", "transition-answers"),
                        subject = named("recipe", "dispatch"),
                        check = named("recipe", "exact"),
                        population = named("recipe", "declared-rows"),
                    },
                },
            };
            mutation(states) {
                module = recipe_mutations,
                refusal = RecipeMutationRefusal,
                support = recipe_mutation_support,
                family = named("recipe", "refusals"),
                point = named("recipe", "state-order"),
                fact = named("recipe", "state-order"),
                map named("recipe", "state-order") = named("recipe", "order-held"),
                permit named("recipe", "order-held") = ["declared-order-permutation"],
            };
            benchmarks {
                support = recipe_bench_support,
                table_function = recipe_bench_table,
                table = named("recipe", "bench-table"),
                reporter = recipe_bench_reporter,
                dispatch_pace {
                    workload = named("recipe", "dispatch"),
                    preflight = named("recipe", "dispatch-correct"),
                    planted_worse = named("recipe", "dispatch-worse"),
                    complexity = named("recipe", "linear"),
                    axis = [2, 4, 8],
                    samples = 16,
                    warmups = 4,
                    ratio_numerator = 3,
                    ratio_denominator = 1,
                    observe = [named("recipe", "rows-touched")],
                },
            };
            network {
                harness = renamed_facade::harness,
                module = recipe_network,
                namespace = "recipe",
                nodes = [client, server],
                link forward = client to server,
                schedule quiet = [],
            };
            concurrency {
                harness = renamed_facade::harness,
                module = recipe_concurrency,
                namespace = "recipe",
                transitions_hold {
                    population = "transition-orders",
                    interleavings = 16,
                    samples = 32,
                    seed = 11,
                },
            };
        };
    }
}
"#;

const TARGET_UNAVAILABLE_RECIPE: &str = r"
pub mod door {
    pub enum State {
        Closed,
        Open,
    }

    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
        };
        evidence {
            trials unavailable;
        };
    }
}
";

const CALLER_OWNED_TRIAL_RECIPE: &str = r"
pub mod door {
    pub enum State {
        Closed,
        Open,
    }

    pub enum Event {
        OpenDoor,
    }

    bake! {
        vocabularies(State, Event);
        transitions {
            (Closed, OpenDoor) => Open with(crate::effects::open);
        };
        absence(refused);
        projections {
            companions;
        };
        evidence {
            trials {
                this is intentionally not the descriptor trial grammar
            };
        };
    }
}
";

struct MirroredCompanions;

struct MirroredDispatch;

struct MirroredTypestate;

struct CallerOwnedTrials;

impl RecipeProjector for CallerOwnedTrials {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        assert_eq!(request.role(), RecipeRole::Trials);
        let evidence =
            view.recipe()
                .evidence(RecipeRole::Trials)
                .ok_or(ProjectionError::Render(
                    macroonz_compiler::RenderError::NothingRendered,
                ))?;
        assert!(!evidence.body().trees().is_empty());
        sink.offer(GeneratedTree::assembled(unit_struct(
            GeneratedToken::word("CallerOwnedTrials"),
            Vec::new(),
            Vec::new(),
        ))?)
    }
}

impl RecipeProjector for MirroredCompanions {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        assert_eq!(request.role(), RecipeRole::Companions);
        assert_eq!(request.effective().role(), RecipeRole::Companions);
        let recipe = view.recipe();
        let mut tokens = roster_constant(
            "STATE_VARIANTS",
            "The state variants in caller-authored order.",
            recipe.states_name_token(),
            recipe.states().members(),
        )?;
        tokens.extend(roster_constant(
            "EVENT_VARIANTS",
            "The event variants in caller-authored order.",
            recipe.events_name_token(),
            recipe.events().members(),
        )?);
        tokens.extend(transition_constant(recipe)?);
        sink.offer(GeneratedTree::assembled(tokens)?)
    }
}

impl RecipeProjector for MirroredDispatch {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        assert_eq!(request.role(), RecipeRole::Dispatch);
        assert_eq!(request.effective().role(), RecipeRole::Dispatch);
        let recipe = view.recipe();
        let refusal = decorated(
            vec![
                documentation("Why generated dispatch did not find an admitted transition row.")?,
                derived(&["Debug", "Clone", "Copy", "PartialEq", "Eq"])?,
            ],
            public(),
            enumeration(
                GeneratedToken::word("TransitionRefusal"),
                Vec::new(),
                Vec::new(),
                vec![decorated(
                    vec![documentation(
                        "No declared transition occupies the supplied state and event seat.",
                    )?],
                    Vec::new(),
                    unit_variant(GeneratedToken::word("Absent")),
                )],
            )?,
        );
        let parameters = vec![
            typed_parameter(
                vec![GeneratedToken::word("state")],
                super_path(recipe.states_name_token()),
            ),
            typed_parameter(
                vec![GeneratedToken::word("event")],
                super_path(recipe.events_name_token()),
            ),
        ];
        let result = result_type(
            super_path(recipe.states_name_token()),
            vec![GeneratedToken::word("TransitionRefusal")],
        );
        let mut arms = recipe
            .transitions()
            .members()
            .map(|transition| dispatch_arm(recipe, transition))
            .collect::<Result<Vec<_>, _>>()?;
        arms.push(match_arm(
            vec![GeneratedToken::word("_")],
            None,
            vec![
                GeneratedToken::word("Err"),
                group(
                    GeneratedDelimiter::Parenthesis,
                    vec![
                        GeneratedToken::word("TransitionRefusal"),
                        GeneratedToken::joint(':'),
                        GeneratedToken::alone(':'),
                        GeneratedToken::word("Absent"),
                    ],
                )?,
            ],
        ));
        let body = match_expression(
            vec![group(
                GeneratedDelimiter::Parenthesis,
                comma_separated(vec![
                    vec![GeneratedToken::word("state")],
                    vec![GeneratedToken::word("event")],
                ]),
            )?],
            arms,
        )?;
        let function = decorated(
            vec![documentation(
                "Applies one declared transition or returns typed absence.",
            )?],
            public(),
            function_item(
                function_signature(
                    Vec::new(),
                    GeneratedToken::word(request.effective().name().unwrap_or("apply")),
                    parameters,
                    Vec::new(),
                    Some(result),
                    Vec::new(),
                )?,
                body,
            )?,
        );
        let mut tokens = refusal;
        tokens.extend(function);
        sink.offer(GeneratedTree::assembled(tokens)?)
    }
}

impl RecipeProjector for MirroredTypestate {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        assert_eq!(request.role(), RecipeRole::Typestate);
        let mut items = use_item(absolute_path(&["core", "marker", "PhantomData"]), None);
        items.extend(mirrored_stage_trait()?);
        items.extend(
            keyed_roster_items(view.recipe().states(), |_position, spelling, member| {
                mirrored_stage_member(spelling, member)
            })
            .map_err(mirrored_row_projection_error)?,
        );
        items.extend(decorated(
            vec![
                documentation("A type-level carrier over one caller-declared stage.")?,
                derived(&["Debug", "Clone", "Copy", "PartialEq", "Eq", "Hash"])?,
            ],
            public(),
            tuple_struct(
                GeneratedToken::word("Stage"),
                vec![vec![GeneratedToken::word("Marker")]],
                vec![decorated(
                    Vec::new(),
                    public(),
                    vec![
                        GeneratedToken::word("PhantomData"),
                        GeneratedToken::alone('<'),
                        GeneratedToken::word("Marker"),
                        GeneratedToken::alone('>'),
                    ],
                )],
                Vec::new(),
            )?,
        ));
        items.extend(mirrored_stage_inherent()?);
        items.extend(mirrored_stage_default()?);
        let projected = decorated(
            vec![documentation(
                "Type-level stages derived from the caller-authored state vocabulary.",
            )?],
            public(),
            inline_module(GeneratedToken::word("typestate"), items)?,
        );
        sink.offer(GeneratedTree::assembled(projected)?)
    }
}

#[test]
fn the_callable_and_wrapper_hosts_emit_one_canonical_projection() -> Result<(), ()> {
    let callable = bake(COMPLETE_RECIPE)?;
    let wrapped_source =
        format!("{{ macroonz }} __macroonz_test_carrier_available {{ {COMPLETE_RECIPE} }}");
    let wrapped_capture = TextCapture::read(&wrapped_source).map_err(|_| ())?;
    let wrapped =
        macroonz_compiler::recipe::bake_wrapped(wrapped_capture.input(), &DOOR).map_err(|_| ())?;

    for destination in [
        Destination::DeclarationSite,
        Destination::TestCarrier,
        Destination::BenchCarrier,
    ] {
        assert_eq!(
            cargo_bytes(callable.projection(), destination),
            cargo_bytes(wrapped.projection(), destination),
            "the two hosts disagreed under {}",
            destination.name()
        );
    }
    assert_eq!(
        callable
            .projection()
            .plan()
            .content()
            .canonical_content_bytes(),
        wrapped
            .projection()
            .plan()
            .content()
            .canonical_content_bytes()
    );
    Ok(())
}

#[test]
fn the_wrapper_carries_the_no_harness_posture_without_changing_the_recipe() -> Result<(), ()> {
    let callable_capture = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let callable = macroonz_compiler::recipe::bake(
        callable_capture.input(),
        HarnessPosture::Unavailable,
        &DOOR,
    )
    .map_err(|_| ())?;
    let wrapped_source =
        format!("{{ macroonz }} __macroonz_test_carrier_unavailable {{ {COMPANION_RECIPE} }}");
    let wrapped_capture = TextCapture::read(&wrapped_source).map_err(|_| ())?;
    let wrapped =
        macroonz_compiler::recipe::bake_wrapped(wrapped_capture.input(), &DOOR).map_err(|_| ())?;

    assert_eq!(
        callable.projection().identity(),
        wrapped.projection().identity()
    );
    assert_eq!(emitted_bytes(&callable), emitted_bytes(&wrapped));
    Ok(())
}

#[test]
fn a_caller_owned_projector_has_the_standard_clients_exact_authority() -> Result<(), ()> {
    let read = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Companions,
        &MirroredCompanions,
    )
    .map_err(|_| ())?;

    assert_eq!(
        standard.projection().identity(),
        custom.projection().identity()
    );
    assert_eq!(
        standard.projection().plan().identity(),
        custom.projection().plan().identity()
    );
    assert_eq!(
        standard.projection().closure().identity(),
        custom.projection().closure().identity()
    );
    assert_eq!(
        standard.projection().explain().identity(),
        custom.projection().explain().identity()
    );
    assert_eq!(emitted_bytes(&standard), emitted_bytes(&custom));
    Ok(())
}

#[test]
fn a_caller_owned_dispatch_projector_uses_the_same_behavior_kernel_and_authority() -> Result<(), ()>
{
    let source = COMPANION_RECIPE.replace("companions;", "dispatch(apply);");
    let read = TextCapture::read(&source).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Dispatch,
        &MirroredDispatch,
    )
    .map_err(|_| ())?;

    assert_eq!(
        standard.projection().identity(),
        custom.projection().identity()
    );
    assert_eq!(
        standard.projection().plan().identity(),
        custom.projection().plan().identity()
    );
    assert_eq!(
        standard.projection().closure().identity(),
        custom.projection().closure().identity()
    );
    assert_eq!(
        standard.projection().explain().identity(),
        custom.projection().explain().identity()
    );
    assert_eq!(emitted_bytes(&standard), emitted_bytes(&custom));
    Ok(())
}

#[test]
fn dispatch_discloses_preset_configuration_and_exact_rust_on_one_seat() -> Result<(), ()> {
    let preset_source = COMPANION_RECIPE.replace("companions;", "dispatch;");
    let preset = bake(&preset_source)?;
    let preset_effective = preset
        .projection()
        .plan()
        .content()
        .effective(RecipeRole::Dispatch)
        .ok_or(())?;
    assert_eq!(preset_effective.source(), LoweringSource::Preset);
    assert_eq!(preset_effective.name(), None);
    assert_eq!(preset_effective.exact_rust(), None);

    let configured_source = COMPANION_RECIPE.replace("companions;", "dispatch(apply);");
    let configured = bake(&configured_source)?;
    let configured_effective = configured
        .projection()
        .plan()
        .content()
        .effective(RecipeRole::Dispatch)
        .ok_or(())?;
    assert_eq!(configured_effective.source(), LoweringSource::Configuration);
    assert_eq!(configured_effective.name(), Some("apply"));
    assert_eq!(configured_effective.exact_rust(), None);

    let exact = bake(EXACT_DISPATCH_RECIPE)?;
    let exact_effective = exact
        .projection()
        .plan()
        .content()
        .effective(RecipeRole::Dispatch)
        .ok_or(())?;
    assert_eq!(exact_effective.source(), LoweringSource::ExactRust);
    assert_eq!(exact_effective.name(), Some("advance"));
    let exact_readback = exact_effective
        .exact_rust()
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    for fragment in [
        "# [ inline ]",
        "pub fn advance < 'a >",
        "current : State",
        "stimulus : Event",
        "where State : 'a",
    ] {
        assert!(
            exact_readback.contains(fragment),
            "the exact readback omitted {fragment}: {exact_readback}"
        );
    }
    let emitted = exact
        .emit()
        .tokens()
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    assert!(emitted.contains("use super :: State"));
    assert!(emitted.contains("use super :: Event"));
    assert!(
        emitted.contains("match ( current , stimulus )"),
        "the generated body did not use the exact bindings: {emitted}"
    );
    assert!(emitted.contains("pub fn advance < 'a >"));
    Ok(())
}

#[test]
fn exact_dispatch_signature_material_moves_recipe_identity() -> Result<(), ()> {
    let first = bake(EXACT_DISPATCH_RECIPE)?;
    let changed = EXACT_DISPATCH_RECIPE.replace("current: State", "source: State");
    let second = bake(&changed)?;

    assert_ne!(
        first.projection().plan().identity(),
        second.projection().plan().identity()
    );
    assert_ne!(
        first.projection().identity(),
        second.projection().identity()
    );
    assert_ne!(emitted_bytes(&first), emitted_bytes(&second));
    Ok(())
}

#[test]
fn commas_inside_exact_parameter_types_do_not_invent_parameter_rows() -> Result<(), ()> {
    let nested_type = EXACT_DISPATCH_RECIPE.replace(
        "current: State",
        "current: core::result::Result<State, TransitionRefusal>",
    );
    let baked = bake(&nested_type)?;
    let exact = baked
        .projection()
        .plan()
        .content()
        .effective(RecipeRole::Dispatch)
        .and_then(|effective| effective.exact_rust())
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    assert!(exact.contains("Result < State , TransitionRefusal >"));
    Ok(())
}

#[test]
fn fully_qualified_exact_types_do_not_emit_unneeded_vocabulary_imports() -> Result<(), ()> {
    let qualified = EXACT_DISPATCH_RECIPE
        .replace("current: State", "current: crate::door::State")
        .replace("stimulus: Event", "stimulus: crate::door::Event")
        .replace(
            "Result<State, TransitionRefusal>",
            "Result<crate::door::State, TransitionRefusal>",
        )
        .replace("State: 'a", "crate::door::State: 'a");
    let baked = bake(&qualified)?;
    let emitted = baked
        .emit()
        .tokens()
        .map(GeneratedTree::inspected)
        .ok_or(())?;

    assert!(!emitted.contains("use super :: State"), "{emitted}");
    assert!(!emitted.contains("use super :: Event"), "{emitted}");
    Ok(())
}

#[test]
fn exact_dispatch_refusals_name_the_owned_repair() -> Result<(), ()> {
    let not_function = EXACT_DISPATCH_RECIPE.replace(
        "#[inline]\n                pub fn advance<'a>(\n                    current: State,\n                    stimulus: Event,\n                ) -> Result<State, TransitionRefusal>\n                where\n                    State: 'a;",
        "pub const ADVANCE: usize = 1;",
    );
    assert!(refusal_summary(&not_function)?.contains(
        "exact dispatch braces must contain one semicolon-terminated Rust function signature"
    ));

    let with_body = EXACT_DISPATCH_RECIPE.replace("State: 'a;", "State: 'a { unreachable!() }");
    assert!(
        refusal_summary(&with_body)?.contains("exact dispatch cannot carry a caller-authored body")
    );

    let one_parameter = EXACT_DISPATCH_RECIPE.replace(
        "                    current: State,\n                    stimulus: Event,",
        "                    current: State,",
    );
    assert!(
        refusal_summary(&one_parameter)?
            .contains("exact dispatch requires two parameters but the signature states 1")
    );

    let missing_type = EXACT_DISPATCH_RECIPE.replace("current: State", "current:");
    assert!(
        refusal_summary(&missing_type)?
            .contains("exact dispatch parameter 1 must use one simple identifier binding")
    );

    let pattern = EXACT_DISPATCH_RECIPE.replace("current: State", "(current, _): (State, State)");
    let read = TextCapture::read(&pattern).map_err(|_| ())?;
    let refusal = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .err()
        .ok_or(())?;
    assert!(
        refusal
            .summary()
            .contains("exact dispatch parameter 1 must use one simple identifier binding")
    );
    let repair = refusal.repairs().first().ok_or(())?;
    assert!(
        repair
            .description
            .shown()
            .contains("write `dispatch { fn apply")
    );
    Ok(())
}

#[test]
fn a_caller_owned_typestate_projector_uses_the_same_item_kernel_and_authority() -> Result<(), ()> {
    let source = COMPANION_RECIPE.replace("companions;", "typestate;");
    let read = TextCapture::read(&source).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Typestate,
        &MirroredTypestate,
    )
    .map_err(|_| ())?;

    assert_eq!(
        standard.projection().identity(),
        custom.projection().identity()
    );
    assert_eq!(
        standard.projection().closure().identity(),
        custom.projection().closure().identity()
    );
    assert_eq!(emitted_bytes(&standard), emitted_bytes(&custom));
    Ok(())
}

#[test]
fn descriptor_native_evidence_uses_the_recipe_account_and_existing_carrier_roads() -> Result<(), ()>
{
    let baked = bake(EVIDENCE_RECIPE)?;
    let recipe = baked.projection().plan().content();
    for role in [
        RecipeRole::Trials,
        RecipeRole::Mutation,
        RecipeRole::Benchmarks,
        RecipeRole::Network,
        RecipeRole::Concurrency,
    ] {
        assert_eq!(
            recipe.projection_disposition(role),
            ProjectionDisposition::Generated,
            "{} did not enter the recipe projection account",
            role.name()
        );
        assert!(recipe.evidence(role).is_some());
    }
    for role in [RecipeRole::CompileContract, RecipeRole::Property] {
        assert_eq!(
            recipe.projection_disposition(role),
            ProjectionDisposition::NotRequested
        );
    }
    let text = baked
        .emit()
        .tokens()
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    for spelling in [
        "recipe_trials_support",
        "recipe_mutation_support",
        "recipe_bench_support",
        "pub mod recipe_network",
        "pub mod recipe_concurrency",
        "Closed",
        "Open",
        "Locked",
    ] {
        assert!(text.contains(spelling), "the bake omitted {spelling}");
    }
    assert!(text.contains("macro_rules"));
    assert!(text.contains("declared-order-permutation"));
    assert!(text.contains(":: renamed_facade :: harness"));
    Ok(())
}

#[test]
fn evidence_movement_moves_the_existing_recipe_identity_chain() -> Result<(), ()> {
    let first = bake(EVIDENCE_RECIPE)?;
    let changed = EVIDENCE_RECIPE.replace("samples = 32", "samples = 33");
    let second = bake(&changed)?;

    assert_ne!(
        first.projection().plan().identity(),
        second.projection().plan().identity()
    );
    assert_ne!(
        first.projection().closure().identity(),
        second.projection().closure().identity()
    );
    assert_ne!(
        first.projection().identity(),
        second.projection().identity()
    );
    assert_ne!(emitted_bytes(&first), emitted_bytes(&second));
    Ok(())
}

#[test]
fn target_unavailability_and_feature_unavailability_remain_distinct() -> Result<(), ()> {
    let available = TextCapture::read(TARGET_UNAVAILABLE_RECIPE).map_err(|_| ())?;
    let target_unavailable =
        macroonz_compiler::recipe::bake(available.input(), HarnessPosture::Available, &DOOR)
            .map_err(|_| ())?;
    assert_eq!(
        target_unavailable
            .projection()
            .plan()
            .content()
            .projection_disposition(RecipeRole::Trials),
        ProjectionDisposition::TargetUnavailable
    );

    let unavailable = TextCapture::read(TARGET_UNAVAILABLE_RECIPE).map_err(|_| ())?;
    let feature_unavailable =
        macroonz_compiler::recipe::bake(unavailable.input(), HarnessPosture::Unavailable, &DOOR)
            .map_err(|_| ())?;
    assert_eq!(
        feature_unavailable
            .projection()
            .plan()
            .content()
            .projection_disposition(RecipeRole::Trials),
        ProjectionDisposition::FeatureUnavailable
    );
    Ok(())
}

#[test]
fn generated_evidence_refuses_without_the_harness_before_any_projector_runs() -> Result<(), ()> {
    let read = TextCapture::read(EVIDENCE_RECIPE).map_err(|_| ())?;
    let refusal = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Unavailable, &DOOR)
        .err()
        .ok_or(())?;
    assert!(
        refusal
            .summary()
            .contains("projection `trials` requires the facade harness feature")
    );
    Ok(())
}

#[test]
fn either_harness_projection_requires_one_declared_support_address() -> Result<(), ()> {
    for role in ["compile_contract", "property"] {
        let source = COMPANION_RECIPE.replace("companions", role);
        let summary = refusal_summary(&source)?;
        assert!(
            summary.contains("support address"),
            "{role} did not require its support address: {summary}"
        );
    }
    Ok(())
}

#[test]
fn a_caller_owned_evidence_projector_uses_the_common_sink_without_standard_privilege()
-> Result<(), ()> {
    let read = TextCapture::read(CALLER_OWNED_TRIAL_RECIPE).map_err(|_| ())?;
    let standard_refusal =
        macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
            .err()
            .ok_or(())?;
    assert_eq!(standard_refusal.phase(), macroonz_compiler::Phase::Capture);

    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Trials,
        &CallerOwnedTrials,
    )
    .map_err(|_| ())?;
    let text = custom
        .projection()
        .emit()
        .tokens()
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    assert!(text.contains("CallerOwnedTrials"));
    assert!(!text.contains("recipe_trials_support"));
    Ok(())
}

#[test]
fn a_caller_owned_projector_cannot_replace_an_unselected_role() -> Result<(), ()> {
    let read = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let refusal = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Dispatch,
        &MirroredCompanions,
    )
    .err()
    .ok_or(())?;
    assert!(refusal.summary().contains("unselected role `dispatch`"));
    Ok(())
}

#[test]
fn semantic_effect_movement_moves_identity_even_when_rendered_companions_do_not() -> Result<(), ()>
{
    let first = bake(COMPANION_RECIPE)?;
    let changed = COMPANION_RECIPE.replace("crate::effects::open", "crate::effects::unlock");
    let second = bake(&changed)?;

    assert_eq!(emitted_bytes(&first), emitted_bytes(&second));
    assert_ne!(
        first
            .projection()
            .plan()
            .content()
            .canonical_content_bytes(),
        second
            .projection()
            .plan()
            .content()
            .canonical_content_bytes()
    );
    assert_ne!(
        first.projection().plan().identity(),
        second.projection().plan().identity()
    );
    assert_ne!(
        first.projection().identity(),
        second.projection().identity()
    );
    Ok(())
}

#[test]
fn every_direct_type_namespace_collision_refuses_before_projection() -> Result<(), ()> {
    for declaration in [
        "mod baked {}",
        "struct baked;",
        "enum baked {}",
        "union baked { value: u8 }",
        "trait baked {}",
        "type baked = ();",
        "extern crate baked;",
        "struct r#baked;",
    ] {
        let source = format!(
            r"
pub mod door {{
    {declaration}

    pub enum State {{
        Closed,
        Open,
    }}

    pub enum Event {{
        OpenDoor,
    }}

    bake! {{
        vocabularies(State, Event);
        transitions {{
            (Closed, OpenDoor) => Open with(crate::effects::open);
        }};
        absence(refused);
        projections {{
            companions;
        }};
    }}
}}
"
        );
        let read = TextCapture::read(&source).map_err(|_| ())?;
        let refusal =
            macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
                .err()
                .ok_or(())?;
        assert!(refusal.summary().contains("generated recipe name `baked`"));
    }
    Ok(())
}

#[test]
fn every_recipe_owned_generated_name_collision_refuses_before_rendering() -> Result<(), ()> {
    let doubled_support = EVIDENCE_RECIPE.replace(
        "support = recipe_mutation_support",
        "support = recipe_trials_support",
    );
    assert!(
        refusal_summary(&doubled_support)?
            .contains("generated recipe name `recipe_trials_support` is already occupied")
    );

    let doubled_module =
        EVIDENCE_RECIPE.replace("module = recipe_network", "module = recipe_concurrency");
    assert!(
        refusal_summary(&doubled_module)?
            .contains("generated recipe name `recipe_concurrency` is already occupied")
    );

    let reserved_companion = COMPLETE_RECIPE.replace("dispatch(apply)", "dispatch(STATE_VARIANTS)");
    assert!(
        refusal_summary(&reserved_companion)?
            .contains("generated recipe name `STATE_VARIANTS` is already occupied")
    );
    Ok(())
}

#[test]
fn malformed_behavior_structure_refuses_before_projection() -> Result<(), ()> {
    let widened = COMPLETE_RECIPE.replace(
        "(Closed, OpenDoor) => Open with(crate::effects::open);",
        "(Missing, OpenDoor) => Open with(crate::effects::open);",
    );
    assert!(
        refusal_summary(&widened)?
            .contains("a transition names undeclared `State` member `Missing`")
    );

    let doubled_row = COMPLETE_RECIPE.replace(
        "(Closed, OpenDoor) => Open with(crate::effects::open);",
        "(Closed, OpenDoor) => Open with(crate::effects::open);\n            (Closed, OpenDoor) => Open with(crate::effects::open);",
    );
    assert!(
        refusal_summary(&doubled_row)?
            .contains("more than one transition occupies state `Closed` and event `OpenDoor`")
    );

    let doubled_posture = COMPLETE_RECIPE.replace(
        "absence(refused);",
        "absence(refused);\n        absence(refused);",
    );
    assert!(refusal_summary(&doubled_posture)?.contains("recipe grammar was not read"));
    Ok(())
}

fn bake(source: &str) -> Result<RecipeBake, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR).map_err(|_| ())
}

fn refusal_summary(source: &str) -> Result<String, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .err()
        .map(|refusal| refusal.summary().to_owned())
        .ok_or(())
}

fn cargo_bytes(
    expansion: &macroonz_compiler::Expansion<macroonz_compiler::recipe::RecipeProjection>,
    destination: Destination,
) -> Option<Vec<u8>> {
    expansion
        .emission()
        .joined(destination)
        .and_then(macroonz_compiler::PartitionCargo::tokens)
        .map(GeneratedTree::canonical_bytes)
}

fn emitted_bytes(bake: &RecipeBake) -> Option<Vec<u8>> {
    bake.emit().tokens().map(GeneratedTree::canonical_bytes)
}

fn roster_constant<'name>(
    constant_name: &str,
    sentence: &str,
    vocabulary: &GeneratedToken,
    members: impl Iterator<Item = &'name macroonz_compiler::recipe::RecipeMember>,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut kind = vec![GeneratedToken::alone('&')];
    kind.push(group(GeneratedDelimiter::Bracket, super_path(vocabulary))?);
    let mut value = vec![GeneratedToken::alone('&')];
    value.push(group(
        GeneratedDelimiter::Bracket,
        comma_separated(
            members
                .map(|member| variant(vocabulary, member.name_token()))
                .collect(),
        ),
    )?);
    Ok(decorated(
        vec![documentation(sentence)?],
        public(),
        constant(constant_name, kind, value),
    ))
}

fn transition_constant(
    recipe: &macroonz_compiler::recipe::Recipe,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut kind = vec![GeneratedToken::alone('&')];
    kind.push(group(
        GeneratedDelimiter::Bracket,
        vec![group(
            GeneratedDelimiter::Parenthesis,
            comma_separated(vec![
                super_path(recipe.states_name_token()),
                super_path(recipe.events_name_token()),
                super_path(recipe.states_name_token()),
            ]),
        )?],
    )?);
    let rows = recipe
        .transitions()
        .members()
        .map(|transition| {
            group(
                GeneratedDelimiter::Parenthesis,
                comma_separated(vec![
                    variant(recipe.states_name_token(), transition.source_name_token()),
                    variant(recipe.events_name_token(), transition.event_name_token()),
                    variant(recipe.states_name_token(), transition.target_name_token()),
                ]),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut separated = Vec::new();
    for row in rows {
        separated.extend([row, GeneratedToken::alone(',')]);
    }
    let value = vec![
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Bracket, separated)?,
    ];
    Ok(decorated(
        vec![documentation(
            "The informed transition rows in caller-authored order.",
        )?],
        public(),
        constant("TRANSITIONS", kind, value),
    ))
}

fn dispatch_arm(
    recipe: &macroonz_compiler::recipe::Recipe,
    transition: &macroonz_compiler::recipe::RecipeTransition,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let pattern = group(
        GeneratedDelimiter::Parenthesis,
        comma_separated(vec![
            variant(recipe.states_name_token(), transition.source_name_token()),
            variant(recipe.events_name_token(), transition.event_name_token()),
        ]),
    )?;
    let mut body = transition.effect().tokens().to_vec();
    body.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    body.push(GeneratedToken::alone(';'));
    body.extend([
        GeneratedToken::word("Ok"),
        group(
            GeneratedDelimiter::Parenthesis,
            variant(recipe.states_name_token(), transition.target_name_token()),
        )?,
    ]);
    Ok(match_arm(
        vec![pattern],
        None,
        vec![group(GeneratedDelimiter::Brace, body)?],
    ))
}

fn super_path(name: &GeneratedToken) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("super"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        name.clone(),
    ]
}

fn variant(vocabulary: &GeneratedToken, member: &GeneratedToken) -> Vec<GeneratedToken> {
    let mut tokens = super_path(vocabulary);
    tokens.extend([
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        member.clone(),
    ]);
    tokens
}

fn mirrored_stage_trait() -> Result<Vec<GeneratedToken>, ProjectionError> {
    let name = associated_constant(GeneratedToken::word("NAME"), mirrored_static_str(), None);
    Ok(decorated(
        vec![documentation(
            "One caller-declared member admitted as a typestate stage.",
        )?],
        public(),
        trait_declaration(
            Vec::new(),
            GeneratedToken::word("RecipeStage"),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            decorated(
                vec![documentation("The caller-authored stage spelling.")?],
                Vec::new(),
                name,
            ),
        )?,
    ))
}

fn mirrored_stage_member(
    spelling: &str,
    member: &macroonz_compiler::recipe::RecipeMember,
) -> Result<Vec<GeneratedToken>, macroonz_compiler::Overflow> {
    let mut tokens = decorated(
        vec![
            documentation("One caller-declared typestate stage.")?,
            derived(&["Debug", "Clone", "Copy", "PartialEq", "Eq", "Hash"])?,
        ],
        public(),
        unit_struct(member.name_token().clone(), Vec::new(), Vec::new()),
    );
    tokens.extend(implementation(
        Vec::new(),
        Vec::new(),
        Some(vec![GeneratedToken::word("RecipeStage")]),
        vec![member.name_token().clone()],
        Vec::new(),
        associated_constant(
            GeneratedToken::word("NAME"),
            mirrored_static_str(),
            Some(vec![GeneratedToken::text(spelling)]),
        ),
    )?);
    Ok(tokens)
}

fn mirrored_stage_inherent() -> Result<Vec<GeneratedToken>, ProjectionError> {
    let constructor = decorated(
        vec![documentation("Constructs the zero-sized stage carrier.")?],
        public(),
        associated_function(
            function_signature(
                vec![GeneratedToken::word("const")],
                GeneratedToken::word("new"),
                Vec::new(),
                Vec::new(),
                Some(vec![GeneratedToken::word("Self")]),
                Vec::new(),
            )?,
            Some(vec![
                GeneratedToken::word("Self"),
                group(
                    GeneratedDelimiter::Parenthesis,
                    vec![GeneratedToken::word("PhantomData")],
                )?,
            ]),
        )?,
    );
    implementation(
        Vec::new(),
        vec![vec![GeneratedToken::word("Marker")]],
        None,
        mirrored_generic_stage(),
        Vec::new(),
        constructor,
    )
    .map_err(ProjectionError::Tokens)
}

fn mirrored_stage_default() -> Result<Vec<GeneratedToken>, ProjectionError> {
    let body = vec![
        GeneratedToken::word("Self"),
        group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::word("PhantomData")],
        )?,
    ];
    let function = associated_function(
        function_signature(
            Vec::new(),
            GeneratedToken::word("default"),
            Vec::new(),
            Vec::new(),
            Some(vec![GeneratedToken::word("Self")]),
            Vec::new(),
        )?,
        Some(body),
    )?;
    implementation(
        Vec::new(),
        vec![vec![GeneratedToken::word("Marker")]],
        Some(absolute_path(&["core", "default", "Default"])),
        mirrored_generic_stage(),
        Vec::new(),
        function,
    )
    .map_err(ProjectionError::Tokens)
}

fn mirrored_generic_stage() -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("Stage"),
        GeneratedToken::alone('<'),
        GeneratedToken::word("Marker"),
        GeneratedToken::alone('>'),
    ]
}

fn mirrored_static_str() -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::alone('&'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("static"),
        GeneratedToken::word("str"),
    ]
}

fn mirrored_row_projection_error(refusal: GeneratedRowRefusal) -> ProjectionError {
    match refusal.cause() {
        NonEmptyError::Empty(_) => {
            ProjectionError::Render(macroonz_compiler::RenderError::NothingRendered)
        }
        NonEmptyError::Overflow(cause) => ProjectionError::Tokens(cause),
    }
}

fn comma_separated(parts: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    for (position, part) in parts.into_iter().enumerate() {
        if position > 0 {
            tokens.push(GeneratedToken::alone(','));
        }
        tokens.extend(part);
    }
    tokens
}

fn public() -> Vec<GeneratedToken> {
    vec![GeneratedToken::word("pub")]
}

fn derived(names: &[&str]) -> Result<Vec<GeneratedToken>, macroonz_compiler::Overflow> {
    attribute(vec![
        GeneratedToken::word("derive"),
        group(
            GeneratedDelimiter::Parenthesis,
            comma_separated(
                names
                    .iter()
                    .map(|name| vec![GeneratedToken::word(name)])
                    .collect(),
            ),
        )?,
    ])
}
