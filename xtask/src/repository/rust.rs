//! Rust syntax, read once, by the decoder that owns it.
//!
//! Every `.rs` file in the tree is parsed one time and the tree is carried. Four
//! laws used to parse their own populations out of their own walks — three of
//! them over the SAME files — so three readers could disagree about what a
//! source declares and nothing would have said so.
//!
//! # What a parse establishes here, and what it does not
//!
//! `syn` answers questions about SYNTAX and nothing else: which item kind is
//! declared, which visibility TOKEN is written on it, which attributes are
//! written on it, which fields it declares, what an implementation's written
//! form is.
//!
//! It does not answer resolved public reachability, what an alias points at,
//! which traits are reachable, which `cfg` a build will enable, what a macro
//! expands to, or who semantically owns a type. A law standing on this reader
//! states its claim in the reader's own terms — what a SOURCE declares — and
//! names the gap where a stronger claim would need the compiler rather than a
//! parse.

use std::collections::BTreeMap;

use crate::repository::snapshot::CanonicalFileMap;
use crate::repository::types::{AbsenceReason, CanonicalPath, Read, ReadFailure};

/// Every Rust source in the tree, parsed once.
pub(crate) struct RustSyntaxSnapshot {
    /// Keyed by canonical path, so no law spells a source twice.
    sources: BTreeMap<CanonicalPath, Read<syn::File>>,
}

impl RustSyntaxSnapshot {
    /// Parses every `.rs` file the file map carries.
    ///
    /// A source that does not parse is carried as [`Read::Unreadable`] rather
    /// than dropped. Several fixtures under `testpak/tests/` are written not to
    /// compile on purpose, and whether such a file declares anything is UNKNOWN
    /// rather than false — a hole reported as nothing is the silence this whole
    /// model exists to end.
    pub(crate) fn read(files: &CanonicalFileMap) -> Self {
        let mut sources = BTreeMap::new();
        for (path, fact) in files.iter() {
            if !path.extension_is("rs") {
                continue;
            }
            let parsed = match *fact.text() {
                Read::Known(ref text) => match syn::parse_file(text) {
                    Ok(file) => Read::Known(file),
                    Err(error) => {
                        Read::Unreadable(ReadFailure::new(path.as_str(), &error.to_string()))
                    }
                },
                Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
                Read::Unreadable(ref failure) => Read::Unreadable(failure.clone()),
            };
            sources.insert(path.clone(), parsed);
        }
        Self { sources }
    }

    /// One parsed source, or the declared absence of the file carrying it.
    pub(crate) fn source(&self, path: &CanonicalPath) -> Read<&syn::File> {
        match self.sources.get(path) {
            Some(Read::Known(parsed)) => Read::Known(parsed),
            Some(Read::DeclaredAbsent(reason)) => Read::DeclaredAbsent(*reason),
            Some(Read::Unreadable(failure)) => Read::Unreadable(failure.clone()),
            None => Read::DeclaredAbsent(AbsenceReason::NoSuchPath),
        }
    }

    /// Every source under one directory, parsed or not, in canonical path
    /// order.
    pub(crate) fn under(
        &self,
        directory: &str,
    ) -> impl Iterator<Item = (&CanonicalPath, &Read<syn::File>)> {
        let inside = format!("{directory}/");
        self.sources
            .iter()
            .filter(move |(path, _)| path.as_str().starts_with(&inside))
    }

    /// Every source under the named directories, parsed, in the order the
    /// directories are named.
    ///
    /// A source that did not parse REFUSES the whole reading rather than
    /// leaving a population one file short. A law derived from a population is
    /// a law about that population, and a population missing a member nobody
    /// mentioned is a denominator that shrank in silence.
    pub(crate) fn parsed_under(
        &self,
        directories: &[&str],
    ) -> Result<Vec<(&CanonicalPath, &syn::File)>, String> {
        let mut parsed = Vec::new();
        for directory in directories {
            for (path, source) in self.under(directory) {
                let file = source.required(path.as_str())?;
                parsed.push((path, file));
            }
        }
        Ok(parsed)
    }

