//! The topology law: which package may reach which.
//!
//! The edges run one way and inward. The machine depends on nothing that
//! projects its contracts and nothing that judges it; a compiler service depends
//! on no frontend of its own. Both halves are read off declared manifests rather
//! than off a resolved graph, because a manifest is where the edge is written and
//! where a reviewer will look for it.

use std::fs;
use std::path::Path;

use crate::repository::manifest::dependency_entries;
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
pub(crate) fn check_no_core_tooling_edge(root: &Path) -> Result<(), String> {
    let mut reported = Vec::new();
    let manifest =
        fs::read_to_string(root.join("Cargo.toml")).map_err(|e| format!("Cargo.toml: {e}"))?;
    for violation in core_tooling_edge_violations(&manifest) {
        reported.push(format!(
            "core package reaches tooling or its judge: {violation}"
        ));
    }
    let services = fs::read_to_string(root.join(SERVICES_MANIFEST))
        .map_err(|e| format!("{SERVICES_MANIFEST}: {e}"))?;
    for violation in services_frontend_edge_violations(&services) {
        reported.push(format!(
            "services reach their expansion surface: {violation}"
        ));
    }
    if reported.is_empty() {
        Ok(())
    } else {
        Err(reported.join("; "))
    }
}

/// Every tooling edge the root manifest declares, one description per edge.
///
/// An entry's PACKAGE IDENTITY is its `package = "…"` key when it carries one
/// and its own key otherwise, and an entry is a violation when that identity
/// names a tooling package or the judge, or when its `path` points into the
/// tooling subsystem directory or the judge's directory. Renaming therefore
/// hides nothing.
fn core_tooling_edge_violations(manifest_text: &str) -> Vec<String> {
    dependency_entries(manifest_text)
        .into_iter()
        .filter_map(|(kind, key, package, path)| {
            judge_dependency(kind, &key, package.as_deref(), path.as_deref())
        })
        .collect()
}

/// Every frontend edge the services manifest declares, one description per
/// edge.
///
/// Read exactly like the core law: package identity first, so a renamed entry
/// betrays itself, and then the declared path, so an entry named anything at
/// all that reaches into `macros/proc/` is caught by where it points.
fn services_frontend_edge_violations(manifest_text: &str) -> Vec<String> {
    dependency_entries(manifest_text)
        .into_iter()
        .filter_map(|(kind, key, package, path)| {
            judge_frontend_dependency(kind, &key, package.as_deref(), path.as_deref())
        })
        .collect()
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

    /// The positive control: a manifest with ordinary edges and none to the
    /// tooling or the judge is clean, so the law reports something real rather
    /// than everything.
    #[test]
    fn a_manifest_without_tooling_edges_is_clean() {
        let found = violations(
            "[dependencies]\nserde = \"1\"\n\n[dev-dependencies]\ntrybuild = { version = \"1\" }\n\n\
             [target.'cfg(windows)'.dependencies]\nwindows-sys = { version = \"0\" }\n",
        );
        assert!(found.is_empty(), "{found:?}");
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
