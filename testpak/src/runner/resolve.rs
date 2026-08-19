//! The identity joins this home owns: the trial one descriptor row declares,
//! the key one execution of it is looked up under, and the revision identity of
//! the row itself.
//!
//! The runner is the one home that holds a descriptor value and a record
//! vocabulary at the same time, so the joins live here. Nothing is derived
//! twice: every identity is minted by the record home's own road, over material
//! the descriptor home owns, and this file adds only the pairing.

use super::types::{Invocation, TrialBinding};
use crate::descriptor::encode::encode_row;
use crate::descriptor::{EncodeRefusal, Row};
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
/// The bytes are the descriptor home's, taken from that home's own encoder: a
/// row's canonical byte string has one author, and this engine derives an
/// identity from it rather than encoding a row it does not own.
///
/// # Errors
///
/// Refuses when that encoder refuses — a length past the width the row encoding
/// declares, which is unreachable on every target this crate is built for. It is
/// carried rather than swallowed because there is no honest stand-in: a census
/// entry under an identity derived from bytes nobody wrote would be two rows'
/// bookkeeping sharing one name, which is the exact thing a revision identity
/// exists to prevent.
pub(super) fn row_revision(row: &Row) -> Result<RowRevisionId, EncodeRefusal> {
    Ok(RowRevisionId::over(&encode_row(row)?))
}
