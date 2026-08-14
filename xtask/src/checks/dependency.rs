//! The topology law: which package may reach which.
//!
//! The edges run one way and inward. The machine depends on nothing that
//! projects its contracts and nothing that judges it; a compiler service depends
//! on no frontend of its own.
//!
//! # Two authorities, and the law stands on both
//!
//! **What the manifests DECLARE** is where a reviewer will look for an edge, and
//! it is read out of the decoded documents: key, resolved package identity,
//! declared path, edge kind, and any `target.<spec>` conditioning. Every Cargo
//! spelling arrives as one declaration, because the decoder resolved the
//! document before this law saw it.
//!
//! **What cargo RESOLVES** is a different question, and the manifests cannot
//! answer it: an edge can arrive through workspace inheritance, and a package
//! identity is something only the resolver settles. `cargo metadata` is asked
//! directly.
//!
//! Neither is a fallback for the other and neither is optional. If cargo could
//! not be asked, this law REFUSES: an absence nobody established is not an
//! absence, and "the core reaches no tooling" reported about a resolution that
//! never happened is the exact silence this repository is eliminating.

use crate::repository::cargo::{DeclaredDependency, MANIFEST_FILE, ResolvedWorkspace};
use crate::repository::snapshot::{JUDGE_DIRECTORY, RepositorySnapshot, TOOLING_DIRECTORY};

/// The metaprogramming packages the core package may never reach.
const TOOLING_PACKAGES: [&str; 2] = ["threadpak-macroc", "threadpak-macros"];

/// The machine's own package.
const CORE_PACKAGE: &str = "threadpak";

/// The services package, and the one manifest this law reads for the second
/// absence.
const SERVICES_MANIFEST: &str = "macros/macroc/Cargo.toml";

/// The services package, as cargo names it.
const SERVICES_PACKAGE: &str = "threadpak-macroc";

/// The Rust-facing expansion surface over the services.
const FRONTEND_PACKAGE: &str = "threadpak-macros";

/// The directory that surface lives in, under [`TOOLING_DIRECTORY`].
const FRONTEND_DIRECTORY: &str = "proc";

/// The qualification plane — the machine's judge.
const JUDGE_PACKAGE: &str = "threadpak-testpak";

/// The topology law, in two parts and over two authorities.
///
/// **Part one: the core never depends on tooling, and never on its judge.** The
/// `threadpak` package carries no dependency edge to the metaprogramming
/// tooling or to `testpak` under any Cargo edge kind. The edges run one way and
/// inward — `macros/proc` → `macros/macroc` → `threadpak`, and `testpak` →
/// everything — so the machine never depends on the tools that project its
/// contracts and never depends on the plane that judges it.
///
/// **Part two: macroc never depends on its frontends.** A compiler service
/// never depends on its frontend surfaces, EVEN FOR TESTS. So the services
/// carry no edge to `threadpak-macros` under any kind either — a dev edge is
/// still an edge, and a composition test bought with one is the participant
/// grading itself. Composition is proven from outside the participants, by the
/// consumer fixture at `xtask/fixtures/macro-consumer`.
///
/// **Both parts are asked of both authorities.** A declared edge is refused
/// where a manifest writes one; a resolved edge is refused where cargo resolves
/// one, whatever the manifests happen to spell. An edge arriving through
/// workspace inheritance is invisible to the first and plain to the second,
/// which is why the second is not optional and why an unavailable resolution
/// refuses rather than passes.
pub(crate) fn check_no_core_tooling_edge(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let census = snapshot.cargo().census();
    let mut reported: Vec<String> = census
        .of(MANIFEST_FILE)
        .into_iter()
        .filter_map(judge_core_declaration)
        .map(|violation| format!("core package reaches tooling or its judge: {violation}"))
        .collect();
    reported.extend(
        census
            .of(SERVICES_MANIFEST)
            .into_iter()
            .filter_map(judge_services_declaration)
            .map(|violation| format!("services reach their expansion surface: {violation}")),
    );
    let resolved = snapshot
        .cargo()
        .resolved()
        .required("what cargo resolved for this workspace")?;
    reported.extend(resolved_offences(
        resolved,
        CORE_PACKAGE,
        judge_core_package,
        "core package reaches tooling or its judge, as cargo resolved it",
    )?);
    reported.extend(resolved_offences(
        resolved,
        SERVICES_PACKAGE,
        judge_services_package,
        "services reach their expansion surface, as cargo resolved it",
    )?);
    if reported.is_empty() {
        Ok(())
    } else {
        Err(reported.join("; "))
    }
}

