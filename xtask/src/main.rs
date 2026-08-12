//! Repository law checks for the `ThreadPak` workspace.
//!
//! `cargo xtask check` runs every day-zero repository law and reports each result;
//! any broken law fails the run. Checks grow one at a time as each written rule
//! gains something to enforce — the repository never carries a rule that nothing
//! checks.

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Directories never visited by repository-wide file checks.
const SKIP_DIRS: [&str; 2] = [".git", "target"];

/// One repository law: a name and the function that checks it.
type Check = (&'static str, fn(&Path) -> Result<(), String>);

fn main() -> Result<(), Box<dyn Error>> {
    let root = repo_root()?;
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("check"));
    match command.as_str() {
        "check" => run_checks(&root),
        other => Err(format!("unknown xtask command: {other}").into()),
    }
}

/// The workspace root: the parent of the xtask crate directory.
fn repo_root() -> Result<PathBuf, Box<dyn Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let parent = manifest_dir
        .parent()
        .ok_or("xtask crate directory has no parent")?;
    Ok(parent.to_path_buf())
}

/// Runs every repository law, printing one PASS or FAIL line per law.
fn run_checks(root: &Path) -> Result<(), Box<dyn Error>> {
    let checks: [Check; 13] = [
        ("agents-claude-parity", check_agents_claude_parity),
        ("lf-and-no-symlinks", check_lf_and_no_symlinks),
        ("no-python", check_no_python),
        ("toolchain-pin-matches-readme", check_toolchain_pin),
        ("workspace-members-match-readme", check_workspace_members),
        ("lint-wall-inherited", check_lint_wall),
        ("no-core-tooling-edge", check_no_core_tooling_edge),
        (
            "underscore-fields-are-phantom",
            check_underscore_fields_are_phantom,
        ),
        ("band-map-matches-lib", check_band_map),
        ("tooling-module-order", check_tooling_module_order),
        ("readme-obligations-join", check_obligations_join),
        ("no-personal-names", check_no_personal_names),
        ("banned-vocabulary", check_banned_vocabulary),
    ];
    let mut failures = Vec::new();
    for (name, check) in checks {
        match check(root) {
            Ok(()) => println!("PASS {name}"),
            Err(reason) => {
                println!("FAIL {name}: {reason}");
                failures.push(name);
            }
        }
    }
    if failures.is_empty() {
        println!("all repository laws hold");
        Ok(())
    } else {
        Err(format!("{} repository law(s) broken", failures.len()).into())
    }
}

/// `AGENTS.md` and `CLAUDE.md` carry the same working law and must stay
/// byte-identical.
fn check_agents_claude_parity(root: &Path) -> Result<(), String> {
    let agents = fs::read(root.join("AGENTS.md")).map_err(|e| format!("AGENTS.md: {e}"))?;
    let claude = fs::read(root.join("CLAUDE.md")).map_err(|e| format!("CLAUDE.md: {e}"))?;
    if agents == claude {
        Ok(())
    } else {
        Err(String::from("AGENTS.md and CLAUDE.md differ"))
    }
}

/// Every file in the repository is LF-only and nothing is a symlink.
fn check_lf_and_no_symlinks(root: &Path) -> Result<(), String> {
    let mut offenders = Vec::new();
    visit_files(root, &mut |path| {
        let metadata =
            fs::symlink_metadata(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if metadata.file_type().is_symlink() {
            offenders.push(format!("symlink: {}", path.display()));
            return Ok(());
        }
        let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        if bytes.contains(&b'\r') {
            offenders.push(format!("CRLF: {}", path.display()));
        }
        Ok(())
    })?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// No Python exists in this repository, ever.
fn check_no_python(root: &Path) -> Result<(), String> {
    let mut offenders = Vec::new();
    visit_files(root, &mut |path| {
        if path.extension().is_some_and(|ext| ext == "py") {
            offenders.push(path.display().to_string());
        }
        Ok(())
    })?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(format!("python files present: {}", offenders.join(", ")))
    }
}

/// The toolchain pinned in `rust-toolchain.toml` matches the toolchain the README
/// yaml block declares.
fn check_toolchain_pin(root: &Path) -> Result<(), String> {
    let toolchain_text = fs::read_to_string(root.join("rust-toolchain.toml"))
        .map_err(|e| format!("rust-toolchain.toml: {e}"))?;
    let pinned = quoted_value(&toolchain_text, "channel")?;
    let yaml = readme_yaml_block(root)?;
    let declared = yaml
        .iter()
        .find_map(|line| line.strip_prefix("toolchain: "))
        .map(|value| value.trim_matches('"').to_string())
        .ok_or_else(|| String::from("README yaml block has no toolchain line"))?;
    if pinned == declared {
        Ok(())
    } else {
        Err(format!(
            "rust-toolchain.toml pins {pinned} but README declares {declared}"
        ))
    }
}

/// The workspace members in `Cargo.toml` match the members the README yaml block
/// declares.
fn check_workspace_members(root: &Path) -> Result<(), String> {
    let manifest =
        fs::read_to_string(root.join("Cargo.toml")).map_err(|e| format!("Cargo.toml: {e}"))?;
    let actual = bracket_list(&manifest, "members")?;
    let yaml = readme_yaml_block(root)?;
    let mut declared = Vec::new();
    let mut in_members = false;
    for line in &yaml {
        if in_members {
            if let Some(item) = line.trim().strip_prefix("- ") {
                declared.push(item.trim().to_string());
            } else {
                in_members = false;
            }
        }
        if line.trim() == "workspace_members:" {
            in_members = true;
        }
    }
    if actual == declared {
        Ok(())
    } else {
        Err(format!(
            "Cargo.toml members {actual:?} but README declares {declared:?}"
        ))
    }
}

/// The root manifest declares the one lint wall and every member inherits it.
fn check_lint_wall(root: &Path) -> Result<(), String> {
    let manifest =
        fs::read_to_string(root.join("Cargo.toml")).map_err(|e| format!("Cargo.toml: {e}"))?;
    if !manifest.contains("[workspace.lints.rust]") {
        return Err(String::from(
            "root Cargo.toml has no [workspace.lints.rust] wall",
        ));
    }
    let members = bracket_list(&manifest, "members")?;
    let mut missing = Vec::new();
    for member in members {
        let path = root.join(&member).join("Cargo.toml");
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        if !text.contains("[lints]\nworkspace = true") {
            missing.push(member);
        }
    }
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("members not inheriting the lint wall: {missing:?}"))
    }
}

/// The metaprogramming packages the core package may never reach.
const TOOLING_PACKAGES: [&str; 2] = ["threadpak-macroc", "threadpak-macros"];

/// The subsystem directory no core dependency path may point into.
const TOOLING_DIRECTORY: &str = "macros";

/// The services package, and the one manifest this law reads for the second
/// absence.
const SERVICES_MANIFEST: &str = "macros/macroc/Cargo.toml";

/// The Rust-facing expansion surface over the services.
const FRONTEND_PACKAGE: &str = "threadpak-macros";

/// The directory that surface lives in, under [`TOOLING_DIRECTORY`].
const FRONTEND_DIRECTORY: &str = "proc";

/// The qualification plane — the machine's judge.
const JUDGE_PACKAGE: &str = "threadpak-testpak";

/// The directory the judge lives in.
const JUDGE_DIRECTORY: &str = "testpak";

