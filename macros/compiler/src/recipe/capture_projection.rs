//! Projection selection and the complete projection standing account.

use super::dispatch::{exact_dispatch, exact_relation_table};
use super::evidence::evidence_standing;
use super::{
    CapturedRelation, EffectiveProjection, HarnessPosture, LoweringSource, PROJECTION_LIMIT,
    ProjectionStanding, RELATION_TABLE_LIMIT, RecipeError, RecipeIssue, RecipeRole,
    RelationTableProjection, RequestedEvidence, RequestedProjection, RequestedRelationTable,
};
use crate::bounded::Bounded;
use crate::token::{
    CaptureCursor, CaptureReadRefusal, CapturedDelimiter, CapturedInput, SpanHandle,
};

/// Read one projection request.
pub(super) fn read_projection(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
) -> Result<RequestedProjection, CaptureReadRefusal> {
    let (token, spelling) = cursor.identifier()?;
    let at = token.span();
    match spelling {
        "companions" => Ok(simple(RecipeRole::Companions, at)),
        "relation_tables" => read_relation_tables(cursor, issued, at),
        "dispatch" => read_dispatch(cursor, issued, at),
        "compile_contract" => Ok(simple(RecipeRole::CompileContract, at)),
        "property" => Ok(simple(RecipeRole::Property, at)),
        "typestate" => read_typestate(cursor, at),
        "codec" => Ok(simple(RecipeRole::Codec, at)),
        _ => Err(CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Word(
                "a recipe projection".to_owned(),
            )),
            Some(at),
        )),
    }
}

fn simple(role: RecipeRole, at: SpanHandle) -> RequestedProjection {
    requested(role, None, None, LoweringSource::Preset, None, at)
}

fn read_relation_tables(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
    at: SpanHandle,
) -> Result<RequestedProjection, CaptureReadRefusal> {
    let tables = cursor
        .group(CapturedDelimiter::Brace)?
        .trailing_separated::<_, { RELATION_TABLE_LIMIT }>(';', |table| {
            read_relation_table(table, issued)
        })?
        .as_slice()
        .to_vec();
    Ok(RequestedProjection {
        role: RecipeRole::RelationTables,
        name: None,
        subject: None,
        source: LoweringSource::Configuration,
        exact: None,
        relation_tables: Some(tables),
        at,
    })
}

fn read_relation_table(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
) -> Result<RequestedRelationTable, CaptureReadRefusal> {
    let (relation_token, relation) = cursor.identifier()?;
    let at = relation_token.span();
    let Some(next) = cursor.next_token() else {
        return Ok(RequestedRelationTable {
            relation: relation.to_owned(),
            function: None,
            source: LoweringSource::Preset,
            exact: None,
            at,
        });
    };
    if next.punct() == Some(';') {
        return Ok(RequestedRelationTable {
            relation: relation.to_owned(),
            function: None,
            source: LoweringSource::Preset,
            exact: None,
            at,
        });
    }
    if next
        .group_fragment(CapturedDelimiter::Parenthesis)
        .is_some()
    {
        let mut configured = cursor.group(CapturedDelimiter::Parenthesis)?;
        let (name_token, name) = configured.identifier()?;
        if name_token.raw_identifier().is_some() {
            return Err(CaptureReadRefusal::projected(
                crate::token::CaptureReadIssue::Unexpected(
                    crate::token::CaptureExpectation::Identifier,
                ),
                Some(name_token.span()),
            ));
        }
        configured.finish()?;
        return Ok(RequestedRelationTable {
            relation: relation.to_owned(),
            function: Some(name.to_owned()),
            source: LoweringSource::Configuration,
            exact: None,
            at,
        });
    }
    if let Some(fragment) = next.group_fragment(CapturedDelimiter::Brace) {
        cursor.token()?;
        let exact = CapturedInput::selected(fragment, issued).map_err(|_| {
            CaptureReadRefusal::projected(
                crate::token::CaptureReadIssue::SequenceUnbounded {
                    limit: crate::token::CAPTURED_TOKEN_LIMIT,
                },
                Some(next.span()),
            )
        })?;
        return Ok(RequestedRelationTable {
            relation: relation.to_owned(),
            function: None,
            source: LoweringSource::ExactRust,
            exact: Some(exact),
            at,
        });
    }
    Err(CaptureReadRefusal::projected(
        crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Group(
            CapturedDelimiter::Brace,
        )),
        Some(next.span()),
    ))
}

