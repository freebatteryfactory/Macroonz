//! The positivity ladder's derived population: a family that bounds an
//! inhabitant-promising seat and declares no compile-time magnitude is on the
//! runtime ladder.
//!
//! # The two ladders, and why only one of them needs a law
//!
//! `crate::types::Limit` families reach a capacity by one of two roads. A
//! DECLARED magnitude is a number in the source: `ConstLimit` carries it, a
//! `const` block stands it under a plane's ceiling, and `PositiveLimit` proves it
//! admits an item — all before the program runs, all by the compiler. An
//! EVIDENCE-SELECTED magnitude does not exist until the owner's evidence selects
//! it, so no `const` block can see it and the same two facts have to be
//! established by VALUES instead: `LimitWitness` carries the selection and
//! `PositiveLimitWitness` carries the promise that it admits an item.
//!
//! `PositiveLimitWitness::inhabited` is bounded on `EvidenceSelectedLimit`, so a
//! family that never declared the runtime ladder has no road to a runtime
//! capacity at all, and the compile-fail fixtures name that refusal from outside
//! the crate. That half is `rustc`'s and this law does not restate it.
//!
//! What `rustc` cannot say is the UNIVERSAL sentence: that every family which
//! needs the runtime ladder is on it. Rust cannot enumerate the types
//! implementing a trait, so a law naming the families would be a list somebody
//! maintains — the hand-maintained inventory this repository bans. The sentence
//! was therefore deliberately never written when the witness landed, and the
//! population it would have been about was named in prose and counted by nothing.
//!
//! # The population, DERIVED
//!
//! Three facts about the machine's own sources decide membership, and every one
//! of them is read off a parse:
//!
//!   1. the source declares `impl … Limit for F` — F is a limit family;
//!   2. no source declares `impl … ConstLimit for F` — F declares no
//!      compile-time magnitude, so the declared road is closed to it;
//!   3. F is the LAST type argument of a `NonEmptyBounded<…>` or an
//!      `AdmittedPrefix<…>` written in a DECLARATION — a field, a signature, an
//!      associated type, an alias. Both of those promise an inhabitant:
//!      `NonEmptyBounded` is structurally non-empty and `AdmittedPrefix` carries
//!      one.
//!
//! A family satisfying all three is in the DENOMINATOR: it bounds a seat that
//! promises an inhabitant, and the only road to that promise is the runtime
//! ladder. It is in the NUMERATOR — witnessed — when a source declares
//! `impl … EvidenceSelectedLimit for F`, which is the one bound under which a
//! `PositiveLimitWitness` for it can be minted at all. Both numbers are printed
//! on every run, because a population that quietly shrank would otherwise keep
//! this law passing while it guarded less.
//!
//! # What a DECLARATION is here, and why bodies are not read
//!
//! A seat is a declaration: what a record carries, what a road takes, what a road
//! hands back. A `let` binding inside a function body is not a seat — its type
//! came from a seat somewhere else, and reading it would count the same family
//! twice while adding a population no owner declared. So this reader descends
//! through items, fields, signatures, aliases, associated constants and
//! associated types, and stops at the first `{` of a body.
//!
//! # Its stated ceilings, said out loud
//!
//! **The services are outside the population, and not because they are
//! uninteresting.** `macros/macroc` declares every one of its limit families
//! through a `macro_rules!` transcriber — one row per family, expanding to
//! `impl Limit` and `impl ConstLimit` together. A reader that resolves no macro
//! sees no `ConstLimit` there, so every services family would read as declaring
//! no compile-time magnitude and this law would refuse all of them falsely. The
//! honest scope is the sources whose declarations are written where they are
//! read, and that is the machine's own.
//!
//! **The proof surface is outside it, for the reason `coupling` excludes it.**
//! `src/laws.rs` declares demonstration families whose whole existence is to give
//! the admission algebra something to refuse; counting them would put a fixture
//! in a denominator about the machine.
//!
//! **It resolves nothing.** A path is read by its LAST SEGMENT, so
//! `crate::types::Limit` and `Limit` are one trait here and a type alias to
//! either is neither. It evaluates no `cfg`: a declaration written under one is
//! read as declared. It expands no macro, so a family or a seat assembled by one
//! is outside this law — and this law does not pretend otherwise.
//!
//! **It does not claim a witness is ever minted.** That a family is ON the
//! ladder is what is read here; whether any runtime road ever selects a
//! magnitude for it is a behavioural fact no parse reaches, and the machine
//! carries no runtime yet.
//!
//! **It does not judge a family declaring BOTH ladders.** Such a family states
//! two authorities for one capacity, `crate::types` names it as a declaration
//! defect no bound can see, and it is outside this population by construction —
//! the denominator here is families with no compile-time magnitude. Widening the
//! claim to cover it is a separate law with its own name.

