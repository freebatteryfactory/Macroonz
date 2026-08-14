//! One reading of the repository, built once, consumed by every law.
//!
//! This is the only module in this crate that touches the filesystem or starts a
//! process. Everything downstream is a pure function over what this established.
//!
//! # Why one reading rather than many
//!
//! Every law used to walk the tree for itself. Three of them parsed the same
//! Rust files three times; two of them read the same manifests through two
//! different readers; one of them decided which fenced block a document meant by
//! counting fences. Two readers over one population do not merely cost twice —
//! they can DISAGREE, and the disagreement is invisible, because each one
//! reports about the population it found. That is how a row seated by one reader
//! and claimed by neither qualified an obligation naming a law nobody wrote.
//!
//! One reading cannot disagree with itself.
//!
//! # Why nothing here has a fallback
//!
//! A reading that failed used to come back as a value that passes: an empty
//! string for a manifest, an empty vector for a tree, `"."` for a root nobody
//! resolved. Each of those answers the question the reader never got to ask, and
//! answers it in the direction that reports clean about bytes nobody opened.
//! Every fact here is a [`Read`], so a law either handles the absence or is
//! refused by [`Read::required`]; there is no method in this crate that turns an
//! unread fact into a value.

use std::collections::BTreeMap;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::repository::cargo::CargoSnapshot;
use crate::repository::markdown::MarkdownSnapshot;
use crate::repository::rust::RustSyntaxSnapshot;
use crate::repository::types::{AbsenceReason, CanonicalPath, LinkState, Read, ReadFailure};

/// Directories the reading never enters.
///
/// `.git` is git's own storage and `target` is the build's; neither is
/// repository material, and both are large enough that walking them would make
/// every run pay for bytes no law is about.
const UNREAD_DIRECTORIES: [&str; 2] = [".git", "target"];

/// The metaprogramming subsystem's directory.
///
/// Four laws name it, so the name answers to no single law and stands here
/// beside the reading they all consume.
pub(crate) const TOOLING_DIRECTORY: &str = "macros";

/// The directory the judge lives in, standing here for the same reason.
pub(crate) const JUDGE_DIRECTORY: &str = "testpak";

/// The machine's own source directory.
pub(crate) const MACHINE_DIRECTORY: &str = "src";

/// One reading of the repository.
///
/// Built once, in `main`, and handed to every law. A law is given this and never
/// a path, which is what makes "no law walks the tree for itself" a fact of the
/// types rather than a convention somebody keeps.
pub(crate) struct RepositorySnapshot {
    /// Every file in the tree, read once, with its bytes and its text.
    files: CanonicalFileMap,
    /// What Cargo's two authorities established.
    cargo: CargoSnapshot,
    /// Every Rust source, parsed once.
    rust: RustSyntaxSnapshot,
    /// Every Markdown document, parsed once.
    markdown: MarkdownSnapshot,
    /// What git says the reading was taken at.
    git: GitSnapshot,
}

impl RepositorySnapshot {
    /// Reads the repository at one root.
    ///
    /// The order is the dependency order of the readings: the files first,
    /// because every other reading is over them; then the three decoders, each
    /// authoritative for its own language; then git, which names what was read.
    pub(crate) fn read(root: &Path) -> Result<Self, String> {
        let files = CanonicalFileMap::read(root)?;
        let cargo = CargoSnapshot::read(root, &files);
        let rust = RustSyntaxSnapshot::read(&files);
        let markdown = MarkdownSnapshot::read(&files);
        let git = GitSnapshot::read(root);
        Ok(Self {
            files,
            cargo,
            rust,
            markdown,
            git,
        })
    }

    /// Every file in the tree.
    pub(crate) const fn files(&self) -> &CanonicalFileMap {
        &self.files
    }

    /// What Cargo's authorities established.
    pub(crate) const fn cargo(&self) -> &CargoSnapshot {
        &self.cargo
    }

    /// Every Rust source, parsed.
    pub(crate) const fn rust(&self) -> &RustSyntaxSnapshot {
        &self.rust
    }

