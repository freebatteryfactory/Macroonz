//! The obligations join: every claim a README makes is answered, and every
//! answer is claimed.
//!
//! An obligation row is a promise written in prose, and prose keeps reading as
//! true after the thing it describes is gone. This join is what stops that: it
//! reads the rows and the proofs from their own owners and refuses every way the
//! two can drift — a claim with no proof, a proof no claim wants, one proof
//! standing in for two claims, and a named reversal nobody wrote. What it cannot
//! refuse it COUNTS, on two denominators printed on every run, because a debt
//! that is stated out loud is a debt somebody can act on.

use std::fs;
use std::path::Path;

use crate::repository::readme::{
    claimed_green_laws, claimed_green_routes, home_readmes, red_twin_rows, tooling_red_rows,
};
use crate::repository::walk::{JUDGE_DIRECTORY, relative_slash_path, visit_files};

/// The READMEs that carry tooling qualification obligations.
///
/// A distinct population from the machine's homes: these are claims about the
/// TOOLS — what a service refuses, what a check catches, what a judge is
/// rehearsed against — and their reversals are counted on their own denominator.
const TOOLING_READMES: [&str; 2] = ["macros/macroc/README.md", "testpak/README.md"];

/// The prefix a lawful debt is spelled with: `owed-to-testpak`,
/// `owed-to-xtask-and-testpak`, and any other named creditor.
const OWED_PREFIX: &str = "owed-to";

/// The obligations join, in four legs.
///
/// **Green, both ways.** Every README obligation naming a `laws.rs` green law
/// points at a law that exists, and every law in `laws.rs` is claimed by some
/// obligation — the READMEs and the laws never drift apart.
///
/// **Green, exactly once.** No law is claimed by two obligations. A law claimed
/// twice is a proof standing in for a claim it does not make, and it reads as
/// discharged from both rows.
///
/// **Green, wherever the positive control actually sits.** A green row may name
/// a testpak seat instead of a law, because a behavioral claim's strongest seat
/// is the plane that drives it from outside. Such a row is a ROUTE, and it must
/// resolve to an EXECUTABLE seat — a test file directly under `testpak/tests/`,
/// which is to say a test that runs. A route is deliberately NOT read on the
/// terms a named red twin is: a red twin may lawfully be a compile-fail fixture,
/// and a fixture whose whole content is that it must not compile can never stand
/// as a positive control. A green route pointing at a file nobody wrote is worse
/// than an unproven claim, because it reads as proven; a green route pointing at
/// its own red twin is worse still, because the evidence it names refutes it.
/// Nothing is counted here — a green side is not a debt ledger, and inventing a
/// third denominator over a population that states no debts would be a number
/// nobody can act on.
///
/// **Red, and counted out loud.** Every obligation also declares a `red:` row.
/// A row spelled `owed-to-…` is a lawful debt: the reversal is named and not yet
/// written, and saying so is the honest state. Any other row NAMES a reversal
/// that is supposed to exist, and it must resolve to a real testpak test file or
/// compile-fail fixture — a row pointing at a reversal nobody wrote is worse
/// than an owed row, because it reads as discharged.
///
/// **Two denominators, printed apart.** The leg prints core red twins and
/// tooling reversals on their own lines, discharged over owed, on every run. The
/// numbers are meant to be uncomfortable and are meant to be watched: a
/// repository that quietly lost red twins would otherwise keep passing this
/// check while the accounting shrank.
pub(crate) fn check_obligations_join(root: &Path) -> Result<(), String> {
    let readmes = home_readmes(root)?;
    let claimed = claimed_green_laws(&readmes)?;
    let laws_path = root.join("src").join("laws.rs");
    let laws = fs::read_to_string(&laws_path).map_err(|e| format!("laws.rs: {e}"))?;
    let mut existing = Vec::new();
    let mut current_module = String::new();
    let mut previous_was_test = false;
    for line in laws.lines() {
        if let Some(rest) = line.strip_prefix("mod ")
            && let Some(module) = rest.strip_suffix(" {")
        {
            current_module = module.to_string();
        }
        if previous_was_test
            && let Some(rest) = line.trim().strip_prefix("fn ")
            && let Some(law) = rest.split('(').next()
        {
            existing.push((current_module.clone(), law.to_string()));
        }
        previous_was_test = line.trim() == "#[test]";
    }
    let mut offenders = Vec::new();
    for (module, law, readme) in &claimed {
        if !existing.iter().any(|(m, l)| m == module && l == law) {
            offenders.push(format!(
                "{} claims {module}::{law} but laws.rs has no such law",
                readme.display()
            ));
        }
    }
    for (module, law) in &existing {
        if !claimed.iter().any(|(m, l, _)| m == module && l == law) {
            offenders.push(format!(
                "laws.rs {module}::{law} is claimed by no obligation"
            ));
        }
    }
    let attributed: Vec<(String, String, String)> = claimed
        .iter()
        .map(|(module, law, readme)| {
            (
                module.clone(),
                law.clone(),
                relative_slash_path(root, readme),
            )
        })
        .collect();
    offenders.extend(double_claimed_offences(&attributed));
    let mut rows = Vec::new();
    let mut routes = Vec::new();
    for readme in &readmes {
        let text = fs::read_to_string(readme).map_err(|e| format!("{}: {e}", readme.display()))?;
        for row in red_twin_rows(&text) {
            rows.push((row, relative_slash_path(root, readme)));
        }
        for route in claimed_green_routes(&text) {
            routes.push((route, relative_slash_path(root, readme)));
        }
    }
    let (reversals, seats) = testpak_populations(root)?;
    let ledger = red_twin_ledger(&rows, &reversals);
    offenders.extend(phantom_green_routes(&routes, &seats));

    let tooling_rows = tooling_rows(root)?;
    let tooling = red_twin_ledger(&tooling_rows, &reversals);

    // TWO denominators, printed apart, always. The populations are challenged by
    // different methods and owned by different homes; one number over both would
    // be a number nobody can act on.
    println!(
        "red twins (core): {} discharged / {} owed",
        ledger.discharged, ledger.owed
    );
    println!(
        "tooling reversals: {} discharged / {} owed",
        tooling.discharged, tooling.owed
    );
    if tooling_rows.is_empty() {
        offenders.push(String::from(
            "no tooling qualification obligation declares a reversal row: the tooling denominator \
             cannot be empty while tooling exists",
        ));
    }
    offenders.extend(ledger.offenders);
    offenders.extend(tooling.offenders);
    if offenders.is_empty() {
        Ok(())
    } else {
        Err(offenders.join("; "))
    }
}

