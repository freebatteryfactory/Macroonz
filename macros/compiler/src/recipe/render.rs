//! Projection through the one capability shared by standard and caller-owned projectors.

use super::evidence::PreparedEvidence;
use super::{
    ProjectionError, ProjectionRequest, ProjectionSink, Recipe, RecipeProjector, RecipeRole,
    RecipeView,
};
use crate::bounded::{AbsencePosture, NonEmptyError};
use crate::token::{
    GeneratedDelimiter, GeneratedRowRefusal, GeneratedToken, GeneratedTree, absolute_path,
    associated_constant, associated_function, attribute, constant, decorated, documentation,
    enumeration, function_item, function_signature, group, implementation, inline_module,
    keyed_roster_items, match_arm, match_expression, result_type, trait_declaration, tuple_struct,
    typed_parameter, unit_struct, unit_variant, use_item,
};

/// The built-in projector catalog used by the paved proc host.
pub(super) struct StandardProjector<'evidence> {
    evidence: &'evidence PreparedEvidence,
}

impl<'evidence> StandardProjector<'evidence> {
    /// Bind the built-in catalog to the descriptor outputs prepared for this recipe walk.
    pub(super) const fn over(evidence: &'evidence PreparedEvidence) -> Self {
        Self { evidence }
    }
}

impl RecipeProjector for StandardProjector<'_> {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<super::ProjectionOffered, ProjectionError> {
        let tree = match request.role() {
            RecipeRole::Companions => companions(view.recipe())?,
            RecipeRole::Dispatch => dispatch(view.recipe(), request.effective())?,
            RecipeRole::CompileContract => compile_contract(view.recipe())?,
            RecipeRole::Property => property(view.recipe())?,
            RecipeRole::Typestate => typestate(view.recipe())?,
            RecipeRole::Trials
            | RecipeRole::Mutation
            | RecipeRole::Benchmarks
            | RecipeRole::Network
            | RecipeRole::Concurrency => self.evidence(request.role())?,
        };
        sink.offer(tree)
    }
}

impl StandardProjector<'_> {
    fn evidence(&self, role: RecipeRole) -> Result<GeneratedTree, ProjectionError> {
        self.evidence
            .tree(role)
            .cloned()
            .ok_or(ProjectionError::Render(
                crate::render::RenderError::NothingRendered,
            ))
    }
}

fn typestate(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
    let mut items = use_item(absolute_path(&["core", "marker", "PhantomData"]), None);
    items.extend(stage_trait()?);
    items.extend(
        keyed_roster_items(recipe.states(), |_position, spelling, member| {
            stage_member(spelling, member)
        })
        .map_err(row_projection_error)?,
    );
    let marker = vec![GeneratedToken::word("Marker")];
    let phantom = vec![
        GeneratedToken::word("PhantomData"),
        GeneratedToken::alone('<'),
        GeneratedToken::word("Marker"),
        GeneratedToken::alone('>'),
    ];
    items.extend(decorated(
        vec![
            documentation("A type-level carrier over one caller-declared stage.")?,
            derive(&["Debug", "Clone", "Copy", "PartialEq", "Eq", "Hash"])?,
        ],
        public(),
        tuple_struct(
            GeneratedToken::word("Stage"),
            vec![marker],
            vec![decorated(Vec::new(), public(), phantom)],
            Vec::new(),
        )?,
    ));
    items.extend(stage_inherent()?);
    items.extend(stage_default()?);
    let projected = decorated(
        vec![documentation(
            "Type-level stages derived from the caller-authored state vocabulary.",
        )?],
        public(),
        inline_module(GeneratedToken::word("typestate"), items)?,
    );
    GeneratedTree::assembled(projected).map_err(ProjectionError::Tokens)
}

fn stage_trait() -> Result<Vec<GeneratedToken>, ProjectionError> {
    let name = associated_constant(GeneratedToken::word("NAME"), static_str(), None);
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

fn stage_member(
    spelling: &str,
    member: &super::RecipeMember,
) -> Result<Vec<GeneratedToken>, crate::bounded::Overflow> {
    let mut tokens = decorated(
        vec![
            documentation("One caller-declared typestate stage.")?,
            derive(&["Debug", "Clone", "Copy", "PartialEq", "Eq", "Hash"])?,
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
            static_str(),
            Some(vec![GeneratedToken::text(spelling)]),
        ),
    )?);
    Ok(tokens)
}

