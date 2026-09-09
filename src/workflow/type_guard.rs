//! The original specimen and complete accounting remain joined across the root entrance.

use super::InputRun;
use crate::harness::input::{self, BoundInput, InputBinding, InputLimits, InputRefusal};
use crate::harness::runner::{self, Invocation, SelectionPlan, TrialTableView};

#[cfg(feature = "native-tooling")]
#[path = "guard_storage.rs"]
mod storage;

/// Bind one declared specimen and execute its selection over the complete supplied table.
///
/// # Errors
/// Preserves envelope and decoder refusals before subject execution.
pub fn run<Input>(
    table: &TrialTableView<'_, BoundInput<Input>>,
    selection: &SelectionPlan,
    decoder: &InputBinding<Input>,
    payload: &[u8],
    invocation: Invocation,
    limits: InputLimits,
) -> Result<InputRun, InputRefusal> {
    let envelope = input::pack(decoder.profile(), payload, limits)?;
    let invocation = invocation.with_input(decoder.decode(envelope)?);
    let report = runner::run_all(table, selection, &invocation);
    Ok(InputRun {
        report,
        input: invocation.input().envelope().clone(),
    })
}

impl InputRun {
    /// The runner's complete report, including unselected rows and all attempt distinctions.
    #[must_use]
    pub const fn report(&self) -> &crate::harness::report::RunReport {
        &self.report
    }

    /// The original decoder-admitted specimen, distinct from any reduced witness.
    #[must_use]
    pub const fn input(&self) -> &input::InputEnvelope {
        &self.input
    }
}
