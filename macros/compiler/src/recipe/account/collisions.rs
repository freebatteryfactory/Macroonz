//! Generated-name collisions between the standard projections and the authored module.

use super::{
    CODEC_LIMIT, PROJECTION_LIMIT, ProjectionStanding, RELATION_LIMIT, RecipeCodec, RecipeError,
    RecipeIssue, RecipeRelation, RecipeRelationPayloadKind, RecipeRole, VOCABULARY_LIMIT,
};
use crate::bounded::KeyedRoster;
use crate::codec::{DECODE_ROAD, ENCODE_ROAD};
use crate::recipe::names::{companion_constant, identifier_key};
use crate::token::SpanHandle;

pub(super) fn ensure_standard_names(
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<&str>,
) -> Result<(), RecipeError> {
    let Some((name, at)) =
        standard_name_collision(projections, vocabularies, relations, transition_relation)
    else {
        return Ok(());
    };
    Err(RecipeError::at(
        RecipeIssue::GeneratedNameCollision { name },
        Some(at),
    ))
}

fn standard_name_collision(
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<&str>,
) -> Option<(String, SpanHandle)> {
    let companions = matches!(
        RecipeRole::Companions.standing(projections),
        ProjectionStanding::Generated(_)
    );
    let mut names = if companions {
        companion_names(vocabularies, relations, transition_relation)
    } else {
        Vec::new()
    };
    if let ProjectionStanding::Generated(effective) =
        RecipeRole::RelationTables.standing(projections)
    {
        names.extend(
            effective
                .relation_tables()
                .map(|table| (table.relation().to_owned(), table.at())),
        );
    }
    if let ProjectionStanding::Generated(effective) = RecipeRole::Typestate.standing(projections) {
        names.push(("typestate".to_owned(), effective.at()));
    }
    for (position, (name, at)) in names.iter().enumerate() {
        if names
            .iter()
            .take(position)
            .any(|(earlier, _)| identifier_key(earlier) == identifier_key(name))
        {
            return Some((name.clone(), *at));
        }
    }
    let ProjectionStanding::Generated(dispatch) = RecipeRole::Dispatch.standing(projections) else {
        return None;
    };
    let name = dispatch.name().unwrap_or("apply");
    names
        .iter()
        .any(|(reserved, _)| identifier_key(reserved) == identifier_key(name))
        .then(|| (name.to_owned(), dispatch.at()))
}

pub(super) fn codec_surface_collision(
    codecs: &KeyedRoster<RecipeCodec, String, CODEC_LIMIT>,
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
) -> Option<(String, SpanHandle)> {
    let declarations = codecs.members().collect::<Vec<_>>();
    let mut reserved_types = Vec::new();
    if let ProjectionStanding::Generated(effective) =
        RecipeRole::RelationTables.standing(projections)
    {
        reserved_types.extend(
            effective
                .relation_tables()
                .map(super::RelationTableProjection::relation),
        );
    }
    if matches!(
        RecipeRole::Dispatch.standing(projections),
        ProjectionStanding::Generated(_)
    ) {
        reserved_types.push("TransitionRefusal");
    }
    if matches!(
        RecipeRole::Typestate.standing(projections),
        ProjectionStanding::Generated(_)
    ) {
        reserved_types.push("typestate");
    }
    for declaration in &declarations {
        let content = declaration.content();
        let refusal = content.shape.refusal();
        if content.direction.reads()
            && reserved_types
                .iter()
                .any(|reserved| identifier_key(reserved) == identifier_key(refusal))
        {
            return Some((refusal.to_owned(), declaration.refusal_at()));
        }
    }
    for (position, first) in declarations.iter().enumerate() {
        for second in declarations.iter().skip(position.saturating_add(1)) {
            let second_refusal_at = second.refusal_at();
            let second_direction_at = second.direction_at();
            let first = first.content();
            let second = second.content();
            if first.direction.reads()
                && second.direction.reads()
                && identifier_key(first.shape.refusal()) == identifier_key(second.shape.refusal())
            {
                return Some((second.shape.refusal().to_owned(), second_refusal_at));
            }
            if let Some(road) = shared_codec_road(first, second) {
                return Some((road.to_owned(), second_direction_at));
            }
        }
    }
    None
}

fn companion_names(
    vocabularies: Option<&KeyedRoster<super::RecipeVocabulary, String, VOCABULARY_LIMIT>>,
    relations: Option<&KeyedRoster<RecipeRelation, String, RELATION_LIMIT>>,
    transition_relation: Option<&str>,
) -> Vec<(String, SpanHandle)> {
    let mut names = vocabularies
        .into_iter()
        .flat_map(KeyedRoster::members)
        .map(|vocabulary| {
            (
                companion_constant(vocabulary.name(), "VARIANTS"),
                vocabulary.at,
            )
        })
        .collect::<Vec<_>>();
    for relation in relations.into_iter().flat_map(KeyedRoster::members) {
        if Some(relation.name()) == transition_relation {
            names.push(("TRANSITIONS".to_owned(), relation.name_at));
            continue;
        }
        names.push((
            companion_constant(relation.name(), "ROWS"),
            relation.name_at,
        ));
        if relation.payload_kind() != RecipeRelationPayloadKind::Unlabeled {
            names.push((
                companion_constant(relation.name(), "PAYLOADS"),
                relation.name_at,
            ));
        }
    }
    names
}

fn shared_codec_road(
    first: &crate::codec::CodecContent,
    second: &crate::codec::CodecContent,
) -> Option<&'static str> {
    if first.shape.owner() != second.shape.owner() {
        return None;
    }
    if first.direction.writes() && second.direction.writes() {
        return Some(ENCODE_ROAD);
    }
    (first.direction.reads() && second.direction.reads()).then_some(DECODE_ROAD)
}
