use super::configure::configuration;
use super::fixture::LIMITS;
use super::install_fixture::destination;
use super::stage_fixture::{DRIVER, authored, request};
use crate::compiler::configure::{bounds, tool};
use crate::compiler::types::Host;
use macroonz::native_process::ProcessLimits;
use macroonz::native_publication::{
    BakeCommand, BakeFormatter, BakeGeneration, BakePreparation, BakeToolBudget,
};
use std::path::Path;
use std::time::Duration;

pub(super) fn preparation(
    source: &Path,
    work: &Path,
    host: &Host,
) -> Result<BakePreparation, String> {
    let rustfmt = host
        .rustc
        .parent()
        .ok_or("toolchain parent absent")?
        .join(format!("rustfmt{}", std::env::consts::EXE_SUFFIX));
    Ok(BakePreparation {
        workspace: work.to_path_buf(),
        formatter: Some(BakeFormatter {
            tool: tool(host, &rustfmt, source, bounds()?)?,
            configuration: configuration(source)?,
        }),
        output: LIMITS,
        tools: BakeToolBudget {
            runs: 6,
            time: Duration::from_secs(390),
            capture_bytes: 12_582_912,
        },
    })
}

pub(super) fn generate(
    preparation: BakePreparation,
    source: &Path,
    output: &Path,
    host: &Host,
    limits: ProcessLimits,
) -> Result<BakeCommand, String> {
    Ok(BakeCommand::Generate(Box::new(BakeGeneration {
        preparation,
        authored: vec![authored("fixture.rs", DRIVER.as_bytes())?],
        compiler: request(
            host,
            &host.rustc,
            source,
            &source.join(format!("baked{}", std::env::consts::EXE_SUFFIX)),
            limits,
        )?,
        source_limits: LIMITS,
        destination: destination(output)?,
    })))
}
