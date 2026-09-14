use super::input::{environment, text};
use super::types::Settings;
use macroonz::configuration::v1;
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::harness::report::TargetTriple;
use macroonz::native_compiler::{CargoFixture, CargoTarget, CompilerRequest};
use std::path::{Path, PathBuf};

pub(super) fn read() -> Result<Settings, String> {
    let configuration = super::input::read()?;
    let directory = PathBuf::from(text(&configuration, "directory")?);
    let environment = environment(&configuration)?;
    let cargo = v1::process_tool(
        PathBuf::from(text(&configuration, "cargo")?),
        directory.clone(),
        environment.clone(),
        &[],
    )
    .map_err(debug)?;
    let tool = v1::process_tool(
        PathBuf::from(text(&configuration, "rustc")?),
        directory.clone(),
        environment,
        &[],
    )
    .map_err(debug)?;
    let target = text(&configuration, "target")?;
    let fixture = CargoFixture::informed(
        PathBuf::from(text(&configuration, "manifest")?),
        PathBuf::from(text(&configuration, "target_directory")?),
        text(&configuration, "package")?.to_owned(),
        CargoTarget::Binary("coverage-record".to_owned()),
        target.to_owned(),
    )
    .map_err(debug)?;
    let locus = RelativeSourcePath::informed("fixtures/coverage-record.rs").map_err(debug)?;
    let compiler = CompilerRequest::cargo(&cargo, fixture, locus)
        .and_then(CompilerRequest::instrumented_target)
        .map_err(debug)?;
    Ok(Settings {
        compiler,
        tool,
        scratch: PathBuf::from(text(&configuration, "scratch")?),
        target: TargetTriple::declared(target),
        campaign: super::campaign::declared(
            &directory,
            Path::new(text(&configuration, "declaration")?),
        )?,
    })
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
