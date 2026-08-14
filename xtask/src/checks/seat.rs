//! The seat law: a module named `seat` carries one record and nothing else.
//!
//! # The defect this replaces, and why it could not be checked
//!
//! Rust's privacy is MODULE-scoped. A private field is private to the module
//! its declaration landed in, and to that module's descendants — so a record
//! declared in a home's `types.rs` puts every other item in that file inside its
//! wall. Two repository laws used to stand on the other side of that fact and
//! ask, of a whole file at a time, *did anybody write a road that hands the
//! sealed value out?* Answering it means resolving types, following aliases,
//! deciding what a receiver stands for, and inferring reachability from
//! visibility and module chains. Both laws tried; between them they were wrong
//! twelve times, in twelve different Rust shapes, and every repair taught the
//! reader one more shape while leaving the thirteenth open.
//!
//! The defect is not in the readers. It is that the question is unanswerable
//! without being a compiler, and it was only ever asked because the wall was
//! drawn around a file full of unrelated code.
//!
//! So the wall moved. Each sealed record now sits in a module of its own —
//! named `seat`, for the private field it exists to hold — whose entire content
//! is that record and inherent implementations of it. What can reach the seat is
//! now a module a person reads in one screen rather than a file with dozens of
//! types in it, and `rustc` is what refuses everything outside it: `E0616` on
//! the field, `E0451` on the literal, `E0603` on a tuple constructor. Those are
//! compiler refusals, not findings.
//!
//! The stamped scope guards took the same move one step further: their module is
//! written BY `scope_guard_version!`, so no hand-written item can be inside the
//! wall at all and no law is left with anything to say about them. That is the
//! drain running downward — the compiler took the claim, and the law that
//! asserted it went.
//!
//! # What is left for a law, and why this one cannot be wrong
//!
//! One thing: the `seat` module has to STAY a module with nothing else in it. A
//! helper function added beside the record, a trait implementation, a nested
//! module, a macro invocation whose expansion nobody here can see — each one
//! quietly widens the set of code inside the wall, and the widening is invisible
//! at the declaration.
//!
//! That is a question about ITEM KINDS and IDENTIFIERS, and this reader asks
//! nothing else. It resolves no type, follows no alias, expands no macro, and
//! reads no visibility.
//!
//! `impl Foo` belongs to a `seat` module declaring `Foo` because the two
//! identifiers are spelled the same, and because Rust will not let them mean two
//! different things: a module that declares `struct Foo` cannot also import
//! another `Foo` into its type namespace — `rustc` refuses that pair outright —
//! so an UNQUALIFIED, single-segment `Foo` written inside such a module names
//! that module's own record and can name nothing else. Nothing is resolved to
//! read that; it follows from what the module declares.
//!
//! A QUALIFIED subject breaks the argument at its root, and is refused for that
//! reason rather than read. Rust admits an inherent implementation in a module
//! other than the one its type was declared in, as long as both sit in one
//! crate, so `impl crate::other::Foo` written here puts a block for somebody
//! else's record INSIDE this seat's privacy wall — free to read the local
//! record's private field, write its literal, and hand the record back — while
//! spelling a name this module never declared. The admitted spelling is
//! therefore the one whose meaning is settled by the declaration beside it: no
//! `qself`, no leading `::`, exactly one segment, that segment the record's own
//! identifier, with generic arguments permitted on it. Everything else refuses,
//! and none of it is resolved. This is a NARROWING of what the reader admits, not
//! a resurrection of the deleted readers that tried to follow a path to its
//! owner.
//!
//! # Its stated ceiling, said out loud
//!
//! **It does not decide which records must be seated.** A closed record declared
//! somewhere else, in a module named anything else, is outside this law's
//! population entirely. For the scope guards that gap is closed by the compiler
//! — a guard exists only where the stamp wrote it, and the stamp always seats it
//! — and for the services' refusal bodies it is closed by the compile-fail
//! fixtures that name each seat from outside the crate.
//!
//! **It does not read what a road hands back.** An inherent road written INSIDE
//! a seat module can still return the seat, and this law will not say so. What
//! it buys is that the set of roads to a seat is a module rather than a file: it
//! is readable, it is small, and it cannot grow sideways without this law
//! refusing.
//!
//! **A `seat` module declared as a FILE is refused rather than read.** This
//! reader judges the module where it is written; a `mod seat;` whose body lives
//! in another file would have its contents judged nowhere at all, and unknown
//! must not read as nothing to say.
//!
//! **Where a `seat` module may be WRITTEN is not restricted, so every place one
//! can be written is entered.** A module at a file's own level, one nested
//! inside another module, and one declared inside a body — a free road's, an
//! implementation's method, a trait's default — are the same wall around the
//! same record, so the walk enters all of them. What it does not enter is an
//! item written inside an EXPRESSION: a block used as a value, a closure body, a
//! match arm. `syn::Expr` is `#[non_exhaustive]`, so a reader that completed
//! itself by listing that enum's variants would be complete only until the
//! decoder gained one more — the set with no last member this repository has
//! already deleted readers for. The gap is named here rather than papered over,
//! and it closes when the one reading gains a visitor owning the whole grammar,
//! not when this law is taught one more shape.
//!
//! **A `seat` module a macro would write is outside the population.** This
//! reader expands nothing, so a module produced by an expansion is not a module
//! it can see. Inside a seat module that fails closed — a macro invocation there
//! is refused by kind — and outside one it is the stated gap above.

