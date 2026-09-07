//! Complete bounded historical backend manifest admission.

use super::super::{
    ArchivedBackendManifest, ArchivedBackendSource, BACKEND_ARCHIVE_TAG, BackendArchiveLimits,
    BackendArchiveRefusal,
};
use super::{material, members};
use crate::muterprater::backend::{roster, wrap};
use crate::muterprater::verdict_archive::{
    ArchivedMutationRun, ArchivedMutationSite, read_mutation_run,
};
use crate::report::archive::{ArchiveRefusal, claim, envelope, finish, frame};
use std::collections::{BTreeMap, BTreeSet};

/// Read a complete historical backend manifest without reconstructing live custody.
///
/// # Errors
///
/// Refuses malformed records, inconsistent backend claims, excess populations or mismatched originals.
pub fn read_backend(
    encoded: &[u8],
    limits: BackendArchiveLimits,
) -> Result<ArchivedBackendManifest, BackendArchiveRefusal> {
    let (address, mut reader) = envelope(encoded, BACKEND_ARCHIVE_TAG, 1, limits.bytes())?;
    let invocation = members::invocation(&mut reader, limits)?;
    let profile = members::profile(&mut reader, &invocation, limits.bytes())?;
    let output = claim(&mut reader, limits.bytes())?;
    let mut sources = members::sources(&mut reader, limits)?;
    let run = read_mutation_run(frame(&mut reader, limits.bytes())?, limits.run())?;
    source_roster(&sources, &run)?;
    let announced = members::announced(&mut reader)?;
    let unparsed = members::unparsed(&mut reader, limits)?;
    let original = material::original(&mut reader, output, &mut sources, limits.bytes())?;
    finish(&reader)?;
    let manifest = ArchivedBackendManifest {
        encoded: encoded.to_vec(),
        address,
        invocation,
        profile,
        output,
        sources,
        run,
        announced,
        unparsed,
        original,
    };
    if let Some(console) = manifest.original_console() {
        wrap::check_archived_console(
            &manifest,
            core::str::from_utf8(console).map_err(|_| ArchiveRefusal::InvalidText)?,
        )?;
    }
    Ok(manifest)
}

fn source_roster(
    sources: &[ArchivedBackendSource],
    run: &ArchivedMutationRun,
) -> Result<(), BackendArchiveRefusal> {
    let supplied: BTreeMap<String, &ArchivedBackendSource> = sources
        .iter()
        .map(|source| (source.file().to_owned(), source))
        .collect();
    let mut expected = BTreeSet::new();
    for report in run.reports() {
        wrap::archived_word(report)?;
        match report.target().site() {
            ArchivedMutationSite::Reported(coordinate) => {
                expected.insert(coordinate.file());
            }
            ArchivedMutationSite::Declared(_) => {
                return Err(BackendArchiveRefusal::BackendRecordMismatch);
            }
        }
    }
    roster::matched(
        &supplied,
        &expected,
        BackendArchiveRefusal::SourceMissing,
        BackendArchiveRefusal::SourceUnexpected,
    )
}
