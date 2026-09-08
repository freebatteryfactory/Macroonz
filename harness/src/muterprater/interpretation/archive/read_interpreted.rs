//! Historical interpreted admission composes complete nested records.

use super::super::{
    ArchivedInterpretedEvidence, ArchivedInterpretedTrust, ArchivedValue, INTERPRETED_ARCHIVE_TAG,
    InterpretedArchiveLimits, InterpretedArchiveRefusal,
};
use super::interpreted_joins;
use crate::muterprater::backend_archive::read_suite_pressure;
use crate::muterprater::discovery_archive::read_surface;
use crate::muterprater::interpretation::archive::size_interpreted::active_size;
use crate::muterprater::specimen_archive::read_projection;
use crate::muterprater::verdict_archive::{interpreted_trial_join, read_mutation};
use crate::report::archive::{envelope, finish, frame, read_trial};

/// Read complete owned historical interpreted evidence without running any caller callback.
///
/// # Errors
///
/// Refuses malformed or oversized members and contradictory surface, selection, report or activation joins.
pub fn read_interpreted(
    encoded: &[u8],
    limits: &InterpretedArchiveLimits,
) -> Result<ArchivedInterpretedEvidence, InterpretedArchiveRefusal> {
    let bytes = limits.bytes();
    let (address, mut reader) = envelope(encoded, INTERPRETED_ARCHIVE_TAG, 1, bytes)?;
    let surface = read_surface(frame(&mut reader, bytes)?, limits.surface())?;
    let suite = read_suite_pressure(frame(&mut reader, bytes)?, limits.suite())?;
    let projection = read_projection(frame(&mut reader, bytes)?, limits.projection())?;
    interpreted_joins::trust(&surface, &projection)?;
    let active = frame(&mut reader, bytes)?;
    active_size(active.len(), limits)?;
    let meaning = ArchivedValue {
        convention: projection.parity().production().convention().clone(),
        bytes: active.to_vec(),
    };
    let report = read_trial(frame(&mut reader, bytes)?, limits.trial())?;
    let mutation = read_mutation(frame(&mut reader, bytes)?, limits.mutation())?;
    finish(&reader)?;
    interpreted_joins::active(&surface, &projection, &report, &mutation)?;
    interpreted_trial_join(&report, &mutation)?;
    Ok(ArchivedInterpretedEvidence {
        encoded: encoded.to_vec(),
        address,
        trust: ArchivedInterpretedTrust {
            surface,
            suite,
            projection,
        },
        meaning,
        report,
        mutation,
    })
}
