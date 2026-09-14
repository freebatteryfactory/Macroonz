use crate::input::text;
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::native_compiler::{CompilerRequest, DependencyLimits};
use macroonz::native_publication::{
    AuthoredFile, BakeCommand, BakeGeneration, BakePreparation, PublicationLimits, PublicationPath,
};
use serde_json::Value;

pub(super) fn generate(
    configuration: &Value,
    preparation: BakePreparation,
    fixture: &[u8],
    artifact: &str,
) -> Result<BakeCommand, String> {
    let compiler = CompilerRequest::rustc(
        &super::prepare::tool(configuration, "rustc")?,
        RelativeSourcePath::informed("fixture.rs").map_err(|error| format!("{error:?}"))?,
        preparation
            .workspace
            .join(format!("{artifact}{}", std::env::consts::EXE_SUFFIX)),
        text(configuration, "target")?,
    )
    .and_then(|request| {
        request.with_dependencies(
            preparation.workspace.join(format!("{artifact}.d")),
            DependencyLimits {
                files: 8,
                bytes: 65_536,
            },
        )
    })
    .map_err(|error| error.to_string())?;
    let files = preparation
        .output
        .files
        .checked_add(1)
        .ok_or("staged file count overflow")?;
    let source_limits = PublicationLimits {
        files,
        bytes: preparation.output.bytes,
    };
    Ok(BakeCommand::Generate(Box::new(BakeGeneration {
        preparation,
        compiler,
        authored: vec![AuthoredFile {
            path: PublicationPath::informed("fixture.rs").map_err(|error| error.to_string())?,
            bytes: fixture.to_vec(),
        }],
        source_limits,
        destination: super::command::destination(configuration)?,
    })))
}
