//! Restoring caller-authored spans onto the generated material that repeats them.

use super::{
    EffectiveProjection, RecipeRelation, RecipeRelationPayload, RecipeRelationRow,
    RecipeTransitionEffect,
};
use crate::token::GeneratedTree;

/// Restore every exact caller fragment retained by one relation.
pub(super) fn restore_relation_references(
    tree: &GeneratedTree,
    relation: &RecipeRelation,
) -> GeneratedTree {
    relation.rows().fold(tree.clone(), |restored, row| {
        restore_row_reference(&restored, row)
    })
}

/// Restore the exact caller fragment retained by one relation row.
fn restore_row_reference(tree: &GeneratedTree, row: &RecipeRelationRow) -> GeneratedTree {
    match row.payload() {
        RecipeRelationPayload::Unlabeled => tree.clone(),
        RecipeRelationPayload::Path(path) | RecipeRelationPayload::ExactRust(path) => {
            tree.restored_from(path)
        }
        RecipeRelationPayload::Transition { effect, .. } => match effect {
            RecipeTransitionEffect::Path(path) => tree.restored_from(path),
            RecipeTransitionEffect::ExactRust {
                target_binding,
                body,
            } => row.effect_binding_at().map_or_else(
                || tree.restored_from(body),
                |binding_at| tree.restored_body_from(body, target_binding, binding_at),
            ),
        },
    }
}

/// Restore every exact caller fragment retained by one effective projection.
pub(super) fn restore_projection_references(
    tree: &GeneratedTree,
    effective: &EffectiveProjection,
) -> GeneratedTree {
    let restored = match (effective.exact_rust(), effective.dispatch_binding_tokens()) {
        (Some(exact), Some(bindings)) => tree.restored_function_from(exact, bindings),
        (Some(exact), None) => tree.restored_from(exact),
        (None, _) => tree.clone(),
    };
    effective
        .relation_tables()
        .fold(restored, |restored, table| {
            match (table.exact_rust(), table.bindings()) {
                (Some(exact), Some(bindings)) => restored.restored_function_from(exact, bindings),
                (Some(exact), None) => restored.restored_from(exact),
                (None, _) => restored,
            }
        })
}
