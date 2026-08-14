//! The topology law: which package may reach which.
//!
//! The edges run one way and inward. The machine depends on nothing that
//! projects its contracts and nothing that judges it; a compiler service depends
//! on no frontend of its own. Both halves are read off declared manifests rather
//! than off a resolved graph, because a manifest is where the edge is written and
//! where a reviewer will look for it.

use std::fs;
use std::path::Path;

use crate::repository::manifest::{Unread, dependency_declarations};
use crate::repository::walk::{JUDGE_DIRECTORY, TOOLING_DIRECTORY};

/// The metaprogramming packages the core package may never reach.
const TOOLING_PACKAGES: [&str; 2] = ["threadpak-macroc", "threadpak-macros"];

/// The services package, and the one manifest this law reads for the second
/// absence.
const SERVICES_MANIFEST: &str = "macros/macroc/Cargo.toml";

/// The Rust-facing expansion surface over the services.
const FRONTEND_PACKAGE: &str = "threadpak-macros";

/// The directory that surface lives in, under [`TOOLING_DIRECTORY`].
const FRONTEND_DIRECTORY: &str = "proc";

/// The qualification plane — the machine's judge.
const JUDGE_PACKAGE: &str = "threadpak-testpak";

/// The topology law, in two parts.
///
/// **Part one: the core never depends on tooling, and never on its judge.** The
/// `threadpak` package carries no dependency edge to the metaprogramming
/// tooling or to `testpak` under any Cargo edge kind. The edges run one way and
/// inward — `macros/proc` → `macros/macroc` → `threadpak`, and `testpak` →
/// everything — so the machine never depends on the tools that project its
/// contracts and never depends on the plane that judges it. Those lawful inward
/// edges live in the subsystem manifests; part one reads the ROOT manifest only,
/// where any such edge at all is a reversal of the topology.
///
/// **Part two: macroc never depends on its frontends.** A compiler service
/// never depends on its frontend surfaces, EVEN FOR TESTS. So the services
/// manifest carries no edge to `threadpak-macros` under any kind either — a dev
/// edge is still an edge, and a composition test bought with one is the
/// participant grading itself. Composition is proven from outside the
/// participants, by the consumer fixture at `xtask/fixtures/macro-consumer`.
///
/// **Both parts refuse a manifest they cannot read.** An absence is only worth
/// reporting by a reader that would have seen the presence, so a manifest
/// written in a shape [`dependency_declarations`] does not enter is refused
/// here rather than passed. That is a third kind of offence and it says so in
/// its own words: nobody reached tooling, and nobody established that anybody
/// had not. Three shapes arrive that way — a dependency table written inline, a
/// value that does not close on the line that opens it, and a basic string
/// carrying an escape this reader does not decode and Cargo does — and each
/// carries the repair that shape takes, because they do not take the same one.
pub(crate) fn check_no_core_tooling_edge(root: &Path) -> Result<(), String> {
    let manifest =
        fs::read_to_string(root.join("Cargo.toml")).map_err(|e| format!("Cargo.toml: {e}"))?;
    let services = fs::read_to_string(root.join(SERVICES_MANIFEST))
        .map_err(|e| format!("{SERVICES_MANIFEST}: {e}"))?;
    let mut reported = core_tooling_edge_violations(&manifest);
    reported.extend(services_frontend_edge_violations(&services));
    if reported.is_empty() {
        Ok(())
    } else {
        Err(reported.join("; "))
    }
}

/// Every offence the root manifest commits: one description per tooling edge,
/// then one per dependency table the reader could not enter.
///
/// An entry's PACKAGE IDENTITY is its `package = "…"` key when it carries one
/// and its own key otherwise, and an entry is a violation when that identity
/// names a tooling package or the judge, or when its `path` points into the
/// tooling subsystem directory or the judge's directory. Renaming therefore
/// hides nothing, and neither does spelling: the reader resolves every Cargo
/// spelling of an entry to the key path Cargo itself resolves it to, so the
/// identity judged here is the package Cargo would build against rather than
/// the text a line happened to start with.
///
/// Each offence carries its own claim rather than a shared prefix, because the
/// two kinds are not the same finding: one says an edge is there, the other
/// says this law could not look.
fn core_tooling_edge_violations(manifest_text: &str) -> Vec<String> {
    let declared = dependency_declarations(manifest_text);
    let mut found: Vec<String> = declared
        .entries
        .into_iter()
        .filter_map(|(kind, key, package, path)| {
            judge_dependency(kind, &key, package.as_deref(), path.as_deref())
        })
        .map(|violation| format!("core package reaches tooling or its judge: {violation}"))
        .collect();
    found.extend(declared.unread.iter().map(|shape| unread("core", shape)));
    found
}

