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
//!
//! Both readings go through the decoders that own what they are reading. A
//! declared module is an ITEM, so `syn` is asked which items a crate root
//! declares and in what order; a reference is a PATH in the token stream, so
//! `proc-macro2` is asked to lex it. The line reader this replaced discovered
//! modules by matching `mod ` at the head of a trimmed line and found the band
//! declarations with `str::find` on an attribute spelled exactly one way — so a
//! module declared across two lines was invisible to it, and an attribute
//! written with different spacing was a band `lib.rs` "did not declare".

use std::collections::BTreeSet;
use std::str::FromStr;

use proc_macro2::{Delimiter, TokenStream, TokenTree};

use crate::repository::snapshot::{MACHINE_DIRECTORY, RepositorySnapshot};
use crate::repository::types::{CanonicalPath, ModuleLayout};

/// The files a numbered band home carries.
const HOME_FILES: [&str; 3] = ["README.md", "mod.rs", "types.rs"];

/// The crate root of the machine.
const MACHINE_ROOT: &str = "src/lib.rs";

/// The services crate's source directory, whose unnumbered module list carries
/// its dependency order the way numbered directories carry the machine's.
const TOOLING_SOURCE: &str = "macros/macroc/src";

/// The attribute a band declaration carries.
const PATH_ATTRIBUTE: &str = "path";

/// The attribute that takes a declaration out of the order it would otherwise
/// stand in.
const CONDITION_ATTRIBUTE: &str = "cfg";

