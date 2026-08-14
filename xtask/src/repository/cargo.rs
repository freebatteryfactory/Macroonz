//! Cargo's own answers about Cargo.
//!
//! Two authorities, two questions, and keeping them apart is the whole of this
//! module.
//!
//! **What a manifest DECLARES** is a question about TOML, and the `toml`
//! decoder answers it. Every spelling Cargo admits — a dotted key, a quoted key,
//! a bracketed-string header, an escaped key, a literal string, a multi-line
//! string, a four-quote terminator, a unicode escape, an inline table, a key
//! carrying `=`, a comment after a header — is one document to that decoder, so
//! a reader standing on it recognizes a spelling nobody thought of.
//!
//! **What Cargo RESOLVES** is a different question, and only cargo answers it:
//! `cargo metadata --locked --format-version 1` reports package identities, edge
//! kinds, renames, target-conditioned edges, and the graph itself. The format
//! version is pinned in the invocation because it is the machine-readable
//! contract; `--locked` is pinned because a run that repaired the lock file on
//! its way past would be reporting about a dependency set nobody chose.
//!
//! # What this replaced, and why the class rather than the site
//!
//! A line reader stood here. It cut a line at its first `=` and read the head as
//! a package name, so `threadpak-macroc.workspace = true` named the package
//! `threadpak-macroc.workspace`, which matched no law. Eleven distinct spellings
//! escaped it over one campaign, and each repair — a dotted-key pass, a
//! quoted-key pass, a literal-string pass, a comment-stripping pass — revealed
//! the next one, because a spelling admitted one at a time is a set with no last
//! member. The reader was not unlucky. It was in the wrong seat: a weaker reader
//! re-deriving what a stronger reader already owned.
//!
//! Nothing here recognizes a spelling. The decoder resolves the document and
//! this module reads key paths out of it.

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;
use std::process::{Command, Stdio};

use serde::Deserialize;

use crate::repository::snapshot::{CanonicalFileMap, cargo_binary};
use crate::repository::types::{AbsenceReason, CanonicalPath, Read, ReadFailure};

/// The manifest every Cargo package is declared in.
pub(crate) const MANIFEST_FILE: &str = "Cargo.toml";

/// The table a platform-conditional dependency table hangs beneath.
const TARGET_TABLE: &str = "target";

/// Everything the Cargo authorities established about this repository, read
/// once.
pub(crate) struct CargoSnapshot {
    /// What cargo resolved, or why nobody asked it.
    resolved: Read<ResolvedWorkspace>,
    /// Every `.toml` document in the tree, decoded by the decoder that owns
    /// TOML. Keyed by canonical path, so no reader spells one twice.
    documents: BTreeMap<CanonicalPath, Read<toml::Table>>,
    /// Every dependency entry every committed manifest declares, in one list.
    census: ManifestCensus,
}

impl CargoSnapshot {
    /// Reads every TOML document in the tree, takes the census off them, and
    /// asks cargo what it resolved.
    pub(crate) fn read(root: &Path, files: &CanonicalFileMap) -> Self {
        let mut documents = BTreeMap::new();
        for (path, fact) in files.iter() {
            if !path.extension_is("toml") {
                continue;
            }
            documents.insert(path.clone(), decode(path, fact.text()));
        }
        let census = ManifestCensus::take(&documents);
        Self {
            resolved: resolve(root, files),
            documents,
            census,
        }
    }

    /// What cargo resolved, or why nobody asked it.
    pub(crate) const fn resolved(&self) -> &Read<ResolvedWorkspace> {
        &self.resolved
    }

    /// One decoded TOML document, or the absence of the file that would carry
    /// it.
    pub(crate) fn document(&self, path: &str) -> Read<&toml::Table> {
        match self.documents.get(&CanonicalPath::spelled(path)) {
            Some(Read::Known(document)) => Read::Known(document),
            Some(Read::DeclaredAbsent(reason)) => Read::DeclaredAbsent(*reason),
            Some(Read::Unreadable(failure)) => Read::Unreadable(failure.clone()),
            None => Read::DeclaredAbsent(AbsenceReason::NoSuchPath),
        }
    }

    /// Every dependency entry every committed manifest declares.
    pub(crate) const fn census(&self) -> &ManifestCensus {
        &self.census
    }
}