/// Every Cargo dependency-edge kind, each of which the law covers.
const DEPENDENCY_TABLE_KINDS: [&str; 3] =
    ["dependencies", "dev-dependencies", "build-dependencies"];

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
fn check_no_core_tooling_edge(root: &Path) -> Result<(), String> {
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

/// Every dependency entry a manifest declares, as
/// `(edge kind, entry key, declared package, declared path)`.
///
/// Ordinary, renamed, dev, build, and target-specific dependencies are all read
/// the same way, across the table spellings Cargo admits: the bare table, the
/// `[KIND.entry]` sub-table, and either under a `target.'…'` prefix.
fn dependency_entries(
    manifest_text: &str,
) -> Vec<(&'static str, String, Option<String>, Option<String>)> {
    let mut entries = Vec::new();
    let mut table: Option<&'static str> = None;
    let mut pending: Option<(&'static str, String, Option<String>, Option<String>)> = None;
    for raw in manifest_text.lines() {
        let line = raw.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(header) = line.strip_prefix('[').and_then(|h| h.strip_suffix(']')) {
            if let Some(entry) = pending.take() {
                entries.push(entry);
            }
            match dependency_table(header.trim()) {
                Some((kind, None)) => table = Some(kind),
                Some((kind, Some(key))) => {
                    table = None;
                    pending = Some((kind, key, None, None));
                }
                None => table = None,
            }
            continue;
        }
        if let Some((_, _, package, path)) = pending.as_mut() {
            if let Some(value) = quoted_assignment(line, "package") {
                *package = Some(value);
            }
            if let Some(value) = quoted_assignment(line, "path") {
                *path = Some(value);
            }
            continue;
        }
        let Some(kind) = table else {
            continue;
        };
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().trim_matches('"');
        if key.is_empty() {
            continue;
        }
        let value = value.trim();
        entries.push((
            kind,
            key.to_string(),
            quoted_assignment(value, "package"),
            quoted_assignment(value, "path"),
        ));
    }
    if let Some(entry) = pending {
        entries.push(entry);
    }
    entries
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

/// The dependency-edge kind a table header declares, plus the single entry the
/// header names when it is the `[dependencies.name]` sub-table form.
///
/// Recognized: the three bare kinds, each `[target.'…'.KIND]` form, and either
/// spelled with a trailing entry name.
fn dependency_table(header: &str) -> Option<(&'static str, Option<String>)> {
    for kind in DEPENDENCY_TABLE_KINDS {
        if header == kind {
            return Some((kind, None));
        }
        if let Some(prefix) = header.strip_suffix(kind)
            && let Some(prefix) = prefix.strip_suffix('.')
            && prefix.starts_with("target.")
        {
            return Some((kind, None));
        }
        if let Some(entry) = header.strip_prefix(&format!("{kind}.")) {
            return Some((kind, Some(entry.trim().trim_matches('"').to_string())));
        }
        if let Some((prefix, entry)) = header.split_once(&format!(".{kind}."))
            && prefix.starts_with("target.")
        {
            return Some((kind, Some(entry.trim().trim_matches('"').to_string())));
        }
    }
    None
}

/// The double-quoted value assigned to `key` anywhere in one line of manifest
/// text, whether the line is a table entry or an inline table body. The key is
/// matched whole, so `package` never matches inside a longer key.
fn quoted_assignment(text: &str, key: &str) -> Option<String> {
    let mut from = 0usize;
    loop {
        let rest = text.get(from..)?;
        let offset = rest.find(key)?;
        let start = from.saturating_add(offset);
        let end = start.saturating_add(key.len());
        let before_is_key = text
            .get(..start)
            .and_then(|head| head.chars().next_back())
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        if !before_is_key
            && let Some(tail) = text.get(end..)
            && let Some(value) = tail.trim_start().strip_prefix('=')
            && let Some(quoted) = value.trim_start().strip_prefix('"')
            && let Some(index) = quoted.find('"')
        {
            return quoted.get(..index).map(str::to_string);
        }
        from = end;
    }
}

/// Every numbered band directory is complete (README.md, mod.rs, types.rs) and
/// `lib.rs` declares every band via its `#[path]` attribute in ascending band
/// order — the band map and the crate never drift apart.
fn check_band_map(root: &Path) -> Result<(), String> {
    let src = root.join("src");
    let mut bands = Vec::new();
    let entries = fs::read_dir(&src).map_err(|e| format!("{}: {e}", src.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", src.display()))?;
        if !entry
            .file_type()
            .map_err(|e| format!("{}: {e}", src.display()))?
            .is_dir()
        {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some((number, _)) = name.split_once('_') else {
            continue;
        };
        if number.len() == 2 && number.chars().all(|c| c.is_ascii_digit()) {
            bands.push(name);
        }
    }
    bands.sort();
    let mut offenders = Vec::new();
    for band in &bands {
        for file in ["README.md", "mod.rs", "types.rs"] {
            if !src.join(band).join(file).is_file() {
                offenders.push(format!("{band} missing {file}"));
            }
        }
    }
    let lib = fs::read_to_string(src.join("lib.rs")).map_err(|e| format!("lib.rs: {e}"))?;
    let mut declared_positions = Vec::new();
    for band in &bands {
        let needle = format!("#[path = \"{band}/mod.rs\"]");
        match lib.find(&needle) {
            Some(position) => declared_positions.push((position, band.clone())),
            None => offenders.push(format!("lib.rs does not declare {band}")),
        }
    }
    let mut sorted = declared_positions.clone();
    sorted.sort();
    if sorted != declared_positions {
        offenders.push(String::from(
            "lib.rs band declarations are out of band order",
        ));
    }
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// The services crate's source directory, whose flat module list carries its
/// dependency order the way numbered directories carry the machine's.
const TOOLING_MODULE_ROOT: [&str; 3] = ["macros", "macroc", "src"];

/// Declaration order IS the dependency order.
///
/// The machine states its bands with numbered directories; the services crate
/// is flat and states the same fact with the only ordering a flat module list
/// has — the order its `mod` declarations appear in `lib.rs`. A module may
/// reference modules declared EARLIER than itself and no others. That single
/// rule outlaws cycles outright, because every cycle contains at least one
/// backward-pointing edge, and it needs no hand-maintained dependency map: the
/// order is read from the declarations and the edges from the sources.
///
/// # The dependency spellings this check recognizes
///
/// The reader is deliberately dumb, and its narrowness is part of the law it
/// states. It recognizes exactly these routes, and nothing else:
///
/// 1. `crate::name` — a plain path, in a `use` line, an inline expression, or a
///    rustdoc link. All three break when the named module moves.
/// 2. `crate::{a::…, b::…}` — a GROUPED use. Every segment head inside the
///    braces is read, so wrapping three imports in one `use` hides none of them.
/// 3. `use crate::name as alias;` — an ALIASED import. The edge is read off the
///    `crate::` path, so renaming the binding hides nothing.
/// 4. `super::name` inside a FLAT module — which is the crate root under another
///    spelling, and therefore the same edge.
/// 5. `crate::Thing` where `Thing` is not a declared module — a CRATE-ROOT
///    RE-EXPORT route. Reaching a sibling's content through the crate root
///    launders the edge: the reference names no owner, so nothing about the
///    declaration order can be read off it. Owner paths only.
///
/// A module is `name.rs` or the directory `name/`, and a directory module's
/// edges are the union of every `.rs` file under it — a submodule reaching
/// forward is its parent reaching forward.
///
/// What it does NOT recognize is stated as plainly: a path built at runtime, a
/// macro that composes `crate::` from fragments, and a re-export chain through a
/// third crate. Those are outside this check, and this check does not pretend
/// otherwise.
///
/// Test-only declarations are excluded: the proof surface (`laws`) is declared
/// `#[cfg(test)] mod laws;` precisely so it can look in every direction without
/// standing in the order it proves.
fn check_tooling_module_order(root: &Path) -> Result<(), String> {
    let mut src = root.to_path_buf();
    for segment in TOOLING_MODULE_ROOT {
        src.push(segment);
    }
    let lib_path = src.join("lib.rs");
    let lib = fs::read_to_string(&lib_path).map_err(|e| format!("{}: {e}", lib_path.display()))?;
    let order = declared_module_order(&lib);
    if order.is_empty() {
        return Err(format!("{} declares no modules", lib_path.display()));
    }
    let mut modules = Vec::new();
    for name in &order {
        let (text, layout) = module_source(&src, name)?;
        modules.push((name.clone(), text, layout));
    }
    let violations = module_order_violations(&order, &modules);
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations.join("; "))
    }
}

/// How a declared module is laid out on disk.
///
/// The layout is not a formatting detail: it decides which crate-root openings
/// can be an edge. In a directory module, a submodule saying `super::` is naming
/// its own parent, which is not a forward reference at all; in a flat module,
/// `super::` and `crate::` name the same place, so both are read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleLayout {
    /// `name.rs` — one file, whose only parent is the crate root.
    Flat,
    /// `name/` — a directory whose submodules can say `super::` about it.
    Directory,
}

/// The source text one declared module contributes, and the layout it is in.
///
/// This is the stage that turns a declared NAME into the text an order is read
/// off. `name.rs` is its own text; `name/` is every `.rs` file under it joined
/// together, because a submodule reaching forward is its parent reaching
/// forward. The check and the law that judges the real tree both read a module
/// through here, so neither can be judging a different tree than the other.
fn module_source(src: &Path, name: &str) -> Result<(String, ModuleLayout), String> {
    let flat = src.join(format!("{name}.rs"));
    if flat.is_file() {
        let text = fs::read_to_string(&flat).map_err(|e| format!("{}: {e}", flat.display()))?;
        return Ok((text, ModuleLayout::Flat));
    }
    let directory = src.join(name);
    if !directory.is_dir() {
        return Err(format!(
            "{name} is declared and is neither {} nor {}/",
            flat.display(),
            directory.display()
        ));
    }
    let mut collected = String::new();
    visit_files(&directory, &mut |path| {
        if path.extension().is_some_and(|extension| extension == "rs") {
            let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
            collected.push_str(&text);
            collected.push('\n');
        }
        Ok(())
    })?;
    Ok((collected, ModuleLayout::Directory))
}

/// The module names one `lib.rs` declares, in declaration order.
///
/// Both `mod name;` and `pub mod name;` count — a private module participates
/// in the order exactly as a public one does. A declaration carrying
/// `#[cfg(test)]` on the line before it does not: the proof surface is outside
/// the order by construction.
fn declared_module_order(lib_text: &str) -> Vec<String> {
    let mut order = Vec::new();
    let mut test_only = false;
    for raw in lib_text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        if line == "#[cfg(test)]" {
            test_only = true;
            continue;
        }
        let declaration = line.strip_prefix("pub ").unwrap_or(line);
        if let Some(rest) = declaration.strip_prefix("mod ")
            && let Some(name) = rest.strip_suffix(';')
            && !name.contains(' ')
        {
            if !test_only {
                order.push(name.to_string());
            }
            test_only = false;
            continue;
        }
        test_only = false;
    }
    order
}

/// Every name one module's text reaches through a crate-root path, in the order
/// the text spells them, duplicates included.
///
/// Both openings are read: `crate::` and `super::`. In a flat module list the
/// two mean the same place, so a module reaching a sibling through `super::` has
/// taken exactly the edge `crate::` would have taken.
///
/// The keyword is matched whole, so a longer identifier ending in `crate` or
/// `super` is never mistaken for the crate root. A grouped use expands: every
/// segment head inside `{ … }` is read, at any nesting, so wrapping three
/// imports in one `use` hides none of them.
///
/// Which openings are read is decided by the module's [`ModuleLayout`], which
/// the caller already holds — the layout was established when the module's text
/// was read, and is carried rather than guessed again here.
fn crate_references(module_text: &str, layout: ModuleLayout) -> Vec<String> {
    let openings: &[&str] = match layout {
        ModuleLayout::Directory => &["crate::"],
        ModuleLayout::Flat => &["crate::", "super::"],
    };
    let mut found = Vec::new();
    for opening in openings.iter().copied() {
        let mut from = 0usize;
        while let Some(offset) = module_text.get(from..).and_then(|rest| rest.find(opening)) {
            let start = from.saturating_add(offset);
            let end = start.saturating_add(opening.len());
            let before_is_word = module_text
                .get(..start)
                .and_then(|head| head.chars().next_back())
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
            if !before_is_word && let Some(tail) = module_text.get(end..) {
                found.extend(referenced_heads(tail));
            }
            from = end;
        }
    }
    found
}

/// The segment heads one crate-root path reaches: the single name of a plain
/// path, or every head inside a grouped use.
fn referenced_heads(tail: &str) -> Vec<String> {
    let trimmed = tail.trim_start();
    if !trimmed.starts_with('{') {
        let name: String = trimmed
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        return if name.is_empty() {
            Vec::new()
        } else {
            vec![name]
        };
    }
    let mut heads = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();
    let mut at_head = false;
    for character in trimmed.chars() {
        match character {
            '{' => {
                depth = depth.saturating_add(1);
                at_head = true;
                current.clear();
            }
            '}' => {
                if !current.is_empty() {
                    heads.push(std::mem::take(&mut current));
                }
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    break;
                }
            }
            ',' => {
                if !current.is_empty() {
                    heads.push(std::mem::take(&mut current));
                }
                at_head = true;
            }
            ':' => {
                if !current.is_empty() {
                    heads.push(std::mem::take(&mut current));
                }
                at_head = false;
            }
            _ if character.is_whitespace() => {}
            _ if at_head && (character.is_ascii_alphanumeric() || character == '_') => {
                current.push(character);
            }
            _ => {
                current.clear();
                at_head = false;
            }
        }
    }
    heads
}

/// Every unlawful edge in one module set, one description per edge.
///
/// Two kinds are refused, and they are different findings:
///
/// 1. **A forward edge** — a module referencing one declared later. That is the
///    shape every cycle contains at least one of.
/// 2. **A crate-root re-export route** — a module referencing a crate-root name
///    that is not a declared module at all. Reaching a sibling's content that
///    way names no owner, so the declaration order says nothing about it, and a
///    check reading owner paths cannot see the edge. Explicit owner paths only.
///
/// A reference to a module declared at the same position (a module naming
/// itself, or a submodule naming its own parent through `super::`) is not an
/// edge.
fn module_order_violations(
    order: &[String],
    modules: &[(String, String, ModuleLayout)],
) -> Vec<String> {
    let position = |name: &str| order.iter().position(|declared| declared == name);
    let mut violations = Vec::new();
    for (name, text, layout) in modules {
        let Some(here) = position(name) else {
            continue;
        };
        let mut reported: Vec<String> = Vec::new();
        for referenced in crate_references(text, *layout) {
            if reported.contains(&referenced) {
                continue;
            }
            match position(&referenced) {
                Some(there) if there > here => {
                    reported.push(referenced.clone());
                    violations.push(format!(
                        "`{name}` (declared {here}) references `{referenced}` (declared \
                         {there}), which is declared later"
                    ));
                }
                Some(_) => {}
                None => {
                    reported.push(referenced.clone());
                    violations.push(format!(
                        "`{name}` reaches `{referenced}` through the crate root, which is no \
                         declared module: owner paths only"
                    ));
                }
            }
        }
    }
    violations
}

/// Every home README the join reads: the root one, and one per numbered band.
fn home_readmes(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut readmes = vec![root.join("README.md")];
    let src = root.join("src");
    let entries = fs::read_dir(&src).map_err(|e| format!("{}: {e}", src.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", src.display()))?;
        let candidate = entry.path().join("README.md");
        if candidate.is_file() {
            readmes.push(candidate);
        }
    }
    Ok(readmes)
}

/// Every green law one README claims, as `(module, law, declaring README)`.
fn claimed_green_laws(readmes: &[PathBuf]) -> Result<Vec<(String, String, PathBuf)>, String> {
    let mut claimed = Vec::new();
    for readme in readmes {
        let text = fs::read_to_string(readme).map_err(|e| format!("{}: {e}", readme.display()))?;
        for line in text.lines() {
            let Some(rest) = line.trim().strip_prefix("green: laws.rs ") else {
                continue;
            };
            let target: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == ':')
                .collect();
            let Some((module, law)) = target.split_once("::") else {
                return Err(format!(
                    "{}: green target `{target}` is not module::law",
                    readme.display()
                ));
            };
            claimed.push((module.to_string(), law.to_string(), readme.clone()));
        }
    }
    Ok(claimed)
}

/// Every law claimed by more than one obligation, one offence per law.
///
/// Two obligations pointing at one law is two claims answered by one proof, and
/// the proof answers at most one of them. Either the pair states one claim, in
/// which case it is one obligation, or it states two, in which case the second
/// one's green half does not exist and the row is saying it does. The join
/// already refuses a law claimed by NOBODY; refusing a law claimed twice closes
/// the same door from the other side.
///
/// Pure over its inputs — `(module, law, declaring README)` triples — so the law
/// is proven against fixture rows rather than against the tree it guards.
fn double_claimed_offences(claimed: &[(String, String, String)]) -> Vec<String> {
    let mut offences = Vec::new();
    let mut reported: Vec<(String, String)> = Vec::new();
    for (module, law, _) in claimed {
        let key = (module.clone(), law.clone());
        if reported.contains(&key) {
            continue;
        }
        let claimants: Vec<&str> = claimed
            .iter()
            .filter(|(m, l, _)| m == module && l == law)
            .map(|(_, _, readme)| readme.as_str())
            .collect();
        if claimants.len() > 1 {
            reported.push(key);
            offences.push(format!(
                "laws.rs {module}::{law} is claimed by {} obligations ({}): one law proves one \
                 claim",
                claimants.len(),
                claimants.join(", ")
            ));
        }
    }
    offences
}

/// The obligations join, in four legs.
///
/// **Green, both ways.** Every README obligation naming a `laws.rs` green law
/// points at a law that exists, and every law in `laws.rs` is claimed by some
/// obligation — the READMEs and the laws never drift apart.
///
/// **Green, exactly once.** No law is claimed by two obligations. A law claimed
/// twice is a proof standing in for a claim it does not make, and it reads as
/// discharged from both rows.
///
/// **Red, and counted out loud.** Every obligation also declares a `red:` row.
/// A row spelled `owed-to-…` is a lawful debt: the reversal is named and not yet
/// written, and saying so is the honest state. Any other row NAMES a reversal
/// that is supposed to exist, and it must resolve to a real testpak test file or
/// compile-fail fixture — a row pointing at a reversal nobody wrote is worse
/// than an owed row, because it reads as discharged.
///
/// **Two denominators, printed apart.** The leg prints core red twins and
/// tooling reversals on their own lines, discharged over owed, on every run. The
/// numbers are meant to be uncomfortable and are meant to be watched: a
/// repository that quietly lost red twins would otherwise keep passing this
/// check while the accounting shrank.
fn check_obligations_join(root: &Path) -> Result<(), String> {
    let readmes = home_readmes(root)?;
    let claimed = claimed_green_laws(&readmes)?;
    let laws_path = root.join("src").join("laws.rs");
    let laws = fs::read_to_string(&laws_path).map_err(|e| format!("laws.rs: {e}"))?;
    let mut existing = Vec::new();
    let mut current_module = String::new();
    let mut previous_was_test = false;
    for line in laws.lines() {
        if let Some(rest) = line.strip_prefix("mod ")
            && let Some(module) = rest.strip_suffix(" {")
        {
            current_module = module.to_string();
        }
        if previous_was_test
            && let Some(rest) = line.trim().strip_prefix("fn ")
            && let Some(law) = rest.split('(').next()
        {
            existing.push((current_module.clone(), law.to_string()));
        }
        previous_was_test = line.trim() == "#[test]";
    }
    let mut offenders = Vec::new();
    for (module, law, readme) in &claimed {
        if !existing.iter().any(|(m, l)| m == module && l == law) {
            offenders.push(format!(
                "{} claims {module}::{law} but laws.rs has no such law",
                readme.display()
            ));
        }
    }
    for (module, law) in &existing {
        if !claimed.iter().any(|(m, l, _)| m == module && l == law) {
            offenders.push(format!(
                "laws.rs {module}::{law} is claimed by no obligation"
            ));
        }
    }
    let attributed: Vec<(String, String, String)> = claimed
        .iter()
        .map(|(module, law, readme)| {
            (
                module.clone(),
                law.clone(),
                relative_slash_path(root, readme),
            )
        })
        .collect();
    offenders.extend(double_claimed_offences(&attributed));
    let mut rows = Vec::new();
    for readme in &readmes {
        let text = fs::read_to_string(readme).map_err(|e| format!("{}: {e}", readme.display()))?;
        for row in red_twin_rows(&text) {
            rows.push((row, relative_slash_path(root, readme)));
        }
    }
    let reversals = testpak_reversals(root)?;
    let ledger = red_twin_ledger(&rows, &reversals);

    let tooling_rows = tooling_rows(root)?;
    let tooling = red_twin_ledger(&tooling_rows, &reversals);

    // TWO denominators, printed apart, always. The populations are challenged by
    // different methods and owned by different homes; one number over both would
    // be a number nobody can act on.
    println!(
        "red twins (core): {} discharged / {} owed",
        ledger.discharged, ledger.owed
    );
    println!(
        "tooling reversals: {} discharged / {} owed",
        tooling.discharged, tooling.owed
    );
    if tooling_rows.is_empty() {
        offenders.push(String::from(
            "no tooling qualification obligation declares a reversal row: the tooling denominator \
             cannot be empty while tooling exists",
        ));
    }
    offenders.extend(ledger.offenders);
    offenders.extend(tooling.offenders);
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// The READMEs that carry tooling qualification obligations.
///
/// A distinct population from the machine's homes: these are claims about the
/// TOOLS — what a service refuses, what a check catches, what a judge is
/// rehearsed against — and their reversals are counted on their own denominator.
const TOOLING_READMES: [&str; 2] = ["macros/macroc/README.md", "testpak/README.md"];

/// Every `tooling-red:` row the tooling READMEs declare, attributed to the file
/// that declared it.
fn tooling_rows(root: &Path) -> Result<Vec<(String, String)>, String> {
    let mut rows = Vec::new();
    for readme in TOOLING_READMES {
        let path = root.join(readme);
        if !path.is_file() {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        for row in tooling_red_rows(&text) {
            rows.push((row, relative_slash_path(root, &path)));
        }
    }
    Ok(rows)
}

/// The value of every `tooling-red:` obligation row in one README, in file
/// order.
///
/// Read exactly like a core `red:` row and counted on its own ledger. An
/// `owed-to-…` row is a lawful debt; any other row NAMES a reversal that must
/// resolve to a real testpak test or compile-fail fixture, and the check refuses
/// it if it does not.
fn tooling_red_rows(readme_text: &str) -> Vec<String> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("tooling-red: "))
        .map(|value| value.trim().to_string())
        .collect()
}

/// The prefix a lawful debt is spelled with: `owed-to-testpak`,
/// `owed-to-xtask-and-testpak`, and any other named creditor.
const OWED_PREFIX: &str = "owed-to";

/// What the red leg counted, and what it refuses.
///
/// A tally over README rows, deliberately not the plane's own
/// `RedTwinLedger`: that one accounts for reversals a qualification run
/// executed, this one for reversals the specification declares. Two
/// denominators over two populations, named apart so neither can stand in for
/// the other.
struct RedTwinTally {
    /// Rows naming a reversal that exists.
    discharged: usize,
    /// Rows declaring a named, unwritten debt.
    owed: usize,
    /// Rows naming a reversal nobody wrote.
    offenders: Vec<String>,
}

/// The value of every `red:` obligation row in one README, in file order.
///
/// The prefix is matched on the TRIMMED line, so a word merely ending in `red`
/// followed by a colon — `unnumbered:`, `authored:`, `Shred:` — is never a row.
fn red_twin_rows(readme_text: &str) -> Vec<String> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("red: "))
        .map(|value| value.trim().to_string())
        .collect()
}

