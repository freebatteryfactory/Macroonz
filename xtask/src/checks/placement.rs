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
//!
//! # Both populations are TOTAL over their directory
//!
//! Each law is about a directory, so each classifies every direct child of that
//! directory into exactly one named state and refuses the state that means
//! "nothing here recognized this". Both readings used to RECOGNIZE their subject
//! — a directory opening with two digits and an underscore, a `mod` declaration
//! carrying no build condition — and let everything else fall through a silent
//! `continue`. That is not a narrow claim, it is an unstated one: a misspelled
//! architectural directory and a source no declaration accounts for each left
//! the population with nothing said anywhere, and both checks went on printing
//! PASS while guarding less than the sentence above them says.
//!
//! Neither law admits a conditionally-compiled production module, and the reason
//! is the same in both crates. These readings establish ONE order. A module
//! compiled in some builds and not others stands in as many orders as there are
//! build populations, and this reading can establish none of them — so the one
//! condition either law admits is the proof surface's exact `#[cfg(test)]`, and
//! every other conditioned declaration refuses rather than quietly leaving the
//! order.

use std::collections::BTreeMap;
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

/// The file cargo compiles a crate from. It declares the order rather than
/// standing in it, in both crates.
const CRATE_ROOT_FILE: &str = "lib.rs";

/// The extension a Rust source carries.
const SOURCE_SUFFIX: &str = ".rs";

/// The files the machine's crate root reserves beside it: the root's own public
/// types, and the residue proof surface. Neither is a semantic home, and neither
/// is an accident — the working law seats both at the root by name.
const RESERVED_ROOT_FILES: [&str; 2] = ["laws.rs", "types.rs"];

/// The attribute a band declaration carries.
const PATH_ATTRIBUTE: &str = "path";

/// The attribute that takes a declaration out of the order it would otherwise
/// stand in.
const CONDITION_ATTRIBUTE: &str = "cfg";

/// The attribute that would attach a build condition to a declaration
/// indirectly, and which therefore conditions it just as surely.
const CONDITIONED_ATTRIBUTE: &str = "cfg_attr";

/// The one build condition either order admits: the proof surface's own.
const PROOF_SURFACE_CONDITION: &str = "test";