/// Every offence the services manifest commits, read exactly like the core
/// manifest: package identity first, so a renamed entry betrays itself, then
/// the declared path, so an entry named anything at all that reaches into
/// `macros/proc/` is caught by where it points, then the tables the reader
/// could not enter.
fn services_frontend_edge_violations(manifest_text: &str) -> Vec<String> {
    let declared = dependency_declarations(manifest_text);
    let mut found: Vec<String> = declared
        .entries
        .into_iter()
        .filter_map(|(kind, key, package, path)| {
            judge_frontend_dependency(kind, &key, package.as_deref(), path.as_deref())
        })
        .map(|violation| format!("services reach their expansion surface: {violation}"))
        .collect();
    found.extend(
        declared
            .unread
            .iter()
            .map(|shape| unread("services", shape)),
    );
    found
}

/// The offence one unreadable declaration commits.
///
/// Not an edge — a manifest this law cannot read. What it declares sits inside a
/// value, and this law reads lines, so the absence it would otherwise report
/// about such a manifest is an absence nobody established. An unreadable
/// declaration is refused rather than passed, which is the difference between a
/// law and a habit, and the repair is in the message because the repair is one
/// line either way. Which one line differs, so each shape says its own.
fn unread(manifest: &str, shape: &Unread) -> String {
    match *shape {
        Unread::InlineTable(ref spelling) => format!(
            "the {manifest} manifest declares `{spelling}` as an inline table, and the entries \
             inside it are not read here: spell it as `[{spelling}]` with its entries on their own \
             lines, so this law can see what it is being asked to allow"
        ),
        Unread::MultiLineValue(ref spelling) => format!(
            "the {manifest} manifest gives `{spelling}` a value that does not close on the line \
             that opens it, and every line it spans is read here as structure rather than as the \
             data it is: put the value on one line, so this law cannot be told a dependency table \
             ended where a string, a list, or an inline table merely continued"
        ),
        Unread::EscapedValue(ref spelling) => format!(
            "the {manifest} manifest gives `{spelling}` a basic string carrying a `\\` escape, \
             which Cargo decodes before it resolves anything and this law does not decode at all, \
             so `\\u002D` reads here as six characters where Cargo reads a `-`: write the value \
             with the characters it means, or write it as a literal string in single quotes, \
             which TOML leaves undecoded and this law reads as it stands"
        ),
    }
}

/// The violation one dependency entry of the ROOT manifest commits, if any.
fn judge_dependency(
    kind: &str,
    key: &str,
    package: Option<&str>,
    path: Option<&str>,
) -> Option<String> {
    let identity = package.unwrap_or(key);
    if TOOLING_PACKAGES.contains(&identity) || identity == JUDGE_PACKAGE {
        return Some(format!("[{kind}] `{key}` resolves to package `{identity}`"));
    }
    if let Some(path) = path {
        if points_into(path, TOOLING_DIRECTORY) {
            return Some(format!(
                "[{kind}] `{key}` has path `{path}` inside `{TOOLING_DIRECTORY}/`"
            ));
        }
        if points_into(path, JUDGE_DIRECTORY) {
            return Some(format!(
                "[{kind}] `{key}` has path `{path}` inside `{JUDGE_DIRECTORY}/`"
            ));
        }
    }
    None
}

/// The violation one dependency entry of the SERVICES manifest commits, if any.
fn judge_frontend_dependency(
    kind: &str,
    key: &str,
    package: Option<&str>,
    path: Option<&str>,
) -> Option<String> {
    let identity = package.unwrap_or(key);
    if identity == FRONTEND_PACKAGE {
        return Some(format!(
            "[{kind}] `{key}` resolves to package `{FRONTEND_PACKAGE}`"
        ));
    }
    if let Some(path) = path
        && points_into(path, FRONTEND_DIRECTORY)
    {
        return Some(format!(
            "[{kind}] `{key}` has path `{path}` inside `{FRONTEND_DIRECTORY}/`"
        ));
    }
    None
}

/// Whether a dependency path enters one named directory. The segment is matched
/// wherever it appears, so `../proc`, `macros/proc`, and any longer detour that
/// lands there are all the same edge.
fn points_into(path: &str, directory: &str) -> bool {
    path.replace('\\', "/")
        .split('/')
        .any(|segment| segment == directory)
}

