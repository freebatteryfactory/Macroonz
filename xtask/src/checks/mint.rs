//! The mint join: a refusal body whose every seat is private is handed back by
//! no road reachable from outside the crate that raises it.
//!
//! A private seat closes the LITERAL. It does not close the ROAD. A function
//! that takes an issue and hands back a refusal is the loading dock behind that
//! fence: any holder of an issue mints a body no pass established, and any
//! holder of a borrowed body clones the issues out and reseats them through the
//! same road. The record either produces is indistinguishable from one a seam
//! returned, which is the whole defect the private seat was supposed to end.
//!
//! The compile-time half of that claim is
//! `testpak/tests/compile-fail/a-services-refusal-minted-outside-its-plane.rs`,
//! which names each road and refuses it with `E0624`. This law is the half a
//! fixture cannot state: a fixture names the roads that EXIST, so a family added
//! tomorrow with a public mint leaves it compiling and passing, and the sentence
//! above it goes false with nothing failing. A universal claim needs a universal
//! fixture or a derived population, and an enumeration is neither. Here the
//! population is DERIVED — every closed record the subsystem declares that some
//! road refuses with, and every road that hands one back, read off the sources —
//! so a family added without a crate-internal mint is caught by the derivation
//! rather than by anybody remembering to extend a list.
//!
//! # The two facts, and why each is the one it is
//!
//! **A refusal is what a road refuses with.** The reader takes the error
//! position of every `Result` the subsystem returns. That is the subsystem's own
//! statement about which of its types are refusals, written where the refusing
//! happens, and it needs no marker anybody has to remember to apply.
//!
//! **A body is closed when no seat is public.** A record with a public seat can
//! be spelled as a literal from outside whatever its roads say, so the mint
//! question is not the question about it — the seat is, and
//! `macroc.a-refusal-body-seat-cannot-be-written-from-outside` is where that one
//! is asked. This law is about the records where the seat is already shut.
//!
//! **A mint hands a body back as its ANSWER.** A seam that refuses returns
//! `Result<_, Body>` and is meant to be public: that is the caller receiving the
//! refusal the seam raised, which is what the type is FOR. A road whose answer is
//! the body — bare, or inside the success position of a `Result` or an `Option` —
//! produces one, and that is the road this law is about. A road taking a
//! receiver is excluded on the same reading: it is handed a body and hands one
//! back, so it is a copy of something a pass already established rather than a
//! new one minted from parts.
//!
//! # The population is read from a parse, not from lines
//!
//! The reader is `syn`, for the reason the coupling law's is. What the parse
//! establishes that a scan cannot: that an item IS a record declaration, that a
//! FIELD is public rather than a `pub` somewhere on the line, that a function's
//! declared visibility is `pub` rather than `pub(crate)`, that a return type's
//! error position is the second type argument of a `Result` rather than the
//! second word after a comma, and that a receiver is a receiver. Those are
//! questions about items, members and paths, and answering them off text means
//! writing a Rust parser by hand inside a check.
//!
//! # What this reader does and does not resolve
//!
//! Its subject is the metaprogramming subsystem and not the machine. Band 00's
//! own report package carries private seats and a PUBLIC mint on purpose — that
//! mint is the road the services reach for — so a law reading the machine on
//! these terms would refuse the thing it depends on. The machine's bodies are
//! guarded by their own laws and this one says nothing about them.
//!
//! A record is resolved by its declared NAME across the whole subsystem rather
//! than inside a home. That is the strict direction and it is deliberate: a road
//! anywhere in the subsystem that hands back a body is a road, wherever the body
//! was declared, and scoping the join to a home would leave exactly the
//! cross-home mint this law exists to refuse outside it. Two homes declaring one
//! name are read as one subject, so a road for either is judged against both —
//! the offence names both paths, and the repair is a rename rather than a looser
//! reading.
//!
//! It does not compile anything. A path is read by its LAST SEGMENT, so
//! `ProjectionPlanning` and `refusal::ProjectionPlanning` are one name here and a
//! type alias to either is neither. It does not evaluate `cfg`: a member written
//! under one is read as declared. It does not expand macros, so a record or a
//! road assembled by one is outside this law — and this law does not pretend
//! otherwise.
//!
//! `pub` is read as reachable. For an inherent road that is exact: an inherent
//! implementation is not name-resolved through modules, so a `pub` road on a
//! public type is reachable from outside the crate however private the module
//! that writes it. For a free function it is strict rather than exact — a `pub`
//! free road inside a private module is not in fact reachable — and the strict
//! direction is the one this reading may fail in: a road that hands a refusal
//! body back and is spelled `pub` states an intent to hand it out, and the
//! narrower spelling is one word away.