/// Every numbered band directory is complete (README.md, mod.rs, types.rs) and
/// `lib.rs` declares every band via its `#[path]` attribute in ascending band
/// order — the band map and the crate never drift apart.
///
/// # Why every child of `src/` is classified rather than only the numbered ones
///
/// A band was once RECOGNIZED — a direct child whose first component opened with
/// two digits and an underscore — and everything the recognizer did not match
/// fell through a silent `continue`. That is not a narrow population, it is an
/// unstated one: `src/O5_bounds/`, `src/5_bounds/`, `src/bounds/` and
/// `src/notes.md` each leave the band map without a word said anywhere, and the
/// check that reports on the band map keeps printing PASS. So the classification
/// is TOTAL. Every direct child of `src/` is exactly one of four things — a
/// numbered semantic home, the crate root, a file the root grammar reserves, or
/// an invalid entry — and the fourth is a refusal naming what it found.
pub(crate) fn check_band_map(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let mut offenders = Vec::new();
    let mut bands = Vec::new();
    for child in machine_source_children(snapshot).values() {
        match *child {
            MachineSourceChild::NumberedHome(ref named) => bands.push(named.clone()),
            MachineSourceChild::CrateRoot | MachineSourceChild::ReservedRootFile => (),
            MachineSourceChild::Invalid(ref reason) => offenders.push(reason.clone()),
        }
    }
    if bands.is_empty() {
        return Err(String::from(
            "no numbered band directory was found: this denominator cannot be empty while the \
             machine states its dependency bands with numbered directories, so the reader is \
             looking at the wrong tree",
        ));
    }
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
    let declared = band_declarations(root, &mut offenders);
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

/// What one direct child of the machine's source directory is.
///
/// Four states, total over the tree: nothing under `src/` is outside them, which
/// is exactly what naming them buys. An entry that matches none of the first
/// three is the fourth and says so, rather than leaving the population by a
/// route nobody reads.
#[derive(Debug, PartialEq, Eq)]
enum MachineSourceChild {
    /// `NN_name/` — a numbered semantic home, carrying the band's own files.
    NumberedHome(String),
    /// `lib.rs` — the crate root, the one file cargo compiles the machine from.
    CrateRoot,
    /// A file the crate root's own grammar reserves beside it.
    ReservedRootFile,
    /// Anything else, carried with the words the refusal is written in.
    Invalid(String),
}

/// Every direct child of the machine's source directory, classified, keyed by
/// the child's own name so the bands come back in ascending band order.
///
/// Derived from the one reading: the children are the distinct first components
/// of the paths beneath `src/`, and a path this reader cannot cut into one is
/// itself an entry rather than a skip.
fn machine_source_children(snapshot: &RepositorySnapshot) -> BTreeMap<String, MachineSourceChild> {
    let mut children = BTreeMap::new();
    for (path, _) in snapshot.files().under(MACHINE_DIRECTORY) {
        let Some(tail) = path
            .as_str()
            .get(MACHINE_DIRECTORY.len().saturating_add(1)..)
        else {
            children.insert(
                path.to_string(),
                MachineSourceChild::Invalid(format!(
                    "`{path}` sits beneath {MACHINE_DIRECTORY}/ and this reader cannot cut it into \
                     a direct child, so what it is under the machine's source directory is unknown \
                     rather than nothing"
                )),
            );
            continue;
        };
        match tail.split_once('/') {
            Some((head, _)) => {
                children.insert(head.to_owned(), classified_directory(head));
            }
            None => {
                children.insert(tail.to_owned(), classified_root_file(tail));
            }
        }
    }
    children
}

/// What one directory sitting directly in `src/` is.
fn classified_directory(named: &str) -> MachineSourceChild {
    let Some((number, rest)) = named.split_once('_') else {
        return MachineSourceChild::Invalid(format!(
            "`{MACHINE_DIRECTORY}/{named}/` carries no band number: a semantic home is `NN_name/`, \
             and a directory that is not one states no band at all, so the band order says nothing \
             about what it may import"
        ));
    };
    if number.len() != 2 || !number.chars().all(|digit| digit.is_ascii_digit()) {
        return MachineSourceChild::Invalid(format!(
            "`{MACHINE_DIRECTORY}/{named}/` opens with `{number}` where a semantic home's two-digit \
             band number belongs, so it names no band and stands in no order"
        ));
    }
    if rest.is_empty() {
        return MachineSourceChild::Invalid(format!(
            "`{MACHINE_DIRECTORY}/{named}/` carries a band number and no name, so the coordinate it \
             occupies is about nothing"
        ));
    }
    MachineSourceChild::NumberedHome(named.to_owned())
}

/// What one file sitting directly in `src/` is.
fn classified_root_file(named: &str) -> MachineSourceChild {
    if named == CRATE_ROOT_FILE {
        return MachineSourceChild::CrateRoot;
    }
    if RESERVED_ROOT_FILES.contains(&named) {
        return MachineSourceChild::ReservedRootFile;
    }
    MachineSourceChild::Invalid(format!(
        "`{MACHINE_DIRECTORY}/{named}` sits directly in the machine's source directory and is \
         neither the crate root nor one of the files the root reserves beside it ({}); a semantic \
         noun lives in its numbered home, and the root is never a shared-noun drawer",
        RESERVED_ROOT_FILES.join(", ")
    ))
}

/// The band directories one crate root declares, in declaration order, with
/// every conditionally-declared band refused into the offences instead.
///
/// Read off the `#[path = "…"]` attribute of each declared module, which is
/// what a band declaration IS. The directory is the path's own leading segment,
/// so the reading never has to be told how a band's `mod.rs` is spelled.
fn band_declarations(root: &syn::File, offenders: &mut Vec<String>) -> Vec<String> {
    let mut declared = Vec::new();
    for item in &root.items {
        match band_declaration(item) {
            Some(BandDeclaration::Unconditional(directory)) => declared.push(directory),
            Some(BandDeclaration::Conditional(directory)) => offenders.push(format!(
                "lib.rs declares `{directory}` under a build condition: the band map states ONE \
                 order, and a band compiled in some builds and not others stands in as many orders \
                 as there are build populations — none of which this reading can establish"
            )),
            None => (),
        }
    }
    declared
}

/// How one band declaration stands in the order.
///
/// Two states rather than a flag on the directory, because `clippy.toml` sets
/// `max-struct-bools = 0` and because a bare `true` says nothing about which way
/// the question was asked.
#[derive(Debug, PartialEq, Eq)]
enum BandDeclaration {
    /// Declared unconditionally: the band stands in one order in every build.
    Unconditional(String),
    /// Declared under a build condition, so WHICH order it stands in depends on
    /// which build is being asked about.
    Conditional(String),
}

/// The band directory one declared item names, where it names one.
fn band_declaration(item: &syn::Item) -> Option<BandDeclaration> {
    let syn::Item::Mod(module) = item else {
        return None;
    };
    let directory = module.attrs.iter().find_map(|attribute| {
        let stated = string_attribute(attribute, PATH_ATTRIBUTE)?;
        let (directory, _) = stated.split_once('/')?;
        Some(directory.to_owned())
    })?;
    if module.attrs.iter().any(is_conditional) {
        Some(BandDeclaration::Conditional(directory))
    } else {
        Some(BandDeclaration::Unconditional(directory))
    }
}

/// Whether one attribute conditions the declaration it sits on.
///
/// `cfg_attr` counts. It attaches an attribute — any attribute, `cfg` among them
/// — under a condition, so a declaration carrying one is conditioned just as
/// surely as one carrying `cfg` directly, and by a route this reader cannot
/// settle without deciding which build is meant.
fn is_conditional(attribute: &syn::Attribute) -> bool {
    attribute.path().is_ident(CONDITION_ATTRIBUTE)
        || attribute.path().is_ident(CONDITIONED_ATTRIBUTE)
}

/// Whether one attribute is exactly the proof surface's own condition,
/// `#[cfg(test)]`.
///
/// Exactly: the attribute is `cfg`, its body is one token, and that token is the
/// identifier `test`. `#[cfg(all(test, …))]`, `#[cfg(not(test))]` and
/// `#[cfg_attr(test, …)]` are none of them this, and each is refused for the
/// reason the whole rule exists — a module compiled in some builds and not
/// others stands in no single order.
fn is_the_proof_surface_condition(attribute: &syn::Attribute) -> bool {
    let syn::Meta::List(ref stated) = attribute.meta else {
        return false;
    };
    if !stated.path.is_ident(CONDITION_ATTRIBUTE) {
        return false;
    }
    let mut body = stated.tokens.clone().into_iter();
    match (body.next(), body.next()) {
        (Some(TokenTree::Ident(ref word)), None) => word == PROOF_SURFACE_CONDITION,
        (Some(_) | None, _) => false,
    }
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
/// Exactly one build condition is admitted, and it is the proof surface's:
/// `#[cfg(test)] mod laws;` is declared that way precisely so it can look in
/// every direction without standing in the order it proves. Every OTHER
/// conditional declaration refuses. The reason is the law's own subject: this
/// reading establishes ONE declaration order, and a module compiled in some
/// builds and not others stands in as many orders as there are build
/// populations — `#[cfg(unix)] mod production_home;` would reach forward on one
/// platform and not on another, and neither answer is the order. A production
/// module belongs in the order unconditionally; the alternative is
/// unrepresentable rather than merely discouraged, because the reader refuses
/// the declaration rather than passing over it.
///
/// Every direct child of the source directory is classified for the same reason
/// the machine's is: a module the crate root never declares stands in no order
/// at all, and a reader that only followed declarations would say nothing about
/// it.
pub(crate) fn check_tooling_module_order(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let root_path = format!("{TOOLING_SOURCE}/{CRATE_ROOT_FILE}");
    let root = snapshot
        .rust()
        .source(&CanonicalPath::spelled(&root_path))
        .taken(&root_path)?;
    let DeclaredModules {
        order,
        proof_surface,
        offences,
    } = declared_modules(root);
    let mut violations = offences;
    for child in tooling_source_children(snapshot, &order, &proof_surface).values() {
        if let ToolingSourceChild::Undeclared(ref reason) = *child {
            violations.push(reason.clone());
        }
    }
    if order.is_empty() {
        violations.push(format!(
            "{root_path} declares no module standing in the order"
        ));
    }
    let mut modules = Vec::new();
    for name in &order {
        let (references, layout) = module_references(snapshot, name)?;
        modules.push((name.clone(), references, layout));
    }
    violations.extend(module_order_violations(&order, &modules));
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations.join("; "))
    }
}

