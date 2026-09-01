//! Projection through the one capability shared by standard and caller-owned projectors.

use super::{
    ProjectionError, ProjectionRequest, ProjectionSink, Recipe, RecipeProjector, RecipeRole,
    RecipeView,
};
use crate::bounded::AbsencePosture;
use crate::token::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, absolute_path, attribute, constant,
    decorated, documentation, enumeration, group, inline_module, tuple_struct, unit_struct,
    unit_variant, use_item,
};

/// The built-in projector catalog used by the paved proc host.
pub(super) struct StandardProjector;

impl RecipeProjector for StandardProjector {
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
        };
        sink.offer(tree)
    }
}

fn typestate(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
    let mut items = use_item(absolute_path(&["core", "marker", "PhantomData"]), None);
    for member in recipe.states().members() {
        items.extend(decorated(
            vec![
                documentation("One caller-declared typestate stage.")?,
                derive(&["Debug", "Clone", "Copy", "PartialEq", "Eq", "Hash"])?,
            ],
            public(),
            unit_struct(member.name_token().clone(), Vec::new(), Vec::new()),
        ));
    }
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
    let projected = decorated(
        vec![documentation(
            "Type-level stages derived from the caller-authored state vocabulary.",
        )?],
        public(),
        inline_module(GeneratedToken::word("typestate"), items)?,
    );
    GeneratedTree::assembled(projected).map_err(ProjectionError::Tokens)
}

fn public() -> Vec<GeneratedToken> {
    vec![GeneratedToken::word("pub")]
}

fn derive(names: &[&str]) -> Result<Vec<GeneratedToken>, ProjectionError> {
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
    let parameters = comma_separated(vec![
        typed("state", super_path(recipe.states_name_token())),
        typed("event", super_path(recipe.events_name_token())),
    ]);
    let mut result = absolute_path(&["core", "result", "Result"]);
    result.push(GeneratedToken::alone('<'));
    result.extend(super_path(recipe.states_name_token()));
    result.push(GeneratedToken::alone(','));
    result.push(GeneratedToken::word("TransitionRefusal"));
    result.push(GeneratedToken::alone('>'));
    let mut arms = Vec::new();
    for transition in recipe.transitions().members() {
        arms.extend(dispatch_arm(recipe, transition)?);
    }
    arms.extend(absent_arm()?);
    let body = vec![
        GeneratedToken::word("match"),
        group(
            GeneratedDelimiter::Parenthesis,
            comma_separated(vec![
                vec![GeneratedToken::word("state")],
                vec![GeneratedToken::word("event")],
            ]),
        )?,
        group(GeneratedDelimiter::Brace, arms)?,
    ];
    let mut tokens = documentation("Applies one declared transition or returns typed absence.")?;
    tokens.extend([GeneratedToken::word("pub"), GeneratedToken::word("fn")]);
    tokens.push(GeneratedToken::word(name));
    tokens.push(group(GeneratedDelimiter::Parenthesis, parameters)?);
    tokens.push(GeneratedToken::joint('-'));
    tokens.push(GeneratedToken::alone('>'));
    tokens.extend(result);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
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
    let mut tokens = vec![
        pattern,
        GeneratedToken::joint('='),
        GeneratedToken::alone('>'),
    ];
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
    tokens.push(GeneratedToken::alone(','));
    Ok(tokens)
}

fn absent_arm() -> Result<Vec<GeneratedToken>, ProjectionError> {
    Ok(vec![
        GeneratedToken::word("_"),
        GeneratedToken::joint('='),
        GeneratedToken::alone('>'),
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
        GeneratedToken::alone(','),
    ])
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
    let mut tokens = attribute(vec![GeneratedToken::word("test")])?;
    tokens.push(GeneratedToken::word("fn"));
    tokens.push(GeneratedToken::word("declared_recipe_rows_are_observed"));
    tokens.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    tokens.push(group(GeneratedDelimiter::Brace, body)?);
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

fn typed(name: &str, kind: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = vec![GeneratedToken::word(name), GeneratedToken::alone(':')];
    tokens.extend(kind);
    tokens
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
