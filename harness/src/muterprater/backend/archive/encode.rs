//! Historical backend encoding from an imported manifest and optional declared originals.

use super::size::encoded_size;
use super::types::OriginalMaterial;
use super::{
    ArchivedBackendManifest, BACKEND_ARCHIVE_TAG, BackendArchiveLimits, BackendArchiveRefusal,
    read_backend,
};
use crate::identity::{ContentAddress, encode_bytes, encode_length};
use crate::muterprater::backend::roster;
use crate::muterprater::verdict_archive::retain_mutation_run;
use crate::muterprater::{
    AnnouncedRoster, BackendVersionPosture, CompiledSuiteArtifactManifest,
    MutationBackendInvocation, MutationSourceRevision, ReadingSource, WrappedBackend,
};
use crate::report::archive::write_foreign;

/// Retain an imported manifest with its historical identities and readings.
///
/// # Errors
///
/// Refuses independent byte and population ceilings or inconsistent backend claims.
pub fn retain_backend(
    manifest: &CompiledSuiteArtifactManifest,
    limits: BackendArchiveLimits,
) -> Result<ArchivedBackendManifest, BackendArchiveRefusal> {
    retained(manifest, None, limits)
}

/// Retain a manifest with the exact caller-held console and complete original source material.
///
/// # Errors
///
/// Refuses byte or population excess, duplicate or mismatched material, and console facts inconsistent with the manifest.
pub fn retain_backend_with_material(
    manifest: &CompiledSuiteArtifactManifest,
    console: &str,
    sources: &[(&str, &[u8])],
    limits: BackendArchiveLimits,
) -> Result<ArchivedBackendManifest, BackendArchiveRefusal> {
    retained(
        manifest,
        Some(OriginalMaterial { console, sources }),
        limits,
    )
}

fn retained(
    manifest: &CompiledSuiteArtifactManifest,
    original: Option<OriginalMaterial<'_>>,
    limits: BackendArchiveLimits,
) -> Result<ArchivedBackendManifest, BackendArchiveRefusal> {
    let total = encoded_size(manifest, original, limits)?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    for word in [1u32, 1, 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    invocation(manifest.invocation(), &mut body);
    let reading = manifest.reading();
    let profile = reading.profile();
    body.push(backend(profile.backend()));
    let BackendVersionPosture::Stated(version) = profile.version() else {
        return Err(BackendArchiveRefusal::ProfileMismatch);
    };
    body.push(1);
    encode_bytes(version.spelling().as_bytes(), &mut body);
    body.push(match profile.source() {
        ReadingSource::ConsoleStream => 0,
    });
    body.extend_from_slice(&profile.grammar().number().to_be_bytes());
    encode_bytes(manifest.output().address().as_bytes(), &mut body);
    encode_length(manifest.sources().len(), &mut body);
    for source in manifest.sources() {
        encode_bytes(source.file().as_bytes(), &mut body);
        encode_bytes(source.revision().address().as_bytes(), &mut body);
    }
    encode_bytes(
        retain_mutation_run(reading.run(), limits.run())?.encoded(),
        &mut body,
    );
    match reading.announced() {
        AnnouncedRoster::Unstated => body.push(0),
        AnnouncedRoster::Stated(count) => {
            body.push(1);
            body.extend_from_slice(&count.to_be_bytes());
        }
    }
    encode_length(reading.unparsed().len(), &mut body);
    for line in reading.unparsed() {
        encode_length(line.ordinal(), &mut body);
        write_foreign(Some(line.text()), &mut body);
    }
    material(manifest, original, &mut body)?;
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(BACKEND_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_backend(&encoded, limits)
}

fn invocation(invocation: &MutationBackendInvocation, body: &mut Vec<u8>) {
    body.push(backend(invocation.backend()));
    encode_bytes(invocation.version().spelling().as_bytes(), body);
    encode_bytes(invocation.command().executable().as_bytes(), body);
    encode_length(invocation.command().arguments().len(), body);
    for argument in invocation.command().arguments() {
        encode_bytes(argument.as_bytes(), body);
    }
    encode_bytes(invocation.target().target().spelling().as_bytes(), body);
    encode_bytes(invocation.target().toolchain().spelling().as_bytes(), body);
}

const fn backend(value: WrappedBackend) -> u8 {
    match value {
        WrappedBackend::CargoMutants => 0,
    }
}

fn material(
    manifest: &CompiledSuiteArtifactManifest,
    original: Option<OriginalMaterial<'_>>,
    body: &mut Vec<u8>,
) -> Result<(), BackendArchiveRefusal> {
    let Some(original) = original else {
        body.push(0);
        return Ok(());
    };
    let supplied = roster::collected(
        original.sources.iter().copied(),
        |source| source.0,
        BackendArchiveRefusal::DuplicateMaterialSource,
    )?;
    let expected = manifest
        .sources()
        .iter()
        .map(MutationSourceRevision::file)
        .collect();
    roster::matched(
        &supplied,
        &expected,
        BackendArchiveRefusal::SourceMissing,
        BackendArchiveRefusal::SourceUnexpected,
    )?;
    body.push(1);
    encode_bytes(original.console.as_bytes(), body);
    for source in manifest.sources() {
        let (_, bytes) = supplied
            .get(source.file())
            .ok_or_else(|| BackendArchiveRefusal::SourceMissing(source.file().to_owned()))?;
        encode_bytes(bytes, body);
    }
    Ok(())
}
