//! Complete size admission before backend archive encoding.

use super::types::OriginalMaterial;
use super::{BackendArchiveLimits, BackendArchiveRefusal};
use crate::muterprater::verdict_archive::mutation_run_size;
use crate::muterprater::{AnnouncedRoster, BackendVersionPosture, CompiledSuiteArtifactManifest};
use crate::report::archive::{ArchiveRefusal, bounded, foreign_size, sum};

pub(super) fn encoded_size(
    manifest: &CompiledSuiteArtifactManifest,
    original: Option<OriginalMaterial<'_>>,
    limits: BackendArchiveLimits,
) -> Result<usize, BackendArchiveRefusal> {
    let invocation = manifest.invocation();
    let reading = manifest.reading();
    population(
        invocation.command().arguments().len(),
        limits.arguments(),
        BackendArchiveRefusal::TooManyArguments,
    )?;
    population(
        manifest.sources().len(),
        limits.sources(),
        BackendArchiveRefusal::TooManySources,
    )?;
    population(
        reading.unparsed().len(),
        limits.unparsed(),
        BackendArchiveRefusal::TooManyUnparsed,
    )?;
    let bytes = limits.bytes();
    bounded(32, bytes)?;
    let mut total = sum(&[
        44,
        41,
        bounded(invocation.version().spelling().len(), bytes)?,
        bounded(invocation.command().executable().len(), bytes)?,
        bounded(invocation.target().target().spelling().len(), bytes)?,
        bounded(invocation.target().toolchain().spelling().len(), bytes)?,
    ])?;
    for argument in invocation.command().arguments() {
        total = sum(&[total, 8, bounded(argument.len(), bytes)?])?;
    }
    let BackendVersionPosture::Stated(version) = reading.profile().version() else {
        return Err(BackendArchiveRefusal::ProfileMismatch);
    };
    total = sum(&[total, 15, bounded(version.spelling().len(), bytes)?, 40, 8])?;
    for source in manifest.sources() {
        total = sum(&[total, 48, bounded(source.file().len(), bytes)?])?;
    }
    total = sum(&[
        total,
        8,
        bounded(mutation_run_size(reading.run(), limits.run())?, bytes)?,
    ])?;
    let announced = match reading.announced() {
        AnnouncedRoster::Unstated => 1,
        AnnouncedRoster::Stated(_) => 5,
    };
    total = sum(&[total, announced, 8])?;
    for line in reading.unparsed() {
        u64::try_from(line.ordinal()).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
        total = sum(&[total, 8, foreign_size(Some(line.text()), bytes)?])?;
    }
    total = sum(&[total, material_size(original, limits)?])?;
    if total > bytes.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge.into());
    }
    Ok(total)
}

fn material_size(
    original: Option<OriginalMaterial<'_>>,
    limits: BackendArchiveLimits,
) -> Result<usize, BackendArchiveRefusal> {
    let Some(original) = original else {
        return Ok(1);
    };
    population(
        original.sources.len(),
        limits.sources(),
        BackendArchiveRefusal::TooManySources,
    )?;
    let mut total = sum(&[9, bounded(original.console.len(), limits.bytes())?])?;
    for (file, bytes) in original.sources {
        bounded(file.len(), limits.bytes())?;
        total = sum(&[total, 8, bounded(bytes.len(), limits.bytes())?])?;
    }
    Ok(total)
}

fn population(
    count: usize,
    ceiling: usize,
    excess: BackendArchiveRefusal,
) -> Result<(), BackendArchiveRefusal> {
    if count > ceiling {
        return Err(excess);
    }
    u64::try_from(count).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
    Ok(())
}
