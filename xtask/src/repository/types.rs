//! The vocabulary the two families share.
//!
//! What crosses the line between reading the repository and judging it lives
//! here, and nothing else does: what a reading established, how a path is
//! spelled once and for all, how a declared module sits on disk, and how one
//! obligation row is written. Everything a single law needs for itself is
//! private to that law, because a name shared by one owner is a name in the
//! wrong place.

use std::fmt;

use crate::repository::snapshot::RepositorySnapshot;

/// One repository law: a name and the function that checks it.
///
/// A law is handed the SNAPSHOT and never a path. That is the whole shape of
/// this model: the tree is read once, by one builder, and every law is a pure
/// function over what that reading established. A law that took a path could
/// walk the filesystem again, and two laws walking it separately are two laws
/// that can be judging different trees.
pub(crate) type Check = (&'static str, fn(&RepositorySnapshot) -> Result<(), String>);

/// What one reading of one fact established.
///
/// Three states, and the third is the one every fallback in this crate used to
/// spell as one of the first two. A file that could not be read came back as an
/// empty string; a directory that could not be listed came back as an empty
/// vector; a root that could not be resolved came back as `"."`. Each of those
/// answers a question the reader never got to ask, and each answers it in the
/// direction that PASSES: an empty manifest declares no prohibited edge, an
/// empty tree holds no offending file, and a law reported clean about bytes
/// nobody opened.
///
/// Unknown is not false and is not an empty collection. A caller either handles
/// all three states or asks [`Read::required`] for the fact and is refused when
/// it is not there — there is no third road, because there is no method here
/// that turns an unread fact into a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Read<T> {
    /// The fact, as the reading that owns it established it.
    Known(T),
    /// The fact is not there, and the reason is DECLARED rather than inferred.
    DeclaredAbsent(AbsenceReason),
    /// The reading failed. What it was reading and what the failure said are
    /// both carried, because a caller reporting "unknown" must be able to say
    /// unknown about WHAT.
    Unreadable(ReadFailure),
}

impl<T> Read<T> {
    /// The fact where it is known, and nothing where it is not.
    ///
    /// The escape hatch is deliberately shaped as an `Option` rather than as a
    /// value with a default: a caller that reaches for this has to write what it
    /// does about the absence, in its own words, at the site where the absence
    /// matters.
    pub(crate) const fn known(&self) -> Option<&T> {
        match *self {
            Read::Known(ref fact) => Some(fact),
            Read::DeclaredAbsent(_) | Read::Unreadable(_) => None,
        }
    }

    /// The fact, or a refusal naming what could not be established about it.
    ///
    /// THE road a law takes to a fact it needs. Absence and failure both become
    /// refusals, in their own words, because a law standing on a fact nobody
    /// established is a law reporting about bytes nobody opened.
    pub(crate) fn required(&self, subject: &str) -> Result<&T, String> {
        match *self {
            Read::Known(ref fact) => Ok(fact),
            Read::DeclaredAbsent(reason) => Err(format!("{subject} is not there: {reason}")),
            Read::Unreadable(ref failure) => Err(format!("{subject} could not be read: {failure}")),
        }
    }

    /// The fact taken OUT of the reading, or a refusal naming what could not be
    /// established about it.
    ///
    /// The twin of [`Read::required`], for the readings that are values rather
    /// than fields: an accessor that answers a question — one file's text, one
    /// key's string, one document's ledger — hands back a reading built for that
    /// call, and a borrow of it would not outlive the call. Same three states,
    /// same refusals, and no fallback in either.
    pub(crate) fn taken(self, subject: &str) -> Result<T, String> {
        match self {
            Read::Known(fact) => Ok(fact),
            Read::DeclaredAbsent(reason) => Err(format!("{subject} is not there: {reason}")),
            Read::Unreadable(failure) => Err(format!("{subject} could not be read: {failure}")),
        }
    }

    /// The same reading, with a known fact renamed into the type that names it.
    ///
    /// Deliberately the only combinator here. A reading may be renamed; it may
    /// not be unwrapped, defaulted, or filtered into one of its other states,
    /// because each of those is a fallback wearing a method name.
    pub(crate) fn map<U>(self, into: impl FnOnce(T) -> U) -> Read<U> {
        match self {
            Read::Known(fact) => Read::Known(into(fact)),
            Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
            Read::Unreadable(failure) => Read::Unreadable(failure),
        }
    }
}

/// Why a fact is absent.
///
/// Every variant is a statement somebody made about the tree, never a shrug. An
/// absence that cannot be spelled as one of these is not an absence — it is a
/// [`Read::Unreadable`], and it says so.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AbsenceReason {
    /// The snapshot's file map carries no such path.
    NoSuchPath,
    /// The root declares no `Cargo.toml`, so there is no workspace for cargo to
    /// read and no reading to ask about.
    NotAWorkspaceCheckout,
    /// The document declares no data block carrying the schema asked for.
    NoBlockDeclaresThisSchema,
    /// The document is there and states no such key.
    NoSuchKey,
}

impl fmt::Display for AbsenceReason {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        let said = match *self {
            AbsenceReason::NoSuchPath => "no file in the repository sits at that path",
            AbsenceReason::NotAWorkspaceCheckout => {
                "the root declares no Cargo.toml, so cargo reports nothing here"
            }
            AbsenceReason::NoBlockDeclaresThisSchema => {
                "no fenced data block in that document declares this schema"
            }
            AbsenceReason::NoSuchKey => "the document states no such key",
        };
        out.write_str(said)
    }
}

/// What a reading that failed was reading, and what the failure said.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReadFailure {
    /// What was being read, as this repository spells it.
    subject: String,
    /// What the failure said, in the words of whatever refused.
    said: String,
}

