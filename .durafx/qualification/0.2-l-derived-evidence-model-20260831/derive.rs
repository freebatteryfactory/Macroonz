#![forbid(unsafe_code)]
#![deny(warnings)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const GRAMMAR: &str = "macroonz-evidence-model/1";
const RECEIPTS: &[&str] = &[
    ".durafx/package/wave-g-release-0.1.0-20260828/receipt.md",
    ".durafx/qualification/0.2-h-api-compatibility-20260831/receipt.md",
    ".durafx/qualification/0.2-j-facade-posture-economics-20260831/receipt.md",
    ".durafx/qualification/0.2-j-guarded-economics-20260831/receipt.md",
    ".durafx/qualification/0.2-j-shape-projection-economics-20260831/receipt.md",
    ".durafx/qualification/0.2-k-cohesive-subject-20260831/receipt.md",
    ".durafx/qualification/0.2-k-concurrency-runtime-20260831/receipt.md",
    ".durafx/qualification/0.2-k-docs-blind-storefront-20260831/receipt.md",
    ".durafx/qualification/0.2-k-guarded-package-journey-20260831/receipt.md",
    ".durafx/qualification/0.2-k-parser-codec-20260831/receipt.md",
    ".durafx/qualification/0.2-m-factoring-ruling-20260831/receipt.md",
    ".durafx/qualification/wave-f-solo-governance-20260828/receipt.md",
];
const OWNERS: &[&str] = &[
    "AGENTS.md",
    "Cargo.toml",
    "harness/Cargo.toml",
    "macros/compiler/Cargo.toml",
    "macros/proc/Cargo.toml",
    "src/lib.rs",
    "harness/src/lib.rs",
    "macros/compiler/src/lib.rs",
    "macros/proc/src/lib.rs",
];

#[derive(Clone, Debug)]
struct Source {
    id: String,
    kind: String,
    path: String,
    blob: String,
    commit: String,
}

#[derive(Clone, Debug)]
struct Section {
    id: String,
    source: String,
    first_line: usize,
    last_line: usize,
    heading: String,
    roles: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct EdgeKey {
    from: String,
    to: String,
    plane: String,
}

#[derive(Clone, Debug)]
struct Edge {
    id: String,
    from: String,
    to: String,
    plane: String,
    standing: EdgeStanding,
}

#[derive(Clone, Copy, Debug)]
enum EdgeStanding {
    Matched,
    DeclaredOnly,
    ObservedOnly,
}

#[derive(Clone, Debug)]
struct Capability {
    id: String,
    owner: String,
    path: String,
    line: usize,
    name: String,
    form: String,
}

#[derive(Clone, Debug)]
struct Model {
    sources: Vec<Source>,
    sections: Vec<Section>,
    edges: Vec<Edge>,
    capabilities: Vec<Capability>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Wave L derivation refused: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let repo = PathBuf::from("../../..");
    let receipt_paths: Vec<_> = RECEIPTS.iter().map(|path| (*path).to_owned()).collect();
    let owner_paths: Vec<_> = OWNERS.iter().map(|path| (*path).to_owned()).collect();
    let model = derive_model(&repo, &receipt_paths, &owner_paths)?;
    validate_model(&model)?;

    let canonical = canonical(&model);
    let model_id = git_hash_bytes(&repo, canonical.as_bytes())?;
    let assurance = assurance(&model, &model_id, "Macroonz assurance view");
    let navigation = navigation(&model, &model_id, "Macroonz evidence navigation graph");
    let validation = validate_renderings(&repo, &model, &model_id, &assurance, &navigation)?;

    let output = Path::new("out");
    fs::create_dir_all(output).map_err(|error| format!("{}: {error}", output.display()))?;
    write_file(&output.join("model.tsv"), &canonical)?;
    write_file(&output.join("assurance.md"), &assurance)?;
    write_file(&output.join("navigation.md"), &navigation)?;
    write_file(&output.join("validation.tsv"), &validation)?;

