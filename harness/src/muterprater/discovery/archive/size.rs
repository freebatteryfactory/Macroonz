//! Complete surface bounds precede allocation of its canonical preimage.

use super::{SurfaceArchiveLimits, SurfaceArchiveRefusal};
use crate::muterprater::EvaluationSurface;
use crate::report::archive::{ArchiveRefusal, bounded, name_size, sum};

pub(crate) fn encoded_size(
    surface: &EvaluationSurface,
    limits: SurfaceArchiveLimits,
) -> Result<usize, SurfaceArchiveRefusal> {
    let bytes = limits.bytes();
    bounded(32, bytes)?;
    if surface.points().len() > limits.points() {
        return Err(SurfaceArchiveRefusal::TooManyPoints);
    }
    u64::try_from(surface.points().len()).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
    let mut canonical = sum(&[48, name_size(surface.family().name(), bytes)?])?;
    for point in surface.points() {
        if point.admitted_alternatives().len() > limits.alternatives() {
            return Err(SurfaceArchiveRefusal::TooManyAlternatives);
        }
        u64::try_from(point.admitted_alternatives().len())
            .map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
        canonical = sum(&[
            canonical,
            16,
            name_size(point.identity().name(), bytes)?,
            name_size(point.owner_claim().name(), bytes)?,
            bounded(point.original_operation().len(), bytes)?,
            name_size(point.activation_site().name(), bytes)?,
        ])?;
        for alternative in point.admitted_alternatives() {
            canonical = sum(&[
                canonical,
                56,
                bounded(alternative.family().slug().len(), bytes)?,
                bounded(alternative.operation().len(), bytes)?,
            ])?;
        }
    }
    let total = sum(&[92, bounded(canonical, bytes)?])?;
    if total > bytes.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge.into());
    }
    Ok(total)
}
