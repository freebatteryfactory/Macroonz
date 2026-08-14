//! The scratch root the tree-shaped reversals are planted against.
//!
//! Several laws judge a TREE rather than a text, so a fixture string cannot
//! reach them: what they judge is what a tree contains. They are planted against
//! a scratch root under the platform's temp directory instead, and read through
//! the same snapshot builder the real run uses — a law proven against a reading
//! built by different machinery would be a law proven against a different model.
//! Nothing is written inside the repository — the laws that guard the tree are
//! never proven by dirtying the tree — and each root is removed when its fixture
//! drops.
//!
//! This module exists only under `cfg(test)`: it is fixture machinery shared by
//! several law families, and it ships in no binary.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::repository::snapshot::RepositorySnapshot;

/// One scratch root outside the repository, and the files planted in it.
pub(crate) struct Scratch {
    /// Where the fixture tree stands.
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

    /// Removes one planted file, so a reversal can plant an absence.
    pub(crate) fn remove(&self, relative: &str) {
        let _removed = fs::remove_file(self.root.join(relative));
    }

    /// The reading of the fixture tree, taken by the builder the real run uses.
    ///
    /// A fixture root is not a workspace and not a checkout, so what cargo
    /// resolved and what git says are DECLARED absences here rather than empty
    /// values. A law that needs either is refused against a fixture, which is
    /// the honest answer and is why the laws that read a fixture are the laws
    /// that need neither.
    pub(crate) fn read(&self) -> Result<RepositorySnapshot, String> {
        RepositorySnapshot::read(&self.root)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _removed = fs::remove_dir_all(&self.root);
    }
}
