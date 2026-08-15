//! One committed reading of the repository, built once, consumed by every law.
//!
//! This module orchestrates the aggregate read and owns the Git processes that
//! derive committed membership and bytes. The Cargo reader owns its separate
//! live Cargo process. Everything downstream is a pure function over the facts
//! those role-distinct readers established.
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
//! Every fallible file projection is a [`Read`], so a law either handles the
//! absence or is refused by [`Read::required`]; there is no method in this crate
//! that turns an unread fact into a value.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::io::{BufRead, BufReader, Read as IoRead, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::repository::cargo::{CargoObservation, CargoSnapshot};
use crate::repository::markdown::MarkdownSnapshot;
use crate::repository::rust::RustSyntaxSnapshot;
use crate::repository::types::{AbsenceReason, CanonicalPath, LinkState, Read, ReadFailure};

/// Git's own storage coordinate at a checkout root.
const GIT_STORAGE: &str = ".git";

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
    /// Every blob in the committed Git tree, read once, with its bytes and text.
    files: CanonicalFileMap,
    /// What the committed TOML bytes declare.
    cargo: CargoSnapshot,
    /// What a live Cargo process reported beside this committed reading.
    cargo_observation: CargoObservation,
    /// Every Rust source, parsed once.
    rust: RustSyntaxSnapshot,
    /// Every Markdown document, parsed once.
    markdown: MarkdownSnapshot,
    /// The commit and tree that own every byte in `files`.
    committed: CommittedTree,
}

impl RepositorySnapshot {
    /// Reads the repository at one root.
    ///
    /// Git owns both membership and bytes: `ls-tree -z` derives the exact blob
    /// population and `cat-file --batch` reads those immutable objects. Ignored
    /// and untracked filesystem entries therefore cannot enter this type. A
    /// tracked checkout difference or a moving `HEAD` refuses construction.
    ///
    /// Cargo metadata remains a live observation. The committed state and
    /// tracked-clean posture are checked again after that process so a run does
    /// not join a stable committed projection to an observation taken while the
    /// checkout moved.
    pub(crate) fn read(root: &Path) -> Result<Self, String> {
        Self::read_with_after_files(root, || Ok(()))
    }

    /// The aggregate read with one private post-file-map observation seam.
    ///
    /// Production supplies a no-op. The snapshot's own planted reversal moves
    /// `HEAD` here, after immutable bytes were read and before the second state
    /// check, so deleting or misplacing the builder check makes that reversal
    /// fail rather than leaving a helper-only test green.
    fn read_with_after_files(
        root: &Path,
        after_files: impl FnOnce() -> Result<(), String>,
    ) -> Result<Self, String> {
        Self::read_with_hooks(root, || Ok(()), after_files)
    }

