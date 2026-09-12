use super::types::{
    CompilerError, CompilerOutput, CompilerRequest, CompilerRun, PendingCompilation,
};
use crate::native_process::ProcessRun;

/// Runs the selected compiler through bounded native supervision and establishes its observation.
///
/// # Errors
/// Returns native startup failure before a compiler observation can be established.
pub fn compile(request: &CompilerRequest) -> Result<CompilerRun, CompilerError> {
    let process =
        crate::native_process::run(request.process(), None).map_err(CompilerError::Process)?;
    Ok(finish(request.clone(), process))
}

pub(super) fn finish(request: CompilerRequest, process: ProcessRun) -> CompilerRun {
    match process {
        ProcessRun::Finished(process) => {
            let observation = super::diagnose::observe(&request, &process);
            CompilerRun::Finished(Box::new(CompilerOutput {
                request,
                process,
                observation,
            }))
        }
        ProcessRun::Pending(process) => {
            CompilerRun::Pending(Box::new(PendingCompilation { request, process }))
        }
    }
}
