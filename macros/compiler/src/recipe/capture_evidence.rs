//! Descriptor-native evidence clauses, support address posture, and evidence standings.

use super::super::types::{RecipeRoleEntrance, RecipeRolePlacement};
use super::{
    EVIDENCE_LIMIT, EffectiveProjection, HarnessPosture, LoweringSource, PROJECTION_LIMIT,
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
    let Some(role) = RecipeRole::from_syntax(spelling, RecipeRoleEntrance::Evidence) else {
        return Err(CaptureReadRefusal::projected(
            crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Word(
                "a descriptor-native evidence projection".to_owned(),
            )),
            Some(token.span()),
        ));
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
        let (_target_token, target) = selected.identifier()?;
        selected.finish()?;
        Some(target.to_owned())
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
    projections: &[ProjectionStanding; PROJECTION_LIMIT],
    support: Option<&SupportName>,
    at: Option<crate::token::SpanHandle>,
) -> Result<(), RecipeError> {
    let evidence = RecipeRole::ALL.iter().copied().any(|role| {
        role.profile().output.placement == RecipeRolePlacement::SupportCarrier
            && generated(projections, role)
    });
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
                    None,
                    LoweringSource::Configuration,
                    row.at,
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
        let role = RecipeRole::evidence_roles()
            .find(|role| role.profile().evidence_position == Some(position))?;
        let row = requested.iter().find(|candidate| candidate.role == role)?;
        let body = row.body.clone()?;
        Some(RecipeEvidence::captured(
            row.role,
            row.target.clone().map(super::EvidenceTarget::named),
            body,
            row.at,
        ))
    })
}

fn generated(projections: &[ProjectionStanding; PROJECTION_LIMIT], role: RecipeRole) -> bool {
    matches!(role.standing(projections), ProjectionStanding::Generated(_))
}
