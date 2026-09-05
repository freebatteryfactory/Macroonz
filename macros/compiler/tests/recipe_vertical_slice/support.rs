//! Independent mirror projectors and their generated-token helpers.

use macroonz_compiler::recipe::{
    EffectiveProjection, ProjectionError, ProjectionOffered, ProjectionRequest, ProjectionSink,
    RecipeProjector, RecipeRelation, RecipeRelationPayload, RecipeRelationPayloadKind,
    RecipeRelationRow, RecipeRole, RecipeTransitionEffect, RecipeView, RecipeVocabulary,
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
    CALLER_OWNED_TRIAL_RECIPE, CODEC_RECIPE, COMPANION_RECIPE, COMPLETE_RECIPE, DOOR,
    EVIDENCE_RECIPE, EXACT_DISPATCH_RECIPE, EXACT_EFFECT_RECIPE, TARGET_UNAVAILABLE_RECIPE,
};

pub(super) struct MirroredCompanions;

pub(super) struct MirroredDispatch;

pub(super) struct MirroredRelationTables;

pub(super) struct MirroredTypestate;

pub(super) struct MirroredCodec;

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
        let (states, events, relation) = transition_account(recipe)?;
        let states_name = companion_constant(states.name(), "VARIANTS");
        let mut tokens = roster_constant(
            states_name.as_str(),
            "The caller-authored vocabulary variants in declared order.",
            states.name_token(),
            states.members().members(),
        )?;
        if states.name() != events.name() {
            let events_name = companion_constant(events.name(), "VARIANTS");
            tokens.extend(roster_constant(
                events_name.as_str(),
                "The caller-authored vocabulary variants in declared order.",
                events.name_token(),
                events.members().members(),
            )?);
        }
        tokens.extend(transition_constant(states, events, relation)?);
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
        sink.offer(mirrored_dispatch(view.recipe(), request.effective())?)
    }
}