    println!("model identity: {model_id}");
    println!("sources: {}", model.sources.len());
    println!("sections: {}", model.sections.len());
    println!("architecture edges: {}", model.edges.len());
    println!("public capabilities: {}", model.capabilities.len());
    println!("nodes: {}", node_roster(&model).len());
    Ok(())
}

fn validate_neutral_path(path: &str) -> Result<(), String> {
    if path.contains('\\') || path.starts_with('/') || path.contains("..") || path.contains(':') {
        Err(format!("non-neutral input path {path:?}"))
    } else {
        Ok(())
    }
}

fn derive_model(repo: &Path, receipts: &[String], owners: &[String]) -> Result<Model, String> {
    let mut sources = Vec::new();
    for path in receipts {
        validate_neutral_path(path)?;
        if !path.starts_with(".durafx/") || !path.ends_with("/receipt.md") {
            return Err(format!("receipt input is outside accepted custody: {path}"));
        }
        sources.push(source(repo, path, "receipt", sources.len())?);
    }
    for path in owners {
        validate_neutral_path(path)?;
        sources.push(source(repo, path, "owner", sources.len())?);
    }

    let source_by_path: BTreeMap<_, _> = sources
        .iter()
        .map(|source| (source.path.as_str(), source.id.as_str()))
        .collect();
    let mut sections = Vec::new();
    for path in receipts {
        let source_id = source_by_path
            .get(path.as_str())
            .ok_or_else(|| format!("receipt source missing for {path}"))?;
        sections.extend(receipt_sections(repo, path, source_id)?);
    }
    let edges = architecture_edges(repo, owners)?;
    let capabilities = capabilities(repo, owners)?;
    Ok(Model {
        sources,
        sections,
        edges,
        capabilities,
    })
}

fn source(repo: &Path, path: &str, kind: &str, ordinal: usize) -> Result<Source, String> {
    git_success(repo, &["ls-files", "--error-unmatch", "--", path])?;
    let bytes = fs::read(repo.join(path)).map_err(|error| format!("{path}: {error}"))?;
    let blob = git_text(repo, &["hash-object", "--", path])?;
    let commit = git_text(repo, &["log", "-1", "--format=%H", "--", path])?;
    if commit.len() != 40 {
        return Err(format!("{path} has no canonical owning commit"));
    }
    let committed = git_bytes(repo, &["show", &format!("{commit}:{path}")])?;
    if committed != bytes {
        return Err(format!(
            "{path} differs from its latest owning commit {commit}"
        ));
    }
    Ok(Source {
        id: format!("source-{ordinal:03}"),
        kind: kind.to_owned(),
        path: path.to_owned(),
        blob,
        commit,
    })
}

fn receipt_sections(repo: &Path, path: &str, source_id: &str) -> Result<Vec<Section>, String> {
    let text = fs::read_to_string(repo.join(path)).map_err(|error| format!("{path}: {error}"))?;
    let lines: Vec<_> = text.lines().collect();
    let starts: Vec<_> = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| line.strip_prefix("## ").map(|heading| (index, heading)))
        .collect();
    if starts.is_empty() {
        return Err(format!("receipt {path} has no section headings"));
    }
    let mut result = Vec::new();
    for (ordinal, (start, heading)) in starts.iter().enumerate() {
        let end = starts
            .get(ordinal + 1)
            .map_or(lines.len(), |(next, _)| *next);
        let body = lines[*start..end].join("\n");
        result.push(Section {
            id: format!("section-{source_id}-{ordinal:03}"),
            source: source_id.to_owned(),
            first_line: start + 1,
            last_line: end,
            heading: heading.trim().to_owned(),
            roles: classify_section(heading, &body),
        });
    }
    Ok(result)
}

fn classify_section(heading: &str, body: &str) -> BTreeSet<String> {
    let text = format!("{heading}\n{body}").to_ascii_lowercase();
    let mut roles = BTreeSet::from(["evidence".to_owned()]);
    for (role, needles) in role_needles() {
        add_role(&mut roles, &text, role, needles);
    }
    roles
}

