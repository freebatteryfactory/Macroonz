//! Historical retention writes only the fields each live verdict actually carries.

use super::{
    ArchivedMethod, ArchivedOracle, ORACLE_ARCHIVE_TAG, OracleArchiveRefusal, read_verdict, size,
};
use crate::identity::{ContentAddress, encode_bytes};
use crate::oracle::{
    CompilationVerdict, CompiledVerdict, StructuralVerdict, TranscriptVerdict, VectorVerdict,
};
use crate::report::archive::ArchiveLimits;

/// Retain one vector verdict and both complete buffers when they disagree.
///
/// # Errors
///
/// Refuses envelope or field ceilings before allocating retained material.
pub fn retain_vector(
    verdict: &VectorVerdict,
    limits: ArchiveLimits,
) -> Result<ArchivedOracle, OracleArchiveRefusal> {
    retain(
        ArchivedMethod::Vector,
        size::vector(verdict, limits)?,
        limits,
        |body| match verdict {
            VectorVerdict::Agrees => body.push(0),
            VectorVerdict::Disagrees(found) => {
                body.push(1);
                encode_bytes(found.expected(), body);
                encode_bytes(found.produced(), body);
            }
        },
    )
}

/// Retain one transcript verdict without inventing its absent preimage.
///
/// # Errors
///
/// Refuses envelope or field ceilings before allocating retained material.
pub fn retain_transcript(
    verdict: TranscriptVerdict,
    limits: ArchiveLimits,
) -> Result<ArchivedOracle, OracleArchiveRefusal> {
    retain(
        ArchivedMethod::Transcript,
        size::transcript(verdict, limits)?,
        limits,
        |body| match verdict {
            TranscriptVerdict::Agrees => body.push(0),
            TranscriptVerdict::Disagrees(found) => {
                body.push(1);
                encode_bytes(found.rederived(), body);
                encode_bytes(found.published(), body);
            }
        },
    )
}

/// Retain every caller-stated structural verdict field without asserting parser provenance.
///
/// # Errors
///
/// Refuses envelope or field ceilings before allocating retained material.
pub fn retain_structural(
    verdict: &StructuralVerdict,
    limits: ArchiveLimits,
) -> Result<ArchivedOracle, OracleArchiveRefusal> {
    retain(
        ArchivedMethod::Structural,
        size::structural(verdict, limits)?,
        limits,
        |body| {
            super::encode_structural::write(verdict, body);
        },
    )
}

/// Retain a caller-stated compiler read-back verdict without asserting compiler execution.
///
/// # Errors
///
/// Refuses envelope or field ceilings before allocating retained material.
pub fn retain_compiled(
    verdict: &CompiledVerdict,
    limits: ArchiveLimits,
) -> Result<ArchivedOracle, OracleArchiveRefusal> {
    retain(
        ArchivedMethod::Compiled,
        size::compiled(verdict, limits)?,
        limits,
        |body| {
            super::encode_compiled::write(verdict, body);
        },
    )
}

/// Retain an exact compilation verdict with all code and span fields it carries.
///
/// # Errors
///
/// Refuses envelope or field ceilings before allocating retained material.
pub fn retain_compilation(
    verdict: &CompilationVerdict,
    limits: ArchiveLimits,
) -> Result<ArchivedOracle, OracleArchiveRefusal> {
    retain(
        ArchivedMethod::Compilation,
        size::compilation(verdict, limits)?,
        limits,
        |body| {
            super::encode_compiled::compilation(verdict, body);
        },
    )
}

fn retain(
    method: ArchivedMethod,
    total: usize,
    limits: ArchiveLimits,
    write: impl FnOnce(&mut Vec<u8>),
) -> Result<ArchivedOracle, OracleArchiveRefusal> {
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    for word in [1u32, method.slot(), 0] {
        body.extend_from_slice(&word.to_be_bytes());
    }
    write(&mut body);
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(ORACLE_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_verdict(&encoded, method, limits)
}