    /// Every Markdown document, parsed.
    pub(crate) const fn markdown(&self) -> &MarkdownSnapshot {
        &self.markdown
    }

    /// The commit this reading was taken at.
    pub(crate) const fn commit(&self) -> &Read<CommitId> {
        &self.git.commit
    }

    /// The tree that commit names.
    pub(crate) const fn tree(&self) -> &Read<TreeId> {
        &self.git.tree
    }
}

/// Every file in the tree, keyed by the one spelling this repository uses.
pub(crate) struct CanonicalFileMap {
    /// Ordered by canonical path, so every traversal — and so every diagnostic
    /// — is the same on every machine and every run.
    entries: BTreeMap<CanonicalPath, FileFact>,
}

impl CanonicalFileMap {
    /// Reads every file under one root, skipping [`UNREAD_DIRECTORIES`].
    ///
    /// A directory that cannot be listed refuses the whole reading. What such a
    /// directory contains is unknown, and a snapshot built around an unknown is
    /// a snapshot every law downstream would report clean about.
    fn read(root: &Path) -> Result<Self, String> {
        let mut entries = BTreeMap::new();
        read_directory(root, "", &mut entries)?;
        Ok(Self { entries })
    }

    /// Every file, in canonical path order.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&CanonicalPath, &FileFact)> {
        self.entries.iter()
    }

    /// One file's facts, or nothing where the tree carries no such path.
    pub(crate) fn get(&self, path: &str) -> Option<&FileFact> {
        self.entries.get(&CanonicalPath::spelled(path))
    }

    /// One file's text.
    pub(crate) fn text(&self, path: &str) -> Read<&str> {
        match self.entries.get(&CanonicalPath::spelled(path)) {
            Some(fact) => match *fact.text() {
                Read::Known(ref text) => Read::Known(text.as_str()),
                Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
                Read::Unreadable(ref failure) => Read::Unreadable(failure.clone()),
            },
            None => Read::DeclaredAbsent(AbsenceReason::NoSuchPath),
        }
    }

    /// One file's bytes.
    pub(crate) fn bytes(&self, path: &str) -> Read<&[u8]> {
        match self.entries.get(&CanonicalPath::spelled(path)) {
            Some(fact) => match *fact.bytes() {
                Read::Known(ref bytes) => Read::Known(bytes.as_slice()),
                Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
                Read::Unreadable(ref failure) => Read::Unreadable(failure.clone()),
            },
            None => Read::DeclaredAbsent(AbsenceReason::NoSuchPath),
        }
    }

    /// Every file under one directory, at any depth, in canonical path order.
    pub(crate) fn under(
        &self,
        directory: &str,
    ) -> impl Iterator<Item = (&CanonicalPath, &FileFact)> {
        let inside = format!("{directory}/");
        self.entries
            .iter()
            .filter(move |(path, _)| path.as_str().starts_with(&inside))
    }

    /// How many files the reading carries, for the line a run opens with.
    pub(crate) fn count(&self) -> usize {
        self.entries.len()
    }
}

/// What one reading of one file established.
pub(crate) struct FileFact {
    /// Whether the entry is a symbolic link, or why that could not be
    /// established. An entry the platform would not describe is UNKNOWN rather
    /// than an ordinary file: a law about symlinks answering "ordinary" about an
    /// entry nobody could stat is the fallback this model exists to delete.
    link: Read<LinkState>,
    /// The bytes, or why they were not read.
    bytes: Read<Vec<u8>>,
    /// The text those bytes decode to, or why they do not.
    text: Read<String>,
}

impl FileFact {
    /// Whether the entry is a symbolic link.
    pub(crate) const fn link(&self) -> &Read<LinkState> {
        &self.link
    }

    /// The bytes, or why they were not read.
    pub(crate) const fn bytes(&self) -> &Read<Vec<u8>> {
        &self.bytes
    }

    /// The text those bytes decode to, or why they do not.
    pub(crate) const fn text(&self) -> &Read<String> {
        &self.text
    }
}

