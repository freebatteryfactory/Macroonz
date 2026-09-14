//! The caller-owned Job count law through typed input, reduction, storage and fresh replay.

mod admission;
mod checks;
mod declaration;
mod reduction;
mod replay;

#[path = "../job_workflow/declaration.rs"]
mod job_declaration;
#[path = "../support/native_input/read.rs"]
mod native_input;

pub use job_declaration::job;

use macroonz::configuration::v1;
#[cfg(test)]
use macroonz::harness::input::{BoundInput, InputRefusal};
use macroonz::harness::report::{TrialReport, TrialSite};
use macroonz::harness::runner::{Invocation, trial_identity};
use macroonz::native_storage::{StorageName, StorageRoot};
use macroonz::workflow::{InputRun, StoredRun};
use std::io::Write;
use std::path::Path;

macroonz::support! { job_replay_support {
    declaring: crate,
    consumer: crate,
    invocation: crate::checks::BUDGETS,
    target: crate::checks::target(),
    clock: macroonz::harness::clock::HarnessClock::unavailable(),
    specimen: crate::specimen(),
} }

#[cfg(test)]
fn specimen() -> Result<BoundInput<Vec<u8>>, InputRefusal> {
    let decoder = checks::decoder()?;
    decoder.decode(macroonz::harness::input::pack(
        decoder.profile(),
        &[1, 2],
        v1::input_limits(),
    )?)
}

fn main() -> Result<(), String> {
    let configuration = native_input::read()?;
    let action = native_input::text(&configuration, "action")?;
    if !matches!(action, "retain" | "inspect" | "replay") {
        return Err("action must be retain, inspect or replay".to_owned());
    }
    let directory = Path::new(native_input::text(&configuration, "storage")?);
    if !directory.is_absolute() {
        return Err("storage must be an absolute path to an existing directory".to_owned());
    }
    let name = StorageName::informed(native_input::text(&configuration, "batch")?)
        .map_err(|error| format!("choose a portable batch name: {error:?}"))?;
    let root = StorageRoot::open(directory)
        .map_err(|error| format!("open the declared storage directory: {error:?}"))?;
    match action {
        "retain" => retain(&root, &name),
        "inspect" => {
            let saved = load(&root, &name)?;
            assert_eq!(checks::observations(), (0, 0, 0));
            write(&macroonz::presentation::stored_run(&saved).json())?;
            write("loaded historical records; decodes=0 checks=0 probes=0")
        }
        "replay" => replay::run(&load(&root, &name)?),
        _ => Err("unrecognized action".to_owned()),
    }
}

fn retain(root: &StorageRoot, name: &StorageName) -> Result<(), String> {
    let lawful = run(&[1, 2])?;
    checks::passed(&lawful)?;
    let original = run(&[1, 2, 4])?;
    let capsule = reduction::capsule(&original, &lawful)?;
    assert_eq!(capsule.input(), &[4]);
    original
        .retain(root, name, &[capsule], v1::retention_limits())
        .map_err(|error| format!("retain into a fresh batch: {error:?}"))?;
    write(&macroonz::presentation::input_run(&original).json())?;
    write("retained original [1, 2, 4]; reached witness [4]; nonfailing baseline refused")
}

fn load(root: &StorageRoot, name: &StorageName) -> Result<StoredRun, String> {
    let saved = StoredRun::load(
        root,
        name,
        checks::decoder().map_err(debug)?.profile(),
        v1::retention_limits(),
    )
    .map_err(|error| format!("load the complete retained batch: {error:?}"))?;
    let binding = trials::row::bounded_work().map_err(debug)?;
    admission::capsule(&saved, trial_identity(binding.row()))?;
    Ok(saved)
}

fn invocation() -> Invocation {
    Invocation::declared(
        checks::BUDGETS,
        checks::target(),
        TrialSite::located(module_path!(), file!(), line!(), "neutral-saved-input"),
        trials::CLOCK,
    )
}

fn run(bytes: &[u8]) -> Result<InputRun, String> {
    let table = trials::table().map_err(debug)?;
    let selection =
        macroonz::workflow::select_suites(&table.view(), &[("neutral-job", "saved-input")])
            .map_err(debug)?;
    v1::run(
        &table.view(),
        &selection,
        &checks::decoder().map_err(debug)?,
        bytes,
        invocation(),
    )
    .map_err(debug)
}

fn selected(run: &InputRun) -> Result<&TrialReport, String> {
    run.report()
        .census()
        .first()
        .and_then(|row| row.disposition().report())
        .ok_or_else(|| "the selected work-count trial did not produce a report".to_owned())
}

fn write(text: &str) -> Result<(), String> {
    writeln!(std::io::stdout(), "{text}").map_err(|error| error.to_string())
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
