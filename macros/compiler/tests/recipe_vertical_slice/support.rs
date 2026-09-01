//! Independent mirror projectors and their generated-token helpers.

use macroonz_compiler::recipe::{
    ProjectionError, ProjectionOffered, ProjectionRequest, ProjectionSink, RecipeProjector,
    RecipeRole, RecipeView,
};
use macroonz_compiler::{
    GeneratedDelimiter, GeneratedRowRefusal, GeneratedToken, GeneratedTree, NonEmptyError,
    absolute_path, associated_constant, associated_function, attribute, constant, decorated,
    documentation, enumeration, function_item, function_signature, group, implementation,
    inline_module, keyed_roster_items, match_arm, match_expression, result_type, trait_declaration,
    tuple_struct, typed_parameter, unit_struct, unit_variant, use_item,
};

#[path = "support/fixtures.rs"]
mod fixtures;
pub(super) use fixtures::{
    CALLER_OWNED_TRIAL_RECIPE, COMPANION_RECIPE, COMPLETE_RECIPE, DOOR, EVIDENCE_RECIPE,
    EXACT_DISPATCH_RECIPE, TARGET_UNAVAILABLE_RECIPE,
};

pub(super) struct MirroredCompanions;

pub(super) struct MirroredDispatch;

pub(super) struct MirroredTypestate;

pub(super) struct CallerOwnedTrials;

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

#[path = "support/observe.rs"]
mod observe;
pub(super) use observe::{bake, cargo_bytes, emitted_bytes, refusal_summary};

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
