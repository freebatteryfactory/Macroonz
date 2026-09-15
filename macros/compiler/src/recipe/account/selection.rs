//! Admission and effective selection of captured recipe projection requests.

use super::{
    CapturedRelation, EVIDENCE_LIMIT, EffectiveProjection, EvidenceTarget, HarnessPosture,
    LoweringSource, PROJECTION_LIMIT, ProjectionStanding, Recipe, RecipeError, RecipeEvidence,
    RecipeIssue, RecipeRole, RecipeRoleAvailability, RecipeRoleEntrance, RecipeRolePlacement,
    RelationTableProjection, RequestedEvidence, RequestedProjection, RequestedRelationTable,
};
use crate::support::SupportName;

impl Recipe {
    /// Admit requests before interpreting their deferred grammar in canonical role order.
    pub(in crate::recipe) fn projection_standings(
        requested: &[RequestedProjection],
        evidence: &[RequestedEvidence],
        harness: HarnessPosture,
        read: impl Fn(&RequestedProjection) -> Result<EffectiveProjection, RecipeError>,
    ) -> Result<[ProjectionStanding; PROJECTION_LIMIT], RecipeError> {
        ensure_requested_admission(requested, harness)?;
        ensure_evidence_admission(evidence, harness)?;
        let mut standings = Vec::with_capacity(PROJECTION_LIMIT);
        for role in RecipeRole::ALL.iter().copied() {
            let standing = match role.profile().entrance {
                RecipeRoleEntrance::Projection => standing(requested, role, harness, &read)?,
                RecipeRoleEntrance::Evidence => evidence_standing(evidence, role, harness),
            };
            standings.push(standing);
        }
        let Ok(standings) = standings.try_into() else {
            unreachable!("the complete role roster has the projection-account magnitude")
        };
        Ok(standings)
    }

    pub(in crate::recipe) fn support_matches_projections(
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

    pub(in crate::recipe) fn captured_evidence(
        requested: &[RequestedEvidence],
    ) -> [Option<RecipeEvidence>; EVIDENCE_LIMIT] {
        core::array::from_fn(|position| {
            let role = RecipeRole::evidence_roles()
                .find(|role| role.profile().evidence_position == Some(position))?;
            let row = requested.iter().find(|candidate| candidate.role == role)?;
            let body = row.body.clone()?;
            Some(RecipeEvidence::captured(
                row.role,
                row.target.clone().map(EvidenceTarget::named),
                body,
                row.at,
            ))
        })
    }
}

impl RelationTableProjection {
    /// Admit this table's relation before its exact signature is interpreted.
    pub(in crate::recipe) fn declared_subject<'a, 'b>(
        requested: &RequestedRelationTable,
        mut earlier: impl Iterator<Item = &'b RequestedRelationTable>,
        relations: &'a [CapturedRelation],
    ) -> Result<(&'a str, &'a str), RecipeError> {
        if earlier.any(|row| row.relation == requested.relation) {
            return Err(RecipeError::at(
                RecipeIssue::DuplicateRelationTable {
                    relation: requested.relation.clone(),
                },
                Some(requested.at),
            ));
        }
        let relation = relations
            .iter()
            .find(|relation| relation.name.spelling == requested.relation)
            .ok_or_else(|| {
                RecipeError::at(
                    RecipeIssue::RelationNotFound {
                        name: requested.relation.clone(),
                    },
                    Some(requested.at),
                )
            })?;
        Ok((
            relation.left.spelling.as_str(),
            relation.right.spelling.as_str(),
        ))
    }
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
            && row.role.profile().availability == RecipeRoleAvailability::Harness
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
        if harness == HarnessPosture::Unavailable
            && row.role.profile().availability == RecipeRoleAvailability::Harness
            && row.body.is_some()
        {
            return Err(RecipeError::at(
                RecipeIssue::HarnessUnavailable { role: row.role },
                Some(row.at),
            ));
        }
    }
    Ok(())
}

fn standing(
    requested: &[RequestedProjection],
    role: RecipeRole,
    harness: HarnessPosture,
    read: &impl Fn(&RequestedProjection) -> Result<EffectiveProjection, RecipeError>,
) -> Result<ProjectionStanding, RecipeError> {
    if harness == HarnessPosture::Unavailable
        && role.profile().availability == RecipeRoleAvailability::Harness
        && !requested.iter().any(|row| row.role == role)
    {
        return Ok(ProjectionStanding::FeatureUnavailable);
    }
    let Some(row) = requested.iter().find(|row| row.role == role) else {
        return Ok(ProjectionStanding::NotRequested);
    };
    Ok(ProjectionStanding::Generated(Box::new(read(row)?)))
}

fn evidence_standing(
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
                ProjectionStanding::Generated(Box::new(EffectiveProjection::effective(
                    role,
                    None,
                    None,
                    LoweringSource::Configuration,
                    row.at,
                )))
            } else {
                ProjectionStanding::TargetUnavailable
            }
        })
}

fn generated(projections: &[ProjectionStanding; PROJECTION_LIMIT], role: RecipeRole) -> bool {
    matches!(role.standing(projections), ProjectionStanding::Generated(_))
}
