use super::input::{environment, text};
use macroonz::harness::oracle::RelativeSourcePath;
use macroonz::native_compiler::{CompilerRequest, DependencyLimits};
use macroonz::native_process::{ProcessLimits, ProcessTool};
use macroonz::native_publication::{
    AuthoredFile, BakeCommand, BakeFormatter, BakeGeneration, BakePreparation, BakeToolBudget,
    DestinationLimits, PublicationDestination, PublicationLimits, PublicationPath,
};
use serde_json::Value;
use std::path::PathBuf;
use std::time::Duration;

pub(super) fn read() -> Result<(BakeCommand, u64), String> {
    let configuration = super::input::read()?;
    let action = text(&configuration, "action")?;
    if action == "recover" {
        return Ok((BakeCommand::Recover(destination(&configuration)?), 0));
    }
    let value = configuration
        .get("value")
        .and_then(Value::as_u64)
        .ok_or("value must be a u64")?;
    if action == "prepare" {
        return Ok((BakeCommand::Prepare, value));
    }
    let preparation = preparation(&configuration)?;
    let command = match action {
        "inspect" => BakeCommand::Inspect(Box::new(preparation)),
        "check" => BakeCommand::Check {
            preparation: Box::new(preparation),
            destination: destination(&configuration)?,
        },
        "generate" => generate(&configuration, preparation)?,
        _ => return Err("action must be prepare, inspect, check, generate or recover".to_owned()),
    };
    Ok((command, value))
}

fn destination(configuration: &Value) -> Result<PublicationDestination, String> {
    PublicationDestination::open(
        &PathBuf::from(text(configuration, "destination")?),
        DestinationLimits {
            files: 8,
            bytes: 65_536,
            metadata: 16_384,
        },
    )
    .map_err(|error| error.to_string())
}

fn preparation(configuration: &Value) -> Result<BakePreparation, String> {
    let formatter = match configuration.get("rustfmt") {
        None | Some(Value::Null) => None,
        Some(Value::String(_)) => Some(BakeFormatter {
            tool: tool(configuration, "rustfmt")?,
            configuration: PathBuf::from(text(configuration, "format_configuration")?),
        }),
        Some(_) => return Err("rustfmt must be an absolute executable path or null".to_owned()),
    };
    Ok(BakePreparation {
        workspace: PathBuf::from(text(configuration, "workspace")?),
        formatter,
        output: PublicationLimits {
            files: 1,
            bytes: 65_536,
        },
        tools: BakeToolBudget {
            runs: 3,
            time: Duration::from_secs(195),
            capture_bytes: 6_291_456,
        },
    })
}

fn tool(configuration: &Value, key: &str) -> Result<ProcessTool, String> {
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

fn generate(configuration: &Value, preparation: BakePreparation) -> Result<BakeCommand, String> {
    let compiler = CompilerRequest::rustc(
        &tool(configuration, "rustc")?,
        RelativeSourcePath::informed("fixture.rs").map_err(|error| format!("{error:?}"))?,
        preparation
            .workspace
            .join(format!("published-value{}", std::env::consts::EXE_SUFFIX)),
        text(configuration, "target")?,
    )
    .and_then(|request| {
        request.with_dependencies(
            preparation.workspace.join("published-value.d"),
            DependencyLimits {
                files: 8,
                bytes: 65_536,
            },
        )
    })
    .map_err(|error| error.to_string())?;
    let driver = b"mod value;\nuse std::io::Write;\nfn main() -> std::io::Result<()> { writeln!(std::io::stdout(), \"{}\", value::VALUE) }\n";
    Ok(BakeCommand::Generate(Box::new(BakeGeneration {
        preparation,
        compiler,
        authored: vec![AuthoredFile {
            path: PublicationPath::informed("fixture.rs").map_err(|error| error.to_string())?,
            bytes: driver.to_vec(),
        }],
        source_limits: PublicationLimits {
            files: 2,
            bytes: 65_536,
        },
        destination: destination(configuration)?,
    })))
}
