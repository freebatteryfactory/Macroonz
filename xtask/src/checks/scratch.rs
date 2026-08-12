//! The scratch root the tree-shaped reversals are planted against.
//!
//! Several laws read a directory rather than a text, so a fixture string cannot
//! reach them: what they judge is what a tree contains. They are planted against
//! a scratch root under the platform's temp directory instead. Nothing is
//! written inside the repository — the laws that guard the tree are never proven
//! by dirtying the tree — and each root is removed when its fixture drops.
//!
//! This module exists only under `cfg(test)`: it is fixture machinery shared by
//! four law families, and it ships in no binary.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// One scratch root outside the repository, and the files planted in it.
pub(crate) struct Scratch {
    root: PathBuf,
}

impl Scratch {
    /// A fresh scratch root, named for the reversal that built it. The
    /// process id and a run counter keep two fixtures — and two concurrent
    /// runs — from sharing one root.
    pub(crate) fn named(name: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "threadpak-xtask-{}-{ordinal}-{name}",
            std::process::id()
        ));
        let _cleared = fs::remove_dir_all(&root);
        let _made = fs::create_dir_all(&root);
        Self { root }
    }

    /// Plants one file at a root-relative path, creating its parents.
    pub(crate) fn write(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            let _made = fs::create_dir_all(parent);
        }
        let _written = fs::write(&path, contents);
    }

    /// The scratch root, as a check reads it.
    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _removed = fs::remove_dir_all(&self.root);
    }
}
