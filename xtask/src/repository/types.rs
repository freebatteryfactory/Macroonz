//! The vocabulary the two families share.
//!
//! What crosses the line between reading the repository and judging it lives
//! here, and nothing else does: the shape of a law, how a declared module sits
//! on disk, and how one obligation row is spelled. Everything a single law needs
//! for itself is private to that law, because a name shared by one owner is a
//! name in the wrong place.

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

/// One obligation RECORD, and the rows its own block declared.
///
/// The unit an obligation is written in, carried as the unit it is written in.
/// A record opens at `- id:` and states its fields beneath itself, and the rows
/// here are exactly the ones that block carried — not every row the file
/// happens to contain.
///
/// It exists because the rows used to be gathered by two independent scans of
/// the WHOLE file, with nothing binding a row to the record it belongs to. A
/// record whose `green:` line was deleted therefore stated no route, resolved
/// against nothing, and qualified on its `red:` row alone; one whose `red:` line
/// was deleted shrank a denominator this repository publishes with no error
/// anywhere. Neither is reachable through a value that cannot be built without
/// the rows it owns: a record carries its own rows or it carries none, and
/// carrying none is a fact the join can see.
///
/// The rows keep their own readers and their own types. This is a grouping of
/// what those readers produced, never a second reading of the same lines.
pub(crate) struct ObligationRecord {
    /// The identity the record opened with, as `- id:` stated it.
    pub(crate) id: String,
    /// Every `green:` row this record's own block declared, classified.
    pub(crate) green: Vec<GreenRow>,
    /// Every `red:` row this record's own block declared, whole.
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
