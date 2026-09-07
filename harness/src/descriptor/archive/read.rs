//! Bounded historical reading of the descriptor owner's canonical candidate bytes.

use super::super::{
    ArchivedCandidate, ArchivedName, ArchivedRowFields, ArchivedSynthesis, CandidateArchiveLimits,
    CandidateArchiveRefusal,
};
use crate::descriptor::Row;
use crate::descriptor::encode::ROW_ENCODING_VERSION;
use crate::identity::BodyReader;

/// Retain an existing candidate's canonical descriptor as owned historical data.
///
/// # Errors
///
/// Refuses another origin or material exceeding the independently supplied ceilings.
pub fn retain_candidate(
    row: &Row,
    limits: CandidateArchiveLimits,
) -> Result<ArchivedCandidate, CandidateArchiveRefusal> {
    read_candidate(row.canonical_bytes().as_bytes(), limits)
}

/// Read canonical row material solely as a historical candidate descriptor.
///
/// # Errors
///
/// Refuses excess bounds, unsupported grammar, non-candidate origins and noncanonical labels.
pub fn read_candidate(
    canonical: &[u8],
    limits: CandidateArchiveLimits,
) -> Result<ArchivedCandidate, CandidateArchiveRefusal> {
    let (mut reader, fields) = descriptor(canonical, limits)?;
    let origin = reader.byte()?;
    if origin != 3 {
        return Err(CandidateArchiveRefusal::NotCandidate { found: origin });
    }
    let synthesis = synthesis(&mut reader, limits)?;
    finish(&reader)?;
    let ArchivedRowFields {
        claim,
        execution_suite,
        roles,
        tags,
        subject,
        check,
        population,
    } = fields;
    Ok(ArchivedCandidate {
        canonical: canonical.to_vec(),
        claim,
        execution_suite,
        roles,
        tags,
        subject,
        check,
        population,
        synthesis,
    })
}

pub(super) fn descriptor(
    canonical: &[u8],
    limits: CandidateArchiveLimits,
) -> Result<(BodyReader<'_, CandidateArchiveRefusal>, ArchivedRowFields), CandidateArchiveRefusal> {
    if canonical.len() > limits.bytes() {
        return Err(CandidateArchiveRefusal::BytesTooLarge);
    }
    let mut reader = BodyReader::over(canonical, CandidateArchiveRefusal::Truncated, |declared| {
        CandidateArchiveRefusal::LengthOutsidePlatform { declared }
    });
    let format = reader.u32()?;
    if format != ROW_ENCODING_VERSION {
        return Err(CandidateArchiveRefusal::UnsupportedFormat { found: format });
    }
    let claim = name(&mut reader, limits)?;
    let execution_suite = name(&mut reader, limits)?;
    let roles = labels(&mut reader, limits)?;
    let tags = labels(&mut reader, limits)?;
    let subject = name(&mut reader, limits)?;
    let check = name(&mut reader, limits)?;
    let population = name(&mut reader, limits)?;
    Ok((
        reader,
        ArchivedRowFields {
            claim,
            execution_suite,
            roles,
            tags,
            subject,
            check,
            population,
        },
    ))
}

pub(super) fn synthesis(
    reader: &mut BodyReader<'_, CandidateArchiveRefusal>,
    limits: CandidateArchiveLimits,
) -> Result<ArchivedSynthesis, CandidateArchiveRefusal> {
    match reader.byte()? {
        1 => Ok(ArchivedSynthesis::Survivor(name(reader, limits)?)),
        2 => Ok(ArchivedSynthesis::ProofGap),
        _ => Err(CandidateArchiveRefusal::InvalidSynthesis),
    }
}

pub(super) fn finish(
    reader: &BodyReader<'_, CandidateArchiveRefusal>,
) -> Result<(), CandidateArchiveRefusal> {
    if reader.remaining() != 0 {
        return Err(CandidateArchiveRefusal::TrailingBytes);
    }
    Ok(())
}

pub(super) fn name(
    reader: &mut BodyReader<'_, CandidateArchiveRefusal>,
    limits: CandidateArchiveLimits,
) -> Result<ArchivedName, CandidateArchiveRefusal> {
    let namespace = text(reader, limits)?;
    let stem = text(reader, limits)?;
    ArchivedName::named(namespace, stem).map_err(|_| CandidateArchiveRefusal::InvalidName)
}

fn text<'body>(
    reader: &mut BodyReader<'body, CandidateArchiveRefusal>,
    limits: CandidateArchiveLimits,
) -> Result<&'body str, CandidateArchiveRefusal> {
    let bytes = reader.bytes()?;
    if bytes.len() > limits.field() {
        return Err(CandidateArchiveRefusal::FieldTooLarge);
    }
    core::str::from_utf8(bytes).map_err(|_| CandidateArchiveRefusal::InvalidName)
}

fn labels(
    reader: &mut BodyReader<'_, CandidateArchiveRefusal>,
    limits: CandidateArchiveLimits,
) -> Result<Vec<ArchivedName>, CandidateArchiveRefusal> {
    let count = reader.count()?;
    if count > limits.labels() {
        return Err(CandidateArchiveRefusal::TooManyLabels);
    }
    let mut labels: Vec<ArchivedName> = Vec::new();
    for _ in 0..count {
        let label = name(reader, limits)?;
        if let Some(previous) = labels.last()
            && (previous.namespace(), previous.stem()) >= (label.namespace(), label.stem())
        {
            return Err(CandidateArchiveRefusal::NonCanonicalLabels);
        }
        labels.push(label);
    }
    Ok(labels)
}
