//! The callable recipe host and the paved wrapper envelope over one informed structural slice.

use macroonz_compiler::recipe::{
    HarnessPosture, ProjectionError, ProjectionOffered, ProjectionRequest, ProjectionSink,
    RecipeBake, RecipeProjector, RecipeRole, RecipeView,
};
use macroonz_compiler::{
    CanonicalContent, CrateBinding, Destination, Door, GeneratedDelimiter, GeneratedToken,
    GeneratedTree, Producer, TextCapture, absolute_path, attribute, constant, decorated,
    documentation, enumeration, function_item, function_signature, group, inline_module, match_arm,
    match_expression, result_type, tuple_struct, typed_parameter, unit_struct, unit_variant,
    use_item,
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

struct MirroredCompanions;

struct MirroredDispatch;

struct MirroredTypestate;

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
        for member in view.recipe().states().members() {
            items.extend(decorated(
                vec![
                    documentation("One caller-declared typestate stage.")?,
                    derived(&["Debug", "Clone", "Copy", "PartialEq", "Eq", "Hash"])?,
                ],
                public(),
                unit_struct(member.name_token().clone(), Vec::new(), Vec::new()),
            ));
        }
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
        assert!(refusal.summary().contains("generated name `baked`"));
    }
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

fn derived(names: &[&str]) -> Result<Vec<GeneratedToken>, ProjectionError> {
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
    .map_err(ProjectionError::Tokens)
}
