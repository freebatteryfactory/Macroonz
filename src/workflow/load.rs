//! Bounded historical members return to their existing format owners.

use super::{RetentionLimits, RetentionRefusal, StoredRun};
use crate::harness::input::{self, InputProfile};
use crate::harness::report::archive::{read_capsule, read_run};
use crate::native_storage::{StorageName, StorageRoot};
use std::collections::BTreeMap;

pub(super) fn load(
    root: &StorageRoot,
    name: &StorageName,
    profile: InputProfile,
    limits: RetentionLimits,
) -> Result<StoredRun, RetentionRefusal> {
    let files = root
        .load(name, limits.storage)
        .map_err(RetentionRefusal::Storage)?;
    let mut input = None;
    let mut report = None;
    let mut capsules = BTreeMap::new();
    for file in files {
        match file.name.spelling() {
            "input" => {
                input = Some(
                    input::read(profile, &file.bytes, limits.input)
                        .map_err(RetentionRefusal::Input)?,
                );
            }
            "run" => {
                report =
                    Some(read_run(&file.bytes, limits.archive).map_err(RetentionRefusal::Archive)?);
            }
            member => {
                let index = position(member, limits.archive.rows())?;
                let capsule = read_capsule(&file.bytes, limits.archive.bytes())
                    .map_err(RetentionRefusal::Archive)?;
                if capsules.insert(index, capsule).is_some() {
                    return Err(RetentionRefusal::Members);
                }
            }
        }
    }
    StoredRun::joined(
        report.ok_or(RetentionRefusal::Members)?,
        input.ok_or(RetentionRefusal::Members)?,
        capsules,
    )
}

fn position(name: &str, rows: usize) -> Result<usize, RetentionRefusal> {
    let spelling = name
        .strip_prefix("capsule-")
        .ok_or(RetentionRefusal::Members)?;
    let index = spelling
        .parse::<usize>()
        .map_err(|_error| RetentionRefusal::Members)?;
    if index >= rows || spelling != index.to_string() {
        return Err(RetentionRefusal::Members);
    }
    Ok(index)
}
