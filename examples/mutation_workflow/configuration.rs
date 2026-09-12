use super::input::{environment, text};
use super::types::Settings;
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::native_mutation::{MutationRequest, MutationSources, MutationToolchain};
use macroonz::native_process::{ProcessLimits, ProcessTool};
use macroonz::native_storage::StorageName;
use std::path::PathBuf;
use std::time::Duration;

pub(super) fn read() -> Result<Settings, String> {
    let configuration = super::input::read()?;
    let directory = PathBuf::from(text(&configuration, "directory")?);
    let tool = ProcessTool::informed(
        PathBuf::from(text(&configuration, "backend")?),
        directory.clone(),
        environment(&configuration)?,
        ProcessLimits::informed(
            Duration::from_secs(180),
            Duration::from_secs(5),
            1_048_576,
            1_048_576,
        )
        .map_err(debug)?,
        &[],
    )
    .map_err(debug)?;
    let toolchain = MutationToolchain::declared(
        PathBuf::from(text(&configuration, "cargo")?),
        PathBuf::from(text(&configuration, "rustc")?),
    )
    .map_err(debug)?;
    let files = configuration
        .get("sources")
        .and_then(serde_json::Value::as_array)
        .ok_or("sources must be an array")?;
    let mut sources = Vec::new();
    for file in files {
        sources.push(
            RelativeSourcePath::informed(file.as_str().ok_or("source must be text")?)
                .map_err(debug)?,
        );
    }
    let request = MutationRequest::cargo_mutants(
        &tool,
        toolchain,
        MutationSources::declared(sources, 1_048_576).map_err(debug)?,
        PathBuf::from(text(&configuration, "output")?),
        &PathBuf::from(text(&configuration, "target_directory")?),
        text(&configuration, "target")?,
    )
    .map_err(debug)?;
    Ok(Settings {
        request,
        directory,
        storage: PathBuf::from(text(&configuration, "storage")?),
        batch: StorageName::informed(text(&configuration, "batch")?).map_err(debug)?,
    })
}

fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
