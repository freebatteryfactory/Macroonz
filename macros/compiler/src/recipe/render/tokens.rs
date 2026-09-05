//! Shared token mechanics used by the standard recipe projector families.

use super::super::{EffectiveProjection, ProjectionError, Recipe, RecipeRole};
use crate::bounded::NonEmptyError;
use crate::token::{GeneratedDelimiter, GeneratedRowRefusal, GeneratedToken, attribute, group};

pub(super) fn row_projection_error(refusal: GeneratedRowRefusal) -> ProjectionError {
    match refusal.cause() {
        NonEmptyError::Empty(_) => {
            ProjectionError::Render(crate::render::RenderError::NothingRendered)
        }
        NonEmptyError::Overflow(cause) => ProjectionError::Tokens(cause),
    }
}

pub(super) fn public() -> Vec<GeneratedToken> {
    vec![GeneratedToken::word("pub")]
}

pub(super) fn derive(names: &[&str]) -> Result<Vec<GeneratedToken>, crate::bounded::Overflow> {
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

pub(super) fn static_str() -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::alone('&'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("static"),
        GeneratedToken::word("str"),
    ]
}

pub(super) fn dispatch_name(recipe: &Recipe) -> &str {
    recipe
        .effective(RecipeRole::Dispatch)
        .and_then(EffectiveProjection::name)
        .unwrap_or("apply")
}

pub(super) fn super_path(name: &GeneratedToken) -> Vec<GeneratedToken> {
    vec![
        GeneratedToken::word("super"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        name.clone(),
    ]
}

pub(super) fn variant(vocabulary: &GeneratedToken, member: &GeneratedToken) -> Vec<GeneratedToken> {
    let mut tokens = super_path(vocabulary);
    tokens.extend([
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        member.clone(),
    ]);
    tokens
}

pub(super) fn crate_recipe_path(recipe: &Recipe, name: &GeneratedToken) -> Vec<GeneratedToken> {
    let mut tokens = crate::support::rooted_path(crate::support::CrateFacing::Declaring, &[]);
    extend_token_path(
        &mut tokens,
        [recipe.module_name_token().clone(), name.clone()],
    );
    tokens
}

pub(super) fn crate_baked_path(recipe: &Recipe, name: &str) -> Vec<GeneratedToken> {
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

pub(super) fn crate_recipe_variant(
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

pub(super) fn call_variant(
    constructor: &str,
    vocabulary: &GeneratedToken,
    member: &GeneratedToken,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    Ok(vec![
        GeneratedToken::word(constructor),
        group(GeneratedDelimiter::Parenthesis, variant(vocabulary, member))?,
    ])
}

pub(super) fn comma_separated(parts: Vec<Vec<GeneratedToken>>) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    for (position, part) in parts.into_iter().enumerate() {
        if position > 0 {
            tokens.push(GeneratedToken::alone(','));
        }
        tokens.extend(part);
    }
    tokens
}

pub(super) fn comma_tokens(parts: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    for part in parts {
        tokens.push(part);
        tokens.push(GeneratedToken::alone(','));
    }
    tokens
}