fn role_needles() -> [(&'static str, &'static [&'static str]); 7] {
    [
        (
            "claim",
            &[
                "accept",
                "standing",
                "behavior",
                "journey",
                "verification",
                "qualification",
                "publication",
                "coverage",
                "replay",
            ],
        ),
        (
            "architecture",
            &[
                "architecture",
                "package",
                "manifest",
                "dependency",
                "public source",
                "facade",
                "owner",
            ],
        ),
        (
            "sensitivity",
            &[
                "mutation",
                "sensitivity",
                "counterexample",
                "hostile",
                "planted",
                "reversal",
                "damage",
            ],
        ),
        (
            "cost",
            &[
                "cost",
                "economics",
                "work curve",
                "timing",
                "performance",
                "bytes",
                "cleanup",
            ],
        ),
        (
            "foundation",
            &[
                "authority",
                "custody",
                "toolchain",
                "host",
                "source",
                "package",
                "registry",
                "git",
            ],
        ),
        (
            "assumption",
            &[
                "shared",
                "same exact",
                "reus",
                "join",
                "independent",
                "baseline",
            ],
        ),
        (
            "ceiling",
            &[
                "ceiling",
                "limit",
                "does not",
                "cannot",
                "unavailable",
                "not claim",
                "excluded",
                "remain later",
                "refus",
            ],
        ),
    ]
}

fn add_role(roles: &mut BTreeSet<String>, text: &str, role: &str, needles: &[&str]) {
    if needles.iter().any(|needle| text.contains(needle)) {
        roles.insert(role.to_owned());
    }
}

fn architecture_edges(repo: &Path, owners: &[String]) -> Result<Vec<Edge>, String> {
    let agents = fs::read_to_string(repo.join("AGENTS.md")).map_err(|error| error.to_string())?;
    let declared = declared_edges(&agents);
    let observed = observed_edges(repo, owners)?;
    let all: BTreeSet<_> = declared.union(&observed).cloned().collect();
    Ok(all
        .into_iter()
        .map(|key| Edge {
            id: format!("edge-{}-{}-{}", slug(&key.from), slug(&key.to), key.plane),
            from: key.from.clone(),
            to: key.to.clone(),
            plane: key.plane.clone(),
            standing: match (declared.contains(&key), observed.contains(&key)) {
                (true, true) => EdgeStanding::Matched,
                (true, false) => EdgeStanding::DeclaredOnly,
                (false, true) => EdgeStanding::ObservedOnly,
                (false, false) => unreachable!("the edge came from the declared-observed union"),
            },
        })
        .collect())
}

fn declared_edges(text: &str) -> BTreeSet<EdgeKey> {
    let mut edges = BTreeSet::new();
    for line in text.lines() {
        for (arrow, plane) in [("──▶", "product"), ("┈┈▶", "test")] {
            if line.contains(arrow) {
                let packages: Vec<_> = line.split(arrow).filter_map(package_token).collect();
                insert_edge_chain(&mut edges, &packages, plane);
            }
        }
    }
    edges
}

fn insert_edge_chain(edges: &mut BTreeSet<EdgeKey>, packages: &[String], plane: &str) {
    for pair in packages.windows(2) {
        let [from, to] = pair else { continue };
        edges.insert(EdgeKey {
            from: from.clone(),
            to: to.clone(),
            plane: plane.to_owned(),
        });
    }
}

fn package_token(segment: &str) -> Option<String> {
    segment
        .split_whitespace()
        .next()
        .map(|value| value.trim_matches('`').to_owned())
        .filter(|value| value.starts_with("macroonz"))
}

fn observed_edges(repo: &Path, owners: &[String]) -> Result<BTreeSet<EdgeKey>, String> {
    let manifests: Vec<_> = owners
        .iter()
        .filter(|path| path.ends_with("Cargo.toml"))
        .collect();
    let mut result = BTreeSet::new();
    for manifest in manifests {
        let text = fs::read_to_string(repo.join(manifest))
            .map_err(|error| format!("{manifest}: {error}"))?;
        let owner = package_name(&text).ok_or_else(|| format!("{manifest} lacks package name"))?;
        let mut section = String::new();
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                trimmed
                    .trim_matches(&['[', ']'][..])
                    .clone_into(&mut section);
                continue;
            }
            let plane = if section == "dev-dependencies" {
                Some("test")
            } else if section == "dependencies"
                || (section.ends_with(".dependencies") && !section.starts_with("workspace."))
            {
                Some("product")
            } else {
                None
            };
            let Some(plane) = plane else { continue };
            let Some((left, right)) = trimmed.split_once('=') else {
                continue;
            };
            let dependency =
                package_override(right).unwrap_or_else(|| left.trim().replace('_', "-"));
            if dependency.starts_with("macroonz") && dependency != owner {
                result.insert(EdgeKey {
                    from: owner.clone(),
                    to: dependency,
                    plane: plane.to_owned(),
                });
            }
        }
    }
    Ok(result)
}