use crate::repository::snapshot::{MACHINE_DIRECTORY, RepositorySnapshot, TOOLING_DIRECTORY};
use crate::repository::types::CanonicalPath;

/// The module name a sealed record is declared in.
const SEAT_MODULE: &str = "seat";

/// Every module named `seat` in the machine and the services carries exactly one
/// record declaration, and beyond that only inherent implementations of that
/// record and the imports they name.
///
/// # Errors
///
/// Returns the offences one line at a time, and returns a read failure as
/// itself: a gate that cannot read its subject says so rather than reporting an
/// empty population.
pub(crate) fn check_seat_modules_carry_nothing_else(
    snapshot: &RepositorySnapshot,
) -> Result<(), String> {
    let sources = seat_sources(snapshot)?;
    let verdict = seat_verdict(&sources);

    // The denominator is DERIVED and printed on every run, because a population
    // that quietly shrank would otherwise keep this check passing while it
    // guarded less.
    println!(
        "seat modules: {} carrying one record alone / {} declared",
        verdict.closed, verdict.declared
    );
    if verdict.declared == 0 {
        return Err(String::from(
            "no `seat` module was found: this denominator cannot be empty while the sealed records \
             exist, so the reader is looking at the wrong tree",
        ));
    }
    if verdict.offenders.is_empty() {
        Ok(())
    } else {
        Err(verdict.offenders.join("; "))
    }
}

/// What the seat leg counted, and what it refuses.
#[derive(Debug)]
struct SeatVerdict {
    /// Modules named `seat` the pass entered.
    declared: usize,
    /// Those of them carrying one record and nothing but implementations of it.
    closed: usize,
    /// Every offence, one line each.
    offenders: Vec<String>,
}

/// Reads every `seat` module out of parsed trees and judges each one.
///
/// Pure over its inputs — `(canonical path, parsed tree)` pairs handed over by
/// the snapshot — so the reversals below are planted in memory and the law that
/// guards the seats is never proven by opening one. A source that did not parse
/// never reaches here: the snapshot carries it as unread, and the caller refuses
/// the whole reading rather than deriving a population one file short.
fn seat_verdict(sources: &[(&CanonicalPath, &syn::File)]) -> SeatVerdict {
    let mut verdict = SeatVerdict {
        declared: 0,
        closed: 0,
        offenders: Vec::new(),
    };
    for (path, file) in sources {
        walk(path.as_str(), &file.items, &mut verdict);
    }
    verdict
}