/// Every law claimed by more than one obligation, one offence per law.
///
/// Two obligations pointing at one law is two claims answered by one proof, and
/// the proof answers at most one of them. Either the pair states one claim, in
/// which case it is one obligation, or it states two, in which case the second
/// one's green half does not exist and the row is saying it does. The join
/// already refuses a law claimed by NOBODY; refusing a law claimed twice closes
/// the same door from the other side.
///
/// Pure over its inputs — `(module, law, declaring README)` triples — so the law
/// is proven against fixture rows rather than against the tree it guards.
fn double_claimed_offences(claimed: &[(String, String, String)]) -> Vec<String> {
    let mut offences = Vec::new();
    let mut reported: Vec<(String, String)> = Vec::new();
    for (module, law, _) in claimed {
        let key = (module.clone(), law.clone());
        if reported.contains(&key) {
            continue;
        }
        let claimants: Vec<&str> = claimed
            .iter()
            .filter(|(m, l, _)| m == module && l == law)
            .map(|(_, _, readme)| readme.as_str())
            .collect();
        if claimants.len() > 1 {
            reported.push(key);
            offences.push(format!(
                "laws.rs {module}::{law} is claimed by {} obligations ({}): one law proves one \
                 claim",
                claimants.len(),
                claimants.join(", ")
            ));
        }
    }
    offences
}

