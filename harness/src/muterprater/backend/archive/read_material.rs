//! Exact supplied original bytes joined to historical content claims.

use super::super::{ArchivedBackendSource, BackendArchiveRefusal};
use crate::identity::{BodyReader, ContentAddress};
use crate::muterprater::{BACKEND_OUTPUT_TAG, MUTATION_SOURCE_REVISION_TAG};
use crate::report::archive::{AddressClaim, ArchiveLimits, ArchiveRefusal, frame};

pub(super) fn original(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    output: AddressClaim,
    sources: &mut [ArchivedBackendSource],
    limits: ArchiveLimits,
) -> Result<Option<Vec<u8>>, BackendArchiveRefusal> {
    match reader.byte()? {
        0 => return Ok(None),
        1 => {}
        _ => return Err(ArchiveRefusal::InvalidSlot.into()),
    }
    let console = frame(reader, limits)?;
    if ContentAddress::derived(BACKEND_OUTPUT_TAG, console).as_bytes() != output.as_bytes() {
        return Err(BackendArchiveRefusal::OutputMaterialMismatch);
    }
    core::str::from_utf8(console).map_err(|_| ArchiveRefusal::InvalidText)?;
    for source in sources {
        let bytes = frame(reader, limits)?;
        if ContentAddress::derived(MUTATION_SOURCE_REVISION_TAG, bytes).as_bytes()
            != source.revision.as_bytes()
        {
            return Err(BackendArchiveRefusal::SourceMaterialMismatch(
                source.file.clone(),
            ));
        }
        source.original = Some(bytes.to_vec());
    }
    Ok(Some(console.to_vec()))
}
