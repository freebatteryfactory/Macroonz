//! Bounded historical target and activation admission.

use super::super::{
    ArchivedActivation, ArchivedActivationReading, ArchivedMutationIdentity, ArchivedMutationSite,
    ArchivedMutationTarget,
};
use crate::identity::BodyReader;
use crate::muterprater::SourceCoordinate;
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, claim, cursor, finish, name, text};

pub(crate) fn read_target(
    bytes: &[u8],
    limits: ArchiveLimits,
) -> Result<ArchivedMutationTarget, ArchiveRefusal> {
    let mut reader = cursor(bytes);
    let identity = identity(&mut reader, limits)?;
    let family = match reader.byte()? {
        0 => None,
        1 => Some(nonempty(&mut reader, limits)?.to_owned()),
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    let site = match reader.byte()? {
        0 => ArchivedMutationSite::Reported(
            SourceCoordinate::reported(
                nonempty(&mut reader, limits)?,
                reader.u32()?,
                reader.u32()?,
            )
            .map_err(|_| ArchiveRefusal::InvalidText)?,
        ),
        1 => ArchivedMutationSite::Declared(name(&mut reader, limits)?),
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    let owner = match reader.byte()? {
        0 => None,
        1 => Some(name(&mut reader, limits)?),
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    finish(&reader)?;
    Ok(ArchivedMutationTarget {
        identity,
        family,
        site,
        owner,
    })
}

fn identity(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedMutationIdentity, ArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedMutationIdentity::External(claim(reader, limits)?)),
        1 => Ok(ArchivedMutationIdentity::Interpreted {
            point: name(reader, limits)?,
            alternative: claim(reader, limits)?,
        }),
        2 => Ok(ArchivedMutationIdentity::CompiledProjection {
            point: name(reader, limits)?,
            alternative: claim(reader, limits)?,
        }),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

fn nonempty<'body>(
    reader: &mut BodyReader<'body, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<&'body str, ArchiveRefusal> {
    let value = text(reader, limits)?;
    if value.is_empty() {
        return Err(ArchiveRefusal::InvalidText);
    }
    Ok(value)
}

pub(crate) fn read_activation(
    bytes: &[u8],
    limits: ArchiveLimits,
) -> Result<ArchivedActivation, ArchiveRefusal> {
    let mut reader = cursor(bytes);
    let activation = match reader.byte()? {
        0 => {
            let surface = claim(&mut reader, limits)?;
            let point = name(&mut reader, limits)?;
            let alternative = claim(&mut reader, limits)?;
            let witness = claim(&mut reader, limits)?;
            let firings = reader.u32()?;
            if firings == 0 {
                return Err(ArchiveRefusal::InvalidSlot);
            }
            ArchivedActivation::Observed(ArchivedActivationReading {
                surface,
                point,
                alternative,
                witness,
                firings,
            })
        }
        1 => ArchivedActivation::NotObserved,
        2 => ArchivedActivation::UnobservableUnderBackend,
        _ => return Err(ArchiveRefusal::InvalidSlot),
    };
    finish(&reader)?;
    Ok(activation)
}
