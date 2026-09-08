//! Complete canonical-surface admission without live membership or selection.

use super::super::{
    ArchivedEvaluationSurface, ArchivedMutationPoint, SURFACE_ARCHIVE_TAG, SurfaceArchiveLimits,
    SurfaceArchiveRefusal,
};
use super::members;
use crate::identity::ContentAddress;
use crate::muterprater::EVALUATION_SURFACE_TAG;
use crate::report::archive::{claim, cursor, envelope, finish, frame, name};

/// Read a bounded owned historical surface without reconstructing execution authority.
///
/// # Errors
///
/// Refuses malformed canonical material, identities, order, operations or resource ceilings.
pub fn read_surface(
    encoded: &[u8],
    limits: SurfaceArchiveLimits,
) -> Result<ArchivedEvaluationSurface, SurfaceArchiveRefusal> {
    let bytes = limits.bytes();
    let (address, mut envelope_reader) = envelope(encoded, SURFACE_ARCHIVE_TAG, 1, bytes)?;
    let expected = claim(&mut envelope_reader, bytes)?;
    let canonical = frame(&mut envelope_reader, bytes)?;
    finish(&envelope_reader)?;
    let identity = ContentAddress::derived(EVALUATION_SURFACE_TAG, canonical);
    if identity.as_bytes() != expected.as_bytes() {
        return Err(SurfaceArchiveRefusal::SurfaceIdentityMismatch);
    }
    let mut reader = cursor(canonical);
    let family = name(&mut reader, bytes)?;
    let policy = claim(&mut reader, bytes)?;
    let count = reader.count()?;
    if count > limits.points() {
        return Err(SurfaceArchiveRefusal::TooManyPoints);
    }
    let mut points: Vec<ArchivedMutationPoint> = Vec::new();
    for _ in 0..count {
        let point = members::point(&mut reader, limits)?;
        if let Some(previous) = points.last()
            && (previous.name().namespace(), previous.name().stem())
                >= (point.name().namespace(), point.name().stem())
        {
            return Err(SurfaceArchiveRefusal::NonCanonicalPoints);
        }
        points.push(point);
    }
    finish(&reader)?;
    Ok(ArchivedEvaluationSurface {
        encoded: encoded.to_vec(),
        address,
        identity,
        canonical: canonical.to_vec(),
        family,
        policy,
        points,
    })
}