    /// The aggregate read with deterministic seams around state and bytes.
    fn read_with_hooks(
        root: &Path,
        after_initial_commit: impl FnOnce() -> Result<(), String>,
        after_files: impl FnOnce() -> Result<(), String>,
    ) -> Result<Self, String> {
        let before = committed_tree_with_after_commit(root, after_initial_commit)?;
        require_tracked_clean(root)?;
        let files = CanonicalFileMap::read(root, &before.tree)?;
        after_files()?;
        require_same_committed_state(&before, &committed_tree(root)?)?;
        require_tracked_clean(root)?;
        let cargo = CargoSnapshot::read(&files);
        let rust = RustSyntaxSnapshot::read(&files);
        let markdown = MarkdownSnapshot::read(&files);
        let cargo_observation = CargoObservation::read(root, &files);
        require_same_committed_state(&before, &committed_tree(root)?)?;
        require_tracked_clean(root)?;
        Ok(Self {
            files,
            cargo,
            cargo_observation,
            rust,
            markdown,
            committed: before,
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

    /// What Cargo reported live beside this committed snapshot.
    pub(crate) const fn cargo_observation(&self) -> &CargoObservation {
        &self.cargo_observation
    }

    /// Every Rust source, parsed.
    pub(crate) const fn rust(&self) -> &RustSyntaxSnapshot {
        &self.rust
    }

    /// Every Markdown document, parsed.
    pub(crate) const fn markdown(&self) -> &MarkdownSnapshot {
        &self.markdown
    }

    /// The commit and tree that own every byte this snapshot carries.
    pub(crate) const fn committed(&self) -> &CommittedTree {
        &self.committed
    }
}

/// Every file in the tree, keyed by the one spelling this repository uses.
pub(crate) struct CanonicalFileMap {
    /// Ordered by canonical path, so every traversal — and so every diagnostic
    /// — is the same on every machine and every run.
    entries: BTreeMap<CanonicalPath, FileFact>,
}

impl CanonicalFileMap {
    /// Reads the exact blob population of one immutable Git tree.
    fn read(root: &Path, tree: &TreeId) -> Result<Self, String> {
        let tracked = tracked_blobs(root, tree)?;
        let entries = read_tracked_blobs(root, &tracked)?;
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
    /// Whether the committed Git mode names a symbolic link. An unsupported
    /// mode refuses construction instead of arriving here as an ordinary file.
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

/// One blob Git lists in a committed tree.
struct TrackedBlob {
    /// Git's canonical root-relative path spelling.
    path: CanonicalPath,
    /// The blob object that owns the bytes.
    object: String,
    /// Whether the tree entry is a regular file or a symbolic link.
    link: LinkState,
}

/// Derives the committed file population from Git's NUL-delimited tree format.
fn tracked_blobs(root: &Path, tree: &TreeId) -> Result<Vec<TrackedBlob>, String> {
    let output = git(root)
        .args(["ls-tree", "-rz", "--full-tree"])
        .arg(tree.to_string())
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| format!("git ls-tree: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git ls-tree refused the committed population: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    parse_tracked_blobs(&output.stdout)
}

/// Parses exactly the records `git ls-tree -rz` emits.
fn parse_tracked_blobs(output: &[u8]) -> Result<Vec<TrackedBlob>, String> {
    if output.last().is_some_and(|last| *last != 0) {
        return Err(String::from(
            "git ls-tree did not terminate its final path record with NUL",
        ));
    }
    if output.windows(2).any(|pair| pair == b"\0\0") {
        return Err(String::from("git ls-tree emitted an empty path record"));
    }
    let mut tracked = Vec::new();
    let mut seen = BTreeSet::new();
    for record in output
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
    {
        let (header, raw_path) = split_once_byte(record, b'\t').ok_or_else(|| {
            format!(
                "git ls-tree emitted a record without its path separator: `{}`",
                String::from_utf8_lossy(record)
            )
        })?;
        let mut fields = header.split(|byte| *byte == b' ');
        let mode = ascii_field(fields.next(), "mode")?;
        let kind = ascii_field(fields.next(), "object kind")?;
        let object = ascii_field(fields.next(), "object identity")?;
        if fields.next().is_some() {
            return Err(format!(
                "git ls-tree emitted extra header fields for `{}`",
                String::from_utf8_lossy(raw_path)
            ));
        }
        if kind != "blob" {
            return Err(format!(
                "git tree entry `{}` is a `{kind}` rather than a blob; committed snapshots do not \
                 invent file semantics for Git links or unknown object kinds",
                String::from_utf8_lossy(raw_path)
            ));
        }
        let link = match mode {
            "100644" | "100755" => LinkState::RegularFile,
            "120000" => LinkState::Symlink,
            other => {
                return Err(format!(
                    "git tree entry `{}` carries unsupported blob mode `{other}`",
                    String::from_utf8_lossy(raw_path)
                ));
            }
        };
        let spelled = std::str::from_utf8(raw_path).map_err(|error| {
            format!(
                "git tracks a path with no Unicode spelling (`{}`): {error}; every canonical path \
                 is a join key against repository text, so lossy conversion cannot be identity",
                String::from_utf8_lossy(raw_path)
            )
        })?;
        if spelled.contains('\\') {
            return Err(format!(
                "git tracks `{spelled}` with a literal backslash; canonical repository paths use \
                 forward slashes, so replacing that byte would collapse two identities"
            ));
        }
        let path = CanonicalPath::spelled(spelled);
        if !seen.insert(path.clone()) {
            return Err(format!("git listed committed path `{path}` more than once"));
        }
        tracked.push(TrackedBlob {
            path,
            object: object.to_owned(),
            link,
        });
    }
    Ok(tracked)
}

/// Reads every committed blob through one batch process.
fn read_tracked_blobs(
    root: &Path,
    tracked: &[TrackedBlob],
) -> Result<BTreeMap<CanonicalPath, FileFact>, String> {
    let mut child = git(root)
        .args(["cat-file", "--batch"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("git cat-file --batch: {error}"))?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| String::from("git cat-file opened no batch input"))?;
    let output = child
        .stdout
        .take()
        .ok_or_else(|| String::from("git cat-file opened no batch output"))?;
    let mut output = BufReader::new(output);
    let mut entries = BTreeMap::new();
    for entry in tracked {
        writeln!(input, "{}", entry.object)
            .map_err(|error| format!("git cat-file batch input: {error}"))?;
        input
            .flush()
            .map_err(|error| format!("git cat-file batch input flush: {error}"))?;
        let mut header = Vec::new();
        output
            .read_until(b'\n', &mut header)
            .map_err(|error| format!("git cat-file batch header: {error}"))?;
        if header.last() == Some(&b'\n') {
            header.pop();
        }
        let header = std::str::from_utf8(&header)
            .map_err(|error| format!("git cat-file emitted a non-UTF-8 header: {error}"))?;
        let mut fields = header.split(' ');
        let reported_object = fields.next().unwrap_or_default();
        let kind = fields.next().unwrap_or_default();
        let size = fields
            .next()
            .ok_or_else(|| format!("git cat-file emitted malformed header `{header}`"))?
            .parse::<usize>()
            .map_err(|error| {
                format!("git cat-file emitted malformed size in `{header}`: {error}")
            })?;
        if fields.next().is_some() || reported_object != entry.object || kind != "blob" {
            return Err(format!(
                "git cat-file reported `{header}` while `{}` was requested for `{}`",
                entry.object, entry.path
            ));
        }
        let mut bytes = vec![0_u8; size];
        output.read_exact(&mut bytes).map_err(|error| {
            format!("git blob `{}` for `{}`: {error}", entry.object, entry.path)
        })?;
        let mut terminator = [0_u8; 1];
        output
            .read_exact(&mut terminator)
            .map_err(|error| format!("git blob `{}` terminator: {error}", entry.object))?;
        if terminator.as_slice() != b"\n" {
            return Err(format!(
                "git cat-file did not terminate blob `{}` with its protocol newline",
                entry.object
            ));
        }
        let text = match std::str::from_utf8(&bytes) {
            Ok(text) => Read::Known(text.to_owned()),
            Err(error) => {
                Read::Unreadable(ReadFailure::new(entry.path.as_str(), &error.to_string()))
            }
        };
        entries.insert(
            entry.path.clone(),
            FileFact {
                link: Read::Known(entry.link),
                bytes: Read::Known(bytes),
                text,
            },
        );
    }
    drop(input);
    let mut trailing = Vec::new();
    output
        .read_to_end(&mut trailing)
        .map_err(|error| format!("git cat-file trailing output: {error}"))?;
    let completed = child
        .wait_with_output()
        .map_err(|error| format!("git cat-file completion: {error}"))?;
    if !completed.status.success() {
        return Err(format!(
            "git cat-file refused committed blobs: {}",
            String::from_utf8_lossy(&completed.stderr).trim()
        ));
    }
    if !trailing.is_empty() {
        return Err(format!(
            "git cat-file emitted {} unexpected trailing byte(s)",
            trailing.len()
        ));
    }
    Ok(entries)
}

/// Splits one byte slice at its first named byte.
fn split_once_byte(bytes: &[u8], separator: u8) -> Option<(&[u8], &[u8])> {
    let at = bytes.iter().position(|byte| *byte == separator)?;
    let (before, from_separator) = bytes.split_at(at);
    Some((before, from_separator.get(1..)?))
}

/// One ASCII field from Git's machine-readable header.
fn ascii_field<'field>(field: Option<&'field [u8]>, named: &str) -> Result<&'field str, String> {
    let field = field.ok_or_else(|| format!("git ls-tree omitted its {named}"))?;
    std::str::from_utf8(field).map_err(|error| format!("git ls-tree {named} is not ASCII: {error}"))
}

/// The committed state git names at one moment: a commit and the tree it names.
///
/// The two are read as ONE fact rather than as two readings side by side. A
/// commit whose tree could not be read is not half a committed state — it is a
/// state nothing can be bound to, and carrying the halves separately left every
/// consumer to decide what a half meant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommittedTree {
    /// The commit `HEAD` names.
    commit: CommitId,
    /// The tree that commit names.
    tree: TreeId,
}

impl fmt::Display for CommittedTree {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "commit {} (tree {})", self.commit, self.tree)
    }
}

/// Asks Git what `HEAD` names, refusing a root with no committed state.
fn committed_tree(root: &Path) -> Result<CommittedTree, String> {
    committed_tree_with_after_commit(root, || Ok(()))
}

/// Captures one commit/tree pair with a private deterministic challenge seam.
fn committed_tree_with_after_commit(
    root: &Path,
    after_commit: impl FnOnce() -> Result<(), String>,
) -> Result<CommittedTree, String> {
    if !root.join(GIT_STORAGE).exists() {
        return Err(format!(
            "{} is not a Git checkout, so no committed repository snapshot can be read",
            root.display()
        ));
    }
    let commit = CommitId(revision(root, "HEAD")?);
    after_commit()?;
    let tree = TreeId(revision(root, &format!("{}^{{tree}}", commit.0))?);
    Ok(CommittedTree { commit, tree })
}

/// Requires every tracked checkout path to match `HEAD`.
///
/// Untracked and ignored paths are deliberately excluded rather than refused:
/// membership and bytes come from the committed tree, so neither population can
/// enter the snapshot. The `-z` porcelain contract is used so a tracked path
/// containing a newline cannot hide by changing record boundaries.
fn require_tracked_clean(root: &Path) -> Result<(), String> {
    let output = git(root)
        .args(["status", "--porcelain=v2", "-z", "--untracked-files=no"])
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| format!("git status --porcelain=v2: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git could not establish tracked checkout cleanliness: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    if output.stdout.is_empty() {
        Ok(())
    } else {
        Err(String::from(
            "tracked checkout bytes differ from HEAD; a committed snapshot refuses rather than \
             report about different live bytes beside the committed tree",
        ))
    }
}

/// Refuses a read whose committed state moved at any point in the operation.
fn require_same_committed_state(
    before: &CommittedTree,
    after: &CommittedTree,
) -> Result<(), String> {
    if before == after {
        Ok(())
    } else {
        Err(format!(
            "the committed state moved while the repository was being read: Git named {before} \
             before the read and {after} after it"
        ))
    }
}

/// The object one revision names.
fn revision(root: &Path, spelling: &str) -> Result<String, String> {
    let output = git(root)
        .args(["rev-parse", spelling])
        .stderr(Stdio::piped())
        .output()
        .map_err(|error| format!("git rev-parse {spelling}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git rev-parse {spelling}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let reported = std::str::from_utf8(&output.stdout)
        .map_err(|error| format!("git rev-parse {spelling} emitted non-UTF-8 output: {error}"))?;
    Ok(reported.trim().to_owned())
}

/// One Git command with replacement-object rewriting disabled.
fn git(root: &Path) -> Command {
    let mut command = Command::new("git");
    command.current_dir(root).env("GIT_NO_REPLACE_OBJECTS", "1");
    command
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
    use std::collections::BTreeSet;
    use std::fs;
    use std::io::Write as _;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{GIT_STORAGE, RepositorySnapshot, parse_tracked_blobs};
    use crate::checks::hygiene::check_lf_and_no_symlinks;
    use crate::repository::types::{LinkState, Read};

    /// One isolated Git repository for a committed-snapshot control.
    struct GitFixture {
        root: PathBuf,
    }

    impl GitFixture {
        /// Creates a repository with deterministic local identity and byte rules.
        fn named(name: &str) -> Result<Self, String> {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "threadpak-snapshot-{}-{ordinal}-{name}",
                std::process::id()
            ));
            let _removed = fs::remove_dir_all(&root);
            fs::create_dir_all(&root).map_err(|error| format!("{}: {error}", root.display()))?;
            let fixture = Self { root };
            fixture.git(&["init", "--quiet"])?;
            fixture.git(&["config", "user.name", "ThreadPak snapshot fixture"])?;
            fixture.git(&["config", "user.email", "fixture@threadpak.invalid"])?;
            fixture.git(&["config", "core.autocrlf", "false"])?;
            fixture.git(&["config", "core.symlinks", "false"])?;
            Ok(fixture)
        }

        /// Writes one fixture path with exact bytes.
        fn write(&self, relative: &str, bytes: &[u8]) -> Result<(), String> {
            let path = self.root.join(relative);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| format!("{}: {error}", parent.display()))?;
            }
            fs::write(&path, bytes).map_err(|error| format!("{}: {error}", path.display()))
        }

