//! The standard roster and transition companion projection.

use super::names::companion_constant;
use super::render_tokens::{comma_separated, comma_tokens, public, super_path, variant};
use super::{
    ProjectionError, Recipe, RecipeMember, RecipeRelation, RecipeRelationPayload,
    RecipeRelationPayloadKind, RecipeRelationRow, RecipeVocabulary,
};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, constant, decorated, documentation, group,
};

pub(super) fn companions(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
    let transition = recipe.transition_account();
    let mut tokens = Vec::new();
    for vocabulary in recipe.vocabularies() {
        let name = companion_constant(vocabulary.name(), "VARIANTS");
        tokens.extend(roster_constant(
            name.as_str(),
            vocabulary.name_token(),
            vocabulary.members().members(),
        )?);
    }
    for relation in recipe.relations() {
        if let Some((states, events, transition_relation)) = transition
            && relation.name() == transition_relation.name()
        {
            tokens.extend(transition_constant(states, events, relation.rows())?);
            continue;
        }
        let Some(left) = recipe.vocabulary(relation.left_vocabulary()) else {
            return Err(nothing_rendered());
        };
        let Some(right) = recipe.vocabulary(relation.right_vocabulary()) else {
            return Err(nothing_rendered());
        };
        tokens.extend(relation_constant(left, right, relation)?);
        if relation.payload_kind() != RecipeRelationPayloadKind::Unlabeled {
            tokens.extend(relation_payload_constant(relation)?);
        }
    }
    GeneratedTree::assembled(tokens).map_err(ProjectionError::Tokens)
}

fn roster_constant<'name>(
    constant_name: &str,
    vocabulary: &GeneratedToken,
    members: impl Iterator<Item = &'name RecipeMember>,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let sentence = "The caller-authored vocabulary variants in declared order.";
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

fn transition_constant<'rows>(
    states: &RecipeVocabulary,
    events: &RecipeVocabulary,
    rows: impl Iterator<Item = &'rows RecipeRelationRow>,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut kind = vec![GeneratedToken::alone('&')];
    let row_type = group(
        GeneratedDelimiter::Parenthesis,
        comma_separated(vec![
            super_path(states.name_token()),
            super_path(events.name_token()),
            super_path(states.name_token()),
        ]),
    )?;
    kind.push(group(GeneratedDelimiter::Bracket, vec![row_type])?);
    let rows = rows
        .map(|row| {
            let Some((_target, target_name, _effect)) = row.payload().transition_parts() else {
                return Err(ProjectionError::Render(
                    crate::render::RenderError::NothingRendered,
                ));
            };
            Ok(group(
                GeneratedDelimiter::Parenthesis,
                comma_separated(vec![
                    variant(states.name_token(), row.left_name_token()),
                    variant(events.name_token(), row.right_name_token()),
                    variant(states.name_token(), target_name),
                ]),
            )?)
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

fn relation_constant(
    left: &RecipeVocabulary,
    right: &RecipeVocabulary,
    relation: &RecipeRelation,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut kind = vec![GeneratedToken::alone('&')];
    let row_type = group(
        GeneratedDelimiter::Parenthesis,
        comma_separated(vec![
            super_path(left.name_token()),
            super_path(right.name_token()),
        ]),
    )?;
    kind.push(group(GeneratedDelimiter::Bracket, vec![row_type])?);
    let rows = relation
        .rows()
        .map(|row| {
            group(
                GeneratedDelimiter::Parenthesis,
                comma_separated(vec![
                    variant(left.name_token(), row.left_name_token()),
                    variant(right.name_token(), row.right_name_token()),
                ]),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let value = vec![
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Bracket, comma_tokens(rows))?,
    ];
    let name = companion_constant(relation.name(), "ROWS");
    Ok(decorated(
        vec![documentation(
            "The informed relation endpoint rows in caller-authored order.",
        )?],
        public(),
        constant(name.as_str(), kind, value),
    ))
}

fn relation_payload_constant(
    relation: &RecipeRelation,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut kind = vec![GeneratedToken::alone('&')];
    kind.push(group(
        GeneratedDelimiter::Bracket,
        vec![GeneratedToken::alone('&'), GeneratedToken::word("str")],
    )?);
    let rows = relation
        .rows()
        .map(|row| {
            let payload = match row.payload() {
                RecipeRelationPayload::Path(path) | RecipeRelationPayload::ExactRust(path) => {
                    path.tokens().to_vec()
                }
                RecipeRelationPayload::Transition {
                    target_name,
                    effect: super::RecipeTransitionEffect::Path(effect),
                    ..
                } => {
                    let mut tokens = vec![target_name.clone(), GeneratedToken::alone(',')];
                    tokens.extend(effect.tokens().iter().cloned());
                    tokens
                }
                RecipeRelationPayload::Transition {
                    target_name,
                    effect:
                        super::RecipeTransitionEffect::ExactRust {
                            target_binding,
                            body,
                        },
                    ..
                } => {
                    let mut tokens = vec![target_name.clone(), GeneratedToken::word("with")];
                    tokens.push(group(
                        GeneratedDelimiter::Parenthesis,
                        vec![target_binding.clone()],
                    )?);
                    tokens.push(group(GeneratedDelimiter::Brace, body.tokens().to_vec())?);
                    tokens
                }
                RecipeRelationPayload::Unlabeled => return Err(nothing_rendered()),
            };
            Ok(vec![
                GeneratedToken::word("stringify"),
                GeneratedToken::alone('!'),
                group(GeneratedDelimiter::Parenthesis, payload)?,
            ])
        })
        .collect::<Result<Vec<_>, ProjectionError>>()?;
    let value = vec![
        GeneratedToken::alone('&'),
        group(GeneratedDelimiter::Bracket, comma_separated(rows))?,
    ];
    let name = companion_constant(relation.name(), "PAYLOADS");
    Ok(decorated(
        vec![documentation(
            "The exact caller-authored relation payloads as Rust spellings.",
        )?],
        public(),
        constant(name.as_str(), kind, value),
    ))
}

const fn nothing_rendered() -> ProjectionError {
    ProjectionError::Render(crate::render::RenderError::NothingRendered)
}
