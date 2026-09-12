use super::types::ExecutionContext;
use super::{MutationError, MutationOutput, MutationRequest, MutationRun, PendingMutation};
use crate::harness::muterprater::{FamilyLookup, OwnerLookup};
use crate::native_process::{self, ProcessRun};

/// Executes the selected mutation profile with original source and process custody.
///
/// # Errors
/// Refuses unsupported versions, source capture failures, existing output directories and native startup failures.
pub fn run(
    request: MutationRequest,
    owner: OwnerLookup,
    family: FamilyLookup,
) -> Result<MutationRun, MutationError> {
    let context = super::prepare::context(request, owner, family)?;
    std::fs::create_dir(&context.request.output).map_err(MutationError::Filesystem)?;
    let process =
        native_process::run(&context.request.process, None).map_err(MutationError::Process)?;
    Ok(finish(context, process))
}

pub(super) fn finish(context: ExecutionContext, run: ProcessRun) -> MutationRun {
    match run {
        ProcessRun::Pending(process) => {
            MutationRun::Pending(Box::new(PendingMutation { context, process }))
        }
        ProcessRun::Finished(process) => {
            let observation = super::observe::observed(&context, &process);
            MutationRun::Finished(Box::new(MutationOutput {
                context,
                process,
                observation,
            }))
        }
    }
}