/// Every numbered band directory is complete (README.md, mod.rs, types.rs) and
/// `lib.rs` declares every band via its `#[path]` attribute in ascending band
/// order — the band map and the crate never drift apart.
pub(crate) fn check_band_map(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let bands = band_directories(snapshot);
    let mut offenders = Vec::new();
    for band in &bands {
        for file in HOME_FILES {
            if snapshot
                .files()
                .get(&format!("{MACHINE_DIRECTORY}/{band}/{file}"))
                .is_none()
            {
                offenders.push(format!("{band} missing {file}"));
            }
        }
    }
    let root = snapshot
        .rust()
        .source(&CanonicalPath::spelled(MACHINE_ROOT))
        .taken(MACHINE_ROOT)?;
    let declared = band_declarations(root);
    let mut positions = Vec::new();
    for band in &bands {
        match declared.iter().position(|stated| stated == band) {
            Some(position) => positions.push((position, band.clone())),
            None => offenders.push(format!("lib.rs does not declare {band}")),
        }
    }
    let mut ascending = positions.clone();
    ascending.sort();
    if ascending != positions {
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

/// Every numbered band directory the machine's tree carries, in ascending band
/// order.
///
/// A band is a directory whose name opens with two digits and an underscore.
/// The set is derived from the reading rather than from a list anybody
/// maintains.
fn band_directories(snapshot: &RepositorySnapshot) -> Vec<String> {
    let mut bands = BTreeSet::new();
    for (path, _) in snapshot.files().under(MACHINE_DIRECTORY) {
        let Some(tail) = path
            .as_str()
            .get(MACHINE_DIRECTORY.len().saturating_add(1)..)
        else {
            continue;
        };
        let Some((head, _)) = tail.split_once('/') else {
            continue;
        };
        let Some((number, _)) = head.split_once('_') else {
            continue;
        };
        if number.len() == 2 && number.chars().all(|digit| digit.is_ascii_digit()) {
            bands.insert(head.to_owned());
        }
    }
    bands.into_iter().collect()
}

/// The band directories one crate root declares, in declaration order.
///
/// Read off the `#[path = "…"]` attribute of each declared module, which is
/// what a band declaration IS. The directory is the path's own leading segment,
/// so the reading never has to be told how a band's `mod.rs` is spelled.
fn band_declarations(root: &syn::File) -> Vec<String> {
    root.items.iter().filter_map(declared_band).collect()
}

/// The band directory one declared item names, where it names one.
fn declared_band(item: &syn::Item) -> Option<String> {
    let syn::Item::Mod(module) = item else {
        return None;
    };
    module.attrs.iter().find_map(|attribute| {
        let stated = string_attribute(attribute, PATH_ATTRIBUTE)?;
        let (directory, _) = stated.split_once('/')?;
        Some(directory.to_owned())
    })
}

/// The string one named attribute states, where it states one.
fn string_attribute(attribute: &syn::Attribute, named: &str) -> Option<String> {
    if !attribute.path().is_ident(named) {
        return None;
    }
    let syn::Meta::NameValue(stated) = &attribute.meta else {
        return None;
    };
    let syn::Expr::Lit(literal) = &stated.value else {
        return None;
    };
    let syn::Lit::Str(written) = &literal.lit else {
        return None;
    };
    Some(written.value())
}

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
/// The reading is deliberately narrow, and its narrowness is part of the law it
/// states. It recognizes exactly these routes, and nothing else:
///
/// 1. `crate::name` — a plain path, in a `use` item or in an expression. Both
///    break when the named module moves.
/// 2. `crate::{a::…, b::…}` — a GROUPED use. Every segment head inside the
///    braces is read, at any nesting, so wrapping three imports in one `use`
///    hides none of them.
/// 3. `use crate::name as alias;` — an ALIASED import. The edge is read off the
///    `crate::` path, so renaming the binding hides nothing.
/// 4. `super::name` inside a SINGLE-FILE module — which is the crate root under
///    another spelling, and therefore the same edge.
/// 5. `crate::Thing` where `Thing` is not a declared module — a CRATE-ROOT
///    RE-EXPORT route. Reaching a sibling's content through the crate root
///    launders the edge: the reference names no owner, so nothing about the
///    declaration order can be read off it. Owner paths only.
/// 6. A rustdoc intra-doc link naming `crate::…`, read out of the documentation
///    string the lexer hands back on the item it documents.
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
pub(crate) fn check_tooling_module_order(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let root_path = format!("{TOOLING_SOURCE}/lib.rs");
    let root = snapshot
        .rust()
        .source(&CanonicalPath::spelled(&root_path))
        .taken(&root_path)?;
    let order = declared_module_order(root);
    if order.is_empty() {
        return Err(format!("{root_path} declares no modules"));
    }
    let mut modules = Vec::new();
    for name in &order {
        let (references, layout) = module_references(snapshot, name)?;
        modules.push((name.clone(), references, layout));
    }
    let violations = module_order_violations(&order, &modules);
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations.join("; "))
    }
}

/// The module names one crate root declares, in declaration order.
///
/// Both `mod name;` and `pub mod name;` count — a private module participates
/// in the order exactly as a public one does. A declaration carrying a build
/// CONDITION does not: the proof surface is outside the order by construction.
fn declared_module_order(root: &syn::File) -> Vec<String> {
    root.items
        .iter()
        .filter_map(|item| {
            let syn::Item::Mod(module) = item else {
                return None;
            };
            if module
                .attrs
                .iter()
                .any(|attribute| attribute.path().is_ident(CONDITION_ATTRIBUTE))
            {
                return None;
            }
            Some(module.ident.to_string())
        })
        .collect()
}

