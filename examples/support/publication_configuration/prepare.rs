use crate::input::{environment, text};
use macroonz::native_process::{ProcessLimits, ProcessTool};
use macroonz::native_publication::{
    BakeFormatter, BakePreparation, BakeToolBudget, PublicationLimits,
};
use serde_json::Value;
use std::path::PathBuf;
use std::time::Duration;

pub(super) fn preparation(
    configuration: &Value,
    output: PublicationLimits,
) -> Result<BakePreparation, String> {
    let formatter = match configuration.get("rustfmt") {
        None | Some(Value::Null) => None,
        Some(Value::String(_)) => Some(BakeFormatter {
            tool: tool(configuration, "rustfmt")?,
            configuration: PathBuf::from(text(configuration, "format_configuration")?),
        }),
        Some(_) => return Err("rustfmt must be an absolute executable path or null".to_owned()),
    };
    let runs = output
        .files
        .checked_add(2)
        .ok_or("process count overflow")?;
    let seconds = u64::try_from(runs)
        .map_err(|error| error.to_string())?
        .checked_mul(65)
        .ok_or("process time overflow")?;
    let capture_bytes = runs
        .checked_mul(2_097_152)
        .ok_or("process capture overflow")?;
    Ok(BakePreparation {
        workspace: PathBuf::from(text(configuration, "workspace")?),
        formatter,
        output,
        tools: BakeToolBudget {
            runs,
            time: Duration::from_secs(seconds),
            capture_bytes,
        },
    })
}

pub(super) fn tool(configuration: &Value, key: &str) -> Result<ProcessTool, String> {
    let limits = ProcessLimits::informed(
        Duration::from_secs(60),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(|error| error.to_string())?;
    ProcessTool::informed(
        PathBuf::from(text(configuration, key)?),
        PathBuf::from(text(configuration, "workspace")?),
        environment(configuration)?,
        limits,
        &[],
    )
    .map_err(|error| error.to_string())
}
