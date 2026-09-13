use super::command_fixture::{generate, preparation};
use super::configure::publication;
use super::destination_fixture::snapshot;
use super::install_fixture::{destination, installed};
use crate::compiler::configure::{bounds, host, root};
use crate::compiler::real::read_count;
use macroonz::native_process::ProcessLimits;
use macroonz::native_publication::{BakeCause, BakeCommand, BakeOutput, StagingRun, bake};
use std::time::Duration;

#[test]
fn registered_command_prepares_inspects_checks_and_regenerates_the_same_formatted_source()
-> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let work = root()?;
    let output = root()?;
    let preparation = preparation(&source, &work, &host)?;
    let calls = std::cell::Cell::new(0usize);
    let declared = bake(BakeCommand::Prepare, || {
        calls.set(1);
        publication()
    })
    .map_err(|error| error.to_string())?;
    assert!(matches!(declared, BakeOutput::Declared(_)));
    assert_eq!(calls.get(), 1);
    assert!(snapshot(&work)?.is_empty());
    let BakeOutput::Prepared(prepared) = bake(
        BakeCommand::Inspect(Box::new(preparation.clone())),
        publication,
    )
    .map_err(|error| error.to_string())?
    else {
        return Err("inspection lost prepared output".to_owned());
    };
    assert_eq!(prepared.formatting().count(), 4);
    let before = snapshot(&output)?;
    let BakeOutput::Checked {
        comparison: absent, ..
    } = bake(
        BakeCommand::Check {
            preparation: Box::new(preparation.clone()),
            destination: destination(&output)?,
        },
        publication,
    )
    .map_err(|error| error.to_string())?
    else {
        return Err("check lost its comparison".to_owned());
    };
    assert!(!absent.is_current());
    assert_eq!(absent.issues().len(), 4);
    assert_eq!(snapshot(&output)?, before);
    let mut cached = None;
    for _repeat in 0usize..2 {
        let BakeOutput::Generated(compiled) = bake(
            generate(preparation.clone(), &source, &output, &host, bounds()?)?,
            publication,
        )
        .map_err(|error| error.to_string())?
        else {
            return Err("generation lost compilation".to_owned());
        };
        let current = compiled.compiler().request().process().directory();
        if let Some(cached) = &cached {
            assert_eq!(current, cached);
        } else {
            cached = Some(current.to_path_buf());
        }
        read_count(compiled.compiler(), &host, &source)?;
        installed(&output, compiled.prepared())?;
        for (expected, actual) in prepared.files().zip(compiled.prepared().files()) {
            assert_eq!(actual.bytes(), expected.bytes());
            assert_eq!(actual.canonical_digest(), expected.canonical_digest());
        }
    }
    let before_check = snapshot(&output)?;
    let BakeOutput::Checked { comparison, .. } = bake(
        BakeCommand::Check {
            preparation: Box::new(preparation),
            destination: destination(&output)?,
        },
        publication,
    )
    .map_err(|error| error.to_string())?
    else {
        return Err("check lost its comparison".to_owned());
    };
    assert!(comparison.is_current());
    assert_eq!(snapshot(&output)?, before_check);
    Ok(())
}

#[test]
fn command_budget_and_caller_refusal_precede_native_effects() -> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let work = root()?;
    let preparation = preparation(&source, &work, &host)?;
    for axis in ["runs", "time", "capture", "files", "bytes"] {
        let mut selected = preparation.clone();
        match axis {
            "runs" => selected.tools.runs = 4,
            "time" => selected.tools.time = Duration::ZERO,
            "capture" => selected.tools.capture_bytes = 0,
            "files" => selected.output.files = 0,
            "bytes" => selected.output.bytes = 0,
            _ => return Err("unknown budget axis".to_owned()),
        }
        assert!(
            bake(BakeCommand::Inspect(Box::new(selected)), publication).is_err(),
            "{axis}"
        );
        assert!(snapshot(&work)?.is_empty());
    }
    let refused =
        bake::<super::types::Example, _>(BakeCommand::Inspect(Box::new(preparation)), || Err(42u8))
            .err()
            .ok_or("caller refusal lost")?;
    assert!(matches!(refused.cause(), BakeCause::Declaration(42)));
    assert!(snapshot(&work)?.is_empty());
    Ok(())
}

#[test]
fn pending_command_keeps_workspace_custody_until_explicit_cleanup() -> Result<(), String> {
    let source = root()?;
    let host = host(&source)?;
    let work = root()?;
    let output = root()?;
    let mut preparation = preparation(&source, &work, &host)?;
    preparation.formatter = None;
    let pending_limits = ProcessLimits::informed(
        Duration::from_secs(30),
        Duration::ZERO,
        1_048_576,
        1_048_576,
    )
    .map_err(|error| error.to_string())?;
    let pending_failure = bake(
        generate(preparation.clone(), &source, &output, &host, pending_limits)?,
        publication,
    )
    .err()
    .ok_or("pending command returned success")?;
    assert!(pending_failure.is_pending());
    assert!(matches!(
        pending_failure.cause(),
        BakeCause::Compilation(StagingRun::Pending(_))
    ));
    let busy = bake(
        BakeCommand::Inspect(Box::new(preparation.clone())),
        publication,
    )
    .err()
    .ok_or("pending workspace was reused")?;
    assert!(matches!(
        busy.cause(),
        BakeCause::Storage(macroonz::native_storage::StorageError::Busy)
    ));
    assert!(snapshot(&output)?.is_empty());
    let finished = pending_failure.finish_cleanup(Duration::from_secs(5));
    assert!(!finished.is_pending());
    assert!(snapshot(&output)?.is_empty());
    let BakeOutput::Generated(compiled) = bake(
        generate(preparation, &source, &output, &host, bounds()?)?,
        publication,
    )
    .map_err(|error| error.to_string())?
    else {
        return Err("retry did not generate".to_owned());
    };
    installed(&output, compiled.prepared())
}
