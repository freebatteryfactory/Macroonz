use std::path::PathBuf;

pub(crate) struct Host {
    pub rustc: PathBuf,
    pub cargo: PathBuf,
    pub triple: String,
    pub environment: Vec<(String, String)>,
}