/// One TOML text, decoded, with the failure carried where it did not decode.
fn decode(path: &CanonicalPath, text: &Read<String>) -> Read<toml::Table> {
    match *text {
        Read::Known(ref text) => match text.parse::<toml::Table>() {
            Ok(document) => Read::Known(document),
            Err(error) => Read::Unreadable(ReadFailure::new(path.as_str(), &error.to_string())),
        },
        Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
        Read::Unreadable(ref failure) => Read::Unreadable(failure.clone()),
    }
}

/// Which Cargo edge kind one declaration sits under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EdgeKind {
    /// `[dependencies]` — the edge a build takes.
    Ordinary,
    /// `[dev-dependencies]` — the edge tests take, and still an edge.
    Development,
    /// `[build-dependencies]` — the edge a build script takes.
    Build,
}

impl EdgeKind {
    /// The table this kind is declared in, as Cargo spells it.
    const fn table(self) -> &'static str {
        match self {
            EdgeKind::Ordinary => "dependencies",
            EdgeKind::Development => "dev-dependencies",
            EdgeKind::Build => "build-dependencies",
        }
    }

    /// The kind `cargo metadata` reports one resolved edge under. Cargo states
    /// nothing for an ordinary edge, so absence IS the ordinary kind here — a
    /// fact of the reported format rather than a default this reader invented.
    fn reported(kind: Option<&str>) -> Read<Self> {
        match kind {
            None => Read::Known(EdgeKind::Ordinary),
            Some("dev") => Read::Known(EdgeKind::Development),
            Some("build") => Read::Known(EdgeKind::Build),
            Some(other) => Read::Unreadable(ReadFailure::new(
                "cargo metadata dependency kind",
                &format!("`{other}` is no edge kind this reader knows"),
            )),
        }
    }
}

impl fmt::Display for EdgeKind {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(self.table())
    }
}

/// Every Cargo edge kind, in the order a census reports them.
const EDGE_KINDS: [EdgeKind; 3] = [EdgeKind::Ordinary, EdgeKind::Development, EdgeKind::Build];

/// One dependency entry, as one manifest DECLARES it.
///
/// The four facts a topology law needs and nothing else: which edge kind the
/// entry sits under, the key it is written at, the package it names where it
/// renames one, and the path it points at where it declares one.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct DeclaredDependency {
    /// The manifest that declares it.
    manifest: CanonicalPath,
    /// The edge kind it sits under.
    kind: EdgeKind,
    /// The key it is written at, which is the local name unless it renames.
    key: String,
    /// The package it names, where the entry states one.
    package: Option<String>,
    /// The path it points at, where the entry states one.
    path: Option<String>,
}

impl DeclaredDependency {
    /// The manifest that declares it.
    pub(crate) const fn manifest(&self) -> &CanonicalPath {
        &self.manifest
    }

    /// The package this entry resolves to: its `package = "…"` where it states
    /// one, and its own key otherwise. That is Cargo's rule, and it is why a
    /// rename hides nothing.
    pub(crate) fn identity(&self) -> &str {
        match self.package {
            Some(ref named) => named,
            None => &self.key,
        }
    }

    /// The path it points at, where the entry declares one.
    pub(crate) fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }
}

impl fmt::Display for DeclaredDependency {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "[{}] `{}`", self.kind, self.key)
    }
}

/// Every dependency entry every committed manifest declares.
///
/// The census is taken ONCE, over every manifest at once, so no law can be
/// judging a different set of entries than another. A number that moves here
/// moved because the tree moved.
pub(crate) struct ManifestCensus(Vec<DeclaredDependency>);

impl ManifestCensus {
    /// The census of every decoded manifest, in manifest-path order and, within
    /// one manifest, in edge-kind then key order.
    fn take(documents: &BTreeMap<CanonicalPath, Read<toml::Table>>) -> Self {
        let mut entries = Vec::new();
        for (path, document) in documents {
            if path.file_name() != MANIFEST_FILE {
                continue;
            }
            if let Read::Known(ref document) = *document {
                entries.extend(dependency_declarations(path, document));
            }
        }
        Self(entries)
    }

    /// Every entry one manifest declares.
    pub(crate) fn of(&self, manifest: &str) -> Vec<&DeclaredDependency> {
        let named = CanonicalPath::spelled(manifest);
        self.0
            .iter()
            .filter(|entry| *entry.manifest() == named)
            .collect()
    }
}