use std::fs;
use std::path::Path;

use crate::repository::walk::{TOOLING_DIRECTORY, relative_slash_path, visit_files};

/// The proof surface, excluded from the population by name.
///
/// The services' `laws.rs` declares demonstration families whose whole content
/// is a pair of constants — they exist so the admission algebra has something to
/// refuse, and counting them would put a fixture in a denominator about the
/// subsystem. Excluded for the reason and by the name the coupling law excludes
/// it.
const PROOF_SURFACE: &str = "macros/macroc/src/laws.rs";

/// The wrappers a road may hand its answer back inside. The SUCCESS position of
/// each, and never the error one.
const ANSWER_WRAPPERS: [&str; 2] = ["Result", "Option"];

/// The return a road refuses through.
const REFUSING_RETURN: &str = "Result";

/// The spelling a road inside an implementation names its own type by.
const OWN_TYPE: &str = "Self";

/// How many wrappers deep this reader follows an answer.
///
/// Stated rather than unbounded: a road wrapping its answer deeper than this is
/// outside the reading, and saying how deep is what makes that a ceiling rather
/// than a surprise.
const ANSWER_DEPTH: usize = 4;

/// Every refusal body the metaprogramming subsystem declares with every seat
/// private is handed back by no road reachable from outside the crate.
///
/// # Errors
///
/// Returns the offences one line at a time, and returns a read failure as
/// itself: a gate that cannot read its subject says so rather than reporting an
/// empty population.
pub(crate) fn check_refusal_mints_are_inside_the_plane(root: &Path) -> Result<(), String> {
    let sources = services_sources(root)?;
    let verdict = mint_verdict(&sources);

    // The denominators are DERIVED and printed on every run, because a
    // population that quietly shrank would otherwise keep this check passing
    // while it guarded less.
    println!(
        "refusal mints: {} roads / {} closed refusal bodies",
        verdict.roads, verdict.bodies
    );
    if verdict.bodies == 0 {
        return Err(String::from(
            "no closed refusal body was found in the metaprogramming subsystem: this denominator \
             cannot be empty while the services refuse, so the reader is looking at the wrong tree",
        ));
    }
    if verdict.offenders.is_empty() {
        Ok(())
    } else {
        Err(verdict.offenders.join("; "))
    }
}

/// What the mint leg counted, and what it refuses.
#[derive(Debug)]
struct MintVerdict {
    /// Closed records some road refuses with: the refusal bodies.
    bodies: usize,
    /// Roads that hand one of those bodies back.
    roads: usize,
    /// Roads reachable from outside the crate, and bodies no road produces, one
    /// offence each.
    offenders: Vec<String>,
}

/// One record the subsystem declares whose every seat is private.
struct ClosedRecord {
    /// The repository-relative path that declares it.
    path: String,
    /// Its declared name.
    name: String,
}

/// One road that hands a value back as its answer.
struct Road {
    /// The repository-relative path that declares it.
    path: String,
    /// Its declared name.
    name: String,
    /// The type it answers with, by last path segment, with `Self` resolved to
    /// the type the enclosing implementation is for.
    answer: String,
    /// How far it reaches.
    reach: Reach,
}

/// How far one road reaches — a named pair rather than a flag, because "true"
/// at a call site says nothing about which direction it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reach {
    /// Out of the crate: the road is spelled `pub`, or it is a trait road, which
    /// is as reachable as the trait it implements.
    OutsideTheCrate,
    /// No further than the crate: `pub(crate)`, `pub(in …)`, `pub(super)`, and
    /// the absence of any spelling at all.
    InsideTheCrate,
}

