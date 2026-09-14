use std::path::PathBuf;

#[path = "type_guard.rs"]
mod guard;

/// Bounds for one compiler-emitted dependency file and its selected source roster.
#[derive(Debug, Clone, Copy)]
pub struct DependencyLimits {
    /// Maximum retained dependency-file bytes.
    pub bytes: usize,
    /// Maximum unique source paths for the selected artifact.
    pub files: usize,
}

/// A bounded compiler dependency rule joined to an actually reported link artifact.
#[derive(Debug)]
pub struct DependencyInfo {
    bytes: Vec<u8>,
    artifact: PathBuf,
    files: Vec<PathBuf>,
}

/// Why compiler output established no selected source-dependency roster.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyError {
    /// The selected compilation did not establish a successful link artifact.
    NotCompiled,
    /// Opening or reading the explicitly selected dependency file failed.
    Filesystem(String),
    /// The dependency file is not an ordinary file.
    NotRegular,
    /// The physical dependency file exceeds its byte allowance.
    ByteBound,
    /// The selected source roster exceeds its file allowance.
    FileBound,
    /// The dependency file has no unique rule for a reported artifact.
    Artifact,
    /// A dependency rule cannot be interpreted without ambiguity.
    Representation(String),
}
