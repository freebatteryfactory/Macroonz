//! Bounded historical invocation, profile and roster members.

use super::super::{
    ArchivedAdapterProfile, ArchivedBackendInvocation, ArchivedBackendSource, ArchivedUnparsedLine,
    BackendArchiveLimits, BackendArchiveRefusal,
};
use crate::identity::BodyReader;
use crate::muterprater::{AnnouncedRoster, GrammarVersion, ReadingSource, WrappedBackend};
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, claim, foreign, text};
use crate::report::{TargetBinding, TargetTriple, ToolchainIdentity};

pub(super) fn count(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    ceiling: usize,
    excess: BackendArchiveRefusal,
) -> Result<usize, BackendArchiveRefusal> {
    let declared = reader.u64()?;
    let count = usize::try_from(declared)
        .map_err(|_| ArchiveRefusal::LengthOutsidePlatform { declared })?;
    if count > ceiling {
        return Err(excess);
    }
    Ok(count)
}

fn nonempty(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<String, ArchiveRefusal> {
    let value = text(reader, limits)?;
    if value.is_empty() {
        return Err(ArchiveRefusal::InvalidText);
    }
    Ok(value.to_owned())
}

fn backend(reader: &mut BodyReader<'_, ArchiveRefusal>) -> Result<WrappedBackend, ArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(WrappedBackend::CargoMutants),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

pub(super) fn invocation(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: BackendArchiveLimits,
) -> Result<ArchivedBackendInvocation, BackendArchiveRefusal> {
    let backend = backend(reader)?;
    let version = nonempty(reader, limits.bytes())?;
    let executable = nonempty(reader, limits.bytes())?;
    let count = count(
        reader,
        limits.arguments(),
        BackendArchiveRefusal::TooManyArguments,
    )?;
    let mut arguments = Vec::new();
    for _ in 0..count {
        arguments.push(text(reader, limits.bytes())?.to_owned());
    }
    let target = TargetBinding::bound(
        TargetTriple::declared(text(reader, limits.bytes())?),
        ToolchainIdentity::declared(text(reader, limits.bytes())?),
    );
    Ok(ArchivedBackendInvocation {
        backend,
        version,
        executable,
        arguments,
        target,
    })
}

pub(super) fn profile(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    invocation: &ArchivedBackendInvocation,
    limits: ArchiveLimits,
) -> Result<ArchivedAdapterProfile, BackendArchiveRefusal> {
    let backend = backend(reader)?;
    match reader.byte()? {
        1 => {}
        0 => return Err(BackendArchiveRefusal::ProfileMismatch),
        _ => return Err(ArchiveRefusal::InvalidSlot.into()),
    }
    let version = nonempty(reader, limits)?;
    let source = match reader.byte()? {
        0 => ReadingSource::ConsoleStream,
        _ => return Err(ArchiveRefusal::InvalidSlot.into()),
    };
    let grammar = GrammarVersion::adapter(reader.u32()?);
    let expected = crate::muterprater::wrap::console_profile(
        crate::muterprater::BackendVersionPosture::Unstated,
    );
    if backend != invocation.backend
        || version != invocation.version
        || source != expected.source()
        || grammar != expected.grammar()
    {
        return Err(BackendArchiveRefusal::ProfileMismatch);
    }
    Ok(ArchivedAdapterProfile {
        backend,
        version,
        source,
        grammar,
    })
}

pub(super) fn sources(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: BackendArchiveLimits,
) -> Result<Vec<ArchivedBackendSource>, BackendArchiveRefusal> {
    let count = count(
        reader,
        limits.sources(),
        BackendArchiveRefusal::TooManySources,
    )?;
    let mut sources: Vec<ArchivedBackendSource> = Vec::new();
    for _ in 0..count {
        let file = nonempty(reader, limits.bytes())?;
        if sources.last().is_some_and(|prior| prior.file >= file) {
            return Err(BackendArchiveRefusal::InvalidSourceOrder);
        }
        sources.push(ArchivedBackendSource {
            file,
            revision: claim(reader, limits.bytes())?,
            original: None,
        });
    }
    Ok(sources)
}

pub(super) fn announced(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
) -> Result<AnnouncedRoster, ArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(AnnouncedRoster::Unstated),
        1 => Ok(AnnouncedRoster::Stated(reader.u32()?)),
        _ => Err(ArchiveRefusal::InvalidSlot),
    }
}

pub(super) fn unparsed(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: BackendArchiveLimits,
) -> Result<Vec<ArchivedUnparsedLine>, BackendArchiveRefusal> {
    let count = count(
        reader,
        limits.unparsed(),
        BackendArchiveRefusal::TooManyUnparsed,
    )?;
    let mut lines: Vec<ArchivedUnparsedLine> = Vec::new();
    for _ in 0..count {
        let ordinal = reader.u64()?;
        if lines.last().is_some_and(|prior| prior.ordinal >= ordinal) {
            return Err(BackendArchiveRefusal::InvalidUnparsedOrder);
        }
        lines.push(ArchivedUnparsedLine {
            ordinal,
            text: foreign(reader, limits.bytes())?.ok_or(ArchiveRefusal::InvalidForeignText)?,
        });
    }
    Ok(lines)
}