/// Every green route naming a positive control that no executable seat answers,
/// one offence per route.
///
/// The green side's twin of the red leg's phantom-fixture refusal, and it exists
/// for the same reason: a row that NAMES its positive control is claiming the
/// control is written, and a name nobody wrote reads as discharged from the
/// ledger while proving nothing.
///
/// The two sides resolve against DIFFERENT populations, and that separation is
/// the law here rather than an optimization. A red row names something that
/// demonstrates a REFUSAL, so it may lawfully be a compile-fail fixture. A green
/// route names an executable CONTROL, which has to actually run to control
/// anything. A fixture that by construction does not compile can never be a
/// positive control, so the green side's population is strictly narrower: the
/// seats directly under `testpak/tests/`, never the fixtures beneath them.
/// Resolving both sides against one population would let an obligation offer its
/// own red twin as its green evidence, and the join would take it.
///
/// Resolution is by exact repository-relative path, here and on the red side. A
/// route is answered by the file it names and by no other: a spelling that
/// merely CONTAINS a real path, or is CONTAINED BY one, names a seat nobody
/// wrote — which is exactly what a rename, a move, or an invented path leaves
/// behind.
///
/// Pure over its rows, so the leg is proven against fixture rows rather than by
/// editing the README it guards.
fn phantom_green_routes(rows: &[(String, String)], seats: &SeatPopulation) -> Vec<String> {
    let mut offences = Vec::new();
    for (named, readme) in rows {
        if !seats.carries(named) {
            offences.push(format!(
                "{readme}: green route names `{named}`, which is no executable test seat directly \
                 under `testpak/tests/`: a green route must name a test that RUNS, spelled as its \
                 exact repository-relative path, and a compile-fail fixture can never be one"
            ));
        }
    }
    offences
}

/// Every `tooling-red:` row the tooling READMEs declare, attributed to the file
/// that declared it.
fn tooling_rows(root: &Path) -> Result<Vec<(String, String)>, String> {
    let mut rows = Vec::new();
    for readme in TOOLING_READMES {
        let path = root.join(readme);
        if !path.is_file() {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        for row in tooling_red_rows(&text) {
            rows.push((row, relative_slash_path(root, &path)));
        }
    }
    Ok(rows)
}

/// What the red leg counted, and what it refuses.
///
/// A tally over README rows, deliberately not the plane's own
/// `RedTwinLedger`: that one accounts for reversals a qualification run
/// executed, this one for reversals the specification declares. Two
/// denominators over two populations, named apart so neither can stand in for
/// the other.
///
/// It never leaves this module: the tally is what the red leg carries between
/// its own steps, and no other law counts in this currency.
struct RedTwinTally {
    /// Rows naming a reversal that exists.
    discharged: usize,
    /// Rows declaring a named, unwritten debt.
    owed: usize,
    /// Rows naming a reversal nobody wrote.
    offenders: Vec<String>,
}

/// Reads one red row and counts it, or names it as an offence.
fn red_twin_ledger(rows: &[(String, String)], reversals: &ReversalPopulation) -> RedTwinTally {
    let mut ledger = RedTwinTally {
        discharged: 0,
        owed: 0,
        offenders: Vec::new(),
    };
    for (value, readme) in rows {
        if value.starts_with(OWED_PREFIX) {
            ledger.owed = ledger.owed.saturating_add(1);
            continue;
        }
        let named = value.split_whitespace().next().unwrap_or(value);
        if reversals.carries(named) {
            ledger.discharged = ledger.discharged.saturating_add(1);
        } else {
            ledger.offenders.push(format!(
                "{readme}: red row names `{named}`, which is no testpak test or compile-fail \
                 fixture at that exact repository-relative path"
            ));
        }
    }
    ledger
}

/// Whether one visited file is a Rust source file.
///
/// Read through `Path` rather than off the end of the string: asking the path
/// type for its extension is the reading that stays right on either platform,
/// and it is the only spelling the lint wall admits.
fn is_rust_file(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "rs")
}

/// Every reversal testpak carries, as repository-relative slash paths: the test
/// files directly under `testpak/tests/` AND the fixtures beneath them.
///
/// The RED side's population, and the wider of the two. A reversal's job is to
/// demonstrate a refusal, and a refusal is demonstrated either by a compile-fail
/// fixture that must not compile or by an executable test that plants a defect
/// and watches the judge catch it. Both are lawful answers to a red row, so both
/// are in here.
///
/// A type rather than a bare `Vec<String>` because the green leg must never be
/// handed one. Naming the two populations apart in prose leaves the wrong call
/// one keystroke away and nothing to catch it; naming them apart in the TYPES
/// makes handing the red side's population to the green leg a compile error.
struct ReversalPopulation(Vec<String>);

