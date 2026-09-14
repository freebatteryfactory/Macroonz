use super::input::{environment, text};
use super::types::Settings;
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::native_compiler::CompilerRequest;
use macroonz::native_process::{ProcessLimits, ProcessRequest, ProcessTool};
use serde_json::Value;
use std::path::PathBuf;
use std::time::Duration;

pub(super) fn read() -> Result<Settings, String> {
    let configuration = super::input::read()?;
    let directory = PathBuf::from(text(&configuration, "directory")?);
    let artifact = PathBuf::from(text(&configuration, "artifact")?);
    let environment = environment(&configuration)?;
    let compile_limits = ProcessLimits::informed(
        Duration::from_secs(60),
        Duration::from_secs(5),
        1_048_576,
        1_048_576,
    )
    .map_err(|error| error.to_string())?;
    let tool = ProcessTool::informed(
        PathBuf::from(text(&configuration, "rustc")?),
        directory.clone(),
        environment.clone(),
        compile_limits,
        &[],
    )
    .map_err(|error| error.to_string())?;
    let source = RelativeSourcePath::informed(text(&configuration, "source")?)
        .map_err(|error| format!("source: {error:?}"))?;
    let compiler = CompilerRequest::rustc(
        &tool,
        source,
        artifact.clone(),
        text(&configuration, "target")?,
    )
    .map_err(|error| error.to_string())?;
    let reader_limits = ProcessLimits::informed(
        Duration::from_secs(10),
        Duration::from_secs(5),
        65_536,
        65_536,
    )
    .map_err(|error| error.to_string())?;
    let reader = ProcessRequest::informed(
        artifact,
        directory,
        Vec::new(),
        environment,
        reader_limits,
        &[],
    )
    .map_err(|error| error.to_string())?;
    let expected_count = configuration
        .get("expected_count")
        .and_then(Value::as_u64)
        .ok_or("expected_count must be an independently supplied unsigned integer")?;
    Ok(Settings {
        compiler,
        reader,
        expected_count,
    })
}