/// Every crate-root name one declared module reaches, and the layout it is in.
///
/// This is the stage that turns a declared NAME into the edges an order is read
/// off. `name.rs` is its own source; `name/` is every `.rs` file under it, read
/// separately and unioned, because a submodule reaching forward is its parent
/// reaching forward.
fn module_references(
    snapshot: &RepositorySnapshot,
    name: &str,
) -> Result<(Vec<String>, ModuleLayout), String> {
    let flat = format!("{TOOLING_SOURCE}/{name}.rs");
    if snapshot.files().get(&flat).is_some() {
        let text = snapshot.files().text(&flat).taken(&flat)?;
        return Ok((references_of(text, ModuleLayout::Flat)?, ModuleLayout::Flat));
    }
    let directory = format!("{TOOLING_SOURCE}/{name}");
    let mut found = Vec::new();
    let mut carried = false;
    for (path, _) in snapshot.files().under(&directory) {
        if !path.extension_is("rs") {
            continue;
        }
        carried = true;
        let text = snapshot.files().text(path.as_str()).taken(path.as_str())?;
        found.extend(references_of(text, ModuleLayout::Directory)?);
    }
    if carried {
        Ok((found, ModuleLayout::Directory))
    } else {
        Err(format!(
            "{name} is declared and is neither {flat} nor {directory}/"
        ))
    }
}

/// Every crate-root name one source reaches.
///
/// The source is LEXED rather than searched. `proc-macro2` owns Rust's token
/// grammar, so `crate` is an identifier here rather than a substring: a longer
/// name ending in `crate`, the word inside an ordinary comment, and the word
/// inside a string literal are all what they are, and none of them is a
/// reference. A documentation comment arrives as the string literal of a `doc`
/// attribute, which is where an intra-doc link lives, so those are read as text
/// on purpose — a rustdoc link IS prose naming a path.
fn references_of(text: &str, layout: ModuleLayout) -> Result<Vec<String>, String> {
    let tokens = TokenStream::from_str(text)
        .map_err(|error| format!("the source does not lex as Rust: {error}"))?;
    let mut found = Vec::new();
    read_references(tokens, layout, &mut found);
    Ok(found)
}

/// Whether one opening word names the crate root under this layout.
///
/// In a directory module, a submodule saying `super::` is naming its own
/// parent, which is not a forward reference at all; in a flat module, `super::`
/// and `crate::` name the same place, so both are read.
fn opens_the_crate_root(word: &str, layout: ModuleLayout) -> bool {
    match layout {
        ModuleLayout::Directory => word == "crate",
        ModuleLayout::Flat => word == "crate" || word == "super",
    }
}

/// Walks one token stream, collecting every crate-root reference it spells.
///
/// A string LITERAL is a value rather than a path, so one is never read as a
/// reference — with exactly one exception, stated here and nowhere else: the
/// literal inside a `doc` attribute, which is where a rustdoc intra-doc link
/// lives. That is why documentation is reached at the attribute rather than by
/// scanning every literal the source happens to carry: a path written inside an
/// ordinary string is a string.
fn read_references(tokens: TokenStream, layout: ModuleLayout, into: &mut Vec<String>) {
    let trees: Vec<TokenTree> = tokens.into_iter().collect();
    for (index, tree) in trees.iter().enumerate() {
        match *tree {
            TokenTree::Ident(ref word) => {
                if opens_the_crate_root(&word.to_string(), layout)
                    && let Some(tail) = separator_follows(&trees, index)
                {
                    into.extend(referenced_heads(tail));
                }
            }
            TokenTree::Group(ref group) => {
                if group.delimiter() == Delimiter::Bracket && opens_documentation(group.stream()) {
                    into.extend(documentation_references(group.stream(), layout));
                    continue;
                }
                read_references(group.stream(), layout, into);
            }
            TokenTree::Literal(_) | TokenTree::Punct(_) => (),
        }
    }
}

/// Whether one attribute's body is a `doc` attribute — which is what a
/// documentation comment arrives as.
fn opens_documentation(tokens: TokenStream) -> bool {
    matches!(tokens.into_iter().next(), Some(TokenTree::Ident(ref word)) if word == "doc")
}

/// Every crate-root name one documentation attribute's own literals name.
fn documentation_references(tokens: TokenStream, layout: ModuleLayout) -> Vec<String> {
    tokens
        .into_iter()
        .filter_map(|written| match written {
            TokenTree::Literal(documented) => Some(documented.to_string()),
            TokenTree::Group(_) | TokenTree::Ident(_) | TokenTree::Punct(_) => None,
        })
        .flat_map(|documented| documented_references(&documented, layout))
        .collect()
}