/// Reads one directory into the map, recursing in file-name order.
///
/// The canonical spelling is BUILT on the way down rather than recovered on the
/// way back: each level appends the name it just read to the spelling it was
/// handed. Stripping a root off an absolute path afterwards would be a second
/// derivation of a fact this walk already has, and it would need a fallback for
/// the case it cannot happen in.
fn read_directory(
    directory: &Path,
    inside: &str,
    into: &mut BTreeMap<CanonicalPath, FileFact>,
) -> Result<(), String> {
    let listing = fs::read_dir(directory).map_err(|e| format!("{}: {e}", directory.display()))?;
    let mut found = Vec::new();
    for entry in listing {
        let entry = entry.map_err(|e| format!("{}: {e}", directory.display()))?;
        let path = entry.path();
        let kind = entry
            .file_type()
            .map_err(|e| format!("{}: {e}", path.display()))?;
        found.push((entry.file_name(), path, kind.is_dir()));
    }
    found.sort_by(|(left, _, _), (right, _, _)| left.cmp(right));
    for (name, path, is_directory) in found {
        let named = name.to_string_lossy().into_owned();
        let spelled = if inside.is_empty() {
            named.clone()
        } else {
            format!("{inside}/{named}")
        };
        if is_directory {
            if !UNREAD_DIRECTORIES.contains(&named.as_str()) {
                read_directory(&path, &spelled, into)?;
            }
            continue;
        }
        into.insert(CanonicalPath::spelled(&spelled), read_file(&path));
    }
    Ok(())
}

/// One file's facts.
fn read_file(path: &Path) -> FileFact {
    let spelled = path.display().to_string();
    let link = match fs::symlink_metadata(path) {
        Ok(metadata) => Read::Known(if metadata.file_type().is_symlink() {
            LinkState::Symlink
        } else {
            LinkState::RegularFile
        }),
        Err(error) => Read::Unreadable(ReadFailure::new(&spelled, &error.to_string())),
    };
    let bytes = match fs::read(path) {
        Ok(bytes) => Read::Known(bytes),
        Err(error) => {
            let failure = ReadFailure::new(&spelled, &error.to_string());
            return FileFact {
                link,
                bytes: Read::Unreadable(failure.clone()),
                text: Read::Unreadable(failure),
            };
        }
    };
    let text = match bytes {
        Read::Known(ref bytes) => match std::str::from_utf8(bytes) {
            Ok(text) => Read::Known(text.to_owned()),
            Err(error) => Read::Unreadable(ReadFailure::new(&spelled, &error.to_string())),
        },
        Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
        Read::Unreadable(ref failure) => Read::Unreadable(failure.clone()),
    };
    FileFact { link, bytes, text }
}

/// What git says the reading was taken at.
struct GitSnapshot {
    /// The commit `HEAD` names.
    commit: Read<CommitId>,
    /// The tree that commit names.
    tree: Read<TreeId>,
}

impl GitSnapshot {
    /// Asks git what `HEAD` names, or states that this root is not a checkout.
    ///
    /// It names the COMMITTED state the run started from, and deliberately not
    /// the bytes that were read: the files map is what was read. Naming both is
    /// what lets a log say which tree a green verdict was about — a campaign has
    /// already produced one false green from a restore that preserved a
    /// modification time, and a run that prints the commit it judged is a run
    /// that cannot be confused with a different one.
    fn read(root: &Path) -> Self {
        if !root.join(".git").exists() {
            return Self {
                commit: Read::DeclaredAbsent(AbsenceReason::NotAGitCheckout),
                tree: Read::DeclaredAbsent(AbsenceReason::NotAGitCheckout),
            };
        }
        Self {
            commit: revision(root, "HEAD").map(CommitId),
            tree: revision(root, "HEAD^{tree}").map(TreeId),
        }
    }
}

