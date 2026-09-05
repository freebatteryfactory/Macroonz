//! The cross-clause contracts an informed recipe must satisfy before projection.

use super::collisions::codec_surface_collision;
use super::settle::{missing_relation, missing_vocabulary, validate_transition_relation};
use super::{
    CODEC_LIMIT, PROJECTION_LIMIT, ProjectionStanding, RELATION_LIMIT, RecipeCodec, RecipeError,
    RecipeEvidence, RecipeIssue, RecipeRelation, RecipeRelationPayloadKind, RecipeRole,
    VOCABULARY_LIMIT,
};
use crate::bounded::KeyedRoster;
use crate::relation::AbsencePosture;

pub(super) fn ensure_evidence_targets(
    evidence: &[Option<RecipeEvidence>],
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
) -> Result<(), RecipeError> {
    for declaration in evidence.iter().flatten() {
        let Some(target) = declaration.target() else {
            continue;
        };
        if vocabularies
            .and_then(|vocabularies| vocabularies.get(target.name()))
            .is_none()
        {
            return Err(RecipeError::at(
                RecipeIssue::VocabularyNotFound {
                    name: target.name().to_owned(),
                },
                Some(declaration.at()),
            ));
        }
    }
    Ok(())
}

pub(super) fn ensure_transition_account(
    transition_relation: Option<&str>,
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
) -> Result<(), RecipeError> {
    let Some(name) = transition_relation else {
        return Ok(());
    };
    let relation = relations
        .and_then(|relations| relations.get(name))
        .ok_or_else(|| missing_relation(name))?;
    let vocabularies =
        vocabularies.ok_or_else(|| missing_vocabulary(relation.left_vocabulary(), None))?;
    validate_transition_relation(relation, vocabularies)
}

fn selected_roles(projections: &[ProjectionStanding; PROJECTION_LIMIT]) -> Vec<RecipeRole> {
    RecipeRole::ALL
        .iter()
        .copied()
        .filter(|role| matches!(role.standing(projections), ProjectionStanding::Generated(_)))
        .collect()
}

pub(super) fn ensure_projection_contracts(
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
    codecs: Option<&KeyedRoster<RecipeCodec, String, CODEC_LIMIT>>,
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<&str>,
) -> Result<(), RecipeError> {
    let selected = selected_roles(projections);
    if selected.is_empty() {
        return Err(RecipeError::at(RecipeIssue::ProjectionRequired, None));
    }
    ensure_codec_projection(&selected, codecs, projections)?;
    ensure_relation_table_projection(&selected, relations, projections)?;
    ensure_projection_dependencies(&selected)?;
    ensure_dispatch_projection(&selected, relations, transition_relation)
}

fn ensure_relation_table_projection(
    selected: &[RecipeRole],
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
) -> Result<(), RecipeError> {
    if !selected.contains(&RecipeRole::RelationTables) {
        return Ok(());
    }
    let ProjectionStanding::Generated(effective) = RecipeRole::RelationTables.standing(projections)
    else {
        return Err(RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::RelationTables,
                expected: "at least one caller-named relation",
            },
            None,
        ));
    };
    let tables = effective.relation_tables().collect::<Vec<_>>();
    if tables.is_empty() {
        return Err(RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::RelationTables,
                expected: "at least one caller-named relation",
            },
            None,
        ));
    }
    for table in tables {
        let relation = relations
            .and_then(|relations| relations.get(table.relation()))
            .ok_or_else(|| missing_relation(table.relation()))?;
        match relation.payload_kind() {
            RecipeRelationPayloadKind::Unlabeled => {}
            RecipeRelationPayloadKind::Path | RecipeRelationPayloadKind::ExactRust
                if table.exact_rust().is_some() => {}
            RecipeRelationPayloadKind::Path | RecipeRelationPayloadKind::ExactRust => {
                return Err(RecipeError::at(
                    RecipeIssue::RelationTableExactRequired {
                        relation: relation.name().to_owned(),
                    },
                    None,
                ));
            }
            RecipeRelationPayloadKind::Transition => {
                return Err(RecipeError::at(
                    RecipeIssue::RelationTableTransitionUnsupported {
                        relation: relation.name().to_owned(),
                    },
                    None,
                ));
            }
        }
    }
    Ok(())
}

fn ensure_codec_projection(
    selected: &[RecipeRole],
    codecs: Option<&KeyedRoster<RecipeCodec, String, CODEC_LIMIT>>,
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
) -> Result<(), RecipeError> {
    if !selected.contains(&RecipeRole::Codec) {
        return Ok(());
    }
    let Some(codecs) = codecs else {
        return Err(RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::Codec,
                expected: "at least one existing-owner codec declaration",
            },
            None,
        ));
    };
    if let Some((name, at)) = codec_surface_collision(codecs, projections) {
        return Err(RecipeError::at(
            RecipeIssue::GeneratedNameCollision { name },
            Some(at),
        ));
    }
    Ok(())
}

fn ensure_projection_dependencies(selected: &[RecipeRole]) -> Result<(), RecipeError> {
    for role in [
        RecipeRole::CompileContract,
        RecipeRole::DeclarationConformance,
    ] {
        if selected.contains(&role) && !selected.contains(&RecipeRole::Dispatch) {
            return Err(RecipeError::at(
                RecipeIssue::ProjectionDependencyAbsent {
                    role,
                    required: RecipeRole::Dispatch,
                },
                None,
            ));
        }
    }
    Ok(())
}

fn ensure_dispatch_projection(
    selected: &[RecipeRole],
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<&str>,
) -> Result<(), RecipeError> {
    if !selected.contains(&RecipeRole::Dispatch) {
        return Ok(());
    }
    let Some(transition_relation) = transition_relation else {
        return Err(RecipeError::at(
            RecipeIssue::ProjectionSubjectRequired {
                role: RecipeRole::Dispatch,
                expected: "one typed transition lowering",
            },
            None,
        ));
    };
    let absence = relations
        .and_then(|relations| relations.get(transition_relation))
        .and_then(|relation| relation.requirements.absence);
    if absence == Some(AbsencePosture::Allowed) {
        return Err(RecipeError::at(
            RecipeIssue::AllowedAbsenceNeedsFallback,
            None,
        ));
    }
    Ok(())
}
