//! Historical reduction encoding embeds the existing complete capsule envelope.

use super::{
    ArchivedReduction, REDUCTION_ARCHIVE_TAG, ReductionArchiveLimits, ReductionArchiveRefusal,
    read_reduction, size,
};
use crate::generate::{
    ByteReducerExecution, ByteReducerId, ReductionEvidence, ReductionHalt,
    SemanticReducerExecution, capture_replay,
};
use crate::identity::{ContentAddress, encode_bytes, encode_length};
use crate::report::ReplayPosture;
use crate::report::archive::retain_capsule;

/// Retain a completed reduction as bounded historical data for caller-owned storage.
///
/// # Errors
///
/// Refuses byte and reducer ceilings before allocating the capsule or encoded members.
pub fn retain_reduction(
    evidence: &ReductionEvidence,
    limits: ReductionArchiveLimits,
) -> Result<ArchivedReduction, ReductionArchiveRefusal> {
    let total = size::encoded_size(evidence, limits)?;
    let capsule = retain_capsule(&capture_replay(evidence), limits.bytes())?;
    let mut body = Vec::with_capacity(total.saturating_sub(32));
    body.extend_from_slice(&1u32.to_be_bytes());
    body.extend_from_slice(&4u32.to_be_bytes());
    body.extend_from_slice(&0u32.to_be_bytes());
    encode_bytes(capsule.encoded(), &mut body);
    body.push(evidence.standing().replay().slot());
    encode_bytes(evidence.probe_revision().revision().as_bytes(), &mut body);
    body.push(
        ReplayPosture::ExactDerived
            .meet_revision(evidence.probe_revision().posture())
            .slot(),
    );
    body.extend_from_slice(&evidence.budget().probes().to_be_bytes());
    encode_length(evidence.semantic_reducers().len(), &mut body);
    for reducer in evidence.semantic_reducers() {
        encode_bytes(&semantic(*reducer), &mut body);
    }
    body.push(match evidence.byte_reducer() {
        ByteReducerExecution::Executed(ByteReducerId::ChunkRemovalAndZeroing) => 0,
        ByteReducerExecution::NotReachedBecauseBudgetSpent => 1,
    });
    let census = evidence.outcome().census();
    for count in [
        census.accepted(),
        census.fingerprint_moved(),
        census.no_failure(),
    ] {
        body.extend_from_slice(&count.to_be_bytes());
    }
    body.push(match evidence.outcome().halt() {
        ReductionHalt::FixedPointReached => 0,
        ReductionHalt::BudgetExhausted => 1,
    });
    let mut encoded = Vec::with_capacity(total);
    encoded.extend_from_slice(ContentAddress::derived(REDUCTION_ARCHIVE_TAG, &body).as_bytes());
    encoded.extend_from_slice(&body);
    read_reduction(&encoded, limits)
}

fn semantic(reducer: SemanticReducerExecution) -> Vec<u8> {
    let mut bytes = Vec::new();
    let name = reducer.reducer().name();
    encode_bytes(name.namespace().written().as_bytes(), &mut bytes);
    encode_bytes(name.stem().written().as_bytes(), &mut bytes);
    encode_bytes(reducer.revision().revision().as_bytes(), &mut bytes);
    bytes.push(
        ReplayPosture::ExactDerived
            .meet_revision(reducer.revision().posture())
            .slot(),
    );
    encode_length(reducer.candidates(), &mut bytes);
    encode_length(reducer.probes(), &mut bytes);
    bytes
}
