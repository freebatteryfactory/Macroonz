//! Verifies one explicitly named sealed bundle against its own manifest and placement.

use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::manifest::{
    self, COMPLETION_NAME, MANIFEST_BYTE_LIMIT, MANIFEST_NAME, PLACEMENT_RECEIPT_BYTE_LIMIT,
    PLACEMENT_RECEIPT_NAME, PlacementReceipt,
};
use crate::storage;

pub(crate) fn run(repository_path: &Path, run_path: &Path) -> Result<(), String> {
    let repository = storage::canonical_real_directory(repository_path, "repository")?;
    let run = storage::canonical_real_directory(run_path, "run")?;
    let warehouse = repository.join(".durafx");
    if !run.starts_with(&warehouse) {
        return Err("run must remain beneath the declared repository `.durafx`".to_owned());
    }
    require_completion(&run)?;

    material(&run, &warehouse)
}

pub(crate) fn precommit(run_path: &Path, warehouse: &Path) -> Result<(), String> {
    refuse_completion(run_path)?;
    material(run_path, warehouse)
}

fn material(run: &Path, warehouse: &Path) -> Result<(), String> {
    let placement_path = run.join(PLACEMENT_RECEIPT_NAME);
    require_regular_file(&placement_path, "placement receipt")?;
    let placement_bytes = storage::read_bounded(
        &placement_path,
        PLACEMENT_RECEIPT_BYTE_LIMIT,
        "placement receipt",
    )?;
    let placement = manifest::parse_placement_receipt(&placement_bytes)?;
    let manifest_path = run.join(MANIFEST_NAME);
    require_regular_file(&manifest_path, "manifest")?;
    let manifest_bytes = storage::read_bounded(&manifest_path, MANIFEST_BYTE_LIMIT, "manifest")?;
    let record_limit = placement
        .entry_limit
        .checked_add(1)
        .ok_or_else(|| "manifest record limit overflowed".to_owned())?;
    let records = manifest::parse_manifest(&manifest_bytes, record_limit)?;
    manifest::validate_manifest_budget(&records, &placement)?;
    require_exact_placement_record(&records, &placement_bytes)?;

    let generated_bytes = represented_length(&placement_bytes, "placement receipt")?
        .checked_add(represented_length(&manifest_bytes, "manifest")?)
        .ok_or_else(|| "generated custody material length overflowed".to_owned())?;
    let budget = storage::CensusBudget::declared(placement.entry_limit, placement.byte_limit)
        .extended(3, generated_bytes)?;
    payload(run, &records, budget)?;
    let digest = manifest::run_digest(&records, &placement)?;
    verify_directory_key(run, warehouse, &placement, &digest)
}

fn refuse_completion(run: &Path) -> Result<(), String> {
    let completion = run.join(COMPLETION_NAME);
    match fs::symlink_metadata(&completion) {
        Ok(_) => Err("incomplete run already carries a completion marker".to_owned()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("cannot inspect completion marker: {error}")),
    }
}

fn require_completion(run: &Path) -> Result<(), String> {
    let completion = run.join(COMPLETION_NAME);
    let metadata = fs::symlink_metadata(&completion).map_err(|error| {
        format!("run is incomplete because its completion marker cannot be inspected: {error}")
    })?;
    if storage::metadata_has_indirection(&metadata) || !metadata.is_dir() {
        return Err("run completion marker is not a real directory".to_owned());
    }
    let mut entries = fs::read_dir(&completion)
        .map_err(|error| format!("cannot read completion marker: {error}"))?;
    match entries.next() {
        None => {}
        Some(Ok(_)) => return Err("run completion marker is not empty".to_owned()),
        Some(Err(error)) => {
            return Err(format!("cannot enumerate completion marker: {error}"));
        }
    }
    Ok(())
}

fn require_regular_file(path: &Path, role: &str) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("cannot inspect {role} {}: {error}", path.display()))?;
    if storage::metadata_has_indirection(&metadata) || !metadata.is_file() {
        return Err(format!("{role} is not a regular file: {}", path.display()));
    }
    Ok(())
}

fn represented_length(bytes: &[u8], role: &str) -> Result<u64, String> {
    u64::try_from(bytes.len())
        .map_err(|error| format!("{role} length cannot be represented: {error}"))
}

fn require_exact_placement_record(
    records: &[manifest::PayloadRecord],
    placement_bytes: &[u8],
) -> Result<(), String> {
    let record = records
        .iter()
        .find(|record| record.path == PLACEMENT_RECEIPT_NAME)
        .ok_or_else(|| "manifest does not include the placement receipt".to_owned())?;
    let observed_bytes = represented_length(placement_bytes, "placement receipt")?;
    let observed_hash = blake3::hash(placement_bytes).to_hex().to_string();
    if record.bytes != observed_bytes || record.hash != observed_hash {
        return Err("placement receipt disagrees with its manifest record".to_owned());
    }
    Ok(())
}

pub(crate) fn payload(
    run_path: &Path,
    expected: &[manifest::PayloadRecord],
    budget: storage::CensusBudget,
) -> Result<(), String> {
    let actual = storage::census_bundle(run_path, budget)?;
    let expected_by_path = expected
        .iter()
        .map(|record| (record.path.as_str(), record))
        .collect::<BTreeMap<_, _>>();
    let actual_by_path = actual
        .iter()
        .map(|record| (record.path.as_str(), record))
        .collect::<BTreeMap<_, _>>();
    let mut findings = Vec::new();
    for (path, expected_record) in &expected_by_path {
        match actual_by_path.get(path) {
            None => findings.push(format!("missing file `{path}`")),
            Some(actual_record)
                if actual_record.bytes != expected_record.bytes
                    || actual_record.hash != expected_record.hash =>
            {
                findings.push(format!("changed file `{path}`"));
            }
            Some(_) => {}
        }
    }
    for path in actual_by_path.keys() {
        if !expected_by_path.contains_key(path) {
            findings.push(format!("additional file `{path}`"));
        }
    }
    if findings.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "bundle verification failed: {}",
            findings.join("; ")
        ))
    }
}

fn verify_directory_key(
    run: &Path,
    expected_warehouse: &Path,
    placement: &PlacementReceipt,
    digest: &str,
) -> Result<(), String> {
    let expected_key = format!("{}-{digest}", placement.label);
    require_component(run, &expected_key, "run key")?;
    let host_directory = run
        .parent()
        .ok_or_else(|| "run lacks host-target parent".to_owned())?;
    require_component(host_directory, &placement.host_target, "host-target")?;
    let revision_directory = host_directory
        .parent()
        .ok_or_else(|| "run lacks source-revision parent".to_owned())?;
    require_component(
        revision_directory,
        &placement.source_revision,
        "source revision",
    )?;
    let plane_directory = revision_directory
        .parent()
        .ok_or_else(|| "run lacks plane parent".to_owned())?;
    require_component(plane_directory, &placement.plane, "plane")?;
    let warehouse = plane_directory
        .parent()
        .ok_or_else(|| "run lacks warehouse parent".to_owned())?;
    require_component(warehouse, ".durafx", "warehouse")?;
    if warehouse != expected_warehouse {
        return Err("run warehouse disagrees with the declared repository".to_owned());
    }
    Ok(())
}

fn require_component(path: &Path, expected: &str, role: &str) -> Result<(), String> {
    let actual = path
        .file_name()
        .and_then(OsStr::to_str)
        .ok_or_else(|| format!("{role} component is not UTF-8"))?;
    if actual != expected {
        return Err(format!(
            "directory key mismatch for {role}: expected `{expected}`, found `{actual}`"
        ));
    }
    Ok(())
}
