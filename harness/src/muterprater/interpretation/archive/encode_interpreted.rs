//! Complete interpreted retention over existing pressure and report owners.

use super::size_interpreted::{active_size, admit_total, known_size};
use super::{
    ArchivedInterpretedEvidence, INTERPRETED_ARCHIVE_TAG, InterpretedArchiveLimits,
    InterpretedArchiveRefusal, ValueEncoder, read_interpreted,
};
use crate::identity::{ContentAddress, encode_bytes};
use crate::muterprater::InterpretedMutationEvidence;
use crate::muterprater::backend_archive::{OriginalMaterial, retained_suite_pressure};
use crate::muterprater::discovery_archive::retain_surface;
use crate::muterprater::specimen_archive::{projection_known_size, retain_projection};
use crate::muterprater::verdict_archive::retain_mutation;
use crate::report::archive::{ArchiveRefusal, bounded, retain_trial, sum};

/// Retain complete interpreted evidence with the backend's absent-originals posture.
///
/// # Errors
///
/// Refuses known bounds before encoders, then callback, complete-size or historical join failures.
/// Each reached value is encoded once in parity input, production, evaluation and active-meaning order.
pub fn retain_interpreted<Input, Meaning>(
    evidence: &InterpretedMutationEvidence<'_, '_, '_, '_, '_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    limits: &InterpretedArchiveLimits,
) -> Result<ArchivedInterpretedEvidence, InterpretedArchiveRefusal> {
    retained(evidence, input, meaning, None, limits)
}

/// Retain complete interpreted evidence with caller-supplied console and complete backend source originals.
///
/// # Errors
///
/// Adds the backend owner's original-material refusals to the bounds and encoding refusals of `retain_interpreted`.
pub fn retain_interpreted_with_material<Input, Meaning>(
    evidence: &InterpretedMutationEvidence<'_, '_, '_, '_, '_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    console: &str,
    sources: &[(&str, &[u8])],
    limits: &InterpretedArchiveLimits,
) -> Result<ArchivedInterpretedEvidence, InterpretedArchiveRefusal> {
    retained(
        evidence,
        input,
        meaning,
        Some(OriginalMaterial { console, sources }),
        limits,
    )
}

fn retained<Input, Meaning>(
    evidence: &InterpretedMutationEvidence<'_, '_, '_, '_, '_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    original: Option<OriginalMaterial<'_>>,
    limits: &InterpretedArchiveLimits,
) -> Result<ArchivedInterpretedEvidence, InterpretedArchiveRefusal> {
    let known = known_size(evidence, input, meaning, original, limits)?;
    let trust = evidence.trust();
    let projection_known =
        projection_known_size(trust.projection(), input, meaning, limits.projection())?;
    let surface = retain_surface(trust.surface(), limits.surface())?;
    let suite = retained_suite_pressure(trust.suite(), original, limits.suite())?;
    let report = retain_trial(evidence.report(), limits.trial())?;
    let mutation = retain_mutation(evidence.mutation(), limits.mutation())?;
    let projection = retain_projection(trust.projection(), input, meaning, limits.projection())?;
    bounded(projection.encoded().len(), limits.bytes())?;
    let known = admit_total(
        sum(&[
            known
                .checked_sub(projection_known)
                .ok_or(ArchiveRefusal::SizeOutsidePlatform)?,
            projection.encoded().len(),
        ])?,
        limits,
    )?;
    let active = meaning
        .encode(evidence.meaning())
        .map_err(InterpretedArchiveRefusal::ActiveEncoder)?;
    let total = admit_total(sum(&[known, active_size(active.len(), limits)?])?, limits)?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    for word in [1u32, 1, 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    for member in [
        surface.encoded(),
        suite.encoded(),
        projection.encoded(),
        &active,
        report.encoded(),
        mutation.encoded(),
    ] {
        encode_bytes(member, &mut body);
    }
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(INTERPRETED_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_interpreted(&encoded, limits)
}