fn package_name(text: &str) -> Option<String> {
    let mut in_package = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_package = trimmed == "[package]";
        } else if in_package && trimmed.starts_with("name = ") {
            return quoted_value(trimmed);
        }
    }
    None
}

fn package_override(text: &str) -> Option<String> {
    let marker = "package = \"";
    let start = text.find(marker)? + marker.len();
    let end = text.get(start..)?.find('"')? + start;
    text.get(start..end).map(str::to_owned)
}

fn quoted_value(text: &str) -> Option<String> {
    let start = text.find('"')? + 1;
    let end = text.get(start..)?.find('"')? + start;
    text.get(start..end).map(str::to_owned)
}

fn capabilities(repo: &Path, owners: &[String]) -> Result<Vec<Capability>, String> {
    let mut result = Vec::new();
    for path in owners.iter().filter(|path| path.ends_with("src/lib.rs")) {
        let owner =
            owner_for_lib(path).ok_or_else(|| format!("unknown library owner for {path}"))?;
        let text =
            fs::read_to_string(repo.join(path)).map_err(|error| format!("{path}: {error}"))?;
        for (index, line) in text.lines().enumerate() {
            let found = capability_line(line.trim());
            if let Some((name, form)) = found {
                result.push(Capability {
                    id: format!("capability-{}-{}", slug(owner), slug(&name)),
                    owner: owner.to_owned(),
                    path: path.clone(),
                    line: index + 1,
                    name,
                    form: form.to_owned(),
                });
            }
        }
    }
    result.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(result)
}

fn capability_line(line: &str) -> Option<(String, &'static str)> {
    if let Some(rest) = line.strip_prefix("pub mod ") {
        return Some((rest.trim_end_matches(';').to_owned(), "module"));
    }
    if let Some(rest) = line.strip_prefix("pub fn ") {
        return rest
            .split_once('(')
            .map(|(name, _)| (name.to_owned(), "function"));
    }
    let rest = line.strip_prefix("pub use ")?;
    if rest.contains('{') {
        return None;
    }
    rest.trim_end_matches(';')
        .rsplit_once(" as ")
        .map(|(_, alias)| (alias.to_owned(), "facade"))
}

fn owner_for_lib(path: &str) -> Option<&'static str> {
    match path {
        "src/lib.rs" => Some("macroonz"),
        "harness/src/lib.rs" => Some("macroonz-harness"),
        "macros/compiler/src/lib.rs" => Some("macroonz-compiler"),
        "macros/proc/src/lib.rs" => Some("macroonz-macros"),
        _ => None,
    }
}

fn slug(value: &str) -> String {
    let mut output = String::new();
    let mut dash = false;
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() {
            output.push(char::from(byte.to_ascii_lowercase()));
            dash = false;
        } else if !dash && !output.is_empty() {
            output.push('-');
            dash = true;
        }
    }
    output.trim_end_matches('-').to_owned()
}

fn validate_model(model: &Model) -> Result<(), String> {
    let mut ids = BTreeSet::new();
    for id in node_roster(model) {
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate model node identity {id}"));
        }
    }
    let source_ids: BTreeSet<_> = model
        .sources
        .iter()
        .map(|source| source.id.as_str())
        .collect();
    for section in &model.sections {
        if !source_ids.contains(section.source.as_str()) || section.roles.is_empty() {
            return Err(format!("section {} lacks source or roles", section.id));
        }
    }
    for required in [
        "claim",
        "architecture",
        "sensitivity",
        "cost",
        "foundation",
        "assumption",
        "ceiling",
    ] {
        if !model
            .sections
            .iter()
            .any(|section| section.roles.contains(required))
        {
            return Err(format!("model has no {required} section reference"));
        }
    }
    if model.edges.is_empty() || model.capabilities.is_empty() {
        return Err("model lacks architecture edges or public capability doors".to_owned());
    }
    Ok(())
}