/// Walks one item list, judging every `seat` module it declares and descending
/// into every other item that carries one — a module's body, and the bodies a
/// road, a method, or a trait default carries.
///
/// No item kind reaches a silent `continue`: an item either declares a module
/// this law judges or is entered for the items IT carries, and an item carrying
/// none contributes an empty list rather than an exemption.
fn walk<'items>(
    path: &str,
    items: impl IntoIterator<Item = &'items syn::Item>,
    verdict: &mut SeatVerdict,
) {
    for item in items {
        let syn::Item::Mod(module) = item else {
            walk(path, beneath(item), verdict);
            continue;
        };
        let named_seat = module.ident == SEAT_MODULE;
        let Some((_, inner)) = &module.content else {
            if named_seat {
                verdict.declared = verdict.declared.saturating_add(1);
                verdict.offenders.push(format!(
                    "{path}: `mod {SEAT_MODULE};` carries its body in another file, so what the \
                     seat module holds is judged nowhere; a seat module is written where it is \
                     declared"
                ));
            }
            continue;
        };
        if named_seat {
            verdict.declared = verdict.declared.saturating_add(1);
            judge(path, inner, verdict);
        }
        walk(path, inner, verdict);
    }
}

/// The items one item carries in a body of its own.
///
/// A module can be declared inside a body, and a body is one of exactly three
/// things: a free road's block, an implementation method's block, or a trait
/// method's default block. Each is entered, because a `seat` module written in
/// one is the same wall around the same record as one written at a file's own
/// level. An item carrying no body carries no items, which is a fact rather than
/// a skip.
fn beneath(item: &syn::Item) -> Vec<&syn::Item> {
    if let syn::Item::Fn(declared) = item {
        return declared_items(&declared.block.stmts);
    }
    if let syn::Item::Impl(declared) = item {
        let mut found = Vec::new();
        for member in &declared.items {
            if let syn::ImplItem::Fn(road) = member {
                found.extend(declared_items(&road.block.stmts));
            }
        }
        return found;
    }
    if let syn::Item::Trait(declared) = item {
        let mut found = Vec::new();
        for member in &declared.items {
            if let syn::TraitItem::Fn(road) = member
                && let Some(body) = road.default.as_ref()
            {
                found.extend(declared_items(&body.stmts));
            }
        }
        return found;
    }
    Vec::new()
}

/// The items one block's statements declare.
fn declared_items(statements: &[syn::Stmt]) -> Vec<&syn::Item> {
    statements
        .iter()
        .filter_map(|statement| {
            if let syn::Stmt::Item(declared) = statement {
                Some(declared)
            } else {
                None
            }
        })
        .collect()
}

/// Judges the items of one `seat` module.
fn judge(path: &str, items: &[syn::Item], verdict: &mut SeatVerdict) {
    let opened = verdict.offenders.len();
    let mut records: Vec<String> = Vec::new();
    for item in items {
        if let syn::Item::Struct(declared) = item {
            records.push(declared.ident.to_string());
        }
    }
    match records.len() {
        1 => {}
        0 => verdict.offenders.push(format!(
            "{path}: a `{SEAT_MODULE}` module declares no record at all, so it is a wall drawn \
             around nothing"
        )),
        several => verdict.offenders.push(format!(
            "{path}: a `{SEAT_MODULE}` module declares {several} records, so each of them is \
             inside the other's wall and neither seat is a module a reader can read alone"
        )),
    }
    for item in items {
        judge_item(path, item, &records, verdict);
    }
    if verdict.offenders.len() == opened {
        verdict.closed = verdict.closed.saturating_add(1);
    }
}

/// Judges one item of a `seat` module against the records it declares.
///
/// Three kinds stand: the record declarations themselves, the imports they name,
/// and inherent implementations whose subject is spelled like one of the
/// records. Everything else is refused BY KIND, which is why this reader has
/// nothing to resolve.
fn judge_item(path: &str, item: &syn::Item, records: &[String], verdict: &mut SeatVerdict) {
    if matches!(*item, syn::Item::Struct(_) | syn::Item::Use(_)) {
        return;
    }
    if let syn::Item::Impl(declared) = item {
        judge_implementation(path, declared, records, verdict);
        return;
    }
    verdict.offenders.push(format!(
        "{path}: a `{SEAT_MODULE}` module carries {}, and a seat module carries its one record, the \
         imports that record names, and inherent implementations of it — nothing else, because \
         everything written inside the module is inside the seat's wall",
        described(item)
    ));
}

