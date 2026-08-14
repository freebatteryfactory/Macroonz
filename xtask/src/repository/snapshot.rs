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
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::repository::cargo::CargoSnapshot;
use crate::repository::markdown::MarkdownSnapshot;
use crate::repository::rust::RustSyntaxSnapshot;
use crate::repository::types::{AbsenceReason, CanonicalPath, LinkState, Read, ReadFailure};

/// Git's own storage, AT THE REPOSITORY ROOT.
///
/// A directory in an ordinary clone and a FILE in a git worktree, where it
/// carries one line naming where the real storage lives. Both are git's, and
/// both are excluded here — otherwise one committed tree yields two different
/// populations depending on how the checkout was made, and every agent on this
/// campaign works in a worktree, so that is the live case rather than the exotic
/// one. It was live: the worktree's `.git` FILE was in the map, because the
/// exclusion asked whether an entry was a DIRECTORY before it asked what it was
/// called.
const GIT_STORAGE: &str = ".git";

/// The build's output directory, AT THE REPOSITORY ROOT.
///
/// Cargo's output for this workspace, which is not repository material and is
/// large enough that walking it would make every run pay for bytes no law is
/// about. Only the root one: `target` is an ordinary word, and excluding it by
/// BASENAME at every depth silently deleted `src/<home>/target/`,
/// `docs/target/`, and `testpak/target/` from a population no law would have
/// reported missing. A file called `target` is not a build directory and is
/// read like any other file.
const BUILD_OUTPUT: &str = "target";

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
    /// What the bytes read are bound to, established AROUND the read rather
    /// than asked afterwards.
    binding: CommitBinding,
}