fn stage_inherent() -> Result<Vec<GeneratedToken>, ProjectionError> {
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
        generic_stage(),
        Vec::new(),
        constructor,
    )
    .map_err(ProjectionError::Tokens)
}

fn stage_default() -> Result<Vec<GeneratedToken>, ProjectionError> {
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
        generic_stage(),
        Vec::new(),
        function,
    )
    .map_err(ProjectionError::Tokens)
}

fn generic_stage() -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("Stage"),
        GeneratedToken::alone('<'),
        GeneratedToken::word("Marker"),
        GeneratedToken::alone('>'),
    ]
}

fn static_str() -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::alone('&'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("static"),
        GeneratedToken::word("str"),
    ]
}

fn row_projection_error(refusal: GeneratedRowRefusal) -> ProjectionError {
    match refusal.cause() {
        NonEmptyError::Empty(_) => {
            ProjectionError::Render(crate::render::RenderError::NothingRendered)
        }
        NonEmptyError::Overflow(cause) => ProjectionError::Tokens(cause),
    }
}

fn public() -> Vec<GeneratedToken> {
    vec![GeneratedToken::word("pub")]
}

fn derive(names: &[&str]) -> Result<Vec<GeneratedToken>, crate::bounded::Overflow> {
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

pub(super) fn project(
    recipe: &Recipe,
    role: RecipeRole,
    sink: ProjectionSink<'_, '_>,
    projector: &dyn RecipeProjector,
) -> Result<(), ProjectionError> {
    let Some(effective) = recipe.effective(role) else {
        return Err(ProjectionError::Render(
            crate::render::RenderError::SeatUnplanned { role: role.name() },
        ));
    };
    projector
        .project(
            RecipeView::over(recipe),
            ProjectionRequest::selected(effective),
            sink,
        )
        .map(|_| ())
}

fn companions(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
    let mut tokens = Vec::new();
    tokens.extend(roster_constant(
        "STATE_VARIANTS",
        recipe.states_name_token(),
        recipe.states().members(),
    )?);
    tokens.extend(roster_constant(
        "EVENT_VARIANTS",
        recipe.events_name_token(),
        recipe.events().members(),
    )?);
    tokens.extend(transition_constant(recipe)?);
    GeneratedTree::assembled(tokens).map_err(ProjectionError::Tokens)
}

fn roster_constant<'name>(
    constant_name: &str,
    vocabulary: &GeneratedToken,
    members: impl Iterator<Item = &'name super::RecipeMember>,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let sentence = match constant_name {
        "STATE_VARIANTS" => "The state variants in caller-authored order.",
        "EVENT_VARIANTS" => "The event variants in caller-authored order.",
        _ => "The caller-authored vocabulary members in declared order.",
    };
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

fn transition_constant(recipe: &Recipe) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut kind = vec![GeneratedToken::alone('&')];
    let row = group(
        GeneratedDelimiter::Parenthesis,
        comma_separated(vec![
            super_path(recipe.states_name_token()),
            super_path(recipe.events_name_token()),
            super_path(recipe.states_name_token()),
        ]),
    )?;
    kind.push(group(GeneratedDelimiter::Bracket, vec![row])?);
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
    let value = vec![
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Bracket, comma_tokens(rows))?,
    ];
    Ok(decorated(
        vec![documentation(
            "The informed transition rows in caller-authored order.",
        )?],
        public(),
        constant("TRANSITIONS", kind, value),
    ))
}

