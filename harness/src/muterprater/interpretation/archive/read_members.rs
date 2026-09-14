//! Bounded historical parity members without callback reconstruction.

use super::super::{
    ArchivedParityDisposition, ArchivedSubstrate, ArchivedSubstrateRoster, ArchivedValueConvention,
    ParityArchiveLimits, ParityArchiveRefusal, ValueRole,
};
use crate::descriptor::archive::{ArchivedName, read_revision};
use crate::identity::BodyReader;
use crate::muterprater::ParityQualificationRefusal;
use crate::report::archive::{ArchiveRefusal, ArchivedConclusion, claim, finding, frame, name};

pub(super) fn convention(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ParityArchiveLimits,
) -> Result<ArchivedValueConvention, ParityArchiveRefusal> {
    let bytes = limits.bytes();
    Ok(ArchivedValueConvention {
        name: name(reader, bytes)?,
        version: reader.u32()?,
        schema: claim(reader, bytes)?,
        revision: read_revision(frame(reader, bytes)?, bytes.field())?,
    })
}

pub(super) fn value(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    role: ValueRole,
    limits: ParityArchiveLimits,
) -> Result<Vec<u8>, ParityArchiveRefusal> {
    let bytes = frame(reader, limits.bytes())?;
    crate::muterprater::interpretation::archive::size::value_size(bytes.len(), role, limits)?;
    Ok(bytes.to_vec())
}

pub(super) fn substrate(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ParityArchiveLimits,
) -> Result<ArchivedSubstrate, ParityArchiveRefusal> {
    match reader.byte()? {
        0 => return Ok(ArchivedSubstrate::DeclaredIndependent),
        1 => {}
        _ => return Err(ArchiveRefusal::InvalidSlot.into()),
    }
    let count = reader.count()?;
    if count > limits.substrates() {
        return Err(ParityArchiveRefusal::TooManySubstrates);
    }
    if count == 0 {
        return Err(ParityArchiveRefusal::InvalidSubstrate);
    }
    let mut names: Vec<ArchivedName> = Vec::new();
    for _ in 0..count {
        let current = name(reader, limits.bytes())?;
        if let Some(previous) = names.last()
            && (previous.namespace(), previous.stem()) >= (current.namespace(), current.stem())
        {
            return Err(ParityArchiveRefusal::InvalidSubstrate);
        }
        names.push(current);
    }
    Ok(ArchivedSubstrate::Standing(ArchivedSubstrateRoster {
        names,
    }))
}

pub(super) fn conclusion(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ParityArchiveLimits,
) -> Result<ArchivedConclusion, ParityArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedConclusion::Passed),
        1 => Ok(ArchivedConclusion::Refused(Box::new(finding(
            reader,
            limits.bytes(),
        )?))),
        _ => Err(ArchiveRefusal::InvalidSlot.into()),
    }
}

pub(super) fn disposition(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
) -> Result<ArchivedParityDisposition, ParityArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedParityDisposition::Raw),
        1 => Ok(ArchivedParityDisposition::Qualified),
        2 => {
            let cause = match reader.byte()? {
                0 => ParityQualificationRefusal::ProductionDidNotQualify,
                1 => ParityQualificationRefusal::EvaluationDidNotQualify,
                2 => ParityQualificationRefusal::NoMutationActivated {
                    firings: reader.u32()?,
                },
                3 => ParityQualificationRefusal::MeaningsDisagreed,
                _ => return Err(ArchiveRefusal::InvalidSlot.into()),
            };
            Ok(ArchivedParityDisposition::Rejected(cause))
        }
        _ => Err(ArchiveRefusal::InvalidSlot.into()),
    }
}
