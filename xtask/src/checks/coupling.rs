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
//! type opens `AdmittedPrefix<`, not a field that merely mentions one somewhere
//! inside a wrapper — and neither a loose `CompletionPosture` seat nor a loose
//! `NonEmptyBounded<…>` carry may stand beside it. A hybrid carrying the coupled
//! seat AND a loose carry is the shape a partial migration leaves behind, and it
//! is refused rather than counted, because the loose half is exactly what a
//! holder can read against another body's posture.
//!
//! The reader is deliberately dumb, and its narrowness is part of the law it
//! states. A shape declaration counts only where it sits directly under the
//! `impl … RefusalFamily for …` line that opens its block, which is how every
//! family in the repository writes it; a body counts only where `pub struct
//! <Name>` opens a braced field block. A field's type is the text after its
//! first colon, so the reader judges the seat exactly as it is spelled: it reads
//! any `NonEmptyBounded<…>` seat as a loose carry rather than joining the
//! generic arguments of two seats, and a seat spelled through a path or an alias
//! reads as neither type. What the reader does not recognize — a shape declared
//! through a type alias, a body assembled by a macro, a field whose type is
//! spelled through an alias or a module path, a field declaration broken across
//! lines — is outside this law, and this law does not pretend otherwise.

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

/// The exact line a collection-shaped family declares its shape with.
const COLLECTION_SHAPE: &str = "const SHAPE: FamilyShape = FamilyShape::IssueCollection;";

/// The impl a shape declaration must sit directly inside.
const FAMILY_IMPL: &str = " RefusalFamily for ";

/// The coupled seat: band 00's report package.
const COUPLED_SEAT: &str = "AdmittedPrefix<";

/// The posture seat a two-seat body carried beside its issues.
const POSTURE_SEAT: &str = "CompletionPosture";

/// The loose carry a two-seat body carried beside its posture — the other half
/// of the pair, which a migration that added the coupled seat without removing
/// the old one leaves standing.
const LOOSE_CARRY: &str = "NonEmptyBounded<";

/// Every collection-shaped family declared anywhere in the machine or the
/// services carries its issues and its coverage claim in one `AdmittedPrefix`
/// seat, and carries no posture seat beside it.
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
struct CouplingVerdict {
    /// Families declaring the collection shape.
    declared: usize,
    /// Families whose declared body is the one coupled seat.
    coupled: usize,
    /// Families whose body is something else, one offence each.
    offenders: Vec<String>,
}

/// Reads the declarations and the bodies out of source text and judges each
/// family.
///
/// Pure over its inputs — `(repository-relative path, source text)` pairs — so
/// the reversals below are planted in memory and the law that guards the tree is
/// never proven by editing one.
fn coupled_body_verdict(sources: &[(String, String)]) -> CouplingVerdict {
    let mut verdict = CouplingVerdict {
        declared: 0,
        coupled: 0,
        offenders: Vec::new(),
    };
    for (path, family) in declared_collection_families(sources) {
        verdict.declared = verdict.declared.saturating_add(1);
        let Some(fields) = body_fields(sources, &family) else {
            verdict.offenders.push(format!(
                "{path}: {family} declares the collection shape and no `pub struct {family}` \
                 stands in the sources"
            ));
            continue;
        };
        let coupled = fields
            .iter()
            .any(|field| field_type(field).is_some_and(|kind| kind.starts_with(COUPLED_SEAT)));
        let loose_posture = fields
            .iter()
            .any(|field| field_type(field).is_some_and(|kind| kind == POSTURE_SEAT));
        let loose_carry = fields
            .iter()
            .any(|field| field_type(field).is_some_and(|kind| kind.starts_with(LOOSE_CARRY)));
        if loose_posture {
            verdict.offenders.push(format!(
                "{path}: {family} carries a {POSTURE_SEAT} seat beside its issues; a coverage \
                 claim seated apart from its body is a claim that can be read against another \
                 body"
            ));
        }
        if coupled && loose_carry {
            verdict.offenders.push(format!(
                "{path}: {family} carries its issues twice — a {COUPLED_SEAT}…> seat and a \
                 {LOOSE_CARRY}…> seat beside it; the loose carry is half of the pair the coupled \
                 seat exists to keep together, and it can be read against another body's posture"
            ));
        }
        if !coupled {
            verdict.offenders.push(format!(
                "{path}: {family} declares the collection shape and carries no {COUPLED_SEAT}…> \
                 seat"
            ));
        }
        if coupled && !loose_posture && !loose_carry {
            verdict.coupled = verdict.coupled.saturating_add(1);
        }
    }
    verdict
}