/// The tree after a `::` separator following the token at `index`, where one
/// follows.
fn separator_follows(trees: &[TokenTree], index: usize) -> Option<&TokenTree> {
    let first = trees.get(index.saturating_add(1))?;
    let second = trees.get(index.saturating_add(2))?;
    let TokenTree::Punct(ref opening) = *first else {
        return None;
    };
    let TokenTree::Punct(ref closing) = *second else {
        return None;
    };
    if opening.as_char() != ':' || closing.as_char() != ':' {
        return None;
    }
    trees.get(index.saturating_add(3))
}

/// The segment heads one crate-root path reaches: the single name of a plain
/// path, or every head inside a grouped use, at any nesting.
fn referenced_heads(tail: &TokenTree) -> Vec<String> {
    match *tail {
        TokenTree::Ident(ref named) => vec![named.to_string()],
        TokenTree::Group(ref group) if group.delimiter() == Delimiter::Brace => {
            grouped_heads(group.stream())
        }
        TokenTree::Group(_) | TokenTree::Punct(_) | TokenTree::Literal(_) => Vec::new(),
    }
}

/// Every head inside one grouped use, at any nesting.
fn grouped_heads(tokens: TokenStream) -> Vec<String> {
    let mut heads = Vec::new();
    let mut at_head = true;
    for tree in tokens {
        match tree {
            TokenTree::Punct(ref mark) if mark.as_char() == ',' => at_head = true,
            TokenTree::Ident(ref named) => {
                if at_head {
                    heads.push(named.to_string());
                    at_head = false;
                }
            }
            TokenTree::Group(ref group) if group.delimiter() == Delimiter::Brace => {
                heads.extend(grouped_heads(group.stream()));
                at_head = false;
            }
            TokenTree::Group(_) | TokenTree::Literal(_) | TokenTree::Punct(_) => (),
        }
    }
    heads
}

/// Every crate-root name one documentation string names.
///
/// A rustdoc intra-doc link is prose carrying a path, which is why this half is
/// read as text — and it is the ONE half that is, stated here rather than left
/// as the case somebody notices later.
fn documented_references(written: &str, layout: ModuleLayout) -> Vec<String> {
    let mut found = Vec::new();
    for opening in ["crate", "super"] {
        if !opens_the_crate_root(opening, layout) {
            continue;
        }
        let needle = format!("{opening}::");
        let mut from = 0usize;
        while let Some(offset) = written.get(from..).and_then(|rest| rest.find(&needle)) {
            let start = from.saturating_add(offset);
            let end = start.saturating_add(needle.len());
            let before_is_word = written
                .get(..start)
                .and_then(|head| head.chars().next_back())
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_');
            if !before_is_word
                && let Some(tail) = written.get(end..)
                && let Some(name) = leading_name(tail)
            {
                found.push(name);
            }
            from = end;
        }
    }
    found
}

