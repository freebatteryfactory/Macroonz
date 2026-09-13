use super::types::Workspace;
use super::{BakeCause, BakePreparation, workspace};
use crate::compiler::Kind;
use crate::native_publication::{FormatRun, Formatter, PreparedPublication, Publication};

pub(super) fn run<K: Kind, E>(
    publication: Publication<K>,
    selection: &BakePreparation,
    workspace: &Workspace,
) -> Result<PreparedPublication<K>, Box<BakeCause<K, E>>> {
    let Some(formatter) = &selection.formatter else {
        return Ok(PreparedPublication::unformatted(publication));
    };
    let formatter = Formatter::qualified(&formatter.tool, formatter.configuration.clone())
        .map_err(|error| Box::new(BakeCause::Formatter(error)))?;
    let mut outputs = Vec::new();
    for file in publication.files() {
        let input =
            workspace::input(workspace).map_err(|error| Box::new(BakeCause::Filesystem(error)))?;
        let run = formatter
            .format(&file, input)
            .map_err(|error| Box::new(BakeCause::Formatter(error)))?;
        match run {
            FormatRun::Finished(output) if output.source().is_ok() => outputs.push(*output),
            stopped @ (FormatRun::Finished(_) | FormatRun::Pending(_)) => {
                return Err(Box::new(BakeCause::Formatting {
                    completed: outputs,
                    run: stopped,
                }));
            }
        }
    }
    PreparedPublication::formatted(publication, outputs, selection.output.bytes)
        .map_err(|error| Box::new(BakeCause::Preparation(error)))
}