impl RepositorySnapshot {
    /// Reads the repository at one root.
    ///
    /// The order is the dependency order of the readings, and the git readings
    /// BRACKET the walk rather than following it. The walk used to happen first
    /// and git was asked afterwards, so a run printed a commit-bound sentence
    /// about bytes it had never compared to that commit: on a dirty tree the
    /// sentence named a commit whose content was not what had been read, and a
    /// commit that moved mid-walk left the reading a mixture of two trees with
    /// nothing saying so. Now git is asked before the walk and again after it,
    /// the checkout is asked what differs, and [`CommitBinding::establish`]
    /// either names the committed tree these bytes ARE or states what stops
    /// them from being one. A commit that moved between the two readings
    /// refuses the whole reading, because those bytes are about no single tree.
    ///
    /// The decoders come after the binding on purpose. `cargo metadata` starts
    /// a process that writes into the build directory, and a reading that asked
    /// what differs AFTER running it would be asking about a checkout its own
    /// reading had touched.
    pub(crate) fn read(root: &Path) -> Result<Self, String> {
        let before = committed_tree(root);
        let files = CanonicalFileMap::read(root)?;
        let after = committed_tree(root);
        let differences = working_tree_differences(root);
        let binding = CommitBinding::establish(&before, &after, &differences)?;
        let cargo = CargoSnapshot::read(root, &files);
        let rust = RustSyntaxSnapshot::read(&files);
        let markdown = MarkdownSnapshot::read(&files);
        Ok(Self {
            files,
            cargo,
            rust,
            markdown,
            binding,
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

    /// What the bytes this reading carries are bound to.
    pub(crate) const fn binding(&self) -> &CommitBinding {
        &self.binding
    }
}

/// Every file in the tree, keyed by the one spelling this repository uses.
pub(crate) struct CanonicalFileMap {
    /// Ordered by canonical path, so every traversal — and so every diagnostic
    /// — is the same on every machine and every run.
    entries: BTreeMap<CanonicalPath, FileFact>,
}

impl CanonicalFileMap {
    /// Reads every file under one root, entering everything except
    /// [`GIT_STORAGE`] and [`BUILD_OUTPUT`] AT THAT ROOT.
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
///
/// # The two exclusions are ROOT-RELATIVE, and one of them ignores kind
///
/// Being at the root is a fact this walk already has — `inside` is empty there
/// and nowhere else — so the two exclusions are asked exactly where they mean
/// something. Asked by BASENAME at every depth, as they were, `target`
/// disappeared a directory of repository material anywhere in the tree that
/// happened to carry the build's name, and no law would have reported the
/// absence, because a law is about the population it was handed.
///
/// [`GIT_STORAGE`] is excluded whatever KIND the entry is, and that asymmetry is
/// the worktree repair: in a clone it is a directory and the old reading skipped
/// it, in a worktree it is a file and the old reading read it into the map. One
/// committed tree, two populations, decided by how somebody checked it out.
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
    let at_root = inside.is_empty();
    for (name, path, is_directory) in found {
        let named = canonical_name(&name)
            .map_err(|refusal| format!("{}: {refusal}", directory.display()))?;
        if at_root && named == GIT_STORAGE {
            continue;
        }
        if at_root && is_directory && named == BUILD_OUTPUT {
            continue;
        }
        let spelled = if at_root {
            named
        } else {
            format!("{inside}/{named}")
        };
        if is_directory {
            read_directory(&path, &spelled, into)?;
            continue;
        }
        into.insert(CanonicalPath::spelled(&spelled), read_file(&path));
    }
    Ok(())
}

/// One entry name as this repository spells names, or the refusal that says it
/// cannot be spelled at all.
///
/// # Lossy conversion cannot be identity, so this refuses instead
///
/// The walk used to build canonical paths with `to_string_lossy`, which maps
/// every unpaired surrogate and every ill-formed byte onto one replacement
/// character. Two entries whose names differ only where the conversion is lossy
/// therefore produce ONE [`CanonicalPath`], and the second insertion silently
/// overwrites the first: a file leaves the population with no error anywhere,
/// which is the exact silence this model exists to end. A rendering is a thing
/// to show a person; it is not an identity, and it was being used as the join
/// key of every law in this crate.
///
/// Of the two lawful repairs — refuse the path, or key on raw platform bytes and
/// render separately — this repository refuses, and the reason is what the key
/// is FOR. A [`CanonicalPath`] is joined against text somebody wrote: an
/// obligation row naming a route, an allowlist entry, a README's declared
/// member, a band map. Those documents are UTF-8, so a path that cannot be
/// spelled in them is a path no row can name and no join can resolve; keying on
/// platform bytes would mint a second spelling for every path in the tree while
/// leaving the unnameable ones exactly as unjoinable as they are now. Refusing
/// costs a repository that carries such a name one clear refusal naming the
/// directory it is in, and this one carries none.
fn canonical_name(name: &OsStr) -> Result<String, String> {
    match name.to_str() {
        Some(named) => Ok(String::from(named)),
        None => Err(format!(
            "the entry rendering as `{}` is not Unicode, so it has no canonical path spelling. \
             Every path identity in this crate is a join key against text — obligation rows, \
             allowlists, declared members — and a lossy rendering is not an identity: two such \
             names collapse onto one key and one of them leaves the population with nothing \
             saying so",
            name.to_string_lossy()
        )),
    }
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

/// Asks git what `HEAD` names, or states that this root is not a checkout.
///
/// Called TWICE around the walk. What it names is a committed state, which is
/// not the same fact as what was read off the disk — the files map is what was
/// read — and keeping those two apart is the whole of [`CommitBinding`].
fn committed_tree(root: &Path) -> Read<CommittedTree> {
    if !root.join(GIT_STORAGE).exists() {
        return Read::DeclaredAbsent(AbsenceReason::NotAGitCheckout);
    }
    let commit = match revision(root, "HEAD") {
        Read::Known(named) => CommitId(named),
        Read::DeclaredAbsent(reason) => return Read::DeclaredAbsent(reason),
        Read::Unreadable(failure) => return Read::Unreadable(failure),
    };
    let tree = match revision(root, "HEAD^{tree}") {
        Read::Known(named) => TreeId(named),
        Read::DeclaredAbsent(reason) => return Read::DeclaredAbsent(reason),
        Read::Unreadable(failure) => return Read::Unreadable(failure),
    };
    Read::Known(CommittedTree { commit, tree })
}

/// Every path git reports as differing from what is committed, one entry per
/// line of `git status --porcelain`, each carrying git's two status columns.
///
/// Empty output is the entire clean condition — git prints one line per path
/// that differs from `HEAD` or is untracked, and nothing at all when there is
/// none. Nothing here interprets a status column; the lines are carried whole so
/// a refusal can name what differs rather than count it.
///
/// This is a reading of a PROCESS's output rather than of repository text, which
/// is why it is a line reading and why that is not the defect class this crate
/// has been deleting: git's porcelain format is a line-per-path contract, and
/// the only fact taken from it here is how many lines there are.
fn working_tree_differences(root: &Path) -> Read<Vec<String>> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["status", "--porcelain"])
        .stderr(Stdio::piped())
        .output();
    let output = match output {
        Ok(output) => output,
        Err(error) => {
            return Read::Unreadable(ReadFailure::new(
                "git status --porcelain",
                &error.to_string(),
            ));
        }
    };
    if !output.status.success() {
        return Read::Unreadable(ReadFailure::new(
            "git status --porcelain",
            String::from_utf8_lossy(&output.stderr).trim(),
        ));
    }
    Read::Known(
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect(),
    )
}

/// What the bytes one reading carries are bound to.
///
/// Two states, and the second one exists because the first was being CLAIMED
/// without being established. The reading walked the live filesystem, then asked
/// git what `HEAD` named, then printed `read N files at commit X` — a sentence
/// about a relationship between those bytes and that commit which nothing had
/// checked. On a dirty checkout the sentence was simply false, and `cargo xtask
/// check` alone never noticed: the worktree-clean stage that would have caught
/// it runs only under `qualify`, and only at the end.
///
/// A verdict that cannot be attached to a tree is a verdict about nothing in
/// particular. So a reading either establishes the attachment or says out loud
/// that it has none — it never prints a commit it did not bind.
pub(crate) enum CommitBinding {
    /// The bytes ARE this committed tree: git named the same commit and the
    /// same tree on both sides of the walk, and the checkout carried nothing
    /// differing from them.
    Bound(CommittedTree),
    /// The bytes are the working tree's, and this is what stops them from being
    /// a committed tree.
    Unbound(UnboundReason),
}

impl CommitBinding {
    /// The binding two git readings and one checkout reading establish, or the
    /// refusal that says the reading is about no single tree.
    ///
    /// Pure over its three inputs, which is what lets the sentence a run opens
    /// with be proven against fixture readings: a binding that could only be
    /// tested by moving a commit under a running walk would never be tested.
    ///
    /// # What a bound reading establishes, and what it does not
    ///
    /// `git status --porcelain` reports every TRACKED path differing from `HEAD`
    /// and every UNTRACKED path, and reports nothing for a path git IGNORES. The
    /// walk enters every directory except the root's two, so an ignored
    /// directory deeper in the tree would enter the file map without moving this
    /// verdict. This repository ignores exactly one directory, and it is the
    /// root build output the walk already refuses to enter, so the gap is empty
    /// here and is stated rather than left to be discovered. It closes when the
    /// walk's path set comes from git rather than from a filesystem listing —
    /// which is a different reading, not a stricter version of this one.
    fn establish(
        before: &Read<CommittedTree>,
        after: &Read<CommittedTree>,
        differences: &Read<Vec<String>>,
    ) -> Result<Self, String> {
        if before != after {
            return Err(format!(
                "the committed state moved while the repository was being read: git named {} \
                 before the walk and {} after it, so the bytes this reading carries are a mixture \
                 of two trees and no verdict over them is about either one",
                named(before),
                named(after)
            ));
        }
        match *before {
            Read::Known(ref committed) => match *differences {
                Read::Known(ref entries) if entries.is_empty() => {
                    Ok(CommitBinding::Bound(committed.clone()))
                }
                Read::Known(ref entries) => Ok(CommitBinding::Unbound(
                    UnboundReason::WorkingTreeDiffers(entries.len()),
                )),
                Read::DeclaredAbsent(reason) => Ok(CommitBinding::Unbound(
                    UnboundReason::GitSaysNothing(reason),
                )),
                Read::Unreadable(ref failure) => Ok(CommitBinding::Unbound(
                    UnboundReason::GitRefused(failure.clone()),
                )),
            },
            Read::DeclaredAbsent(reason) => Ok(CommitBinding::Unbound(
                UnboundReason::GitSaysNothing(reason),
            )),
            Read::Unreadable(ref failure) => Ok(CommitBinding::Unbound(UnboundReason::GitRefused(
                failure.clone(),
            ))),
        }
    }
}

impl fmt::Display for CommitBinding {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CommitBinding::Bound(ref committed) => write!(
                out,
                "and they are the committed tree at commit {} (tree {})",
                committed.commit, committed.tree
            ),
            CommitBinding::Unbound(ref reason) => write!(
                out,
                "from the WORKING TREE; they are not a committed tree, so nothing this run reports \
                 is bound to a commit: {reason}"
            ),
        }
    }
}

