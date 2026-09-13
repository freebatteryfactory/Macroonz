use super::types::Workspace;
use super::{
    BakeCause, BakeCommand, BakeError, BakeGeneration, BakeOutput, BakePreparation, budget,
    prepare, workspace,
};
use crate::compiler::Kind;
use crate::native_publication::{
    CompiledPublication, DestinationError, PreparedPublication, Publication,
    PublicationDestination, StagingPlan, StagingRun,
};

/// Executes one explicitly selected command through the caller-registered generation operation and existing native owners.
///
/// # Errors
/// Retains the caller's typed refusal or the actual native owner cause and any unfinished cleanup custody.
pub fn bake<K: Kind, E>(
    command: BakeCommand,
    operation: impl FnOnce() -> Result<Publication<K>, E>,
) -> Result<BakeOutput<K>, BakeError<K, E>> {
    if let BakeCommand::Recover(destination) = command {
        return recover(&destination)
            .map(|()| BakeOutput::Recovered)
            .map_err(|error| {
                failure(BakeCause::Destination {
                    error,
                    compiled: None,
                })
            });
    }
    let publication = operation().map_err(|error| failure(BakeCause::Declaration(error)))?;
    if let BakeCommand::Prepare = command {
        return Ok(BakeOutput::Declared(publication));
    }
    let preparation = match &command {
        BakeCommand::Inspect(preparation) | BakeCommand::Check { preparation, .. } => {
            preparation.as_ref()
        }
        BakeCommand::Generate(selection) => &selection.preparation,
        BakeCommand::Prepare | BakeCommand::Recover(_) => {
            return Err(failure(BakeCause::Configuration(
                "command selection disagrees".to_owned(),
            )));
        }
    };
    admit(&publication, preparation, &command).map_err(|cause| BakeError { cause, lease: None })?;
    let workspace = workspace::open(&preparation.workspace)
        .map_err(|error| failure(BakeCause::Storage(error)))?;
    let result = prepared_action(command, publication, &workspace);
    result.map_err(|cause| {
        let lease = if cause.is_pending() {
            Some(workspace.lease)
        } else {
            None
        };
        BakeError { cause, lease }
    })
}

fn failure<K: Kind, E>(cause: BakeCause<K, E>) -> BakeError<K, E> {
    BakeError {
        cause: Box::new(cause),
        lease: None,
    }
}

fn admit<K: Kind, E>(
    publication: &Publication<K>,
    preparation: &BakePreparation,
    command: &BakeCommand,
) -> Result<(), Box<BakeCause<K, E>>> {
    drop(
        crate::native_publication::inventory::bounded_paths(
            publication
                .files()
                .map(|file| (file.path().clone(), file.source().len())),
            preparation.output,
        )
        .map_err(|error| Box::new(BakeCause::Inventory(error)))?,
    );
    let compiler = match command {
        BakeCommand::Generate(selection) => Some(selection.compiler.process().limits()),
        BakeCommand::Prepare
        | BakeCommand::Inspect(_)
        | BakeCommand::Check { .. }
        | BakeCommand::Recover(_) => None,
    };
    budget::admit(preparation, publication.files().count(), compiler)
        .map_err(|error| Box::new(BakeCause::Configuration(error)))?;
    let destination = match command {
        BakeCommand::Check { destination, .. } => Some(destination),
        BakeCommand::Generate(selection) => Some(&selection.destination),
        BakeCommand::Prepare | BakeCommand::Inspect(_) | BakeCommand::Recover(_) => None,
    };
    if let Some(destination) = destination {
        destination
            .excludes_workspace(&preparation.workspace)
            .map_err(|error| {
                Box::new(BakeCause::Destination {
                    error,
                    compiled: None,
                })
            })?;
    }
    if let BakeCommand::Generate(selection) = command {
        for path in selection
            .compiler
            .output_root()
            .into_iter()
            .chain(selection.compiler.artifact_file())
            .chain(selection.compiler.dependency_file())
        {
            selection
                .destination
                .excludes_output(path)
                .map_err(|error| {
                    Box::new(BakeCause::Destination {
                        error,
                        compiled: None,
                    })
                })?;
        }
    }
    Ok(())
}

fn prepared_action<K: Kind, E>(
    command: BakeCommand,
    publication: Publication<K>,
    workspace: &Workspace,
) -> Result<BakeOutput<K>, Box<BakeCause<K, E>>> {
    match command {
        BakeCommand::Inspect(selection) => {
            prepare::run(publication, &selection, workspace).map(BakeOutput::Prepared)
        }
        BakeCommand::Check {
            preparation,
            destination,
        } => {
            let prepared = prepare::run(publication, &preparation, workspace)?;
            let comparison = destination.check(&prepared).map_err(|error| {
                Box::new(BakeCause::Destination {
                    error,
                    compiled: None,
                })
            })?;
            Ok(BakeOutput::Checked {
                prepared,
                comparison,
            })
        }
        BakeCommand::Generate(selection) => {
            let prepared = prepare::run(publication, &selection.preparation, workspace)?;
            generate(*selection, prepared)
        }
        BakeCommand::Prepare | BakeCommand::Recover(_) => Err(Box::new(BakeCause::Configuration(
            "unexpected prepared command".to_owned(),
        ))),
    }
}

fn generate<K: Kind, E>(
    selection: BakeGeneration,
    prepared: PreparedPublication<K>,
) -> Result<BakeOutput<K>, Box<BakeCause<K, E>>> {
    let stage = StagingPlan::declared(prepared, selection.authored, selection.source_limits)
        .and_then(|plan| plan.cached(&selection.preparation.workspace))
        .map_err(|error| Box::new(BakeCause::Staging(error)))?;
    let request = selection
        .compiler
        .relocated(stage.path().to_path_buf())
        .map_err(|error| {
            Box::new(BakeCause::Staging(
                crate::native_publication::StagingError::Compiler(error),
            ))
        })?;
    let run = stage
        .compile(&request)
        .map_err(|error| Box::new(BakeCause::Staging(error)))?;
    let StagingRun::Compiled(compiled) = run else {
        return Err(Box::new(BakeCause::Compilation(run)));
    };
    if let Err(error) = install(&selection.destination, &compiled) {
        return Err(Box::new(BakeCause::Destination {
            error,
            compiled: Some(compiled),
        }));
    }
    Ok(BakeOutput::Generated(compiled))
}

fn install<K: Kind>(
    destination: &PublicationDestination,
    compiled: &CompiledPublication<K>,
) -> Result<(), DestinationError> {
    let mut installation = destination.begin(compiled)?;
    while installation.write_next()?.is_some() {}
    installation.commit()?;
    if !destination.check(compiled.prepared())?.is_current() {
        return Err(DestinationError::Conflict(
            "destination changed after installation".to_owned(),
        ));
    }
    Ok(())
}

fn recover(destination: &PublicationDestination) -> Result<(), DestinationError> {
    if let Some(mut installation) = destination.recover()? {
        while installation.write_next()?.is_some() {}
        installation.commit()?;
    }
    Ok(())
}