/// Judges one implementation written inside a `seat` module.
fn judge_implementation(
    path: &str,
    declared: &syn::ItemImpl,
    records: &[String],
    verdict: &mut SeatVerdict,
) {
    if declared.trait_.is_some() {
        verdict.offenders.push(format!(
            "{path}: a `{SEAT_MODULE}` module carries a trait implementation, which hands the seat \
             out through a contract's own members rather than through a road with a name; a trait \
             implementation that does not need the seat belongs outside the module, and one that \
             does is the road this wall exists to make readable"
        ));
        return;
    }
    match subject_of(&declared.self_ty) {
        ImplementationSubject::Declared(subject) => {
            if !records.contains(&subject) {
                verdict.offenders.push(format!(
                    "{path}: a `{SEAT_MODULE}` module carries an implementation of `{subject}`, \
                     which is not the record it declares; a seat module is the wall around ONE \
                     record and an implementation of anything else is other code standing inside it"
                ));
            }
        }
        ImplementationSubject::Qualified(spelled) => verdict.offenders.push(format!(
            "{path}: a `{SEAT_MODULE}` module carries an implementation of `{spelled}`, a \
             QUALIFIED path. Rust admits an inherent implementation in a module other than the one \
             its type was declared in, so this block sits inside the seat's privacy wall — able to \
             read the record's private field, write its literal, and hand the record back — while \
             naming a type declared somewhere else; a seat module's implementations name its one \
             record unqualified and in one segment, because that is the only spelling settled by \
             the declaration beside it"
        )),
        ImplementationSubject::Unnamed => verdict.offenders.push(format!(
            "{path}: a `{SEAT_MODULE}` module carries an implementation whose subject is not a \
             plain name, so whether it is an implementation of this module's record is unknown \
             rather than yes"
        )),
    }
}

/// What one implementation inside a `seat` module names as its subject.
///
/// Three states, and only the first can be this module's record. The second is
/// the one this reader used to collapse into the first by keeping a path's last
/// segment: `impl crate::other::Foo` inside a module declaring `Foo` reduced to
/// `Foo` and was admitted as a road to the local record, while it was a road to
/// somebody else's record standing inside the local record's wall.
#[derive(Debug)]
enum ImplementationSubject {
    /// An unqualified, single-segment path. A module declaring `struct Foo`
    /// cannot import another `Foo` into its type namespace, so inside such a
    /// module this spelling names that module's own declaration.
    Declared(String),
    /// A path reaching out of the module — a qualified self type, a leading
    /// `::`, or more than one segment — carried as spelled, for the refusal.
    Qualified(String),
    /// Not a path at all.
    Unnamed,
}

/// What one implementation's self type names, classified.
fn subject_of(declared: &syn::Type) -> ImplementationSubject {
    let syn::Type::Path(typed) = declared else {
        return ImplementationSubject::Unnamed;
    };
    if typed.qself.is_some() || typed.path.leading_colon.is_some() {
        return ImplementationSubject::Qualified(spelling_of(&typed.path));
    }
    let mut segments = typed.path.segments.iter();
    match (segments.next(), segments.next()) {
        (Some(only), None) => ImplementationSubject::Declared(only.ident.to_string()),
        (Some(_) | None, _) => ImplementationSubject::Qualified(spelling_of(&typed.path)),
    }
}

/// How one path is spelled, for a refusal that has to name what it read.
///
/// Segment identifiers joined by `::`, with the leading `::` kept where the path
/// carries one. Generic arguments are left out: what the refusal is about is the
/// route, and a route is its segments.
fn spelling_of(path: &syn::Path) -> String {
    let mut spelled = String::new();
    if path.leading_colon.is_some() {
        spelled.push_str("::");
    }
    for (position, segment) in path.segments.iter().enumerate() {
        if position > 0 {
            spelled.push_str("::");
        }
        spelled.push_str(&segment.ident.to_string());
    }
    spelled
}