fn canonical(model: &Model) -> String {
    let mut output = format!("grammar\t{GRAMMAR}\n");
    for source in &model.sources {
        writeln!(
            output,
            "source\t{}\t{}\t{}\t{}\t{}",
            source.id,
            source.kind,
            clean(&source.path),
            source.blob,
            source.commit
        )
        .expect("String writing cannot fail");
    }
    for section in &model.sections {
        writeln!(
            output,
            "section\t{}\t{}\t{}\t{}\t{}\t{}",
            section.id,
            section.source,
            section.first_line,
            section.last_line,
            clean(&section.heading),
            section.roles.iter().cloned().collect::<Vec<_>>().join(",")
        )
        .expect("String writing cannot fail");
    }
    for edge in &model.edges {
        writeln!(
            output,
            "edge\t{}\t{}\t{}\t{}\t{}\t{}",
            edge.id,
            edge.from,
            edge.to,
            edge.plane,
            edge.standing.declared(),
            edge.standing.observed()
        )
        .expect("String writing cannot fail");
    }
    for capability in &model.capabilities {
        writeln!(
            output,
            "capability\t{}\t{}\t{}\t{}\t{}\t{}",
            capability.id,
            capability.owner,
            capability.path,
            capability.line,
            capability.name,
            capability.form
        )
        .expect("String writing cannot fail");
    }
    output
}

fn clean(value: &str) -> String {
    value.replace(['\t', '\r', '\n'], " ")
}

fn model_header(output: &mut String, title: &str, model_id: &str) {
    writeln!(output, "# {title}\n").expect("String writing cannot fail");
    writeln!(output, "- Model grammar: `{GRAMMAR}`").expect("String writing cannot fail");
    writeln!(output, "- Model identity: `{model_id}`\n").expect("String writing cannot fail");
}

fn assurance(model: &Model, model_id: &str, title: &str) -> String {
    let mut output = String::new();
    model_header(&mut output, title, model_id);
    output.push_str("This view navigates authoritative sources by mechanically derived role; each receipt remains the semantic owner.\n\n");
    output.push_str("## Claim and evidence references\n\n");
    let mut sections: Vec<_> = model.sections.iter().collect();
    sections.sort_by_key(|section| {
        (
            !section.roles.contains("claim"),
            section.source.as_str(),
            section.first_line,
        )
    });
    for section in sections {
        let source = model
            .sources
            .iter()
            .find(|source| source.id == section.source)
            .expect("validated source relation");
        writeln!(
            output,
            "- `{}` -> `{}:{}` ({}) [{}]",
            section.id,
            source.path,
            section.first_line,
            section.heading,
            section.roles.iter().cloned().collect::<Vec<_>>().join(", ")
        )
        .expect("String writing cannot fail");
    }
    output.push('\n');
    output.push_str("## Architecture liveness\n\n| Edge | Plane | Declared | Observed | Standing |\n| --- | --- | --- | --- | --- |\n");
    for edge in &model.edges {
        writeln!(
            output,
            "| `{}` -> `{}` | {} | {} | {} | {} |",
            edge.from,
            edge.to,
            edge.plane,
            edge.standing.declared(),
            edge.standing.observed(),
            edge.standing.label()
        )
        .expect("String writing cannot fail");
    }
    output.push_str("\n## Public capability doors\n\n");
    for capability in &model.capabilities {
        writeln!(
            output,
            "- `{}` -> `{}` at `{}:{}` ({})",
            capability.owner, capability.name, capability.path, capability.line, capability.form
        )
        .expect("String writing cannot fail");
    }
    append_roster(&mut output, model);
    output
}

