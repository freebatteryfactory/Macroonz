//! The coupling join: a collection-shaped refusal family carries ONE body seat.
//!
//! A refusal family that declares `FamilyShape::IssueCollection` reports issues
//! it established and a claim about its own coverage of them. Written as two
//! seats, those are two values a holder may pair freely, so the body one pass
//! produced can be reported under the coverage claim another pass wrote — both
//! halves individually honest, the pair false, and nothing in the types
//! noticing. Written as one `AdmittedPrefix`, the pairing is not expressible.
//!
//! The compile-time half of that claim is `laws.rs`
//! `refusal::every_collection_family_carries_the_coupled_seat`, which names each
//! family and proves its reader pair. This law is the half a law cannot state:
//! Rust cannot enumerate its own impls, so a law naming families is a list
//! somebody maintains, and a list somebody maintains is exactly the
//! hand-maintained inventory this repository bans. Here the population is
//! DERIVED — every `FamilyShape::IssueCollection` declaration in the machine and
//! in the services, read off the sources — and the seat each one declares is
//! read off the same sources. A family added without the coupled seat is caught
//! by the derivation rather than by anybody remembering to extend a list.
//!
//! Three facts about one body decide the verdict, because a migration that
//! lands the coupled seat and leaves the old seats standing recreates the exact
//! pair. The body's own seat must BE the coupled type — a field whose declared
//! type IS an `AdmittedPrefix`, not a field that merely carries one inside a
//! wrapper — and neither a loose `CompletionPosture` seat nor a loose
//! `NonEmptyBounded<…>` carry may stand beside it. A hybrid carrying the coupled
//! seat AND a loose carry is the shape a partial migration leaves behind, and it
//! is refused rather than counted, because the loose half is exactly what a
//! holder can read against another body's posture.
//!
//! # The population is read from a parse, not from lines
//!
//! The reader is `syn`, and the reason is that the previous one was a line
//! scanner that recognized a family only where the shape constant sat on the
//! line IMMEDIATELY BELOW the `impl … RefusalFamily for …` line. A doc comment,
//! a blank line, an attribute, or another associated constant in between made
//! the family vanish from the numerator AND from the denominator at once — so
//! the printed count stayed whole while the guarded population shrank, which is
//! the one failure a derived denominator exists to prevent. A denominator that
//! silently drops its subject is worse than no denominator, because it reads as
//! coverage.
//!
//! What the parse establishes that a scan cannot: that an item IS a trait
//! implementation, that the trait it implements is the family contract, which
//! type it implements it FOR, that `SHAPE` is a MEMBER of that implementation
//! wherever it sits inside it, and that a field's declared type is that type
//! rather than a substring of some other one. Those are questions about items,
//! members, and paths, and answering them off text means writing a Rust parser
//! by hand inside a check. There is one reader, not two: a shallow second lane
//! kept beside this one would be a weaker statement of the same claim, and a
//! weaker statement is exactly what keeps passing after the stronger one is
//! removed.
//!
//! # What this reader does and does not resolve
//!
//! A family's body is resolved in the DECLARING HOME and nowhere else — the
//! module the implementation was read inside, which for a top-level
//! implementation is the home directory whose `mod.rs` joins `types.rs` and
//! `type_contract.rs` into one module family. There is no tree-wide fallback: two
//! homes may each declare a `pub struct Refusal`, and each family resolves only
//! its own. A body deliberately separated from its implementation across homes is
//! a relationship somebody must declare in types, not one this reader may guess
//! at from a name collision.
//!
//! It does not compile anything. A path is read by its LAST SEGMENT, so
//! `AdmittedPrefix<…>` and `refusal::AdmittedPrefix<…>` are the same seat here
//! and a type alias to either is neither. It does not evaluate `cfg`: a member
//! written under one is read as declared. It does not expand macros, so a family
//! or a body assembled by one is outside this law — and this law does not pretend
//! otherwise.

use std::fs;
use std::path::Path;

use crate::repository::walk::{TOOLING_DIRECTORY, relative_slash_path, visit_files};

