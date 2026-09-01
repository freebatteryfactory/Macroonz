//! The standard roster and transition companion projection.

use super::render_tokens::{comma_separated, comma_tokens, public, super_path, variant};
use super::{ProjectionError, Recipe, RecipeMember};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, constant, decorated, documentation, group,
};

pub(super) fn companions(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
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
    members: impl Iterator<Item = &'name RecipeMember>,
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