/// What one crate root's module declarations state.
///
/// Three lists rather than one, because a declaration is one of three things and
/// the reader that stood here collapsed two of them: it kept the unconditional
/// declarations and dropped every conditioned one into the same silence,
/// so `#[cfg(test)] mod laws;` and `#[cfg(unix)] mod production_home;` were
/// treated alike — the first correctly, the second by accident.
#[derive(Debug)]
struct DeclaredModules {
    /// The names standing in the dependency order, in declaration order.
    order: Vec<String>,
    /// The names declared under the proof surface's own condition. Outside the
    /// order by construction, and still declared sources.
    proof_surface: Vec<String>,
    /// Every declaration this law refuses outright, one line each.
    offences: Vec<String>,
}

/// What one crate root declares, classified.
///
/// Both `mod name;` and `pub mod name;` count — a private module participates in
/// the order exactly as a public one does.
fn declared_modules(root: &syn::File) -> DeclaredModules {
    let mut declared = DeclaredModules {
        order: Vec::new(),
        proof_surface: Vec::new(),
        offences: Vec::new(),
    };
    for item in &root.items {
        let syn::Item::Mod(module) = item else {
            continue;
        };
        let named = module.ident.to_string();
        let conditions: Vec<&syn::Attribute> = module
            .attrs
            .iter()
            .filter(|held| is_conditional(held))
            .collect();
        match *conditions.as_slice() {
            [] => declared.order.push(named),
            [only] if is_the_proof_surface_condition(only) => declared.proof_surface.push(named),
            _ => declared.offences.push(format!(
                "`{named}` is declared under a build condition other than the proof surface's \
                 `#[cfg({PROOF_SURFACE_CONDITION})]`: this law reads ONE declaration order, and a \
                 module compiled in some builds and not others stands in as many orders as there \
                 are build populations, none of which this reading can establish. A production \
                 module stands in the order unconditionally."
            )),
        }
    }
    declared
}

