//! Structural transition and effect alternatives rendered by the ordinary recipe compiler.

use super::{
    ALTERNATIVE_LIMIT, Alternative, Declaration, EFFECT_BINDING_FAMILY, FamilySlug,
    RecipeMutationError, Surface, TRANSITION_TARGET_FAMILY,
};
use crate::descriptor::{DeclarationError, Seat};
use crate::identity::encode_bytes;
use crate::kind::CanonicalContent;
use crate::recipe::{
    self, HarnessPosture, Recipe, RecipeEdit, RecipeEditError, RecipeRelationPayload,
};
use crate::request::Door;
use crate::token::{CapturedInput, GeneratedToken};

/// Complete one row's destination pressure with every other member of its declared source vocabulary.
///
/// The [mutation home](super) owns the source material, scope and evidence ceiling.
///
/// # Errors
///
/// Returns [`RecipeMutationError`] before partial delivery if the row or any required source cannot be produced.
pub fn completed_from_transition_targets(
    declaration: Declaration,
    capture: &CapturedInput,
    harness: HarnessPosture,
    row: usize,
    door: &Door,
) -> Result<Surface, RecipeMutationError> {
    let recipe =
        recipe::read_recipe(capture, harness, door).map_err(RecipeMutationError::Compiler)?;
    let relation = recipe
        .transition_relation()
        .ok_or_else(transition_required)?;
    let selected = relation.rows().nth(row).ok_or_else(|| row_absent(row))?;
    let RecipeRelationPayload::Transition { target, .. } = selected.payload() else {
        return Err(transition_required());
    };
    let vocabulary = recipe
        .vocabulary(relation.left_vocabulary())
        .ok_or_else(transition_required)?;
    let edits = vocabulary
        .members()
        .members()
        .filter(|member| member.spelling() != target)
        .map(|member| RecipeEdit::TransitionTarget {
            row,
            target: member.spelling().to_owned(),
        })
        .collect::<Vec<_>>();
    complete(
        declaration,
        capture,
        &recipe,
        &edits,
        TRANSITION_TARGET_FAMILY,
        door,
    )
}

/// Complete one row's effect pressure with the distinct effect bindings already declared by sibling rows.
///
/// The [mutation home](super) owns the source material, scope and evidence ceiling.
///
/// # Errors
///
/// Returns [`RecipeMutationError`] before partial delivery if the row or any required source cannot be produced.
pub fn completed_from_effect_bindings(
    declaration: Declaration,
    capture: &CapturedInput,
    harness: HarnessPosture,
    row: usize,
    door: &Door,
) -> Result<Surface, RecipeMutationError> {
    let recipe =
        recipe::read_recipe(capture, harness, door).map_err(RecipeMutationError::Compiler)?;
    let relation = recipe
        .transition_relation()
        .ok_or_else(transition_required)?;
    if relation.rows().nth(row).is_none() {
        return Err(row_absent(row));
    }
    let edits = (0..relation.row_count())
        .filter(|from_row| *from_row != row)
        .map(|from_row| RecipeEdit::TransitionEffect { row, from_row })
        .collect::<Vec<_>>();
    complete(
        declaration,
        capture,
        &recipe,
        &edits,
        EFFECT_BINDING_FAMILY,
        door,
    )
}

fn complete(
    declaration: Declaration,
    capture: &CapturedInput,
    recipe: &Recipe,
    edits: &[RecipeEdit],
    family: &str,
    door: &Door,
) -> Result<Surface, RecipeMutationError> {
    let unchanged = operation(recipe, family);
    let mut candidates = Vec::new();
    for edit in edits {
        let changed = recipe
            .edited(edit, door)
            .map_err(RecipeMutationError::Edit)?;
        let operation = operation(&changed, family);
        if operation != unchanged && !candidates.iter().any(|(bytes, _)| *bytes == operation) {
            candidates.push((operation, changed));
        }
    }
    if candidates.is_empty() {
        return Err(RecipeMutationError::NoAlternative);
    }
    if candidates.len() > ALTERNATIVE_LIMIT {
        return Err(RecipeMutationError::Declaration(
            DeclarationError::unbounded(Seat::Alternative, ALTERNATIVE_LIMIT, candidates.len()),
        ));
    }
    let family = FamilySlug::declared(family).map_err(RecipeMutationError::Declaration)?;
    let production = source(capture, recipe, door)?;
    let alternatives = candidates
        .into_iter()
        .map(|(bytes, changed)| {
            Alternative::stated(family.clone(), bytes, source(capture, &changed, door)?)
                .map_err(RecipeMutationError::Declaration)
        })
        .collect::<Result<Vec<_>, _>>()?;
    declaration
        .completed(
            vec![
                GeneratedToken::alone('&'),
                GeneratedToken::joint('\''),
                GeneratedToken::word("static"),
                GeneratedToken::word("str"),
            ],
            production,
            unchanged,
            alternatives,
        )
        .map_err(RecipeMutationError::Declaration)
}

fn operation(recipe: &Recipe, family: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    encode_bytes(b"recipe-structural-change/v1", &mut bytes);
    encode_bytes(family.as_bytes(), &mut bytes);
    encode_bytes(&recipe.canonical_content_bytes(), &mut bytes);
    bytes
}

fn source(
    capture: &CapturedInput,
    recipe: &Recipe,
    door: &Door,
) -> Result<Vec<GeneratedToken>, RecipeMutationError> {
    let baked =
        recipe::bake_informed(capture, recipe, door, &[]).map_err(RecipeMutationError::Compiler)?;
    let source = baked
        .emit()
        .tokens()
        .ok_or(RecipeMutationError::SourceAbsent)?
        .inspected();
    Ok(vec![GeneratedToken::text(&source)])
}

fn transition_required() -> RecipeMutationError {
    RecipeMutationError::Edit(RecipeEditError::TransitionRequired)
}

fn row_absent(position: usize) -> RecipeMutationError {
    RecipeMutationError::Edit(RecipeEditError::RowAbsent { position })
}