    /// Every function item one source declares, at any inline module depth, or
    /// the declared absence of the source that would carry them.
    ///
    /// The reading a law asks when its subject is "which functions does this
    /// file declare, and what is written on each" — where the item stands, what
    /// it is called, and its COMPLETE attribute set. Which of those functions
    /// mean something is the asking law's question and is never decided here.
    pub(crate) fn functions_in(&self, path: &CanonicalPath) -> Read<Vec<DeclaredFunction<'_>>> {
        self.source(path).map(declared_functions)
    }
}

/// One function item a source declares: where it stands, what it is called, and
/// every attribute written on it.
///
/// The attribute set is carried WHOLE and unfiltered, because a reader that kept
/// only the attributes it recognized would answer "no such attribute" about one
/// it had never been told to look for. What an attribute means is the asking
/// law's question; that it is written is this reading's answer.
pub(crate) struct DeclaredFunction<'source> {
    /// The inline modules the function is declared inside, spelled `outer::inner`
    /// and BUILT on the way down. Empty where the function stands at the file's
    /// own level, which is a fact about the source rather than a missing value.
    module: String,
    /// The name the item is declared under.
    name: String,
    /// Every attribute written on the item, in the order written.
    attributes: &'source [syn::Attribute],
}

impl<'source> DeclaredFunction<'source> {
    /// The module path the function is declared inside.
    pub(crate) fn module(&self) -> &str {
        &self.module
    }

    /// The name the item is declared under.
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    /// Every attribute written on the item.
    pub(crate) const fn attributes(&self) -> &'source [syn::Attribute] {
        self.attributes
    }
}

/// Every function item one parsed source declares, in declaration order.
///
/// # Why the module path is built on the way down
///
/// The same reason the file walker builds a canonical path on its way down: a
/// path recovered afterwards is a second derivation of a fact the walk already
/// had, and it needs a fallback for the case it cannot happen in. Each level
/// appends the module it just entered to the spelling it was handed.
///
/// # What this reads, and what it cannot
///
/// A `mod name;` reaching a SEPARATE file is not followed — this reading is
/// about one source. A function declared inside another function's BODY is not
/// an item of any module and is not here either. Both directions fail closed for
/// every caller in this crate: a function this reading does not carry is a
/// function no law can claim, and the law that would have claimed it is refused
/// by name rather than qualifying quietly.
///
/// Nothing here resolves anything. No attribute is interpreted, no path is
/// resolved, no macro is expanded, no condition is evaluated. Two readers were
/// deleted from this crate for reaching past syntax into semantics, and this one
/// answers exactly the three questions a parse can answer about an item.
pub(crate) fn declared_functions(file: &syn::File) -> Vec<DeclaredFunction<'_>> {
    let mut declared = Vec::new();
    read_items(&file.items, "", &mut declared);
    declared
}

/// Reads one scope's items into the list, entering every inline module.
///
/// Written as `if let` rather than as a `match` because `syn::Item` is
/// non-exhaustive: a match would need a wildcard arm, and a wildcard over a
/// foreign enum is the reading that stops being right the day the enum grows.
fn read_items<'source>(
    items: &'source [syn::Item],
    inside: &str,
    into: &mut Vec<DeclaredFunction<'source>>,
) {
    for item in items {
        if let syn::Item::Fn(declared) = item {
            into.push(DeclaredFunction {
                module: String::from(inside),
                name: declared.sig.ident.to_string(),
                attributes: &declared.attrs,
            });
        } else if let syn::Item::Mod(module) = item
            && let Some((_, inner)) = module.content.as_ref()
        {
            let named = module.ident.to_string();
            let deeper = if inside.is_empty() {
                named
            } else {
                format!("{inside}::{named}")
            };
            read_items(inner, &deeper, into);
        }
    }
}
