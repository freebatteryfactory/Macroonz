//! Seats one verified staging census beneath its declared repository warehouse.

use std::fs;
use std::path::{Path, PathBuf};

use crate::arguments::SealDeclaration;
use crate::manifest::{
    self, COMPLETION_NAME, MANIFEST_NAME, PLACEMENT_RECEIPT_NAME, PlacementReceipt,
};
use crate::{storage, verify};

struct SeatMaterial<'a> {
    warehouse: &'a Path,
    staging: &'a Path,
    staging_budget: storage::CensusBudget,
    sources: &'a [storage::SourceRecord],
    placement_bytes: &'a [u8],
    manifest_bytes: &'a [u8],
    bundle_budget: storage::CensusBudget,
}

pub(crate) fn run(declaration: &SealDeclaration) -> Result<PathBuf, String> {
    validate_arguments(declaration)?;
    let repository = storage::canonical_real_directory(&declaration.repository, "repository")?;
    let staging = storage::canonical_real_directory(&declaration.staging, "staging")?;
    let warehouse = repository.join(".durafx");
    refuse_storage_overlap(&staging, &repository, &warehouse)?;

    let staging_budget =
        storage::CensusBudget::declared(declaration.entry_limit, declaration.byte_limit);
    let sources =
        storage::census_staging(&staging, staging_budget).map_err(|refusal| refusal.to_string())?;
    let placement = PlacementReceipt {
        plane: declaration.plane.clone(),
        source_revision: declaration.source_revision.clone(),
        host_target: declaration.host_target.clone(),
        entry_limit: declaration.entry_limit,
        byte_limit: declaration.byte_limit,
        label: declaration.label.clone(),
    };
    let placement_bytes = manifest::render_placement_receipt(&placement);
    let staged_records = source_manifests(&sources);
    let digest = manifest::run_digest(&staged_records, &placement)?;
    let records = manifest::complete_records(&staged_records, &placement_bytes)?;
    manifest::validate_manifest_budget(&records, &placement)?;
    let manifest_bytes = manifest::render_manifest(&records)?;
    let generated_bytes = represented_length(&placement_bytes, "placement receipt")?
        .checked_add(represented_length(&manifest_bytes, "manifest")?)
        .ok_or_else(|| "generated custody material length overflowed".to_owned())?;
    let bundle_budget = staging_budget.extended(3, generated_bytes)?;
    let run_key = format!("{}-{digest}", declaration.label);
    let host_directory = prepare_host_directory(&warehouse, &placement)?;
    let material = SeatMaterial {
        warehouse: &warehouse,
        staging: &staging,
        staging_budget,
        sources: &sources,
        placement_bytes: &placement_bytes,
        manifest_bytes: &manifest_bytes,
        bundle_budget,
    };
    seat(&host_directory, &run_key, &material)
}

fn seat(
    host_directory: &Path,
    run_key: &str,
    material: &SeatMaterial<'_>,
) -> Result<PathBuf, String> {
    let final_path = host_directory.join(run_key);
    match fs::create_dir(&final_path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(format!(
                "destination already exists: {}",
                final_path.display()
            ));
        }
        Err(error) => {
            return Err(format!(
                "cannot claim destination {}: {error}",
                final_path.display()
            ));
        }
    }

    let population = populate_incomplete(&final_path, material);
    if let Err(refusal) = population {
        return Err(cleanup_refusal(
            refusal,
            &final_path,
            material.bundle_budget,
        ));
    }
    Ok(final_path)
}

fn populate_incomplete(destination: &Path, material: &SeatMaterial<'_>) -> Result<(), String> {
    for source in material.sources {
        let target = storage::payload_path(destination, &source.manifest.path);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
        }
        storage::copy_checked(source, &target)?;
    }
    storage::write_new(
        &destination.join(PLACEMENT_RECEIPT_NAME),
        material.placement_bytes,
    )?;
    storage::write_new(&destination.join(MANIFEST_NAME), material.manifest_bytes)?;

    let refreshed = storage::census_staging(material.staging, material.staging_budget)
        .map_err(|refusal| format!("staging changed during sealing: {refusal}"))?;
    if source_manifests(&refreshed) != source_manifests(material.sources) {
        return Err("staging changed during sealing".to_owned());
    }
    storage::apply_readonly_guard(destination, material.bundle_budget.entry_limit())?;
    verify::precommit(destination, material.warehouse)?;
    fs::create_dir(destination.join(COMPLETION_NAME)).map_err(|error| {
        format!(
            "cannot publish completion marker beneath {}: {error}",
            destination.display()
        )
    })
}

fn source_manifests(sources: &[storage::SourceRecord]) -> Vec<manifest::PayloadRecord> {
    sources
        .iter()
        .map(|source| source.manifest.clone())
        .collect()
}

fn represented_length(bytes: &[u8], role: &str) -> Result<u64, String> {
    u64::try_from(bytes.len())
        .map_err(|error| format!("{role} length cannot be represented: {error}"))
}

fn cleanup_refusal(refusal: String, incomplete: &Path, budget: storage::CensusBudget) -> String {
    match storage::remove_temporary(incomplete, budget.entry_limit()) {
        Ok(()) => refusal,
        Err(cleanup) => format!("{refusal}; incomplete-seat cleanup also failed: {cleanup}"),
    }
}

fn validate_arguments(declaration: &SealDeclaration) -> Result<(), String> {
    manifest::validate_seat("plane", &declaration.plane, 64)?;
    manifest::validate_seat("source revision", &declaration.source_revision, 80)?;
    manifest::validate_seat("host-target", &declaration.host_target, 80)?;
    manifest::validate_seat("label", &declaration.label, 40)
}

fn prepare_host_directory(
    warehouse: &Path,
    placement: &PlacementReceipt,
) -> Result<PathBuf, String> {
    ensure_directory(warehouse)?;
    let plane_directory = warehouse.join(&placement.plane);
    ensure_directory(&plane_directory)?;
    let revision_directory = plane_directory.join(&placement.source_revision);
    ensure_directory(&revision_directory)?;
    let host_directory = revision_directory.join(&placement.host_target);
    ensure_directory(&host_directory)?;
    Ok(host_directory)
}

fn refuse_storage_overlap(
    staging: &Path,
    repository: &Path,
    warehouse: &Path,
) -> Result<(), String> {
    if staging == repository || !staging.starts_with(repository) {
        return Err(
            "resolved staging must be a strict descendant of the declared repository".to_owned(),
        );
    }
    if staging.starts_with(warehouse) || warehouse.starts_with(staging) {
        return Err(format!(
            "staging and warehouse must not contain one another: {} and {}",
            staging.display(),
            warehouse.display()
        ));
    }
    Ok(())
}

fn ensure_directory(path: &Path) -> Result<(), String> {
    match fs::create_dir(path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(error) => {
            return Err(format!(
                "cannot create directory {}: {error}",
                path.display()
            ));
        }
    }
    let _resolved = storage::canonical_real_directory(path, "destination component")?;
    Ok(())
}