/// The violation one DECLARED entry of the root manifest commits, if any.
///
/// An entry's PACKAGE IDENTITY is its `package = "…"` key when it carries one
/// and its own key otherwise, which is Cargo's own rule — so renaming hides
/// nothing. Neither does spelling: the decoder resolved every Cargo spelling of
/// an entry to one declaration before this reading saw it.
fn judge_core_declaration(entry: &DeclaredDependency) -> Option<String> {
    let identity = entry.identity();
    if TOOLING_PACKAGES.contains(&identity) || identity == JUDGE_PACKAGE {
        return Some(format!("{entry} resolves to package `{identity}`"));
    }
    let path = entry.path()?;
    if points_into(path, TOOLING_DIRECTORY) {
        return Some(format!(
            "{entry} has path `{path}` inside `{TOOLING_DIRECTORY}/`"
        ));
    }
    if points_into(path, JUDGE_DIRECTORY) {
        return Some(format!(
            "{entry} has path `{path}` inside `{JUDGE_DIRECTORY}/`"
        ));
    }
    None
}

/// The violation one DECLARED entry of the services manifest commits, if any.
fn judge_services_declaration(entry: &DeclaredDependency) -> Option<String> {
    if entry.identity() == FRONTEND_PACKAGE {
        return Some(format!("{entry} resolves to package `{FRONTEND_PACKAGE}`"));
    }
    let path = entry.path()?;
    if points_into(path, FRONTEND_DIRECTORY) {
        return Some(format!(
            "{entry} has path `{path}` inside `{FRONTEND_DIRECTORY}/`"
        ));
    }
    None
}

/// Every offence one RESOLVED package commits.
///
/// The resolved reading judges package IDENTITY and nothing else. A resolved
/// path is absolute — it names where a checkout happens to sit — so reading a
/// directory out of one would make this law depend on what somebody called the
/// folder they cloned into. The declared reading is where a path is judged,
/// because a declared path is relative and is what a manifest actually states.
fn resolved_offences(
    resolved: &ResolvedWorkspace,
    package: &str,
    judge: fn(&str) -> Option<String>,
    claim: &str,
) -> Result<Vec<String>, String> {
    let found = resolved
        .package(package)
        .taken(&format!("the package `{package}` in what cargo resolved"))?;
    let mut offences = Vec::new();
    for edge in found.dependencies() {
        let Some(violation) = judge(edge.package()) else {
            continue;
        };
        let kind = edge.kind().taken("the edge kind cargo reported")?;
        let conditioned = match edge.target() {
            Some(predicate) => format!(" under `target.{predicate}`"),
            None => String::new(),
        };
        offences.push(format!(
            "{claim}: [{kind}] `{}`{conditioned} {violation}",
            edge.key()
        ));
    }
    Ok(offences)
}

/// The violation one package the CORE resolved to commits, if any.
fn judge_core_package(identity: &str) -> Option<String> {
    if TOOLING_PACKAGES.contains(&identity) || identity == JUDGE_PACKAGE {
        Some(format!("resolves to package `{identity}`"))
    } else {
        None
    }
}

/// The violation one package the SERVICES resolved to commits, if any.
fn judge_services_package(identity: &str) -> Option<String> {
    if identity == FRONTEND_PACKAGE {
        Some(format!("resolves to package `{FRONTEND_PACKAGE}`"))
    } else {
        None
    }
}

/// Whether a declared dependency path enters one named directory. The segment is
/// matched wherever it appears, so `../proc`, `macros/proc`, and any longer
/// detour that lands there are all the same edge.
fn points_into(path: &str, directory: &str) -> bool {
    path.replace('\\', "/")
        .split('/')
        .any(|segment| segment == directory)
}

/// Planted reversals for a law whose declared half is text.
///
/// A check that cannot fail is not a check. Each reversal hands the DECLARED
/// reading a synthetic manifest that reverses the topology one Cargo edge kind
/// at a time and proves the reversal is caught. The manifests are written here
/// rather than on disk: the law is proven against fixture text, never by
/// dirtying the repository it guards.
///
/// The spellings are no longer the subject. Eleven of these reversals used to
/// exist because a line reader had to be taught one spelling at a time; the
/// decoder resolves them all to one declaration, and where every spelling of one
/// declaration lands identically is proven once, in `repository::cargo`, rather
/// than eleven times here.
#[cfg(test)]
mod tests {
    use super::{check_no_core_tooling_edge, judge_core_declaration, judge_services_declaration};
    use crate::repository::cargo::dependency_declarations;
    use crate::repository::snapshot::repository_snapshot;
    use crate::repository::types::CanonicalPath;