fn read_dispatch(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
    at: SpanHandle,
) -> Result<RequestedProjection, CaptureReadRefusal> {
    let Some(next) = cursor.next_token() else {
        return Ok(simple(RecipeRole::Dispatch, at));
    };
    if next.punct() == Some(';') {
        return Ok(simple(RecipeRole::Dispatch, at));
    }
    if next
        .group_fragment(CapturedDelimiter::Parenthesis)
        .is_some()
    {
        let mut configured = cursor.group(CapturedDelimiter::Parenthesis)?;
        let (configured_name, configured_spelling) = configured.identifier()?;
        if configured_name.raw_identifier().is_some() {
            return Err(CaptureReadRefusal::projected(
                crate::token::CaptureReadIssue::Unexpected(
                    crate::token::CaptureExpectation::Identifier,
                ),
                Some(configured_name.span()),
            ));
        }
        configured.finish()?;
        return Ok(requested(
            RecipeRole::Dispatch,
            Some(configured_spelling.to_owned()),
            None,
            LoweringSource::Configuration,
            None,
            at,
        ));
    }
    if let Some(fragment) = next.group_fragment(CapturedDelimiter::Brace) {
        let exact_at = next.span();
        cursor.token()?;
        let exact = CapturedInput::selected(fragment, issued).map_err(|_| {
            CaptureReadRefusal::projected(
                crate::token::CaptureReadIssue::SequenceUnbounded {
                    limit: crate::token::CAPTURED_TOKEN_LIMIT,
                },
                Some(exact_at),
            )
        })?;
        return Ok(requested(
            RecipeRole::Dispatch,
            None,
            None,
            LoweringSource::ExactRust,
            Some(exact),
            at,
        ));
    }
    Err(CaptureReadRefusal::projected(
        crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Group(
            CapturedDelimiter::Parenthesis,
        )),
        Some(next.span()),
    ))
}

fn read_typestate(
    cursor: &mut CaptureCursor<'_>,
    at: SpanHandle,
) -> Result<RequestedProjection, CaptureReadRefusal> {
    let subject = if cursor.next_token().is_some_and(|next| {
        next.group_fragment(CapturedDelimiter::Parenthesis)
            .is_some()
    }) {
        let mut configured = cursor.group(CapturedDelimiter::Parenthesis)?;
        let (_subject_token, subject) = configured.identifier()?;
        configured.finish()?;
        Some(subject.to_owned())
    } else {
        None
    };
    let source = if subject.is_some() {
        LoweringSource::Configuration
    } else {
        LoweringSource::Preset
    };
    Ok(requested(
        RecipeRole::Typestate,
        None,
        subject,
        source,
        None,
        at,
    ))
}

fn requested(
    role: RecipeRole,
    name: Option<String>,
    subject: Option<String>,
    source: LoweringSource,
    exact: Option<CapturedInput>,
    at: SpanHandle,
) -> RequestedProjection {
    RequestedProjection {
        role,
        name,
        subject,
        source,
        exact,
        relation_tables: None,
        at,
    }
}

/// Build the complete projection account and enforce harness posture before planning.
pub(super) fn projections(
    requested: &[RequestedProjection],
    evidence: &[RequestedEvidence],
    harness: HarnessPosture,
    transition_subject: Option<(&str, &str)>,
    relations: &[CapturedRelation],
) -> Result<[ProjectionStanding; PROJECTION_LIMIT], RecipeError> {
    ensure_requested_admission(requested, harness)?;
    ensure_evidence_admission(evidence, harness)?;
    projection_standings(requested, evidence, harness, transition_subject, relations)
}

fn ensure_requested_admission(
    requested: &[RequestedProjection],
    harness: HarnessPosture,
) -> Result<(), RecipeError> {
    for (position, row) in requested.iter().enumerate() {
        if requested
            .iter()
            .take(position)
            .any(|earlier| earlier.role == row.role)
        {
            return Err(RecipeError::at(
                RecipeIssue::DuplicateProjection { role: row.role },
                Some(row.at),
            ));
        }
        if harness == HarnessPosture::Unavailable
            && matches!(row.role, RecipeRole::CompileContract | RecipeRole::Property)
        {
            return Err(RecipeError::at(
                RecipeIssue::HarnessUnavailable { role: row.role },
                Some(row.at),
            ));
        }
    }
    Ok(())
}

fn ensure_evidence_admission(
    evidence: &[RequestedEvidence],
    harness: HarnessPosture,
) -> Result<(), RecipeError> {
    for (position, row) in evidence.iter().enumerate() {
        if evidence
            .iter()
            .take(position)
            .any(|earlier| earlier.role == row.role)
        {
            return Err(RecipeError::at(
                RecipeIssue::DuplicateProjection { role: row.role },
                Some(row.at),
            ));
        }
        if harness == HarnessPosture::Unavailable && row.body.is_some() {
            return Err(RecipeError::at(
                RecipeIssue::HarnessUnavailable { role: row.role },
                Some(row.at),
            ));
        }
    }
    Ok(())
}