/// The proof surfaces, excluded from the population by name.
///
/// Both crates' `laws.rs` declare demonstration families whose whole content is
/// a pair of constants — they exist so the admission algebra has something to
/// refuse, they have no body at all, and counting them would put a fixture in a
/// denominator about the machine.
const PROOF_SURFACES: [&str; 2] = ["src/laws.rs", "macros/macroc/src/laws.rs"];

/// The contract an implementation must implement to declare a family.
const FAMILY_CONTRACT: &str = "RefusalFamily";

/// The associated constant a family states its shape with.
const SHAPE_CONSTANT: &str = "SHAPE";

/// The shape word that puts a family in this population.
const COLLECTION_SHAPE: &str = "IssueCollection";

/// The coupled seat: band 00's report package.
const COUPLED_SEAT: &str = "AdmittedPrefix";

/// The posture seat a two-seat body carried beside its issues.
const POSTURE_SEAT: &str = "CompletionPosture";

/// The loose carry a two-seat body carried beside its posture — the other half
/// of the pair, which a migration that added the coupled seat without removing
/// the old one leaves standing.
const LOOSE_CARRY: &str = "NonEmptyBounded";

/// Every collection-shaped family declared anywhere in the machine or the
/// services carries its issues and its coverage claim in one `AdmittedPrefix`
/// seat, and carries no posture seat beside it.
///
/// # Errors
///
/// Returns the offences one line at a time, and returns a read failure as
/// itself: a gate that cannot read its subject says so rather than reporting an
/// empty population.
pub(crate) fn check_collection_bodies_are_coupled(root: &Path) -> Result<(), String> {
    let sources = coupling_sources(root)?;
    let verdict = coupled_body_verdict(&sources);

    // The denominator is DERIVED and printed on every run, because a population
    // that quietly shrank would otherwise keep this check passing while it
    // guarded less.
    println!(
        "collection bodies: {} coupled / {} declared",
        verdict.coupled, verdict.declared
    );
    if verdict.declared == 0 {
        return Err(String::from(
            "no collection-shaped refusal family was found: this denominator cannot be empty \
             while the families exist, so the reader is looking at the wrong tree",
        ));
    }
    if verdict.offenders.is_empty() {
        Ok(())
    } else {
        Err(verdict.offenders.join("; "))
    }
}

/// What the coupling leg counted, and what it refuses.
#[derive(Debug)]
struct CouplingVerdict {
    /// Families declaring the collection shape.
    declared: usize,
    /// Families whose declared body is the one coupled seat.
    coupled: usize,
    /// Families whose body is something else, one offence each.
    offenders: Vec<String>,
}

/// One family declaration, as it was read out of a parsed tree.
struct DeclaredFamily {
    /// The repository-relative path that declares it.
    path: String,
    /// The module its body is resolved in, and only there.
    home: String,
    /// The type it implements the family contract for, by its last path
    /// segment: `Refusal<R>` and `Refusal` name one body.
    family: String,
}

/// One public body, as it was read out of a parsed tree.
struct DeclaredBody {
    /// The module it stands in.
    home: String,
    /// Its declared name.
    name: String,
    /// The head of each field's declared type, in declaration order. A field
    /// whose type is not a plain path contributes `None`, which is neither the
    /// coupled seat nor either loose one.
    seats: Vec<Option<String>>,
}

/// Everything one pass over the sources read.
struct Reading {
    /// Every collection-shaped family declaration.
    families: Vec<DeclaredFamily>,
    /// Every public body, in every module the pass entered.
    bodies: Vec<DeclaredBody>,
    /// Sources that are not parseable Rust, one offence each. Never a skip: a
    /// file this reader could not read is a hole in the population, and a hole
    /// reported as nothing is the defect this whole leg is about.
    unparsable: Vec<String>,
}

