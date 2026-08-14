//! What the tree may spell.
//!
//! Three laws that judge files and declarations by what they ARE rather than by
//! what they agree with: LF line endings with no symlinks, no Python anywhere
//! ever, and no underscore-prefixed field carrying real data. They share one
//! shape — read the whole population, collect every offender, name them all in
//! one refusal — because a rule about every file is worth nothing if some corner
//! of the tree is exempt and worth little if it reports only the first hit.

use crate::repository::snapshot::{
    JUDGE_DIRECTORY, MACHINE_DIRECTORY, RepositorySnapshot, TOOLING_DIRECTORY,
};
use crate::repository::types::{CanonicalPath, LinkState};

/// The marker type an underscore-prefixed field is lawful for.
const TYPE_LEVEL_MARKER: &str = "PhantomData";

/// The trees the field law scans: the machine, the tools that project its
/// contracts, and the plane that judges it.
const SCANNED_TREES: [&str; 3] = [MACHINE_DIRECTORY, TOOLING_DIRECTORY, JUDGE_DIRECTORY];

/// The directory the judge keeps its compiler fixtures under.
const JUDGE_FIXTURES: &str = "testpak/tests";

/// Every file in the repository is LF-only and nothing is a symlink.
///
/// Read off the one snapshot, so the population is the population every other
/// law is about. A file whose bytes could not be read REFUSES rather than
/// passing: whether it carries a carriage return is then unknown, and unknown is
/// not clean.
pub(crate) fn check_lf_and_no_symlinks(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let mut offenders = Vec::new();
    for (path, fact) in snapshot.files().iter() {
        match *fact.link().required(path.as_str())? {
            LinkState::Symlink => {
                offenders.push(format!("symlink: {path}"));
                continue;
            }
            LinkState::RegularFile => (),
        }
        if fact.bytes().required(path.as_str())?.contains(&b'\r') {
            offenders.push(format!("CRLF: {path}"));
        }
    }
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// No Python exists in this repository, ever.
pub(crate) fn check_no_python(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let offenders: Vec<String> = snapshot
        .files()
        .iter()
        .map(|(path, _)| path)
        .filter(|path| path.extension_is("py"))
        .map(CanonicalPath::to_string)
        .collect();
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(format!("python files present: {}", offenders.join(", ")))
    }
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
///
/// # What the reader establishes, and what it does not
///
/// A FIELD is what this law is about, so a field is what it reads: `syn` hands
/// back the fields a source declares, with the type each one declares, and the
/// question is asked of those. The line scanner this replaced asked a different
/// question — whether a LINE began with an underscore and carried a colon — and
/// answered it about function parameters, local bindings, match arms, and
/// anything inside a string or a doc comment that happened to be shaped that
/// way, while a field written across two lines escaped it entirely.
///
/// Two things a parse cannot reach, both failing CLOSED: a field written inside
/// a `macro_rules!` transcriber is not a field until it is expanded, and this
/// reader does not expand; and a type alias that resolves TO `PhantomData` is
/// not recognized, because resolving an alias is the compiler's question rather
/// than a parse's. Each costs a lawful field a refusal it has to spell
/// differently, and neither admits a field carrying data.
///
/// # The one exclusion, and it is the judge's own
///
/// A file BENEATH `testpak/tests/` is a fixture the judge feeds to a compiler on
/// purpose, and several of them are written not to compile — one of them does not
/// parse at all. Such a file declares no field this reader can see and no field
/// any build compiles, so it is outside this law's subject rather than a hole in
/// it. The top-level sources of `testpak/tests/` are real tests and are scanned;
/// the exclusion is exactly the narrowing `crate::checks::obligations` already
/// draws between a seat and a fixture, drawn here for the same reason.
pub(crate) fn check_underscore_fields_are_phantom(
    snapshot: &RepositorySnapshot,
) -> Result<(), String> {
    let mut offenders = Vec::new();
    for tree in SCANNED_TREES {
        for (path, source) in snapshot.rust().under(tree) {
            if is_a_judge_fixture(path) {
                continue;
            }
            let file = source.required(path.as_str())?;
            for named in suppressed_fields(&file.items) {
                offenders.push(format!(
                    "{path}: `{named}` is an underscore field without \
                                        {TYPE_LEVEL_MARKER}"
                ));
            }
        }
    }
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// Whether one source is a fixture the judge feeds to a compiler rather than a
/// source anything builds: a file BENEATH `testpak/tests/` rather than directly
/// in it.
fn is_a_judge_fixture(path: &CanonicalPath) -> bool {
    path.is_under(JUDGE_FIXTURES) && !path.sits_directly_in(JUDGE_FIXTURES)
}

/// Every underscore-prefixed field one item list declares whose type is not the
/// type-level marker, by name.
fn suppressed_fields<'items>(items: impl IntoIterator<Item = &'items syn::Item>) -> Vec<String> {
    let mut found = Vec::new();
    for item in items {
        if let syn::Item::Struct(declared) = item {
            collect(&declared.fields, &mut found);
        } else if let syn::Item::Union(declared) = item {
            for field in &declared.fields.named {
                consider(field, &mut found);
            }
        } else if let syn::Item::Enum(declared) = item {
            for variant in &declared.variants {
                collect(&variant.fields, &mut found);
            }
        } else if let syn::Item::Mod(declared) = item
            && let Some((_, inner)) = declared.content.as_ref()
        {
            found.extend(suppressed_fields(inner));
        } else if let syn::Item::Fn(declared) = item {
            found.extend(suppressed_fields(nested_items(&declared.block.stmts)));
        }
    }
    found
}

/// The items one function body declares, which is where a record can be
/// declared without standing at a module's own level.
fn nested_items(statements: &[syn::Stmt]) -> impl Iterator<Item = &syn::Item> {
    statements.iter().filter_map(|statement| {
        if let syn::Stmt::Item(declared) = statement {
            Some(declared)
        } else {
            None
        }
    })
}

/// Every offending field of one field list.
fn collect(fields: &syn::Fields, into: &mut Vec<String>) {
    for field in fields {
        consider(field, into);
    }
}

/// One field, kept where it is an underscore field carrying real data.
fn consider(field: &syn::Field, into: &mut Vec<String>) {
    let Some(named) = field.ident.as_ref() else {
        return;
    };
    let spelled = named.to_string();
    if !spelled.starts_with('_') {
        return;
    }
    if !mentions_marker(&field.ty) {
        into.push(spelled);
    }
}

/// Whether one declared type mentions the type-level marker, at any depth.
///
/// A type this reader does not open contributes nothing, which is the
/// conservative direction here: it can refuse a lawful field, and it can never
/// admit one carrying data.
fn mentions_marker(declared: &syn::Type) -> bool {
    if let syn::Type::Path(typed) = declared {
        typed.path.segments.iter().any(|segment| {
            segment.ident == TYPE_LEVEL_MARKER || marker_in_arguments(&segment.arguments)
        })
    } else if let syn::Type::Reference(borrowed) = declared {
        mentions_marker(&borrowed.elem)
    } else if let syn::Type::Ptr(pointer) = declared {
        mentions_marker(&pointer.elem)
    } else if let syn::Type::Paren(parenthesized) = declared {
        mentions_marker(&parenthesized.elem)
    } else if let syn::Type::Group(grouped) = declared {
        mentions_marker(&grouped.elem)
    } else if let syn::Type::Tuple(tuple) = declared {
        tuple.elems.iter().any(mentions_marker)
    } else if let syn::Type::Array(array) = declared {
        mentions_marker(&array.elem)
    } else if let syn::Type::Slice(sliced) = declared {
        mentions_marker(&sliced.elem)
    } else {
        false
    }
}

/// Whether one segment's arguments mention the type-level marker.
fn marker_in_arguments(arguments: &syn::PathArguments) -> bool {
    if let syn::PathArguments::AngleBracketed(angled) = arguments {
        angled.args.iter().any(|argument| {
            if let syn::GenericArgument::Type(inner) = argument {
                mentions_marker(inner)
            } else {
                false
            }
        })
    } else {
        false
    }
}

/// Planted reversals for the laws whose subject is a tree rather than a text.
///
/// A fixture string cannot reach a law that reads a whole population, so these
/// are planted against a scratch root outside the repository and read through
/// the same snapshot builder a real run uses. Nothing is written inside the
/// repository — the laws that guard the tree are never proven by dirtying the
/// tree.
#[cfg(test)]
mod tests {
    use super::{
        SCANNED_TREES, check_lf_and_no_symlinks, check_no_python,
        check_underscore_fields_are_phantom, suppressed_fields,
    };
    use crate::checks::scratch::Scratch;

    /// Planted reversal: a file carrying CRLF.
    ///
    /// The symlink half of this law is NOT planted. Creating a symlink is a
    /// privileged operation on one of the supported platforms, so a fixture
    /// that planted one would pass or fail on who ran it rather than on the
    /// law. That half stands on the law's own code and on nothing executed
    /// here, and this doc line is where that is admitted rather than implied.
    #[test]
    fn a_crlf_file_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("lf-only");
        scratch.write("clean.md", "one line\nanother\n");
        assert!(check_lf_and_no_symlinks(&scratch.read()?).is_ok());

        scratch.write("drifted.md", "one line\r\nanother\r\n");
        let found = check_lf_and_no_symlinks(&scratch.read()?);
        assert!(found.is_err_and(|reason| reason.contains("CRLF") && reason.contains("drifted")));
        Ok(())
    }

    /// Planted reversal: a Python file anywhere in the tree.
    #[test]
    fn a_python_file_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("no-python");
        scratch.write("tool.rs", "fn main() {}\n");
        scratch.write("notes/readme.md", "prose\n");
        assert!(check_no_python(&scratch.read()?).is_ok());

        scratch.write("notes/helper.py", "the file's presence is the offence\n");
        let found = check_no_python(&scratch.read()?);
        assert!(found.is_err_and(|reason| reason.contains("helper.py")));
        Ok(())
    }

    /// Planted reversal: real data behind an underscore — the suppressor idiom
    /// this law exists to refuse — planted in each of the three trees the scan
    /// covers, so no tree is scanned in name only.
    #[test]
    fn an_underscore_field_carrying_data_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("underscore-fields");
        let lawful = "use core::marker::PhantomData;\n\
                      pub struct Demo {\n    _law: PhantomData<*const ()>,\n}\n";
        for tree in SCANNED_TREES {
            scratch.write(&format!("{tree}/lawful.rs"), lawful);
        }
        assert!(check_underscore_fields_are_phantom(&scratch.read()?).is_ok());

        for tree in SCANNED_TREES {
            scratch.write(
                &format!("{tree}/smuggled.rs"),
                "pub struct Demo {\n    _hidden: u64,\n}\n",
            );
            let found = check_underscore_fields_are_phantom(&scratch.read()?);
            assert!(
                found.is_err_and(|reason| reason.contains("smuggled.rs")
                    && reason.contains("underscore field without PhantomData")),
                "{tree} tree is not scanned"
            );
            scratch.remove(&format!("{tree}/smuggled.rs"));
        }
        Ok(())
    }

    /// The reader answers about FIELDS, which the line scanner it replaced
    /// could not.
    ///
    /// Planted reversal in both directions at once. A field declared across two
    /// lines is still a field, and the scanner missed it because its subject was
    /// a line. A function parameter, a local binding, and a `_field: u64`
    /// written inside a doc comment are not fields at all, and the scanner
    /// reported every one of them — which is a law refusing lawful sources, the
    /// one direction that costs an author a refusal they cannot repair by being
    /// right.
    #[test]
    fn the_reader_answers_about_fields_and_not_about_lines() {
        let declared_across_lines =
            syn::parse_file("pub struct Demo {\n    _hidden:\n        u64,\n}\n")
                .map(|file| suppressed_fields(&file.items));
        assert!(
            declared_across_lines.is_ok_and(|found| found == vec![String::from("_hidden")]),
            "a field written across two lines escaped the reader"
        );

        let not_fields = syn::parse_file(
            "/// A doc comment showing `_hidden: u64` inside it.\n\
             pub fn road(_ignored: u64) -> u64 {\n\
             \x20   let _unused: u64 = 1;\n\
             \x20   _unused\n\
             }\n\
             pub struct Lawful {\n    _law: core::marker::PhantomData<*const ()>,\n}\n",
        )
        .map(|file| suppressed_fields(&file.items));
        assert!(
            not_fields.is_ok_and(|found| found.is_empty()),
            "something that is not a field was reported as one"
        );
    }

    /// A record declared inside a function body is still a record, and its
    /// fields are still read.
    #[test]
    fn a_record_declared_inside_a_road_is_still_read() {
        let nested = syn::parse_file(
            "pub fn road() {\n    struct Hidden {\n        _smuggled: u64,\n    }\n}\n",
        )
        .map(|file| suppressed_fields(&file.items));
        assert!(
            nested.is_ok_and(|found| found == vec![String::from("_smuggled")]),
            "a record declared inside a road escaped the reader"
        );
    }
}