/// Every dependency entry one decoded manifest declares.
///
/// Nothing here recognizes a spelling. The decoder resolved the document, and
/// this reads three key paths out of it plus the same three beneath every
/// `target.<spec>` table — which is what a platform-conditional edge is, and
/// which is why one is read exactly like the unconditional edge it conditions.
pub(crate) fn dependency_declarations(
    manifest: &CanonicalPath,
    document: &toml::Table,
) -> Vec<DeclaredDependency> {
    let mut declared = Vec::new();
    read_edge_tables(manifest, document, &mut declared);
    if let Some(toml::Value::Table(targets)) = document.get(TARGET_TABLE) {
        for conditioned in targets.values() {
            if let toml::Value::Table(conditioned) = conditioned {
                read_edge_tables(manifest, conditioned, &mut declared);
            }
        }
    }
    declared
}

/// The three edge tables of one table, whether that table is the document root
/// or one `target.<spec>` beneath it.
fn read_edge_tables(
    manifest: &CanonicalPath,
    table: &toml::Table,
    into: &mut Vec<DeclaredDependency>,
) {
    for kind in EDGE_KINDS {
        if let Some(toml::Value::Table(entries)) = table.get(kind.table()) {
            for (key, value) in entries {
                into.push(DeclaredDependency {
                    manifest: manifest.clone(),
                    kind,
                    key: key.clone(),
                    package: field(value, "package"),
                    path: field(value, "path"),
                });
            }
        }
    }
}

/// One string field of one entry's value, where the entry states a table at
/// all. A version-only entry — `serde = "1"` — states neither field, which is
/// the declaration it is rather than an absence anybody has to interpret.
fn field(value: &toml::Value, named: &str) -> Option<String> {
    let toml::Value::Table(fields) = value else {
        return None;
    };
    if let Some(toml::Value::String(spelled)) = fields.get(named) {
        Some(spelled.clone())
    } else {
        None
    }
}

/// The value one key path names, or the declared absence of it.
fn value_at<'document>(
    document: &'document toml::Table,
    key_path: &[&str],
) -> Read<&'document toml::Value> {
    let Some((last, leading)) = key_path.split_last() else {
        return Read::DeclaredAbsent(AbsenceReason::NoSuchKey);
    };
    let mut table = document;
    for segment in leading {
        let Some(toml::Value::Table(inner)) = table.get(*segment) else {
            return Read::DeclaredAbsent(AbsenceReason::NoSuchKey);
        };
        table = inner;
    }
    match table.get(*last) {
        Some(found) => Read::Known(found),
        None => Read::DeclaredAbsent(AbsenceReason::NoSuchKey),
    }
}

/// The word one TOML value's kind is named by, for a message about a document
/// that states the wrong kind at a key.
fn kind_word(value: &toml::Value) -> &'static str {
    match *value {
        toml::Value::String(_) => "string",
        toml::Value::Integer(_) => "integer",
        toml::Value::Float(_) => "float",
        toml::Value::Boolean(_) => "boolean",
        toml::Value::Datetime(_) => "datetime",
        toml::Value::Array(_) => "array",
        toml::Value::Table(_) => "table",
    }
}

/// The string one key path states.
pub(crate) fn string_at(document: &toml::Table, key_path: &[&str]) -> Read<String> {
    match value_at(document, key_path) {
        Read::Known(toml::Value::String(spelled)) => Read::Known(spelled.clone()),
        Read::Known(other) => Read::Unreadable(ReadFailure::new(
            &key_path.join("."),
            &format!("states a {} where a string was declared", kind_word(other)),
        )),
        Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
        Read::Unreadable(failure) => Read::Unreadable(failure),
    }
}

/// The list of strings one key path states.
pub(crate) fn strings_at(document: &toml::Table, key_path: &[&str]) -> Read<Vec<String>> {
    match value_at(document, key_path) {
        Read::Known(toml::Value::Array(items)) => {
            let mut listed = Vec::new();
            for item in items {
                let toml::Value::String(spelled) = item else {
                    return Read::Unreadable(ReadFailure::new(
                        &key_path.join("."),
                        &format!("lists a {} where a string was declared", kind_word(item)),
                    ));
                };
                listed.push(spelled.clone());
            }
            Read::Known(listed)
        }
        Read::Known(other) => Read::Unreadable(ReadFailure::new(
            &key_path.join("."),
            &format!("states a {} where a list was declared", kind_word(other)),
        )),
        Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
        Read::Unreadable(failure) => Read::Unreadable(failure),
    }
}