/// Reads the declarations and the bodies out of source text and judges each
/// family.
///
/// Pure over its inputs — `(repository-relative path, source text)` pairs — so
/// the reversals below are planted in memory and the law that guards the tree is
/// never proven by editing one.
fn coupled_body_verdict(sources: &[(String, String)]) -> CouplingVerdict {
    let reading = read_sources(sources);
    let mut verdict = CouplingVerdict {
        declared: 0,
        coupled: 0,
        offenders: reading.unparsable,
    };
    for declared in &reading.families {
        verdict.declared = verdict.declared.saturating_add(1);
        let path = &declared.path;
        let family = &declared.family;
        let mut standing = reading
            .bodies
            .iter()
            .filter(|body| body.home == declared.home && body.name == *family);
        let Some(body) = standing.next() else {
            verdict.offenders.push(format!(
                "{path}: {family} declares the collection shape and no `pub struct {family}` \
                 stands in the home that declares it"
            ));
            continue;
        };
        if standing.next().is_some() {
            verdict.offenders.push(format!(
                "{path}: two `pub struct {family}` bodies stand in the home that declares the \
                 family, so which one carries the seat is a traversal order rather than a fact"
            ));
            continue;
        }
        let seats = &body.seats;
        let coupled = declares(seats, COUPLED_SEAT);
        let loose_posture = declares(seats, POSTURE_SEAT);
        let loose_carry = declares(seats, LOOSE_CARRY);
        if loose_posture {
            verdict.offenders.push(format!(
                "{path}: {family} carries a {POSTURE_SEAT} seat beside its issues; a coverage \
                 claim seated apart from its body is a claim that can be read against another \
                 body"
            ));
        }
        if coupled && loose_carry {
            verdict.offenders.push(format!(
                "{path}: {family} carries its issues twice — a {COUPLED_SEAT}<…> seat and a \
                 {LOOSE_CARRY}<…> seat beside it; the loose carry is half of the pair the coupled \
                 seat exists to keep together, and it can be read against another body's posture"
            ));
        }
        if !coupled {
            verdict.offenders.push(format!(
                "{path}: {family} declares the collection shape and carries no {COUPLED_SEAT}<…> \
                 seat"
            ));
        }
        if coupled && !loose_posture && !loose_carry {
            verdict.coupled = verdict.coupled.saturating_add(1);
        }
    }
    verdict
}

/// Whether one body seats the named type directly.
///
/// The seat must BE the type. A field carrying it inside a wrapper contributes
/// the wrapper's head instead, so `Vec<AdmittedPrefix<…>>` is not the body seat
/// and is not counted as one.
fn declares(seats: &[Option<String>], named: &str) -> bool {
    seats
        .iter()
        .any(|seat| seat.as_deref().is_some_and(|head| head == named))
}

/// Parses every source and reads the declarations, the bodies, and the failures
/// out of the trees.
fn read_sources(sources: &[(String, String)]) -> Reading {
    let mut reading = Reading {
        families: Vec::new(),
        bodies: Vec::new(),
        unparsable: Vec::new(),
    };
    for (path, text) in sources {
        match syn::parse_file(text) {
            Ok(file) => read_module(path, &declaring_home(path), &file.items, &mut reading),
            Err(error) => reading.unparsable.push(format!(
                "{path}: this file is not parseable Rust, so the population derived from it is \
                 unknown rather than empty: {error}"
            )),
        }
    }
    reading
}

/// The module a top-level declaration in one file is resolved in: the home
/// directory the file sits in.
///
/// The repository's file grammar splits one home across `types.rs`,
/// `type_guard.rs`, and `type_contract.rs`, and `mod.rs` joins them — the
/// implementation and the body it is about are one module family written in two
/// files. The directory is that module family spelled as a path, which is why it
/// is the resolution scope rather than the file.
fn declaring_home(path: &str) -> String {
    match path.rsplit_once('/') {
        Some((directory, _)) => directory.to_string(),
        None => String::new(),
    }
}

