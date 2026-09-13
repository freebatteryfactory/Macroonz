//! Explicit tool admission with the versioned process ceilings.

use crate::native_process::{ProcessError, ProcessLimits, ProcessTool, ResourceControl};
use std::path::PathBuf;
use std::time::Duration;

/// The version-one process deadlines and retained output ceilings.
///
/// # Errors
/// Preserves the process owner's admission refusal on an unrepresentable bound.
pub fn process_limits() -> Result<ProcessLimits, ProcessError> {
    ProcessLimits::informed(
        Duration::from_secs(600),
        Duration::from_secs(5),
        16_777_216,
        4_194_304,
    )
}

/// Admits an explicitly selected tool with version-one process ceilings.
///
/// # Errors
/// Preserves limit, path, environment and unsupported-resource refusals.
pub fn process_tool(
    executable: PathBuf,
    directory: PathBuf,
    environment: Vec<(String, String)>,
    resources: &[ResourceControl],
) -> Result<ProcessTool, ProcessError> {
    ProcessTool::informed(
        executable,
        directory,
        environment,
        process_limits()?,
        resources,
    )
}