    /// The violations a fixture manifest commits, as the CORE manifest.
    fn violations(body: &str) -> Result<Vec<String>, String> {
        Ok(entries(body)?
            .iter()
            .filter_map(judge_core_declaration)
            .collect())
    }

    /// The violations a fixture manifest commits, as the SERVICES manifest.
    fn services_violations(body: &str) -> Result<Vec<String>, String> {
        Ok(entries(body)?
            .iter()
            .filter_map(judge_services_declaration)
            .collect())
    }

    /// The declared entries of one fixture manifest.
    fn entries(body: &str) -> Result<Vec<crate::repository::cargo::DeclaredDependency>, String> {
        let document = body
            .parse::<toml::Table>()
            .map_err(|error| format!("fixture manifest does not decode: {error}"))?;
        Ok(dependency_declarations(
            &CanonicalPath::spelled("Cargo.toml"),
            &document,
        ))
    }

    /// Reversal (a): the plain edge — a normal dependency on the services.
    #[test]
    fn a_normal_tooling_dependency_is_a_violation() -> Result<(), String> {
        let found =
            violations("[dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n")?;
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macroc")));
        Ok(())
    }

    /// Reversal (b): the disguised edge — the package renamed at the key, so
    /// only the resolved package identity betrays it.
    #[test]
    fn a_renamed_tooling_dependency_is_a_violation() -> Result<(), String> {
        let found = violations(
            "[dependencies]\nhelpers = { package = \"threadpak-macroc\", version = \"0.0.0\" }\n",
        )?;
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macroc")));
        Ok(())
    }

    /// Reversal (c): the test-only edge, and (d) the build-script edge. A dev
    /// edge is still an edge and a build edge is still an edge.
    #[test]
    fn a_dev_or_build_tooling_dependency_is_a_violation() -> Result<(), String> {
        let dev =
            violations("[dev-dependencies]\nthreadpak-macros = { path = \"macros/proc\" }\n")?;
        assert_eq!(dev.len(), 1, "{dev:?}");
        assert!(dev.iter().any(|v| v.contains("dev-dependencies")));
        let build =
            violations("[build-dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n")?;
        assert_eq!(build.len(), 1, "{build:?}");
        assert!(build.iter().any(|v| v.contains("build-dependencies")));
        Ok(())
    }

    /// Reversal (e): the platform-conditional edge, which no scan of the three
    /// bare tables would ever see.
    #[test]
    fn a_target_specific_tooling_dependency_is_a_violation() -> Result<(), String> {
        let found = violations(
            "[target.'cfg(unix)'.dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n",
        )?;
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macroc")));
        Ok(())
    }