/// Every executable test seat testpak carries: the `.rs` files sitting DIRECTLY
/// under `testpak/tests/`, which cargo builds and runs as test binaries.
///
/// The GREEN side's population, and strictly the narrower of the two. A file one
/// level down — `compile-fail/…`, `compiled-mutant/…` — is a fixture the judge
/// feeds to a compiler on purpose, not a test cargo runs, and several of them do
/// not compile by design. Naming one as a positive control would offer a
/// negative as proof of a positive.
struct SeatPopulation(Vec<String>);

impl ReversalPopulation {
    /// Whether this population carries the reversal a red row names, resolved by
    /// EXACT repository-relative path and by nothing looser.
    fn carries(&self, named: &str) -> bool {
        self.0.iter().any(|path| path.as_str() == named)
    }
}

impl SeatPopulation {
    /// Whether this population carries the seat a green route names, resolved by
    /// EXACT repository-relative path and by nothing looser.
    ///
    /// Deliberately restated rather than shared with [`ReversalPopulation`]: the
    /// two readings agree today, and the moment one side is tempted to loosen —
    /// to accept a bare file name, a directory, a stale path — it must loosen
    /// alone, where the other side's law cannot be dragged along with it.
    fn carries(&self, named: &str) -> bool {
        self.0.iter().any(|path| path.as_str() == named)
    }
}

/// Both populations, drawn from ONE walk of `testpak/tests/`.
///
/// One walk rather than two, so the red side and the green side can never be
/// judging different trees — the same rule the repository's walker exists to
/// enforce. The green population is the red one narrowed by depth, and that
/// containment is a fact of this function rather than a hope about two of them.
fn testpak_populations(root: &Path) -> Result<(ReversalPopulation, SeatPopulation), String> {
    let tests = root.join(JUDGE_DIRECTORY).join("tests");
    if !tests.is_dir() {
        return Ok((ReversalPopulation(Vec::new()), SeatPopulation(Vec::new())));
    }
    let mut reversals = Vec::new();
    let mut seats = Vec::new();
    visit_files(&tests, &mut |path| {
        if is_rust_file(path) {
            let spelled = relative_slash_path(root, path);
            if path.parent() == Some(tests.as_path()) {
                seats.push(spelled.clone());
            }
            reversals.push(spelled);
        }
        Ok(())
    })?;
    Ok((ReversalPopulation(reversals), SeatPopulation(seats)))
}

/// Planted reversals for the join, and the real repository judged by it.
///
/// Every leg here is pure over its rows, so the reversals are fixture rows held
/// in memory: the join that guards the READMEs is never proven by editing one.
/// The tests that read the real tree are named `the_real_…` and state what they
/// found rather than what they hoped for.
#[cfg(test)]
mod tests {
    use super::{
        ReversalPopulation, SeatPopulation, double_claimed_offences, phantom_green_routes,
        red_twin_ledger, testpak_populations,
    };
    use crate::repository::readme::{
        claimed_green_laws, claimed_green_routes, home_readmes, red_twin_rows, tooling_red_rows,
    };
    use crate::repository::walk::{relative_slash_path, repo_root};
    use std::fs;
    use std::path::PathBuf;

    /// One synthetic README's rows, attributed to a fixture file name.
    fn rows(readme_text: &str) -> Vec<(String, String)> {
        red_twin_rows(readme_text)
            .into_iter()
            .map(|value| (value, String::from("FIXTURE.md")))
            .collect()
    }

    /// One synthetic row naming a route or reversal, attributed to a fixture.
    fn named(paths: &[&str]) -> Vec<(String, String)> {
        paths
            .iter()
            .map(|path| ((*path).to_string(), String::from("FIXTURE.md")))
            .collect()
    }

    /// One synthetic RED population.
    fn red_population(paths: &[&str]) -> ReversalPopulation {
        ReversalPopulation(paths.iter().map(|path| (*path).to_string()).collect())
    }

    /// One synthetic GREEN population.
    fn green_population(paths: &[&str]) -> SeatPopulation {
        SeatPopulation(paths.iter().map(|path| (*path).to_string()).collect())
    }

    /// One synthetic claim row.
    fn claim(module: &str, law: &str, readme: &str) -> (String, String, String) {
        (module.to_string(), law.to_string(), readme.to_string())
    }

