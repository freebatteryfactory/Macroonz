//! The standard sparse dispatch projection over informed transition rows.

use super::render_tokens::{call_variant, comma_separated, derive, public, super_path, variant};
use super::{
    EffectiveProjection, ProjectionError, Recipe, RecipeRelation, RecipeRelationRow,
    RecipeVocabulary,
};
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
    let Some((states, events, relation)) = recipe.transition_account() else {
        return Err(ProjectionError::Render(
            crate::render::RenderError::NothingRendered,
        ));
    };
    if relation.requirements().absence() != Some(AbsencePosture::Refusal) {
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
    tokens.extend(dispatch_function(states, events, relation, effective)?);
    GeneratedTree::assembled(tokens).map_err(ProjectionError::Tokens)
}

fn dispatch_function(
    states: &RecipeVocabulary,
    events: &RecipeVocabulary,
    relation: &RecipeRelation,
    effective: &EffectiveProjection,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let name = effective.name().unwrap_or("apply");
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
    let mut arms = Vec::new();
    for row in relation.rows() {
        arms.push(dispatch_arm(states, events, row)?);
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
        let mut tokens = exact_dispatch_vocabulary_imports(states, events, effective);
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
    states: &RecipeVocabulary,
    events: &RecipeVocabulary,
    effective: &EffectiveProjection,
) -> Vec<GeneratedToken> {
    let Some([import_states, import_events]) = effective.exact_dispatch_imports().copied() else {
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

fn dispatch_arm(
    states: &RecipeVocabulary,
    events: &RecipeVocabulary,
    row: &RecipeRelationRow,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let Some((_target, target_name, effect)) = row.payload().transition_parts() else {
        return Err(ProjectionError::Render(
            crate::render::RenderError::NothingRendered,
        ));
    };
    let pattern = group(
        GeneratedDelimiter::Parenthesis,
        comma_separated(vec![
            variant(states.name_token(), row.left_name_token()),
            variant(events.name_token(), row.right_name_token()),
        ]),
    )?;
    let mut body = effect.tokens().to_vec();
    body.push(group(GeneratedDelimiter::Parenthesis, Vec::new())?);
    body.push(GeneratedToken::alone(';'));
    body.extend(call_variant("Ok", states.name_token(), target_name)?);
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
