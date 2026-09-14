//! A new process loads canonical storage and selects its own current implementation.

use super::{
    fixture::{self, mapped},
    retention::{self, LIMITS},
};
use macroonz::harness::report::replay::ReplayOutcome;
use macroonz::native_storage::{StorageName, StorageRoot};
use macroonz::workflow::StoredRun;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn child(
    call: macroonz::harness::runner::TrialCall<macroonz::harness::input::BoundInput<Vec<u8>>>,
    expected: ReplayOutcome,
) -> Result<(), String> {
    let mut path = String::new();
    std::io::stdin()
        .lock()
        .take(4096)
        .read_to_string(&mut path)
        .map_err(|error| error.to_string())?;
    let root = mapped(StorageRoot::open(Path::new(&path)))?;
    let key = mapped(StorageName::informed("run"))?;
    fixture::reset();
    let saved = mapped(StoredRun::load(
        &root,
        &key,
        fixture::decoder()?.profile(),
        LIMITS,
    ))?;
    assert_eq!(fixture::observations(), (0, 0, 0));
    let current = mapped(saved.replay(
        0,
        &fixture::binding("selected", fixture::fixed_revision(), call)?,
        &fixture::decoder()?,
        fixture::invocation(1),
        fixture::INPUT_LIMITS,
    ))?;
    assert_eq!(fixture::observations(), (1, 1, 0));
    assert_eq!(current.witness().payload(), &[1]);
    assert_eq!(mapped(current.comparison().as_ref())?.outcome(), expected);
    Ok(())
}

#[test]
#[ignore = "Invoked by the root retained-workflow parent in a fresh process."]
fn defective_child() -> Result<(), String> {
    child(fixture::defective, ReplayOutcome::DefectReproduced)
}

#[test]
#[ignore = "Invoked by the root retained-workflow parent in a fresh process."]
fn fixed_child() -> Result<(), String> {
    child(fixture::fixed, ReplayOutcome::FixedOnWitness)
}

#[test]
fn fresh_process_loads_storage_and_observes_defect_and_repair() -> Result<(), String> {
    retention::observe(|root, path| {
        let run = fixture::run(&[7, 1, 9])?;
        let key = mapped(StorageName::informed("run"))?;
        mapped(run.retain(root, &key, &[fixture::capsule(&run)?], LIMITS))?;
        for test in [
            "workflow::process::defective_child",
            "workflow::process::fixed_child",
        ] {
            let mut child =
                Command::new(std::env::current_exe().map_err(|error| error.to_string())?)
                    .args([
                        "--exact",
                        test,
                        "--ignored",
                        "--nocapture",
                        "--test-threads=1",
                    ])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .map_err(|error| error.to_string())?;
            child
                .stdin
                .take()
                .ok_or("child stdin missing")?
                .write_all(path.to_str().ok_or("root path is not UTF-8")?.as_bytes())
                .map_err(|error| error.to_string())?;
            assert!(child.wait().map_err(|error| error.to_string())?.success());
        }
        Ok(())
    })
}
