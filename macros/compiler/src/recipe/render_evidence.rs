//! Standard compile-contract and property projections over one informed recipe.

use super::render_tokens::{
    comma_separated, crate_baked_path, crate_recipe_path, crate_recipe_variant, dispatch_name,
};
use super::{ProjectionError, Recipe, RecipeTransition};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, attribute, decorated, function_item,
    function_signature, group,
};

pub(super) fn compile_contract(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
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
    let mut result = crate::token::absolute_path(&["core", "result", "Result"]);
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

pub(super) fn property(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
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
    transition: &RecipeTransition,
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