/// Everything one pass over the sources read.
struct Reading {
    /// Every record declared with no public seat.
    closed: Vec<ClosedRecord>,
    /// Every type name a road in the subsystem refuses with.
    refused: Vec<String>,
    /// Every road that hands a value back as its answer.
    roads: Vec<Road>,
    /// Sources that are not parseable Rust, one offence each. Never a skip: a
    /// file this reader could not read is a hole in the population, and a hole
    /// reported as nothing is the defect this whole leg is about.
    unparsable: Vec<String>,
}

/// Reads the records, the refusals and the roads out of source text and judges
/// each body.
///
/// Pure over its inputs — `(repository-relative path, source text)` pairs — so
/// the reversals below are planted in memory and the law that guards the tree is
/// never proven by opening a seat in one.
fn mint_verdict(sources: &[(String, String)]) -> MintVerdict {
    let reading = read_sources(sources);
    let mut verdict = MintVerdict {
        bodies: 0,
        roads: 0,
        offenders: reading.unparsable,
    };
    for record in &reading.closed {
        let name = &record.name;
        if !reading.refused.iter().any(|refused| refused == name) {
            continue;
        }
        verdict.bodies = verdict.bodies.saturating_add(1);
        let declared = &record.path;
        let roads: Vec<&Road> = reading
            .roads
            .iter()
            .filter(|road| &road.answer == name)
            .collect();
        if roads.is_empty() {
            verdict.offenders.push(format!(
                "{declared}: {name} is refused with and every seat it declares is private, and no \
                 road in the subsystem hands one back; this law's numerator over it is empty, \
                 which guards nothing while reading as coverage"
            ));
            continue;
        }
        for road in roads {
            verdict.roads = verdict.roads.saturating_add(1);
            if matches!(road.reach, Reach::OutsideTheCrate) {
                let at = &road.path;
                let spelled = &road.name;
                verdict.offenders.push(format!(
                    "{at}: {name}::{spelled} hands back {name}, whose every seat {declared} \
                     declares is private, and it is reachable from outside the crate; a private \
                     seat closes the literal and a public road is the loading dock behind it, \
                     because any holder of an issue mints a body no pass established"
                ));
            }
        }
    }
    verdict
}

/// Parses every source and reads the records, the refusals, the roads and the
/// failures out of the trees.
fn read_sources(sources: &[(String, String)]) -> Reading {
    let mut reading = Reading {
        closed: Vec::new(),
        refused: Vec::new(),
        roads: Vec::new(),
        unparsable: Vec::new(),
    };
    for (path, text) in sources {
        match syn::parse_file(text) {
            Ok(file) => read_module(path, &file.items, &mut reading),
            Err(error) => reading.unparsable.push(format!(
                "{path}: this file is not parseable Rust, so the population derived from it is \
                 unknown rather than empty: {error}"
            )),
        }
    }
    reading
}

/// Reads one module's items, then every inline module inside it.
///
/// Written as an `if let` chain rather than a match because `syn::Item` is
/// `non_exhaustive`: the items this reading has a question about are named, and
/// every other item is passed over without a wildcard arm standing in for a set
/// no crate outside `syn` can enumerate.
fn read_module(path: &str, items: &[syn::Item], reading: &mut Reading) {
    for item in items {
        if let syn::Item::Struct(declared) = item {
            if is_closed(declared) {
                reading.closed.push(ClosedRecord {
                    path: path.to_string(),
                    name: declared.ident.to_string(),
                });
            }
        } else if let syn::Item::Fn(declared) = item {
            read_signature(path, &declared.sig, reach_of(&declared.vis), None, reading);
        } else if let syn::Item::Impl(declared) = item {
            read_implementation(path, declared, reading);
        } else if let syn::Item::Trait(declared) = item {
            read_contract(path, declared, reading);
        } else if let syn::Item::Mod(module) = item
            && let Some((_, inner)) = &module.content
        {
            read_module(path, inner, reading);
        }
    }
}

