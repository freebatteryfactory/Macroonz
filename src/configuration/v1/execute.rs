//! Input-bound execution through the existing root workflow.

use crate::harness::input::{BoundInput, InputBinding, InputLimits, InputRefusal};
use crate::harness::runner::{Invocation, SelectionPlan, TrialTableView};
use crate::workflow::{self, InputRun};

/// The version-one envelope and payload ceilings.
#[must_use]
pub const fn input_limits() -> InputLimits {
    InputLimits::declared(2_097_152, 1_048_576)
}

/// Executes a caller-bound table and specimen with version-one input ceilings.
///
/// # Errors
/// Preserves input admission and decoder refusals before execution.
pub fn run<Input>(
    table: &TrialTableView<'_, BoundInput<Input>>,
    selection: &SelectionPlan,
    decoder: &InputBinding<Input>,
    payload: &[u8],
    invocation: Invocation,
) -> Result<InputRun, InputRefusal> {
    workflow::run(
        table,
        selection,
        decoder,
        payload,
        invocation,
        input_limits(),
    )
}
