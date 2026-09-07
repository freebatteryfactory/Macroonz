//! Bound every retained member before capsule capture or archive encoding allocates.

use super::{ReductionArchiveLimits, ReductionArchiveRefusal};
use crate::generate::{ReductionEvidence, SemanticReducerExecution};
use crate::report::archive::{
    ArchiveLimits, ArchiveRefusal, bounded, capsule_size, name_size, sum,
};

pub(super) fn reducer_size(
    reducer: SemanticReducerExecution,
    limits: ArchiveLimits,
) -> Result<usize, ArchiveRefusal> {
    u64::try_from(reducer.candidates()).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
    u64::try_from(reducer.probes()).map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
    bounded(
        sum(&[57, name_size(reducer.reducer().name(), limits)?])?,
        limits,
    )
}

pub(super) fn encoded_size(
    evidence: &ReductionEvidence,
    limits: ReductionArchiveLimits,
) -> Result<usize, ReductionArchiveRefusal> {
    if evidence.semantic_reducers().len() > limits.reducers() {
        return Err(ReductionArchiveRefusal::TooManyReducers);
    }
    u64::try_from(evidence.semantic_reducers().len())
        .map_err(|_| ArchiveRefusal::SizeOutsidePlatform)?;
    let bytes = limits.bytes();
    bounded(32, bytes)?;
    let capsule = bounded(
        capsule_size(
            evidence.standing().key(),
            evidence.outcome().fingerprint(),
            evidence.outcome().input(),
            evidence.generation(),
            evidence.minimization(),
            bytes,
        )?,
        bytes,
    )?;
    let mut total = sum(&[120, capsule])?;
    for reducer in evidence.semantic_reducers() {
        total = sum(&[total, 8, reducer_size(*reducer, bytes)?])?;
    }
    if total > bytes.envelope() {
        return Err(ArchiveRefusal::EnvelopeTooLarge.into());
    }
    Ok(total)
}
