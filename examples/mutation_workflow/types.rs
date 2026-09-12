use macroonz::native_mutation::MutationRequest;
use macroonz::native_storage::StorageName;
use std::path::PathBuf;

pub(super) struct Settings {
    pub request: MutationRequest,
    pub directory: PathBuf,
    pub storage: PathBuf,
    pub batch: StorageName,
}
