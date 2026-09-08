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
    let (report, witness) =
        execute_saved(historical.input(), binding, decoder, invocation, limits)?;
    Ok(ReplayedTrial::earned(historical, report, witness))
}

/// Execute a sparse source's witness using independently supplied current bindings.
///
/// # Errors
///
/// Preserves missing and null witnesses separately, then preserves the input owner's admission refusal.
pub fn replay_legacy<Input>(
    historical: &crate::report::legacy::LegacyRecord,
    binding: &TrialBinding<BoundInput<Input>>,
    decoder: &InputBinding<Input>,
    invocation: Invocation,
    limits: InputLimits,
) -> Result<super::LegacyReplayedTrial, super::LegacyReplayRefusal> {
    let bytes = match historical.witness() {
        crate::report::legacy::LegacyPresence::Missing => {
            return Err(super::LegacyReplayRefusal::MissingWitness);
        }
        crate::report::legacy::LegacyPresence::Null => {
            return Err(super::LegacyReplayRefusal::NullWitness);
        }
        crate::report::legacy::LegacyPresence::Present(bytes) => bytes,
    };
    let (report, witness) = execute_saved(bytes, binding, decoder, invocation, limits)
        .map_err(super::LegacyReplayRefusal::Input)?;
    Ok(super::LegacyReplayedTrial::earned(
        historical, report, witness,
    ))
}

fn execute_saved<Input>(
    bytes: &[u8],
    binding: &TrialBinding<BoundInput<Input>>,
    decoder: &InputBinding<Input>,
    invocation: Invocation,
    limits: InputLimits,
) -> Result<(crate::report::TrialReport, crate::input::InputEnvelope), InputRefusal> {
    let envelope = pack(decoder.profile(), bytes, limits)?;
    let invocation = invocation.with_input(decoder.decode(envelope)?);
    let report = run_one(binding, &invocation);
    Ok((report, invocation.input().envelope().clone()))
}
