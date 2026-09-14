//! Bounded historical reading without invoking a producer or an oracle judgment.

use super::super::{
    ArchivedMethod, ArchivedOracle, ArchivedTranscriptDisagreement, ArchivedVectorDisagreement,
    ArchivedVerdict, ORACLE_ARCHIVE_TAG, OracleArchiveRefusal,
};
use crate::identity::BodyReader;
use crate::oracle::vector::first_difference;
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, claim, envelope, finish, frame};

/// Read one complete historical verdict under the independently expected method and byte limits.
///
/// # Errors
///
/// Refuses unsupported framing, excessive bytes, unknown dispositions and fields that violate their owning construction boundary.
pub fn read_verdict(
    encoded: &[u8],
    method: ArchivedMethod,
    limits: ArchiveLimits,
) -> Result<ArchivedOracle, OracleArchiveRefusal> {
    let (address, mut reader) = envelope(encoded, ORACLE_ARCHIVE_TAG, method.slot(), limits)?;
    let verdict = match method {
        ArchivedMethod::Vector => vector(&mut reader, limits)?,
        ArchivedMethod::Transcript => transcript(&mut reader, limits)?,
        ArchivedMethod::Structural => super::structural::read(&mut reader, limits)?,
        ArchivedMethod::Compiled => super::compiled::read(&mut reader, limits)?,
        ArchivedMethod::Compilation => super::compiled::compilation(&mut reader, limits)?,
    };
    finish(&reader)?;
    Ok(ArchivedOracle {
        encoded: encoded.to_vec(),
        address,
        verdict,
    })
}

fn vector(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedVerdict, OracleArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedVerdict::VectorAgrees),
        1 => {
            let expected = frame(reader, limits)?;
            let produced = frame(reader, limits)?;
            let difference = first_difference(expected, produced)
                .ok_or(OracleArchiveRefusal::EqualVectorBuffers)?;
            Ok(ArchivedVerdict::VectorDisagrees(
                ArchivedVectorDisagreement {
                    expected: expected.to_vec(),
                    produced: produced.to_vec(),
                    difference,
                },
            ))
        }
        _ => Err(ArchiveRefusal::InvalidSlot.into()),
    }
}

fn transcript(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedVerdict, OracleArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedVerdict::TranscriptAgrees),
        1 => {
            let rederived = claim(reader, limits)?;
            let published = claim(reader, limits)?;
            if rederived == published {
                return Err(OracleArchiveRefusal::EqualTranscriptClaims);
            }
            Ok(ArchivedVerdict::TranscriptDisagrees(
                ArchivedTranscriptDisagreement {
                    rederived,
                    published,
                },
            ))
        }
        _ => Err(ArchiveRefusal::InvalidSlot.into()),
    }
}