    /// An owed row is lawful and counts as owed, whoever the named creditor is.
    #[test]
    fn an_owed_row_is_counted_not_refused() {
        let text = "    red: owed-to-testpak\n\
                        red: owed-to-xtask-and-testpak\n\
                        red: owed-to-testpak — cloning a Budget must not compile\n";
        let ledger = red_twin_ledger(&rows(text), &red_population(&[]));
        assert_eq!(ledger.owed, 3);
        assert_eq!(ledger.discharged, 0);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
    }

    /// Planted reversal: a row naming a reversal nobody wrote. This is the
    /// failure the leg exists for — it reads as discharged and is not.
    #[test]
    fn a_phantom_fixture_name_is_a_violation() {
        let text = "    red: testpak/tests/compile-fail/nobody-ever-wrote-this.rs\n";
        let ledger = red_twin_ledger(
            &rows(text),
            &red_population(&["testpak/tests/compile-fail/a-real-fixture-that-exists.rs"]),
        );
        assert_eq!(ledger.discharged, 0);
        assert_eq!(ledger.owed, 0);
        assert_eq!(ledger.offenders.len(), 1, "{:?}", ledger.offenders);
        assert!(
            ledger
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("nobody-ever-wrote-this.rs"))
        );
    }

    /// The positive control for the red side: a row naming a real reversal by
    /// its exact repository-relative path discharges it, and a compile-fail
    /// fixture is a lawful answer to a red row.
    #[test]
    fn a_named_reversal_that_exists_is_discharged() {
        let reversals = red_population(&[
            "testpak/tests/compile-fail/a-real-fixture.rs",
            "testpak/tests/planted_defect.rs",
        ]);
        let fixture = red_twin_ledger(
            &rows("    red: testpak/tests/compile-fail/a-real-fixture.rs\n"),
            &reversals,
        );
        assert_eq!(fixture.discharged, 1);
        assert!(fixture.offenders.is_empty(), "{:?}", fixture.offenders);

        let executable = red_twin_ledger(
            &rows("    red: testpak/tests/planted_defect.rs\n"),
            &reversals,
        );
        assert_eq!(executable.discharged, 1);
        assert!(
            executable.offenders.is_empty(),
            "{:?}",
            executable.offenders
        );
    }

    /// Planted reversal: the red-side twin of the green side's defect. A row
    /// spelling a STALE path whose tail is a real file name, and a row spelling
    /// only a bare file name, both resolved under the containment reading this
    /// leg used to carry — and both counted as DISCHARGED while naming nothing
    /// the repository holds.
    ///
    /// This is the load-bearing half: a discharged row is subtracted from a debt
    /// the whole campaign reports, so a loose resolver here does not merely wave
    /// a bad row through, it shrinks the published denominator.
    #[test]
    fn a_red_row_naming_a_stale_or_bare_path_is_a_violation() {
        let reversals = red_population(&["testpak/tests/planted_defect.rs"]);

        // A move or a rename leaves this behind: the tail is real, the path is
        // not. Containment either way used to read it as discharged.
        let stale = red_twin_ledger(
            &rows("    red: testpak/old/planted_defect.rs\n"),
            &reversals,
        );
        assert_eq!(stale.discharged, 0, "a stale path discharged a red row");
        assert_eq!(stale.owed, 0);
        assert_eq!(stale.offenders.len(), 1, "{:?}", stale.offenders);

        // A directory names no file at all, and every file beneath it used to
        // "contain" the spelling and answer for it.
        let directory = red_twin_ledger(
            &rows("    red: testpak/tests/ — one fixture per road\n"),
            &reversals,
        );
        assert_eq!(directory.discharged, 0, "a directory discharged a red row");
        assert_eq!(directory.offenders.len(), 1, "{:?}", directory.offenders);

        // A bare file name is not a repository-relative path.
        let bare = red_twin_ledger(&rows("    red: planted_defect.rs\n"), &reversals);
        assert_eq!(bare.discharged, 0, "a bare name discharged a red row");
        assert_eq!(bare.offenders.len(), 1, "{:?}", bare.offenders);
    }

    /// The real repository holds: every named red twin resolves to a reversal
    /// that exists, and the denominator is real rather than empty.
    #[test]
    fn the_real_red_ledger_names_only_reversals_that_exist() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let (reversals, _) = testpak_populations(&root)
            .unwrap_or_else(|_| (red_population(&[]), green_population(&[])));
        assert!(!reversals.0.is_empty(), "testpak carries no reversal files");
        let mut collected = Vec::new();
        let readmes = home_readmes(&root).unwrap_or_default();
        assert!(!readmes.is_empty(), "no home READMEs found");
        for readme in &readmes {
            let text = fs::read_to_string(readme).unwrap_or_default();
            let name = readme.display().to_string();
            for value in red_twin_rows(&text) {
                collected.push((value, name.clone()));
            }
        }
        let ledger = red_twin_ledger(&collected, &reversals);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
        assert!(
            ledger.owed > 0,
            "no owed red twins found; the ledger cannot be empty here"
        );
    }

    /// A tooling row is read off the trimmed line and counted on its OWN
    /// ledger, never folded into the core one. An `owed-to-…` tooling row is a
    /// lawful debt exactly as a core one is.
    #[test]
    fn a_tooling_row_is_read_and_counted_apart() {
        let text = "  tooling-red: testpak/tests/planted_defect.rs\n\
                    red: owed-to-testpak\n\
                    tooling-red: owed-to-testpak — the structural lane\n";
        let found = tooling_red_rows(text);
        assert_eq!(found.len(), 2, "{found:?}");
        assert_eq!(red_twin_rows(text).len(), 1);
        let attributed: Vec<(String, String)> = found
            .into_iter()
            .map(|row| (row, String::from("FIXTURE.md")))
            .collect();
        let ledger = red_twin_ledger(
            &attributed,
            &red_population(&["testpak/tests/planted_defect.rs"]),
        );
        assert_eq!(ledger.discharged, 1);
        assert_eq!(ledger.owed, 1);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
    }

    /// Planted reversal: a tooling row naming a reversal nobody wrote. It reads
    /// as discharged and is not.
    #[test]
    fn a_phantom_tooling_reversal_is_a_violation() {
        let ledger = red_twin_ledger(
            &named(&["testpak/tests/nobody-wrote-this-lane.rs"]),
            &red_population(&["testpak/tests/planted_defect.rs"]),
        );
        assert_eq!(ledger.offenders.len(), 1, "{:?}", ledger.offenders);
    }

    /// The real tooling READMEs declare a non-empty denominator, and every row
    /// naming a reversal resolves to one that exists.
    #[test]
    fn the_real_tooling_ledger_names_only_reversals_that_exist() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let (reversals, _) = testpak_populations(&root)
            .unwrap_or_else(|_| (red_population(&[]), green_population(&[])));
        let mut collected = Vec::new();
        for readme in ["macros/macroc/README.md", "testpak/README.md"] {
            let text = fs::read_to_string(root.join(readme)).unwrap_or_default();
            for row in tooling_red_rows(&text) {
                collected.push((row, String::from(readme)));
            }
        }
        assert!(!collected.is_empty(), "no tooling reversal rows found");
        let ledger = red_twin_ledger(&collected, &reversals);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
        assert!(ledger.owed > 0, "the tooling ledger claims no debt at all");
    }

    /// Planted reversal: a green route naming a positive control nobody wrote,
    /// and one whose name is a letter off.
    ///
    /// This is the failure the green leg exists for. Both rows read as proven
    /// from the ledger, and neither is: the first names a file that was never
    /// written, the second names one that was written under another spelling —
    /// which is what a rename or a move leaves behind.
    #[test]
    fn a_phantom_green_route_is_a_violation() {
        let seats = green_population(&["testpak/tests/stamp_row_ceiling.rs"]);
        let vanished =
            phantom_green_routes(&named(&["testpak/tests/nobody-ever-wrote-this.rs"]), &seats);
        assert_eq!(vanished.len(), 1, "{vanished:?}");
        assert!(
            vanished
                .first()
                .is_some_and(|offence| offence.contains("nobody-ever-wrote-this.rs"))
        );

        let misspelled =
            phantom_green_routes(&named(&["testpak/tests/stamp_row_ceilings.rs"]), &seats);
        assert_eq!(misspelled.len(), 1, "{misspelled:?}");
    }

    /// Planted reversal: a green route offering a COMPILE-FAIL FIXTURE as its
    /// positive control.
    ///
    /// The most serious spelling of the defect, and the one a resolver fix alone
    /// does not reach: the fixture exists, its path is exact, and every
    /// path-matching reading in the world says yes. Only the narrowed population
    /// refuses it. A fixture beneath `compile-fail/` does not compile by
    /// construction, so a row naming one is offering its own red twin — a proof
    /// of REFUSAL — as proof that the behavior works.
    #[test]
    fn a_green_route_naming_a_fixture_is_a_violation() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let (reversals, seats) = testpak_populations(&root)
            .unwrap_or_else(|_| (red_population(&[]), green_population(&[])));

        for fixture in [
            "testpak/tests/compile-fail/a-discarded-refusal.rs",
            "testpak/tests/compiled-mutant/shape-altered.rs",
        ] {
            // The file is REAL, and the red side rightly takes it: naming it as
            // a reversal discharges a red row. Exactness is not what refuses it.
            let discharged = red_twin_ledger(&named(&[fixture]), &reversals);
            assert_eq!(
                discharged.discharged, 1,
                "{fixture} is not a reversal the red side carries"
            );

            // The same real file, offered as a positive control, is refused —
            // and only the narrowed population can refuse it.
            let offered = phantom_green_routes(&named(&[fixture]), &seats);
            assert_eq!(
                offered.len(),
                1,
                "{fixture} stood as a green positive control: {offered:?}"
            );
        }
    }

    /// Planted reversal: a green route spelled loosely rather than exactly —
    /// a STALE path whose tail is a real seat name, and a BARE file name.
    ///
    /// Both resolved under the containment reading this leg used to carry. Each
    /// reads as a written, running positive control and names nothing that runs.
    #[test]
    fn a_green_route_spelled_loosely_is_a_violation() {
        let seats = green_population(&["testpak/tests/stamp_row_ceiling.rs"]);

        let stale = phantom_green_routes(&named(&["testpak/old/stamp_row_ceiling.rs"]), &seats);
        assert_eq!(stale.len(), 1, "a stale path stood as a route: {stale:?}");

        let bare = phantom_green_routes(&named(&["stamp_row_ceiling.rs"]), &seats);
        assert_eq!(bare.len(), 1, "a bare name stood as a route: {bare:?}");

        // The mirror of the stale case: a real seat whose path CONTAINS the
        // declared spelling. `row_ceiling.rs` names no seat anyone declared.
        let contained = phantom_green_routes(&named(&["row_ceiling.rs"]), &seats);
        assert_eq!(
            contained.len(),
            1,
            "a fragment of a real path stood as a route: {contained:?}"
        );
    }

    /// The positive control: a green route naming a real executable seat by its
    /// exact repository-relative path is lawful. A leg that flagged everything
    /// would satisfy every reversal above and be worthless.
    #[test]
    fn a_green_route_that_exists_is_lawful() {
        let seats = green_population(&[
            "testpak/tests/stamp_row_ceiling.rs",
            "testpak/tests/planted_defect.rs",
        ]);
        let found = phantom_green_routes(
            &named(&[
                "testpak/tests/stamp_row_ceiling.rs",
                "testpak/tests/planted_defect.rs",
            ]),
            &seats,
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// The real repository holds: every green route a home README declares
    /// resolves to an EXECUTABLE seat that exists, and the population is real
    /// rather than empty.
    #[test]
    fn the_real_green_routes_name_only_seats_that_exist() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let (_, seats) = testpak_populations(&root)
            .unwrap_or_else(|_| (red_population(&[]), green_population(&[])));
        let mut collected = Vec::new();
        let readmes = home_readmes(&root).unwrap_or_default();
        for readme in &readmes {
            let text = fs::read_to_string(readme).unwrap_or_default();
            let name = relative_slash_path(&root, readme);
            for route in claimed_green_routes(&text) {
                collected.push((route, name.clone()));
            }
        }
        assert!(
            !collected.is_empty(),
            "no green route found; this leg would be guarding nothing"
        );
        let found = phantom_green_routes(&collected, &seats);
        assert!(found.is_empty(), "{found:?}");
    }

    /// The real repository holds: the two populations are genuinely two.
    ///
    /// The separation is only worth something if the tree actually carries
    /// fixtures the green side must refuse. If testpak ever flattened — every
    /// reversal sitting directly under `testpak/tests/` — the narrowing would
    /// still be law but nothing would exercise it, and this test says so out
    /// loud rather than passing quietly.
    #[test]
    fn the_real_populations_are_named_apart() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let (reversals, seats) = testpak_populations(&root)
            .unwrap_or_else(|_| (red_population(&[]), green_population(&[])));
        assert!(!seats.0.is_empty(), "testpak carries no executable seat");
        assert!(
            seats.0.len() < reversals.0.len(),
            "the green population is not narrower than the red one: {} seats, {} reversals",
            seats.0.len(),
            reversals.0.len()
        );
        // Every seat is one directory deep, and the red side carries all of
        // them: the green population is the red one narrowed, never a second
        // reading of the tree that could drift from it.
        for seat in &seats.0 {
            assert!(
                reversals.0.contains(seat),
                "{seat} is an executable seat the reversal population does not carry"
            );
            assert_eq!(
                seat.matches('/').count(),
                2,
                "{seat} is not directly under testpak/tests/"
            );
        }
        let fixtures: Vec<&str> = reversals
            .0
            .iter()
            .filter(|path| path.matches('/').count() > 2)
            .map(String::as_str)
            .collect();
        assert!(
            !fixtures.is_empty(),
            "testpak carries no fixture beneath testpak/tests/; the narrowing guards nothing"
        );
        let offered = phantom_green_routes(&named(&fixtures), &seats);
        assert_eq!(
            offered.len(),
            fixtures.len(),
            "a real fixture stood as a green positive control: {offered:?}"
        );
    }

    /// Planted reversal: two obligations pointing at one law. The second row's
    /// green half does not exist, and both rows read as discharged.
    #[test]
    fn a_law_claimed_by_two_obligations_is_a_violation() {
        let doubled = [
            claim(
                "bounds",
                "charge_shrinks_or_refuses",
                "src/05_bounds/README.md",
            ),
            claim(
                "bounds",
                "charge_shrinks_or_refuses",
                "src/05_bounds/README.md",
            ),
        ];
        let found = double_claimed_offences(&doubled);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.first().is_some_and(|offence| {
            offence.contains("charge_shrinks_or_refuses") && offence.contains("2 obligations")
        }));

        // Two homes claiming one law is the same offence, and it is reported
        // once rather than once per claimant.
        let across_homes = [
            claim(
                "evidence",
                "coverage_is_unordered",
                "src/23_evidence/README.md",
            ),
            claim("evidence", "coverage_is_unordered", "README.md"),
        ];
        assert_eq!(double_claimed_offences(&across_homes).len(), 1);
    }

    /// The positive control: distinct laws, and one law NAME reused under two
    /// different modules, are both lawful. A check that flagged everything would
    /// satisfy the reversal above and be worthless.
    #[test]
    fn distinct_laws_and_a_reused_law_name_are_lawful() {
        let distinct = [
            claim(
                "bounds",
                "charge_shrinks_or_refuses",
                "src/05_bounds/README.md",
            ),
            claim("bounds", "budget_is_affine", "src/05_bounds/README.md"),
            claim(
                "bytes",
                "decode_maxima_are_sixteen",
                "src/07_bytes/README.md",
            ),
            claim(
                "bytes",
                "width_conventions_are_eight",
                "src/07_bytes/README.md",
            ),
        ];
        assert!(double_claimed_offences(&distinct).is_empty());

        // The join key is module AND law: `bounds::roster_is_closed` and
        // `bytes::roster_is_closed` are two laws in two sections.
        let same_name = [
            claim("bounds", "roster_is_closed", "src/05_bounds/README.md"),
            claim("bytes", "roster_is_closed", "src/07_bytes/README.md"),
        ];
        assert!(double_claimed_offences(&same_name).is_empty());
    }

    /// The real repository holds: every green law it claims is claimed by
    /// exactly one obligation.
    #[test]
    fn the_real_obligations_claim_each_law_once() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let readmes = home_readmes(&root).unwrap_or_default();
        let claimed = claimed_green_laws(&readmes).unwrap_or_default();
        assert!(!claimed.is_empty(), "no green obligations found");
        let attributed: Vec<(String, String, String)> = claimed
            .iter()
            .map(|(module, law, readme)| {
                (
                    module.clone(),
                    law.clone(),
                    relative_slash_path(&root, readme),
                )
            })
            .collect();
        let found = double_claimed_offences(&attributed);
        assert!(found.is_empty(), "{found:?}");
    }
}
