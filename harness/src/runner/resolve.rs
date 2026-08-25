//! The identity joins this home owns: the trial one row declares, the key one execution of it is looked up under, and the revision identity of the row itself.
//!
//! The runner is the one home holding a descriptor value and the record vocabulary at the same time, so the joins live here.
//! Nothing is derived twice: every identity is minted by the record home's own road over material the descriptor home owns, and this file adds the pairing.

use super::types::{Invocation, TrialBinding};
use crate::descriptor::Row;
use crate::report::{
    CheckRevisionId, ExecutionKey, ExecutionRevisions, RowRevisionId, SubjectRevisionId, TrialId,
    TrialProfile,
};

/// The semantic identity of the trial one row declares.
///
/// The four semantic references come from the row's own trial key, so the execution suite, the roles, and the tags stay out of what a trial means.
/// The profile coordinate is [`TrialProfile::Unprofiled`]: nothing splits trials by feature profile, so no trial carries one.
#[must_use]
pub fn trial_identity(row: &Row) -> TrialId {
    TrialId::of_key(row.trial_key(), TrialProfile::Unprofiled)
}

/// The key one execution of one bound trial is looked up under.
///
/// The parts are the trial's semantic identity, the attachment's two revision bindings, the invocation's profile, and the declared target binding — the last unconditionally, because the key's own constructor admits no shape without it.
/// What a hit under this key is worth is the attachment's posture question, answered by [`attachment_cache_eligibility`](crate::report::attachment_cache_eligibility).
///
/// Holding a key is not holding a cached result.
/// This engine looks nothing up: it executes what the selection admitted, and a caller that keeps results across runs decides what a matching key buys.
#[must_use]
pub fn execution_key(binding: &TrialBinding, invocation: &Invocation) -> ExecutionKey {
    let revisions = execution_revisions(binding);
    ExecutionKey::over(
        trial_identity(binding.row()),
        revisions.subject(),
        revisions.check(),
        invocation.profile(),
        invocation.target().clone(),
    )
}

/// The exact subject and check revision standing one binding declares.
///
/// One join shared by the execution key and the complete-table accounting, so selected and unselected rows read the same revision relationship.
#[must_use]
pub(super) fn execution_revisions(binding: &TrialBinding) -> ExecutionRevisions {
    let attachment = binding.attachment();
    ExecutionRevisions::bound(
        SubjectRevisionId::of_binding(attachment.subject_revision()),
        CheckRevisionId::of_binding(attachment.check_revision()),
    )
}

/// The revision identity of one authored row.
///
/// Total, and the join is the whole of it.
/// The row carries its canonical bytes from the moment it was born, so this engine neither encodes a row it does not own nor asks whether an encoding succeeded that already did.
#[must_use]
pub(super) fn row_revision(row: &Row) -> RowRevisionId {
    RowRevisionId::over(row.canonical_bytes())
}