/// Every family declaring the collection shape, with the path that declares it.
///
/// The shape line counts only directly under the `impl … RefusalFamily for …`
/// line that opens its block, so the same words inside a match arm, an
/// assertion, or a rendering table are not a declaration and are not read as
/// one.
fn declared_collection_families(sources: &[(String, String)]) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for (path, text) in sources {
        let mut previous = "";
        for line in text.lines() {
            if line.trim() == COLLECTION_SHAPE
                && let Some(family) = impl_target(previous)
            {
                found.push((path.clone(), family));
            }
            previous = line;
        }
    }
    found
}

/// The type one `impl … RefusalFamily for …` line implements the family
/// contract for, with any generic arguments cut off — `Refusal<R>` and
/// `Refusal` name one body.
fn impl_target(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with("impl") {
        return None;
    }
    let target = trimmed.split(FAMILY_IMPL).nth(1)?;
    let target = target.trim().trim_end_matches('{').trim();
    let name = target.split('<').next().unwrap_or(target).trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// The field lines of one named public struct, doc comments and attributes
/// dropped, or `None` where no such body stands in the sources.
fn body_fields(sources: &[(String, String)], family: &str) -> Option<Vec<String>> {
    let opening = format!("pub struct {family}");
    sources
        .iter()
        .find_map(|(_, text)| fields_under_opening(text, &opening))
}

/// The fields of the first body one text opens under `opening`.
fn fields_under_opening(text: &str, opening: &str) -> Option<Vec<String>> {
    let mut lines = text.lines();
    lines.find(|line| opens_body(line, opening))?;
    Some(collect_fields(lines))
}

/// Whether one line opens the named body rather than a longer name's —
/// `Construction` must not match `ConstructionIssue`.
fn opens_body(line: &str, opening: &str) -> bool {
    line.trim().strip_prefix(opening).is_some_and(|rest| {
        rest.starts_with('<') || rest.starts_with(' ') || rest.starts_with('{') || rest.is_empty()
    })
}

/// The field lines up to the body's closing brace, doc comments, attributes, and
/// blank space dropped.
fn collect_fields<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<String> {
    lines
        .take_while(|line| line.trim() != "}")
        .filter(|line| is_field(line))
        .map(|line| line.trim().to_string())
        .collect()
}

/// Whether one line inside a body is a field rather than documentation, an
/// attribute, or blank space.
fn is_field(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty() && !trimmed.starts_with("//") && !trimmed.starts_with("#[")
}

/// The declared type of one field line, or `None` where the line carries no
/// `name: type,` pair.
fn field_type(field: &str) -> Option<&str> {
    let (_, declared) = field.split_once(':')?;
    Some(declared.trim().trim_end_matches(',').trim())
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
    use std::path::PathBuf;

    /// One synthetic source file.
    fn source(text: &str) -> Vec<(String, String)> {
        vec![(String::from("FIXTURE.rs"), text.to_string())]
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
    /// seated as the body. The old reader accepted any field whose text merely
    /// mentioned the package, so a body that never carried one could read as
    /// coupled.
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

    /// The reader's narrowness, stated as a test: the shape words inside a match
    /// arm are not a declaration, a single-cause family is not this population,
    /// and a longer name that merely starts with a family's name is not that
    /// family's body.
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
             fn render(shape: FamilyShape) -> &'static str {\n\
             \x20   match shape {\n\
             \x20       const SHAPE: FamilyShape = FamilyShape::IssueCollection;\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.declared, 1);
        assert_eq!(verdict.coupled, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// The real repository holds: every collection-shaped family it declares
    /// carries the coupled seat, and the derived population is real rather than
    /// empty.
    #[test]
    fn the_real_tree_couples_every_collection_body() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let sources = coupling_sources(&root).unwrap_or_default();
        assert!(!sources.is_empty(), "no sources found to derive from");
        let verdict = coupled_body_verdict(&sources);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
        assert!(
            verdict.declared > 0,
            "no collection-shaped family found in the real tree"
        );
        assert_eq!(verdict.coupled, verdict.declared);
    }
}