/// What one direct child of the services' source directory is.
///
/// Total over that directory for the reason the machine's classification is
/// total: declaration order IS this crate's dependency order, so a source the
/// crate root never declares stands in no order, and a reader following
/// declarations alone would never say so.
#[derive(Debug, PartialEq, Eq)]
enum ToolingSourceChild {
    /// `lib.rs` — the crate root, which declares the order rather than standing
    /// in it.
    CrateRoot,
    /// A module the crate root declares, written as `name.rs` or as `name/`.
    DeclaredModule,
    /// A module the crate root declares under the proof surface's condition.
    ProofSurface,
    /// A source no declaration accounts for, carried with the words the refusal
    /// is written in.
    Undeclared(String),
}

/// Every direct child of the services' source directory, classified, keyed by
/// the child's own name.
fn tooling_source_children(
    snapshot: &RepositorySnapshot,
    order: &[String],
    proof_surface: &[String],
) -> BTreeMap<String, ToolingSourceChild> {
    let mut children = BTreeMap::new();
    for (path, _) in snapshot.files().under(TOOLING_SOURCE) {
        let Some(tail) = path.as_str().get(TOOLING_SOURCE.len().saturating_add(1)..) else {
            children.insert(
                path.to_string(),
                ToolingSourceChild::Undeclared(format!(
                    "`{path}` sits beneath {TOOLING_SOURCE}/ and this reader cannot cut it into a \
                     direct child, so which declared module owns it is unknown rather than nothing"
                )),
            );
            continue;
        };
        let child = match tail.split_once('/') {
            Some((head, _)) => head,
            None => tail,
        };
        children.insert(
            child.to_owned(),
            classified_tooling_child(child, order, proof_surface),
        );
    }
    children
}