/// Planted reversals for a law whose subject is text rather than the tree.
///
/// A check that cannot fail is not a check. Each reversal here hands
/// [`core_tooling_edge_violations`] or [`services_frontend_edge_violations`] a
/// synthetic manifest that reverses the topology one Cargo edge kind at a time
/// and proves the reversal is caught. The manifests are written here rather than
/// on disk: the law is proven against fixture text, never by dirtying the
/// repository it guards.
#[cfg(test)]
mod tests {
    use super::{
        SERVICES_MANIFEST, core_tooling_edge_violations, services_frontend_edge_violations,
    };
    use crate::repository::walk::repo_root;
    use std::fs;
    use std::path::PathBuf;

    /// The manifest preamble every core fixture shares: the core package itself.
    const PREAMBLE: &str = "[package]\nname = \"threadpak\"\n\n";

    /// The manifest preamble every services fixture shares.
    const SERVICES_PREAMBLE: &str = "[package]\nname = \"threadpak-macroc\"\n\n";

    /// The violations a fixture manifest commits, preamble supplied.
    fn violations(body: &str) -> Vec<String> {
        core_tooling_edge_violations(&format!("{PREAMBLE}{body}"))
    }

    /// The frontend violations a services fixture manifest commits.
    fn services_violations(body: &str) -> Vec<String> {
        services_frontend_edge_violations(&format!("{SERVICES_PREAMBLE}{body}"))
    }

