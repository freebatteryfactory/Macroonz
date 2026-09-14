//! Projection grammar and deferred exact material read under account-owned selection.

use super::super::types::RecipeRoleEntrance;
use super::dispatch::{exact_dispatch, exact_relation_table};
use super::{
    CapturedRelation, EffectiveProjection, LoweringSource, RELATION_TABLE_LIMIT, RecipeError,
    RecipeIssue, RecipeRole, RelationTableProjection, RequestedProjection, RequestedRelationTable,
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
    let Some(role) = RecipeRole::from_syntax(spelling, RecipeRoleEntrance::Projection) else {
        return Err(CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Word(
                "a recipe projection".to_owned(),
            )),
            Some(at),
        ));
    };
    match role {
        RecipeRole::RelationTables => read_relation_tables(cursor, issued, at),
        RecipeRole::Dispatch => read_dispatch(cursor, issued, at),
        RecipeRole::Typestate => read_typestate(cursor, issued, at),
        RecipeRole::Companions
        | RecipeRole::CompileContract
        | RecipeRole::DeclarationConformance
        | RecipeRole::Codec => Ok(simple(role, at)),
        RecipeRole::Trials
        | RecipeRole::Mutation
        | RecipeRole::Benchmarks
        | RecipeRole::Network
        | RecipeRole::Concurrency => {
            unreachable!("the role profile admits only projection roles through this reader")
        }
    }
}

fn simple(role: RecipeRole, at: SpanHandle) -> RequestedProjection {
    requested(role, None, None, LoweringSource::Preset, None, None, at)
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
        dispatch_bindings: None,
        relation_tables: Some(tables),
        consuming: None,
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
        return read_parenthesized_dispatch(cursor, issued, at);
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
            None,
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

fn read_parenthesized_dispatch(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
    at: SpanHandle,
) -> Result<RequestedProjection, CaptureReadRefusal> {
    let mut configured = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (configured_name, configured_spelling) = configured.identifier()?;
    if configured.next_token().is_none() {
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
            None,
            configured_name.span(),
        ));
    }
    configured.punctuation(',', crate::token::CapturedSpacing::Alone)?;
    let (event_binding, event_spelling) = configured.identifier()?;
    if configured_spelling == event_spelling {
        return Err(CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Word(
                "two distinct dispatch bindings".to_owned(),
            )),
            Some(event_binding.span()),
        ));
    }
    configured.finish()?;
    read_selected_dispatch_signature(
        cursor,
        issued,
        [configured_spelling.to_owned(), event_spelling.to_owned()],
        configured_name.span(),
        at,
    )
}

fn read_selected_dispatch_signature(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
    bindings: [String; 2],
    binding_at: SpanHandle,
    at: SpanHandle,
) -> Result<RequestedProjection, CaptureReadRefusal> {
    let Some(exact_token) = cursor.next_token() else {
        return Err(CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::Missing(crate::token::CaptureExpectation::Group(
                CapturedDelimiter::Brace,
            )),
            Some(binding_at),
        ));
    };
    let Some(fragment) = exact_token.group_fragment(CapturedDelimiter::Brace) else {
        return Err(CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Group(
                CapturedDelimiter::Brace,
            )),
            Some(exact_token.span()),
        ));
    };
    let exact_at = exact_token.span();
    cursor.token()?;
    let exact = CapturedInput::selected(fragment, issued).map_err(|_| {
        CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: crate::token::CAPTURED_TOKEN_LIMIT,
            },
            Some(exact_at),
        )
    })?;
    Ok(requested(
        RecipeRole::Dispatch,
        None,
        None,
        LoweringSource::ExactRust,
        Some(exact),
        Some(bindings),
        at,
    ))
}

fn read_typestate(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
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
    let mut requested = requested(RecipeRole::Typestate, None, subject, source, None, None, at);
    if let Some(fragment) = cursor
        .next_token()
        .and_then(|token| token.group_fragment(CapturedDelimiter::Brace))
    {
        cursor.token()?;
        requested.consuming = Some(CapturedInput::selected(fragment, issued).map_err(|_| {
            CaptureReadRefusal::projected(
                crate::token::CaptureReadIssue::SequenceUnbounded {
                    limit: crate::token::CAPTURED_TOKEN_LIMIT,
                },
                Some(at),
            )
        })?);
        requested.source = LoweringSource::Configuration;
    }
    Ok(requested)
}

fn requested(
    role: RecipeRole,
    name: Option<String>,
    subject: Option<String>,
    source: LoweringSource,
    exact: Option<CapturedInput>,
    dispatch_bindings: Option<[String; 2]>,
    at: SpanHandle,
) -> RequestedProjection {
    RequestedProjection {
        role,
        name,
        subject,
        source,
        exact,
        dispatch_bindings,
        relation_tables: None,
        consuming: None,
        at,
    }
}

/// Interpret one admitted projection's deferred grammar.
pub(super) fn read_effective(
    row: &RequestedProjection,
    transition_subject: Option<(&str, &str)>,
    relations: &[CapturedRelation],
) -> Result<EffectiveProjection, RecipeError> {
    if row.role == RecipeRole::RelationTables {
        return relation_table_standing(row, relations);
    }
    if let Some(input) = row.consuming.as_ref() {
        return Ok(EffectiveProjection::with_consuming(
            row.subject.clone(),
            super::consuming::read(input)?,
            row.at,
        ));
    }
    if let Some(exact) = row.exact.as_ref() {
        let exact = exact_dispatch(
            exact,
            row.at,
            transition_subject,
            row.dispatch_bindings.as_ref(),
        )?;
        return Ok(EffectiveProjection::exact_dispatch(
            exact.name,
            exact.signature,
            exact.bindings,
            exact.binding_names,
            exact.imports,
            exact.name_at,
        ));
    }
    Ok(EffectiveProjection::effective(
        row.role,
        row.name.clone(),
        row.subject.clone(),
        row.source,
        row.at,
    ))
}

fn relation_table_standing(
    requested: &RequestedProjection,
    relations: &[CapturedRelation],
) -> Result<EffectiveProjection, RecipeError> {
    let requested_tables = requested.relation_tables.as_deref().unwrap_or(&[]);
    let mut tables = Vec::new();
    for (position, table) in requested_tables.iter().enumerate() {
        let subject = RelationTableProjection::declared_subject(
            table,
            requested_tables.iter().take(position),
            relations,
        )?;
        let (function, exact_rust, bindings, imports) = match table.exact.as_ref() {
            Some(exact) => {
                let (name, signature, bindings, imports) =
                    exact_relation_table(exact, table.at, subject)?;
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
            table.at,
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
    Ok(EffectiveProjection::with_relation_tables(
        tables,
        requested.at,
    ))
}
