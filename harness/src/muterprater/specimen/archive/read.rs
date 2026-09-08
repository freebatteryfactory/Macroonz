//! Bounded historical projection admission without current pressure constructors.

use super::super::{
    ArchivedProjectionPressure, ArchivedSpecimenStanding, PROJECTION_ARCHIVE_TAG,
    ProjectionArchiveLimits, ProjectionArchiveRefusal,
};
use super::joins;
use crate::identity::BodyReader;
use crate::muterprater::ArtifactContent;
use crate::muterprater::discovery_archive::read_selection;
use crate::muterprater::interpretation_archive::{ArchivedParityDisposition, read_parity};
use crate::muterprater::specimen::archive::size::source_size;
use crate::muterprater::verdict_archive::read_mutation;
use crate::report::archive::{ArchiveRefusal, claim, envelope, finish, frame, read_trial};

/// Read complete historical compiled pressure without invoking generic callbacks.
///
/// # Errors
///
/// Refuses oversized or malformed members, moved content identities and contradictory retained joins.
pub fn read_projection(
    encoded: &[u8],
    limits: ProjectionArchiveLimits,
) -> Result<ArchivedProjectionPressure, ProjectionArchiveRefusal> {
    let bytes = limits.bytes();
    let (address, mut reader) = envelope(encoded, PROJECTION_ARCHIVE_TAG, 1, bytes)?;
    let parity = read_parity(frame(&mut reader, bytes)?, limits.parity())?;
    if parity.disposition() != ArchivedParityDisposition::Qualified {
        return Err(ProjectionArchiveRefusal::ParityNotQualified);
    }
    let baseline = artifact(&mut reader, limits)?;
    let selected = artifact(&mut reader, limits)?;
    if baseline.identity() == selected.identity() {
        return Err(ProjectionArchiveRefusal::ArtifactDidNotChange);
    }
    let selection = read_selection(&mut reader, bytes)?;
    if selection.surface() != parity.pair().surface() {
        return Err(ProjectionArchiveRefusal::StandingMismatch);
    }
    let baseline_report = read_trial(frame(&mut reader, bytes)?, limits.trial())?;
    let selected_report = read_trial(frame(&mut reader, bytes)?, limits.trial())?;
    let mutation = read_mutation(frame(&mut reader, bytes)?, limits.mutation())?;
    finish(&reader)?;
    joins::reports(&parity, &baseline_report, &selected_report)?;
    joins::mutation(&parity, &selection, &selected_report, &mutation)?;
    let standing = ArchivedSpecimenStanding {
        artifact: selected.identity(),
        pair: parity.pair().clone(),
        selection,
        execution: parity.production_report().key().clone(),
        check: parity.witness().check().clone(),
    };
    Ok(ArchivedProjectionPressure {
        encoded: encoded.to_vec(),
        address,
        parity,
        baseline,
        selected,
        standing,
        baseline_report,
        selected_report,
        mutation,
    })
}

fn artifact(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ProjectionArchiveLimits,
) -> Result<ArtifactContent, ProjectionArchiveRefusal> {
    let identity = claim(reader, limits.bytes())?;
    let bytes = frame(reader, limits.bytes())?;
    source_size(bytes.len(), limits)?;
    let content = ArtifactContent::recorded(bytes.to_vec());
    if content.identity().address().as_bytes() != identity.as_bytes() {
        return Err(ProjectionArchiveRefusal::ArtifactIdentityMismatch);
    }
    Ok(content)
}
