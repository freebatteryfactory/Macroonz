//! Historical selection coordinates shared by composed records.

use crate::identity::encode_bytes;
use crate::muterprater::ActiveSelection;
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, bounded, name_size, sum};

pub(crate) fn selection_size(
    selection: ActiveSelection,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    bounded(32, limits)?;
    sum(&[80, name_size(selection.point().name(), limits)?])
}

pub(crate) fn write_selection(selection: ActiveSelection, body: &mut Vec<u8>) {
    encode_bytes(selection.surface().address().as_bytes(), body);
    selection.point().name().encode_into(body);
    encode_bytes(selection.alternative().address().as_bytes(), body);
}
