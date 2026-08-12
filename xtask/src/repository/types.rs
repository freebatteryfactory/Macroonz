//! The vocabulary the two families share.
//!
//! Exactly two facts cross the line between reading the repository and judging
//! it: the shape of a law, and how a declared module sits on disk. Everything
//! else a law needs is private to the law that needs it, because a name shared
//! by one owner is a name in the wrong place.

use std::path::Path;

/// One repository law: a name and the function that checks it.
pub(crate) type Check = (&'static str, fn(&Path) -> Result<(), String>);

/// How a declared module is laid out on disk.
///
/// The layout is not a formatting detail: it decides which crate-root openings
/// can be an edge. In a directory module, a submodule saying `super::` is naming
/// its own parent, which is not a forward reference at all; in a flat module,
/// `super::` and `crate::` name the same place, so both are read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModuleLayout {
    /// `name.rs` — one file, whose only parent is the crate root.
    Flat,
    /// `name/` — a directory whose submodules can say `super::` about it.
    Directory,
}