/// Whether one key path states `true`.
pub(crate) fn declares_yes(document: &toml::Table, key_path: &[&str]) -> Read<Declaration> {
    match value_at(document, key_path) {
        Read::Known(toml::Value::Boolean(stated)) => Read::Known(if *stated {
            Declaration::Yes
        } else {
            Declaration::No
        }),
        Read::Known(other) => Read::Unreadable(ReadFailure::new(
            &key_path.join("."),
            &format!("states a {} where a boolean was declared", kind_word(other)),
        )),
        Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
        Read::Unreadable(failure) => Read::Unreadable(failure),
    }
}

/// What one boolean key states, as a name rather than as a bare `true`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Declaration {
    /// The key states `true`.
    Yes,
    /// The key states `false`.
    No,
}

/// The table one key path states.
///
/// The twin of [`string_at`] for the readings whose subject is a whole table
/// rather than one value in it — a configuration profile, say, which is only
/// comparable to another profile as a table. A key path stating something that
/// is not a table is UNREADABLE rather than absent, for the reason every
/// accessor here says so: a caller told "no such table" about a key stating a
/// string would go looking for a key nobody deleted.
pub(crate) fn table_at<'document>(
    document: &'document toml::Table,
    key_path: &[&str],
) -> Read<&'document toml::Table> {
    match value_at(document, key_path) {
        Read::Known(toml::Value::Table(stated)) => Read::Known(stated),
        Read::Known(other) => Read::Unreadable(ReadFailure::new(
            &key_path.join("."),
            &format!("states a {} where a table was declared", kind_word(other)),
        )),
        Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
        Read::Unreadable(failure) => Read::Unreadable(failure),
    }
}

/// Whether one key path names a table at all.
pub(crate) fn declares_table(document: &toml::Table, key_path: &[&str]) -> Read<Declaration> {
    match value_at(document, key_path) {
        Read::Known(toml::Value::Table(_)) => Read::Known(Declaration::Yes),
        Read::Known(_) => Read::Known(Declaration::No),
        Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
        Read::Unreadable(failure) => Read::Unreadable(failure),
    }
}

/// What `cargo metadata` reported.
///
/// The fields are exactly the ones a law reads. `cargo metadata` reports a great
/// deal more, and every field named here is one this repository has a reader
/// for — a field carried because it was available would be an inventory nobody
/// joins.
#[derive(Debug, Deserialize)]
pub(crate) struct ResolvedWorkspace {
    /// Every package in the resolved graph, workspace members included.
    packages: Vec<ResolvedPackage>,
}

impl ResolvedWorkspace {
    /// The package cargo resolved under one name, or the declared absence of
    /// it.
    pub(crate) fn package(&self, named: &str) -> Read<&ResolvedPackage> {
        match self.packages.iter().find(|package| package.name == named) {
            Some(found) => Read::Known(found),
            None => Read::DeclaredAbsent(AbsenceReason::NoSuchKey),
        }
    }
}

/// One package as cargo resolved it.
#[derive(Debug, Deserialize)]
pub(crate) struct ResolvedPackage {
    /// The package name, which is the identity a law judges.
    name: String,
    /// Every edge cargo resolved out of it, of every kind.
    dependencies: Vec<ResolvedDependency>,
}

impl ResolvedPackage {
    /// Every edge cargo resolved out of this package.
    pub(crate) fn dependencies(&self) -> &[ResolvedDependency] {
        &self.dependencies
    }
}

/// One resolved edge, as cargo reports it.
#[derive(Debug, Deserialize)]
pub(crate) struct ResolvedDependency {
    /// The PACKAGE the edge reaches. Cargo reports the package rather than the
    /// key, so a rename is already resolved here.
    name: String,
    /// The key the declaring manifest wrote, where it renamed the package.
    rename: Option<String>,
    /// The edge kind, as cargo spells it: nothing for an ordinary edge.
    kind: Option<String>,
    /// The platform predicate the edge is conditioned on, where it is
    /// conditioned at all.
    target: Option<String>,
}

impl ResolvedDependency {
    /// The package the edge reaches.
    pub(crate) fn package(&self) -> &str {
        &self.name
    }