use std::collections::{BTreeMap, BTreeSet};

use crate::repository::snapshot::{MACHINE_DIRECTORY, RepositorySnapshot};
use crate::repository::types::CanonicalPath;

/// The proof surface, excluded from the population by name.
const PROOF_SURFACE: &str = "src/laws.rs";

/// The contract every limit family implements.
const LIMIT_CONTRACT: &str = "Limit";

/// The contract a family declaring a compile-time magnitude implements.
const DECLARED_LADDER: &str = "ConstLimit";

/// The contract a family declaring an evidence-selected magnitude implements —
/// the one bound `PositiveLimitWitness`'s mint stands on.
const RUNTIME_LADDER: &str = "EvidenceSelectedLimit";

/// The seats that promise an inhabitant: one structurally non-empty collection,
/// and the report package that carries one.
const INHABITANT_PROMISING_SEATS: [&str; 2] = ["NonEmptyBounded", "AdmittedPrefix"];

/// Every limit family bounding an inhabitant-promising seat in the machine, with
/// no compile-time magnitude declared for it, declares the runtime ladder.
///
/// # Errors
///
/// Returns the offences one line at a time, and returns a read failure as
/// itself: a gate that cannot read its subject says so rather than reporting an
/// empty population.
pub(crate) fn check_inhabitant_promising_limits(
    snapshot: &RepositorySnapshot,
) -> Result<(), String> {
    let sources = positivity_sources(snapshot)?;
    let verdict = positivity_verdict(&sources);

    // The denominator is DERIVED and printed on every run, because a population
    // that quietly shrank would otherwise keep this check passing while it
    // guarded less.
    println!(
        "inhabitant-promising limits: {} witnessed / {} declared",
        verdict.witnessed, verdict.declared
    );
    if verdict.declared == 0 {
        return Err(String::from(
            "no limit family was found bounding an inhabitant-promising seat without a \
             compile-time magnitude: this denominator cannot be empty while the runtime ladder \
             exists, so the reader is looking at the wrong tree",
        ));
    }
    if verdict.offenders.is_empty() {
        Ok(())
    } else {
        Err(verdict.offenders.join("; "))
    }
}

/// What the positivity leg counted, and what it refuses.
#[derive(Debug)]
struct PositivityVerdict {
    /// Families bounding an inhabitant-promising seat with no compile-time
    /// magnitude declared for them.
    declared: usize,
    /// Those of them declaring the runtime ladder.
    witnessed: usize,
    /// Every offence, one line each.
    offenders: Vec<String>,
}

/// What the sources declare about one limit family.
///
/// The ladders are carried as the SET OF CONTRACTS a source declares for the
/// family rather than as two flags, which is what `clippy.toml`'s
/// `max-struct-bools = 0` asks for and is also the honest shape: a family
/// declaring both ladders is representable here, and a pair of booleans would
/// have read as a state machine nobody named.
#[derive(Debug, Default)]
struct FamilyFacts {
    /// Where `impl … Limit for F` was read, where a source declares it.
    declared_at: Option<String>,
    /// Every ladder contract a source declares for it, by name.
    ladders: BTreeSet<String>,
}

/// Everything one pass over the sources read.
#[derive(Debug, Default)]
struct Reading {
    /// Every limit family the machine declares, by name.
    families: BTreeMap<String, FamilyFacts>,
    /// Every family bounding an inhabitant-promising seat, and the first
    /// declaration that seats it.
    seated: BTreeMap<String, String>,
}

