//! Descriptor-native evidence clauses, support address posture, and evidence standings.

use super::{
    EVIDENCE_LIMIT, EffectiveProjection, EvidenceTarget, HarnessPosture, LoweringSource,
    ProjectionStanding, RecipeError, RecipeEvidence, RecipeIssue, RecipeRole, RequestedEvidence,
    grammar,
};
use crate::support::SupportName;
use crate::token::{
    CaptureCursor, CaptureReadRefusal, CapturedDelimiter, CapturedInput, CapturedSpacing,
};

pub(super) fn read_evidence_block(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
) -> Result<Vec<RequestedEvidence>, RecipeError> {
    if cursor.next_word() != Some("evidence") {
        return Ok(Vec::new());
    }
    cursor.word("evidence").map_err(grammar)?;
    let rows = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, EVIDENCE_LIMIT>(';', |row| read_evidence(row, issued))
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(rows)
}

fn read_evidence(
    cursor: &mut CaptureCursor<'_>,
    issued: usize,
) -> Result<RequestedEvidence, CaptureReadRefusal> {
    let (token, spelling) = cursor.identifier()?;
    let role = match spelling {
        "trials" => RecipeRole::Trials,
        "mutation" => RecipeRole::Mutation,
        "benchmarks" => RecipeRole::Benchmarks,
        "network" => RecipeRole::Network,
        "concurrency" => RecipeRole::Concurrency,
        _ => {
            return Err(CaptureReadRefusal::projected(
                crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Word(
                    "a descriptor-native evidence projection".to_owned(),
                )),
                Some(token.span()),
            ));
        }
    };
    if cursor.next_word() == Some("unavailable") {
        cursor.word("unavailable")?;
        return Ok(RequestedEvidence {
            role,
            target: None,
            body: None,
            at: token.span(),
        });
    }
    let target = if role == RecipeRole::Mutation {
        let mut selected = cursor.group(CapturedDelimiter::Parenthesis)?;
        let (target_token, target) = selected.identifier()?;
        let target = match target {
            "states" => EvidenceTarget::States,
            "events" => EvidenceTarget::Events,
            _ => {
                return Err(CaptureReadRefusal::projected(
                    crate::token::CaptureReadIssue::Unexpected(
                        crate::token::CaptureExpectation::Word("states or events".to_owned()),
                    ),
                    Some(target_token.span()),
                ));
            }
        };
        selected.finish()?;
        Some(target)
    } else {
        None
    };
    let group = cursor.token()?;
    let Some(fragment) = group.group_fragment(CapturedDelimiter::Brace) else {
        return Err(CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Group(
                CapturedDelimiter::Brace,
            )),
            Some(group.span()),
        ));
    };
    let body = CapturedInput::selected(fragment, issued).map_err(|_| {
        CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::SequenceUnbounded {
                limit: crate::token::CAPTURED_TOKEN_LIMIT,
            },
            Some(group.span()),
        )
    })?;
    Ok(RequestedEvidence {
        role,
        target,
        body: Some(body),
        at: token.span(),
    })
}

pub(super) fn read_support(
    cursor: &mut CaptureCursor<'_>,
) -> Result<Option<SupportName>, RecipeError> {
    if cursor.is_finished() {
        return Ok(None);
    }
    cursor.word("support").map_err(grammar)?;
    let mut address = cursor
        .group(CapturedDelimiter::Parenthesis)
        .map_err(grammar)?;
    let (token, spelling) = address.identifier().map_err(grammar)?;
    let declared = SupportName::declared(spelling).map_err(|_| {
        RecipeError::at(
            RecipeIssue::GeneratedNameNotIdentifier {
                name: spelling.to_owned(),
            },
            Some(token.span()),
        )
    })?;
    address.finish().map_err(grammar)?;
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    Ok(Some(declared))
}

pub(super) fn support_matches_projections(
    projections: &[ProjectionStanding; 10],
    support: Option<&SupportName>,
    at: Option<crate::token::SpanHandle>,
) -> Result<(), RecipeError> {
    let evidence = generated(projections, RecipeRole::CompileContract)
        || generated(projections, RecipeRole::Property);
    match (evidence, support.is_some()) {
        (true, false) => Err(RecipeError::at(RecipeIssue::SupportAddressRequired, at)),
        (false, true) => Err(RecipeError::at(RecipeIssue::SupportAddressUnneeded, at)),
        (true, true) | (false, false) => Ok(()),
    }
}

pub(super) fn evidence_standing(
    requested: &[RequestedEvidence],
    role: RecipeRole,
    harness: HarnessPosture,
) -> ProjectionStanding {
    if harness == HarnessPosture::Unavailable {
        return ProjectionStanding::FeatureUnavailable;
    }
    requested
        .iter()
        .find(|row| row.role == role)
        .map_or(ProjectionStanding::NotRequested, |row| {
            if row.body.is_some() {
                ProjectionStanding::Generated(EffectiveProjection::effective(
                    role,
                    None,
                    LoweringSource::Configuration,
                ))
            } else {
                ProjectionStanding::TargetUnavailable
            }
        })
}

pub(super) fn evidence(
    requested: &[RequestedEvidence],
) -> [Option<RecipeEvidence>; EVIDENCE_LIMIT] {
    core::array::from_fn(|position| {
        let role = evidence_role(position)?;
        let row = requested.iter().find(|candidate| candidate.role == role)?;
        let body = row.body.clone()?;
        Some(RecipeEvidence::captured(row.role, row.target, body, row.at))
    })
}

const fn evidence_role(position: usize) -> Option<RecipeRole> {
    match position {
        0 => Some(RecipeRole::Trials),
        1 => Some(RecipeRole::Mutation),
        2 => Some(RecipeRole::Benchmarks),
        3 => Some(RecipeRole::Network),
        4 => Some(RecipeRole::Concurrency),
        _ => None,
    }
}

fn generated(projections: &[ProjectionStanding; 10], role: RecipeRole) -> bool {
    let [
        companions,
        dispatch,
        compile_contract,
        property,
        typestate,
        trials,
        mutation,
        benchmarks,
        network,
        concurrency,
    ] = projections;
    matches!(
        match role {
            RecipeRole::Companions => companions,
            RecipeRole::Dispatch => dispatch,
            RecipeRole::CompileContract => compile_contract,
            RecipeRole::Property => property,
            RecipeRole::Typestate => typestate,
            RecipeRole::Trials => trials,
            RecipeRole::Mutation => mutation,
            RecipeRole::Benchmarks => benchmarks,
            RecipeRole::Network => network,
            RecipeRole::Concurrency => concurrency,
        },
        ProjectionStanding::Generated(_)
    )
}
