//! Typed membership and payload lookup projections over caller-named relations.

use super::super::{
    EffectiveProjection, ProjectionError, Recipe, RecipeRelation, RecipeRelationPayload,
    RecipeRelationPayloadKind, RecipeRelationRow, RecipeVocabulary, RelationTableProjection,
};
use super::tokens::{comma_separated, public};
use crate::token::{
    GeneratedDelimiter, GeneratedToken, GeneratedTree, decorated, documentation, function_item,
    function_signature, group, inline_module, match_arm, match_expression, typed_parameter,
    use_item,
};

pub(super) fn relation_tables(
    recipe: &Recipe,
    effective: &EffectiveProjection,
) -> Result<GeneratedTree, ProjectionError> {
    let mut tokens = Vec::new();
    for table in effective.relation_tables() {
        let relation = recipe
            .relation(table.relation())
            .ok_or_else(nothing_rendered)?;
        let left = recipe
            .vocabulary(relation.left_vocabulary())
            .ok_or_else(nothing_rendered)?;
        let right = recipe
            .vocabulary(relation.right_vocabulary())
            .ok_or_else(nothing_rendered)?;
        let body = table_function(left, right, relation, table)?;
        tokens.extend(decorated(
            vec![documentation(
                "Typed behavior projected from one caller-named relation.",
            )?],
            public(),
            inline_module(relation.name_token().clone(), body)?,
        ));
    }
    GeneratedTree::assembled(tokens).map_err(ProjectionError::Tokens)
}

fn table_function(
    left: &RecipeVocabulary,
    right: &RecipeVocabulary,
    relation: &RecipeRelation,
    table: &RelationTableProjection,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let bindings = table.bindings().map_or_else(
        || [GeneratedToken::word("left"), GeneratedToken::word("right")],
        Clone::clone,
    );
    let body = table_body(left, right, relation, &bindings)?;
    if let Some(exact) = table.exact_rust() {
        let mut tokens = exact_imports(left, right, table);
        tokens.extend(exact.tokens().iter().cloned());
        tokens.push(group(GeneratedDelimiter::Brace, body)?);
        return Ok(tokens);
    }
    let parameters = vec![
        typed_parameter(
            vec![bindings[0].clone()],
            borrowed(super_super_path(left.name_token())),
        ),
        typed_parameter(
            vec![bindings[1].clone()],
            borrowed(super_super_path(right.name_token())),
        ),
    ];
    Ok(decorated(
        vec![documentation(
            "Reports whether the supplied endpoints occupy one declared relation row.",
        )?],
        public(),
        function_item(
            function_signature(
                vec![GeneratedToken::word("const")],
                GeneratedToken::word(table.function()),
                parameters,
                Vec::new(),
                Some(vec![GeneratedToken::word("bool")]),
                Vec::new(),
            )?,
            body,
        )?,
    ))
}

fn table_body(
    left: &RecipeVocabulary,
    right: &RecipeVocabulary,
    relation: &RecipeRelation,
    bindings: &[GeneratedToken; 2],
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let mut arms = relation
        .rows()
        .map(|row| table_arm(left, right, row))
        .collect::<Result<Vec<_>, _>>()?;
    let absent = match relation.payload_kind() {
        RecipeRelationPayloadKind::Unlabeled => vec![GeneratedToken::word("false")],
        RecipeRelationPayloadKind::Path | RecipeRelationPayloadKind::ExactRust => {
            vec![GeneratedToken::word("None")]
        }
        RecipeRelationPayloadKind::Transition => return Err(nothing_rendered()),
    };
    arms.push(match_arm(vec![GeneratedToken::word("_")], None, absent));
    match_expression(
        vec![group(
            GeneratedDelimiter::Parenthesis,
            comma_separated(vec![vec![bindings[0].clone()], vec![bindings[1].clone()]]),
        )?],
        arms,
    )
    .map_err(ProjectionError::Tokens)
}

fn table_arm(
    left: &RecipeVocabulary,
    right: &RecipeVocabulary,
    row: &RecipeRelationRow,
) -> Result<Vec<GeneratedToken>, ProjectionError> {
    let pattern = group(
        GeneratedDelimiter::Parenthesis,
        comma_separated(vec![
            super_super_variant(left.name_token(), row.left_name_token()),
            super_super_variant(right.name_token(), row.right_name_token()),
        ]),
    )?;
    let value = match row.payload() {
        RecipeRelationPayload::Unlabeled => vec![GeneratedToken::word("true")],
        RecipeRelationPayload::Path(payload) | RecipeRelationPayload::ExactRust(payload) => vec![
            GeneratedToken::word("Some"),
            group(GeneratedDelimiter::Parenthesis, payload.tokens().to_vec())?,
        ],
        RecipeRelationPayload::Transition { .. } => return Err(nothing_rendered()),
    };
    Ok(match_arm(vec![pattern], None, value))
}

fn exact_imports(
    left: &RecipeVocabulary,
    right: &RecipeVocabulary,
    table: &RelationTableProjection,
) -> Vec<GeneratedToken> {
    let Some([import_left, import_right]) = table.imports().copied() else {
        return Vec::new();
    };
    let mut imports = Vec::new();
    if import_left {
        imports.extend(use_item(super_super_path(left.name_token()), None));
    }
    if import_right && left.name() != right.name() {
        imports.extend(use_item(super_super_path(right.name_token()), None));
    }
    imports
}

fn borrowed(mut kind: Vec<GeneratedToken>) -> Vec<GeneratedToken> {
    kind.insert(0, GeneratedToken::alone('&'));
    kind
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

const fn nothing_rendered() -> ProjectionError {
    ProjectionError::Render(crate::render::RenderError::NothingRendered)
}