/// Derives the population out of parsed trees and judges each family in it.
///
/// Pure over its inputs — `(canonical path, parsed tree)` pairs handed over by
/// the snapshot — so the reversals below are planted in memory and the law that
/// guards the ladder is never proven by editing a home. A source that did not
/// parse never reaches here: the snapshot carries it as unread, and the caller
/// refuses the whole reading rather than deriving a population one file short.
fn positivity_verdict(sources: &[(&CanonicalPath, &syn::File)]) -> PositivityVerdict {
    let reading = read_sources(sources);
    let mut verdict = PositivityVerdict {
        declared: 0,
        witnessed: 0,
        offenders: Vec::new(),
    };
    for (family, facts) in &reading.families {
        // No `impl … Limit for F` was read, so nothing here established that
        // this name is a limit family at all. Unreachable in code that
        // compiles — both ladders have `Limit` as their supertrait — and
        // stated rather than assumed, because a fixture can write it.
        let Some(declared_at) = &facts.declared_at else {
            continue;
        };
        if facts.ladders.contains(DECLARED_LADDER) {
            continue;
        }
        let Some(seat) = reading.seated.get(family) else {
            continue;
        };
        verdict.declared = verdict.declared.saturating_add(1);
        if facts.ladders.contains(RUNTIME_LADDER) {
            verdict.witnessed = verdict.witnessed.saturating_add(1);
            continue;
        }
        verdict.offenders.push(format!(
            "{seat}: `{family}` bounds a seat that promises an inhabitant and declares no \
             compile-time magnitude, and {declared_at} puts it on neither ladder; the only road to \
             a capacity for it is `{RUNTIME_LADDER}`, so no `PositiveLimitWitness<{family}>` can \
             be minted and the seat promises what nothing can establish"
        ));
    }
    verdict
}

/// Reads the family declarations and the seats out of the parsed trees.
fn read_sources(sources: &[(&CanonicalPath, &syn::File)]) -> Reading {
    let mut reading = Reading::default();
    for (path, file) in sources {
        read_items(path.as_str(), &file.items, &mut reading);
    }
    reading
}

/// Reads one module's items, then every inline module inside it.
///
/// Written as an `if let` chain rather than a match because `syn::Item` is
/// `non_exhaustive`: the items this reading has a question about are named, and
/// every other item is passed over without a wildcard arm standing in for a set
/// no crate outside `syn` can enumerate.
fn read_items(path: &str, items: &[syn::Item], reading: &mut Reading) {
    for item in items {
        if let syn::Item::Impl(declared) = item {
            read_implementation(path, declared, reading);
        } else if let syn::Item::Struct(declared) = item {
            read_fields(path, &declared.fields, reading);
        } else if let syn::Item::Union(declared) = item {
            for field in &declared.fields.named {
                read_seat(path, &field.ty, reading);
            }
        } else if let syn::Item::Enum(declared) = item {
            for variant in &declared.variants {
                read_fields(path, &variant.fields, reading);
            }
        } else if let syn::Item::Fn(declared) = item {
            read_signature(path, &declared.sig, reading);
        } else if let syn::Item::Type(declared) = item {
            read_seat(path, &declared.ty, reading);
        } else if let syn::Item::Const(declared) = item {
            read_seat(path, &declared.ty, reading);
        } else if let syn::Item::Static(declared) = item {
            read_seat(path, &declared.ty, reading);
        } else if let syn::Item::Trait(declared) = item {
            for member in &declared.items {
                read_trait_member(path, member, reading);
            }
        } else if let syn::Item::Mod(module) = item
            && let Some((_, inner)) = &module.content
        {
            read_items(path, inner, reading);
        }
    }
}

/// Reads one implementation: which ladder it declares for which family, and the
/// seats its own members declare.
fn read_implementation(path: &str, declared: &syn::ItemImpl, reading: &mut Reading) {
    if let Some((contract, _)) = &declared.trait_
        && let Some(contract) = last_segment(contract)
        && let Some(family) = head_of(&declared.self_ty)
    {
        record_ladder(path, &contract, &family, reading);
    }
    for member in &declared.items {
        if let syn::ImplItem::Fn(road) = member {
            read_signature(path, &road.sig, reading);
        } else if let syn::ImplItem::Const(held) = member {
            read_seat(path, &held.ty, reading);
        } else if let syn::ImplItem::Type(named) = member {
            read_seat(path, &named.ty, reading);
        }
    }
}