fn mirrored_dispatch(
    recipe: &macroonz_compiler::recipe::Recipe,
    effective: &EffectiveProjection,
) -> Result<GeneratedTree, ProjectionError> {
    let (states, events, relation) = transition_account(recipe)?;
    let refusal = mirrored_transition_refusal()?;
    let mut arms = relation
        .rows()
        .map(|row| dispatch_arm(states, events, row))
        .collect::<Result<Vec<_>, _>>()?;
    arms.push(absent_dispatch_arm()?);
    let bindings = effective.dispatch_binding_tokens();
    let state = bindings.map_or_else(
        || GeneratedToken::word("state"),
        |bindings| bindings[0].clone(),
    );
    let event = bindings.map_or_else(
        || GeneratedToken::word("event"),
        |bindings| bindings[1].clone(),
    );
    let body = match_expression(
        vec![group(
            GeneratedDelimiter::Parenthesis,
            comma_separated(vec![vec![state], vec![event]]),
        )?],
        arms,
    )?;
    if let Some(exact) = effective.exact_rust() {
        let mut tokens = refusal;
        tokens.extend(mirrored_exact_dispatch_imports(states, events, effective));
        tokens.extend(exact.tokens().iter().cloned());
        tokens.push(group(GeneratedDelimiter::Brace, body)?);
        return GeneratedTree::assembled(tokens).map_err(ProjectionError::Tokens);
    }
    let parameters = vec![
        typed_parameter(
            vec![GeneratedToken::word("state")],
            super_path(states.name_token()),
        ),
        typed_parameter(
            vec![GeneratedToken::word("event")],
            super_path(events.name_token()),
        ),
    ];
    let result = result_type(
        super_path(states.name_token()),
        vec![GeneratedToken::word("TransitionRefusal")],
    );
    let function = decorated(
        vec![documentation(
            "Applies one declared transition or returns typed absence.",
        )?],
        public(),
        function_item(
            function_signature(
                Vec::new(),
                GeneratedToken::word(effective.name().unwrap_or("apply")),
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
    GeneratedTree::assembled(tokens).map_err(ProjectionError::Tokens)
}

fn mirrored_transition_refusal() -> Result<Vec<GeneratedToken>, ProjectionError> {
    Ok(decorated(
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
    ))
}

fn absent_dispatch_arm() -> Result<Vec<GeneratedToken>, ProjectionError> {
    Ok(match_arm(
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
    ))
}

fn mirrored_exact_dispatch_imports(
    states: &RecipeVocabulary,
    events: &RecipeVocabulary,
    effective: &EffectiveProjection,
) -> Vec<GeneratedToken> {
    let Some([import_states, import_events]) = effective.dispatch_subject_imports() else {
        return Vec::new();
    };
    let mut imports = Vec::new();
    if import_states {
        imports.extend(use_item(super_path(states.name_token()), None));
    }
    if import_events && states.name() != events.name() {
        imports.extend(use_item(super_path(events.name_token()), None));
    }
    imports
}

impl RecipeProjector for MirroredRelationTables {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        assert_eq!(request.role(), RecipeRole::RelationTables);
        let recipe = view.recipe();
        let mut tokens = Vec::new();
        for table in request.effective().relation_tables() {
            let relation = recipe
                .relation(table.relation())
                .ok_or_else(nothing_rendered)?;
            if relation.payload_kind() != RecipeRelationPayloadKind::Unlabeled
                || table.exact_rust().is_some()
            {
                return Err(nothing_rendered());
            }
            let left = recipe
                .vocabulary(relation.left_vocabulary())
                .ok_or_else(nothing_rendered)?;
            let right = recipe
                .vocabulary(relation.right_vocabulary())
                .ok_or_else(nothing_rendered)?;
            tokens.extend(mirrored_relation_table(
                left,
                right,
                relation,
                table.function(),
            )?);
        }
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
        let (states, _events, _relation) = transition_account(view.recipe())?;
        let mut items = use_item(absolute_path(&["core", "marker", "PhantomData"]), None);
        items.extend(mirrored_stage_trait()?);
        items.extend(
            keyed_roster_items(states.members(), |_position, spelling, member| {
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

impl RecipeProjector for MirroredCodec {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        assert_eq!(request.role(), RecipeRole::Codec);
        let mut rendered = GeneratedTree::assembled(Vec::new())?;
        let mut observed = false;
        for declaration in view.recipe().codecs() {
            observed = true;
            let next = macroonz_compiler::codec::codec_surface(declaration.content())?;
            rendered = rendered.joined(&next)?;
        }
        if !observed {
            return Err(ProjectionError::Render(
                macroonz_compiler::RenderError::NothingRendered,
            ));
        }
        sink.offer(rendered)
    }
}

#[path = "support/observe.rs"]
mod observe;
pub(super) use observe::{
    bake, bake_at, bake_under, bake_with, bake_with_refusal, cargo_bytes, emitted_bytes, refusal,
    refusal_at, refusal_summary, refusal_under,
};
#[path = "support/tokens.rs"]
mod tokens;
pub(super) use tokens::{
    Occurrence, group_after_word, last_group_directly_containing, narrow_group_containing,
    word_handle,
};

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
    states: &RecipeVocabulary,
    events: &RecipeVocabulary,
    relation: &RecipeRelation,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut kind = vec![GeneratedToken::alone('&')];
    kind.push(group(
        GeneratedDelimiter::Bracket,
        vec![group(
            GeneratedDelimiter::Parenthesis,
            comma_separated(vec![
                super_path(states.name_token()),
                super_path(events.name_token()),
                super_path(states.name_token()),
            ]),
        )?],
    )?);
    let rows = relation
        .rows()
        .map(|row| {
            let (target_name, _effect) = transition_payload(row)?;
            group(
                GeneratedDelimiter::Parenthesis,
                comma_separated(vec![
                    variant(states.name_token(), row.left_name_token()),
                    variant(events.name_token(), row.right_name_token()),
                    variant(states.name_token(), target_name),
                ]),
            )
            .map_err(ProjectionError::Tokens)
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

fn mirrored_relation_table(
    left: &RecipeVocabulary,
    right: &RecipeVocabulary,
    relation: &RecipeRelation,
    function: &str,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let parameters = vec![
        typed_parameter(
            vec![GeneratedToken::word("left")],
            borrowed(super_super_path(left.name_token())),
        ),
        typed_parameter(
            vec![GeneratedToken::word("right")],
            borrowed(super_super_path(right.name_token())),
        ),
    ];
    let mut arms = relation
        .rows()
        .map(|row| {
            let pattern = group(
                GeneratedDelimiter::Parenthesis,
                comma_separated(vec![
                    super_super_variant(left.name_token(), row.left_name_token()),
                    super_super_variant(right.name_token(), row.right_name_token()),
                ]),
            )?;
            Ok(match_arm(
                vec![pattern],
                None,
                vec![GeneratedToken::word("true")],
            ))
        })
        .collect::<Result<Vec<_>, macroonz_compiler::Overflow>>()?;
    arms.push(match_arm(
        vec![GeneratedToken::word("_")],
        None,
        vec![GeneratedToken::word("false")],
    ));
    let body = match_expression(
        vec![group(
            GeneratedDelimiter::Parenthesis,
            comma_separated(vec![
                vec![GeneratedToken::word("left")],
                vec![GeneratedToken::word("right")],
            ]),
        )?],
        arms,
    )?;
    let function = decorated(
        vec![documentation(
            "Reports whether the supplied endpoints occupy one declared relation row.",
        )?],
        public(),
        function_item(
            function_signature(
                vec![GeneratedToken::word("const")],
                GeneratedToken::word(function),
                parameters,
                Vec::new(),
                Some(vec![GeneratedToken::word("bool")]),
                Vec::new(),
            )?,
            body,
        )?,
    );
    Ok(decorated(
        vec![documentation(
            "Typed behavior projected from one caller-named relation.",
        )?],
        public(),
        inline_module(relation.name_token().clone(), function)?,
    ))
}

fn dispatch_arm(
    states: &RecipeVocabulary,
    events: &RecipeVocabulary,
    row: &RecipeRelationRow,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let (target_name, effect) = transition_payload(row)?;
    let pattern = group(
        GeneratedDelimiter::Parenthesis,
        comma_separated(vec![
            variant(states.name_token(), row.left_name_token()),
            variant(events.name_token(), row.right_name_token()),
        ]),
    )?;
    let body = match effect {
        RecipeTransitionEffect::Path(effect) => {
            let mut body = effect.tokens().to_vec();
            body.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
            body.push(GeneratedToken::alone(';'));
            body.extend([
                GeneratedToken::word("Ok"),
                group(
                    GeneratedDelimiter::Parenthesis,
                    variant(states.name_token(), target_name),
                )?,
            ]);
            body
        }
        RecipeTransitionEffect::ExactRust {
            target_binding,
            body,
        } => {
            let mut exact = vec![GeneratedToken::word("let"), target_binding.clone()];
            exact.push(GeneratedToken::alone('='));
            exact.extend(variant(states.name_token(), target_name));
            exact.push(GeneratedToken::alone(';'));
            exact.extend(body.tokens().iter().cloned());
            exact
        }
        _ => {
            return Err(ProjectionError::Render(
                macroonz_compiler::RenderError::NothingRendered,
            ));
        }
    };
    Ok(match_arm(
        vec![pattern],
        None,
        vec![group(GeneratedDelimiter::Brace, body)?],
    ))
}

fn transition_account(
    recipe: &macroonz_compiler::recipe::Recipe,
) -> Result<(&RecipeVocabulary, &RecipeVocabulary, &RecipeRelation), ProjectionError> {
    let relation = recipe.transition_relation().ok_or(ProjectionError::Render(
        macroonz_compiler::RenderError::NothingRendered,
    ))?;
    let states = recipe
        .vocabulary(relation.left_vocabulary())
        .ok_or(ProjectionError::Render(
            macroonz_compiler::RenderError::NothingRendered,
        ))?;
    let events = recipe
        .vocabulary(relation.right_vocabulary())
        .ok_or(ProjectionError::Render(
            macroonz_compiler::RenderError::NothingRendered,
        ))?;
    Ok((states, events, relation))
}

fn transition_payload(
    row: &RecipeRelationRow,
) -> Result<(&GeneratedToken, &RecipeTransitionEffect), ProjectionError> {
    let RecipeRelationPayload::Transition {
        target_name,
        effect,
        ..
    } = row.payload()
    else {
        return Err(ProjectionError::Render(
            macroonz_compiler::RenderError::NothingRendered,
        ));
    };
    Ok((target_name, effect))
}

fn super_path(name: &GeneratedToken) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("super"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        name.clone(),
    ]
}

fn super_super_path(name: &GeneratedToken) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("super"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word("super"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        name.clone(),
    ]
}

fn super_super_variant(
    vocabulary: &GeneratedToken,
    member: &GeneratedToken,
) -> Vec<GeneratedToken> {
    let mut tokens = super_super_path(vocabulary);
    tokens.extend([
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        member.clone(),
    ]);
    tokens
}

fn borrowed(mut kind: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    kind.insert(0, GeneratedToken::alone('&'));
    kind
}

const fn nothing_rendered() -> ProjectionError {
    ProjectionError::Render(macroonz_compiler::RenderError::NothingRendered)
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

fn companion_constant(name: &str, suffix: &str) -> String {
    let name = name.strip_prefix("r#").unwrap_or(name);
    let mut generated = String::new();
    let mut previous_lowercase = false;
    for character in name.chars() {
        if character.is_uppercase() && previous_lowercase {
            generated.push('_');
        }
        for uppercase in character.to_uppercase() {
            generated.push(uppercase);
        }
        previous_lowercase = character.is_lowercase() || character.is_numeric();
    }
    generated.push('_');
    generated.push_str(suffix);
    generated
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
