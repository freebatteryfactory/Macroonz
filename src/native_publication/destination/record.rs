#[cfg(any(unix, windows))]
use super::types::WireFile;
use super::types::{OwnedFile, Ownership};
use super::{DestinationError, DestinationLimits};
use crate::compiler::Kind;
use crate::native_publication::PreparedPublication;
#[cfg(any(unix, windows))]
use crate::native_publication::PublicationPath;

const FORMAT: &str = "macroonz-publication/1";

pub(super) fn expected<K: Kind>(
    prepared: &PreparedPublication<K>,
    limits: DestinationLimits,
) -> Result<Ownership, DestinationError> {
    let mut files = prepared
        .files()
        .map(|file| OwnedFile {
            path: file.path().clone(),
            canonical: *file.canonical_digest().as_bytes(),
            published: *file.published_digest().as_bytes(),
            bytes: file.bytes().len(),
        })
        .collect::<Vec<_>>();
    files.sort_by(|left, right| left.path.spelling().cmp(right.path.spelling()));
    Ownership::admitted(files, limits)
}

pub(super) fn encode(record: &Ownership) -> Result<Vec<u8>, DestinationError> {
    let files = record
        .files
        .iter()
        .map(|file| {
            (
                file.path.spelling(),
                file.canonical,
                file.published,
                file.bytes,
            )
        })
        .collect::<Vec<_>>();
    let mut bytes = serde_json::to_vec(&(FORMAT, files))
        .map_err(|error| DestinationError::Metadata(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

#[cfg(any(unix, windows))]
pub(super) fn decode(
    bytes: &[u8],
    limits: DestinationLimits,
) -> Result<Ownership, DestinationError> {
    if bytes.len() > limits.metadata {
        return Err(DestinationError::Metadata(
            "ownership byte bound".to_owned(),
        ));
    }
    let (format, files): (String, Vec<WireFile>) = serde_json::from_slice(bytes)
        .map_err(|error| DestinationError::Metadata(error.to_string()))?;
    if format != FORMAT {
        return Err(DestinationError::Metadata("ownership format".to_owned()));
    }
    let files = files
        .into_iter()
        .map(|(path, canonical, published, length)| {
            Ok(OwnedFile {
                path: PublicationPath::informed(&path).map_err(DestinationError::Inventory)?,
                canonical,
                published,
                bytes: length,
            })
        })
        .collect::<Result<Vec<_>, DestinationError>>()?;
    let record = Ownership::admitted(files, limits)?;
    if encode(&record)? != bytes {
        return Err(DestinationError::Metadata(
            "noncanonical ownership".to_owned(),
        ));
    }
    Ok(record)
}
