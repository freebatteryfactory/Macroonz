//! Surface retention wraps the existing canonical writer without changing its bytes.

use super::{
    ArchivedEvaluationSurface, SURFACE_ARCHIVE_TAG, SurfaceArchiveLimits, SurfaceArchiveRefusal,
    read_surface,
};
use crate::identity::{ContentAddress, encode_bytes};
use crate::muterprater::EvaluationSurface;

/// Retain a surface's complete canonical material as historical owned data.
///
/// # Errors
///
/// Refuses byte and population limits before allocating the existing canonical preimage.
pub fn retain_surface(
    surface: &EvaluationSurface,
    limits: SurfaceArchiveLimits,
) -> Result<ArchivedEvaluationSurface, SurfaceArchiveRefusal> {
    let total = super::size::encoded_size(surface, limits)?;
    let canonical = super::super::encode::surface_preimage(
        surface.family(),
        surface.policy(),
        surface.points(),
    );
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());
    encode_bytes(surface.identity().address().as_bytes(), &mut body);
    encode_bytes(&canonical, &mut body);
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(SURFACE_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_surface(&encoded, limits)
}