/// Reads one module's items, then every inline module inside it.
///
/// An inline `mod` is its own resolution scope, keyed by the file that writes it
/// so two files' identically named inline modules never resolve into each other.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`syn::Item` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every item it catches is one this reading has no question about"
)]
fn read_module(path: &str, home: &str, items: &[syn::Item], reading: &mut Reading) {
    for item in items {
        match item {
            syn::Item::Impl(declared) => {
                if let Some(family) = collection_family(declared) {
                    reading.families.push(DeclaredFamily {
                        path: path.to_string(),
                        home: home.to_string(),
                        family,
                    });
                }
            }
            syn::Item::Struct(declared) => {
                if matches!(declared.vis, syn::Visibility::Public(_)) {
                    reading.bodies.push(DeclaredBody {
                        home: home.to_string(),
                        name: declared.ident.to_string(),
                        seats: body_seats(&declared.fields),
                    });
                }
            }
            syn::Item::Mod(module) => {
                if let Some((_, inner)) = &module.content {
                    let inside = format!("{path}::{}", module.ident);
                    read_module(path, &inside, inner, reading);
                }
            }
            _ => {}
        }
    }
}

/// The type one implementation declares the collection shape for, or `None`
/// where the implementation is not a collection-shaped family declaration.
///
/// Three things must hold together: the item implements a trait, that trait's
/// last path segment is the family contract, and the implementation carries a
/// `SHAPE` member whose value names the collection shape. The member is looked
/// for ANYWHERE in the implementation — the whole point of reading a tree is
/// that a member's position inside its item is not a fact about the item.
fn collection_family(declared: &syn::ItemImpl) -> Option<String> {
    let (contract, _) = declared.trait_.as_ref()?;
    if last_segment(contract)? != FAMILY_CONTRACT {
        return None;
    }
    let collection = declared.items.iter().any(|member| {
        let syn::ImplItem::Const(constant) = member else {
            return false;
        };
        constant.ident == SHAPE_CONSTANT
            && shape_word(&constant.expr).is_some_and(|word| word == COLLECTION_SHAPE)
    });
    if collection {
        head_of(&declared.self_ty)
    } else {
        None
    }
}

/// The head of each field's declared type, in declaration order.
///
/// Named and positional bodies are read alike: a tuple body seating the coupled
/// type carries it exactly as a named one does, and a unit body seats nothing.
fn body_seats(fields: &syn::Fields) -> Vec<Option<String>> {
    match *fields {
        syn::Fields::Named(ref named) => named.named.iter().map(|f| head_of(&f.ty)).collect(),
        syn::Fields::Unnamed(ref unnamed) => {
            unnamed.unnamed.iter().map(|f| head_of(&f.ty)).collect()
        }
        syn::Fields::Unit => Vec::new(),
    }
}

/// The last path segment of one type, or `None` where the type is not a plain
/// path.
///
/// Generic arguments are not part of the head, so `Refusal<R>` and `Refusal`
/// name one type here, exactly as they name one body.
fn head_of(declared: &syn::Type) -> Option<String> {
    if let syn::Type::Path(typed) = declared {
        last_segment(&typed.path)
    } else {
        None
    }
}

/// The variant word one path-spelled constant states, or `None` where the value
/// is not a path.
fn shape_word(expression: &syn::Expr) -> Option<String> {
    if let syn::Expr::Path(spelled) = expression {
        last_segment(&spelled.path)
    } else {
        None
    }
}

/// The last segment of one path, by name.
fn last_segment(path: &syn::Path) -> Option<String> {
    path.segments.last().map(|last| last.ident.to_string())
}

/// Every source file the population is derived from: the machine's own sources
/// and the services', minus the two proof surfaces.
fn coupling_sources(root: &Path) -> Result<Vec<(String, String)>, String> {
    let mut sources = Vec::new();
    for directory in ["src", TOOLING_DIRECTORY] {
        let base = root.join(directory);
        if !base.is_dir() {
            continue;
        }
        visit_files(&base, &mut |path| {
            if path.extension().is_none_or(|extension| extension != "rs") {
                return Ok(());
            }
            let relative = relative_slash_path(root, path);
            if PROOF_SURFACES.contains(&relative.as_str()) {
                return Ok(());
            }
            let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
            sources.push((relative, text));
            Ok(())
        })?;
    }
    Ok(sources)
}

