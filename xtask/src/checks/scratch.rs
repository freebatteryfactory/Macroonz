//! The scratch root the tree-shaped reversals are planted against.
//!
//! Several laws judge a TREE rather than a text, so a fixture string cannot
//! reach them: what they judge is what a tree contains. They are planted against
//! a scratch root under the platform's temp directory instead, and read through
//! the same snapshot builder the real run uses — a law proven against a reading
//! built by different machinery would be a law proven against a different model.
//! Nothing is written inside the repository — the laws that guard the tree are
//! never proven by dirtying the tree. Fixture planting and deliberate removal
//! carry every filesystem refusal; drop makes a best-effort cleanup only after
//! the fixture can no longer affect a verdict.
//! Scratch Git operations use the repository reader's command constructor, so
//! the explicit fixture root receives the same complete ambient-routing
//! containment as the production committed snapshot.
//!
//! This module exists only under `cfg(test)`: it is fixture machinery shared by
//! several law families, and it ships in no binary.

use std::fs;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::repository::snapshot::{RepositorySnapshot, git as repository_git};

/// One scratch root outside the repository, and the files planted in it.
pub(crate) struct Scratch {
    /// Where the fixture tree stands.
    root: PathBuf,
}

impl Scratch {
    /// A fresh scratch root, named for the reversal that built it. The
    /// process id and a run counter keep two fixtures — and two concurrent
    /// runs — from sharing one root.
    pub(crate) fn named(name: &str) -> Result<Self, String> {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "threadpak-xtask-{}-{ordinal}-{name}",
            std::process::id()
        ));
        match fs::remove_dir_all(&root) {
            Ok(()) => (),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
            Err(error) => {
                return Err(format!(
                    "could not clear scratch root {}: {error}",
                    root.display()
                ));
            }
        }
        fs::create_dir_all(&root).map_err(|error| {
            format!("could not create scratch root {}: {error}", root.display())
        })?;
        Ok(Self { root })
    }

    /// Plants one file at a root-relative path, creating its parents.
    pub(crate) fn write(&self, relative: &str, contents: &str) -> Result<(), String> {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("could not create {}: {error}", parent.display()))?;
        }
        fs::write(&path, contents)
            .map_err(|error| format!("could not write scratch file {}: {error}", path.display()))
    }

    /// Removes one planted file, so a reversal can plant an absence.
    pub(crate) fn remove(&self, relative: &str) -> Result<(), String> {
        let path = self.root.join(relative);
        fs::remove_file(&path)
            .map_err(|error| format!("could not remove scratch file {}: {error}", path.display()))
    }

    /// The reading of the fixture tree, taken by the builder the real run uses.
    ///
    /// The fixture is committed immediately before it is read. Production has
    /// no working-tree snapshot road, so a test does not gain one merely for
    /// convenience: the same Git-owned population and immutable-blob builder
    /// reads both real and planted trees.
    pub(crate) fn read(&self) -> Result<RepositorySnapshot, String> {
        self.commit_current_tree()?;
        RepositorySnapshot::read(&self.root)
    }

    /// Makes the fixture's current files one clean committed tree.
    fn commit_current_tree(&self) -> Result<(), String> {
        if !self.root.join(".git").exists() {
            self.git(&["init", "--quiet"])?;
            self.git(&["config", "user.name", "ThreadPak fixture"])?;
            self.git(&["config", "user.email", "fixture@threadpak.invalid"])?;
            self.git(&["config", "core.autocrlf", "false"])?;
        }
        self.git(&["add", "-A"])?;
        self.git(&[
            "commit",
            "--quiet",
            "--allow-empty",
            "--message",
            "planted fixture state",
        ])?;
        Ok(())
    }

    /// Runs one Git operation against this fixture and carries its refusal.
    fn git(&self, arguments: &[&str]) -> Result<(), String> {
        let output = repository_git(&self.root)
            .args(arguments)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|error| format!("fixture git {}: {error}", arguments.join(" ")))?;
        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "fixture git {} refused: {}",
                arguments.join(" "),
                String::from_utf8_lossy(&output.stderr).trim()
            ))
        }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        // Destructors cannot return a refusal. Every semantic planting/removal
        // road above is fallible; this is only best-effort temporary cleanup
        // after the fixture can no longer affect a verdict.
        let _removed = fs::remove_dir_all(&self.root);
    }
}

#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};

    use super::Scratch;
    use crate::repository::snapshot::RepositorySnapshot;

    /// Git routing inherited by the test process cannot redirect the fixture's
    /// initialization, index, or commit away from the explicit scratch root.
    ///
    /// The hostile is injected only into a child process. No process-global
    /// environment is changed while the rest of the Rust tests may be running.
    #[test]
    fn ambient_git_routing_cannot_redirect_scratch_commits() -> Result<(), String> {
        const CHILD: &str = "THREADPAK_SCRATCH_GIT_ROUTING_CHILD";

        if std::env::var_os(CHILD).is_some() {
            let scratch = Scratch::named("routing-explicit")?;
            scratch.write("explicit.txt", "explicit scratch root\n")?;
            let snapshot = scratch.read()?;
            assert!(snapshot.files().get("explicit.txt").is_some());
            assert!(snapshot.files().get("alternate.txt").is_none());
            return Ok(());
        }

        let alternate = Scratch::named("routing-alternate")?;
        alternate.write("alternate.txt", "alternate repository\n")?;
        let before = alternate.read()?.committed().to_string();

        let child = std::env::current_exe()
            .map_err(|error| format!("current xtask test executable: {error}"))?;
        let output = Command::new(child)
            .arg("ambient_git_routing_cannot_redirect_scratch_commits")
            .arg("--test-threads=1")
            .env(CHILD, "1")
            .env("GIT_DIR", alternate.root.join(".git"))
            .env("GIT_WORK_TREE", &alternate.root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|error| format!("scratch-routing child test: {error}"))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !output.status.success() || !stdout.contains("1 passed") {
            return Err(format!(
                "scratch-routing child did not commit the explicit root:\n{stdout}\n{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let after = RepositorySnapshot::read(&alternate.root)?
            .committed()
            .to_string();
        assert_eq!(after, before, "Scratch Git mutated the routed repository");
        Ok(())
    }

    /// A failed removal is an explicit fixture failure, not a successful
    /// absence plant that lets the challenged law run against the wrong tree.
    #[test]
    fn removing_a_path_that_was_never_planted_refuses() -> Result<(), String> {
        let scratch = Scratch::named("missing-removal")?;
        let found = scratch.remove("never-there.txt");
        assert!(
            found.is_err_and(|refusal| refusal.contains("never-there.txt")),
            "a failed scratch removal was discarded"
        );
        Ok(())
    }

    /// A failed write is likewise explicit. Planting a file where the next
    /// write needs a directory makes the failure deterministic on every host.
    #[test]
    fn a_write_whose_parent_cannot_be_created_refuses() -> Result<(), String> {
        let scratch = Scratch::named("blocked-parent")?;
        scratch.write("blocked", "an ordinary file\n")?;
        let found = scratch.write("blocked/child.txt", "cannot be planted\n");
        assert!(
            found.is_err_and(|refusal| refusal.contains("blocked")),
            "a failed scratch write was discarded"
        );
        Ok(())
    }
}
