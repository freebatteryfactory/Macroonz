//! The identity joins this home owns: the trial one descriptor row declares,
//! the key one execution of it is looked up under, and the revision identity of
//! the row itself.
//!
//! The runner is the one home that holds a descriptor value and a record
//! vocabulary at the same time, so the joins live here. Nothing is derived
//! twice: every identity is minted by the record home's own road, over material
//! the descriptor home owns, and this file adds only the pairing.

use super::types::{Invocation, TrialBinding};
use crate::descriptor::Row;
use crate::report::{
    CheckRevisionId, ExecutionKey, RowRevisionId, SubjectRevisionId, TrialId, TrialProfile,
};

/// The semantic identity of the trial one row declares.
///
/// # Authority
///
/// The four semantic references come from the row's own trial key, so the
/// execution suite, the roles, and the tags stay out of what a trial MEANS. The
/// profile coordinate is [`TrialProfile::Unprofiled`], the sole lawful value
/// and an honest statement about every trial the harness runs: nothing splits
/// trials by feature profile, so no trial carries one. The first real feature
/// split gives that coordinate an owner on the row, and this join reads it from
/// there instead of naming it.
#[must_use]
pub fn trial_identity(row: &Row) -> TrialId {
    TrialId::of_key(row.trial_key(), TrialProfile::Unprofiled)
}

/// The key one execution of one bound trial is looked up under.
///
/// # Authority
///
/// The parts are the trial's semantic identity, the attachment's two revision
/// bindings under the record vocabulary's names for them, the invocation's
/// profile, and the target binding the caller declared — the last
/// unconditionally, because the key's own constructor admits no shape without
/// it. What a hit under this key is worth is the attachment's posture question,
/// answered by the record home's one statement
/// ([`attachment_cache_eligibility`](crate::report::attachment_cache_eligibility)).
///
/// # Nonclaims
///
/// Holding a key is not holding a cached result. This engine looks nothing up:
/// it executes what the selection admitted, and a caller that keeps results
/// across runs is the party that decides what a matching key buys.
#[must_use]
pub fn execution_key(binding: &TrialBinding, invocation: &Invocation) -> ExecutionKey {
    let attachment = binding.attachment();
    ExecutionKey::over(
        trial_identity(binding.row()),
        SubjectRevisionId::of_binding(attachment.subject_revision()),
        CheckRevisionId::of_binding(attachment.check_revision()),
        invocation.profile(),
        invocation.target().clone(),
    )
}

/// The revision identity of one authored row.
///
/// # Authority
///
/// Total, and the join is the whole of it. The bytes are the descriptor home's
/// and the row already carries them — written once, where the row was born — so
/// this engine neither encodes a row it does not own nor asks whether an
/// encoding succeeded that already did. A row nothing could encode is a row that
/// was never constructed, and no census entry can be stated over one.
#[must_use]
pub(super) fn row_revision(row: &Row) -> RowRevisionId {
    RowRevisionId::over(row.canonical_bytes())
}
