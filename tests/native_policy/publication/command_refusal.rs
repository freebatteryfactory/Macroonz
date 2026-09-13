use super::command_fixture::{generate, preparation};
use super::configure::publication;
use super::destination_fixture::snapshot;
use super::install_fixture::{destination, installed, subject};
use crate::compiler::configure::{bounds, host, root};
use macroonz::native_publication::{BakeCause, BakeCommand, BakeOutput, DestinationError, bake};

#[test]
fn pending_formatter_query_retains_command_custody_until_cleanup() -> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let work = root()?;
    let mut preparation = preparation(&source, &work, &host)?;
    let formatter = preparation.formatter.as_mut().ok_or("formatter absent")?;
    let limits = macroonz::native_process::ProcessLimits::informed(
        std::time::Duration::from_secs(30),
        std::time::Duration::ZERO,
        1_048_576,
        1_048_576,
    )
    .map_err(|error| error.to_string())?;
    formatter.tool =
        crate::compiler::configure::tool(&host, formatter.tool.executable(), &source, limits)?;
    let pending = bake(
        BakeCommand::Inspect(Box::new(preparation.clone())),
        publication,
    )
    .err()
    .ok_or("query cleanup was not retained")?;
    assert!(pending.is_pending());
    assert!(
        matches!(pending.cause(), BakeCause::Formatter(macroonz::native_publication::FormatError::Query { run, .. })
        if matches!(run.as_ref(), macroonz::native_process::ProcessRun::Pending(_)))
    );
    let busy = bake(BakeCommand::Inspect(Box::new(preparation)), publication)
        .err()
        .ok_or("pending query released workspace")?;
    assert!(matches!(
        busy.cause(),
        BakeCause::Storage(macroonz::native_storage::StorageError::Busy)
    ));
    let finished = pending.finish_cleanup(std::time::Duration::from_secs(5));
    assert!(!finished.is_pending());
    assert!(
        matches!(finished.cause(), BakeCause::Formatter(macroonz::native_publication::FormatError::Query { run, .. })
        if matches!(run.as_ref(), macroonz::native_process::ProcessRun::Finished(_)))
    );
    let fresh = super::command_fixture::preparation(&source, &work, &host)?;
    let result = bake(BakeCommand::Inspect(Box::new(fresh)), publication)
        .map_err(|error| error.to_string())?;
    assert!(matches!(result, BakeOutput::Prepared(_)));
    Ok(())
}

#[test]
fn command_refuses_destination_workspace_overlap_before_writes() -> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let output = root()?;
    let nested = output.join("scratch");
    std::fs::create_dir(&nested).map_err(|error| error.to_string())?;
    let aliases = root()?;
    let alias = aliases.join("destination");
    super::stage_refusal::link(&output, &alias)?;
    for work in [&output, &nested, &alias] {
        let preparation = preparation(&source, work, &host)?;
        let before = snapshot(&output)?;
        for command in [
            BakeCommand::Check {
                preparation: Box::new(preparation.clone()),
                destination: destination(&output)?,
            },
            generate(preparation, &source, &output, &host, bounds()?)?,
        ] {
            let error = bake(command, publication).err().ok_or("overlap accepted")?;
            assert!(matches!(
                error.cause(),
                BakeCause::Destination {
                    error: DestinationError::Conflict(_),
                    compiled: None
                }
            ));
            assert_eq!(snapshot(&output)?, before);
        }
    }
    super::stage_refusal::unlink(&alias)
}

#[test]
fn compiler_is_included_in_the_whole_command_allowance() -> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let work = root()?;
    let output = root()?;
    let preparation = preparation(&source, &work, &host)?;
    for axis in ["runs", "time", "capture"] {
        let mut selected = preparation.clone();
        match axis {
            "runs" => selected.tools.runs = 5,
            "time" => selected.tools.time = std::time::Duration::from_secs(325),
            "capture" => selected.tools.capture_bytes = 10_485_760,
            _ => return Err("unknown budget axis".to_owned()),
        }
        let error = bake(
            generate(selected, &source, &output, &host, bounds()?)?,
            publication,
        )
        .err()
        .ok_or("compiler allowance omitted")?;
        assert!(
            matches!(error.cause(), BakeCause::Configuration(_)),
            "{axis}"
        );
        assert!(snapshot(&work)?.is_empty());
        assert!(snapshot(&output)?.is_empty());
    }
    Ok(())
}

#[test]
fn recovery_uses_retained_intent_without_calling_generation() -> Result<(), String> {
    let source = root()?;
    let output = root()?;
    let compiled = subject(
        &source,
        "command-recovery",
        [7, 9, 42],
        "generated/other.rs",
    )?;
    let destination = destination(&output)?;
    let mut installation = destination
        .begin(&compiled)
        .map_err(|error| error.to_string())?;
    assert!(
        installation
            .write_next()
            .map_err(|error| error.to_string())?
            .is_some()
    );
    drop(installation);
    let called = std::cell::Cell::new(false);
    let result = bake::<super::types::Example, _>(BakeCommand::Recover(destination), || {
        called.set(true);
        Err("must not call generation")
    })
    .map_err(|error| error.to_string())?;
    assert!(matches!(result, BakeOutput::Recovered));
    assert!(!called.get());
    installed(&output, compiled.prepared())
}
