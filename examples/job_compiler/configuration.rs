use super::input::{environment, text};
use super::types::Settings;
use macroonz::configuration::v1;
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::native_compiler::{CargoFixture, CargoTarget, CompilerRequest};
use macroonz::native_process::ProcessTool;
use serde_json::Value;
use std::path::PathBuf;

pub(super) fn read() -> Result<Settings, String> {
    let configuration = super::input::read()?;
    let tool = v1::process_tool(
        PathBuf::from(text(&configuration, "cargo")?),
        PathBuf::from(text(&configuration, "directory")?),
        environment(&configuration)?,
        &[],
    )
    .map_err(|error| error.to_string())?;
    Ok(Settings {
        lawful: request(&configuration, &tool, "lawful-state")?,
        hostile: request(&configuration, &tool, "wrong-state")?,
    })
}

fn request(
    configuration: &Value,
    tool: &ProcessTool,
    binary: &str,
) -> Result<CompilerRequest, String> {
    let fixture = CargoFixture::informed(
        PathBuf::from(text(configuration, "manifest")?),
        PathBuf::from(text(configuration, "target_directory")?),
        text(configuration, "package")?.to_owned(),
        CargoTarget::Binary(binary.to_owned()),
        text(configuration, "target")?.to_owned(),
    )
    .map_err(|error| error.to_string())?;
    let locus = RelativeSourcePath::informed(&format!("fixtures/{binary}.rs"))
        .map_err(|error| format!("source: {error:?}"))?;
    CompilerRequest::cargo(tool, fixture, locus).map_err(|error| error.to_string())
}