/// Records what one trait implementation says about one family.
///
/// A contract this law has no question about — and the machine declares many —
/// leaves the reading untouched, which is why the family map holds limit
/// families rather than every type that implements anything.
fn record_ladder(path: &str, contract: &str, family: &str, reading: &mut Reading) {
    if contract != LIMIT_CONTRACT && contract != DECLARED_LADDER && contract != RUNTIME_LADDER {
        return;
    }
    let facts = reading.families.entry(family.to_owned()).or_default();
    if contract == LIMIT_CONTRACT && facts.declared_at.is_none() {
        facts.declared_at = Some(path.to_owned());
    }
    facts.ladders.insert(contract.to_owned());
}

/// Reads one trait's members for the seats their declarations carry.
fn read_trait_member(path: &str, member: &syn::TraitItem, reading: &mut Reading) {
    if let syn::TraitItem::Fn(road) = member {
        read_signature(path, &road.sig, reading);
    } else if let syn::TraitItem::Const(held) = member {
        read_seat(path, &held.ty, reading);
    } else if let syn::TraitItem::Type(named) = member
        && let Some((_, stated)) = &named.default
    {
        read_seat(path, stated, reading);
    }
}

/// Reads the seats one record's fields declare, named or positional alike.
fn read_fields(path: &str, fields: &syn::Fields, reading: &mut Reading) {
    for field in fields {
        read_seat(path, &field.ty, reading);
    }
}

/// Reads the seats one signature declares: what the road takes and what it hands
/// back. The body is not read — see the module documentation.
fn read_signature(path: &str, signature: &syn::Signature, reading: &mut Reading) {
    for taken in &signature.inputs {
        if let syn::FnArg::Typed(named) = taken {
            read_seat(path, &named.ty, reading);
        }
    }
    if let syn::ReturnType::Type(_, handed) = &signature.output {
        read_seat(path, handed, reading);
    }
}

/// Records every family one declared type seats, at any depth inside it.
///
/// A seat behind a reference, inside a wrapper, or in a tuple is the same seat:
/// what decides membership is that an inhabitant-promising type names the family
/// as its bound, not where in the type that promise sits. Written as an `if let`
/// chain because `syn::Type` is `non_exhaustive`; the kinds a declared seat in
/// this repository is written in are named, and a seat written behind
/// `impl Trait`, a trait object, or a macro is outside this reading and is not
/// claimed to be inside it.
fn read_seat(path: &str, declared: &syn::Type, reading: &mut Reading) {
    if let syn::Type::Path(typed) = declared {
        if let Some(qualified) = &typed.qself {
            read_seat(path, &qualified.ty, reading);
        }
        for segment in &typed.path.segments {
            read_segment(path, segment, reading);
        }
    } else if let syn::Type::Reference(held) = declared {
        read_seat(path, &held.elem, reading);
    } else if let syn::Type::Ptr(held) = declared {
        read_seat(path, &held.elem, reading);
    } else if let syn::Type::Paren(held) = declared {
        read_seat(path, &held.elem, reading);
    } else if let syn::Type::Group(held) = declared {
        read_seat(path, &held.elem, reading);
    } else if let syn::Type::Slice(held) = declared {
        read_seat(path, &held.elem, reading);
    } else if let syn::Type::Array(held) = declared {
        read_seat(path, &held.elem, reading);
    } else if let syn::Type::Tuple(held) = declared {
        for inner in &held.elems {
            read_seat(path, inner, reading);
        }
    } else if let syn::Type::FnPtr(road) = declared {
        for taken in &road.inputs {
            read_seat(path, &taken.ty, reading);
        }
        if let syn::ReturnType::Type(_, handed) = &road.output {
            read_seat(path, handed, reading);
        }
    }
}