/// Reads every road one implementation declares.
///
/// A road in a TRAIT implementation states no visibility of its own — it is as
/// reachable as the trait — so this reading takes it as reachable rather than as
/// the `Visibility::Inherited` `syn` hands back, which would read every trait
/// road in the subsystem as closed.
fn read_implementation(path: &str, declared: &syn::ItemImpl, reading: &mut Reading) {
    let own = head_of(&declared.self_ty);
    let implements_a_trait = declared.trait_.is_some();
    for member in &declared.items {
        if let syn::ImplItem::Fn(road) = member {
            let reach = if implements_a_trait {
                Reach::OutsideTheCrate
            } else {
                reach_of(&road.vis)
            };
            read_signature(path, &road.sig, reach, own.as_deref(), reading);
        }
    }
}

/// Reads every road one contract declares.
///
/// A contract's road states no visibility of its own and is as reachable as the
/// contract, so the contract's own spelling is what this reading takes.
fn read_contract(path: &str, declared: &syn::ItemTrait, reading: &mut Reading) {
    let reach = reach_of(&declared.vis);
    for member in &declared.items {
        if let syn::TraitItem::Fn(road) = member {
            read_signature(path, &road.sig, reach, None, reading);
        }
    }
}

/// How far one declared visibility reaches.
///
/// `pub` and nothing else. `pub(crate)`, `pub(in …)` and `pub(super)` arrive as
/// `syn::Visibility::Restricted` and reach no further than the crate, which is
/// exactly the distinction this law turns on.
fn reach_of(declared: &syn::Visibility) -> Reach {
    if matches!(*declared, syn::Visibility::Public(_)) {
        Reach::OutsideTheCrate
    } else {
        Reach::InsideTheCrate
    }
}

/// Reads both legs off one signature: what it refuses with, and what it hands
/// back.
///
/// The refusing leg is read first and for every road, receiver or not: a seam
/// that refuses is usually a method, and reading only receiver-free signatures
/// would leave most of the subsystem's refusals unnamed and the population
/// empty.
fn read_signature(
    path: &str,
    sig: &syn::Signature,
    reach: Reach,
    own: Option<&str>,
    reading: &mut Reading,
) {
    if let Some(refused) = refusal_of(&sig.output) {
        reading.refused.push(resolved(refused, own));
    }
    if takes_a_receiver(sig) {
        return;
    }
    if let Some(answer) = answer_of(&sig.output) {
        reading.roads.push(Road {
            path: path.to_string(),
            name: sig.ident.to_string(),
            answer: resolved(answer, own),
            reach,
        });
    }
}

/// One record declared with at least one seat and no public seat.
///
/// A record with no seat at all is not a body: there is nothing a literal could
/// write and nothing a mint could fill, so the question this law asks is not
/// about it.
fn is_closed(declared: &syn::ItemStruct) -> bool {
    if !matches!(declared.vis, syn::Visibility::Public(_)) {
        return false;
    }
    let mut seats = declared.fields.iter();
    let Some(first) = seats.next() else {
        return false;
    };
    [first]
        .into_iter()
        .chain(seats)
        .all(|seat| !matches!(seat.vis, syn::Visibility::Public(_)))
}

/// The type one signature refuses with, or `None` where it refuses with
/// nothing.
///
/// The error position of a `Result`, which is the subsystem's own statement
/// about which of its types are refusals — written where the refusing happens
/// rather than in a register somebody maintains.
fn refusal_of(output: &syn::ReturnType) -> Option<String> {
    let syn::ReturnType::Type(_, declared) = output else {
        return None;
    };
    if head_of(declared)? != REFUSING_RETURN {
        return None;
    }
    head_of(type_arguments(declared).get(1).copied()?)
}

