//! Re-declaring an explicit transition seat through the ordinary recipe invariant nucleus.

use super::{
    Recipe, RecipeEdit, RecipeEditError, RecipeParts, RecipeRelation, RecipeRelationParts,
    RecipeRelationPayload, RecipeVocabulary, RecipeVocabularyParts,
};
use crate::request::Door;

impl Recipe {
    pub(crate) fn edited(&self, edit: &RecipeEdit, door: &Door) -> Result<Self, RecipeEditError> {
        let relation = self
            .transition_relation()
            .ok_or(RecipeEditError::TransitionRequired)?;
        let mut parts = self.redeclared_parts();
        let changed = parts
            .relations
            .iter_mut()
            .find(|row| row.name == relation.name())
            .ok_or(RecipeEditError::TransitionRequired)?;
        match edit {
            RecipeEdit::TransitionTarget { row, target } => {
                let member = self
                    .vocabulary(relation.left_vocabulary())
                    .and_then(|vocabulary| vocabulary.members().get(target.as_str()))
                    .ok_or_else(|| RecipeEditError::TargetAbsent {
                        spelling: target.clone(),
                    })?;
                let row = changed
                    .rows
                    .get_mut(*row)
                    .ok_or(RecipeEditError::RowAbsent { position: *row })?;
                let RecipeRelationPayload::Transition {
                    target: spelling,
                    target_name,
                    ..
                } = &mut row.payload
                else {
                    return Err(RecipeEditError::TransitionRequired);
                };
                spelling.clone_from(&member.spelling);
                *target_name = member.name.clone();
            }
            RecipeEdit::TransitionEffect { row, from_row } => {
                let provider = changed
                    .rows
                    .get(*from_row)
                    .ok_or(RecipeEditError::RowAbsent {
                        position: *from_row,
                    })?;
                let RecipeRelationPayload::Transition { effect, .. } = &provider.payload else {
                    return Err(RecipeEditError::TransitionRequired);
                };
                let effect = effect.clone();
                let binding_at = provider.effect_binding_at;
                let changed_row = changed
                    .rows
                    .get_mut(*row)
                    .ok_or(RecipeEditError::RowAbsent { position: *row })?;
                let RecipeRelationPayload::Transition { effect: seat, .. } =
                    &mut changed_row.payload
                else {
                    return Err(RecipeEditError::TransitionRequired);
                };
                *seat = effect;
                changed_row.effect_binding_at = binding_at;
            }
        }
        Self::informed(parts).map_err(|refusal| {
            RecipeEditError::Compiler(crate::recipe::bake::recipe_refused(&refusal, door))
        })
    }

    fn redeclared_parts(&self) -> RecipeParts {
        RecipeParts {
            module_name: self.module_name.clone(),
            module_name_token: self.module_name_token.clone(),
            module_head: self.module_head.clone(),
            authored_body: self.authored_body.clone(),
            authored_declaration: self.authored_declaration.clone(),
            module_body_at: self.module_body_at,
            vocabularies: self.vocabularies().map(vocabulary_parts).collect(),
            relations: self.relations().map(relation_parts).collect(),
            transition_relation: self.transition_relation.clone(),
            codecs: self.codecs().cloned().collect(),
            projections: self.projections.clone(),
            evidence: self.evidence.clone(),
            support: self.support.clone(),
        }
    }
}

fn vocabulary_parts(vocabulary: &RecipeVocabulary) -> RecipeVocabularyParts {
    RecipeVocabularyParts {
        name: vocabulary.name.clone(),
        name_token: vocabulary.name_token.clone(),
        members: vocabulary.members.members().cloned().collect(),
        at: vocabulary.at,
    }
}

fn relation_parts(relation: &RecipeRelation) -> RecipeRelationParts {
    RecipeRelationParts {
        name: relation.name.clone(),
        name_token: relation.name_token.clone(),
        name_at: relation.name_at,
        left_vocabulary: relation.left_vocabulary.clone(),
        left_vocabulary_at: relation.name_at,
        right_vocabulary: relation.right_vocabulary.clone(),
        right_vocabulary_at: relation.name_at,
        rows: relation.rows.iter().cloned().collect(),
        requirements: relation.requirements,
    }
}
