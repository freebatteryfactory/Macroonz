//! Bounded point and alternative readings reuse the discovery identity owner.

use super::super::{
    ArchivedAlternative, ArchivedMutationPoint, SurfaceArchiveLimits, SurfaceArchiveRefusal,
};
use crate::descriptor::archive::ArchivedName;
use crate::identity::{BodyReader, ContentAddress};
use crate::muterprater::MUTATION_ALTERNATIVE_TAG;
use crate::muterprater::discovery::encode::alternative_claim_preimage;
use crate::report::archive::{ArchiveRefusal, claim, frame, name, text};

pub(super) fn point(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: SurfaceArchiveLimits,
) -> Result<ArchivedMutationPoint, SurfaceArchiveRefusal> {
    let bytes = limits.bytes();
    let name = name(reader, bytes)?;
    let owner_claim = crate::report::archive::name(reader, bytes)?;
    let original = frame(reader, bytes)?;
    if original.is_empty() {
        return Err(SurfaceArchiveRefusal::EmptyOperation);
    }
    let activation_site = crate::report::archive::name(reader, bytes)?;
    let count = reader.count()?;
    if count > limits.alternatives() {
        return Err(SurfaceArchiveRefusal::TooManyAlternatives);
    }
    if count == 0 {
        return Err(SurfaceArchiveRefusal::NonCanonicalAlternatives);
    }
    let mut alternatives: Vec<ArchivedAlternative> = Vec::new();
    for _ in 0..count {
        let alternative = alternative(reader, &name, original, limits)?;
        if let Some(previous) = alternatives.last()
            && previous.identity() >= alternative.identity()
        {
            return Err(SurfaceArchiveRefusal::NonCanonicalAlternatives);
        }
        alternatives.push(alternative);
    }
    Ok(ArchivedMutationPoint {
        name,
        owner_claim,
        original: original.to_vec(),
        activation_site,
        alternatives,
    })
}

fn alternative(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    point: &ArchivedName,
    original: &[u8],
    limits: SurfaceArchiveLimits,
) -> Result<ArchivedAlternative, SurfaceArchiveRefusal> {
    let bytes = limits.bytes();
    let expected = claim(reader, bytes)?;
    let family = text(reader, bytes)?;
    if family.is_empty() {
        return Err(ArchiveRefusal::InvalidText.into());
    }
    let operation = frame(reader, bytes)?;
    if operation.is_empty() {
        return Err(SurfaceArchiveRefusal::EmptyOperation);
    }
    if operation == original {
        return Err(SurfaceArchiveRefusal::AlternativeIsOriginal);
    }
    let preimage = alternative_claim_preimage(point.namespace(), point.stem(), family, operation);
    let identity = ContentAddress::derived(MUTATION_ALTERNATIVE_TAG, &preimage);
    if identity.as_bytes() != expected.as_bytes() {
        return Err(SurfaceArchiveRefusal::AlternativeIdentityMismatch);
    }
    Ok(ArchivedAlternative {
        identity,
        family: family.to_owned(),
        operation: operation.to_vec(),
    })
}