/// Planted reversals for the join, and the real repository judged by it.
///
/// Every leg is pure over `(path, text)` pairs, so a reversal is a fixture held
/// in memory: the law that guards the machine's bodies is never proven by
/// breaking one. The test that reads the real tree is named `the_real_…` and
/// states what it found rather than what it hoped for.
#[cfg(test)]
mod tests {
    use super::{coupled_body_verdict, coupling_sources};
    use crate::repository::walk::repo_root;

    /// One synthetic source file.
    fn source(text: &str) -> Vec<(String, String)> {
        vec![(String::from("home/types.rs"), text.to_string())]
    }

    /// One synthetic source file at a named path.
    fn source_at(path: &str, text: &str) -> (String, String) {
        (path.to_string(), text.to_string())
    }

    /// The positive control: a family whose body is the one coupled seat. A
    /// check that flagged everything would satisfy every reversal below and be
    /// worthless.
    #[test]
    fn a_coupled_body_is_lawful() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   /// The established issues and what the body says about them.\n\
             \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             }\n\
             \n\
             impl RefusalFamily for DemoRefusal {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Planted reversal: the two-seat body this law exists to end. Both halves
    /// read as honest and the pair is what nothing else catches.
    #[test]
    fn a_two_seat_body_is_a_violation() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   pub issues: NonEmptyBounded<DemoIssue, DemoIssueLimit>,\n\
             \x20   pub posture: CompletionPosture,\n\
             }\n\
             \n\
             impl RefusalFamily for DemoRefusal {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 0);
        assert_eq!(verdict.offenders.len(), 2, "{:?}", verdict.offenders);
    }

    /// Planted reversal: a coupled seat with a posture seat left standing beside
    /// it. The body would read as migrated and the loose claim would still be
    /// writable.
    #[test]
    fn a_posture_seat_beside_a_coupled_one_is_a_violation() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             \x20   pub posture: CompletionPosture,\n\
             }\n\
             \n\
             impl RefusalFamily for DemoRefusal {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.coupled, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("beside its issues"))
        );
    }

    /// Planted reversal: the hybrid a half-finished migration leaves behind —
    /// the coupled seat landed and the loose carry never removed. The body reads
    /// as migrated at a glance, and the swappable pair is fully recreated: a
    /// holder can carry the loose issues away and report them under another
    /// body's posture, which is the one defect the coupled seat exists to make
    /// unwritable.
    #[test]
    fn a_loose_carry_beside_a_coupled_seat_is_a_violation() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   report: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             \x20   pub issues: NonEmptyBounded<DemoIssue, DemoIssueLimit>,\n\
             }\n\
             \n\
             impl RefusalFamily for DemoRefusal {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("carries its issues twice"))
        );
    }

    /// Planted reversal: a coupled type nested inside a wrapper rather than
    /// seated as the body. A reader that accepted any field mentioning the
    /// package would let a body that never carried one read as coupled.
    #[test]
    fn a_nested_coupled_type_is_not_the_body_seat() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   pub attempts: Vec<AdmittedPrefix<DemoIssue, DemoIssueLimit>>,\n\
             }\n\
             \n\
             impl RefusalFamily for DemoRefusal {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("carries no"))
        );
    }

    /// Planted reversal: a declared family whose body nobody wrote. The
    /// declaration reads as a family and there is nothing behind it.
    #[test]
    fn a_declared_family_with_no_body_is_a_violation() {
        let verdict = coupled_body_verdict(&source(
            "impl RefusalFamily for AbsentRefusal {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("no `pub struct AbsentRefusal`"))
        );
    }

    /// The reader's narrowness, stated as a test: the shape words inside a
    /// function are not a declaration, a single-cause family is not this
    /// population, and a longer name that merely starts with a family's name is
    /// not that family's body.
    #[test]
    fn the_reader_counts_declarations_and_nothing_else() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusalIssue {\n\
             \x20   pub posture: CompletionPosture,\n\
             }\n\
             \n\
             pub struct DemoRefusal {\n\
             \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             }\n\
             \n\
             impl RefusalFamily for DemoRefusal {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n\
             \n\
             impl RefusalFamily for LadderRefusal {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::SingleCause;\n\
             }\n\
             \n\
             fn render() -> FamilyShape {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             \x20   SHAPE\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Planted reversal for the line scanner this reader replaced: a doc comment
    /// between the implementation line and the shape constant.
    ///
    /// Under a reader that only looked one line up, this family was in neither
    /// the numerator NOR the denominator — the count stayed whole while the
    /// guarded population shrank. It must be counted, and it must be judged.
    #[test]
    fn a_doc_comment_before_the_shape_still_declares_a_family() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   pub issues: NonEmptyBounded<DemoIssue, DemoIssueLimit>,\n\
             \x20   pub posture: CompletionPosture,\n\
             }\n\
             \n\
             impl RefusalFamily for DemoRefusal {\n\
             \x20   /// The shape this family reports under.\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 0);
        assert_eq!(verdict.offenders.len(), 2, "{:?}", verdict.offenders);
    }

    /// Planted reversal: a blank line between the implementation line and the
    /// shape constant, which the line scanner also lost entirely.
    #[test]
    fn a_blank_line_before_the_shape_still_declares_a_family() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             }\n\
             \n\
             impl RefusalFamily for DemoRefusal {\n\
             \n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Planted reversal: an attribute on the shape constant. The member is the
    /// same member wherever its attributes put it on the page.
    #[test]
    fn an_attribute_before_the_shape_still_declares_a_family() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   pub issues: NonEmptyBounded<DemoIssue, DemoIssueLimit>,\n\
             }\n\
             \n\
             impl RefusalFamily for DemoRefusal {\n\
             \x20   #[rustfmt::skip]\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("carries no"))
        );
    }

    /// Planted reversal: another associated constant standing ahead of the
    /// shape. Every family in the repository declares a selection order beside
    /// its shape, so an author reordering the two members is not writing
    /// anything unusual — and under the line scanner it was enough to make the
    /// family disappear.
    #[test]
    fn a_constant_before_the_shape_still_declares_a_family() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusal {\n\
             \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
             \x20   pub posture: CompletionPosture,\n\
             }\n\
             \n\
             impl RefusalFamily for DemoRefusal {\n\
             \x20   const SELECTION_ORDER: &'static [&'static str] = &[];\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("beside its issues"))
        );
    }

    /// A generic family implementation is the same declaration: the services'
    /// closure family is written that way, so a reader that could not name a
    /// generic target would drop a real family out of both counts.
    #[test]
    fn a_generic_family_implementation_is_counted() {
        let verdict = coupled_body_verdict(&source(
            "pub struct DemoRefusal<R: RenderedRole> {\n\
             \x20   pub report: AdmittedPrefix<DemoIssue<R>, DemoIssueLimit>,\n\
             }\n\
             \n\
             impl<R: RenderedRole> RefusalFamily for DemoRefusal<R>\n\
             where\n\
             \x20   R: Copy,\n\
             {\n\
             \x20   /// The shape this family reports under.\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A family declared in one home resolves its body in that home and in no
    /// other. Two homes each declaring a `pub struct Refusal` is an ordinary
    /// repository, and a reader that took the first one it walked past would
    /// hand down a verdict decided by traversal order — the coupled home would
    /// launder the loose one, or the loose one would condemn the coupled one,
    /// depending on which directory sorted first.
    #[test]
    fn each_home_resolves_only_its_own_body() {
        let coupled = "pub struct Refusal {\n\
             \x20   body: AdmittedPrefix<Issue, IssueLimit>,\n\
             }\n\
             \n\
             impl RefusalFamily for Refusal {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n";
        let loose = "pub struct Refusal {\n\
             \x20   pub issues: NonEmptyBounded<Issue, IssueLimit>,\n\
             \x20   pub posture: CompletionPosture,\n\
             }\n\
             \n\
             impl RefusalFamily for Refusal {\n\
             \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             }\n";
        let forward = coupled_body_verdict(&[
            source_at("src/00_first/types.rs", coupled),
            source_at("src/01_second/types.rs", loose),
        ]);
        let backward = coupled_body_verdict(&[
            source_at("src/01_second/types.rs", loose),
            source_at("src/00_first/types.rs", coupled),
        ]);
        assert_eq!(forward.declared, 2);
        assert_eq!(forward.coupled, 1);
        assert_eq!(forward.offenders.len(), 2, "{:?}", forward.offenders);
        assert!(
            forward
                .offenders
                .iter()
                .all(|offence| offence.starts_with("src/01_second/types.rs")),
            "{:?}",
            forward.offenders
        );
        // The verdict is the same read in the other order, which is the whole
        // claim: nothing here depends on which file the walk reached first.
        assert_eq!(backward.declared, forward.declared);
        assert_eq!(backward.coupled, forward.coupled);
        assert_eq!(backward.offenders.len(), forward.offenders.len());
    }

    /// One home is one module family across its files, because `mod.rs` joins
    /// them: the services declare the family in `type_contract.rs` and the body
    /// in `types.rs`, and a reader scoped to the FILE would report every one of
    /// those bodies missing.
    #[test]
    fn a_home_is_read_across_its_files() {
        let verdict = coupled_body_verdict(&[
            source_at(
                "macros/macroc/src/demo/types.rs",
                "pub struct DemoRefusal {\n\
                 \x20   pub report: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
                 }\n",
            ),
            source_at(
                "macros/macroc/src/demo/type_contract.rs",
                "impl RefusalFamily for DemoRefusal {\n\
                 \x20   const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
                 }\n",
            ),
        ]);
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// An inline module is its own scope inside the file that writes it, so a
    /// body one `mod` block declares does not answer for a family another one
    /// declares.
    #[test]
    fn an_inline_module_resolves_only_its_own_body() {
        let verdict = coupled_body_verdict(&source(
            "mod coupled {\n\
             \x20   pub struct Refusal {\n\
             \x20       body: AdmittedPrefix<Issue, IssueLimit>,\n\
             \x20   }\n\
             \x20   impl RefusalFamily for Refusal {\n\
             \x20       const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             \x20   }\n\
             }\n\
             \n\
             mod loose {\n\
             \x20   pub struct Refusal {\n\
             \x20       pub issues: NonEmptyBounded<Issue, IssueLimit>,\n\
             \x20   }\n\
             \x20   impl RefusalFamily for Refusal {\n\
             \x20       const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 2);
        assert_eq!(verdict.coupled, 1);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("carries no"))
        );
    }

    /// A source this reader cannot parse is a hole in the population, and it is
    /// reported as one. Silently reading it as "no families here" is the exact
    /// failure the derived denominator exists to prevent.
    #[test]
    fn an_unparsable_source_is_an_offence_rather_than_an_absence() {
        let verdict = coupled_body_verdict(&source("impl RefusalFamily for {\n"));
        assert_eq!(verdict.declared, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("not parseable Rust"))
        );
    }

    /// The real repository holds: every collection-shaped family it declares
    /// carries the coupled seat, and the derived population is real rather than
    /// empty.
    ///
    /// A gate that cannot READ its subject says it could not read its subject.
    /// Falling back to the current directory and then failing with "no sources
    /// found" reports the wrong defect: the reader would be blamed for a tree it
    /// never opened, and the real error — the one naming the path and the
    /// operating system's reason — would be gone.
    #[test]
    fn the_real_tree_couples_every_collection_body() {
        let read = repo_root()
            .map_err(|error| format!("the repository root could not be found: {error}"))
            .and_then(|root| coupling_sources(&root))
            .map(|sources| coupled_body_verdict(&sources));
        assert!(
            read.is_ok(),
            "the coupling gate could not read its subject: {read:?}"
        );
        assert!(
            read.as_ref()
                .is_ok_and(|verdict| verdict.offenders.is_empty()),
            "{read:?}"
        );
        assert!(
            read.as_ref().is_ok_and(|verdict| verdict.declared > 0),
            "no collection-shaped family found in the real tree: {read:?}"
        );
        assert!(
            read.is_ok_and(|verdict| verdict.coupled == verdict.declared),
            "the real tree declares a collection-shaped family whose body is not the coupled seat"
        );
    }
}
