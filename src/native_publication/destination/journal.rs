use super::types::{InstallationIntent, WireIntent};
use super::{DestinationError, DestinationLimits, files, record};
use crate::native_storage::{
    StorageArtifact, StorageBatch, StorageError, StorageLimits, StorageName, StorageRoot,
    StorageTransaction,
};
use cap_std::fs::Dir;

const FORMAT: &str = "macroonz-publication-intent/1";
const PAYLOAD: &str = ".macroonz-publication/item-pending/item-intent";
const COMMITTED: &str = ".macroonz-publication/item-pending/.committed";

pub(super) fn bound(limits: DestinationLimits) -> Result<usize, DestinationError> {
    limits
        .bytes
        .checked_mul(4)
        .and_then(|bytes| {
            limits
                .metadata
                .checked_mul(8)
                .and_then(|metadata| bytes.checked_add(metadata))
        })
        .and_then(|bytes| {
            limits
                .files
                .checked_mul(4)
                .and_then(|files| bytes.checked_add(files))
        })
        .and_then(|bytes| bytes.checked_add(256))
        .ok_or_else(|| DestinationError::Metadata("intent byte bound overflow".to_owned()))
}

pub(super) fn encode(intent: &InstallationIntent) -> Result<Vec<u8>, DestinationError> {
    let before = intent.before.as_ref().map(record::encode).transpose()?;
    let after = record::encode(&intent.after)?;
    let mut bytes = serde_json::to_vec(&(FORMAT, before, after, &intent.payloads))
        .map_err(|error| DestinationError::Metadata(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub(super) fn decode(
    bytes: &[u8],
    limits: DestinationLimits,
) -> Result<InstallationIntent, DestinationError> {
    if bytes.len() > bound(limits)? {
        return Err(DestinationError::Metadata("intent byte bound".to_owned()));
    }
    let (format, before, after, payloads): WireIntent = serde_json::from_slice(bytes)
        .map_err(|error| DestinationError::Metadata(error.to_string()))?;
    if format != FORMAT {
        return Err(DestinationError::Metadata("intent format".to_owned()));
    }
    let before = before
        .as_ref()
        .map(|recorded| record::decode(recorded, limits))
        .transpose()?;
    let intent =
        InstallationIntent::admitted(before, record::decode(&after, limits)?, payloads, limits)?;
    if encode(&intent)? != bytes {
        return Err(DestinationError::Metadata("noncanonical intent".to_owned()));
    }
    Ok(intent)
}

pub(super) fn root(directory: &Dir) -> Result<StorageRoot, DestinationError> {
    if !files::directory(directory, files::CONTROL)? {
        directory
            .create_dir(files::CONTROL)
            .map_err(DestinationError::Filesystem)?;
    }
    Ok(StorageRoot::from_directory(
        directory
            .open_dir(files::CONTROL)
            .map_err(DestinationError::Filesystem)?,
    ))
}

pub(super) fn committed(directory: &Dir) -> Result<bool, DestinationError> {
    Ok(files::read(directory, COMMITTED, 1024)?.is_some())
}

pub(super) fn retain(
    directory: &Dir,
    intent: &InstallationIntent,
    limits: DestinationLimits,
) -> Result<(), DestinationError> {
    if committed(directory)? {
        return Err(DestinationError::Pending);
    }
    let root = root(directory)?;
    let name = StorageName::informed("pending").map_err(DestinationError::Storage)?;
    let payload = StorageName::informed("intent").map_err(DestinationError::Storage)?;
    let bytes = encode(intent)?;
    let artifacts = [StorageArtifact {
        name: &payload,
        bytes: &bytes,
    }];
    let batch = StorageBatch::informed(
        &artifacts,
        StorageLimits {
            artifacts: 1,
            bytes: bound(limits)?,
        },
    )
    .map_err(DestinationError::Storage)?;
    let mut transaction = match StorageTransaction::begin(&root, &name, batch) {
        Ok(transaction) => transaction,
        Err(StorageError::Collision) => {
            StorageTransaction::recover(&root, &name, batch).map_err(DestinationError::Storage)?
        }
        Err(error) => return Err(DestinationError::Storage(error)),
    };
    while transaction
        .write_next()
        .map_err(DestinationError::Storage)?
        .is_some()
    {}
    transaction.commit().map_err(DestinationError::Storage)
}

pub(super) fn load(
    directory: &Dir,
    limits: DestinationLimits,
) -> Result<Option<InstallationIntent>, DestinationError> {
    let Some(bytes) = files::read(directory, PAYLOAD, bound(limits)?)? else {
        return Ok(None);
    };
    if committed(directory)? {
        let root = root(directory)?;
        let name = StorageName::informed("pending").map_err(DestinationError::Storage)?;
        let artifacts = root
            .load(
                &name,
                StorageLimits {
                    artifacts: 1,
                    bytes: bound(limits)?,
                },
            )
            .map_err(DestinationError::Storage)?;
        if !matches!(artifacts.as_slice(), [artifact] if artifact.name.spelling() == "intent" && artifact.bytes == bytes)
        {
            return Err(DestinationError::Metadata(
                "committed intent inventory".to_owned(),
            ));
        }
    }
    decode(&bytes, limits).map(Some)
}

pub(super) fn retire(
    directory: &Dir,
    intent: Option<&InstallationIntent>,
    limits: DestinationLimits,
) -> Result<(), DestinationError> {
    let root = root(directory)?;
    let name = StorageName::informed("pending").map_err(DestinationError::Storage)?;
    let payload = StorageName::informed("intent").map_err(DestinationError::Storage)?;
    let bytes = intent.map(encode).transpose()?;
    let expected = bytes
        .as_ref()
        .map(|bytes| StorageArtifact {
            name: &payload,
            bytes,
        })
        .into_iter()
        .collect::<Vec<_>>();
    root.discard(
        &name,
        &expected,
        StorageLimits {
            artifacts: 1,
            bytes: bound(limits)?,
        },
    )
    .map_err(DestinationError::Storage)
}