/// Reads one path segment: whether it IS an inhabitant-promising seat, and
/// whatever its own arguments seat one level down.
///
/// The bound is the LAST type argument, because that is where both promising
/// types carry their family — `NonEmptyBounded<T, L>` and
/// `AdmittedPrefix<T, L>`. Every type argument is then descended into, so a
/// promise wrapped in anything is still read.
fn read_segment(path: &str, segment: &syn::PathSegment, reading: &mut Reading) {
    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return;
    };
    let bounds: Vec<&syn::Type> = arguments.args.iter().filter_map(type_argument).collect();
    if INHABITANT_PROMISING_SEATS.contains(&segment.ident.to_string().as_str())
        && let Some(family) = bounds.last().and_then(|last| head_of(last))
    {
        reading
            .seated
            .entry(family)
            .or_insert_with(|| path.to_owned());
    }
    for inner in bounds {
        read_seat(path, inner, reading);
    }
}

/// The type one generic argument states, where it states a type at all: a
/// lifetime and a constant are arguments too, and neither is a seat.
fn type_argument(argument: &syn::GenericArgument) -> Option<&syn::Type> {
    if let syn::GenericArgument::Type(stated) = argument {
        Some(stated)
    } else {
        None
    }
}

/// The last path segment of one type, or `None` where the type is not a plain
/// path.
///
/// Generic arguments are not part of the head, so `Refusal<R>` and `Refusal`
/// name one type here.
fn head_of(declared: &syn::Type) -> Option<String> {
    if let syn::Type::Path(typed) = declared {
        last_segment(&typed.path)
    } else {
        None
    }
}

/// The last segment of one path, by name.
fn last_segment(path: &syn::Path) -> Option<String> {
    path.segments.last().map(|last| last.ident.to_string())
}

/// Every parsed source the population is derived from: the machine's own
/// sources, minus the proof surface.
///
/// Taken from the one reading. A source the snapshot could not parse refuses
/// the whole law rather than leaving the population one file short — a
/// denominator that shrank in silence is the single failure a derived
/// denominator exists to prevent.
fn positivity_sources(
    snapshot: &RepositorySnapshot,
) -> Result<Vec<(&CanonicalPath, &syn::File)>, String> {
    Ok(snapshot
        .rust()
        .parsed_under(&[MACHINE_DIRECTORY])?
        .into_iter()
        .filter(|(path, _)| path.as_str() != PROOF_SURFACE)
        .collect())
}

/// The families one reading found seated, by name, for the reversals that
/// assert which seats a declaration is read through.
#[cfg(test)]
fn seated_families(sources: &[(&CanonicalPath, &syn::File)]) -> Vec<String> {
    read_sources(sources).seated.into_keys().collect()
}

/// Planted reversals for the positivity leg, and the real tree judged by it.
///
/// Every leg is pure over `(path, text)` pairs, so a reversal is a fixture held
/// in memory: the law that guards the ladder is never proven by taking a family
/// off it. The test that reads the real tree is named `the_real_…` and states
/// what it found rather than what it hoped for.
#[cfg(test)]
mod tests {
    use super::{
        PositivityVerdict, positivity_sources, positivity_verdict as verdict_of_trees,
        seated_families as seated_of_trees,
    };
    use crate::repository::snapshot::repository_snapshot;
    use crate::repository::types::CanonicalPath;

    /// One synthetic source file.
    fn source(text: &str) -> Vec<(String, String)> {
        vec![(String::from("src/00_home/types.rs"), text.to_string())]
    }

    /// The parsed form of a fixture set, or the offences of the ones that did
    /// not parse.
    fn parse(sources: &[(String, String)]) -> (Vec<(CanonicalPath, syn::File)>, Vec<String>) {
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
        (parsed, unparsable)
    }

    /// The verdict over fixture source TEXT.
    ///
    /// The law itself is handed trees the snapshot already parsed, so a source
    /// it could not read never reaches it. A fixture is text, so this adapter
    /// parses one and reports a fixture that does not parse exactly as the
    /// reading reports a source it could not read.
    fn positivity_verdict(sources: &[(String, String)]) -> PositivityVerdict {
        let (parsed, unparsable) = parse(sources);
        let trees: Vec<(&CanonicalPath, &syn::File)> =
            parsed.iter().map(|(path, file)| (path, file)).collect();
        let mut verdict = verdict_of_trees(&trees);
        verdict.offenders.splice(0..0, unparsable);
        verdict
    }

