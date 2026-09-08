//! Complete known composition bounds before any retained-value callback.

use super::{InterpretedArchiveLimits, InterpretedArchiveRefusal, ValueEncoder};
use crate::muterprater::InterpretedMutationEvidence;
use crate::muterprater::backend_archive::{OriginalMaterial, suite_pressure_size};
use crate::muterprater::discovery_archive::surface_size;
use crate::muterprater::specimen_archive::projection_known_size;
use crate::muterprater::verdict_archive::mutation_size;
use crate::report::archive::{ArchiveRefusal, bounded, sum, trial_size};

pub(super) fn known_size<Input, Meaning>(
    evidence: &InterpretedMutationEvidence<'_, '_, '_, '_, '_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    original: Option<OriginalMaterial<'_>>,
    limits: &InterpretedArchiveLimits,
) -> Result<usize, InterpretedArchiveRefusal> {
    let trust = evidence.trust();
    let bytes = limits.bytes();
    admit_total(
        sum(&[
            92,
            bounded(surface_size(trust.surface(), limits.surface())?, bytes)?,
            bounded(
                suite_pressure_size(trust.suite(), original, limits.suite())?,
                bytes,
            )?,
            bounded(
                projection_known_size(trust.projection(), input, meaning, limits.projection())?,
                bytes,
            )?,
            bounded(trial_size(evidence.report(), limits.trial())?, bytes)?,
            bounded(
                mutation_size(evidence.mutation(), limits.mutation())?,
                bytes,
            )?,
        ])?,
        limits,
    )
}

pub(super) fn active_size(
    length: usize,
    limits: &InterpretedArchiveLimits,
) -> Result<usize, InterpretedArchiveRefusal> {
    if length > limits.active() {
        return Err(InterpretedArchiveRefusal::ActiveValueTooLarge);
    }
    Ok(bounded(length, limits.bytes())?)
}

pub(super) fn admit_total(
    length: usize,
    limits: &InterpretedArchiveLimits,
) -> Result<usize, InterpretedArchiveRefusal> {
    if length > limits.bytes().envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge.into());
    }
    Ok(length)
}