fn projection_standings(
    requested: &[RequestedProjection],
    evidence: &[RequestedEvidence],
    harness: HarnessPosture,
    transition_subject: Option<(&str, &str)>,
    relations: &[CapturedRelation],
) -> Result<[ProjectionStanding; PROJECTION_LIMIT], RecipeError> {
    Ok([
        standing(
            requested,
            RecipeRole::Companions,
            harness,
            transition_subject,
            relations,
        )?,
        standing(
            requested,
            RecipeRole::RelationTables,
            harness,
            transition_subject,
            relations,
        )?,
        standing(
            requested,
            RecipeRole::Dispatch,
            harness,
            transition_subject,
            relations,
        )?,
        standing(
            requested,
            RecipeRole::CompileContract,
            harness,
            transition_subject,
            relations,
        )?,
        standing(
            requested,
            RecipeRole::Property,
            harness,
            transition_subject,
            relations,
        )?,
        standing(
            requested,
            RecipeRole::Typestate,
            harness,
            transition_subject,
            relations,
        )?,
        evidence_standing(evidence, RecipeRole::Trials, harness),
        evidence_standing(evidence, RecipeRole::Mutation, harness),
        evidence_standing(evidence, RecipeRole::Benchmarks, harness),
        evidence_standing(evidence, RecipeRole::Network, harness),
        evidence_standing(evidence, RecipeRole::Concurrency, harness),
        standing(
            requested,
            RecipeRole::Codec,
            harness,
            transition_subject,
            relations,
        )?,
    ])
}

fn standing(
    requested: &[RequestedProjection],
    role: RecipeRole,
    harness: HarnessPosture,
    transition_subject: Option<(&str, &str)>,
    relations: &[CapturedRelation],
) -> Result<ProjectionStanding, RecipeError> {
    if harness == HarnessPosture::Unavailable
        && matches!(role, RecipeRole::CompileContract | RecipeRole::Property)
        && !requested.iter().any(|row| row.role == role)
    {
        return Ok(ProjectionStanding::FeatureUnavailable);
    }
    let Some(row) = requested.iter().find(|row| row.role == role) else {
        return Ok(ProjectionStanding::NotRequested);
    };
    if role == RecipeRole::RelationTables {
        return relation_table_standing(row, relations);
    }
    if let Some(exact) = row.exact.as_ref() {
        let (name, signature, bindings, imports) =
            exact_dispatch(exact, row.at, transition_subject)?;
        return Ok(ProjectionStanding::Generated(
            EffectiveProjection::exact_dispatch(name, signature, bindings, imports),
        ));
    }
    Ok(ProjectionStanding::Generated(
        EffectiveProjection::effective(role, row.name.clone(), row.subject.clone(), row.source),
    ))
}

fn relation_table_standing(
    requested: &RequestedProjection,
    relations: &[CapturedRelation],
) -> Result<ProjectionStanding, RecipeError> {
    let requested_tables = requested.relation_tables.as_deref().unwrap_or(&[]);
    let mut tables = Vec::new();
    for (position, table) in requested_tables.iter().enumerate() {
        if requested_tables
            .iter()
            .take(position)
            .any(|earlier| earlier.relation == table.relation)
        {
            return Err(RecipeError::at(
                RecipeIssue::DuplicateRelationTable {
                    relation: table.relation.clone(),
                },
                Some(table.at),
            ));
        }
        let relation = relations
            .iter()
            .find(|relation| relation.name.spelling == table.relation)
            .ok_or_else(|| {
                RecipeError::at(
                    RecipeIssue::RelationNotFound {
                        name: table.relation.clone(),
                    },
                    Some(table.at),
                )
            })?;
        let (function, exact_rust, bindings, imports) = match table.exact.as_ref() {
            Some(exact) => {
                let (name, signature, bindings, imports) = exact_relation_table(
                    exact,
                    table.at,
                    (
                        relation.left.spelling.as_str(),
                        relation.right.spelling.as_str(),
                    ),
                )?;
                (name, Some(signature), Some(bindings), Some(imports))
            }
            None => (
                table
                    .function
                    .clone()
                    .unwrap_or_else(|| "contains".to_owned()),
                None,
                None,
                None,
            ),
        };
        tables.push(RelationTableProjection::informed(
            table.relation.clone(),
            function,
            table.source,
            exact_rust,
            bindings,
            imports,
        ));
    }
    let tables = Bounded::new(tables).map_err(|_| {
        RecipeError::at(
            RecipeIssue::Grammar(crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: RELATION_TABLE_LIMIT,
            }),
            Some(requested.at),
        )
    })?;
    Ok(ProjectionStanding::Generated(
        EffectiveProjection::with_relation_tables(tables),
    ))
}