/// The identifier one text opens with, where it opens with one.
fn leading_name(tail: &str) -> Option<String> {
    let name: String = tail
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() { None } else { Some(name) }
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
    modules: &[(String, Vec<String>, ModuleLayout)],
) -> Vec<String> {
    let position = |name: &str| order.iter().position(|declared| declared == name);
    let mut violations = Vec::new();
    for (name, references, _) in modules {
        let Some(here) = position(name) else {
            continue;
        };
        let mut reported: Vec<String> = Vec::new();
        for referenced in references {
            if reported.contains(referenced) {
                continue;
            }
            match position(referenced) {
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
/// The module-order law is pure over `(order, module references)`, so its
/// reversals are synthetic module sets held in memory. The band map reads a
/// tree, so its reversal is planted against a scratch root outside the
/// repository. Neither writes inside the tree it guards.
#[cfg(test)]
mod tests {
    use super::{
        check_band_map, check_tooling_module_order, declared_module_order, module_order_violations,
        references_of,
    };
    use crate::checks::scratch::Scratch;
    use crate::repository::snapshot::repository_snapshot;
    use crate::repository::types::ModuleLayout;

    /// One synthetic module set, as `(name, source text)` pairs. Every synthetic
    /// module is flat: the directory layout is exercised against the real tree,
    /// where the directory exists.
    fn sources(pairs: &[(&str, &str)]) -> Result<Vec<(String, Vec<String>, ModuleLayout)>, String> {
        let mut read = Vec::new();
        for (name, text) in pairs {
            read.push((
                (*name).to_owned(),
                references_of(text, ModuleLayout::Flat)?,
                ModuleLayout::Flat,
            ));
        }
        Ok(read)
    }

    /// One crate root, parsed.
    fn root(text: &str) -> Result<syn::File, String> {
        syn::parse_file(text).map_err(|error| error.to_string())
    }

    /// The declaration order is read out of the ITEMS in file order, not
    /// sorted, and the test-only proof surface is excluded from it.
    ///
    /// Planted reversal for the line reader this replaced: the last declaration
    /// here is written across two lines, which no reader whose subject is a
    /// trimmed line can see.
    #[test]
    fn the_declaration_order_is_item_order_without_the_proof_surface() -> Result<(), String> {
        let lib = "//! doc\n\npub mod plane;\n\n/// note\npub mod refusal;\n\nmod helper;\n\n\
                   #[cfg(test)]\nmod laws;\n\nmod\n    wrapped;\n";
        assert_eq!(
            declared_module_order(&root(lib)?),
            vec![
                String::from("plane"),
                String::from("refusal"),
                String::from("helper"),
                String::from("wrapped"),
            ]
        );
        Ok(())
    }

    /// Planted reversal: a module reaching FORWARD to a module declared after
    /// it — the shape every cycle contains at least one of.
    #[test]
    fn a_forward_reference_is_a_violation() -> Result<(), String> {
        let order = vec![String::from("plane"), String::from("planning")];
        let found = module_order_violations(
            &order,
            &sources(&[
                ("plane", "use crate::planning::ProjectionPlan;\n"),
                ("planning", "use crate::plane::ExactIdentity;\n"),
            ])?,
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("declared later")));
        assert!(found.iter().any(|v| v.contains("`plane`")));
        Ok(())
    }

    /// Planted reversal: the exact cycle this discipline was written to kill —
    /// two modules importing each other. Whichever way the pair is declared,
    /// one of the two edges points forward.
    #[test]
    fn a_two_module_cycle_is_a_violation() -> Result<(), String> {
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
            ])?,
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.iter().any(|v| v.contains("`planning`")));
        Ok(())
    }

    /// Planted reversal: the forward reference spelled inline rather than in a
    /// `use` line, inside a grouped use, and inside a rustdoc link — three
    /// places no scan of import lines alone would see.
    #[test]
    fn a_forward_reference_hides_in_none_of_its_spellings() -> Result<(), String> {
        let order = vec![String::from("plane"), String::from("diagnostics")];
        for spelling in [
            "fn f() { let _ = crate::diagnostics::MacrocPhase::Capture; }\n",
            "use crate::{plane::Own, diagnostics::MacrocPhase};\n",
            "/// See [`crate::diagnostics::MacrocPhase`] for the phases.\npub fn f() {}\n",
            "use crate::diagnostics::MacrocPhase as Phase;\n",
        ] {
            let found = module_order_violations(&order, &sources(&[("plane", spelling)])?);
            assert_eq!(found.len(), 1, "{spelling} -> {found:?}");
        }
        Ok(())
    }

    /// The positive control: a clean set passes. Backward references, repeated
    /// references, a module naming itself, and a longer identifier merely
    /// ENDING in `crate` are all lawful, so the check reports something real
    /// rather than everything.
    ///
    /// The lexer is what makes the last three of these free: `othercrate` is one
    /// identifier, a comment is not a token at all, and a path inside a string
    /// literal is a string.
    #[test]
    fn a_clean_module_set_passes() -> Result<(), String> {
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
                     use othercrate::planning::Nothing;\n\
                     // crate::diagnostics in a comment is not an edge\n\
                     fn f() -> &'static str { \"crate::diagnostics\" }\n\
                     fn g() { crate::planning::own(); }\n",
                ),
            ])?,
        );
        assert!(found.is_empty(), "{found:?}");
        Ok(())
    }

    /// The real services tree holds: declaration order IS its dependency order.
    #[test]
    fn the_real_services_modules_are_in_dependency_order() -> Result<(), String> {
        let found = check_tooling_module_order(repository_snapshot()?);
        assert!(found.is_ok(), "{found:?}");
        Ok(())
    }

    /// The real machine tree holds: every band is complete and declared in band
    /// order.
    #[test]
    fn the_real_band_map_matches_lib() -> Result<(), String> {
        let found = check_band_map(repository_snapshot()?);
        assert!(found.is_ok(), "{found:?}");
        Ok(())
    }

    /// Planted reversal: an incomplete band, a band `lib.rs` does not declare,
    /// and declarations out of band order. Three ways the band map and the
    /// crate drift apart, and the third is the one no file listing would catch.
    #[test]
    fn a_band_map_that_drifts_from_lib_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("band-map");
        let ordered = "#[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n\
                       #[path = \"01_logic/mod.rs\"]\npub mod logic;\n";
        for band in ["00_refusal", "01_logic"] {
            for file in ["README.md", "mod.rs", "types.rs"] {
                scratch.write(&format!("src/{band}/{file}"), "the home's content\n");
            }
        }
        scratch.write("src/lib.rs", ordered);
        assert!(check_band_map(&scratch.read()?).is_ok());

        scratch.remove("src/01_logic/types.rs");
        let incomplete = check_band_map(&scratch.read()?);
        assert!(incomplete.is_err_and(|reason| reason.contains("01_logic missing types.rs")));
        scratch.write("src/01_logic/types.rs", "the home's content\n");

        scratch.write(
            "src/lib.rs",
            "#[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n",
        );
        let undeclared = check_band_map(&scratch.read()?);
        assert!(undeclared.is_err_and(|reason| reason.contains("does not declare 01_logic")));

        scratch.write(
            "src/lib.rs",
            "#[path = \"01_logic/mod.rs\"]\npub mod logic;\n\
             #[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n",
        );
        let reordered = check_band_map(&scratch.read()?);
        assert!(reordered.is_err_and(|reason| reason.contains("out of band order")));
        Ok(())
    }

    /// The band declaration is read off the ATTRIBUTE rather than off a
    /// literal spelling of it.
    ///
    /// Planted reversal for the substring reader this replaced, which looked for
    /// `#[path = "00_refusal/mod.rs"]` exactly. Written with different spacing
    /// the attribute states the same declaration and that reader reported a band
    /// `lib.rs` "does not declare" — a law refusing a lawful crate over
    /// whitespace.
    #[test]
    fn a_band_declaration_is_read_however_it_is_spaced() -> Result<(), String> {
        let scratch = Scratch::named("band-map-spacing");
        for file in ["README.md", "mod.rs", "types.rs"] {
            scratch.write(&format!("src/00_refusal/{file}"), "the home's content\n");
        }
        scratch.write(
            "src/lib.rs",
            "#[path=\"00_refusal/mod.rs\"]\npub mod refusal;\n",
        );
        let found = check_band_map(&scratch.read()?);
        assert!(found.is_ok(), "{found:?}");
        Ok(())
    }
}
