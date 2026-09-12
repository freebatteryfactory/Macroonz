use super::{MutationCustodyError, MutationError, MutationSources};
use crate::harness::muterprater::MutationSourceRevision;
use crate::harness::muterprater::backend_archive::{
    ArchivedBackendManifest, BackendSourceComparison,
};
use crate::harness::oracle::RelativeSourcePath;
use std::path::Path;

/// Reads today's declared source locations and compares them without upgrading historical authority.
///
/// # Errors
/// Refuses invalid historical paths, bounded filesystem failures and differing source revisions.
pub fn compare_historical<'archive>(
    archive: &'archive ArchivedBackendManifest,
    root: &Path,
    byte_bound: usize,
) -> Result<BackendSourceComparison<'archive>, MutationCustodyError> {
    let current = revisions(
        root,
        archive
            .sources()
            .iter()
            .map(crate::harness::muterprater::backend_archive::ArchivedBackendSource::file),
        byte_bound,
    )?;
    archive
        .compared_sources(current)
        .map_err(MutationCustodyError::Historical)
}

pub(super) fn revisions<'file>(
    root: &Path,
    files: impl Iterator<Item = &'file str>,
    byte_bound: usize,
) -> Result<Vec<MutationSourceRevision>, MutationCustodyError> {
    let mut paths = Vec::new();
    for file in files {
        paths.push(
            RelativeSourcePath::informed(file).map_err(|error| {
                source_error(MutationError::Configuration(format!("{error:?}")))
            })?,
        );
    }
    if paths.is_empty() {
        return Ok(Vec::new());
    }
    let sources = MutationSources::declared(paths, byte_bound).map_err(source_error)?;
    super::source::capture(root, &sources)
        .map_err(source_error)?
        .iter()
        .map(|source| {
            MutationSourceRevision::from_content(&source.file, &source.bytes)
                .map_err(|error| source_error(MutationError::Configuration(format!("{error:?}"))))
        })
        .collect()
}

fn source_error(error: MutationError) -> MutationCustodyError {
    MutationCustodyError::Source(Box::new(error))
}