    /// Reversal (a): the plain edge — a normal dependency on the services.
    #[test]
    fn a_normal_tooling_dependency_is_a_violation() {
        let found = violations("[dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macroc")));
    }

    /// Reversal (b): the disguised edge — the package renamed at the key, so
    /// only the resolved package identity betrays it.
    #[test]
    fn a_renamed_tooling_dependency_is_a_violation() {
        let found = violations(
            "[dependencies]\nhelpers = { package = \"threadpak-macroc\", version = \"0.0.0\" }\n",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macroc")));
    }

    /// Reversal (c): the test-only edge.
    #[test]
    fn a_tooling_dev_dependency_is_a_violation() {
        let found =
            violations("[dev-dependencies]\nthreadpak-macros = { path = \"macros/proc\" }\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("dev-dependencies")));
    }

    /// Reversal (d): the build-script edge.
    #[test]
    fn a_tooling_build_dependency_is_a_violation() {
        let found =
            violations("[build-dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("build-dependencies")));
    }

    /// Reversal (e): the platform-conditional edge, which no scan of the three
    /// bare tables would ever see.
    #[test]
    fn a_target_specific_tooling_dependency_is_a_violation() {
        let found = violations(
            "[target.'cfg(unix)'.dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macroc")));
    }

    /// Reversal (f): the path edge — a dependency named anything at all whose
    /// path reaches into the tooling subsystem directory.
    #[test]
    fn a_path_into_the_tooling_directory_is_a_violation() {
        let found = violations("[dependencies]\nhelpers = { path = \"macros/macroc\" }\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("macros/")));
    }

    /// Reversal (g): the judge edge — the machine taking an ordinary dependency
    /// on the plane that judges it. Production never depends on its judge.
    #[test]
    fn a_core_dependency_on_the_judge_is_a_violation() {
        let found = violations("[dependencies]\nthreadpak-testpak = { path = \"testpak\" }\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-testpak")));
    }

    /// Reversal (h): the judge edge bought for tests only — the shape a "just
    /// for the test harness" edge actually takes.
    #[test]
    fn a_core_dev_dependency_on_the_judge_is_a_violation() {
        let found = violations("[dev-dependencies]\nthreadpak-testpak = { path = \"testpak\" }\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("dev-dependencies")));
    }

    /// Reversal (i): the disguised judge edge — renamed at the key, and
    /// separately an entry named anything at all whose path reaches into the
    /// judge's directory.
    #[test]
    fn a_renamed_or_path_edge_to_the_judge_is_a_violation() {
        let renamed = violations(
            "[dependencies]\nharness = { package = \"threadpak-testpak\", version = \"0.0.0\" }\n",
        );
        assert_eq!(renamed.len(), 1, "{renamed:?}");
        assert!(renamed.iter().any(|v| v.contains("threadpak-testpak")));
        let by_path = violations("[dependencies]\nharness = { path = \"testpak\" }\n");
        assert_eq!(by_path.len(), 1, "{by_path:?}");
        assert!(by_path.iter().any(|v| v.contains("testpak/")));
    }

    /// Reversal (j): the DOTTED edge — workspace inheritance written
    /// `name.workspace = true` rather than `name = { workspace = true }`.
    ///
    /// The two are one declaration to Cargo. They were not one declaration to
    /// this law: a reader that cut the line at its first `=` saw the key
    /// `threadpak-macroc.workspace`, which matches no package, so the edge
    /// passed. The repository answered that with a comment in the root manifest
    /// telling authors never to use the dotted spelling — and a prose "never"
    /// is not an invariant. This is the invariant.
    #[test]
    fn a_dotted_workspace_inheritance_is_a_violation() {
        let found = violations("[dependencies]\nthreadpak-macroc.workspace = true\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macroc")));
    }

    /// Reversal (k): the same dotted shape carrying the fields that hide an
    /// edge — `name.package` and `name.path` — and spelled across several
    /// lines, which is how a dotted entry is usually written.
    ///
    /// The lines of one dotted entry accumulate into the one entry they
    /// declare, so a renamed edge is caught once rather than reported twice or
    /// missed entirely because its identity and its version sat on different
    /// lines.
    #[test]
    fn a_dotted_entry_spelled_across_lines_is_one_violation() {
        let renamed = violations(
            "[dependencies]\nhelpers.version = \"0.0.0\"\nhelpers.package = \"threadpak-macroc\"\n",
        );
        assert_eq!(renamed.len(), 1, "{renamed:?}");
        assert!(renamed.iter().any(|v| v.contains("threadpak-macroc")));
        let by_path = violations("[dependencies]\nhelpers.path = \"macros/macroc\"\n");
        assert_eq!(by_path.len(), 1, "{by_path:?}");
        assert!(by_path.iter().any(|v| v.contains("macros/")));
    }

    /// Reversal (l): the quoted key, under either TOML quote, and quoted at the
    /// head of a dotted key. A package name never needs quoting, which is
    /// exactly why quoting it was somewhere to hide.
    #[test]
    fn a_quoted_entry_key_is_a_violation() {
        let literal = violations("[dependencies]\n'threadpak-macroc' = { version = \"0.0.0\" }\n");
        assert_eq!(literal.len(), 1, "{literal:?}");
        assert!(literal.iter().any(|v| v.contains("threadpak-macroc")));
        let dotted = violations("[dependencies]\n\"threadpak-macroc\".workspace = true\n");
        assert_eq!(dotted.len(), 1, "{dotted:?}");
        assert!(dotted.iter().any(|v| v.contains("threadpak-macroc")));
    }

    /// Reversal (m): the quoted TABLE header. Every segment of a key path may
    /// be quoted, including the ones naming the edge kind and the `target`
    /// prefix, and a header nobody recognized used to close the table rather
    /// than open it — which left every entry beneath it unread.
    #[test]
    fn a_quoted_table_header_is_a_violation() {
        let quoted_kind =
            violations("[\"dependencies\"]\nthreadpak-macroc = { version = \"0.0.0\" }\n");
        assert_eq!(quoted_kind.len(), 1, "{quoted_kind:?}");
        assert!(quoted_kind.iter().any(|v| v.contains("threadpak-macroc")));
        let quoted_target = violations(
            "[\"target\".'cfg(unix)'.\"dev-dependencies\"]\n\
             threadpak-macroc = { version = \"0.0.0\" }\n",
        );
        assert_eq!(quoted_target.len(), 1, "{quoted_target:?}");
        assert!(quoted_target.iter().any(|v| v.contains("dev-dependencies")));
    }

    /// Reversal (n): the table named by the KEY rather than by a header, which
    /// is what a dotted key written before any header declares. There is no
    /// `[dependencies]` line anywhere in this manifest and the edge is there
    /// all the same.
    #[test]
    fn a_dotted_table_key_before_any_header_is_a_violation() {
        let found = core_tooling_edge_violations(
            "dependencies.threadpak-macroc.workspace = true\n\n[package]\nname = \"threadpak\"\n",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macroc")));
    }

    /// Reversal (o): the sub-table form, `[KIND.name]` with its fields beneath
    /// it, plainly and under a `target.'…'` prefix and with the identity
    /// renamed. The reader has always read this shape; nothing had ever proven
    /// it, so it stood on a reading rather than on a reversal.
    #[test]
    fn a_sub_table_dependency_is_a_violation() {
        let plain = violations("[dependencies.threadpak-macroc]\nversion = \"0.0.0\"\n");
        assert_eq!(plain.len(), 1, "{plain:?}");
        assert!(plain.iter().any(|v| v.contains("threadpak-macroc")));
        let renamed = violations("[dependencies.helpers]\npackage = \"threadpak-macroc\"\n");
        assert_eq!(renamed.len(), 1, "{renamed:?}");
        assert!(renamed.iter().any(|v| v.contains("threadpak-macroc")));
        let conditional = violations(
            "[target.'cfg(unix)'.dev-dependencies.threadpak-macroc]\nversion = \"0.0.0\"\n",
        );
        assert_eq!(conditional.len(), 1, "{conditional:?}");
        assert!(conditional.iter().any(|v| v.contains("dev-dependencies")));
    }

    /// Reversal (p): the literal-string VALUE. TOML's two string forms are one
    /// value, so a path written in single quotes is the same edge as a path
    /// written in double quotes, and reading only one of them read only half
    /// the declarations a path can be written in.
    #[test]
    fn a_literal_string_path_is_a_violation() {
        let found = violations("[dependencies]\nhelpers = { path = 'macros/macroc' }\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("macros/")));
    }

    /// Reversal (q): a comment after the table header. The header still opens
    /// the table, so the entries beneath it are still read — a comment is not a
    /// place to put a dependency table.
    #[test]
    fn a_header_carrying_a_comment_still_opens_its_table() {
        let found = violations(
            "[dependencies] # inherited from the workspace\n\
             threadpak-macroc = { version = \"0.0.0\" }\n",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macroc")));
    }

    /// Reversal (r): the dependency table written as an INLINE table, whose
    /// entries sit inside one line's value.
    ///
    /// This one is refused rather than read, and the refusal is the point. A
    /// line-oriented reader does not enter a value, so what it would report
    /// about such a manifest is an absence it never established. The law says
    /// so and fails, which is what separates a ceiling that is written down
    /// from a ceiling that is enforced.
    #[test]
    fn an_inline_dependency_table_is_refused_unread() {
        let at_root = core_tooling_edge_violations(
            "dependencies = { threadpak-macroc = { path = \"macros/macroc\" } }\n\n\
             [package]\nname = \"threadpak\"\n",
        );
        assert_eq!(at_root.len(), 1, "{at_root:?}");
        assert!(at_root.iter().any(|v| v.contains("inline table")));
        let under_target = violations(
            "[target.'cfg(unix)']\ndependencies = { threadpak-macroc = { version = \"0.0.0\" } }\n",
        );
        assert_eq!(under_target.len(), 1, "{under_target:?}");
        assert!(under_target.iter().any(|v| v.contains("inline table")));
        let whole_tree = core_tooling_edge_violations(
            "target = { 'cfg(unix)' = { dependencies = { threadpak-macroc = { version = \
             \"0.0.0\" } } } }\n\n[package]\nname = \"threadpak\"\n",
        );
        assert_eq!(whole_tree.len(), 1, "{whole_tree:?}");
        assert!(whole_tree.iter().any(|v| v.contains("inline table")));
    }

    /// Reversal (s): THE bypass, in the shape it was executed in.
    ///
    /// A dependency sub-table carries a multi-line basic string whose body
    /// holds one line shaped like a table header, and the rename is written
    /// AFTER the string. MEASURED on cargo 1.97.1 against the real root
    /// manifest, with this law run as its own compiled binary: cargo accepts
    /// the manifest, `cargo tree` resolves `threadpak → threadpak-macroc`, and
    /// the law printed PASS — because by the time the reader reached `package`
    /// it believed the dependency table had ended three lines earlier.
    ///
    /// Nothing about the edge is read now, and that is the point: the offence
    /// is the unreadable value, reported before the reader can be lied to
    /// about what follows it.
    #[test]
    fn a_multi_line_string_inside_a_dependency_table_is_refused_unread() {
        let found = violations(
            "[dev-dependencies.helpers]\nversion = \"0.0.0\"\nmetadata = \"\"\"\n\
             this string contains a line that looks like a header\n[package]\n\
             and keeps going\n\"\"\"\npackage = \"threadpak-macroc\"\n\
             path = \"macros/macroc\"\n",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found
                .iter()
                .any(|v| v.contains("dev-dependencies.helpers.metadata")
                    && v.contains("does not close on the line that opens it")),
            "{found:?}"
        );
    }

    /// Reversal (t): the same bypass in TOML's other multi-line string. Two
    /// spellings of one construct are one construct, and reading only the
    /// basic one would have left the literal one open.
    #[test]
    fn a_multi_line_literal_string_inside_a_dependency_table_is_refused_unread() {
        let found = violations(
            "[dev-dependencies.helpers]\nmetadata = '''\n[package]\n'''\n\
             package = \"threadpak-macroc\"\npath = \"macros/macroc\"\n",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found.iter().any(|v| v.contains("does not close")),
            "{found:?}"
        );
    }

    /// Reversal (u): the bracket list broken across lines, one of whose
    /// elements is itself a list and so occupies a line shaped like a header.
    /// Measured accepted by cargo and resolving the same edge.
    #[test]
    fn a_multi_line_list_inside_a_dependency_table_is_refused_unread() {
        let found = violations(
            "[dev-dependencies.helpers]\nmetadata = [\n[1]\n]\n\
             package = \"threadpak-macroc\"\npath = \"macros/macroc\"\n",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found.iter().any(|v| v.contains("does not close")),
            "{found:?}"
        );
    }

    /// Reversal (v): the inline table broken across lines. Cargo takes it, and
    /// its continuation lines are read here as fresh entries of the enclosing
    /// table, so the entry that actually carries the rename is left holding
    /// nothing but its own innocent key.
    #[test]
    fn an_inline_table_broken_across_lines_is_refused_unread() {
        let found = violations(
            "[dev-dependencies]\nhelpers = { version = \"0.0.0\",\n\
             package = \"threadpak-macroc\",\npath = \"macros/macroc\" }\n",
        );
        assert!(
            found
                .iter()
                .any(|v| v.contains("dev-dependencies.helpers") && v.contains("does not close")),
            "{found:?}"
        );
    }

    /// Reversal (w): the spanning value that is nowhere near a dependency
    /// table, which is why the refusal is on every line rather than on the
    /// lines inside one.
    ///
    /// The string sits at the top of the file and quotes a header. Everything
    /// written after it is read at that phantom table, so the dotted
    /// dependency keys below resolve for cargo — measured accepted, resolving
    /// `threadpak → threadpak-macroc` — and reach no law here at all.
    #[test]
    fn a_spanning_value_outside_any_dependency_table_is_refused_unread() {
        let found = core_tooling_edge_violations(
            "notes = \"\"\"\n[x]\n\"\"\"\n\
             dev-dependencies.helpers.package = \"threadpak-macroc\"\n\
             dev-dependencies.helpers.path = \"macros/macroc\"\n\n\
             [package]\nname = \"threadpak\"\n",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("`notes`")), "{found:?}");
    }

    /// Reversal (x): the second seat reads the same refusal. Both halves of
    /// this law stand on one manifest reader, and that is a claim, so it is
    /// executed rather than asserted.
    #[test]
    fn a_spanning_value_in_the_services_manifest_is_refused_unread() {
        let found = services_violations(
            "[dev-dependencies.shell]\nmetadata = \"\"\"\n[package]\n\"\"\"\n\
             package = \"threadpak-macros\"\n",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found.iter().any(|v| v.contains("does not close")),
            "{found:?}"
        );
    }

    /// The direction the refusal is allowed to be wrong in, stated by what it
    /// does NOT refuse: every value here closes on the line that opens it, so
    /// every one of them is read.
    ///
    /// The single-line `"""…"""` form is a self-contained value and passes; a
    /// `#` inside a literal string is content and does not end the line; a `\"`
    /// inside a basic string does not close it early; and a one-line inline
    /// table holding a bracket list nests in and back out to nothing. The
    /// declared `path` is read off the last of them, which is how this control
    /// proves the reader still WORKS rather than merely stays quiet.
    #[test]
    fn a_value_that_closes_on_its_line_is_read_rather_than_refused() {
        let found = violations(
            "[dependencies]\nserde = { version = \"1\", features = [\"derive\", \"std\"] }\n\n\
             [dependencies.helpers]\nmetadata = \"\"\"one line\"\"\"\n\
             notes = 'a # not a comment'\n\
             path = 'vendor/helpers'\n",
        );
        assert!(found.is_empty(), "{found:?}");
        let reached = violations(
            "[dependencies.helpers]\nmetadata = \"\"\"one line\"\"\"\n\
             path = 'macros/macroc'\n",
        );
        assert_eq!(reached.len(), 1, "{reached:?}");
        assert!(reached.iter().any(|v| v.contains("macros/")), "{reached:?}");
    }

    /// Reversal (y): the escape sequence inside a value this law compares by
    /// TEXT — a different mechanism from every reversal above, and the second
    /// hole this reader was measured falling into.
    ///
    /// MEASURED on cargo 1.97.1 against the real root manifest with the gate run
    /// as its own compiled binary: cargo decodes both escapes, `cargo tree -e
    /// dev` resolves `threadpak -> threadpak-macroc`, and the law printed PASS.
    /// No line lies about being structure here; the reader compared the text of
    /// a value against names Cargo compares the DECODED value against, and
    /// `\u002D` is six characters here and a `-` there.
    ///
    /// The escape is refused wherever it sits and whatever it spells, including
    /// where the old normalization happened to catch it anyway: `\u002F` in a
    /// path used to be turned into a separator by the same pass that reads
    /// `..\macros` as a Windows spelling, which caught the edge by luck rather
    /// than by law, and luck is not what an absence may rest on.
    #[test]
    fn a_basic_string_carrying_an_escape_is_refused_unread() {
        let hidden = violations(
            "[dev-dependencies]\nhelpers = { package = \"threadpak\\u002Dmacroc\", \
             path = \"\\u006Dacros/macroc\" }\n",
        );
        assert_eq!(hidden.len(), 1, "{hidden:?}");
        assert!(
            hidden
                .iter()
                .any(|v| v.contains("dev-dependencies.helpers") && v.contains("escape")),
            "{hidden:?}"
        );
        let lucky = violations(
            "[dev-dependencies]\nhelpers = { package = \"threadpak-macroc\", \
             path = \"macros\\u002Fmacroc\" }\n",
        );
        assert!(lucky.iter().any(|v| v.contains("escape")), "{lucky:?}");
    }

    /// The refusal knows which shape it found, which is the whole reason there
    /// are two messages: an escaped value that CLOSES on its line is reported as
    /// an escape and never as a value that spans lines.
    ///
    /// This is what the escape-honouring inside the string skipper buys now that
    /// an escape is refused on its own account. A skipper that stopped at the
    /// escaped quote would think the string ran off the end of the line, and
    /// would send the author to put a value on one line it is already on.
    #[test]
    fn an_escaped_value_that_closes_is_reported_as_an_escape() {
        let found = violations("[dependencies.helpers]\nnotes = \"a \\\" b\"\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("escape")), "{found:?}");
        assert!(
            !found.iter().any(|v| v.contains("does not close")),
            "{found:?}"
        );
    }

    /// The direction the escape refusal is allowed to be wrong in, stated by
    /// what it does NOT refuse: a LITERAL string decodes nothing, so a `\` in
    /// one is a backslash to TOML, to Cargo, and here.
    ///
    /// MEASURED, both halves. `path = 'macros\macroc'` is the lawful Windows
    /// spelling of a path: cargo accepts it and resolves the edge, and it is
    /// caught here as the edge it is — refusing it would be a false refusal on
    /// something nobody wrote to hide anything. And there is nothing to hide
    /// there: `package = 'threadpak\u002Dmacroc'` is refused by CARGO ITSELF
    /// with `invalid character \ in package name`, because no decoding ever
    /// turned those six characters into a `-`.
    #[test]
    fn a_literal_string_carrying_a_backslash_is_read_rather_than_refused() {
        let found = violations(
            "[dev-dependencies]\nhelpers = { package = 'threadpak-macroc', \
             path = 'macros\\macroc' }\n",
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found.iter().any(|v| v.contains("threadpak-macroc")),
            "{found:?}"
        );
        assert!(!found.iter().any(|v| v.contains("escape")), "{found:?}");
    }
    /// The positive control: a manifest with ordinary edges and none to the
    /// tooling or the judge is clean, so the law reports something real rather
    /// than everything.
    ///
    /// The dotted spellings are here too, on lawful entries. The reader that
    /// now refuses `threadpak-macroc.workspace = true` reads
    /// `serde.workspace = true` as the ordinary declaration it is: what changed
    /// is which package a key resolves to, not which spellings are allowed.
    #[test]
    fn a_manifest_without_tooling_edges_is_clean() {
        let found = violations(
            "[dependencies]\nserde = \"1\"\ntokio.workspace = true\ntokio.features = [\"rt\"]\n\n\
             [dev-dependencies]\ntrybuild = { version = \"1\" }\n\n\
             [target.'cfg(windows)'.dependencies]\nwindows-sys = { version = \"0\" }\n",
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// The workspace's declaration POOL is not an edge, and this is the line
    /// the dotted reversal above stands on.
    ///
    /// `[workspace.dependencies]` states what a member may inherit; nothing in
    /// it is a dependency of the core package, so naming the tooling there is
    /// lawful — testpak inherits from that table and is supposed to. The edge
    /// exists when `[dependencies]` asks for the inheritance, which is the
    /// second half below. A law that read a kind wherever the word appeared
    /// would refuse the pool and be wrong; a law that read the key path only
    /// where Cargo resolves one refuses the ask and is right.
    #[test]
    fn a_workspace_declaration_pool_is_not_an_edge() {
        let pool = violations(
            "[workspace.dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n",
        );
        assert!(pool.is_empty(), "{pool:?}");
        let asked = violations(
            "[dependencies]\nthreadpak-macroc.workspace = true\n\n\
             [workspace.dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n",
        );
        assert_eq!(asked.len(), 1, "{asked:?}");
        assert!(asked.iter().any(|v| v.contains("threadpak-macroc")));
    }

    /// The law judges the ROOT manifest, and the root manifest holds. The
    /// lawful inward edges — `macros/macroc` on the machine, `macros/proc` on
    /// `macros/macroc` — live in the subsystem manifests, which this law never
    /// reads and which are not violations.
    #[test]
    fn the_real_root_manifest_carries_no_tooling_edge() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let manifest = fs::read_to_string(root.join("Cargo.toml")).unwrap_or_default();
        assert!(!manifest.is_empty(), "root Cargo.toml is unreadable");
        let found = core_tooling_edge_violations(&manifest);
        assert!(found.is_empty(), "{found:?}");
    }

    /// Part-two reversal (a): the plain edge — the services taking an ordinary
    /// dependency on the surface that is supposed to call THEM.
    #[test]
    fn a_services_dependency_on_the_frontend_is_a_violation() {
        let found =
            services_violations("[dependencies]\nthreadpak-macros = { path = \"../proc\" }\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macros")));
    }

    /// Part-two reversal (b): the test-only edge — the exact shape the law was
    /// written to kill, since a composition test bought with a dev edge is the
    /// participant grading itself.
    #[test]
    fn a_services_dev_dependency_on_the_frontend_is_a_violation() {
        let found =
            services_violations("[dev-dependencies]\nthreadpak-macros = { path = \"../proc\" }\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("dev-dependencies")));
    }

    /// Part-two reversal (c): the disguised edge — the surface renamed at the
    /// key, and separately an entry named anything at all whose path reaches
    /// into the surface's directory.
    #[test]
    fn a_renamed_services_dependency_on_the_frontend_is_a_violation() {
        let renamed = services_violations(
            "[dev-dependencies]\nshell = { package = \"threadpak-macros\", version = \"0.0.0\" }\n",
        );
        assert_eq!(renamed.len(), 1, "{renamed:?}");
        assert!(renamed.iter().any(|v| v.contains("threadpak-macros")));
        let by_path = services_violations("[dependencies]\nshell = { path = \"../proc\" }\n");
        assert_eq!(by_path.len(), 1, "{by_path:?}");
        assert!(by_path.iter().any(|v| v.contains("proc/")));
    }

    /// Part-two reversal (d): the dotted edge at the second seat. Both halves
    /// of this law read one manifest reader, so a spelling caught at the core
    /// manifest is caught here too — and that is a claim, so it is executed
    /// rather than asserted.
    #[test]
    fn a_dotted_services_dependency_on_the_frontend_is_a_violation() {
        let found = services_violations("[dependencies]\nthreadpak-macros.workspace = true\n");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macros")));
    }

    /// The part-two positive control: the services depending only on the
    /// machine are clean, so the law reports something real rather than
    /// everything.
    #[test]
    fn a_services_manifest_without_a_frontend_edge_is_clean() {
        let found = services_violations("[dependencies]\nthreadpak = { path = \"../..\" }\n");
        assert!(found.is_empty(), "{found:?}");
    }

    /// Part two judges the real services manifest, and it holds.
    #[test]
    fn the_real_services_manifest_carries_no_frontend_edge() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let manifest = fs::read_to_string(root.join(SERVICES_MANIFEST)).unwrap_or_default();
        assert!(!manifest.is_empty(), "services Cargo.toml is unreadable");
        let found = services_frontend_edge_violations(&manifest);
        assert!(found.is_empty(), "{found:?}");
    }
}
