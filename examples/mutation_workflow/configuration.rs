use super::input::{environment, text};
use super::types::{Operation, Settings};
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::native_mutation::{MutationRequest, MutationSources, MutationToolchain};
use macroonz::native_process::{ProcessLimits, ProcessTool};
use macroonz::native_storage::StorageName;
use std::path::PathBuf;
use std::time::Duration;

pub(super) const SOURCE_BYTES: usize = 1_048_576;

pub(super) fn read() -> Result<Settings, String> {
    let configuration = super::input::read()?;
    let directory = PathBuf::from(text(&configuration, "directory")?);
    if !directory.is_absolute() {
        return Err("directory must be absolute".to_owned());
    }
    let action = configuration
        .get("action")
        .map_or(Some("run"), serde_json::Value::as_str)
        .ok_or("action must be run or compare")?;
    let operation = match action {
        "run" => Operation::Execute(Box::new(request(&configuration, &directory)?)),
        "compare" => Operation::Compare,
        _ => return Err("action must be run or compare".to_owned()),
    };
    let storage = PathBuf::from(text(&configuration, "storage")?);
    if !storage.is_absolute() {
        return Err("storage must be absolute".to_owned());
    }
    Ok(Settings {
        operation,
        directory,
        storage,
        batch: StorageName::informed(text(&configuration, "batch")?).map_err(debug)?,
    })
}

fn request(
    configuration: &serde_json::Value,
    directory: &std::path::Path,
) -> Result<MutationRequest, String> {
    let tool = ProcessTool::informed(
        PathBuf::from(text(configuration, "backend")?),
        directory.to_path_buf(),
        environment(configuration)?,
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
        PathBuf::from(text(configuration, "cargo")?),
        PathBuf::from(text(configuration, "rustc")?),
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
    MutationRequest::cargo_mutants(
        &tool,
        toolchain,
        MutationSources::declared(sources, SOURCE_BYTES).map_err(debug)?,
        PathBuf::from(text(configuration, "output")?),
        &PathBuf::from(text(configuration, "target_directory")?),
        text(configuration, "target")?,
    )
    .map_err(debug)
}

fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}
