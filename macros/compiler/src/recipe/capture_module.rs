//! The authored module boundary, bake suffix, generated-name firewall, and enum-member lenses.

use super::{BAKE, RecipeError, RecipeIssue, RecipeMember, identifier_token};
use crate::token::{CapturedDelimiter, CapturedFragment, CapturedTokenTree};

/// Split the required final `bake! { ... }` suffix from the authored module body.
pub(super) fn bake_suffix(
    body: CapturedFragment<'_>,
) -> Result<(&[CapturedTokenTree], CapturedFragment<'_>), RecipeError> {
    let tokens = body.tokens();
    let suffix = match tokens {
        [authored @ .., name, bang, group]
            if name.word() == Some(BAKE)
                && bang.punct() == Some('!')
                && group.group_fragment(CapturedDelimiter::Brace).is_some() =>
        {
            (authored, group)
        }
        [authored @ .., name, bang, group, end]
            if name.word() == Some(BAKE)
                && bang.punct() == Some('!')
                && group.group_fragment(CapturedDelimiter::Brace).is_some()
                && end.punct() == Some(';') =>
        {
            (authored, group)
        }
        _ => {
            return Err(RecipeError::at(
                RecipeIssue::BakeRequiredLast,
                body.last_span().or(body.enclosing_span()),
            ));
        }
    };
    if let Some(duplicate) = suffix.0.windows(3).find_map(|window| {
        let [name, bang, group] = window else {
            return None;
        };
        (name.word() == Some(BAKE)
            && bang.punct() == Some('!')
            && group.group_fragment(CapturedDelimiter::Brace).is_some())
        .then_some(name)
    }) {
        return Err(RecipeError::at(
            RecipeIssue::BakeRequiredLast,
            Some(duplicate.span()),
        ));
    }
    let declaration = suffix
        .1
        .group_fragment(CapturedDelimiter::Brace)
        .ok_or_else(|| RecipeError::at(RecipeIssue::BakeRequiredLast, Some(suffix.1.span())))?;
    Ok((suffix.0, declaration))
}

/// Refuse a direct authored type-namespace occupant of the generated child name before any projector runs.
pub(super) fn collision_free(authored: &[CapturedTokenTree]) -> Result<(), RecipeError> {
    for pair in authored.windows(2) {
        let [kind, name] = pair else {
            continue;
        };
        if matches!(
            kind.word(),
            Some("mod" | "struct" | "enum" | "union" | "trait" | "type")
        ) && name
            .word()
            .or_else(|| name.raw_identifier())
            .is_some_and(|spelling| spelling == "baked")
        {
            return Err(generated_name_collision(name));
        }
    }
    for triple in authored.windows(3) {
        let [external, crate_word, name] = triple else {
            continue;
        };
        if external.word() == Some("extern")
            && crate_word.word() == Some("crate")
            && name
                .word()
                .or_else(|| name.raw_identifier())
                .is_some_and(|spelling| spelling == "baked")
        {
            return Err(generated_name_collision(name));
        }
    }
    Ok(())
}

fn generated_name_collision(name: &CapturedTokenTree) -> RecipeError {
    RecipeError::at(
        RecipeIssue::GeneratedNameCollision {
            name: "baked".to_owned(),
        },
        Some(name.span()),
    )
}

/// Read one named authored enum and its unit-variant roster.
pub(super) fn enum_members(
    authored: &[CapturedTokenTree],
    sought: &str,
) -> Result<Vec<RecipeMember>, RecipeError> {
    let found = authored.windows(2).position(|pair| {
        matches!(pair, [kind, name]
            if kind.word() == Some("enum")
                && name
                    .word()
                    .or_else(|| name.raw_identifier())
                    .is_some_and(|spelling| spelling == sought))
    });
    let Some(position) = found else {
        return Err(RecipeError::at(
            RecipeIssue::VocabularyNotFound {
                name: sought.to_owned(),
            },
            authored.first().map(CapturedTokenTree::span),
        ));
    };
    let name_position = position.saturating_add(1);
    let Some(name) = authored.get(name_position) else {
        return Err(RecipeError::at(
            RecipeIssue::VocabularyNotFound {
                name: sought.to_owned(),
            },
            authored.first().map(CapturedTokenTree::span),
        ));
    };
    let body_position = name_position.saturating_add(1);
    let body = authored
        .get(body_position..)
        .and_then(|after_name| {
            after_name
                .iter()
                .find_map(|candidate| candidate.group_fragment(CapturedDelimiter::Brace))
        })
        .ok_or_else(|| {
            RecipeError::at(
                RecipeIssue::VocabularyNotFound {
                    name: sought.to_owned(),
                },
                Some(name.span()),
            )
        })?;
    unit_variants(body, sought)
}

fn unit_variants(
    body: CapturedFragment<'_>,
    vocabulary: &str,
) -> Result<Vec<RecipeMember>, RecipeError> {
    let mut members = Vec::new();
    for row in variant_rows(body.tokens()) {
        if row.is_empty() {
            continue;
        }
        members.push(member(row, vocabulary)?);
    }
    Ok(members)
}

fn member(row: &[CapturedTokenTree], vocabulary: &str) -> Result<RecipeMember, RecipeError> {
    let stripped = without_outer_attributes(row);
    let (token, spelling) = match stripped {
        [token] => identifier(token),
        [token, equals, discriminant @ ..]
            if equals.punct() == Some('=') && !discriminant.is_empty() =>
        {
            identifier(token)
        }
        [token, ..] => (token, None),
        [] => {
            return Err(RecipeError::at(
                RecipeIssue::VariantNotUnit {
                    vocabulary: vocabulary.to_owned(),
                    variant: "<unnamed>".to_owned(),
                },
                row.first().map(CapturedTokenTree::span),
            ));
        }
    };
    spelling
        .map(|name| {
            RecipeMember::authored(name.to_owned(), identifier_token(token, name), token.span())
        })
        .ok_or_else(|| variant_refusal(token, vocabulary))
}

fn variant_rows(tokens: &[CapturedTokenTree]) -> Vec<&[CapturedTokenTree]> {
    let mut rows = Vec::new();
    let mut opening = 0usize;
    for (position, token) in tokens.iter().enumerate() {
        if token.punct() == Some(',') {
            if let Some(row) = tokens.get(opening..position) {
                rows.push(row);
            }
            opening = position.saturating_add(1);
        }
    }
    if let Some(row) = tokens.get(opening..tokens.len()) {
        rows.push(row);
    }
    rows
}

fn without_outer_attributes(mut row: &[CapturedTokenTree]) -> &[CapturedTokenTree] {
    loop {
        match row {
            [hash, attribute, rest @ ..]
                if hash.punct() == Some('#')
                    && attribute
                        .group_fragment(CapturedDelimiter::Bracket)
                        .is_some() =>
            {
                row = rest;
            }
            _ => return row,
        }
    }
}

fn identifier(token: &CapturedTokenTree) -> (&CapturedTokenTree, Option<&str>) {
    (token, token.word().or_else(|| token.raw_identifier()))
}

fn variant_refusal(token: &CapturedTokenTree, vocabulary: &str) -> RecipeError {
    let spelling = token
        .word()
        .or_else(|| token.raw_identifier())
        .unwrap_or("<unnamed>");
    RecipeError::at(
        RecipeIssue::VariantNotUnit {
            vocabulary: vocabulary.to_owned(),
            variant: spelling.to_owned(),
        },
        Some(token.span()),
    )
}
