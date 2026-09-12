use macroonz::harness::muterprater::backend_archive::{
    ArchivedBackendManifest, BackendArchiveLimits, read_backend,
};
use macroonz::harness::muterprater::verdict_archive::MutationRunArchiveLimits;
use macroonz::harness::report::archive::ArchiveLimits;
use macroonz::native_storage::{
    StorageArtifact, StorageBatch, StorageLimits, StorageName, StorageRoot, StorageTransaction,
};
use std::path::Path;

pub(super) const fn limits() -> BackendArchiveLimits {
    BackendArchiveLimits::declared(
        MutationRunArchiveLimits::declared(ArchiveLimits::declared(4_194_304, 1_048_576), 1024),
        128,
        16,
        1024,
    )
}

pub(super) fn round_trip(
    directory: &Path,
    name: &StorageName,
    archive: &ArchivedBackendManifest,
) -> Result<ArchivedBackendManifest, String> {
    let root = StorageRoot::open(directory).map_err(|error| format!("{error:?}"))?;
    let payload = StorageName::informed("manifest").map_err(|error| format!("{error:?}"))?;
    let artifacts = [StorageArtifact {
        name: &payload,
        bytes: archive.encoded(),
    }];
    let bounds = StorageLimits {
        artifacts: 1,
        bytes: 4_194_304,
    };
    let batch = StorageBatch::informed(&artifacts, bounds).map_err(|error| format!("{error:?}"))?;
    let mut transaction =
        StorageTransaction::begin(&root, name, batch).map_err(|error| format!("{error:?}"))?;
    while transaction
        .write_next()
        .map_err(|error| format!("{error:?}"))?
        .is_some()
    {}
    transaction.commit().map_err(|error| format!("{error:?}"))?;
    let loaded = root
        .load(name, bounds)
        .map_err(|error| format!("{error:?}"))?;
    let [found] = loaded.as_slice() else {
        return Err("stored manifest roster differs".to_owned());
    };
    if found.name != payload {
        return Err("stored manifest name differs".to_owned());
    }
    read_backend(&found.bytes, limits()).map_err(|error| format!("{error:?}"))
}