/// What one refused item is, in the words its own declaration uses.
///
/// Written as an `if let` chain rather than a match because `syn::Item` is
/// `non_exhaustive`: the kinds this reader has a word for are named, and every
/// other kind falls through to the unrecognized description — which still
/// refuses, because the verdict was already decided by the caller and this
/// function only says what the item is.
fn described(item: &syn::Item) -> &'static str {
    if matches!(*item, syn::Item::Fn(_)) {
        return "a free function";
    }
    if matches!(*item, syn::Item::Mod(_)) {
        return "a nested module";
    }
    if matches!(*item, syn::Item::Enum(_)) {
        return "a second record, spelled as an enum";
    }
    if matches!(*item, syn::Item::Trait(_) | syn::Item::TraitAlias(_)) {
        return "a trait declaration";
    }
    if matches!(*item, syn::Item::Type(_)) {
        return "a type alias";
    }
    if matches!(*item, syn::Item::Const(_)) {
        return "a constant";
    }
    if matches!(*item, syn::Item::Static(_)) {
        return "a static";
    }
    if matches!(*item, syn::Item::Union(_)) {
        return "a union";
    }
    if let syn::Item::Macro(declared) = item {
        return if declared.ident.is_some() {
            "a macro definition"
        } else {
            "a macro invocation, whose expansion this reader cannot see"
        };
    }
    if matches!(*item, syn::Item::ExternCrate(_)) {
        return "an extern-crate declaration";
    }
    if matches!(*item, syn::Item::ForeignMod(_)) {
        return "a foreign block";
    }
    "an item this reader has no name for"
}

/// Every parsed source the population is derived from: the machine's own
/// sources and the services'.
///
/// Taken from the one reading. A source the snapshot could not parse refuses
/// the whole law rather than leaving the population one file short — a
/// denominator that shrank in silence is the single failure a derived
/// denominator exists to prevent.
///
/// No proof surface is excluded. A `seat` module on a proof surface is a seat
/// module, and it is judged: this law reads item kinds rather than semantic
/// roles, so a demonstration seat costs the denominator one honest entry rather
/// than inflating it with a role nothing ships.
fn seat_sources(
    snapshot: &RepositorySnapshot,
) -> Result<Vec<(&CanonicalPath, &syn::File)>, String> {
    snapshot
        .rust()
        .parsed_under(&[MACHINE_DIRECTORY, TOOLING_DIRECTORY])
}

/// Planted reversals for the seat law, and the real tree judged by it.
///
/// Every leg is pure over `(path, text)` pairs, so a reversal is a fixture held
/// in memory: the law that guards the seats is never proven by opening one. The
/// test that reads the real tree is named `the_real_…` and states what it found
/// rather than what it hoped for.
#[cfg(test)]
mod tests {
    use super::{SeatVerdict, seat_sources, seat_verdict as verdict_of_trees};
    use crate::repository::snapshot::repository_snapshot;
    use crate::repository::types::CanonicalPath;

    /// One synthetic source carrying one `seat` module with the given body.
    fn seat(body: &str) -> Vec<(String, String)> {
        vec![(
            String::from("macros/macroc/src/home/type_guard.rs"),
            format!("pub use seat::DemoRefusal;\n\nmod seat {{\n{body}}}\n"),
        )]
    }

    /// The verdict over fixture source TEXT.
    ///
    /// The law itself is handed trees the snapshot already parsed, so a source
    /// it could not read never reaches it. A fixture is text, so this adapter
    /// parses one and reports a fixture that does not parse exactly as the
    /// reading reports a source it could not read — which keeps the reversal
    /// below about a hole in the population rather than about who parses.
    fn seat_verdict(sources: &[(String, String)]) -> SeatVerdict {
        let mut parsed = Vec::new();
        let mut unparsable = Vec::new();
        for (path, text) in sources {
            match syn::parse_file(text) {
                Ok(file) => parsed.push((CanonicalPath::spelled(path), file)),
                Err(error) => unparsable.push(format!(
                    "{path}: this file is not parseable Rust, so the population derived from it \
                     is unknown rather than empty: {error}"
                )),
            }
        }
        let trees: Vec<(&CanonicalPath, &syn::File)> =
            parsed.iter().map(|(path, file)| (path, file)).collect();
        let mut verdict = verdict_of_trees(&trees);
        verdict.offenders.splice(0..0, unparsable);
        verdict
    }

