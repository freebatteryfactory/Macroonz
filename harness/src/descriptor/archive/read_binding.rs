//! Historical attachment reading through the shared descriptor name and row owners.

use super::super::{
    ArchivedBinding, ArchivedOrigin, ArchivedProvenance, ArchivedRevisionBinding,
    BindingArchiveLimits, BindingArchiveRefusal, CandidateArchiveRefusal,
};
use super::read::{finish, name};
use super::row::read_row;
use crate::descriptor::RevisionPosture;
use crate::identity::BodyReader;

fn cursor(bytes: &[u8]) -> BodyReader<'_, CandidateArchiveRefusal> {
    BodyReader::over(bytes, CandidateArchiveRefusal::Truncated, |declared| {
        CandidateArchiveRefusal::LengthOutsidePlatform { declared }
    })
}

/// Read a complete historical binding without constructing a callable or live revision.
///
/// # Errors
///
/// Refuses bounded grammar failures, then subject, check and generated-provenance disagreement.
pub fn read_binding(
    encoded: &[u8],
    limits: BindingArchiveLimits,
) -> Result<ArchivedBinding, BindingArchiveRefusal> {
    use BindingArchiveRefusal::Canonical;
    if encoded.len() > limits.bytes() {
        return Err(Canonical(CandidateArchiveRefusal::BytesTooLarge));
    }
    let mut reader = cursor(encoded);
    let format = reader.u32().map_err(Canonical)?;
    if format != 1 {
        return Err(Canonical(CandidateArchiveRefusal::UnsupportedFormat {
            found: format,
        }));
    }
    let row =
        read_row(reader.bytes().map_err(Canonical)?, limits).map_err(BindingArchiveRefusal::Row)?;
    let subject = name(&mut reader, limits).map_err(Canonical)?;
    let check = name(&mut reader, limits).map_err(Canonical)?;
    let subject_revision = read_revision(reader.bytes().map_err(Canonical)?, limits.field())?;
    let check_revision = read_revision(reader.bytes().map_err(Canonical)?, limits.field())?;
    let provenance = match reader.byte().map_err(Canonical)? {
        0 => ArchivedProvenance::Unproduced,
        1 => ArchivedProvenance::Produced {
            producer: name(&mut reader, limits).map_err(Canonical)?,
            schema: address(&mut reader, limits.field())?,
        },
        _ => return Err(BindingArchiveRefusal::InvalidSlot),
    };
    finish(&reader).map_err(Canonical)?;
    if &subject != row.subject() {
        return Err(BindingArchiveRefusal::SubjectMismatch);
    }
    if &check != row.check() {
        return Err(BindingArchiveRefusal::CheckMismatch);
    }
    if matches!(row.origin(), ArchivedOrigin::Generated { .. })
        && matches!(provenance, ArchivedProvenance::Unproduced)
    {
        return Err(BindingArchiveRefusal::GeneratedWithoutSchemaPin);
    }
    Ok(ArchivedBinding {
        encoded: encoded.to_vec(),
        row,
        subject,
        check,
        subject_revision,
        check_revision,
        provenance,
    })
}

pub(crate) fn read_revision(
    encoded: &[u8],
    field: usize,
) -> Result<ArchivedRevisionBinding, BindingArchiveRefusal> {
    use BindingArchiveRefusal::Canonical;
    let mut reader = cursor(encoded);
    let revision = address(&mut reader, field)?;
    let posture = match reader.byte().map_err(Canonical)? {
        0 => RevisionPosture::Derived,
        1 => RevisionPosture::Declared,
        2 => RevisionPosture::Untracked,
        _ => return Err(BindingArchiveRefusal::InvalidSlot),
    };
    finish(&reader).map_err(Canonical)?;
    Ok(ArchivedRevisionBinding { revision, posture })
}

fn address(
    reader: &mut BodyReader<'_, CandidateArchiveRefusal>,
    field: usize,
) -> Result<[u8; 32], BindingArchiveRefusal> {
    use BindingArchiveRefusal::Canonical;
    let bytes = reader.bytes().map_err(Canonical)?;
    if bytes.len() > field {
        return Err(Canonical(CandidateArchiveRefusal::FieldTooLarge));
    }
    bytes
        .try_into()
        .map_err(|_| BindingArchiveRefusal::InvalidAddressWidth)
}
