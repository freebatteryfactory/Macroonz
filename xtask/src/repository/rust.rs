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
}
