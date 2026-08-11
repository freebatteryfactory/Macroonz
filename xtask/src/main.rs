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
    let checks: [Check; 12] = [
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

/// The obligations join: every README obligation naming a `laws.rs` green law
/// points at a law that exists, and every law in `laws.rs` is claimed by some
/// obligation — the READMEs and the laws never drift apart. (The third leg —
/// the owed red twin — joins when testpak lands.)
fn check_obligations_join(root: &Path) -> Result<(), String> {
    let mut claimed = Vec::new();
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
    for readme in &readmes {
        let text = fs::read_to_string(readme).map_err(|e| format!("{}: {e}", readme.display()))?;
        for line in text.lines() {
            let Some(rest) = line.trim().strip_prefix("green: laws.rs ") else {
                continue;
            };
            let target: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == ':')
                .collect();
            match target.split_once("::") {
                Some((module, law)) => {
                    claimed.push((module.to_string(), law.to_string(), readme.clone()));
                }
                None => {
                    return Err(format!(
                        "{}: green target `{target}` is not module::law",
                        readme.display()
                    ));
                }
            }
        }
    }
    let laws_path = src.join("laws.rs");
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
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
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
        let lowered = text.to_lowercase();
        let mut hits: Vec<&'static str> = Vec::new();
        for word in BANNED_VOCABULARY {
            if contains_whole_word(&lowered, word) && !hits.contains(&word) {
                hits.push(word);
            }
        }
        for banned in split_scan_hits(&text) {
            if !hits.contains(&banned) {
                hits.push(banned);
            }
        }
        for word in hits {
            let allowed = BANNED_VOCABULARY_ALLOWLIST
                .iter()
                .any(|(file, allowed, _)| *file == relative && *allowed == word);
            if !allowed {
                offenders.push(format!("{relative}: {word}"));
            }
        }
        Ok(())
    };
    visit_files(&root.join("src"), &mut inspect)?;
    visit_files(&root.join(TOOLING_DIRECTORY), &mut inspect)?;
    visit_files(&root.join(JUDGE_DIRECTORY), &mut inspect)?;
    inspect(&root.join("README.md"))?;
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "banned vocabulary present: {}",
            offenders.join(", ")
        ))
    }
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
        SERVICES_MANIFEST, core_tooling_edge_violations, repo_root,
        services_frontend_edge_violations,
    };

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
        let root = repo_root().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let manifest = std::fs::read_to_string(root.join("Cargo.toml")).unwrap_or_default();
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
        let root = repo_root().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let manifest = std::fs::read_to_string(root.join(SERVICES_MANIFEST)).unwrap_or_default();
        assert!(!manifest.is_empty(), "services Cargo.toml is unreadable");
        let found = services_frontend_edge_violations(&manifest);
        assert!(found.is_empty(), "{found:?}");
    }
}