    /// The families one fixture set seats, by name.
    fn seated_families(sources: &[(String, String)]) -> Vec<String> {
        let (parsed, _) = parse(sources);
        let trees: Vec<(&CanonicalPath, &syn::File)> =
            parsed.iter().map(|(path, file)| (path, file)).collect();
        seated_of_trees(&trees)
    }

    /// A family on the runtime ladder, seated in a record that promises an
    /// inhabitant.
    const WITNESSED: &str = "\
    pub struct DemoIssueLimit;\n\
    impl Limit for DemoIssueLimit {}\n\
    impl crate::types::EvidenceSelectedLimit for DemoIssueLimit {}\n\
    pub struct DemoRefusal {\n\
    \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
    }\n";

    /// The same family with the runtime ladder taken off it: the seat still
    /// promises an inhabitant and nothing can establish the promise.
    const OFF_THE_LADDER: &str = "\
    pub struct DemoIssueLimit;\n\
    impl Limit for DemoIssueLimit {}\n\
    pub struct DemoRefusal {\n\
    \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
    }\n";

    /// The positive control: a seated family with no compile-time magnitude,
    /// declaring the runtime ladder, is lawful and IS counted. A law that
    /// flagged everything would satisfy every reversal below and be worthless.
    #[test]
    fn a_seated_family_on_the_runtime_ladder_is_lawful() {
        let verdict = positivity_verdict(&source(WITNESSED));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.witnessed, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Planted reversal, and the first of the two directions this leg must see:
    /// a seat that promises an inhabitant while its family stays off the ladder.
    #[test]
    fn a_seat_promising_an_inhabitant_off_the_ladder_is_a_violation() {
        let verdict = positivity_verdict(&source(OFF_THE_LADDER));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.witnessed, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("puts it on neither ladder")),
            "{:?}",
            verdict.offenders
        );
    }

    /// The second direction: a family that GAINS a witness moves the numerator.
    ///
    /// One body, read twice, differing by the one line that puts the family on
    /// the ladder. This is what makes the printed numerator a derivation rather
    /// than a constant: it moved because the tree moved, and a reader that
    /// returned a fixed number would fail here rather than in production.
    #[test]
    fn a_family_that_gains_a_witness_moves_the_numerator() {
        let before = positivity_verdict(&source(OFF_THE_LADDER));
        let after = positivity_verdict(&source(WITNESSED));
        assert_eq!(before.declared, after.declared, "the denominator moved");
        assert_eq!(before.witnessed, 0);
        assert_eq!(after.witnessed, 1);
        assert_eq!(before.offenders.len(), 1, "{:?}", before.offenders);
        assert!(after.offenders.is_empty(), "{:?}", after.offenders);
    }

    /// A family with a compile-time magnitude is outside this population
    /// entirely: the declared road is open to it, `PositiveLimit` proves its
    /// positivity before the program runs, and the runtime ladder is not the
    /// road it takes.
    #[test]
    fn a_family_with_a_declared_magnitude_is_not_this_laws_subject() {
        let verdict = positivity_verdict(&source(
            "pub struct DemoIssueLimit;\n\
             impl Limit for DemoIssueLimit {}\n\
             impl ConstLimit for DemoIssueLimit {\n\
             \x20   const MAX: usize = 32;\n\
             }\n\
             pub struct DemoRefusal {\n\
             \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 0);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A family that bounds no inhabitant-promising seat is outside it too. A
    /// `Bounded` seat admits an empty collection on purpose, so a magnitude of
    /// zero is an honest selection for it and no positivity is promised.
    #[test]
    fn a_family_bounding_no_inhabitant_promising_seat_is_not_in_the_population() {
        let verdict = positivity_verdict(&source(
            "pub struct DemoTextLimit;\n\
             impl Limit for DemoTextLimit {}\n\
             pub struct DemoText {\n\
             \x20   letters: Bounded<u8, DemoTextLimit>,\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 0);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Both inhabitant-promising seats are read, and read through whatever the
    /// declaration wraps them in: a reference handed back by an accessor, and a
    /// wrapper around the package.
    #[test]
    fn a_seat_is_read_through_the_declaration_that_wraps_it() {
        let seated = seated_families(&source(
            "impl DemoRefusal {\n\
             \x20   pub const fn issues(&self) -> &NonEmptyBounded<DemoIssue, ReturnedLimit> {\n\
             \x20       self.body.carried()\n\
             \x20   }\n\
             }\n\
             pub struct Held {\n\
             \x20   held: Option<AdmittedPrefix<DemoIssue, WrappedLimit>>,\n\
             \x20   listed: [NonEmptyBounded<DemoIssue, ArrayedLimit>; 2],\n\
             }\n",
        ));
        assert_eq!(
            seated,
            vec!["ArrayedLimit", "ReturnedLimit", "WrappedLimit"]
        );
    }

    /// A generic parameter is not a family. A signature bounded on `L: Limit`
    /// seats whatever its caller supplies, and reading `L` as a family would put
    /// a name no owner declared into the population — where, declaring neither
    /// ladder, it would refuse forever.
    #[test]
    fn a_generic_parameter_is_not_a_family() {
        let verdict = positivity_verdict(&source(
            "pub struct AdmittedPrefix<T, L: Limit> {\n\
             \x20   carried: NonEmptyBounded<T, L>,\n\
             }\n\
             impl<T, L: Limit> AdmittedPrefix<T, L> {\n\
             \x20   pub const fn carried(&self) -> &NonEmptyBounded<T, L> {\n\
             \x20       &self.carried\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 0);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A `let` binding is not a seat. Its type came from a seat somewhere else,
    /// and counting it would derive a population out of function bodies that no
    /// owner declared.
    #[test]
    fn a_binding_inside_a_body_is_not_a_seat() {
        let seated = seated_families(&source(
            "pub struct BoundLimit;\n\
             impl Limit for BoundLimit {}\n\
             fn demonstrate() {\n\
             \x20   let held: NonEmptyBounded<u8, BoundLimit> = NonEmptyBounded::singleton(5);\n\
             \x20   drop(held);\n\
             }\n",
        ));
        assert!(seated.is_empty(), "{seated:?}");
    }

    /// A family and the seat that bounds it are read across the files of one
    /// crate: the machine declares its families in `types.rs` and seats several
    /// of them in the accessors beside them, and a reader scoped to one file
    /// would drop half of every pair.
    #[test]
    fn a_family_and_its_seat_are_read_across_files() {
        let verdict = positivity_verdict(&[
            (
                String::from("src/13_home/types.rs"),
                String::from(
                    "pub struct SplitLimit;\nimpl crate::types::Limit for SplitLimit {}\n",
                ),
            ),
            (
                String::from("src/13_home/type_guard.rs"),
                String::from(
                    "impl SplitRefusal {\n    pub const fn issues(&self) -> &NonEmptyBounded<SplitIssue, SplitLimit> {\n        self.body.carried()\n    }\n}\n",
                ),
            ),
        ]);
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.witnessed, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
    }

    /// A source this reader cannot parse is a hole in the population, and it is
    /// reported as one. Silently reading it as "no families here" is the exact
    /// failure the derived denominator exists to prevent.
    #[test]
    fn an_unparsable_source_is_an_offence_rather_than_an_absence() {
        let verdict = positivity_verdict(&source("impl Limit for {\n"));
        assert_eq!(verdict.declared, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("not parseable Rust")),
            "{:?}",
            verdict.offenders
        );
    }

    /// The real machine holds: every family bounding an inhabitant-promising
    /// seat without a compile-time magnitude is on the runtime ladder, and the
    /// derived population is real rather than empty.
    ///
    /// The count is asserted as a RELATION and never as a number. A test naming
    /// eight would be the hand list this leg exists to replace, moved one file
    /// over; the run prints the numbers, and the relation is what has to hold.
    #[test]
    fn the_real_machine_witnesses_every_inhabitant_promising_limit() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let sources = positivity_sources(snapshot)?;
        let verdict = verdict_of_trees(&sources);
        assert!(verdict.offenders.is_empty(), "{verdict:?}");
        assert!(
            verdict.declared > 0,
            "no inhabitant-promising limit family found in the real machine: {verdict:?}"
        );
        assert_eq!(
            verdict.witnessed, verdict.declared,
            "the machine seats a family that promises an inhabitant and is on neither ladder"
        );
        Ok(())
    }
}
