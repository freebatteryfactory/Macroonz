//! A historical witness enters the existing input and trial execution roads once.

use super::{Invocation, ReplayedTrial, TrialBinding, run_one};
use crate::input::{BoundInput, InputBinding, InputLimits, InputRefusal, pack};
use crate::report::archive::ArchivedCapsule;

/// Execute saved witness bytes through independently supplied current bindings.
///
/// The complete contract is owned by the [saved-witness road](super#saved-witness-replay).
///
/// # Errors
///
/// Preserves input admission or decoding refusal before executing a trial.
pub fn replay<Input>(
    historical: &ArchivedCapsule,
    binding: &TrialBinding<BoundInput<Input>>,
    decoder: &InputBinding<Input>,
    invocation: Invocation,
    limits: InputLimits,
) -> Result<ReplayedTrial, InputRefusal> {
    let envelope = pack(decoder.profile(), historical.input(), limits)?;
    let invocation = invocation.with_input(decoder.decode(envelope)?);
    let report = run_one(binding, &invocation);
    Ok(ReplayedTrial::earned(
        historical,
        report,
        invocation.input().envelope().clone(),
    ))
}
