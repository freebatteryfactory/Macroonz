//! The standard sparse dispatch projection over informed transition rows.

use super::render_tokens::{call_variant, comma_separated, derive, public, super_path, variant};
use super::{EffectiveProjection, ProjectionError, Recipe, RecipeTransition};
use crate::relation::AbsencePosture;
use crate::token::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, decorated, documentation, enumeration,
    function_item, function_signature, group, match_arm, match_expression, result_type,
    typed_parameter, unit_variant, use_item,
};

pub(super) fn dispatch(
    recipe: &Recipe,
    effective: &EffectiveProjection,
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
    effective: &EffectiveProjection,
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
    effective: &EffectiveProjection,
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
    transition: &RecipeTransition,
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