fn navigation(model: &Model, model_id: &str, title: &str) -> String {
    let mut output = String::new();
    model_header(&mut output, title, model_id);
    output.push_str("This graph routes from live source owners to their section references and public doors without restating their semantic claims.\n\n## Evidence graph\n\n```mermaid\nflowchart LR\n");
    for (index, source) in model.sources.iter().enumerate() {
        writeln!(output, "  s{index}[\"{}\"]", mermaid(&source.path))
            .expect("String writing cannot fail");
    }
    for (index, section) in model.sections.iter().enumerate() {
        let source_index = model
            .sources
            .iter()
            .position(|source| source.id == section.source)
            .expect("validated source relation");
        writeln!(
            output,
            "  n{index}[\"{}:{} {}\"]",
            section.first_line,
            mermaid(&section.heading),
            section.roles.iter().cloned().collect::<Vec<_>>().join(",")
        )
        .expect("String writing cannot fail");
        writeln!(output, "  s{source_index} --> n{index}").expect("String writing cannot fail");
    }
    output.push_str("```\n\n## Package graph\n\n```mermaid\nflowchart LR\n");
    let packages: BTreeSet<_> = model
        .edges
        .iter()
        .flat_map(|edge| [edge.from.as_str(), edge.to.as_str()])
        .collect();
    let package_ids: BTreeMap<_, _> = packages
        .iter()
        .enumerate()
        .map(|(index, package)| (*package, format!("p{index}")))
        .collect();
    for (package, identity) in &package_ids {
        writeln!(output, "  {identity}[\"{package}\"]").expect("String writing cannot fail");
    }
    for edge in &model.edges {
        let arrow = if edge.plane == "test" { "-.->" } else { "-->" };
        let from = package_ids
            .get(edge.from.as_str())
            .expect("architecture package has generated identity");
        let to = package_ids
            .get(edge.to.as_str())
            .expect("architecture package has generated identity");
        writeln!(
            output,
            "  {from} {arrow}|{}:{}| {to}",
            edge.plane,
            edge.standing.label()
        )
        .expect("String writing cannot fail");
    }
    output.push_str("```\n\n## Capability navigation\n\n");
    for capability in &model.capabilities {
        writeln!(
            output,
            "- `{}` -> `{}` -> `{}:{}`",
            capability.owner, capability.name, capability.path, capability.line
        )
        .expect("String writing cannot fail");
    }
    append_roster(&mut output, model);
    output
}

fn mermaid(value: &str) -> String {
    value.replace(['"', '[', ']', '{', '}'], "'")
}

impl EdgeStanding {
    const fn declared(self) -> bool {
        matches!(self, Self::Matched | Self::DeclaredOnly)
    }

    const fn observed(self) -> bool {
        matches!(self, Self::Matched | Self::ObservedOnly)
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Matched => "matched",
            Self::DeclaredOnly => "declared-only",
            Self::ObservedOnly => "observed-only",
        }
    }
}

fn node_roster(model: &Model) -> Vec<String> {
    let mut nodes = Vec::new();
    nodes.extend(model.sources.iter().map(|value| value.id.clone()));
    nodes.extend(model.sections.iter().map(|value| value.id.clone()));
    nodes.extend(model.edges.iter().map(|value| value.id.clone()));
    nodes.extend(model.capabilities.iter().map(|value| value.id.clone()));
    nodes.sort();
    nodes
}

fn append_roster(output: &mut String, model: &Model) {
    output.push_str("\n## Node roster\n\n");
    for node in node_roster(model) {
        writeln!(output, "- `{node}`").expect("String writing cannot fail");
    }
}

