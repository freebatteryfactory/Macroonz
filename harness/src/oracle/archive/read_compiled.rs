//! Historical compiler payloads reuse pure diagnostic admission without executing a compiler.

use super::super::{ArchivedVerdict, OracleArchiveRefusal};
use crate::identity::BodyReader;
use crate::oracle::{
    CompilationDisagreement, CompiledDisagreement, DiagnosticAnchor, PrimarySourceSpan,
    RelativeSourcePath, RustcErrorCode, SourcePosition,
};
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, text};

pub(super) fn read(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedVerdict, OracleArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedVerdict::CompiledConforms),
        1 => Ok(ArchivedVerdict::CompiledDeviates(disagreement(
            reader, limits,
        )?)),
        _ => Err(ArchiveRefusal::InvalidSlot.into()),
    }
}

fn disagreement(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<CompiledDisagreement, OracleArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(CompiledDisagreement::AcceptedWhereRefusalDeclared),
        1 => Ok(CompiledDisagreement::RefusedWhereAcceptanceDeclared),
        2 => Ok(CompiledDisagreement::UnexpectedMember {
            member: text(reader, limits)?.to_owned(),
        }),
        3 => Ok(CompiledDisagreement::DuplicateMember {
            member: text(reader, limits)?.to_owned(),
        }),
        4 => Ok(CompiledDisagreement::MissingMember {
            member: text(reader, limits)?.to_owned(),
        }),
        5 => Ok(CompiledDisagreement::MemberValue {
            member: text(reader, limits)?.to_owned(),
        }),
        _ => Err(ArchiveRefusal::InvalidSlot.into()),
    }
}

pub(super) fn compilation(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedVerdict, OracleArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(ArchivedVerdict::CompilationConforms),
        1 => Ok(ArchivedVerdict::CompilationDeviates(
            compilation_disagreement(reader, limits)?,
        )),
        _ => Err(ArchiveRefusal::InvalidSlot.into()),
    }
}

fn compilation_disagreement(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<CompilationDisagreement, OracleArchiveRefusal> {
    match reader.byte()? {
        0 => Ok(CompilationDisagreement::AcceptedWhereRefusalDeclared),
        1 => Ok(CompilationDisagreement::RefusedWhereAcceptanceDeclared {
            observed: DiagnosticAnchor::at(code(reader, limits)?, span(reader, limits)?),
        }),
        2 => Ok(CompilationDisagreement::ErrorCode {
            expected: code(reader, limits)?,
            observed: code(reader, limits)?,
        }),
        3 => Ok(CompilationDisagreement::PrimarySpan {
            expected: span(reader, limits)?,
            observed: span(reader, limits)?,
        }),
        _ => Err(ArchiveRefusal::InvalidSlot.into()),
    }
}

fn code(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<RustcErrorCode, OracleArchiveRefusal> {
    Ok(RustcErrorCode::informed(text(reader, limits)?)?)
}

fn span(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<PrimarySourceSpan, OracleArchiveRefusal> {
    let source = RelativeSourcePath::informed(text(reader, limits)?)?;
    let start = SourcePosition::informed(reader.u64()?, reader.u64()?)?;
    let end = SourcePosition::informed(reader.u64()?, reader.u64()?)?;
    Ok(PrimarySourceSpan::informed(source, start, end)?)
}
