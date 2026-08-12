//! Declaration order and home completeness.
//!
//! Two laws state the same fact about two crates that spell it differently. The
//! machine states its dependency bands with numbered directories, and `lib.rs`
//! must declare every one of them in band order. The services crate carries no
//! numbers and states the same fact with the one ordering that is left to it —
//! the order its `mod` declarations appear — so that order IS its dependency
//! order. A services home may be a file or a directory, and the check reads both
//! the same way. In both crates the map and the crate are derived from each
//! other rather than maintained by hand, which is why neither can quietly drift.

use std::fs;
use std::path::Path;

use crate::repository::types::ModuleLayout;
use crate::repository::walk::module_source;

/// Every numbered band directory is complete (README.md, mod.rs, types.rs) and
/// `lib.rs` declares every band via its `#[path]` attribute in ascending band
/// order — the band map and the crate never drift apart.
pub(crate) fn check_band_map(root: &Path) -> Result<(), String> {
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

/// The services crate's source directory, whose unnumbered module list carries
/// its dependency order the way numbered directories carry the machine's.
const TOOLING_MODULE_ROOT: [&str; 3] = ["macros", "macroc", "src"];

/// Declaration order IS the dependency order.
///
/// The machine states its bands with numbered directories; the services crate
/// carries no numbers and states the same fact with the one ordering that is
/// left to it — the order its `mod` declarations appear in `lib.rs`. A module
/// may not reference another listed module declared LATER than itself. A module
/// naming ITSELF is not an edge and is lawful, which is why the rule is stated
/// as a prohibition on forward references rather than as a permission for
/// backward ones. That single rule outlaws cycles outright, because every cycle
/// contains at least one forward-pointing edge, and it needs no hand-maintained
/// dependency map: the order is read from the declarations and the edges from
/// the sources.
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
/// 4. `super::name` inside a SINGLE-FILE module — which is the crate root under
///    another spelling, and therefore the same edge.
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
pub(crate) fn check_tooling_module_order(root: &Path) -> Result<(), String> {
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
/// Both openings are read: `crate::` and `super::`. In a single-file module the
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

/// Planted reversals for both orders.
///
/// The module-order law is pure over `(order, module sources)`, so its
/// reversals are synthetic module sets held in memory. The band map reads a
/// directory, so its reversal is planted against a scratch root outside the
/// repository. Neither writes inside the tree it guards.
#[cfg(test)]
mod tests {
    use super::{
        TOOLING_MODULE_ROOT, check_band_map, declared_module_order, module_order_violations,
    };
    use crate::checks::scratch::Scratch;
    use crate::repository::types::ModuleLayout;
    use crate::repository::walk::{module_source, repo_root};
    use std::fs;
    use std::path::PathBuf;

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