/// Why a reading's bytes are not a committed tree.
///
/// Every variant is a statement somebody can act on: commit something, ask in a
/// checkout, or repair whatever refused. None of them is a shrug, and none of
/// them is an excuse to print a commit anyway.
pub(crate) enum UnboundReason {
    /// The checkout carries paths differing from what is committed.
    WorkingTreeDiffers(usize),
    /// Git declared there is nothing to name here.
    GitSaysNothing(AbsenceReason),
    /// Git was asked and refused.
    GitRefused(ReadFailure),
}

impl fmt::Display for UnboundReason {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UnboundReason::WorkingTreeDiffers(entries) => write!(
                out,
                "git reports {entries} path(s) in this checkout differing from what is committed"
            ),
            UnboundReason::GitSaysNothing(reason) => write!(out, "{reason}"),
            UnboundReason::GitRefused(ref failure) => write!(out, "{failure}"),
        }
    }
}

/// How one read fact is named in a refusal about it.
fn named<T: fmt::Display>(read: &Read<T>) -> String {
    match *read {
        Read::Known(ref fact) => fact.to_string(),
        Read::DeclaredAbsent(reason) => reason.to_string(),
        Read::Unreadable(ref failure) => failure.to_string(),
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
    use std::ffi::{OsStr, OsString};

    use super::{
        BUILD_OUTPUT, CommitBinding, CommitId, CommittedTree, GIT_STORAGE, TreeId, canonical_name,
        repository_snapshot,
    };
    use crate::repository::types::{AbsenceReason, Read, ReadFailure};

    /// One synthetic committed state.
    fn committed(commit: &str, tree: &str) -> Read<CommittedTree> {
        Read::Known(CommittedTree {
            commit: CommitId(String::from(commit)),
            tree: TreeId(String::from(tree)),
        })
    }

    /// One synthetic checkout reading listing the paths that differ.
    fn differing(paths: &[&str]) -> Read<Vec<String>> {
        Read::Known(paths.iter().map(|path| (*path).to_string()).collect())
    }

    /// The reading names what it read, and never names a commit it did not
    /// bind.
    ///
    /// Read against the real tree, which is dirty exactly when somebody is
    /// working in it — so this states the rule that holds in BOTH states rather
    /// than a fact about one of them. A bound reading names a real committed
    /// state; an unbound one accounts for itself and its sentence carries no
    /// commit at all.
    #[test]
    fn the_reading_never_names_a_commit_it_did_not_bind() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let sentence = snapshot.binding().to_string();
        match *snapshot.binding() {
            CommitBinding::Bound(ref committed) => {
                assert_eq!(committed.commit.to_string().len(), 40, "{committed}");
                assert_eq!(committed.tree.to_string().len(), 40, "{committed}");
                assert!(
                    sentence.contains(&committed.commit.to_string()),
                    "{sentence}"
                );
            }
            CommitBinding::Unbound(ref reason) => {
                assert!(
                    sentence.contains("not a committed tree"),
                    "an unbound reading did not say so: {sentence}"
                );
                assert!(
                    !sentence.contains("at commit"),
                    "an unbound reading named a commit anyway: {sentence}"
                );
                assert!(!reason.to_string().is_empty(), "{sentence}");
            }
        }
        Ok(())
    }

    /// Planted reversal: the commit moving under the walk.
    ///
    /// THE failure the bracketing exists for. The reading used to walk the disk
    /// and ask git afterwards, so a commit that moved mid-walk left the file map
    /// carrying bytes from two trees while the sentence named whichever tree
    /// happened to be current when the walk finished. Those bytes are about no
    /// single tree, so the reading refuses rather than picking one.
    #[test]
    fn a_commit_that_moves_under_the_walk_refuses_the_reading() {
        let found = CommitBinding::establish(
            &committed("aaaa", "bbbb"),
            &committed("cccc", "dddd"),
            &differing(&[]),
        );
        assert!(
            found.is_err_and(|refusal| refusal
                .contains("moved while the repository was being read")
                && refusal.contains("aaaa")
                && refusal.contains("cccc")),
            "a reading spanning two trees was accepted"
        );
    }

    /// Planted reversal: a dirty checkout is NOT a commit-bound reading.
    ///
    /// The sentence `cargo xtask check` opens with used to name a commit on any
    /// tree at all, and only `qualify`'s closing stage — which runs last, and
    /// only under `qualify` — would eventually notice. The bytes read on a dirty
    /// tree are the working tree's, and saying so is the honest state.
    #[test]
    fn a_dirty_checkout_is_unbound_and_its_sentence_names_no_commit() -> Result<(), String> {
        let state = committed(
            "0123456789012345678901234567890123456789",
            "9876543210987654321098765432109876543210",
        );
        let found = CommitBinding::establish(&state, &state, &differing(&[" M src/lib.rs"]))?;
        let sentence = found.to_string();
        assert!(
            !sentence.contains("0123456789"),
            "an unbound reading named the commit anyway: {sentence}"
        );
        assert!(sentence.contains("1 path(s)"), "{sentence}");
        Ok(())
    }

    /// The positive control: a clean checkout at a settled commit IS bound, and
    /// its sentence names the tree the bytes are.
    #[test]
    fn a_clean_checkout_at_one_commit_is_bound() -> Result<(), String> {
        let state = committed(
            "0123456789012345678901234567890123456789",
            "9876543210987654321098765432109876543210",
        );
        let found = CommitBinding::establish(&state, &state, &differing(&[]))?;
        let sentence = found.to_string();
        assert!(
            sentence.contains("0123456789012345678901234567890123456789"),
            "{sentence}"
        );
        assert!(
            sentence.contains("9876543210987654321098765432109876543210"),
            "{sentence}"
        );
        Ok(())
    }

    /// Planted reversal: a root that is no checkout, and a git that refused.
    ///
    /// Both used to print `unknown (…)` beside the word `commit` and carry on,
    /// which is a run claiming a commit-bound result while stating it has no
    /// commit. Neither is bound now, and the sentence stops claiming one.
    #[test]
    fn an_unknown_commit_binds_nothing() -> Result<(), String> {
        let absent: Read<CommittedTree> = Read::DeclaredAbsent(AbsenceReason::NotAGitCheckout);
        let sentence = CommitBinding::establish(&absent, &absent, &differing(&[]))?.to_string();
        assert!(sentence.contains("not a committed tree"), "{sentence}");
        assert!(sentence.contains("not a git checkout"), "{sentence}");

        let refused: Read<CommittedTree> =
            Read::Unreadable(ReadFailure::new("git rev-parse HEAD", "no such ref"));
        let said = CommitBinding::establish(&refused, &refused, &differing(&[]))?.to_string();
        assert!(said.contains("not a committed tree"), "{said}");
        assert!(said.contains("no such ref"), "{said}");
        Ok(())
    }

    /// Planted reversal: a name that has no Unicode spelling refuses the
    /// reading rather than collapsing onto a replacement character.
    ///
    /// `to_string_lossy` maps every ill-formed name onto the same replacement
    /// character, so two entries differing only there produced ONE canonical
    /// path and the second insertion overwrote the first — a file leaving the
    /// population with no error anywhere. The ill-formed name is built in the
    /// platform's own terms, because there is no portable spelling of one; both
    /// arms assert the same refusal, so whichever platform a run happens on, one
    /// of them executes.
    #[test]
    fn a_name_with_no_unicode_spelling_refuses_the_reading() {
        #[cfg(windows)]
        let ill_formed = {
            use std::os::windows::ffi::OsStringExt;
            // An unpaired high surrogate: a name Windows accepts and UTF-8
            // cannot spell.
            OsString::from_wide(&[0x0073_u16, 0xD800_u16, 0x0074_u16])
        };
        #[cfg(unix)]
        let ill_formed = {
            use std::os::unix::ffi::OsStringExt;
            OsString::from_vec(vec![0x73_u8, 0xFF_u8, 0x74_u8])
        };
        assert!(
            canonical_name(&ill_formed).is_err_and(|refusal| refusal.contains("is not Unicode")),
            "a name with no Unicode spelling was given a canonical path anyway"
        );
        assert_eq!(
            canonical_name(OsStr::new("README.md")),
            Ok(String::from("README.md"))
        );
    }

    /// The two unread entries are unread AT THE ROOT and nowhere else.
    ///
    /// Planted reversal for the basename rule, read off the real tree. Both
    /// names are excluded where they mean what they say — git's storage and the
    /// build's output, at the root — and neither is excluded as a WORD, so a
    /// directory called `target` anywhere else in the tree is repository
    /// material and is read like any other.
    #[test]
    fn the_root_exclusions_are_root_relative() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let unread: Vec<&str> = snapshot
            .files()
            .iter()
            .map(|(path, _)| path.as_str())
            .filter(|path| {
                path.split('/')
                    .next()
                    .is_some_and(|head| head == GIT_STORAGE || head == BUILD_OUTPUT)
            })
            .collect();
        assert!(unread.is_empty(), "{unread:?}");
        assert!(
            snapshot.files().get(GIT_STORAGE).is_none(),
            "the worktree's `{GIT_STORAGE}` file is in the canonical file map, so this checkout \
             reads differently from a clone of the same commit"
        );
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
