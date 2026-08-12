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
    claimed_green_laws, home_readmes, red_twin_rows, tooling_red_rows,
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
    for readme in &readmes {
        let text = fs::read_to_string(readme).map_err(|e| format!("{}: {e}", readme.display()))?;
        for row in red_twin_rows(&text) {
            rows.push((row, relative_slash_path(root, readme)));
        }
    }
    let reversals = testpak_reversals(root)?;
    let ledger = red_twin_ledger(&rows, &reversals);

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
fn red_twin_ledger(rows: &[(String, String)], reversals: &[String]) -> RedTwinTally {
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
        if reversals.iter().any(|path| names_reversal(path, named)) {
            ledger.discharged = ledger.discharged.saturating_add(1);
        } else {
            ledger.offenders.push(format!(
                "{readme}: red row names `{named}`, which is no testpak test or fixture"
            ));
        }
    }
    ledger
}

/// Whether one red row's spelling names one existing reversal file. Containment
/// either way: the row may state the repository-relative path or just the file
/// name, and both name the same reversal.
fn names_reversal(path: &str, named: &str) -> bool {
    let file = path.rsplit('/').next().unwrap_or(path);
    path == named || path.contains(named) || (!file.is_empty() && named.contains(file))
}

/// Every reversal testpak carries, as repository-relative slash paths: the test
/// files under `testpak/tests/` and the compile-fail fixtures beneath them.
fn testpak_reversals(root: &Path) -> Result<Vec<String>, String> {
    let tests = root.join(JUDGE_DIRECTORY).join("tests");
    if !tests.is_dir() {
        return Ok(Vec::new());
    }
    let mut found = Vec::new();
    visit_files(&tests, &mut |path| {
        if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(relative_slash_path(root, path));
        }
        Ok(())
    })?;
    Ok(found)
}

/// Planted reversals for the join, and the real repository judged by it.
///
/// Every leg here is pure over its rows, so the reversals are fixture rows held
/// in memory: the join that guards the READMEs is never proven by editing one.
/// The tests that read the real tree are named `the_real_…` and state what they
/// found rather than what they hoped for.
#[cfg(test)]
mod tests {
    use super::{double_claimed_offences, red_twin_ledger, testpak_reversals};
    use crate::repository::readme::{
        claimed_green_laws, home_readmes, red_twin_rows, tooling_red_rows,
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
        let ledger = red_twin_ledger(&rows(text), &[]);
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
            &[String::from(
                "testpak/tests/compile-fail/a-real-fixture-that-exists.rs",
            )],
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

    /// A row naming a real reversal discharges it, whether it states the
    /// repository-relative path or only the file name.
    #[test]
    fn a_named_reversal_that_exists_is_discharged() {
        let reversals = vec![
            String::from("testpak/tests/compile-fail/a-real-fixture.rs"),
            String::from("testpak/tests/planted_defect.rs"),
        ];
        let by_path = red_twin_ledger(
            &rows("    red: testpak/tests/compile-fail/a-real-fixture.rs\n"),
            &reversals,
        );
        assert_eq!(by_path.discharged, 1);
        assert!(by_path.offenders.is_empty(), "{:?}", by_path.offenders);
        let by_name = red_twin_ledger(&rows("    red: planted_defect.rs\n"), &reversals);
        assert_eq!(by_name.discharged, 1);
        assert!(by_name.offenders.is_empty(), "{:?}", by_name.offenders);
    }

    /// The real repository holds: every named red twin resolves to a reversal
    /// that exists, and the denominator is real rather than empty.
    #[test]
    fn the_real_red_ledger_names_only_reversals_that_exist() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let reversals = testpak_reversals(&root).unwrap_or_default();
        assert!(!reversals.is_empty(), "testpak carries no reversal files");
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
            &[String::from("testpak/tests/planted_defect.rs")],
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
            &[(
                String::from("testpak/tests/nobody-wrote-this-lane.rs"),
                String::from("FIXTURE.md"),
            )],
            &[String::from("testpak/tests/planted_defect.rs")],
        );
        assert_eq!(ledger.offenders.len(), 1, "{:?}", ledger.offenders);
    }

    /// The real tooling READMEs declare a non-empty denominator, and every row
    /// naming a reversal resolves to one that exists.
    #[test]
    fn the_real_tooling_ledger_names_only_reversals_that_exist() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let reversals = testpak_reversals(&root).unwrap_or_default();
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
