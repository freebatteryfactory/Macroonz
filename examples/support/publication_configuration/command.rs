use crate::input::text;
use macroonz::native_publication::{
    BakeCommand, DestinationLimits, PublicationDestination, PublicationLimits,
};
use serde_json::Value;
use std::path::PathBuf;

pub(crate) fn command(
    configuration: &Value,
    output: PublicationLimits,
    fixture: &[u8],
    artifact: &str,
) -> Result<BakeCommand, String> {
    let action = text(configuration, "action")?;
    if action == "recover" {
        return Ok(BakeCommand::Recover(destination(configuration)?));
    }
    if action == "prepare" {
        return Ok(BakeCommand::Prepare);
    }
    let preparation = super::prepare::preparation(configuration, output)?;
    match action {
        "inspect" => Ok(BakeCommand::Inspect(Box::new(preparation))),
        "check" => Ok(BakeCommand::Check {
            preparation: Box::new(preparation),
            destination: destination(configuration)?,
        }),
        "generate" => super::generate::generate(configuration, preparation, fixture, artifact),
        _ => Err("action must be prepare, inspect, check, generate or recover".to_owned()),
    }
}

pub(super) fn destination(configuration: &Value) -> Result<PublicationDestination, String> {
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
