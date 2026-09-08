//! Complete envelope sizing precedes retained buffer allocation.

use crate::oracle::{
    CompilationDisagreement, CompilationVerdict, CompiledDisagreement, CompiledVerdict,
    PrimarySourceSpan, StructuralDisagreement, StructuralVerdict, TranscriptVerdict, VectorVerdict,
};
use crate::report::archive::{ArchiveLimits, ArchiveRefusal, bounded, sum};

fn envelope(payload: usize, limits: ArchiveLimits) -> Result<usize, ArchiveRefusal> {
    let total = sum(&[45, payload])?;
    if total > limits.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge);
    }
    Ok(total)
}

pub(super) fn vector(
    verdict: &VectorVerdict,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let payload = match verdict {
        VectorVerdict::Agrees => 0,
        VectorVerdict::Disagrees(found) => sum(&[
            16,
            bounded(found.expected().len(), limits)?,
            bounded(found.produced().len(), limits)?,
        ])?,
    };
    envelope(payload, limits)
}

pub(super) fn transcript(
    verdict: TranscriptVerdict,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let payload = match verdict {
        TranscriptVerdict::Agrees => 0,
        TranscriptVerdict::Disagrees(_) => {
            bounded(32, limits)?;
            80
        }
    };
    envelope(payload, limits)
}

pub(super) fn structural(
    verdict: &StructuralVerdict,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let payload = match verdict {
        StructuralVerdict::Conforms | StructuralVerdict::Unparsable => 0,
        StructuralVerdict::Deviates(found) => structural_disagreement(found, limits)?,
    };
    envelope(payload, limits)
}

fn structural_disagreement(
    found: &StructuralDisagreement,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    match found {
        StructuralDisagreement::UnexpectedItem => Ok(1),
        StructuralDisagreement::OutputCardinality { .. } => Ok(17),
        StructuralDisagreement::DuplicateImplementation { .. }
        | StructuralDisagreement::ImplementationTarget { .. }
        | StructuralDisagreement::TraitPath { .. }
        | StructuralDisagreement::ImplPosture { .. } => Ok(9),
        StructuralDisagreement::MeaningBearingAttribute { attribute, .. } => {
            sum(&[17, bounded(attribute.len(), limits)?])
        }
        StructuralDisagreement::UnexpectedImplMember { member, .. }
        | StructuralDisagreement::DuplicateMember { member, .. }
        | StructuralDisagreement::MissingImplMember { member, .. }
        | StructuralDisagreement::MemberValueUnread { member, .. }
        | StructuralDisagreement::MemberValue { member, .. } => {
            sum(&[17, bounded(member.len(), limits)?])
        }
    }
}

pub(super) fn compiled(
    verdict: &CompiledVerdict,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let payload = match verdict {
        CompiledVerdict::Conforms => 0,
        CompiledVerdict::Deviates(
            CompiledDisagreement::AcceptedWhereRefusalDeclared
            | CompiledDisagreement::RefusedWhereAcceptanceDeclared,
        ) => 1,
        CompiledVerdict::Deviates(
            CompiledDisagreement::UnexpectedMember { member }
            | CompiledDisagreement::DuplicateMember { member }
            | CompiledDisagreement::MissingMember { member }
            | CompiledDisagreement::MemberValue { member },
        ) => sum(&[9, bounded(member.len(), limits)?])?,
    };
    envelope(payload, limits)
}

pub(super) fn compilation(
    verdict: &CompilationVerdict,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    let payload = match verdict {
        CompilationVerdict::Conforms => 0,
        CompilationVerdict::Deviates(found) => match found {
            CompilationDisagreement::AcceptedWhereRefusalDeclared => 1,
            CompilationDisagreement::RefusedWhereAcceptanceDeclared { observed } => sum(&[
                9,
                bounded(observed.code().spelling().len(), limits)?,
                span(observed.primary(), limits)?,
            ])?,
            CompilationDisagreement::ErrorCode { expected, observed } => sum(&[
                17,
                bounded(expected.spelling().len(), limits)?,
                bounded(observed.spelling().len(), limits)?,
            ])?,
            CompilationDisagreement::PrimarySpan { expected, observed } => {
                sum(&[1, span(expected, limits)?, span(observed, limits)?])?
            }
        },
    };
    envelope(payload, limits)
}

fn span(primary: &PrimarySourceSpan, limits: ArchiveLimits) -> Result<usize, ArchiveRefusal> {
    sum(&[40, bounded(primary.source().spelling().len(), limits)?])
}
