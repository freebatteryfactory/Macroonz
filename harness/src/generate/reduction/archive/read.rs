//! Bounded historical reduction decoding through the existing archive and capsule owners.

use super::super::{
    ArchivedReduction, ArchivedReductionCensus, ArchivedSemanticReducer, REDUCTION_ARCHIVE_TAG,
    ReductionArchiveLimits, ReductionArchiveRefusal,
};
use super::account;
use crate::generate::{ByteReducerExecution, ByteReducerId, ReductionBudget, ReductionHalt};
use crate::identity::BodyReader;
use crate::report::archive::{
    ArchiveLimits, ArchiveRefusal, claim, cursor, envelope, finish, frame, name, posture,
    read_capsule,
};

/// Read a historical reduction under independently supplied byte and roster limits.
///
/// # Errors
///
/// Refuses malformed envelopes, excess bounds and contradictory counts or participant ceilings.
/// Internal consistency does not authenticate a producer or establish execution.
pub fn read_reduction(
    encoded: &[u8],
    limits: ReductionArchiveLimits,
) -> Result<ArchivedReduction, ReductionArchiveRefusal> {
    let bytes = limits.bytes();
    let (address, mut reader) = envelope(encoded, REDUCTION_ARCHIVE_TAG, 4, bytes)?;
    let capsule = read_capsule(frame(&mut reader, bytes)?, bytes)?;
    let report_posture = posture(reader.byte()?)?;
    let probe_revision = claim(&mut reader, bytes)?;
    let probe_posture = posture(reader.byte()?)?;
    let budget = ReductionBudget::declared(reader.u32()?);
    if budget.probes() == 0 {
        return Err(ReductionArchiveRefusal::ZeroBudget);
    }
    let count = reader.count()?;
    if count > limits.reducers() {
        return Err(ReductionArchiveRefusal::TooManyReducers);
    }
    let mut semantic_reducers = Vec::new();
    for _ in 0..count {
        semantic_reducers.push(semantic(frame(&mut reader, bytes)?, bytes)?);
    }
    let byte_reducer = match reader.byte()? {
        0 => ByteReducerExecution::Executed(ByteReducerId::ChunkRemovalAndZeroing),
        1 => ByteReducerExecution::NotReachedBecauseBudgetSpent,
        _ => return Err(ArchiveRefusal::InvalidSlot.into()),
    };
    let census = census(&mut reader)?;
    let halt = match reader.byte()? {
        0 => ReductionHalt::FixedPointReached,
        1 => ReductionHalt::BudgetExhausted,
        _ => return Err(ArchiveRefusal::InvalidSlot.into()),
    };
    finish(&reader)?;
    account::joined(ArchivedReduction {
        encoded: encoded.to_vec(),
        address,
        capsule,
        report_posture,
        probe_revision,
        probe_posture,
        budget,
        semantic_reducers,
        byte_reducer,
        census,
        halt,
    })
}

fn semantic(
    encoded: &[u8],
    limits: ArchiveLimits,
) -> Result<ArchivedSemanticReducer, ArchiveRefusal> {
    let mut reader = cursor(encoded);
    let record = ArchivedSemanticReducer {
        name: name(&mut reader, limits)?,
        revision: claim(&mut reader, limits)?,
        posture: posture(reader.byte()?)?,
        candidates: reader.u64()?,
        probes: reader.u64()?,
    };
    finish(&reader)?;
    Ok(record)
}

fn census(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
) -> Result<ArchivedReductionCensus, ReductionArchiveRefusal> {
    let accepted = reader.u32()?;
    let fingerprint_moved = reader.u32()?;
    let no_failure = reader.u32()?;
    let probes = accepted
        .checked_add(fingerprint_moved)
        .and_then(|count| count.checked_add(no_failure))
        .ok_or(ReductionArchiveRefusal::AccountingMismatch)?;
    Ok(ArchivedReductionCensus {
        accepted,
        fingerprint_moved,
        no_failure,
        probes,
    })
}