/// Reads one red row and counts it, or names it as an offence.
fn red_twin_ledger(rows: &[(String, String)], reversals: &[String]) -> RedTwinTally {
    let mut ledger = RedTwinTally {
        discharged: 0,
        owed: 0,
        offenders: Vec::new(),
    };
    for (value, readme) in rows {
        if value.starts_with(OWED_PREFIX) {
            ledger.owed = ledger.owed.saturating_add(1);
            continue;
        }
        let named = value.split_whitespace().next().unwrap_or(value);
        if reversals.iter().any(|path| names_reversal(path, named)) {
            ledger.discharged = ledger.discharged.saturating_add(1);
        } else {
            ledger.offenders.push(format!(
                "{readme}: red row names `{named}`, which is no testpak test or fixture"
            ));
        }
    }
    ledger
}

/// Whether one red row's spelling names one existing reversal file. Containment
/// either way: the row may state the repository-relative path or just the file
/// name, and both name the same reversal.
fn names_reversal(path: &str, named: &str) -> bool {
    let file = path.rsplit('/').next().unwrap_or(path);
    path == named || path.contains(named) || (!file.is_empty() && named.contains(file))
}

/// Every reversal testpak carries, as repository-relative slash paths: the test
/// files under `testpak/tests/` and the compile-fail fixtures beneath them.
fn testpak_reversals(root: &Path) -> Result<Vec<String>, String> {
    let tests = root.join(JUDGE_DIRECTORY).join("tests");
    if !tests.is_dir() {
        return Ok(Vec::new());
    }
    let mut found = Vec::new();
    visit_files(&tests, &mut |path| {
        if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(relative_slash_path(root, path));
        }
        Ok(())
    })?;
    Ok(found)
}