    /// The record and its one crate-internal mint: the shape every seat module
    /// in the tree is written in.
    const LAWFUL_BODY: &str = "\
    use super::super::DemoIssue;\n\
    use threadpak::refusal::AdmittedPrefix;\n\
\n\
    /// The demo refusal family body.\n\
    pub struct DemoRefusal {\n\
        body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
    }\n\
\n\
    impl DemoRefusal {\n\
        pub(super) fn established(issue: DemoIssue) -> Self {\n\
            Self { body: AdmittedPrefix::carrying_one(issue) }\n\
        }\n\
    }\n";

    /// Whether some offence says the named thing.
    fn says(verdict: &SeatVerdict, fragment: &str) -> bool {
        verdict
            .offenders
            .iter()
            .any(|offence| offence.contains(fragment))
    }

    /// The positive control: one record, its imports, and one inherent
    /// implementation of it. A check that flagged everything would satisfy every
    /// reversal below and be worthless.
    #[test]
    fn a_seat_module_carrying_one_record_and_its_roads_is_lawful() {
        let verdict = seat_verdict(&seat(LAWFUL_BODY));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.closed, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Planted reversal: a hand-written free function beside the record. It
    /// reaches the seat exactly as the record's own roads do, and it is other
    /// code standing inside the wall.
    #[test]
    fn a_free_function_in_a_seat_module_is_a_violation() {
        let verdict = seat_verdict(&seat(&format!(
            "{LAWFUL_BODY}\n\
             \x20   fn laundered(held: &DemoRefusal) -> AdmittedPrefix<DemoIssue, DemoIssueLimit> {{\n\
             \x20       held.body.clone()\n\
             \x20   }}\n"
        )));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.closed, 0);
        assert!(says(&verdict, "a free function"), "{:?}", verdict.offenders);
    }

    /// Planted reversal: a nested module. The seat's privacy does not exclude
    /// descendants, so a module written inside the wall constructs the record as
    /// freely as the roads beside it do.
    #[test]
    fn a_nested_module_in_a_seat_module_is_a_violation() {
        let verdict = seat_verdict(&seat(&format!(
            "{LAWFUL_BODY}\n\
             \x20   mod inner {{\n\
             \x20       pub fn reach() {{}}\n\
             \x20   }}\n"
        )));
        assert_eq!(verdict.closed, 0);
        assert!(says(&verdict, "a nested module"), "{:?}", verdict.offenders);
    }

    /// Planted reversal: a trait implementation, which hands the seat out
    /// through a contract's members and never through a road with a name.
    #[test]
    fn a_trait_implementation_in_a_seat_module_is_a_violation() {
        let verdict = seat_verdict(&seat(&format!(
            "{LAWFUL_BODY}\n\
             \x20   impl core::ops::Deref for DemoRefusal {{\n\
             \x20       type Target = AdmittedPrefix<DemoIssue, DemoIssueLimit>;\n\
             \x20       fn deref(&self) -> &Self::Target {{ &self.body }}\n\
             \x20   }}\n"
        )));
        assert_eq!(verdict.closed, 0);
        assert!(
            says(&verdict, "a trait implementation"),
            "{:?}",
            verdict.offenders
        );
    }

    /// Planted reversal: a macro invocation. Whether its expansion reaches the
    /// seat is unknown, and unknown must not read as no.
    #[test]
    fn a_macro_invocation_in_a_seat_module_is_a_violation() {
        let verdict = seat_verdict(&seat(&format!("{LAWFUL_BODY}\n\x20   launder!();\n")));
        assert_eq!(verdict.closed, 0);
        assert!(
            says(&verdict, "a macro invocation"),
            "{:?}",
            verdict.offenders
        );
    }

    /// Planted reversal: an implementation of something else. A seat module is
    /// the wall around ONE record, and a second subject inside it is a second
    /// thing that can reach the seat.
    #[test]
    fn an_implementation_of_another_subject_is_a_violation() {
        let verdict = seat_verdict(&seat(&format!(
            "{LAWFUL_BODY}\n\
             \x20   impl DemoIssue {{\n\
             \x20       pub fn into_refusal(self) -> DemoRefusal {{ DemoRefusal::established(self) }}\n\
             \x20   }}\n"
        )));
        assert_eq!(verdict.closed, 0);
        assert!(says(&verdict, "`DemoIssue`"), "{:?}", verdict.offenders);
    }

    /// Planted reversal: the privacy-wall bypass a last-segment reading
    /// admitted.
    ///
    /// Rust permits an inherent implementation in a module other than the one
    /// its type was declared in, provided both are in one crate. So each of
    /// these blocks really does sit inside the local record's wall — it can read
    /// the private field, write the literal, and hand the record out — while its
    /// subject was declared somewhere else entirely. A reader keeping the last
    /// path segment saw `DemoRefusal` in every one of them and passed.
    ///
    /// Four spellings, because the qualification hides in four places: a
    /// crate-rooted path, a `super`-rooted one, a bare leading `::`, and a
    /// qualified self type. None of them is resolved — each is refused for being
    /// a spelling whose meaning the declaration beside it does not settle.
    #[test]
    fn an_implementation_of_a_qualified_subject_is_a_violation() {
        for spelling in [
            "crate::other::DemoRefusal",
            "super::super::other::DemoRefusal",
            "::demo::DemoRefusal",
            "<DemoIssue as Carrier>::DemoRefusal",
        ] {
            let verdict = seat_verdict(&seat(&format!(
                "{LAWFUL_BODY}\n\
                 \x20   impl {spelling} {{\n\
                 \x20       pub fn road(&self, local: &DemoRefusal) -> DemoRefusal {{\n\
                 \x20           DemoRefusal {{ body: local.body.clone() }}\n\
                 \x20       }}\n\
                 \x20   }}\n"
            )));
            assert_eq!(verdict.declared, 1, "{spelling}");
            assert_eq!(verdict.closed, 0, "{spelling}: {:?}", verdict.offenders);
            assert!(
                says(&verdict, "a QUALIFIED path"),
                "{spelling}: {:?}",
                verdict.offenders
            );
        }
    }

    /// The narrowing refuses the qualification and nothing else: the record's
    /// own generic arguments stay lawful on the one segment that carries them.
    #[test]
    fn generic_arguments_on_the_records_own_name_stay_lawful() {
        let verdict = seat_verdict(&seat(
            "    use threadpak::refusal::AdmittedPrefix;\n\
             \x20   pub struct DemoRefusal<F> {\n\
             \x20       body: AdmittedPrefix<F, DemoIssueLimit>,\n\
             \x20   }\n\
             \n\
             \x20   impl<F> DemoRefusal<F> {\n\
             \x20       pub(super) fn established(body: AdmittedPrefix<F, DemoIssueLimit>) -> Self {\n\
             \x20           Self { body }\n\
             \x20       }\n\
             \x20   }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.closed, 1, "{:?}", verdict.offenders);
    }

    /// A seat module written inside a BODY is still in the population.
    ///
    /// Planted reversal in the three places a body can be: a free road, an
    /// implementation's method, and a trait's default. The walk used to enter
    /// item-level modules only, so a wall built inside any of these stood
    /// outside the law that guards walls.
    #[test]
    fn a_seat_module_written_inside_a_body_is_still_in_the_population() {
        let offending = "mod seat {\n    pub struct Held { body: u64 }\n    fn reach() {}\n}\n";
        for surrounding in [
            format!("pub fn road() {{\n{offending}}}\n"),
            format!(
                "struct Subject;\nimpl Subject {{\n    fn road(&self) {{\n{offending}    }}\n}}\n"
            ),
            format!("trait Contract {{\n    fn road(&self) {{\n{offending}    }}\n}}\n"),
        ] {
            let verdict = seat_verdict(&[(
                String::from("macros/macroc/src/home/type_guard.rs"),
                surrounding.clone(),
            )]);
            assert_eq!(verdict.declared, 1, "{surrounding}");
            assert!(
                says(&verdict, "a free function"),
                "{surrounding} -> {:?}",
                verdict.offenders
            );
        }
    }

    /// A seat module declaring two records puts each inside the other's wall.
    #[test]
    fn two_records_in_one_seat_module_is_a_violation() {
        let verdict = seat_verdict(&seat(&format!(
            "{LAWFUL_BODY}\n\x20   pub struct OtherRefusal {{ body: u8 }}\n"
        )));
        assert_eq!(verdict.closed, 0);
        assert!(says(&verdict, "2 records"), "{:?}", verdict.offenders);
    }

    /// A seat module declaring no record is a wall around nothing.
    #[test]
    fn a_seat_module_with_no_record_is_a_violation() {
        let verdict = seat_verdict(&seat("    use super::super::DemoIssue;\n"));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.closed, 0);
        assert!(
            says(&verdict, "declares no record at all"),
            "{:?}",
            verdict.offenders
        );
    }

    /// A `seat` module whose body lives in another file is judged nowhere, so it
    /// is refused rather than passed over.
    #[test]
    fn a_seat_module_carried_in_another_file_is_an_offence() {
        let verdict = seat_verdict(&[(
            String::from("macros/macroc/src/home/type_guard.rs"),
            String::from("mod seat;\n"),
        )]);
        assert_eq!(verdict.declared, 1);
        assert!(says(&verdict, "judged nowhere"), "{:?}", verdict.offenders);
    }

    /// A seat module nested inside another module is still read: the population
    /// is every `seat` module the tree declares, at whatever depth.
    #[test]
    fn a_seat_module_nested_deeper_is_still_in_the_population() {
        let verdict = seat_verdict(&[(
            String::from("src/00_home/types.rs"),
            format!("mod outer {{\n    mod seat {{\n{LAWFUL_BODY}    }}\n}}\n"),
        )]);
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.closed, 1, "{:?}", verdict.offenders);
    }

    /// Every module that is NOT named `seat` is outside this law entirely: it
    /// reads a name and nothing else, and a file full of ordinary code is not
    /// its subject.
    #[test]
    fn a_module_that_is_not_a_seat_is_not_this_laws_subject() {
        let verdict = seat_verdict(&[(
            String::from("src/00_home/types.rs"),
            String::from(
                "mod guard {\n    fn helper() {}\n    pub struct One;\n    pub struct Two;\n}\n",
            ),
        )]);
        assert_eq!(verdict.declared, 0);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A source this reader cannot parse is a hole in the population, and it is
    /// reported as one.
    #[test]
    fn an_unparsable_source_is_an_offence_rather_than_an_absence() {
        let verdict = seat_verdict(&[(
            String::from("src/00_home/types.rs"),
            String::from("mod seat {\n"),
        )]);
        assert_eq!(verdict.declared, 0);
        assert!(
            says(&verdict, "not parseable Rust"),
            "{:?}",
            verdict.offenders
        );
    }

    /// The real tree holds: every `seat` module it declares carries one record
    /// and nothing else, and the derived population is real rather than empty.
    ///
    /// The count is asserted as a RELATION and never as a number. A test naming
    /// seven would be the hand-maintained inventory this repository bans, moved
    /// one file over; the run prints the numbers, and the relation is what has
    /// to hold.
    #[test]
    fn the_real_tree_carries_nothing_else_in_a_seat_module() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let sources = seat_sources(snapshot)?;
        let verdict = verdict_of_trees(&sources);
        assert!(verdict.offenders.is_empty(), "{verdict:?}");
        assert!(
            verdict.declared > 0,
            "no `seat` module found in the real tree: {verdict:?}"
        );
        assert_eq!(
            verdict.closed, verdict.declared,
            "the real tree carries a `seat` module with something else in it"
        );
        Ok(())
    }
}
