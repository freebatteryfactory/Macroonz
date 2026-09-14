//! Caller-stated structural fields retain portable positions without invented parser provenance.

use super::super::{ArchivedStructuralDisagreement, ArchivedVerdict, OracleArchiveRefusal};
use crate::identity::BodyReader;
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, text};

pub(super) fn read(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedVerdict, OracleArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedVerdict::StructuralConforms),
        1 => Ok(ArchivedVerdict::StructuralDeviates(disagreement(
            reader, limits,
        )?)),
        2 => Ok(ArchivedVerdict::StructuralUnparsable),
        _ => Err(ArchiveRefusal::InvalidSlot.into()),
    }
}

fn disagreement(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedStructuralDisagreement, OracleArchiveRefusal> {
    let slot = reader.byte()?;
    match slot {
        0 => Ok(ArchivedStructuralDisagreement::UnexpectedItem),
        1 => Ok(ArchivedStructuralDisagreement::OutputCardinality {
            declared: reader.u64()?,
            read: reader.u64()?,
        }),
        2 => Ok(ArchivedStructuralDisagreement::DuplicateImplementation { at: reader.u64()? }),
        3 => Ok(ArchivedStructuralDisagreement::ImplementationTarget { at: reader.u64()? }),
        4 => Ok(ArchivedStructuralDisagreement::TraitPath { at: reader.u64()? }),
        5 => Ok(ArchivedStructuralDisagreement::ImplPosture { at: reader.u64()? }),
        6 => Ok(ArchivedStructuralDisagreement::MeaningBearingAttribute {
            at: reader.u64()?,
            attribute: text(reader, limits)?.to_owned(),
        }),
        7 => Ok(ArchivedStructuralDisagreement::UnexpectedImplMember {
            at: reader.u64()?,
            member: text(reader, limits)?.to_owned(),
        }),
        8 => Ok(ArchivedStructuralDisagreement::DuplicateMember {
            at: reader.u64()?,
            member: text(reader, limits)?.to_owned(),
        }),
        9 => Ok(ArchivedStructuralDisagreement::MissingImplMember {
            at: reader.u64()?,
            member: text(reader, limits)?.to_owned(),
        }),
        10 => Ok(ArchivedStructuralDisagreement::MemberValueUnread {
            at: reader.u64()?,
            member: text(reader, limits)?.to_owned(),
        }),
        11 => Ok(ArchivedStructuralDisagreement::MemberValue {
            at: reader.u64()?,
            member: text(reader, limits)?.to_owned(),
        }),
        _ => Err(ArchiveRefusal::InvalidSlot.into()),
    }
}
