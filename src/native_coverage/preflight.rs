use super::{NativeCoverage, NativeCoverageFailure, NativeCoverageProcessError};
use crate::harness::fuzz::{
    CoverageCommand, CoverageHostFailure, PreflightIncomplete, RustcCommand, RustcProfileRequest,
};
use crate::native_process::{ProcessError, ProcessLimits, ProcessTool};

/// Establishes compiler and matching LLVM readiness through bounded native process execution.
///
/// # Errors
/// Refuses mismatched declared compilers and retains native or owning preflight failures.
pub fn preflight(
    request: RustcProfileRequest,
    tool: ProcessTool,
    target_limits: ProcessLimits,
) -> Result<NativeCoverage, NativeCoverageFailure<PreflightIncomplete>> {
    if request.rustc() != tool.executable() {
        return Err(super::cleanup::retain(CoverageHostFailure::Executor {
            operation: CoverageCommand::Rustc(RustcCommand::VerboseVersion),
            error: NativeCoverageProcessError::Invalid(ProcessError::InvalidRequest(
                "coverage request and process policy select different compilers".to_owned(),
            )),
            cleanup: None,
        }));
    }
    let ready = crate::harness::fuzz::preflight_ready_with(request, |invocation| {
        super::execute::supervised(&tool, target_limits, invocation)
    })
    .map_err(super::cleanup::retain)?;
    Ok(NativeCoverage {
        ready,
        tool,
        target_limits,
    })
}
