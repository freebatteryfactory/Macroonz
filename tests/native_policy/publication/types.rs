#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Values(pub(super) [u8; 3]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Seat {
    Definition,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Example;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct InlineSeat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct InlineExample;

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
pub(super) type DirectorySnapshot = Vec<(std::path::PathBuf, Option<Vec<u8>>)>;
