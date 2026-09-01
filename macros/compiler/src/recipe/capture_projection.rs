//! Projection selection and the complete projection standing account.

use super::dispatch::exact_dispatch;
use super::evidence::evidence_standing;
use super::{
    EffectiveProjection, HarnessPosture, LoweringSource, ProjectionStanding, RecipeError,
    RecipeIssue, RecipeRole, RequestedEvidence, RequestedProjection,
};
use crate::token::{CaptureCursor, CaptureReadRefusal, CapturedDelimiter, CapturedInput};

/// Read one projection request.
pub(super) fn read_projection(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
) -> Result<RequestedProjection, CaptureReadRefusal> {
    let (token, spelling) = cursor.identifier()?;
    let (role, name, source, exact) = match spelling {
        "companions" => (RecipeRole::Companions, None, LoweringSource::Preset, None),
        "dispatch" => {
            let Some(next) = cursor.next_token() else {
                return Ok(RequestedProjection {
                    role: RecipeRole::Dispatch,
                    name: None,
                    source: LoweringSource::Preset,
                    exact: None,
                    at: token.span(),
                });
            };
            if next.punct() == Some(';') {
                (RecipeRole::Dispatch, None, LoweringSource::Preset, None)
            } else if next
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
                (
                    RecipeRole::Dispatch,
                    Some(configured_spelling.to_owned()),
                    LoweringSource::Configuration,
                    None,
                )
            } else if let Some(fragment) = next.group_fragment(CapturedDelimiter::Brace) {
                let at = next.span();
                cursor.token()?;
                let exact = CapturedInput::selected(fragment, issued).map_err(|_| {
                    CaptureReadRefusal::projected(
                        crate::token::CaptureReadIssue::SequenceUnbounded {
                            limit: crate::token::CAPTURED_TOKEN_LIMIT,
                        },
                        Some(at),
                    )
                })?;
                (
                    RecipeRole::Dispatch,
                    None,
                    LoweringSource::ExactRust,
                    Some(exact),
                )
            } else {
                return Err(CaptureReadRefusal::projected(
                    crate::token::CaptureReadIssue::Unexpected(
                        crate::token::CaptureExpectation::Group(CapturedDelimiter::Parenthesis),
                    ),
                    Some(next.span()),
                ));
            }
        }
        "compile_contract" => (
            RecipeRole::CompileContract,
            None,
            LoweringSource::Preset,
            None,
        ),
        "property" => (RecipeRole::Property, None, LoweringSource::Preset, None),
        "typestate" => (RecipeRole::Typestate, None, LoweringSource::Preset, None),
        _ => {
            return Err(CaptureReadRefusal::projected(
                crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Word(
                    "a recipe projection".to_owned(),
                )),
                Some(token.span()),
            ));
        }
    };
    Ok(RequestedProjection {
        role,
        name,
        source,
        exact,
        at: token.span(),
    })
}

/// Build the complete projection account and enforce harness posture before planning.
pub(super) fn projections(
    requested: &[RequestedProjection],
    evidence: &[RequestedEvidence],
    harness: HarnessPosture,
    states: &str,
    events: &str,
) -> Result<[ProjectionStanding; 10], RecipeError> {
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
    Ok([
        standing(requested, RecipeRole::Companions, harness, states, events)?,
        standing(requested, RecipeRole::Dispatch, harness, states, events)?,
        standing(
            requested,
            RecipeRole::CompileContract,
            harness,
            states,
            events,
        )?,
        standing(requested, RecipeRole::Property, harness, states, events)?,
        standing(requested, RecipeRole::Typestate, harness, states, events)?,
        evidence_standing(evidence, RecipeRole::Trials, harness),
        evidence_standing(evidence, RecipeRole::Mutation, harness),
        evidence_standing(evidence, RecipeRole::Benchmarks, harness),
        evidence_standing(evidence, RecipeRole::Network, harness),
        evidence_standing(evidence, RecipeRole::Concurrency, harness),
    ])
}

fn standing(
    requested: &[RequestedProjection],
    role: RecipeRole,
    harness: HarnessPosture,
    states: &str,
    events: &str,
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
    if let Some(exact) = row.exact.as_ref() {
        let (name, signature, bindings, imports) = exact_dispatch(exact, row.at, states, events)?;
        return Ok(ProjectionStanding::Generated(
            EffectiveProjection::exact_dispatch(name, signature, bindings, imports),
        ));
    }
    Ok(ProjectionStanding::Generated(
        EffectiveProjection::effective(role, row.name.clone(), row.source),
    ))
}