    /// The key the declaring manifest wrote it at.
    pub(crate) fn key(&self) -> &str {
        match self.rename {
            Some(ref renamed) => renamed,
            None => &self.name,
        }
    }

    /// The edge kind, or the failure of reading one cargo spelled a way this
    /// reader does not know.
    pub(crate) fn kind(&self) -> Read<EdgeKind> {
        EdgeKind::reported(self.kind.as_deref())
    }

    /// The platform predicate, where the edge is conditioned.
    pub(crate) fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }
}

/// Asks cargo what it resolved, or states why nobody asked.
///
/// A root declaring no manifest is not a workspace, and saying so is a
/// DECLARED absence rather than an empty resolution: an empty resolution would
/// answer "the core reaches no tooling" about a tree cargo never opened.
fn resolve(root: &Path, files: &CanonicalFileMap) -> Read<ResolvedWorkspace> {
    if files.get(MANIFEST_FILE).is_none() {
        return Read::DeclaredAbsent(AbsenceReason::NotAWorkspaceCheckout);
    }
    let output = Command::new(cargo_binary())
        .current_dir(root)
        .args([
            "metadata",
            "--locked",
            "--format-version",
            "1",
            "--manifest-path",
        ])
        .arg(root.join(MANIFEST_FILE))
        .stderr(Stdio::piped())
        .output();
    let output = match output {
        Ok(output) => output,
        Err(error) => {
            return Read::Unreadable(ReadFailure::new("cargo metadata", &error.to_string()));
        }
    };
    if !output.status.success() {
        return Read::Unreadable(ReadFailure::new(
            "cargo metadata",
            String::from_utf8_lossy(&output.stderr).trim(),
        ));
    }
    match serde_json::from_slice::<ResolvedWorkspace>(&output.stdout) {
        Ok(resolved) => Read::Known(resolved),
        Err(error) => Read::Unreadable(ReadFailure::new(
            "cargo metadata --format-version 1",
            &error.to_string(),
        )),
    }
}

/// Planted reversals for the reader that replaced eleven passes over eleven
/// spellings.
///
/// The claim under test is not "this reader knows these spellings". It is that
/// there are no spellings to know: the decoder resolves the document and this
/// reader reads key paths, so every way Cargo admits of writing one declaration
/// arrives as one declaration. Every case below is a fixture string — the reader
/// is proven against text, never against the tree it guards.
#[cfg(test)]
mod tests {
    use super::{DeclaredDependency, EdgeKind, dependency_declarations, string_at, strings_at};
    use crate::repository::snapshot::repository_snapshot;
    use crate::repository::types::{CanonicalPath, Read};

    /// The entries one fixture manifest declares, decoded by the decoder that
    /// owns TOML.
    fn declared(text: &str) -> Result<Vec<DeclaredDependency>, String> {
        let document = text
            .parse::<toml::Table>()
            .map_err(|error| format!("fixture manifest does not decode: {error}"))?;
        Ok(dependency_declarations(
            &CanonicalPath::spelled("Cargo.toml"),
            &document,
        ))
    }

    /// The one entry every spelling below declares: an ordinary edge written at
    /// the key `helpers`, naming the package `threadpak-macroc`, pointing at
    /// `macros/macroc`.
    fn the_one_entry() -> DeclaredDependency {
        DeclaredDependency {
            manifest: CanonicalPath::spelled("Cargo.toml"),
            kind: EdgeKind::Ordinary,
            key: String::from("helpers"),
            package: Some(String::from("threadpak-macroc")),
            path: Some(String::from("macros/macroc")),
        }
    }

