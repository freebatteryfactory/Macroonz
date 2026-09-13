use super::types::Workspace;
use crate::native_storage::StorageError;
#[cfg(any(unix, windows))]
use crate::native_storage::StorageRoot;
use std::path::Path;

pub(super) fn open(path: &Path) -> Result<Workspace, StorageError> {
    #[cfg(any(unix, windows))]
    {
        let root = StorageRoot::open(path)?;
        let lease = root.exclusive()?;
        Ok(Workspace { root, lease })
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _path = path;
        Err(StorageError::Unavailable)
    }
}

pub(super) fn input(workspace: &Workspace) -> Result<std::fs::File, std::io::Error> {
    #[cfg(any(unix, windows))]
    {
        let root = workspace.root.directory();
        let name = ".macroonz-bake-input";
        match root.symlink_metadata(name) {
            Ok(metadata) if metadata.is_file() => root.remove_file(name)?,
            Ok(_metadata) => {
                return Err(std::io::Error::other(
                    "command input is not a regular disposable file",
                ));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        root.open_with(
            name,
            cap_std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true),
        )
        .map(cap_std::fs::File::into_std)
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _workspace = workspace;
        Err(std::io::Error::other(
            "command input is unavailable on this target",
        ))
    }
}