fn dispatch(
    recipe: &Recipe,
    effective: &super::EffectiveProjection,
) -> Result<GeneratedTree, ProjectionError> {
    if recipe.absence() != AbsencePosture::Refusal {
        return Err(ProjectionError::Render(
            crate::render::RenderError::NothingRendered,
        ));
    }
    let refusal = decorated(
        vec![
            documentation("Why generated dispatch did not find an admitted transition row.")?,
            derive(&["Debug", "Clone", "Copy", "PartialEq", "Eq"])?,
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
    let mut tokens = refusal;
    tokens.extend(dispatch_function(recipe, effective)?);
    GeneratedTree::assembled(tokens).map_err(ProjectionError::Tokens)
}

fn dispatch_function(
    recipe: &Recipe,
    effective: &super::EffectiveProjection,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let name = effective.name().unwrap_or("apply");
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
    let mut arms = Vec::new();
    for transition in recipe.transitions().members() {
        arms.push(dispatch_arm(recipe, transition)?);
    }
    arms.push(absent_arm()?);
    let bindings = effective.exact_dispatch_bindings();
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
        let mut tokens = exact_dispatch_vocabulary_imports(recipe, effective);
        tokens.extend(exact.tokens().iter().cloned());
        tokens.push(group(GeneratedDelimiter::Brace, body)?);
        return Ok(tokens);
    }
    Ok(decorated(
        vec![documentation(
            "Applies one declared transition or returns typed absence.",
        )?],
        public(),
        function_item(
            function_signature(
                Vec::new(),
                GeneratedToken::word(name),
                parameters,
                Vec::new(),
                Some(result),
                Vec::new(),
            )?,
            body,
        )?,
    ))
}

fn exact_dispatch_vocabulary_imports(
    recipe: &Recipe,
    effective: &super::EffectiveProjection,
) -> Vec<GeneratedToken> {
    let Some([states, events]) = effective.exact_dispatch_imports().copied() else {
        return Vec::new();
    };
    let mut imports = Vec::new();
    if states {
        imports.extend(use_item(super_path(recipe.states_name_token()), None));
    }
    if events && recipe.states_name() != recipe.events_name() {
        imports.extend(use_item(super_path(recipe.events_name_token()), None));
    }
    imports
}

fn dispatch_arm(
    recipe: &Recipe,
    transition: &super::RecipeTransition,
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
    body.extend(call_variant(
        "Ok",
        recipe.states_name_token(),
        transition.target_name_token(),
    )?);
    Ok(match_arm(
        vec![pattern],
        None,
        vec![group(GeneratedDelimiter::Brace, body)?],
    ))
}

fn absent_arm() -> Result<Vec<GeneratedToken>, ProjectionError> {
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

fn compile_contract(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
    let name = dispatch_name(recipe);
    let mut tokens = vec![GeneratedToken::word("const"), GeneratedToken::word("_")];
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(group(
        GeneratedDelimiter::Parenthesis,
        comma_separated(vec![
            crate_recipe_path(recipe, recipe.states_name_token()),
            crate_recipe_path(recipe, recipe.events_name_token()),
        ]),
    )?);
    tokens.push(GeneratedToken::joint('-'));
    tokens.push(GeneratedToken::alone('>'));
    let mut result = absolute_path(&["core", "result", "Result"]);
    result.push(GeneratedToken::alone('<'));
    result.extend(crate_recipe_path(recipe, recipe.states_name_token()));
    result.push(GeneratedToken::alone(','));
    result.extend(crate_baked_path(recipe, "TransitionRefusal"));
    result.push(GeneratedToken::alone('>'));
    tokens.extend(result);
    tokens.push(GeneratedToken::alone('='));
    tokens.extend(crate_baked_path(recipe, name));
    tokens.push(GeneratedToken::alone(';'));
    GeneratedTree::assembled(tokens).map_err(ProjectionError::Tokens)
}

fn property(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
    let mut body = Vec::new();
    for (position, transition) in recipe.transitions().members().enumerate() {
        body.extend(property_row(recipe, transition, position)?);
    }
    let tokens = decorated(
        vec![attribute(vec![GeneratedToken::word("test")])?],
        Vec::new(),
        function_item(
            function_signature(
                Vec::new(),
                GeneratedToken::word("declared_recipe_rows_are_observed"),
                Vec::new(),
                Vec::new(),
                None,
                Vec::new(),
            )?,
            body,
        )?,
    );
    GeneratedTree::assembled(tokens).map_err(ProjectionError::Tokens)
}

fn property_row(
    recipe: &Recipe,
    transition: &super::RecipeTransition,
    position: usize,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let observed = format!("observed_{position}");
    let conclusion = format!("conclusion_{position}");
    let mut applied = crate_baked_path(recipe, dispatch_name(recipe));
    applied.push(group(
        GeneratedDelimiter::Parenthesis,
        comma_separated(vec![
            crate_recipe_variant(
                recipe,
                recipe.states_name_token(),
                transition.source_name_token(),
            ),
            crate_recipe_variant(
                recipe,
                recipe.events_name_token(),
                transition.event_name_token(),
            ),
        ]),
    )?);
    let pattern = vec![
        GeneratedToken::word("Ok"),
        group(
            GeneratedDelimiter::Parenthesis,
            crate_recipe_variant(
                recipe,
                recipe.states_name_token(),
                transition.target_name_token(),
            ),
        )?,
    ];
    let mut tokens = vec![
        GeneratedToken::word("let"),
        GeneratedToken::word(observed.as_str()),
        GeneratedToken::alone('='),
        GeneratedToken::word("matches"),
        GeneratedToken::alone('!'),
        group(
            GeneratedDelimiter::Parenthesis,
            comma_separated(vec![applied, pattern]),
        )?,
        GeneratedToken::alone(';'),
    ];
    tokens.extend([
        GeneratedToken::word("let"),
        GeneratedToken::word(conclusion.as_str()),
        GeneratedToken::alone('='),
    ]);
    tokens.extend(harness_path(&["properties", "concluded"]));
    let mut arguments = vec![
        GeneratedToken::word("if"),
        GeneratedToken::word(observed.as_str()),
    ];
    arguments.push(group(
        GeneratedDelimiter::Brace,
        harness_path(&["properties", "Holding", "Holds"]),
    )?);
    arguments.push(GeneratedToken::word("else"));
    arguments.push(group(
        GeneratedDelimiter::Brace,
        harness_path(&["properties", "Holding", "Fails"]),
    )?);
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(harness_path(&[
        "report",
        "FailureClass",
        "PropertyDisagreement",
    ]));
    arguments.push(GeneratedToken::alone(','));
    arguments.extend(harness_path(&["properties", "ANSWER_EXPECTED"]));
    tokens.push(group(GeneratedDelimiter::Parenthesis, arguments)?);
    tokens.push(GeneratedToken::alone(';'));
    tokens.push(GeneratedToken::word("assert"));
    tokens.push(GeneratedToken::alone('!'));
    tokens.push(group(GeneratedDelimiter::Parenthesis, {
        let mut matched = vec![GeneratedToken::word("matches"), GeneratedToken::alone('!')];
        matched.push(group(
            GeneratedDelimiter::Parenthesis,
            comma_separated(vec![
                vec![GeneratedToken::word(conclusion.as_str())],
                harness_path(&["report", "TrialConclusion", "Passed"]),
            ]),
        )?);
        matched
    })?);
    tokens.push(GeneratedToken::alone(';'));
    Ok(tokens)
}

fn harness_path(segments: &[&str]) -> Vec<GeneratedToken> {
    crate::support::rooted_path(crate::support::CrateFacing::Harness, segments)
}

fn dispatch_name(recipe: &Recipe) -> &str {
    recipe
        .effective(RecipeRole::Dispatch)
        .and_then(super::EffectiveProjection::name)
        .unwrap_or("apply")
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

fn crate_recipe_path(recipe: &Recipe, name: &GeneratedToken) -> Vec<GeneratedToken> {
    let mut tokens = crate::support::rooted_path(crate::support::CrateFacing::Declaring, &[]);
    extend_token_path(
        &mut tokens,
        [recipe.module_name_token().clone(), name.clone()],
    );
    tokens
}

fn crate_baked_path(recipe: &Recipe, name: &str) -> Vec<GeneratedToken> {
    let mut tokens = crate::support::rooted_path(crate::support::CrateFacing::Declaring, &[]);
    extend_token_path(
        &mut tokens,
        [
            recipe.module_name_token().clone(),
            GeneratedToken::word("baked"),
            GeneratedToken::word(name),
        ],
    );
    tokens
}

fn crate_recipe_variant(
    recipe: &Recipe,
    vocabulary: &GeneratedToken,
    member: &GeneratedToken,
) -> Vec<GeneratedToken> {
    let mut tokens = crate_recipe_path(recipe, vocabulary);
    extend_token_path(&mut tokens, [member.clone()]);
    tokens
}

fn extend_token_path(
    tokens: &mut Vec<GeneratedToken>,
    segments: impl IntoIterator<Item = GeneratedToken>,
) {
    for segment in segments {
        tokens.push(GeneratedToken::joint(':'));
        tokens.push(GeneratedToken::alone(':'));
        tokens.push(segment);
    }
}

fn call_variant(
    constructor: &str,
    vocabulary: &GeneratedToken,
    member: &GeneratedToken,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    Ok(vec![
        GeneratedToken::word(constructor),
        group(GeneratedDelimiter::Parenthesis, variant(vocabulary, member))?,
    ])
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

fn comma_tokens(parts: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    for part in parts {
        tokens.push(part);
        tokens.push(GeneratedToken::alone(','));
    }
    tokens
}
