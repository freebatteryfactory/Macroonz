use super::types::{DirectorySnapshot, Example};
use macroonz::native_publication::{DestinationLimits, PreparedPublication};
use std::path::Path;

pub(super) const LIMITS: DestinationLimits = DestinationLimits {
    files: 16,
    bytes: 65536,
    metadata: 16384,
};

pub(super) fn record(prepared: &PreparedPublication<Example>) -> serde_json::Value {
    let mut files = prepared
        .files()
        .map(|file| {
            serde_json::json!([
                file.path().spelling(),
                file.canonical_digest().as_bytes(),
                file.published_digest().as_bytes(),
                file.bytes().len()
            ])
        })
        .collect::<Vec<_>>();
    files.sort_by(|left, right| {
        left.get(0)
            .and_then(serde_json::Value::as_str)
            .cmp(&right.get(0).and_then(serde_json::Value::as_str))
    });
    serde_json::json!(["macroonz-publication/1", files])
}

pub(super) fn write_record(root: &Path, record: &serde_json::Value) -> Result<(), String> {
    std::fs::create_dir_all(root.join(".macroonz-publication"))
        .map_err(|error| error.to_string())?;
    std::fs::write(
        root.join(".macroonz-publication/current"),
        format!("{record}\n"),
    )
    .map_err(|error| error.to_string())
}

pub(super) fn historical(
    root: &Path,
    prepared: &PreparedPublication<Example>,
) -> Result<(), String> {
    for file in prepared.files() {
        let path = root.join(file.path().spelling());
        std::fs::create_dir_all(path.parent().ok_or("no parent")?)
            .map_err(|error| error.to_string())?;
        std::fs::write(path, file.bytes()).map_err(|error| error.to_string())?;
    }
    std::fs::write(root.join(".macroonz-storage-lock"), b"").map_err(|error| error.to_string())?;
    write_record(root, &record(prepared))
}

pub(super) fn snapshot(root: &Path) -> Result<DirectorySnapshot, String> {
    let mut result = Vec::new();
    let mut directories = vec![root.to_path_buf()];
    while let Some(directory) = directories.pop() {
        for entry in std::fs::read_dir(directory).map_err(|error| error.to_string())? {
            let entry = entry.map_err(|error| error.to_string())?;
            let kind = entry.file_type().map_err(|error| error.to_string())?;
            let path = entry.path();
            let bytes = if kind.is_dir() {
                directories.push(path.clone());
                None
            } else if kind.is_file() {
                Some(std::fs::read(&path).map_err(|error| error.to_string())?)
            } else {
                return Err("unexpected snapshot entry".to_owned());
            };
            result.push((path, bytes));
        }
    }
    result.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(result)
}

pub(super) fn selected_snapshot(root: &Path, paths: &[&str]) -> Result<DirectorySnapshot, String> {
    paths
        .iter()
        .map(|path| {
            let path = root.join(path);
            let bytes = match std::fs::read(&path) {
                Ok(bytes) => Some(bytes),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
                Err(error) => return Err(error.to_string()),
            };
            Ok((path, bytes))
        })
        .collect()
}
