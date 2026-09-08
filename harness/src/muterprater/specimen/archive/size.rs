//! Complete known projection bounds before any generic value encoder.

use super::{ProjectionArchiveLimits, ProjectionArchiveRefusal};
use crate::muterprater::CompiledProjectionPressure;
use crate::muterprater::discovery_archive::selection_size;
use crate::muterprater::interpretation_archive::{ValueEncoder, qualified_size};
use crate::muterprater::verdict_archive::mutation_size;
use crate::report::archive::{ArchiveRefusal, bounded, sum, trial_size};

pub(crate) fn known_size<Input, Meaning>(
    pressure: &CompiledProjectionPressure<'_, '_, '_, Input, Meaning>,
    input: &ValueEncoder<Input>,
    meaning: &ValueEncoder<Meaning>,
    limits: ProjectionArchiveLimits,
) -> Result<usize, ProjectionArchiveRefusal> {
    let standing = pressure.standing();
    let reading = pressure.parity().reading();
    if standing.artifact() != pressure.selected_content().identity()
        || standing.pair() != reading.pair().standing()
        || standing.execution() != reading.production_report().standing().key()
        || standing.check() != reading.witness().check_ref()
        || standing.selection().surface() != standing.pair().surface()
    {
        return Err(ProjectionArchiveRefusal::StandingMismatch);
    }
    if pressure.baseline_content().identity() == pressure.selected_content().identity() {
        return Err(ProjectionArchiveRefusal::ArtifactDidNotChange);
    }
    let bytes = limits.bytes();
    bounded(32, bytes)?;
    let parity = bounded(
        qualified_size(pressure.parity(), input, meaning, limits.parity())?,
        bytes,
    )?;
    admit_total(
        sum(&[
            172,
            parity,
            source_size(pressure.baseline_content().bytes().len(), limits)?,
            source_size(pressure.selected_content().bytes().len(), limits)?,
            selection_size(standing.selection(), bytes)?,
            bounded(
                trial_size(pressure.baseline_report(), limits.trial())?,
                bytes,
            )?,
            bounded(
                trial_size(pressure.selected_report(), limits.trial())?,
                bytes,
            )?,
            bounded(
                mutation_size(pressure.mutation(), limits.mutation())?,
                bytes,
            )?,
        ])?,
        limits,
    )
}

pub(super) fn source_size(
    length: usize,
    limits: ProjectionArchiveLimits,
) -> Result<usize, ProjectionArchiveRefusal> {
    if length > limits.source() {
        return Err(ProjectionArchiveRefusal::SourceTooLarge);
    }
    Ok(bounded(length, limits.bytes())?)
}

pub(super) fn admit_total(
    total: usize,
    limits: ProjectionArchiveLimits,
) -> Result<usize, ProjectionArchiveRefusal> {
    if total > limits.bytes().envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge.into());
    }
    Ok(total)
}