    /// Reversal (f): the path edge — a dependency named anything at all whose
    /// path reaches into the tooling subsystem directory.
    #[test]
    fn a_path_into_the_tooling_directory_is_a_violation() -> Result<(), String> {
        let found = violations("[dependencies]\nhelpers = { path = \"macros/macroc\" }\n")?;
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("macros/")));
        Ok(())
    }

    /// Reversal (g)–(i): the judge edges — ordinary, test-only, renamed, and by
    /// path. Production never depends on its judge, and "just for the test
    /// harness" is the shape that edge actually takes.
    #[test]
    fn any_edge_to_the_judge_is_a_violation() -> Result<(), String> {
        for body in [
            "[dependencies]\nthreadpak-testpak = { path = \"testpak\" }\n",
            "[dev-dependencies]\nthreadpak-testpak = { path = \"testpak\" }\n",
            "[dependencies]\nharness = { package = \"threadpak-testpak\", version = \"0.0.0\" }\n",
            "[dependencies]\nharness = { path = \"testpak\" }\n",
        ] {
            let found = violations(body)?;
            assert_eq!(found.len(), 1, "{body} -> {found:?}");
        }
        Ok(())
    }

    /// Reversal (j): the DOTTED edge — workspace inheritance written
    /// `name.workspace = true` rather than `name = { workspace = true }`.
    ///
    /// The two are one declaration to Cargo. They were not one declaration to
    /// this law: a reader that cut the line at its first `=` saw the key
    /// `threadpak-macroc.workspace`, which matches no package, so the edge
    /// passed. The repository answered that with a comment in the root manifest
    /// telling authors never to use the dotted spelling — and a prose "never"
    /// is not an invariant. The decoder is the invariant.
    #[test]
    fn a_dotted_workspace_inheritance_is_a_violation() -> Result<(), String> {
        let found = violations("[dependencies]\nthreadpak-macroc.workspace = true\n")?;
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("threadpak-macroc")));
        Ok(())
    }

    /// Reversal (r), and it is no longer a ceiling: the dependency table written
    /// as an INLINE table.
    ///
    /// A line-oriented reader does not enter a value, so it could not read this
    /// at all, and the honest answer it had was to REFUSE the manifest as
    /// unread. The decoder enters the value, so the edge inside is now the edge
    /// it always was — a violation, named as one, rather than a manifest this
    /// law could not look at.
    #[test]
    fn an_inline_dependency_table_is_read_as_the_edge_it_carries() -> Result<(), String> {
        let at_root =
            violations("dependencies = { threadpak-macroc = { path = \"macros/macroc\" } }\n")?;
        assert_eq!(at_root.len(), 1, "{at_root:?}");
        assert!(at_root.iter().any(|v| v.contains("threadpak-macroc")));
        let whole_tree = violations(
            "target = { 'cfg(unix)' = { dependencies = { threadpak-macroc = { version = \
             \"0.0.0\" } } } }\n",
        )?;
        assert_eq!(whole_tree.len(), 1, "{whole_tree:?}");
        Ok(())
    }

    /// The positive control: a manifest with ordinary edges and none to the
    /// tooling or the judge is clean, so the law reports something real rather
    /// than everything.
    #[test]
    fn a_manifest_without_tooling_edges_is_clean() -> Result<(), String> {
        let found = violations(
            "[dependencies]\nserde = \"1\"\ntokio.workspace = true\ntokio.features = [\"rt\"]\n\n\
             [dev-dependencies]\ntrybuild = { version = \"1\" }\n\n\
             [target.'cfg(windows)'.dependencies]\nwindows-sys = { version = \"0\" }\n",
        )?;
        assert!(found.is_empty(), "{found:?}");
        Ok(())
    }

    /// The workspace's declaration POOL is not an edge, and this is the line
    /// the dotted reversal above stands on.
    ///
    /// `[workspace.dependencies]` states what a member may inherit; nothing in
    /// it is a dependency of the core package, so naming the tooling there is
    /// lawful — testpak inherits from that table and is supposed to. The edge
    /// exists when `[dependencies]` asks for the inheritance, which is the
    /// second half below.
    #[test]
    fn a_workspace_declaration_pool_is_not_an_edge() -> Result<(), String> {
        let pool = violations(
            "[workspace.dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n",
        )?;
        assert!(pool.is_empty(), "{pool:?}");
        let asked = violations(
            "[dependencies]\nthreadpak-macroc.workspace = true\n\n\
             [workspace.dependencies]\nthreadpak-macroc = { path = \"macros/macroc\" }\n",
        )?;
        assert_eq!(asked.len(), 1, "{asked:?}");
        Ok(())
    }

    /// Part-two reversals: the services taking an edge on the surface that is
    /// supposed to call THEM — plainly, for tests only, renamed, by path, and
    /// through workspace inheritance.
    #[test]
    fn any_services_edge_to_the_frontend_is_a_violation() -> Result<(), String> {
        for body in [
            "[dependencies]\nthreadpak-macros = { path = \"../proc\" }\n",
            "[dev-dependencies]\nthreadpak-macros = { path = \"../proc\" }\n",
            "[dev-dependencies]\nshell = { package = \"threadpak-macros\", version = \"0.0.0\" }\n",
            "[dependencies]\nshell = { path = \"../proc\" }\n",
            "[dependencies]\nthreadpak-macros.workspace = true\n",
        ] {
            let found = services_violations(body)?;
            assert_eq!(found.len(), 1, "{body} -> {found:?}");
        }
        Ok(())
    }

    /// The part-two positive control: the services depending only on the
    /// machine are clean.
    #[test]
    fn a_services_manifest_without_a_frontend_edge_is_clean() -> Result<(), String> {
        let found = services_violations("[dependencies]\nthreadpak = { path = \"../..\" }\n")?;
        assert!(found.is_empty(), "{found:?}");
        Ok(())
    }

    /// The real repository holds, on BOTH authorities at once.
    ///
    /// This is the only test here that reaches the resolved half, because the
    /// resolved half is cargo's answer about a real workspace and cannot be
    /// written as a fixture. It is also where the law's refusal-on-unknown is
    /// exercised: a run that could not ask cargo fails here rather than
    /// reporting an absence nobody established.
    #[test]
    fn the_real_workspace_carries_no_prohibited_edge() -> Result<(), String> {
        let found = check_no_core_tooling_edge(repository_snapshot()?);
        assert!(found.is_ok(), "{found:?}");
        Ok(())
    }
}