/// What one direct child of the services' source directory is, by name.
fn classified_tooling_child(
    child: &str,
    order: &[String],
    proof_surface: &[String],
) -> ToolingSourceChild {
    if child == CRATE_ROOT_FILE {
        return ToolingSourceChild::CrateRoot;
    }
    let named = match child.strip_suffix(SOURCE_SUFFIX) {
        Some(stem) => stem,
        None => child,
    };
    if order.iter().any(|declared| declared == named) {
        return ToolingSourceChild::DeclaredModule;
    }
    if proof_surface.iter().any(|declared| declared == named) {
        return ToolingSourceChild::ProofSurface;
    }
    ToolingSourceChild::Undeclared(format!(
        "`{TOOLING_SOURCE}/{child}` is a source no `mod` declaration in \
         {TOOLING_SOURCE}/{CRATE_ROOT_FILE} accounts for: declaration order IS this crate's \
         dependency order, so a module the crate root never declares stands in no order and this \
         law reads nothing about what it imports"
    ))
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
    let flat = format!("{TOOLING_SOURCE}/{name}{SOURCE_SUFFIX}");
    let directory = format!("{TOOLING_SOURCE}/{name}");
    let carried: Vec<&CanonicalPath> = snapshot
        .files()
        .under(&directory)
        .map(|(path, _)| path)
        .filter(|path| path.extension_is("rs"))
        .collect();
    match (snapshot.files().get(&flat), carried.as_slice()) {
        (Some(_), []) => {
            let text = snapshot.files().text(&flat).taken(&flat)?;
            Ok((references_of(text, ModuleLayout::Flat)?, ModuleLayout::Flat))
        }
        (None, [_, ..]) => {
            let mut found = Vec::new();
            for path in carried {
                let text = snapshot.files().text(path.as_str()).taken(path.as_str())?;
                found.extend(references_of(text, ModuleLayout::Directory)?);
            }
            Ok((found, ModuleLayout::Directory))
        }
        // Both, which is one declaration written twice. The reader that stood
        // here took the file and left every source under the directory unread,
        // so a submodule reaching forward was invisible while the check kept
        // reporting on the module it belongs to.
        (Some(_), [_, ..]) => Err(format!(
            "{name} is declared once and written twice — as {flat} and as {directory}/ — so which \
             sources carry its edges is a question two answers fit, and this law reads one"
        )),
        (None, []) => Err(format!(
            "{name} is declared and is neither {flat} nor {directory}/"
        )),
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
/// path, or every path rooted directly inside a grouped use.
fn referenced_heads(tail: &TokenTree) -> Vec<String> {
    match *tail {
        TokenTree::Ident(ref named) => vec![named.to_string()],
        TokenTree::Group(ref group) if group.delimiter() == Delimiter::Brace => {
            grouped_heads(group.stream())
        }
        TokenTree::Group(_) | TokenTree::Punct(_) | TokenTree::Literal(_) => Vec::new(),
    }
}

/// Every root head inside one grouped use.
///
/// A nested group continues the path whose head preceded it. Its members are
/// below that root and are not sibling modules of it, so this bounded token
/// grammar deliberately does not descend into nested brace groups.
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
        HOME_FILES, RESERVED_ROOT_FILES, check_band_map, check_tooling_module_order,
        declared_modules, module_order_violations, references_of,
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
        let declared = declared_modules(&root(lib)?);
        assert_eq!(
            declared.order,
            vec![
                String::from("plane"),
                String::from("refusal"),
                String::from("helper"),
                String::from("wrapped"),
            ]
        );
        assert_eq!(declared.proof_surface, vec![String::from("laws")]);
        assert!(declared.offences.is_empty(), "{:?}", declared.offences);
        Ok(())
    }

    /// Planted reversal: a module declared under a build condition that is NOT
    /// the proof surface's.
    ///
    /// The exclusion was written for `#[cfg(test)] mod laws;` and stated as "a
    /// declaration carrying a build condition", so it also let
    /// `#[cfg(unix)] mod production_home;` out of the order entirely: that module
    /// could reach forward on one platform and the law would report nothing,
    /// because it had never been in the population. The exclusion is now the
    /// exact condition it was written for, and every other conditioned
    /// declaration refuses — including the ones that CONTAIN the proof surface's
    /// condition without being it, and the one that attaches a condition
    /// indirectly.
    #[test]
    fn a_module_conditioned_on_anything_but_the_proof_surface_is_a_violation() -> Result<(), String>
    {
        for condition in [
            "#[cfg(unix)]",
            "#[cfg(all(test, feature = \"extra\"))]",
            "#[cfg(not(test))]",
            "#[cfg_attr(test, path = \"elsewhere.rs\")]",
            "#[cfg(test)]\n#[cfg(unix)]",
        ] {
            let lib = format!("pub mod plane;\n\n{condition}\nmod production_home;\n");
            let declared = declared_modules(&root(&lib)?);
            assert_eq!(declared.order, vec![String::from("plane")], "{condition}");
            assert!(declared.proof_surface.is_empty(), "{condition}");
            assert!(
                declared
                    .offences
                    .iter()
                    .any(|offence| offence.contains("production_home")
                        && offence.contains("build populations")),
                "{condition} -> {:?}",
                declared.offences
            );
        }
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

    /// The heads of a grouped use are the paths rooted directly in the group.
    /// Nested members remain beneath their root and do not become crate-root
    /// module edges of their own.
    #[test]
    fn a_nested_grouped_use_reports_only_its_root_heads() -> Result<(), String> {
        let references = references_of("use crate::{a::{b, c}, d};\n", ModuleLayout::Flat)?;
        assert_eq!(references, vec![String::from("a"), String::from("d")]);
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
        let scratch = Scratch::named("band-map")?;
        let ordered = "#[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n\
                       #[path = \"01_logic/mod.rs\"]\npub mod logic;\n";
        for band in ["00_refusal", "01_logic"] {
            for file in ["README.md", "mod.rs", "types.rs"] {
                scratch.write(&format!("src/{band}/{file}"), "the home's content\n")?;
            }
        }
        scratch.write("src/lib.rs", ordered)?;
        assert!(check_band_map(&scratch.read()?).is_ok());

        scratch.remove("src/01_logic/types.rs")?;
        let incomplete = check_band_map(&scratch.read()?);
        assert!(incomplete.is_err_and(|reason| reason.contains("01_logic missing types.rs")));
        scratch.write("src/01_logic/types.rs", "the home's content\n")?;

        scratch.write(
            "src/lib.rs",
            "#[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n",
        )?;
        let undeclared = check_band_map(&scratch.read()?);
        assert!(undeclared.is_err_and(|reason| reason.contains("does not declare 01_logic")));

        scratch.write(
            "src/lib.rs",
            "#[path = \"01_logic/mod.rs\"]\npub mod logic;\n\
             #[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n",
        )?;
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
        let scratch = Scratch::named("band-map-spacing")?;
        for file in ["README.md", "mod.rs", "types.rs"] {
            scratch.write(&format!("src/00_refusal/{file}"), "the home's content\n")?;
        }
        scratch.write(
            "src/lib.rs",
            "#[path=\"00_refusal/mod.rs\"]\npub mod refusal;\n",
        )?;
        let found = check_band_map(&scratch.read()?);
        assert!(found.is_ok(), "{found:?}");
        Ok(())
    }

    /// Planted reversal: an entry under `src/` that is no semantic home.
    ///
    /// Four spellings, and the reader that stood here passed silently over every
    /// one of them, because it RECOGNIZED a band and let everything else fall
    /// through a `continue`. A misspelled architectural directory left the band
    /// population with nothing said anywhere, and the check that reports on the
    /// band map kept printing PASS about a tree with an unclassified directory
    /// in it.
    #[test]
    fn an_entry_under_src_that_is_no_home_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("src-population")?;
        scratch.write(
            "src/lib.rs",
            "#[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n",
        )?;
        for file in HOME_FILES {
            scratch.write(&format!("src/00_refusal/{file}"), "the home's content\n")?;
        }
        for reserved in RESERVED_ROOT_FILES {
            scratch.write(&format!("src/{reserved}"), "the root's own file\n")?;
        }
        assert!(check_band_map(&scratch.read()?).is_ok());

        for (planted, said) in [
            ("src/bounds/mod.rs", "carries no band number"),
            ("src/5_bounds/mod.rs", "opens with `5`"),
            ("src/05_/mod.rs", "band number and no name"),
            ("src/notes.md", "sits directly in"),
        ] {
            scratch.write(planted, "an entry the population never classified\n")?;
            let found = check_band_map(&scratch.read()?);
            assert!(
                found.is_err_and(|reason| reason.contains(said)),
                "{planted} left the population in silence"
            );
            scratch.remove(planted)?;
            assert!(check_band_map(&scratch.read()?).is_ok(), "{planted}");
        }
        Ok(())
    }

    /// Planted reversal: a band declared under a build CONDITION.
    ///
    /// The band map states one order. A band compiled in some builds and not
    /// others stands in as many orders as there are build populations, and the
    /// reader that read `#[path]` alone accepted the declaration as though it
    /// stood in all of them.
    #[test]
    fn a_conditionally_declared_band_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("band-map-condition")?;
        for file in HOME_FILES {
            scratch.write(&format!("src/00_refusal/{file}"), "the home's content\n")?;
        }
        scratch.write(
            "src/lib.rs",
            "#[cfg(unix)]\n#[path = \"00_refusal/mod.rs\"]\npub mod refusal;\n",
        )?;
        let found = check_band_map(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("under a build condition")),
            "a band declared for one platform passed as a band declared for the crate"
        );
        Ok(())
    }

    /// Planted reversal: a source in the services' own directory that no `mod`
    /// declaration accounts for.
    ///
    /// Declaration order IS this crate's dependency order, so a module the crate
    /// root never declares stands in no order at all — and a reader that walked
    /// the declarations never had a word to say about it.
    #[test]
    fn a_services_source_no_declaration_accounts_for_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("tooling-population")?;
        scratch.write(
            "macros/macroc/src/lib.rs",
            "pub mod plane;\n\n#[cfg(test)]\nmod laws;\n",
        )?;
        scratch.write("macros/macroc/src/plane.rs", "//! no edges at all\n")?;
        scratch.write("macros/macroc/src/laws.rs", "//! the proof surface\n")?;
        assert!(check_tooling_module_order(&scratch.read()?).is_ok());

        scratch.write("macros/macroc/src/orphan.rs", "//! nobody declares this\n")?;
        let found = check_tooling_module_order(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("orphan.rs")),
            "a source outside every declaration stood outside the law as well"
        );
        Ok(())
    }

    /// Planted reversal: one declared module written BOTH as a file and as a
    /// directory.
    ///
    /// The reader took the file and returned, leaving every source under the
    /// directory unread — so a submodule reaching forward was invisible while
    /// the check went on reporting about the module that owns it.
    #[test]
    fn a_module_written_as_both_a_file_and_a_directory_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("tooling-layout")?;
        scratch.write(
            "macros/macroc/src/lib.rs",
            "pub mod plane;\npub mod token;\n",
        )?;
        scratch.write("macros/macroc/src/plane.rs", "//! no edges at all\n")?;
        scratch.write("macros/macroc/src/token.rs", "use crate::plane::Own;\n")?;
        assert!(check_tooling_module_order(&scratch.read()?).is_ok());

        scratch.write(
            "macros/macroc/src/plane/inner.rs",
            "use crate::token::Reaching;\n",
        )?;
        let found = check_tooling_module_order(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("written twice")),
            "one declaration written two ways was read one way and passed"
        );
        Ok(())
    }
}