/// No personal name appears in any repository file — role terms only. The
/// banned spellings are assembled from bytes so this checker never contains
/// what it forbids.
fn check_no_personal_names(root: &Path) -> Result<(), String> {
    let banned: [Vec<u8>; 2] = [
        vec![0x65, 0x61, 0x73, 0x73, 0x61],
        vec![0x61, 0x79, 0x6f, 0x75, 0x62],
    ];
    let banned: Vec<String> = banned
        .iter()
        .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
        .collect();
    let mut offenders = Vec::new();
    visit_files(root, &mut |path| {
        let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let text = String::from_utf8_lossy(&bytes).to_lowercase();
        for name in &banned {
            if text.contains(name.as_str()) {
                offenders.push(path.display().to_string());
                break;
            }
        }
        Ok(())
    })?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "personal name present in: {}",
            offenders.join(", ")
        ))
    }
}

/// The construction-lifecycle vocabulary the working law bans in prose and in
/// identifiers. This checker spells the words plainly because `xtask` sits
/// outside the tree it scans; `AGENTS.md` and `CLAUDE.md` state the ban itself
/// and are likewise outside it.
const BANNED_VOCABULARY: [&str; 4] = ["factory", "candidate", "promotion", "self-hosting"];

/// Lawful survivals: `(repository-relative path, word, why it stands)`. A term
/// stands only where it is named to FORBID it, to record a kill, or to
/// document a rename — never as live vocabulary.
const BANNED_VOCABULARY_ALLOWLIST: [(&str, &str, &str); 3] = [
    (
        "src/23_evidence/README.md",
        "candidate",
        "the executed-rename record: the dead word is named once to record that \
         `proposal` replaced it",
    ),
    (
        "src/23_evidence/README.md",
        "promotion",
        "the same record: `adoption` replaced it",
    ),
    (
        "src/23_evidence/README.md",
        "factory",
        "the same record: `realization owner` replaced it",
    ),
];

/// Every allowlist entry whose named file no longer spells the word it excuses,
/// one offence per stale entry.
///
/// An allowance is a claim about a file: this word survives HERE, for this
/// reason. When the word leaves the file the claim is no longer about anything,
/// and what is left is a standing hole nobody is watching — the next edit that
/// reintroduces the word to that file passes silently, and the reason line still
/// reads as if somebody had looked. So an entry that matches nothing is refused
/// exactly as a red row naming a reversal nobody wrote is refused: both read as
/// discharged and are not.
///
/// The scan is the same one the ban uses, so an entry is stale by the check's own
/// standard rather than by a second, looser reading of the file.
///
/// Pure over its inputs — `(repository-relative path, that file's text)` pairs
/// for every scanned file — so the law is proven against fixture text.
fn stale_allowlist_offences(scanned: &[(String, String)]) -> Vec<String> {
    let mut offences = Vec::new();
    for (file, word, reason) in BANNED_VOCABULARY_ALLOWLIST {
        let matched = scanned
            .iter()
            .any(|(path, text)| path == file && banned_words_in(text).contains(&word));
        if !matched {
            offences.push(format!(
                "stale allowlist entry: {file} no longer spells `{word}` ({reason})"
            ));
        }
    }
    offences
}

/// An underscore-prefixed field is lawful only when it is a `PhantomData`
/// type-level law. Real data behind an underscore is the suppressor idiom —
/// "ignore this mess" — and the repository refuses it: the only honest `_`
/// is one with nothing to read.
///
/// The scan covers the machine (`src/`), the metaprogramming subsystem
/// (`macros/`), and the qualification plane (`testpak/`): the tools that project
/// the machine's contracts, and the plane that judges them, are held to the
/// machine's own honesty about what a field carries.
fn check_underscore_fields_are_phantom(root: &Path) -> Result<(), String> {
    let mut offenders = Vec::new();
    let mut inspect = |path: &Path| -> Result<(), String> {
        if path.extension().is_none_or(|extension| extension != "rs") {
            return Ok(());
        }
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        for (index, line) in text.lines().enumerate() {
            let trimmed = line.trim_start();
            let field = trimmed
                .strip_prefix("pub(crate) ")
                .or_else(|| trimmed.strip_prefix("pub "))
                .unwrap_or(trimmed);
            if field.starts_with('_')
                && !field.starts_with("_ ")
                && field.contains(": ")
                && !trimmed.starts_with("//")
                && !line.contains("PhantomData")
            {
                offenders.push(format!(
                    "{}:{}: underscore field without PhantomData",
                    path.display(),
                    index.saturating_add(1)
                ));
            }
        }
        Ok(())
    };
    visit_files(&root.join("src"), &mut inspect)?;
    visit_files(&root.join(TOOLING_DIRECTORY), &mut inspect)?;
    visit_files(&root.join(JUDGE_DIRECTORY), &mut inspect)?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// No banned construction-lifecycle word appears in the specification tree.
///
/// Two scans run over every file, and a hit from either is an offence:
///
/// 1. Whole-word, case-insensitive over the whole text: word edges are ASCII
///    alphanumerics, so `snake_case`, `SCREAMING_SNAKE`, kebab-case strings,
///    and plain prose all count, while a longer word merely containing the
///    term does not.
/// 2. Split-identifier: every identifier-like token is cut on `camelCase` and
///    `snake_case` boundaries and each resulting word is compared
///    case-insensitively against the banned list AND its simple plural, so
///    a `CamelCase` type name ending in the plural, a `mixedCase` field, and
///    the plural in plain prose are all caught. A hyphenated banned term
///    matches a consecutive run of split words inside one token, so
///    `SelfHosting` and `self_hosting` are caught too.
///
/// Both scans report the banned ROOT word, so one allowlist entry covers a
/// file for either scan. The scanned tree is the machine (`src/`), the root
/// `README.md`, the metaprogramming subsystem (`macros/`), and the
/// qualification plane (`testpak/`): the tools and the judge speak the
/// machine's vocabulary or they speak none.
fn check_banned_vocabulary(root: &Path) -> Result<(), String> {
    let mut offenders = Vec::new();
    let mut read: Vec<(String, String)> = Vec::new();
    let mut inspect = |path: &Path| -> Result<(), String> {
        let scanned = path
            .extension()
            .is_some_and(|extension| extension == "rs" || extension == "md");
        if !scanned {
            return Ok(());
        }
        let relative = relative_slash_path(root, path);
        let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let text = String::from_utf8_lossy(&bytes).into_owned();
        offenders.extend(banned_vocabulary_offences(&relative, &text));
        read.push((relative, text));
        Ok(())
    };
    visit_files(&root.join("src"), &mut inspect)?;
    visit_files(&root.join(TOOLING_DIRECTORY), &mut inspect)?;
    visit_files(&root.join(JUDGE_DIRECTORY), &mut inspect)?;
    inspect(&root.join("README.md"))?;
    // The allowlist is joined against the same scan: every allowance has to
    // still be excusing something.
    offenders.extend(stale_allowlist_offences(&read));
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "banned vocabulary present: {}",
            offenders.join(", ")
        ))
    }
}