impl ReadFailure {
    /// One failure, carrying its subject and the words the refusal came in.
    pub(crate) fn new(subject: &str, said: &str) -> Self {
        Self {
            subject: subject.to_owned(),
            said: said.to_owned(),
        }
    }
}

impl fmt::Display for ReadFailure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{}: {}", self.subject, self.said)
    }
}

/// One path, spelled the one way this repository spells paths: relative to the
/// root, forward slashes, on every platform.
///
/// A newtype rather than a `String` because the spelling is the join key. Every
/// resolution in this crate — a green route against a test seat, a red row
/// against a fixture, an allowlist entry against a scanned file — compares one
/// of these to another, and a comparison between a path spelled two ways is a
/// join that silently answers no.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CanonicalPath(String);

impl CanonicalPath {
    /// The one construction road, taking the spelling as this repository writes
    /// it: repository-relative, forward slashes, no leading `./`.
    pub(crate) fn spelled(relative: &str) -> Self {
        Self(relative.replace('\\', "/"))
    }

    /// The path as text, for a message or a comparison against a row's value.
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether the path sits inside a named directory, at any depth. The prefix
    /// is matched with its trailing slash, so `src` never matches `src-notes/`.
    pub(crate) fn is_under(&self, directory: &str) -> bool {
        self.0.starts_with(&format!("{directory}/"))
    }

    /// Whether the path sits DIRECTLY inside a named directory and no deeper.
    pub(crate) fn sits_directly_in(&self, directory: &str) -> bool {
        self.is_under(directory)
            && self
                .0
                .get(directory.len().saturating_add(1)..)
                .is_some_and(|tail| !tail.contains('/'))
    }

    /// Whether the path names a file with the given extension.
    pub(crate) fn extension_is(&self, extension: &str) -> bool {
        self.0
            .rsplit_once('.')
            .is_some_and(|(_, found)| found == extension)
    }

    /// The last segment of the path.
    pub(crate) fn file_name(&self) -> &str {
        self.0
            .rsplit_once('/')
            .map_or(self.0.as_str(), |(_, name)| name)
    }
}

impl fmt::Display for CanonicalPath {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(&self.0)
    }
}

/// Whether one file is a symbolic link.
///
/// A named state rather than a boolean field, because `clippy.toml` sets
/// `max-struct-bools = 0` and because `true` says nothing about which way the
/// question was asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LinkState {
    /// An ordinary file, whose bytes are its content.
    RegularFile,
    /// A symbolic link, whose content is a path to somewhere else.
    Symlink,
}

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

/// One obligation RECORD, and the rows its own block declared.
///
/// The unit an obligation is written in, carried as the unit it is written in.
/// A record opens at a sequence item stating `id:` and carries the fields
/// written beneath that item, and the rows here are exactly the ones that item
/// carried — not every row the file happens to contain.
///
/// It exists because the rows used to be gathered by two independent scans of
/// the WHOLE file, with nothing binding a row to the record it belongs to. A
/// record whose `green:` line was deleted therefore stated no route, resolved
/// against nothing, and qualified on its `red:` row alone; one whose `red:` line
/// was deleted shrank a denominator this repository publishes with no error
/// anywhere. Neither is reachable through a value that cannot be built without
/// the rows it owns: a record carries its own rows or it carries none, and
/// carrying none is a fact the join can see.
pub(crate) struct ObligationRecord {
    /// The identity the record opened with, as its `id:` field stated it.
    pub(crate) id: String,
    /// Every `green:` row this record's own item declared, classified.
    pub(crate) green: Vec<GreenRow>,
    /// Every `red:` row this record's own item declared, whole.
    pub(crate) red: Vec<String>,
}

/// The spelling one `green:` obligation row states its positive control in.
///
/// A green row is written one of three ways, and this is all three plus the
/// state of being none of them. `laws.rs module::name` names a compile-time seat
/// and is joined against `laws.rs` itself; `none — …`, `owed — …`, and
/// `structural (…)` state that no file holds a positive control and account for
/// why; anything else path-shaped names the file that does.
///
/// The fourth variant is why this is an enum rather than a filter. A reader that
/// kept only the spellings it could USE would silently drop the ones it could
/// not read — a route that lost its suffix, a bare word nobody declared, a value
/// somebody emptied — and a dropped row is an obligation that qualifies while
/// the positive control it names is never looked for. Every row leaves the
/// reader classified, and one that no spelling reads leaves it named.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GreenRow {
    /// `laws.rs module::name`: the positive control is a compile-time seat, and
    /// this is the target the row named, split where the row split it.
    ///
    /// The target is CARRIED rather than left to a reader of its own. It was
    /// once dropped here, on the argument that reading it twice would answer one
    /// claim twice — but nothing was ever read twice. One reader classified the
    /// row and a SECOND one resolved its target off a stricter prefix, and the
    /// two were never compared: a row the strict prefix did not match was seated
    /// by the first and claimed by neither, so the obligation qualified while
    /// naming a law nobody wrote. Carrying the target is what makes that
    /// unrepresentable rather than merely unlikely — a seat cannot be
    /// constructed without the claim it makes, so there is no second reader left
    /// to disagree with.
    CompileTimeSeat {
        /// The `laws.rs` module the row named, before the `::`.
        module: String,
        /// The law within that module, after the `::`.
        law: String,
    },
    /// `none — …`, `owed — …`, or `structural (…)`: the row states that no file
    /// holds a positive control, and accounts for why. The account is part of
    /// the form, because a bare word states the absence and withholds the
    /// reason for it.
    Disposition,
    /// A path to a Rust file, carried as written: the row says that file holds
    /// the positive control, and the join requires it to be a test that runs.
    Route(String),
    /// No spelling this repository reads, carried as written so the join can
    /// name the row against the README that declared it.
    Unreadable(String),
}
