//! Complete mutation-run encoding through the existing mutation-record writer.

use super::size_record;
use super::{
    ArchivedMutationRun, MUTATION_RUN_ARCHIVE_TAG, MutationArchiveRefusal,
    MutationRunArchiveLimits, read_mutation_run, retain_mutation,
};
use crate::identity::{ContentAddress, encode_bytes, encode_length};
use crate::muterprater::MutationRun;
use crate::report::archive::{ArchiveRefusal, bounded, sum};

/// Retain every mutation in its original order, with bounded historical run framing.
///
/// # Errors
///
/// Refuses excess report populations, fields or complete envelopes before encoding any report.
pub fn retain_mutation_run(
    run: &MutationRun,
    limits: MutationRunArchiveLimits,
) -> Result<ArchivedMutationRun, MutationArchiveRefusal> {
    let total = encoded_size(run, limits)?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    for word in [1u32, 2, 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    body.push(0);
    encode_length(run.reports().len(), &mut body);
    for report in run.reports() {
        encode_bytes(
            retain_mutation(report, limits.bytes())?.encoded(),
            &mut body,
        );
    }
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(MUTATION_RUN_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_mutation_run(&encoded, limits)
}

pub(crate) fn encoded_size(
    run: &MutationRun,
    limits: MutationRunArchiveLimits,
) -> Result<usize, MutationArchiveRefusal> {
    if run.reports().len() > limits.reports() {
        return Err(MutationArchiveRefusal::TooManyReports);
    }
    u64::try_from(run.reports().len()).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
    let mut total = 53;
    for report in run.reports() {
        let size = bounded(
            size_record::encoded_size(report, limits.bytes())?,
            limits.bytes(),
        )?;
        total = sum(&[total, 8, size])?;
    }
    if total > limits.bytes().envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge.into());
    }
    Ok(total)
}