/// Every banned root word one text spells, by either scan, each reported once.
///
/// Both scans are pure over the text, which is what makes the law provable
/// against fixture strings rather than against the tree it guards.
fn banned_words_in(text: &str) -> Vec<&'static str> {
    let lowered = text.to_lowercase();
    let mut hits: Vec<&'static str> = Vec::new();
    for word in BANNED_VOCABULARY {
        if contains_whole_word(&lowered, word) && !hits.contains(&word) {
            hits.push(word);
        }
    }
    for banned in split_scan_hits(text) {
        if !hits.contains(&banned) {
            hits.push(banned);
        }
    }
    hits
}

/// The offences one file commits, its allowlisted survivals removed.
fn banned_vocabulary_offences(relative: &str, text: &str) -> Vec<String> {
    banned_words_in(text)
        .into_iter()
        .filter(|word| {
            !BANNED_VOCABULARY_ALLOWLIST
                .iter()
                .any(|(file, allowed, _)| *file == relative && allowed == word)
        })
        .map(|word| format!("{relative}: {word}"))
        .collect()
}

/// Whether `haystack` contains `needle` bounded by non-alphanumerics on both
/// sides.
fn contains_whole_word(haystack: &str, needle: &str) -> bool {
    let mut from = 0usize;
    loop {
        let Some(rest) = haystack.get(from..) else {
            return false;
        };
        let Some(offset) = rest.find(needle) else {
            return false;
        };
        let start = from.saturating_add(offset);
        let end = start.saturating_add(needle.len());
        let before_is_word = haystack
            .get(..start)
            .and_then(|head| head.chars().next_back())
            .is_some_and(|c| c.is_ascii_alphanumeric());
        let after_is_word = haystack
            .get(end..)
            .and_then(|tail| tail.chars().next())
            .is_some_and(|c| c.is_ascii_alphanumeric());
        if !before_is_word && !after_is_word {
            return true;
        }
        from = end;
    }
}

/// Every banned root word spelled by an identifier-like token in `text` once
/// that token is cut on `camelCase` and `snake_case` boundaries.
fn split_scan_hits(text: &str) -> Vec<&'static str> {
    let mut hits: Vec<&'static str> = Vec::new();
    let tokens = text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'));
    for token in tokens {
        if token.is_empty() {
            continue;
        }
        let words = split_identifier_words(token);
        for word in &words {
            if let Some(banned) = spells_banned_word(word)
                && !hits.contains(&banned)
            {
                hits.push(banned);
            }
        }
        for banned in BANNED_VOCABULARY {
            let parts: Vec<&str> = banned.split('-').collect();
            if parts.len() < 2 || words.len() < parts.len() {
                continue;
            }
            let spelled = words.windows(parts.len()).any(|run| {
                run.iter()
                    .zip(parts.iter())
                    .all(|(word, part)| word == part)
            });
            if spelled && !hits.contains(&banned) {
                hits.push(banned);
            }
        }
    }
    hits
}

/// Cuts one identifier-like token into its lowercase words on `snake_case`
/// separators, `camelCase` boundaries, and acronym-to-word boundaries
/// (`SELFHosting` cuts before `Hosting`).
fn split_identifier_words(token: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut previous_is_lower_or_digit = false;
    let mut chars = token.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '_' {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            previous_is_lower_or_digit = false;
            continue;
        }
        let next_is_lower = chars.peek().is_some_and(char::is_ascii_lowercase);
        if c.is_ascii_uppercase()
            && !current.is_empty()
            && (previous_is_lower_or_digit || next_is_lower)
        {
            words.push(std::mem::take(&mut current));
        }
        current.push(c.to_ascii_lowercase());
        previous_is_lower_or_digit = c.is_ascii_lowercase() || c.is_ascii_digit();
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

/// The banned root word a single lowercase split word spells, counting the
/// simple plural (`candidates`, `promotions`, `factories`). Hyphenated banned
/// terms are matched as word runs, never here.
fn spells_banned_word(word: &str) -> Option<&'static str> {
    BANNED_VOCABULARY.into_iter().find(|banned| {
        if banned.contains('-') {
            return false;
        }
        let plural = banned
            .strip_suffix('y')
            .map_or_else(|| format!("{banned}s"), |stem| format!("{stem}ies"));
        word == *banned || word == plural
    })
}

/// The repository-relative path, slash-separated on every platform.
fn relative_slash_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Visits every file under `dir`, skipping [`SKIP_DIRS`].
///
/// Entries are visited in file-name order rather than in the order the
/// filesystem happens to return them, so every check's traversal — and so every
/// diagnostic it emits — is the same on every machine and every run.
fn visit_files(
    dir: &Path,
    visit: &mut dyn FnMut(&Path) -> Result<(), String>,
) -> Result<(), String> {
    let read = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut entries = Vec::new();
    for entry in read {
        let entry = entry.map_err(|e| format!("{}: {e}", dir.display()))?;
        let path = entry.path();
        let is_dir = entry
            .file_type()
            .map_err(|e| format!("{}: {e}", path.display()))?
            .is_dir();
        entries.push((entry.file_name(), path, is_dir));
    }
    entries.sort_by(|(left, _, _), (right, _, _)| left.cmp(right));
    for (name, path, is_dir) in entries {
        if is_dir {
            if !SKIP_DIRS.contains(&name.to_string_lossy().as_ref()) {
                visit_files(&path, visit)?;
            }
        } else {
            visit(&path)?;
        }
    }
    Ok(())
}

/// Extracts the double-quoted value of a `key = "value"` line.
fn quoted_value(text: &str, key: &str) -> Result<String, String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(key)
            && let Some(rest) = rest.trim_start().strip_prefix('=')
        {
            return Ok(rest.trim().trim_matches('"').to_string());
        }
    }
    Err(format!("no `{key}` line found"))
}

/// Extracts the items of a `key = ["a", "b"]` bracket list.
fn bracket_list(text: &str, key: &str) -> Result<Vec<String>, String> {
    let start = text
        .find(&format!("{key} = ["))
        .ok_or_else(|| format!("no `{key}` list found"))?;
    let rest = text.get(start..).ok_or_else(|| String::from("bad slice"))?;
    let open = rest.find('[').ok_or_else(|| String::from("no bracket"))?;
    let close = rest
        .find(']')
        .ok_or_else(|| format!("unterminated `{key}` list"))?;
    let inner = rest
        .get(open.saturating_add(1)..close)
        .ok_or_else(|| String::from("bad slice"))?;
    Ok(inner
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_string())
        .filter(|item| !item.is_empty())
        .collect())
}

/// The lines inside the README's fenced yaml block.
fn readme_yaml_block(root: &Path) -> Result<Vec<String>, String> {
    let readme =
        fs::read_to_string(root.join("README.md")).map_err(|e| format!("README.md: {e}"))?;
    let mut lines = Vec::new();
    let mut inside = false;
    for line in readme.lines() {
        if inside {
            if line.trim() == "```" {
                return Ok(lines);
            }
            lines.push(line.to_string());
        } else if line.trim() == "```yaml" {
            inside = true;
        }
    }
    Err(String::from("README.md has no fenced yaml block"))
}