/// The type one signature hands back as its ANSWER, or `None` where the answer
/// is not a plain path.
///
/// The wrappers are unwrapped through their SUCCESS position alone, which is the
/// whole distinction this leg turns on: `Result<_, Body>` is a seam handing a
/// caller the refusal it raised, and `Result<Body, _>` is a road producing one.
fn answer_of(output: &syn::ReturnType) -> Option<String> {
    let syn::ReturnType::Type(_, declared) = output else {
        return None;
    };
    let mut current: &syn::Type = declared;
    for _ in 0..ANSWER_DEPTH {
        let head = head_of(current)?;
        if !ANSWER_WRAPPERS.contains(&head.as_str()) {
            return Some(head);
        }
        current = type_arguments(current).first().copied()?;
    }
    None
}

/// One head with the enclosing implementation's own type put back in place of
/// `Self`.
///
/// A road inside `impl ProjectionPlanning` spells its answer `Self`, and a
/// reading that took the word at face value would file every mint in the
/// subsystem under one name nobody declared.
fn resolved(head: String, own: Option<&str>) -> String {
    if head == OWN_TYPE {
        own.map_or(head, str::to_string)
    } else {
        head
    }
}

/// Whether one signature takes a receiver.
fn takes_a_receiver(sig: &syn::Signature) -> bool {
    sig.inputs
        .iter()
        .any(|input| matches!(*input, syn::FnArg::Receiver(_)))
}

/// The type arguments one type's last path segment carries, in declaration
/// order, with lifetimes and constants passed over.
fn type_arguments(declared: &syn::Type) -> Vec<&syn::Type> {
    let syn::Type::Path(typed) = declared else {
        return Vec::new();
    };
    let Some(last) = typed.path.segments.last() else {
        return Vec::new();
    };
    let syn::PathArguments::AngleBracketed(bracketed) = &last.arguments else {
        return Vec::new();
    };
    bracketed
        .args
        .iter()
        .filter_map(|argument| {
            if let syn::GenericArgument::Type(inner) = argument {
                Some(inner)
            } else {
                None
            }
        })
        .collect()
}

/// The last path segment of one type, or `None` where the type is not a plain
/// path.
///
/// Generic arguments are not part of the head, so `ProjectionClosureRefusal<R>`
/// and `ProjectionClosureRefusal` name one type here, exactly as they name one
/// body.
fn head_of(declared: &syn::Type) -> Option<String> {
    if let syn::Type::Path(typed) = declared {
        typed
            .path
            .segments
            .last()
            .map(|last| last.ident.to_string())
    } else {
        None
    }
}

/// Every source file the population is derived from: the metaprogramming
/// subsystem's own sources, minus the proof surface.
fn services_sources(root: &Path) -> Result<Vec<(String, String)>, String> {
    let base = root.join(TOOLING_DIRECTORY);
    if !base.is_dir() {
        return Err(format!(
            "{TOOLING_DIRECTORY}/ is not there: the subsystem this law is about cannot be read, \
             which is not the same as its having no refusal bodies"
        ));
    }
    let mut sources = Vec::new();
    visit_files(&base, &mut |path| {
        if path.extension().is_none_or(|extension| extension != "rs") {
            return Ok(());
        }
        let relative = relative_slash_path(root, path);
        if relative == PROOF_SURFACE {
            return Ok(());
        }
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        sources.push((relative, text));
        Ok(())
    })?;
    Ok(sources)
}

/// Planted reversals for the join, and the real subsystem judged by it.
///
/// The leg is pure over `(path, text)` pairs, so a reversal is a fixture held in
/// memory: the law that guards the services' mints is never proven by opening
/// one. The test that reads the real tree is named `the_real_…` and states what
/// it found rather than what it hoped for.
#[cfg(test)]
mod tests {
    use super::{mint_verdict, services_sources};
    use crate::repository::walk::repo_root;

    /// One synthetic source file.
    fn source(text: &str) -> Vec<(String, String)> {
        vec![(
            String::from("macros/macroc/src/home/types.rs"),
            text.to_string(),
        )]
    }

    /// One synthetic source file at a named path.
    fn source_at(path: &str, text: &str) -> (String, String) {
        (path.to_string(), text.to_string())
    }