        /// Commits the fixture's current tracked population.
        fn commit(&self) -> Result<(), String> {
            self.git(&["add", "-A"])?;
            self.commit_index()
        }

        /// Commits the index without restaging the working tree.
        fn commit_index(&self) -> Result<(), String> {
            self.git(&[
                "commit",
                "--quiet",
                "--allow-empty",
                "--message",
                "snapshot fixture",
            ])
        }

        /// Stages one symbolic-link tree entry without asking the host to mint it.
        fn stage_symlink(&self, relative: &str, target: &str) -> Result<(), String> {
            let mut child = Command::new("git")
                .current_dir(&self.root)
                .env("GIT_NO_REPLACE_OBJECTS", "1")
                .args(["hash-object", "-w", "--stdin"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|error| format!("git hash-object: {error}"))?;
            child
                .stdin
                .take()
                .ok_or_else(|| String::from("git hash-object opened no input"))?
                .write_all(target.as_bytes())
                .map_err(|error| format!("git hash-object input: {error}"))?;
            let output = child
                .wait_with_output()
                .map_err(|error| format!("git hash-object completion: {error}"))?;
            if !output.status.success() {
                return Err(format!(
                    "git hash-object refused: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                ));
            }
            let object = std::str::from_utf8(&output.stdout)
                .map_err(|error| format!("git hash-object emitted non-UTF-8 output: {error}"))?
                .trim();
            let cache = format!("120000,{object},{relative}");
            self.git(&["update-index", "--add", "--cacheinfo", &cache])
        }

        /// Runs one Git operation and carries its refusal.
        fn git(&self, arguments: &[&str]) -> Result<(), String> {
            run_git(&self.root, arguments)
        }

        /// Reads the committed snapshot.
        fn snapshot(&self) -> Result<RepositorySnapshot, String> {
            RepositorySnapshot::read(&self.root)
        }
    }

    impl Drop for GitFixture {
        fn drop(&mut self) {
            let _removed = fs::remove_dir_all(&self.root);
        }
    }

    /// Runs one Git command at a named root.
    fn run_git(root: &Path, arguments: &[&str]) -> Result<(), String> {
        let output = Command::new("git")
            .current_dir(root)
            .env("GIT_NO_REPLACE_OBJECTS", "1")
            .args(arguments)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|error| format!("git {}: {error}", arguments.join(" ")))?;
        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "git {} refused: {}",
                arguments.join(" "),
                String::from_utf8_lossy(&output.stderr).trim()
            ))
        }
    }

    /// The committed population and bytes, in stable order.
    fn facts(snapshot: &RepositorySnapshot) -> Result<Vec<(String, Vec<u8>, LinkState)>, String> {
        snapshot
            .files()
            .iter()
            .map(|(path, fact)| {
                Ok((
                    path.as_str().to_owned(),
                    fact.bytes().required(path.as_str())?.clone(),
                    *fact.link().required(path.as_str())?,
                ))
            })
            .collect()
    }

    /// Positive control: committed files are read from their Git blobs.
    #[test]
    fn a_clean_committed_tree_is_read_from_git() -> Result<(), String> {
        let fixture = GitFixture::named("clean")?;
        fixture.write("nested/file.txt", b"committed bytes\n")?;
        fixture.commit()?;
        let snapshot = fixture.snapshot()?;
        assert_eq!(
            snapshot
                .files()
                .bytes("nested/file.txt")
                .required("nested/file.txt")?,
            b"committed bytes\n"
        );
        assert!(!snapshot.committed().to_string().is_empty());
        Ok(())
    }

    /// A large committed roster completes through the one-request/one-response
    /// protocol. Writing every request before reading any response can fill the
    /// two pipes in opposite directions and never reach a verdict.
    #[test]
    fn a_many_blob_roster_completes_without_cross_pipe_blocking() -> Result<(), String> {
        let fixture = GitFixture::named("many-blobs")?;
        let body = vec![b'x'; 1_024];
        let mut expected = BTreeSet::new();
        for ordinal in 0..2_048_u16 {
            let path = format!("many/{ordinal:04}.txt");
            fixture.write(&path, &body)?;
            expected.insert(path);
        }
        fixture.commit()?;

        let snapshot = fixture.snapshot()?;
        let found: BTreeSet<_> = snapshot
            .files()
            .iter()
            .map(|(path, _)| path.as_str().to_owned())
            .collect();
        assert_eq!(found, expected);
        Ok(())
    }

    /// Git's symlink mode reaches the existing no-symlink law on every host.
    #[test]
    fn a_committed_symlink_mode_reaches_the_no_symlink_law() -> Result<(), String> {
        let fixture = GitFixture::named("symlink-mode")?;
        fixture.stage_symlink("link", "target.txt")?;
        fixture.commit_index()?;
        fixture.git(&["checkout", "--", "link"])?;
        let snapshot = fixture.snapshot()?;
        assert!(matches!(
            snapshot.files().get("link").map(super::FileFact::link),
            Some(Read::Known(LinkState::Symlink))
        ));
        let found = check_lf_and_no_symlinks(&snapshot);
        assert!(
            found.is_err_and(|refusal| refusal.contains("symlink: link")),
            "Git mode 120000 did not activate the no-symlink law"
        );
        Ok(())
    }

    /// Moving `HEAD` inside the aggregate read refuses the constructor.
    #[test]
    fn a_moving_committed_state_refuses() -> Result<(), String> {
        let fixture = GitFixture::named("moving-head")?;
        fixture.write("first.txt", b"first\n")?;
        fixture.commit()?;
        let found = RepositorySnapshot::read_with_after_files(&fixture.root, || {
            fixture.write("second.txt", b"second\n")?;
            fixture.commit()
        });
        assert!(
            found.is_err_and(|refusal| refusal.contains("committed state moved")),
            "the aggregate builder accepted a read spanning two committed states"
        );
        Ok(())
    }

    /// Moving `HEAD` between commit and tree capture cannot mint a mixed pair.
    #[test]
    fn commit_and_tree_capture_remain_one_aggregate_fact() -> Result<(), String> {
        let fixture = GitFixture::named("mixed-pair")?;
        fixture.write("first.txt", b"first\n")?;
        fixture.commit()?;
        let first = super::committed_tree(&fixture.root)?;
        fixture.write("second.txt", b"second\n")?;
        fixture.commit()?;
        let second = super::committed_tree(&fixture.root)?;
        fixture.git(&["reset", "--hard", &first.commit.to_string()])?;

        let found = RepositorySnapshot::read_with_hooks(
            &fixture.root,
            || fixture.git(&["reset", "--hard", &second.commit.to_string()]),
            || Ok(()),
        );
        let refusal = found
            .err()
            .ok_or_else(|| String::from("a state moving during aggregate capture was accepted"))?;
        assert!(
            refusal.contains(&format!("Git named {first} before the read"))
                && refusal.contains(&format!("and {second} after it")),
            "the aggregate refusal carried a mixed commit/tree pair: {refusal}"
        );
        Ok(())
    }

    /// A tracked checkout difference refuses the committed constructor.
    #[test]
    fn tracked_dirt_refuses_the_committed_snapshot() -> Result<(), String> {
        let fixture = GitFixture::named("dirty")?;
        fixture.write("tracked.txt", b"committed\n")?;
        fixture.commit()?;
        fixture.write("tracked.txt", b"different\n")?;
        let found = fixture.snapshot();
        assert!(
            found.is_err_and(|refusal| refusal.contains("tracked checkout bytes differ")),
            "tracked dirt entered a committed snapshot"
        );
        Ok(())
    }

    /// Root and nested ignored bytes are outside the committed population.
    #[test]
    fn ignored_files_never_enter_the_committed_snapshot() -> Result<(), String> {
        let fixture = GitFixture::named("ignored")?;
        fixture.write(".gitignore", b"root.ignored\nnested/ignored/\n")?;
        fixture.write("tracked.txt", b"tracked\n")?;
        fixture.commit()?;
        fixture.write("root.ignored", b"ambient root\n")?;
        fixture.write("nested/ignored/file.txt", b"ambient nested\n")?;
        let snapshot = fixture.snapshot()?;
        assert!(snapshot.files().get("root.ignored").is_none());
        assert!(snapshot.files().get("nested/ignored/file.txt").is_none());
        assert!(snapshot.files().get("tracked.txt").is_some());
        Ok(())
    }

    /// Untracked nonignored bytes are explicitly outside, not a hidden member.
    #[test]
    fn untracked_files_are_outside_the_committed_snapshot() -> Result<(), String> {
        let fixture = GitFixture::named("untracked")?;
        fixture.write("tracked.txt", b"tracked\n")?;
        fixture.commit()?;
        fixture.write("untracked.txt", b"ambient\n")?;
        let snapshot = fixture.snapshot()?;
        assert!(snapshot.files().get("untracked.txt").is_none());
        assert!(snapshot.files().get("tracked.txt").is_some());
        Ok(())
    }

    /// A non-Unicode Git path refuses instead of collapsing through lossy text.
    #[test]
    fn a_non_unicode_git_path_refuses() {
        let found = parse_tracked_blobs(b"100644 blob abcdef\tbad\xffname\0");
        assert!(
            found.is_err_and(|refusal| refusal.contains("no Unicode spelling")),
            "a non-Unicode tracked path gained a canonical identity"
        );
    }

    /// A clone and linked worktree of one tree have one canonical population.
    #[test]
    fn clone_and_linked_worktree_read_identically() -> Result<(), String> {
        let fixture = GitFixture::named("checkout-shapes")?;
        fixture.write("a.txt", b"a\n")?;
        fixture.write("nested/b.txt", b"b\n")?;
        fixture.commit()?;
        let parent = fixture
            .root
            .parent()
            .ok_or_else(|| String::from("fixture root has no parent"))?;
        let suffix = fixture
            .root
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| String::from("fixture root has no Unicode name"))?;
        let clone = parent.join(format!("{suffix}-clone"));
        let worktree = parent.join(format!("{suffix}-worktree"));
        let _clone_cleanup = fs::remove_dir_all(&clone);
        let _worktree_removed = fs::remove_dir_all(&worktree);
        let clone_output = Command::new("git")
            .args(["clone", "--quiet"])
            .arg(&fixture.root)
            .arg(&clone)
            .output()
            .map_err(|error| format!("git clone: {error}"))?;
        if !clone_output.status.success() {
            return Err(format!(
                "git clone refused: {}",
                String::from_utf8_lossy(&clone_output.stderr).trim()
            ));
        }
        let worktree_output = Command::new("git")
            .current_dir(&fixture.root)
            .args(["worktree", "add", "--quiet", "--detach"])
            .arg(&worktree)
            .arg("HEAD")
            .output()
            .map_err(|error| format!("git worktree add: {error}"))?;
        if !worktree_output.status.success() {
            return Err(format!(
                "git worktree add refused: {}",
                String::from_utf8_lossy(&worktree_output.stderr).trim()
            ));
        }
        let source_facts = facts(&fixture.snapshot()?)?;
        let clone_facts = facts(&RepositorySnapshot::read(&clone)?)?;
        let worktree_snapshot = RepositorySnapshot::read(&worktree)?;
        let worktree_facts = facts(&worktree_snapshot)?;
        assert_eq!(source_facts, clone_facts);
        assert_eq!(source_facts, worktree_facts);
        assert!(worktree_snapshot.files().get(GIT_STORAGE).is_none());
        let _removed = Command::new("git")
            .current_dir(&fixture.root)
            .args(["worktree", "remove", "--force"])
            .arg(&worktree)
            .output();
        let _clone_removed = fs::remove_dir_all(&clone);
        Ok(())
    }

    /// Missing committed paths remain declared absent rather than empty.
    #[test]
    fn a_missing_committed_path_is_absent() -> Result<(), String> {
        let fixture = GitFixture::named("absent")?;
        fixture.write("present.txt", b"present\n")?;
        fixture.commit()?;
        let snapshot = fixture.snapshot()?;
        assert!(matches!(
            snapshot.files().text("missing.txt"),
            Read::DeclaredAbsent(_)
        ));
        Ok(())
    }

    /// The NUL roster keeps a newline-bearing path inside one record.
    #[test]
    fn tracked_population_is_nul_delimited() -> Result<(), String> {
        let parsed = parse_tracked_blobs(b"100644 blob abcdef\tline\nname.txt\0")?;
        assert_eq!(parsed.len(), 1);
        assert_eq!(
            parsed.first().map(|entry| entry.path.as_str()),
            Some("line\nname.txt")
        );
        Ok(())
    }

    /// A line-terminated roster cannot impersonate Git's NUL contract.
    #[test]
    fn an_unterminated_tracked_record_refuses() {
        let found = parse_tracked_blobs(b"100644 blob abcdef\tpath.txt\n");
        assert!(
            found.is_err_and(|refusal| refusal.contains("final path record with NUL")),
            "a non-NUL Git population was accepted"
        );
    }
}
