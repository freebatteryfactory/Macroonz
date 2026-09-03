//! The item home's declarations: the structural item family, the borrowed lens, and how a lens read refuses.
//!
//! Declarations only.
//! Every road that reaches a private field lives in `lens.rs`, this file's own child.

use super::super::{
    CapturedDelimiter, CapturedFragment, CapturedInput, CapturedTokenTree, SpanHandle,
};

#[path = "lens.rs"]
mod lens;

/// The structural Rust item family one authored-item lens recognized.
///
/// The row identifies only the item's mechanical envelope.
/// Rustc remains responsible for whether the complete tokens form lawful Rust and what that Rust means.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthoredItemKind {
    /// A module item.
    Module,
    /// A structure item.
    Structure,
    /// An enumeration item.
    Enumeration,
    /// A union item.
    Union,
    /// A trait item.
    Trait,
    /// A function item.
    Function,
    /// An implementation item.
    Implementation,
    /// A type alias.
    TypeAlias,
    /// A constant item.
    Constant,
    /// A static item.
    Static,
    /// A use item.
    Use,
    /// An external-crate item.
    ExternalCrate,
}

/// A checked structural lens into one supported complete caller-authored Rust item.
///
/// Every fragment borrows the same captured material as [`AuthoredItem::preserved`].
/// The lens identifies only an item's outer attributes, visibility, qualifiers, family, optional name, generic-parameter run, where-clause run, signature run, and optional body group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthoredItem<'tokens> {
    preserved: CapturedFragment<'tokens>,
    attributes: CapturedFragment<'tokens>,
    visibility: CapturedFragment<'tokens>,
    qualifiers: CapturedFragment<'tokens>,
    signature: CapturedFragment<'tokens>,
    generics: Option<CapturedFragment<'tokens>>,
    where_clause: Option<CapturedFragment<'tokens>>,
    body: Option<CapturedFragment<'tokens>>,
    body_delimiter: Option<CapturedDelimiter>,
    kind: AuthoredItemKind,
    kind_token: &'tokens CapturedTokenTree,
    name_token: Option<&'tokens CapturedTokenTree>,
    unsafe_token: Option<&'tokens CapturedTokenTree>,
}

/// Why one captured input could not provide an authored-item structural lens.
///
/// These rows describe only the item envelope Macroonz must coordinate.
/// They do not replace Rustc's syntax, type, ownership, or coherence judgments.
#[must_use = "an authored-item issue names the structural envelope that was not present"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthoredItemReadIssue {
    /// The declared item boundary carried no token.
    ItemMissing,
    /// No supported item-family keyword followed the outer attributes, visibility, and qualifiers.
    ItemKindMissing,
    /// An item family that has a name carried no identifier in its name seat.
    ItemNameMissing(AuthoredItemKind),
    /// The declared item boundary ended without a braced body or semicolon.
    ItemBoundaryUnfinished(AuthoredItemKind),
    /// A structural coordinate no longer names a run inside the captured item.
    LensRangeContradiction,
}

/// One refused authored-item lens with the exact available producer span.
#[must_use = "an authored-item refusal carries its structural issue and exact available span"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthoredItemReadRefusal {
    issue: AuthoredItemReadIssue,
    at: Option<SpanHandle>,
}