    /// A body refused with, closed at every seat, minted crate-internally.
    const LAWFUL: &str = "pub struct DemoRefusal {\n\
         \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
         }\n\
         \n\
         impl DemoRefusal {\n\
         \x20   pub(crate) fn established(issue: DemoIssue) -> Self {\n\
         \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
         \x20   }\n\
         }\n\
         \n\
         impl DemoSeam {\n\
         \x20   pub fn checked(&self) -> Result<Self, DemoRefusal> {\n\
         \x20       Ok(Self)\n\
         \x20   }\n\
         }\n";

    /// The positive control: a closed body, refused with, whose one road is
    /// crate-internal. A check that flagged everything would satisfy every
    /// reversal below and be worthless.
    #[test]
    fn a_crate_internal_mint_is_lawful() {
        let verdict = mint_verdict(&source(LAWFUL));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Planted reversal: the fence with a loading dock behind it. The seat is
    /// private, the record cannot be written as a literal from outside, and one
    /// `pub` road hands the whole body back to any holder of an issue.
    #[test]
    fn a_public_mint_on_a_closed_body_is_a_violation() {
        let verdict = mint_verdict(&source(
            &LAWFUL.replace("pub(crate) fn established", "pub fn established"),
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(verdict.offenders.first().is_some_and(|offence| {
            offence.contains("DemoRefusal::established") && offence.contains("home/types.rs")
        }));
    }

    /// A body added later with a public mint is refused by the same derivation,
    /// with nothing about it written down anywhere: the population is derived
    /// rather than named, which is the whole of what this law adds to the
    /// fixture beside it.
    #[test]
    fn a_family_added_later_is_already_in_the_population() {
        let verdict = mint_verdict(&[
            source_at("macros/macroc/src/home/types.rs", LAWFUL),
            source_at(
                "macros/macroc/src/later/types.rs",
                "pub struct LaterRefusal {\n\
                 \x20   body: AdmittedPrefix<LaterIssue, LaterIssueLimit>,\n\
                 }\n\
                 \n\
                 impl LaterRefusal {\n\
                 \x20   pub fn established(issue: LaterIssue) -> Self {\n\
                 \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
                 \x20   }\n\
                 }\n\
                 \n\
                 impl LaterSeam {\n\
                 \x20   pub fn checked(&self) -> Result<Self, LaterRefusal> {\n\
                 \x20       Ok(Self)\n\
                 \x20   }\n\
                 }\n",
            ),
        ]);
        assert_eq!(verdict.bodies, 2);
        assert_eq!(verdict.roads, 2);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("LaterRefusal::established"))
        );
    }

    /// A road handing the body back inside an `Option` is a mint: the answer is
    /// the body, and a reader that only understood the bare spelling would let
    /// the subsystem's own `refused` helpers out of the population.
    #[test]
    fn an_optional_answer_is_still_a_mint() {
        let verdict = mint_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             }\n\
             \n\
             pub fn refused(issues: Vec<DemoIssue>) -> Option<DemoRefusal> {\n\
             \x20   None\n\
             }\n\
             \n\
             impl DemoSeam {\n\
             \x20   pub fn checked(&self) -> Result<Self, DemoRefusal> {\n\
             \x20       Ok(Self)\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
    }

    /// A public seam that REFUSES with the body is not a mint. This is the
    /// distinction the whole leg turns on: a caller receiving the refusal a seam
    /// raised is what the type exists for, and a reader that could not tell the
    /// error position from the success one would refuse every refusing road in
    /// the subsystem.
    #[test]
    fn a_public_refusing_seam_is_not_a_mint() {
        let verdict = mint_verdict(&source(LAWFUL));
        assert_eq!(verdict.roads, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A road taking a receiver is handed a body and hands one back, so it is a
    /// copy of something a pass already established rather than a new one minted
    /// from parts. Reading it as a mint would refuse a hand-written `Clone` and
    /// admit the derived one, which is a verdict decided by how a road was
    /// spelled.
    #[test]
    fn a_road_taking_a_receiver_is_not_a_mint() {
        let verdict = mint_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             }\n\
             \n\
             impl DemoRefusal {\n\
             \x20   pub fn duplicated(&self) -> Self {\n\
             \x20       Self { body: self.body.clone() }\n\
             \x20   }\n\
             \x20   pub(crate) fn established(issue: DemoIssue) -> Self {\n\
             \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
             \x20   }\n\
             }\n\
             \n\
             impl DemoSeam {\n\
             \x20   pub fn checked(&self) -> Result<Self, DemoRefusal> {\n\
             \x20       Ok(Self)\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A record with a public seat is not this law's subject, whatever its roads
    /// say. The literal is already writable from outside, so the question about
    /// it is the SEAT's, and answering it here would report the wrong defect.
    #[test]
    fn a_record_with_a_public_seat_is_not_in_the_population() {
        let verdict = mint_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   pub body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             }\n\
             \n\
             impl DemoRefusal {\n\
             \x20   pub fn established(issue: DemoIssue) -> Self {\n\
             \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
             \x20   }\n\
             }\n\
             \n\
             impl DemoSeam {\n\
             \x20   pub fn checked(&self) -> Result<Self, DemoRefusal> {\n\
             \x20       Ok(Self)\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 0);
        assert_eq!(verdict.roads, 0);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A closed record nobody refuses with is an ordinary guarded type, and a
    /// public constructor on one is ordinary. A reader that took every closed
    /// record for a refusal body would refuse half the subsystem.
    #[test]
    fn a_closed_record_nobody_refuses_with_is_not_a_body() {
        let verdict = mint_verdict(&source(
            "pub struct SpanTable {\n\
             \x20   positions: Vec<u32>,\n\
             }\n\
             \n\
             impl SpanTable {\n\
             \x20   pub fn issued(positions: Vec<u32>) -> Self {\n\
             \x20       Self { positions }\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 0);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A refusal body no road produces is an offence rather than a quiet pass.
    /// The law's numerator over it is empty, so it guards nothing while the
    /// printed denominator counts it as covered — which is the one failure a
    /// derived population exists to prevent.
    #[test]
    fn a_body_no_road_produces_is_a_violation() {
        let verdict = mint_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             }\n\
             \n\
             impl DemoSeam {\n\
             \x20   pub fn checked(&self) -> Result<Self, DemoRefusal> {\n\
             \x20       Ok(Self)\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("no road in the subsystem hands one back"))
        );
    }

    /// A source this reader cannot parse is a hole in the population, and it is
    /// reported as one. Silently reading it as "no bodies here" is the exact
    /// failure the derived denominator exists to prevent.
    #[test]
    fn an_unparsable_source_is_an_offence_rather_than_an_absence() {
        let verdict = mint_verdict(&source("pub struct DemoRefusal {\n"));
        assert_eq!(verdict.bodies, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("not parseable Rust"))
        );
    }

    /// The real subsystem holds: every closed refusal body it declares is minted
    /// only from inside the crate, and the derived population is real rather
    /// than empty.
    ///
    /// The counts are asserted as RELATIONS and never as a number. A test
    /// naming seven would be the hand-maintained inventory this law was written
    /// to retire, moved one file over; the run prints the numbers, and the
    /// relation is what has to hold.
    #[test]
    fn the_real_subsystem_mints_every_body_from_inside() {
        let read = repo_root()
            .map_err(|error| format!("the repository root could not be found: {error}"))
            .and_then(|root| services_sources(&root))
            .map(|sources| mint_verdict(&sources));
        assert!(
            read.is_ok(),
            "the mint gate could not read its subject: {read:?}"
        );
        assert!(
            read.as_ref()
                .is_ok_and(|verdict| verdict.offenders.is_empty()),
            "{read:?}"
        );
        assert!(
            read.as_ref().is_ok_and(|verdict| verdict.bodies > 0),
            "no closed refusal body found in the real subsystem: {read:?}"
        );
        assert!(
            read.is_ok_and(|verdict| verdict.roads >= verdict.bodies),
            "a closed refusal body in the real subsystem is produced by no road at all"
        );
    }
}
