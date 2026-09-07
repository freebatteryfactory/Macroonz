//! Complete historical origins over the candidate reader's shared canonical prefix.

use super::super::{
    ArchivedOrigin, ArchivedRow, CandidateArchiveRefusal, RowArchiveLimits, RowArchiveRefusal,
};
use super::read;
use crate::descriptor::{ReplayBearingGround, Row};
use crate::identity::BodyReader;

/// Retain every field and origin of an existing row as historical data.
///
/// # Errors
///
/// Refuses canonical material exceeding the independently supplied bounds.
pub fn retain_row(row: &Row, limits: RowArchiveLimits) -> Result<ArchivedRow, RowArchiveRefusal> {
    read_row(row.canonical_bytes().as_bytes(), limits)
}

/// Read a complete canonical row without constructing a live descriptor or admission.
///
/// # Errors
///
/// Refuses excess bounds, unsupported grammar, malformed origin members and undeclared trailing bytes.
pub fn read_row(
    canonical: &[u8],
    limits: RowArchiveLimits,
) -> Result<ArchivedRow, RowArchiveRefusal> {
    let (mut reader, fields) =
        read::descriptor(canonical, limits).map_err(RowArchiveRefusal::Canonical)?;
    let origin = origin(&mut reader, limits)?;
    read::finish(&reader).map_err(RowArchiveRefusal::Canonical)?;
    Ok(ArchivedRow {
        canonical: canonical.to_vec(),
        fields,
        origin,
    })
}

fn origin(
    reader: &mut BodyReader<'_, CandidateArchiveRefusal>,
    limits: RowArchiveLimits,
) -> Result<ArchivedOrigin, RowArchiveRefusal> {
    match reader.byte().map_err(RowArchiveRefusal::Canonical)? {
        1 => Ok(ArchivedOrigin::HandWritten),
        2 => Ok(ArchivedOrigin::Generated {
            door: read::name(reader, limits).map_err(RowArchiveRefusal::Canonical)?,
            projection: read::name(reader, limits).map_err(RowArchiveRefusal::Canonical)?,
        }),
        3 => read::synthesis(reader, limits)
            .map(ArchivedOrigin::Candidate)
            .map_err(RowArchiveRefusal::Canonical),
        4 => {
            let proposal = address(reader, limits)?;
            let ground = match reader.byte().map_err(RowArchiveRefusal::Canonical)? {
                1 => ReplayBearingGround::MutantKilled,
                2 => ReplayBearingGround::ClaimPinned,
                found => return Err(RowArchiveRefusal::InvalidReplayGround { found }),
            };
            let destination = read::name(reader, limits).map_err(RowArchiveRefusal::Canonical)?;
            let replay = address(reader, limits)?;
            Ok(ArchivedOrigin::AdmittedReplay {
                proposal,
                ground,
                destination,
                replay,
            })
        }
        5 => Ok(ArchivedOrigin::AdmittedDischarge {
            proposal: address(reader, limits)?,
            destination: read::name(reader, limits).map_err(RowArchiveRefusal::Canonical)?,
        }),
        found => Err(RowArchiveRefusal::InvalidOrigin { found }),
    }
}

fn address(
    reader: &mut BodyReader<'_, CandidateArchiveRefusal>,
    limits: RowArchiveLimits,
) -> Result<[u8; 32], RowArchiveRefusal> {
    let bytes = reader.bytes().map_err(RowArchiveRefusal::Canonical)?;
    if bytes.len() > limits.field() {
        return Err(RowArchiveRefusal::Canonical(
            CandidateArchiveRefusal::FieldTooLarge,
        ));
    }
    bytes
        .try_into()
        .map_err(|_| RowArchiveRefusal::InvalidAddressWidth)
}
