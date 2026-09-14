use macroonz::native_mutation::MutationRequest;
use macroonz::native_storage::StorageName;
use std::path::PathBuf;

pub(super) struct Settings {
    pub operation: Operation,
    pub directory: PathBuf,
    pub storage: PathBuf,
    pub batch: StorageName,
}

pub(super) enum Operation {
    Execute(Box<MutationRequest>),
    Compare,
}