/// The object one revision names.
fn revision(root: &Path, spelling: &str) -> Read<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["rev-parse", spelling])
        .stderr(Stdio::piped())
        .output();
    let output = match output {
        Ok(output) => output,
        Err(error) => {
            return Read::Unreadable(ReadFailure::new(
                &format!("git rev-parse {spelling}"),
                &error.to_string(),
            ));
        }
    };
    if !output.status.success() {
        return Read::Unreadable(ReadFailure::new(
            &format!("git rev-parse {spelling}"),
            String::from_utf8_lossy(&output.stderr).trim(),
        ));
    }
    Read::Known(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// The commit a reading was taken at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommitId(String);

impl fmt::Display for CommitId {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(&self.0)
    }
}

/// The tree a commit names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TreeId(String);

impl fmt::Display for TreeId {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(&self.0)
    }
}

/// The workspace root: the parent of the xtask crate directory.
pub(crate) fn repo_root() -> Result<PathBuf, Box<dyn Error>> {
    let manifest_directory = Path::new(env!("CARGO_MANIFEST_DIR"));
    let parent = manifest_directory
        .parent()
        .ok_or("xtask crate directory has no parent")?;
    Ok(parent.to_path_buf())
}

/// The cargo binary a spawned stage or reading is given.
///
/// Cargo sets `CARGO` for every process it starts, so a nested invocation
/// reaches the exact binary that started this one — the pinned toolchain's
/// cargo, not whatever a machine's search path resolves today. The fallback
/// covers the case where the xtask binary is run directly, where no pin has
/// been resolved and the search path is all there is.
pub(crate) fn cargo_binary() -> OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"))
}

/// The reading of the real repository, built once for the whole test binary.
///
/// Every law that judges the real tree is proven against THIS reading rather
/// than against one of its own, for the reason the model exists: two readings of
/// one tree are two trees a law can be judging.
#[cfg(test)]
pub(crate) fn repository_snapshot() -> Result<&'static RepositorySnapshot, String> {
    use std::cell::RefCell;
    // Once per test THREAD rather than once per binary: a parsed Rust tree is
    // not `Sync`, because `proc-macro2` holds its tokens behind a reference
    // count that is not atomic. One reading per thread is the sharing this model
    // asks for — no two laws on one thread can be judging two trees — and it is
    // the strongest sharing the parsed tree's own type admits.
    thread_local! {
        static READ: RefCell<Option<&'static RepositorySnapshot>> =
            const { RefCell::new(None) };
    }
    READ.with(|held| {
        if let Some(already) = *held.borrow() {
            return Ok(already);
        }
        let root = repo_root().map_err(|error| error.to_string())?;
        let built: &'static RepositorySnapshot =
            Box::leak(Box::new(RepositorySnapshot::read(&root)?));
        *held.borrow_mut() = Some(built);
        Ok(built)
    })
}

/// Planted reversals for the reading itself.
#[cfg(test)]
mod tests {
    use super::repository_snapshot;
    use crate::repository::types::Read;

    /// The reading names what it read.
    ///
    /// A run that cannot say which commit it judged is a run whose green cannot
    /// be attached to a tree, and this campaign has already produced one false
    /// green from a restore that preserved a modification time.
    #[test]
    fn the_reading_names_the_commit_it_was_taken_at() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let commit = snapshot.commit().required("the commit HEAD names")?;
        assert_eq!(commit.to_string().len(), 40, "{commit}");
        let tree = snapshot.tree().required("the tree HEAD names")?;
        assert_eq!(tree.to_string().len(), 40, "{tree}");
        Ok(())
    }

    /// A file the tree does not carry is ABSENT, and absent is not empty.
    ///
    /// Planted reversal for every fallback this model deleted. The reading used
    /// to answer a missing manifest with an empty string, and an empty manifest
    /// declares no prohibited edge — a law reporting clean about bytes nobody
    /// opened.
    #[test]
    fn a_path_the_tree_does_not_carry_is_absent_rather_than_empty() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        assert!(matches!(
            snapshot.files().text("no/such/file.md"),
            Read::DeclaredAbsent(_)
        ));
        assert!(
            snapshot
                .files()
                .text("Cargo.toml")
                .required("the root manifest")?
                .contains("[workspace]"),
            "the root manifest was not read"
        );
        Ok(())
    }
}