fn validate_renderings(
    repo: &Path,
    model: &Model,
    model_id: &str,
    assurance_view: &str,
    navigation_view: &str,
) -> Result<String, String> {
    let expected = node_roster(model);
    let assurance_identity = read_identity(assurance_view)?;
    let navigation_identity = read_identity(navigation_view)?;
    if assurance_identity != model_id || navigation_identity != model_id {
        return Err("primary renderings disagree about model identity".to_owned());
    }
    if read_rendered_roster(assurance_view) != expected
        || read_rendered_roster(navigation_view) != expected
    {
        return Err("primary renderings disagree about model node roster".to_owned());
    }
    if canonical(model) != canonical(model) {
        return Err("canonical model encoding is nondeterministic".to_owned());
    }

    let mut mutated = model.clone();
    let first = mutated
        .sections
        .first_mut()
        .ok_or_else(|| "model has no section mutation seat".to_owned())?;
    if !first.roles.insert("mutated-control".to_owned()) {
        return Err("model mutation control already existed".to_owned());
    }
    let mutated_id = git_hash_bytes(repo, canonical(&mutated).as_bytes())?;
    let mutated_assurance = assurance(&mutated, &mutated_id, "Macroonz assurance view");
    let mutated_navigation =
        navigation(&mutated, &mutated_id, "Macroonz evidence navigation graph");
    if mutated_id == model_id
        || mutated_assurance == assurance_view
        || mutated_navigation == navigation_view
    {
        return Err("model mutation failed to move both primary renderings".to_owned());
    }

    let renderer_only = navigation(model, model_id, "Alternate navigation presentation");
    if renderer_only == navigation_view || read_identity(&renderer_only)? != model_id {
        return Err("renderer-only change failed identity-preservation control".to_owned());
    }

    let mut architecture_control = model.clone();
    let edge = architecture_control
        .edges
        .first_mut()
        .ok_or_else(|| "model has no architecture control seat".to_owned())?;
    edge.standing = EdgeStanding::DeclaredOnly;
    if edge.standing.label() != "declared-only" {
        return Err(
            "planted architecture removal did not expose declared-only standing".to_owned(),
        );
    }
    let architecture_id = git_hash_bytes(repo, canonical(&architecture_control).as_bytes())?;
    if architecture_id == model_id {
        return Err("planted architecture removal did not move model identity".to_owned());
    }

    let matched = model
        .edges
        .iter()
        .filter(|edge| edge.standing.label() == "matched")
        .count();
    let unmatched = model.edges.len().saturating_sub(matched);
    Ok(format!(
        "check\tstanding\tdetail\nidentity\tpassed\t{model_id}\nshared-node-roster\tpassed\t{} nodes\ndeterministic-canonical\tpassed\tbyte-identical\nmodel-mutation\tpassed\t{}\nrenderer-only-change\tpassed\tidentity-preserved\narchitecture-liveness\tpassed\t{matched} matched; {unmatched} unmatched\narchitecture-removal-control\tpassed\t{}\nsemantic-authority\tpassed\tcoordinates-and-headings-only\n",
        expected.len(),
        mutated_id,
        architecture_id
    ))
}

fn read_identity(rendering: &str) -> Result<&str, String> {
    rendering
        .lines()
        .find_map(|line| {
            line.strip_prefix("- Model identity: `")
                .and_then(|rest| rest.strip_suffix('`'))
        })
        .ok_or_else(|| "rendering lacks model identity".to_owned())
}

fn read_rendered_roster(rendering: &str) -> Vec<String> {
    let mut inside = false;
    let mut result = Vec::new();
    for line in rendering.lines() {
        if line == "## Node roster" {
            inside = true;
            continue;
        }
        if inside {
            if line.starts_with("## ") {
                break;
            }
            if let Some(value) = line
                .strip_prefix("- `")
                .and_then(|rest| rest.strip_suffix('`'))
            {
                result.push(value.to_owned());
            }
        }
    }
    result.sort();
    result
}

fn write_file(path: &Path, content: &str) -> Result<(), String> {
    if content.contains('\r') {
        return Err(format!("{} contains CR bytes", path.display()));
    }
    fs::write(path, content.as_bytes()).map_err(|error| format!("{}: {error}", path.display()))
}

fn git_hash_bytes(repo: &Path, bytes: &[u8]) -> Result<String, String> {
    let mut child = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["hash-object", "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cannot execute git hash-object: {error}"))?;
    child
        .stdin
        .as_mut()
        .ok_or_else(|| "git hash-object stdin unavailable".to_owned())?
        .write_all(bytes)
        .map_err(|error| format!("cannot feed git hash-object: {error}"))?;
    let output = child
        .wait_with_output()
        .map_err(|error| format!("cannot wait for git hash-object: {error}"))?;
    if output.status.success() {
        String::from_utf8(output.stdout)
            .map(|value| value.trim().to_owned())
            .map_err(|error| error.to_string())
    } else {
        Err(format!("git hash-object refused with {}", output.status))
    }
}

fn git_success(repo: &Path, arguments: &[&str]) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(arguments)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| format!("cannot execute git: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("git {} refused with {status}", arguments.join(" ")))
    }
}

fn git_text(repo: &Path, arguments: &[&str]) -> Result<String, String> {
    let bytes = git_bytes(repo, arguments)?;
    String::from_utf8(bytes)
        .map(|value| value.trim().to_owned())
        .map_err(|error| error.to_string())
}

fn git_bytes(repo: &Path, arguments: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(arguments)
        .output()
        .map_err(|error| format!("cannot execute git: {error}"))?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!(
            "git {} refused with {}: {}",
            arguments.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