    /// Nine spellings of ONE declaration, and every one of them resolves
    /// identically.
    ///
    /// Every case here escaped the line reader this replaced, and each was
    /// repaired on its own, in its own pass, after its own escape. The dotted
    /// key was read as a package name with `.workspace` on the end. The quoted
    /// header closed the table instead of opening it, leaving every entry
    /// beneath it unread. The literal-string value was half the spellings a path
    /// can be written in. The comment after a header made a whole table
    /// invisible. The multi-line string and the unicode escape were never
    /// reached at all.
    ///
    /// None of them is handled HERE. There is no case for any of them in this
    /// module, which is the point: the decoder owns TOML, so a tenth spelling
    /// nobody has thought of is read correctly by a reader that was never told
    /// about it.
    #[test]
    fn every_spelling_of_one_declaration_resolves_identically() -> Result<(), String> {
        let spellings: [(&str, &str); 9] = [
            (
                "inline table",
                "[dependencies]\nhelpers = { package = \"threadpak-macroc\", path = \"macros/macroc\" }\n",
            ),
            (
                "sub-table header",
                "[dependencies.helpers]\npackage = \"threadpak-macroc\"\npath = \"macros/macroc\"\n",
            ),
            (
                "dotted keys",
                "[dependencies]\nhelpers.package = \"threadpak-macroc\"\nhelpers.path = \"macros/macroc\"\n",
            ),
            (
                "dotted table key written before any header",
                "dependencies.helpers.package = \"threadpak-macroc\"\ndependencies.helpers.path = \"macros/macroc\"\n",
            ),
            (
                "quoted key path, under both TOML quotes",
                "[\"dependencies\".'helpers']\npackage = \"threadpak-macroc\"\npath = \"macros/macroc\"\n",
            ),
            (
                "literal-string values",
                "[dependencies]\nhelpers = { package = 'threadpak-macroc', path = 'macros/macroc' }\n",
            ),
            (
                "multi-line basic strings",
                "[dependencies.helpers]\npackage = \"\"\"threadpak-macroc\"\"\"\npath = \"\"\"\\\nmacros/macroc\"\"\"\n",
            ),
            (
                "unicode escapes inside values",
                "[dependencies]\nhelpers = { package = \"threadpak\\u002Dmacroc\", path = \"macros\\u002Fmacroc\" }\n",
            ),
            (
                "a comment after the table header",
                "[dependencies] # inherited from the workspace\nhelpers = { package = \"threadpak-macroc\", path = \"macros/macroc\" }\n",
            ),
        ];
        let expected = vec![the_one_entry()];
        for (name, text) in spellings {
            let found = declared(text)?;
            assert_eq!(found, expected, "{name} did not resolve to the one entry");
        }
        Ok(())
    }

    /// The four spellings that are not merely another way of writing the same
    /// thing: each one changes what the declaration IS, and the decoder is what
    /// says so.
    ///
    /// An escaped key in a header — `["dev-dependencies"]` — is the
    /// dev-dependency table, spelled so that no reader matching the word would
    /// know. A four-quote terminator closes a multi-line string one quote late,
    /// so the value carries that quote; the line reader took the first `"` it
    /// found and read the path as empty. A quoted key carrying `=` is one key,
    /// and the line reader cut it in half at the `=` and read a key that does
    /// not exist. And a manifest QUOTING a dependency table inside a multi-line
    /// string declares no dependency at all — the line reader read the quoted
    /// lines as the table they resemble and reported a phantom edge, which is
    /// the one ceiling that file wrote down and could not close.
    #[test]
    fn a_spelling_that_changes_the_declaration_changes_it_exactly() -> Result<(), String> {
        let escaped =
            declared("[\"dev\\u002Ddependencies\"]\nhelpers = { path = \"macros/macroc\" }\n")?;
        assert_eq!(escaped.len(), 1, "{escaped:?}");
        assert!(
            escaped
                .first()
                .is_some_and(|entry| entry.kind == EdgeKind::Development),
            "{escaped:?}"
        );

        let four_quote = declared("[dependencies.helpers]\npath = \"\"\"macros/macroc\"\"\"\"\n")?;
        assert!(
            four_quote
                .first()
                .is_some_and(|entry| entry.path() == Some("macros/macroc\"")),
            "{four_quote:?}"
        );

        let equals = declared("[dependencies]\n\"a=b\" = { path = \"macros/macroc\" }\n")?;
        assert!(
            equals.first().is_some_and(|entry| entry.key == "a=b"),
            "{equals:?}"
        );

        let quoted_table = declared(
            "description = \"\"\"\n[dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n\"\"\"\n",
        )?;
        assert!(
            quoted_table.is_empty(),
            "a table quoted inside a string was read as a table: {quoted_table:?}"
        );
        Ok(())
    }