/// Planted reversals for the laws whose subject is text rather than the tree.
///
/// A check that cannot fail is not a check. Each reversal here hands
/// [`core_tooling_edge_violations`] a synthetic manifest that reverses the
/// topology one Cargo edge kind at a time and proves the reversal is caught.
/// The manifests are written here rather than on disk: the law is proven
/// against fixture text, never by dirtying the repository it guards.
#[cfg(test)]
mod tests {
    use super::{
        BANNED_VOCABULARY_ALLOWLIST, ModuleLayout, SERVICES_MANIFEST, TOOLING_MODULE_ROOT,
        banned_vocabulary_offences, banned_words_in, check_agents_claude_parity, check_band_map,
        check_lf_and_no_symlinks, check_lint_wall, check_no_python, check_toolchain_pin,
        check_underscore_fields_are_phantom, check_workspace_members, claimed_green_laws,
        core_tooling_edge_violations, declared_module_order, double_claimed_offences, home_readmes,
        module_order_violations, module_source, red_twin_ledger, red_twin_rows,
        relative_slash_path, repo_root, services_frontend_edge_violations,
        stale_allowlist_offences, testpak_reversals, tooling_red_rows,
    };
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicUsize, Ordering};

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

    // -----------------------------------------------------------------------
    // tooling-module-order
    // -----------------------------------------------------------------------

    /// One synthetic module set, as `(name, source text)` pairs. Every synthetic
    /// module is flat: the directory layout is exercised against the real tree,
    /// where the directory exists.
    fn sources(pairs: &[(&str, &str)]) -> Vec<(String, String, ModuleLayout)> {
        pairs
            .iter()
            .map(|(name, text)| ((*name).to_string(), (*text).to_string(), ModuleLayout::Flat))
            .collect()
    }

    /// The declaration order is read out of the file in file order, not sorted,
    /// and the test-only proof surface is excluded from it.
    #[test]
    fn the_declaration_order_is_file_order_without_the_proof_surface() {
        let lib = "//! doc\n\npub mod plane;\n\n/// note\npub mod refusal;\n\nmod helper;\n\n\
                   #[cfg(test)]\nmod laws;\n";
        assert_eq!(
            declared_module_order(lib),
            vec![
                String::from("plane"),
                String::from("refusal"),
                String::from("helper")
            ]
        );
    }

    /// Planted reversal: a module reaching FORWARD to a module declared after
    /// it — the shape every cycle contains at least one of.
    #[test]
    fn a_forward_reference_is_a_violation() {
        let order = vec![String::from("plane"), String::from("planning")];
        let found = module_order_violations(
            &order,
            &sources(&[
                ("plane", "use crate::planning::ProjectionPlan;\n"),
                ("planning", "use crate::plane::ExactIdentity;\n"),
            ]),
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("declared later")));
        assert!(found.iter().any(|v| v.contains("`plane`")));
    }

    /// Planted reversal: the exact cycle this discipline was written to kill —
    /// two modules importing each other. Whichever way the pair is declared,
    /// one of the two edges points forward.
    #[test]
    fn a_two_module_cycle_is_a_violation() {
        let order = vec![
            String::from("planning"),
            String::from("explanation_protocol"),
        ];
        let found = module_order_violations(
            &order,
            &sources(&[
                (
                    "planning",
                    "use crate::explanation_protocol::ExplanationQuestion;\n",
                ),
                (
                    "explanation_protocol",
                    "use crate::planning::ProjectionPlan;\n",
                ),
            ]),
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("`planning`")));
    }

    /// Planted reversal: the forward reference spelled inline rather than in a
    /// `use` line, which no scan of import lines alone would see.
    #[test]
    fn an_inline_forward_path_is_a_violation() {
        let order = vec![String::from("plane"), String::from("diagnostics")];
        let found = module_order_violations(
            &order,
            &sources(&[(
                "plane",
                "fn f() { let _ = crate::diagnostics::MacrocPhase::Capture; }\n",
            )]),
        );
        assert_eq!(found.len(), 1, "{found:?}");
    }

    /// The positive control: a clean set passes. Backward references, repeated
    /// references, a module naming itself, and a longer identifier merely
    /// ENDING in `crate` are all lawful, so the check reports something real
    /// rather than everything.
    #[test]
    fn a_clean_module_set_passes() {
        let order = vec![
            String::from("plane"),
            String::from("refusal"),
            String::from("planning"),
        ];
        let found = module_order_violations(
            &order,
            &sources(&[
                ("plane", "//! no edges at all\n"),
                (
                    "refusal",
                    "use crate::plane::ExactIdentity;\nfn f() { crate::plane::helper(); }\n",
                ),
                (
                    "planning",
                    "use crate::plane::OwnerFactRef;\nuse crate::refusal::PlanSeat;\n\
                     // othercrate::planning is not this crate\n\
                     fn f() { crate::planning::own(); }\n",
                ),
            ]),
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// The real services tree holds: declaration order IS its dependency order.
    #[test]
    fn the_real_services_modules_are_in_dependency_order() -> Result<(), String> {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let mut src = root;
        for segment in TOOLING_MODULE_ROOT {
            src.push(segment);
        }
        let lib = fs::read_to_string(src.join("lib.rs")).unwrap_or_default();
        let order = declared_module_order(&lib);
        assert!(order.len() > 1, "services lib.rs declares {order:?}");
        let mut modules = Vec::new();
        for name in &order {
            let (text, layout) = module_source(&src, name)?;
            assert!(!text.is_empty(), "{name} is unreadable");
            modules.push((name.clone(), text, layout));
        }
        let found = module_order_violations(&order, &modules);
        assert!(found.is_empty(), "{found:?}");
        Ok(())
    }

    // -----------------------------------------------------------------------
    // the red-twin ledger
    // -----------------------------------------------------------------------

    /// One synthetic README's rows, attributed to a fixture file name.
    fn rows(readme_text: &str) -> Vec<(String, String)> {
        red_twin_rows(readme_text)
            .into_iter()
            .map(|value| (value, String::from("FIXTURE.md")))
            .collect()
    }

    /// A row is read off the trimmed line, so an ordinary word ending in `red`
    /// followed by a colon is never mistaken for one.
    #[test]
    fn only_a_red_row_is_a_red_row() {
        let text = "unnumbered: first-class, not a row\n\
                    Shred: the four progress facts, not a row\n\
                    ## The connectives (authored: a heading, not a row)\n\
                    \x20   red: owed-to-testpak\n";
        assert_eq!(red_twin_rows(text), vec![String::from("owed-to-testpak")]);
    }

    /// An owed row is lawful and counts as owed, whoever the named creditor is.
    #[test]
    fn an_owed_row_is_counted_not_refused() {
        let text = "    red: owed-to-testpak\n\
                        red: owed-to-xtask-and-testpak\n\
                        red: owed-to-testpak — cloning a Budget must not compile\n";
        let ledger = red_twin_ledger(&rows(text), &[]);
        assert_eq!(ledger.owed, 3);
        assert_eq!(ledger.discharged, 0);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
    }

    /// Planted reversal: a row naming a reversal nobody wrote. This is the
    /// failure the leg exists for — it reads as discharged and is not.
    #[test]
    fn a_phantom_fixture_name_is_a_violation() {
        let text = "    red: testpak/tests/compile-fail/nobody-ever-wrote-this.rs\n";
        let ledger = red_twin_ledger(
            &rows(text),
            &[String::from(
                "testpak/tests/compile-fail/a-real-fixture-that-exists.rs",
            )],
        );
        assert_eq!(ledger.discharged, 0);
        assert_eq!(ledger.owed, 0);
        assert_eq!(ledger.offenders.len(), 1, "{:?}", ledger.offenders);
        assert!(
            ledger
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("nobody-ever-wrote-this.rs"))
        );
    }

    /// A row naming a real reversal discharges it, whether it states the
    /// repository-relative path or only the file name.
    #[test]
    fn a_named_reversal_that_exists_is_discharged() {
        let reversals = vec![
            String::from("testpak/tests/compile-fail/a-real-fixture.rs"),
            String::from("testpak/tests/planted_defect.rs"),
        ];
        let by_path = red_twin_ledger(
            &rows("    red: testpak/tests/compile-fail/a-real-fixture.rs\n"),
            &reversals,
        );
        assert_eq!(by_path.discharged, 1);
        assert!(by_path.offenders.is_empty(), "{:?}", by_path.offenders);
        let by_name = red_twin_ledger(&rows("    red: planted_defect.rs\n"), &reversals);
        assert_eq!(by_name.discharged, 1);
        assert!(by_name.offenders.is_empty(), "{:?}", by_name.offenders);
    }

    /// The real repository holds: every named red twin resolves to a reversal
    /// that exists, and the denominator is real rather than empty.
    #[test]
    fn the_real_red_ledger_names_only_reversals_that_exist() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let reversals = testpak_reversals(&root).unwrap_or_default();
        assert!(!reversals.is_empty(), "testpak carries no reversal files");
        let mut collected = Vec::new();
        let readmes = home_readmes(&root).unwrap_or_default();
        assert!(!readmes.is_empty(), "no home READMEs found");
        for readme in &readmes {
            let text = fs::read_to_string(readme).unwrap_or_default();
            let name = readme.display().to_string();
            for value in red_twin_rows(&text) {
                collected.push((value, name.clone()));
            }
        }
        let ledger = red_twin_ledger(&collected, &reversals);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
        assert!(
            ledger.owed > 0,
            "no owed red twins found; the ledger cannot be empty here"
        );
    }

    /// A tooling row is read off the trimmed line and counted on its OWN
    /// ledger, never folded into the core one. An `owed-to-…` tooling row is a
    /// lawful debt exactly as a core one is.
    #[test]
    fn a_tooling_row_is_read_and_counted_apart() {
        let text = "  tooling-red: testpak/tests/planted_defect.rs\n\
                    red: owed-to-testpak\n\
                    tooling-red: owed-to-testpak — the structural lane\n";
        let found = tooling_red_rows(text);
        assert_eq!(found.len(), 2, "{found:?}");
        assert_eq!(red_twin_rows(text).len(), 1);
        let attributed: Vec<(String, String)> = found
            .into_iter()
            .map(|row| (row, String::from("FIXTURE.md")))
            .collect();
        let ledger = red_twin_ledger(
            &attributed,
            &[String::from("testpak/tests/planted_defect.rs")],
        );
        assert_eq!(ledger.discharged, 1);
        assert_eq!(ledger.owed, 1);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
    }

    /// Planted reversal: a tooling row naming a reversal nobody wrote. It reads
    /// as discharged and is not.
    #[test]
    fn a_phantom_tooling_reversal_is_a_violation() {
        let ledger = red_twin_ledger(
            &[(
                String::from("testpak/tests/nobody-wrote-this-lane.rs"),
                String::from("FIXTURE.md"),
            )],
            &[String::from("testpak/tests/planted_defect.rs")],
        );
        assert_eq!(ledger.offenders.len(), 1, "{:?}", ledger.offenders);
    }

    /// The real tooling READMEs declare a non-empty denominator, and every row
    /// naming a reversal resolves to one that exists.
    #[test]
    fn the_real_tooling_ledger_names_only_reversals_that_exist() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let reversals = testpak_reversals(&root).unwrap_or_default();
        let mut collected = Vec::new();
        for readme in ["macros/macroc/README.md", "testpak/README.md"] {
            let text = fs::read_to_string(root.join(readme)).unwrap_or_default();
            for row in tooling_red_rows(&text) {
                collected.push((row, String::from(readme)));
            }
        }
        assert!(!collected.is_empty(), "no tooling reversal rows found");
        let ledger = red_twin_ledger(&collected, &reversals);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
        assert!(ledger.owed > 0, "the tooling ledger claims no debt at all");
    }

    // -----------------------------------------------------------------------
    // banned-vocabulary
    // -----------------------------------------------------------------------
    //
    // The scans are pure over text, so every reversal below is a fixture
    // string. Nothing on disk is written, read, or mutated: the law that
    // guards the tree is never proven by dirtying the tree.

    /// Planted reversal: the term smuggled into a `camelCase` identifier, where
    /// no whole-word scan of the text would ever find it.
    #[test]
    fn a_camel_case_smuggle_is_caught() {
        let found = banned_words_in("let selectedFactoryFloor = 1;");
        assert_eq!(found, vec!["factory"], "{found:?}");
    }

    /// Planted reversal: the plural, in prose and in a `CamelCase` type name.
    #[test]
    fn a_plural_is_caught() {
        let prose = banned_words_in("the surviving candidates were counted");
        assert_eq!(prose, vec!["candidate"], "{prose:?}");
        let irregular = banned_words_in("struct RegisteredFactories;");
        assert_eq!(irregular, vec!["factory"], "{irregular:?}");
    }

    /// Planted reversal: a hyphenated term spelled as a consecutive run of
    /// words inside one identifier, in both casings.
    #[test]
    fn a_hyphen_run_is_caught() {
        let camel = banned_words_in("enum SelfHosting { No }");
        assert_eq!(camel, vec!["self-hosting"], "{camel:?}");
        let snake = banned_words_in("const SELF_HOSTING_POSTURE: u8 = 0;");
        assert_eq!(snake, vec!["self-hosting"], "{snake:?}");
    }

    /// Planted reversal: plain `snake_case`, and the kebab-case string a
    /// README row would carry.
    #[test]
    fn a_snake_case_or_kebab_spelling_is_caught() {
        let snake = banned_words_in("fn promotion_route() {}");
        assert_eq!(snake, vec!["promotion"], "{snake:?}");
        let kebab = banned_words_in("id: gate.promotion-ladder");
        assert_eq!(kebab, vec!["promotion"], "{kebab:?}");
    }

    /// The positive control: clean text passes, and a longer word merely
    /// CONTAINING a banned root is not a hit. A checker that flagged
    /// everything would satisfy every reversal above and be worthless.
    #[test]
    fn clean_text_passes() {
        let found = banned_words_in(
            "The proposal was adopted by its realization owner. \
             Manufactured goods and refactoring are ordinary words.",
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// An allowlisted path keeps its one named survival and nothing else: the
    /// allowance is per file AND per word, never a blanket pass.
    #[test]
    fn an_allowlisted_path_keeps_only_its_named_survival() {
        let allowed = banned_vocabulary_offences(
            "src/23_evidence/README.md",
            "`proposal` replaced the dead word candidate",
        );
        assert!(allowed.is_empty(), "{allowed:?}");
        let elsewhere =
            banned_vocabulary_offences("src/00_refusal/README.md", "the dead word candidate");
        assert_eq!(elsewhere.len(), 1, "{elsewhere:?}");
        let unallowed = banned_vocabulary_offences(
            "src/23_evidence/README.md",
            "a self-hosting posture is not allowlisted anywhere",
        );
        assert_eq!(unallowed.len(), 1, "{unallowed:?}");
        assert!(
            unallowed
                .first()
                .is_some_and(|offence| offence.contains("self-hosting"))
        );
    }

    /// Planted reversal: an allowlist entry whose named file no longer spells
    /// the word it excuses. The allowance reads as if somebody had looked at
    /// that file, and the hole it leaves open is unwatched.
    #[test]
    fn a_stale_allowlist_entry_is_a_violation() {
        // The lawful state: every entry's file still spells the word it excuses.
        let live: Vec<(String, String)> = BANNED_VOCABULARY_ALLOWLIST
            .iter()
            .map(|(file, word, _)| {
                (
                    (*file).to_string(),
                    format!("the dead word {word} is recorded here once"),
                )
            })
            .collect();
        assert!(stale_allowlist_offences(&live).is_empty());

        // One entry's word gone from its file, the other two still there.
        let partial: Vec<(String, String)> = live
            .iter()
            .filter(|(_, text)| !text.contains("promotion"))
            .cloned()
            .collect();
        let found = stale_allowlist_offences(&partial);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.first().is_some_and(|offence| {
            offence.contains("stale allowlist entry") && offence.contains("promotion")
        }));

        // The file gone entirely: every entry naming it is stale, and each one
        // is reported on its own rather than folded into one line.
        let gone = stale_allowlist_offences(&[]);
        assert_eq!(gone.len(), BANNED_VOCABULARY_ALLOWLIST.len(), "{gone:?}");
    }

    /// The real allowlist holds: every entry still excuses a word its named
    /// file spells, read through the ban's own scan.
    #[test]
    fn the_real_allowlist_still_excuses_something() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let scanned: Vec<(String, String)> = BANNED_VOCABULARY_ALLOWLIST
            .iter()
            .map(|(file, _, _)| {
                (
                    (*file).to_string(),
                    fs::read_to_string(root.join(file)).unwrap_or_default(),
                )
            })
            .collect();
        let found = stale_allowlist_offences(&scanned);
        assert!(found.is_empty(), "{found:?}");
    }

    // -----------------------------------------------------------------------
    // readme-obligations-join: one law proves one claim
    // -----------------------------------------------------------------------

    /// One synthetic claim row.
    fn claim(module: &str, law: &str, readme: &str) -> (String, String, String) {
        (module.to_string(), law.to_string(), readme.to_string())
    }

    /// Planted reversal: two obligations pointing at one law. The second row's
    /// green half does not exist, and both rows read as discharged.
    #[test]
    fn a_law_claimed_by_two_obligations_is_a_violation() {
        let doubled = [
            claim(
                "bounds",
                "charge_shrinks_or_refuses",
                "src/05_bounds/README.md",
            ),
            claim(
                "bounds",
                "charge_shrinks_or_refuses",
                "src/05_bounds/README.md",
            ),
        ];
        let found = double_claimed_offences(&doubled);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.first().is_some_and(|offence| {
            offence.contains("charge_shrinks_or_refuses") && offence.contains("2 obligations")
        }));

        // Two homes claiming one law is the same offence, and it is reported
        // once rather than once per claimant.
        let across_homes = [
            claim(
                "evidence",
                "coverage_is_unordered",
                "src/23_evidence/README.md",
            ),
            claim("evidence", "coverage_is_unordered", "README.md"),
        ];
        assert_eq!(double_claimed_offences(&across_homes).len(), 1);
    }

    /// The positive control: distinct laws, and one law NAME reused under two
    /// different modules, are both lawful. A check that flagged everything would
    /// satisfy the reversal above and be worthless.
    #[test]
    fn distinct_laws_and_a_reused_law_name_are_lawful() {
        let distinct = [
            claim(
                "bounds",
                "charge_shrinks_or_refuses",
                "src/05_bounds/README.md",
            ),
            claim("bounds", "budget_is_affine", "src/05_bounds/README.md"),
            claim(
                "bytes",
                "decode_maxima_are_sixteen",
                "src/07_bytes/README.md",
            ),
            claim(
                "bytes",
                "width_conventions_are_eight",
                "src/07_bytes/README.md",
            ),
        ];
        assert!(double_claimed_offences(&distinct).is_empty());

        // The join key is module AND law: `bounds::roster_is_closed` and
        // `bytes::roster_is_closed` are two laws in two sections.
        let same_name = [
            claim("bounds", "roster_is_closed", "src/05_bounds/README.md"),
            claim("bytes", "roster_is_closed", "src/07_bytes/README.md"),
        ];
        assert!(double_claimed_offences(&same_name).is_empty());
    }

    /// The real repository holds: every green law it claims is claimed by
    /// exactly one obligation.
    #[test]
    fn the_real_obligations_claim_each_law_once() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let readmes = home_readmes(&root).unwrap_or_default();
        let claimed = claimed_green_laws(&readmes).unwrap_or_default();
        assert!(!claimed.is_empty(), "no green obligations found");
        let attributed: Vec<(String, String, String)> = claimed
            .iter()
            .map(|(module, law, readme)| {
                (
                    module.clone(),
                    law.clone(),
                    relative_slash_path(&root, readme),
                )
            })
            .collect();
        let found = double_claimed_offences(&attributed);
        assert!(found.is_empty(), "{found:?}");
    }

    // -----------------------------------------------------------------------
    // The tree-shaped laws, planted against a scratch root
    // -----------------------------------------------------------------------
    //
    // Eight checks read a directory rather than a text, so a fixture string
    // cannot reach them: what they judge is what a tree contains. They are
    // planted against a scratch root under the platform's temp directory
    // instead. Nothing is written inside the repository — the laws that guard
    // the tree are never proven by dirtying the tree — and each root is removed
    // when its fixture drops.

    /// One scratch root outside the repository, and the files planted in it.
    struct Scratch {
        root: PathBuf,
    }

    impl Scratch {
        /// A fresh scratch root, named for the reversal that built it. The
        /// process id and a run counter keep two fixtures — and two concurrent
        /// runs — from sharing one root.
        fn named(name: &str) -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "threadpak-xtask-{}-{ordinal}-{name}",
                std::process::id()
            ));
            let _cleared = fs::remove_dir_all(&root);
            let _made = fs::create_dir_all(&root);
            Self { root }
        }

        /// Plants one file at a root-relative path, creating its parents.
        fn write(&self, relative: &str, contents: &str) {
            let path = self.root.join(relative);
            if let Some(parent) = path.parent() {
                let _made = fs::create_dir_all(parent);
            }
            let _written = fs::write(&path, contents);
        }

        /// The scratch root, as a check reads it.
        fn root(&self) -> &Path {
            &self.root
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            let _removed = fs::remove_dir_all(&self.root);
        }
    }

    /// A README carrying the fenced yaml block the joins read.
    const FIXTURE_README: &str = "# Fixture\n\n```yaml\nphase: architecture-closure\n\
                                  toolchain: \"1.97.1\"\nworkspace_members:\n  - one\n  - two\n\
                                  ```\n";

    /// Planted reversal: the two working-law files drift apart. One of them
    /// edited alone is exactly how a working law stops being one law.
    #[test]
    fn a_drifted_working_law_pair_is_a_violation() {
        let scratch = Scratch::named("agents-parity");
        scratch.write("AGENTS.md", "the working law\n");
        scratch.write("CLAUDE.md", "the working law\n");
        assert!(check_agents_claude_parity(scratch.root()).is_ok());

        scratch.write("CLAUDE.md", "the working law, edited on one side only\n");
        let found = check_agents_claude_parity(scratch.root());
        assert!(found.is_err_and(|reason| reason.contains("differ")));
    }

    /// Planted reversal: a file carrying CRLF.
    ///
    /// The symlink half of this law is NOT planted. Creating a symlink is a
    /// privileged operation on one of the supported platforms, so a fixture
    /// that planted one would pass or fail on who ran it rather than on the
    /// law. That half stands on the check's own code and on nothing executed
    /// here, and this doc line is where that is admitted rather than implied.
    #[test]
    fn a_crlf_file_is_a_violation() {
        let scratch = Scratch::named("lf-only");
        scratch.write("clean.md", "one line\nanother\n");
        assert!(check_lf_and_no_symlinks(scratch.root()).is_ok());

        scratch.write("drifted.md", "one line\r\nanother\r\n");
        let found = check_lf_and_no_symlinks(scratch.root());
        assert!(found.is_err_and(|reason| reason.contains("CRLF") && reason.contains("drifted")));
    }

    /// Planted reversal: a Python file anywhere in the tree.
    #[test]
    fn a_python_file_is_a_violation() {
        let scratch = Scratch::named("no-python");
        scratch.write("tool.rs", "fn main() {}\n");
        scratch.write("notes/readme.md", "prose\n");
        assert!(check_no_python(scratch.root()).is_ok());

        scratch.write("notes/helper.py", "the file's presence is the offence\n");
        let found = check_no_python(scratch.root());
        assert!(found.is_err_and(|reason| reason.contains("helper.py")));
    }

    /// Planted reversal: the pinned toolchain and the declared one disagree.
    /// The pin is what builds run under and the README is what a reader
    /// believes, so a drift makes the document wrong about the build.
    #[test]
    fn a_toolchain_pin_that_drifts_from_the_readme_is_a_violation() {
        let scratch = Scratch::named("toolchain-pin");
        scratch.write("README.md", FIXTURE_README);
        scratch.write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.97.1\"\n");
        assert!(check_toolchain_pin(scratch.root()).is_ok());

        scratch.write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.98.0\"\n");
        let found = check_toolchain_pin(scratch.root());
        assert!(found.is_err_and(|reason| reason.contains("1.98.0") && reason.contains("1.97.1")));
    }

    /// Planted reversal: a workspace member the README does not declare, in
    /// both directions — a member added to the manifest alone, and one removed
    /// from the manifest while the README still lists it.
    #[test]
    fn a_member_set_that_drifts_from_the_readme_is_a_violation() {
        let scratch = Scratch::named("workspace-members");
        scratch.write("README.md", FIXTURE_README);
        scratch.write("Cargo.toml", "[workspace]\nmembers = [\"one\", \"two\"]\n");
        assert!(check_workspace_members(scratch.root()).is_ok());

        scratch.write(
            "Cargo.toml",
            "[workspace]\nmembers = [\"one\", \"two\", \"three\"]\n",
        );
        let added = check_workspace_members(scratch.root());
        assert!(added.is_err_and(|reason| reason.contains("three")));

        scratch.write("Cargo.toml", "[workspace]\nmembers = [\"one\"]\n");
        let removed = check_workspace_members(scratch.root());
        assert!(removed.is_err_and(|reason| reason.contains("two")));
    }

    /// Planted reversal: a member that does not inherit the lint wall, and
    /// separately a root that declares no wall at all. The two are different
    /// failures — one member walking out, and the wall never existing — and the
    /// check names them apart.
    #[test]
    fn a_member_outside_the_lint_wall_is_a_violation() {
        let scratch = Scratch::named("lint-wall");
        let inheriting = "[package]\nname = \"member\"\n\n[lints]\nworkspace = true\n";
        scratch.write(
            "Cargo.toml",
            "[workspace]\nmembers = [\"one\", \"two\"]\n\n[workspace.lints.rust]\n\
             warnings = { level = \"deny\", priority = -1 }\n",
        );
        scratch.write("one/Cargo.toml", inheriting);
        scratch.write("two/Cargo.toml", inheriting);
        assert!(check_lint_wall(scratch.root()).is_ok());

        scratch.write("two/Cargo.toml", "[package]\nname = \"member\"\n");
        let escaped = check_lint_wall(scratch.root());
        assert!(escaped.is_err_and(|reason| reason.contains("two") && !reason.contains("\"one\"")));

        scratch.write("two/Cargo.toml", inheriting);
        scratch.write("Cargo.toml", "[workspace]\nmembers = [\"one\", \"two\"]\n");
        let wall_free = check_lint_wall(scratch.root());
        assert!(wall_free.is_err_and(|reason| reason.contains("no [workspace.lints.rust] wall")));
    }

    /// Planted reversal: real data behind an underscore — the suppressor idiom
    /// this law exists to refuse — planted in each of the three trees the scan
    /// covers, so no tree is scanned in name only.
    #[test]
    fn an_underscore_field_carrying_data_is_a_violation() {
        let scratch = Scratch::named("underscore-fields");
        let lawful = "struct Demo {\n    _law: PhantomData<*const ()>,\n}\n";
        scratch.write("src/lawful.rs", lawful);
        scratch.write("macros/lawful.rs", lawful);
        scratch.write("testpak/lawful.rs", lawful);
        assert!(check_underscore_fields_are_phantom(scratch.root()).is_ok());

        for tree in ["src", "macros", "testpak"] {
            scratch.write(
                &format!("{tree}/smuggled.rs"),
                "struct Demo {\n    _hidden: u64,\n}\n",
            );
            let found = check_underscore_fields_are_phantom(scratch.root());
            assert!(
                found.is_err_and(|reason| reason.contains("smuggled.rs")
                    && reason.contains("underscore field without PhantomData")),
                "{tree} tree is not scanned"
            );
            let _removed = fs::remove_file(scratch.root().join(tree).join("smuggled.rs"));
        }
    }

    /// Planted reversal: an incomplete band, a band `lib.rs` does not declare,
    /// and declarations out of band order. Three ways the band map and the
    /// crate drift apart, and the third is the one no file listing would catch.
    #[test]
    fn a_band_map_that_drifts_from_lib_is_a_violation() {
        let scratch = Scratch::named("band-map");
        let ordered = "#[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n\
                       #[path = \"01_logic/mod.rs\"]\npub mod logic;\n";
        for band in ["00_refusal", "01_logic"] {
            for file in ["README.md", "mod.rs", "types.rs"] {
                scratch.write(&format!("src/{band}/{file}"), "the home's content\n");
            }
        }
        scratch.write("src/lib.rs", ordered);
        assert!(check_band_map(scratch.root()).is_ok());

        let _removed = fs::remove_file(scratch.root().join("src/01_logic/types.rs"));
        let incomplete = check_band_map(scratch.root());
        assert!(incomplete.is_err_and(|reason| reason.contains("01_logic missing types.rs")));
        scratch.write("src/01_logic/types.rs", "the home's content\n");

        scratch.write(
            "src/lib.rs",
            "#[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n",
        );
        let undeclared = check_band_map(scratch.root());
        assert!(undeclared.is_err_and(|reason| reason.contains("does not declare 01_logic")));

        scratch.write(
            "src/lib.rs",
            "#[path = \"01_logic/mod.rs\"]\npub mod logic;\n\
             #[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n",
        );
        let reordered = check_band_map(scratch.root());
        assert!(reordered.is_err_and(|reason| reason.contains("out of band order")));
    }
}
