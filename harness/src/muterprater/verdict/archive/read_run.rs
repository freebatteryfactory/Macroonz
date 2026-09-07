//! Complete ordered historical mutation-run admission without current baseline evidence.

use super::super::{
    ArchivedMutationRun, MUTATION_RUN_ARCHIVE_TAG, MutationArchiveRefusal, MutationRunArchiveLimits,
};
use super::record::read_mutation;
use crate::muterprater::{BaselineAxis, MutationCensus, MutationVerdict};
use crate::report::archive::{ArchiveRefusal, envelope, finish, frame};

/// Read a bounded complete historical mutation run and derive its census.
///
/// # Errors
///
/// Refuses malformed envelopes, unqualified run baselines, excess report populations or invalid nested records.
pub fn read_mutation_run(
    encoded: &[u8],
    limits: MutationRunArchiveLimits,
) -> Result<ArchivedMutationRun, MutationArchiveRefusal> {
    let (address, mut reader) = envelope(encoded, MUTATION_RUN_ARCHIVE_TAG, 2, limits.bytes())?;
    match reader.byte()? {
        0 => {}
        1 | 2 => return Err(MutationArchiveRefusal::BaselineNotQualified),
        _ => return Err(ArchiveRefusal::InvalidSlot.into()),
    }
    let declared = reader.u64()?;
    let count = usize::try_from(declared)
        .map_err(|_| ArchiveRefusal::LengthOutsidePlatform { declared })?;
    if count > limits.reports() {
        return Err(MutationArchiveRefusal::TooManyReports);
    }
    let mut reports = Vec::new();
    for _ in 0..count {
        reports.push(read_mutation(
            frame(&mut reader, limits.bytes())?,
            limits.bytes(),
        )?);
    }
    finish(&reader)?;
    let census = MutationCensus::over_verdicts(
        reports
            .iter()
            .map(|report| MutationVerdict::from(report.outcome())),
    );
    Ok(ArchivedMutationRun {
        encoded: encoded.to_vec(),
        address,
        baseline: BaselineAxis::Qualified,
        reports,
        census,
    })
}