    /// The positive control for the census reader: the entries a manifest
    /// declares are exactly the entries it declares, and a `[workspace]` pool is
    /// not one of them.
    ///
    /// `[workspace.dependencies]` states what a member MAY inherit. Nothing in
    /// it is an edge of the declaring package, which is why the pool is read as
    /// what it is rather than as a table whose name happens to contain the word.
    #[test]
    fn a_workspace_pool_declares_no_edge() -> Result<(), String> {
        let pool = declared(
            "[workspace.dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n",
        )?;
        assert!(pool.is_empty(), "{pool:?}");
        let asked = declared(
            "[dependencies]\nthreadpak-macroc.workspace = true\n\n[workspace.dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n",
        )?;
        assert_eq!(asked.len(), 1, "{asked:?}");
        assert!(
            asked
                .first()
                .is_some_and(|entry| entry.identity() == "threadpak-macroc"),
            "{asked:?}"
        );
        Ok(())
    }

    /// A platform-conditional edge is read exactly like the unconditional edge
    /// it conditions, under every kind and under either quote on the predicate.
    #[test]
    fn a_conditioned_edge_is_read_like_the_edge_it_conditions() -> Result<(), String> {
        let conditioned = declared(
            "[target.'cfg(unix)'.dev-dependencies]\nhelpers = { path = \"macros/macroc\" }\n",
        )?;
        assert_eq!(conditioned.len(), 1, "{conditioned:?}");
        assert!(
            conditioned
                .first()
                .is_some_and(|entry| entry.kind == EdgeKind::Development
                    && entry.path() == Some("macros/macroc")),
            "{conditioned:?}"
        );
        let whole_tree = declared(
            "target = { 'cfg(unix)' = { dependencies = { helpers = { path = \"macros/macroc\" } } } }\n",
        )?;
        assert_eq!(whole_tree.len(), 1, "{whole_tree:?}");
        Ok(())
    }

    /// A key path names a value or it names nothing, and nothing is not an
    /// empty string.
    #[test]
    fn a_key_path_that_names_nothing_is_absent_rather_than_empty() -> Result<(), String> {
        let document = "[toolchain]\nchannel = \"1.97.1\"\nlisted = [\"a\", \"b\"]\n"
            .parse::<toml::Table>()
            .map_err(|error| error.to_string())?;
        assert_eq!(
            string_at(&document, &["toolchain", "channel"]),
            Read::Known(String::from("1.97.1"))
        );
        assert_eq!(
            strings_at(&document, &["toolchain", "listed"]),
            Read::Known(vec![String::from("a"), String::from("b")])
        );
        assert!(
            string_at(&document, &["toolchain", "nothing"])
                .known()
                .is_none()
        );
        assert!(string_at(&document, &["toolchain"]).known().is_none());
        Ok(())
    }

    /// The census this repository commits, entry for entry.
    ///
    /// A reader replacement moves numbers only where the TREE moved, and this is
    /// where that is pinned rather than asserted. Nineteen entries across seven
    /// committed manifests; the root manifest declares none of them, because
    /// what it carries is a workspace POOL.
    ///
    /// It was fifteen before the decoders were admitted, and the four that
    /// arrived are the four this crate now reads through: `toml`,
    /// `pulldown-cmark`, `serde`, and `serde_json`, every one of them an entry
    /// of `xtask/Cargo.toml`. Nothing else moved — same manifests, same kinds,
    /// same keys, same declared packages and paths — so the number moved exactly
    /// where the tree did and nowhere else.
    #[test]
    fn the_committed_census_is_nineteen_entries() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let census: Vec<String> = snapshot
            .cargo()
            .census()
            .0
            .iter()
            .map(|entry| {
                format!(
                    "{} [{}] {} package={:?} path={:?}",
                    entry.manifest(),
                    entry.kind,
                    entry.key,
                    entry.package,
                    entry.path
                )
            })
            .collect();
        assert_eq!(census.len(), 19, "{census:#?}");
        assert!(
            census.iter().any(|entry| entry
                == "xtask/fixtures/renamed-consumer/Cargo.toml [dependencies] tp \
                    package=Some(\"threadpak\") path=Some(\"../../..\")"),
            "{census:#?}"
        );
        // The four the decoders brought, and no fifth.
        assert_eq!(
            census
                .iter()
                .filter(|entry| entry.starts_with("xtask/Cargo.toml"))
                .count(),
            6,
            "{census:#?}"
        );
        assert!(
            snapshot.cargo().census().of("Cargo.toml").is_empty(),
            "the root manifest's workspace pool was read as an edge"
        );
        Ok(())
    }
}
