//! Projection retention composes qualified parity and the existing report owners.

use super::size::{admit_total, known_size};
use super::{
    ArchivedProjectionPressure, PROJECTION_ARCHIVE_TAG, ProjectionArchiveLimits,
    ProjectionArchiveRefusal, read_projection,
};
use crate::identity::{ContentAddress, encode_bytes};
use crate::muterprater::discovery_archive::write_selection;
use crate::muterprater::interpretation_archive::{ValueEncoder, qualified_size, retain_qualified};
use crate::muterprater::verdict_archive::retain_mutation;
use crate::muterprater::{ArtifactContent, CompiledProjectionPressure};
use crate::report::archive::{ArchiveRefusal, bounded, retain_trial, sum};

/// Retain exact compiled pressure through the existing qualified-parity value encoders.
///
/// # Errors
///
/// Refuses known bounds before callbacks, then encoder, complete-size and historical consistency failures.
/// Each reached generic value is encoded once; no materializer, host or witness is invoked.
pub fn retain_projection<Input, Meaning>(
    pressure: &CompiledProjectionPressure<'_, '_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    limits: ProjectionArchiveLimits,
) -> Result<ArchivedProjectionPressure, ProjectionArchiveRefusal> {
    let known = known_size(pressure, input, meaning, limits)?;
    let parity_known = qualified_size(pressure.parity(), input, meaning, limits.parity())?;
    let parity = retain_qualified(pressure.parity(), input, meaning, limits.parity())?;
    bounded(parity.encoded().len(), limits.bytes())?;
    let total = admit_total(
        sum(&[
            known
                .checked_sub(parity_known)
                .ok_or(ArchiveRefusal::SizeOutsidePlatform)?,
            parity.encoded().len(),
        ])?,
        limits,
    )?;
    let baseline = retain_trial(pressure.baseline_report(), limits.trial())?;
    let selected = retain_trial(pressure.selected_report(), limits.trial())?;
    let mutation = retain_mutation(pressure.mutation(), limits.mutation())?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());
    encode_bytes(parity.encoded(), &mut body);
    write_artifact(pressure.baseline_content(), &mut body);
    write_artifact(pressure.selected_content(), &mut body);
    write_selection(pressure.standing().selection(), &mut body);
    encode_bytes(baseline.encoded(), &mut body);
    encode_bytes(selected.encoded(), &mut body);
    encode_bytes(mutation.encoded(), &mut body);
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(PROJECTION_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_projection(&encoded, limits)
}

fn write_artifact(content: &ArtifactContent, body: &mut Vec<u8>) {
    encode_bytes(content.identity().address().as_bytes(), body);
    encode_bytes(content.bytes(), body);
}
