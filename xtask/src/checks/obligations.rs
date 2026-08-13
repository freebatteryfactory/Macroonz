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
    classify_green_rows, home_readmes, red_twin_rows, tooling_red_rows,
};
use crate::repository::types::GreenRow;
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

/// The attribute the test harness collects a function by.
const HARNESS_ATTRIBUTE: &str = "test";

/// The attribute that leaves a collected test in the binary and stops a plain
/// run from executing it.
const SKIP_ATTRIBUTE: &str = "ignore";

/// The attribute a build reads a condition off, and the one place this reader
/// looks at a condition at all.
const CONDITION_ATTRIBUTE: &str = "cfg";

/// The predicate that holds when one of its alternatives holds — and so, over
/// NO alternatives, holds in no build there is.
const EMPTY_DISJUNCTION: &str = "any";

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
/// **Green, every row read, by ONE reader.** A green row states its positive
/// control in one of three spellings — a `laws.rs` target, a path to a file, or
/// a declared disposition accounting for why no file holds one — and every row
/// is classified as one of them or reported. Reading only the spellings a leg
/// can use is how a row that lost its suffix, or its value, leaves the
/// population without anything refusing it: the obligation then qualifies while
/// the positive control it names is never looked for, which is the failure this
/// whole boundary exists to prevent, one level up in the reader.
///
/// Every README is read ONCE and classified once, and the claims this join
/// resolves are the seats that reading produced — not a second pass with a
/// prefix of its own. Two readers over one population is the same defect
/// arriving through the door the classifier was built to shut: the strict reader
/// this join used to call matched `"green: laws.rs "` literally, so a row
/// spelled `green:laws.rs …`, or with a tab, or with two spaces, was seated by
/// the classifier and claimed by nobody. It answered no leg, it failed no count
/// — the classifier had already counted it as read — and the obligation
/// qualified while naming a law that does not exist. One read rather than two,
/// so the seat and the claim can never be about different rows: a fact of this
/// function rather than a hope about two of them.
///
/// **Green, wherever the positive control actually sits.** A green row may name
/// a testpak seat rather than a `laws.rs` one, because a behavioral claim's
/// strongest seat is the plane that drives it from outside. Such a row is a
/// ROUTE, and it must resolve to an EXECUTABLE seat — a file directly under
/// `testpak/tests/` that DECLARES at least one `#[test]` a plain harness run
/// EXECUTES, which is to say a test that is written and is not skipped. A route
/// is deliberately NOT read on the terms a named red twin is:
/// a red twin may lawfully be a compile-fail fixture, and a fixture whose whole
/// content is that it must not compile can never stand as a positive control.
///
/// Depth alone could not say that, and the gap it left is why the seat
/// population is read out of a PARSE. A `.rs` file at the top of
/// `testpak/tests/` with no test function in it is built by cargo into a test
/// binary exactly as its neighbours are; that binary runs zero tests. Narrowed
/// by depth alone the file was a seat, so a route naming it read as a positive
/// control that executes while nothing executed — the same defect as offering a
/// compile-fail fixture, arriving one level up, through a population that was
/// narrowed by where a file SITS rather than by what it declares. A file whose
/// every test carries `#[ignore]` runs zero tests in exactly the same way, one
/// attribute deeper, and a file whose every test is removed from the binary by a
/// `cfg` that is false in every build runs zero tests one attribute further out
/// still. The parse is where all of those are read.
///
/// A green route pointing at a file nobody wrote is worse than an unproven
/// claim, because it reads as proven; a green route pointing at its own red
/// twin, or at an empty test binary, is worse still, because the evidence it
/// names establishes nothing at all.
///
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
    let laws_path = root.join("src").join("laws.rs");
    let laws = fs::read_to_string(&laws_path).map_err(|e| format!("laws.rs: {e}"))?;
    let existing = declared_laws(&laws);
    let mut claimed = Vec::new();
    let mut rows = Vec::new();
    let mut routes = Vec::new();
    let mut unreadable = Vec::new();
    for readme in &readmes {
        let text = fs::read_to_string(readme).map_err(|e| format!("{}: {e}", readme.display()))?;
        let home = relative_slash_path(root, readme);
        for row in red_twin_rows(&text) {
            rows.push((row, home.clone()));
        }
        for row in classify_green_rows(&text) {
            match row {
                GreenRow::CompileTimeSeat { module, law } => {
                    claimed.push((module, law, home.clone()));
                }
                GreenRow::Route(named) => routes.push((named, home.clone())),
                GreenRow::Unreadable(value) => unreadable.push((value, home.clone())),
                GreenRow::Disposition => {}
            }
        }
    }
    let mut offenders = drifted_claim_offences(&claimed, &existing);
    offenders.extend(double_claimed_offences(&claimed));
    let judge = testpak_populations(root)?;
    let ledger = red_twin_ledger(&rows, &judge.reversals);
    offenders.extend(phantom_green_routes(&routes, &judge.seats));
    offenders.extend(unreadable_green_offences(&unreadable));
    offenders.extend(judge.unparsable);

    let tooling_rows = tooling_rows(root)?;
    let tooling = red_twin_ledger(&tooling_rows, &judge.reversals);

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

/// Every `#[test]` law `laws.rs` declares, as `(module, law)` in file order.
///
/// A law is a `#[test]` function inside a `mod`, so the reading is the pair: the
/// module last opened at the crate root, and the function the attribute sits
/// on. Read as text rather than parsed, because what the join needs is the two
/// names, and a reader that needed a syntax tree to learn them would be a second
/// opinion about the same file.
fn declared_laws(laws_text: &str) -> Vec<(String, String)> {
    let mut declared = Vec::new();
    let mut current_module = String::new();
    let mut previous_was_test = false;
    for line in laws_text.lines() {
        if let Some(rest) = line.strip_prefix("mod ")
            && let Some(module) = rest.strip_suffix(" {")
        {
            current_module = module.to_string();
        }
        if previous_was_test
            && let Some(rest) = line.trim().strip_prefix("fn ")
            && let Some(law) = rest.split('(').next()
        {
            declared.push((current_module.clone(), law.to_string()));
        }
        previous_was_test = line.trim() == "#[test]";
    }
    declared
}

/// Where the READMEs and `laws.rs` have drifted apart, in both directions: a
/// claim naming a law that does not exist, and a law no obligation claims.
///
/// Both halves or neither. A claim nobody can resolve reads as proven and is
/// not; a law nobody claims is a proof standing in the tree with no obligation
/// admitting it exists, which is how a law survives the removal of the thing it
/// was written for. Refusing only the first would let the READMEs shrink
/// quietly, and refusing only the second would let them claim anything.
///
/// The offence names the README as this repository spells paths — relative, with
/// forward slashes — so a run on any machine names the same file.
///
/// Pure over its inputs, so the leg is proven against fixture rows rather than
/// by editing a README the repository stands on.
fn drifted_claim_offences(
    claimed: &[(String, String, String)],
    existing: &[(String, String)],
) -> Vec<String> {
    let mut offences = Vec::new();
    for (module, law, readme) in claimed {
        if !existing.iter().any(|(m, l)| m == module && l == law) {
            offences.push(format!(
                "{readme} claims {module}::{law} but laws.rs has no such law"
            ));
        }
    }
    for (module, law) in existing {
        if !claimed.iter().any(|(m, l, _)| m == module && l == law) {
            offences.push(format!(
                "laws.rs {module}::{law} is claimed by no obligation"
            ));
        }
    }
    offences
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
/// the rule here rather than an optimization. A red row names something that
/// demonstrates a REFUSAL, so it may lawfully be a compile-fail fixture. A green
/// route names an executable CONTROL, which has to actually run to control
/// anything. So the green side's population is strictly narrower, and it is
/// narrowed twice: a file beneath `testpak/tests/` is a fixture rather than a
/// seat, and a file AT the top that declares no test is a test binary with
/// nothing in it. Resolving both sides against one population would let an
/// obligation offer its own red twin as its green evidence, and the join would
/// take it; resolving the green side by depth alone let it offer an empty
/// binary, and the join took that.
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
                "{readme}: green route names `{named}`, which is no executable test seat: a green \
                 route must name a file directly under `testpak/tests/` that DECLARES at least \
                 one `#[test]` a plain harness run EXECUTES, spelled as its exact \
                 repository-relative path. A compile-fail fixture can never be one; neither can a \
                 top-level file that builds a test binary with no test in it; neither can one \
                 whose every test carries `#[ignore]`, which a plain run reports as ignored while \
                 it finishes with nothing executed; and neither can one whose every test is \
                 compiled out of every build there is by a `cfg` that needs no build to decide it \
                 — `any()` over no alternatives, or the literal `false` — written on the test or \
                 on a module around it"
            ));
        }
    }
    offences
}

/// Every green row no lawful spelling reads, one offence per row.
///
/// The leg that stops a malformed row from leaving the population quietly. A
/// route that lost a letter off its suffix, a bare word that is neither a path
/// nor a declared disposition, a value somebody emptied — each of these used to
/// be filtered out one level up, before [`phantom_green_routes`] ever saw it, so
/// the obligation qualified with its named positive control unexamined and
/// nothing said a word. That is the same defect the phantom-route leg refuses,
/// arriving through the reader instead of through the join, and it is worse for
/// it: the phantom leg at least reports the row it cannot resolve.
///
/// The offence names the row AND the README that declared it, because the row's
/// value is not enough to find it — the same spelling can be written in any
/// home, and the repair is made in the file that wrote it.
///
/// Pure over its rows, so the leg is proven against fixture rows rather than by
/// malforming a README the repository stands on.
fn unreadable_green_offences(rows: &[(String, String)]) -> Vec<String> {
    let mut offences = Vec::new();
    for (value, readme) in rows {
        let spelled = if value.is_empty() {
            String::from("no value at all")
        } else {
            format!("`{value}`")
        };
        offences.push(format!(
            "{readme}: green row states {spelled}, which is no spelling this repository reads: a \
             green row states `laws.rs <module>::<name>` and nothing after it, or a \
             repository-relative path to a `.rs` file, or a declared disposition — `none — …`, \
             `owed — …`, `structural (…)` — accounting for why no file holds a positive control. \
             A row nobody can read is an obligation whose positive control nobody looks for"
        ));
    }
    offences
}

/// Every `tooling-red:` row the tooling READMEs declare, attributed to the file
/// that declared it.
///
/// A declared README that is not there REFUSES rather than being stepped over.
/// [`TOOLING_READMES`] is a statement that these files carry the tooling
/// denominator, so one of them going missing takes its rows out of the published
/// count with nothing said — and the emptiness guard downstream cannot see it,
/// because the other file's rows keep the population non-empty. A ledger that
/// shrinks quietly is the failure this whole join exists to refuse.
fn tooling_rows(root: &Path) -> Result<Vec<(String, String)>, String> {
    let mut rows = Vec::new();
    for readme in TOOLING_READMES {
        let path = root.join(readme);
        if !path.is_file() {
            return Err(format!(
                "{readme} is declared as a tooling obligation ledger and is not there: its rows \
                 would leave the tooling denominator with nothing saying so"
            ));
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
/// its own steps, and nothing else counts in this currency.
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
        let Some(named) = value.split_whitespace().next() else {
            ledger.offenders.push(format!(
                "{readme}: red row states no value at all: a row either declares its debt as \
                 `owed-to-…` with the creditor named, or names the reversal that exists"
            ));
            continue;
        };
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
/// under `testpak/tests/` that DECLARE at least one `#[test]`.
///
/// The GREEN side's population, and strictly the narrower of the two, narrowed
/// on two different facts because one of them was never enough.
///
/// Placement is the first. A file one level down — `compile-fail/…`,
/// `compiled-mutant/…` — is a fixture the judge feeds to a compiler on purpose,
/// not a test cargo runs, and several of them do not compile by design. Naming
/// one as a positive control would offer a negative as proof of a positive.
///
/// What the file DECLARES is the second, and it is the one placement could not
/// answer. Cargo builds every `.rs` at the top of `tests/` into a test binary
/// whether or not a test is written in it, so a file holding only helpers,
/// only a module declaration, or nothing at all sits exactly where a seat sits
/// and runs zero tests. Admitted by depth, such a file stood as a positive
/// control that executes, and nothing executed. A file whose every test is
/// `#[ignore]`d is the same empty run written one attribute deeper, and one
/// whose every test is removed by a `cfg` false in every build — `any()`, the
/// literal `false` — is that same empty run written one attribute further out
/// still, on the test or on a module around it. A seat here is a file that
/// declares a test the harness RUNS, established from a parse of it.
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
    /// alone, where the other side's reading cannot be dragged along with it.
    fn carries(&self, named: &str) -> bool {
        self.0.iter().any(|path| path.as_str() == named)
    }
}

/// Everything one walk of `testpak/tests/` established: both populations, and
/// every top-level source the seat reading could not read.
struct JudgeTree {
    /// The RED side's population: every reversal the tree carries.
    reversals: ReversalPopulation,
    /// The GREEN side's population: every top-level source that declares a
    /// test.
    seats: SeatPopulation,
    /// Top-level sources that are not parseable Rust, one offence each. Never a
    /// skip: whether such a file declares a test is UNKNOWN rather than false,
    /// and a hole in the green population reported as nothing is the silence
    /// this whole boundary exists to end.
    unparsable: Vec<String>,
}

/// Both populations, drawn from ONE walk of `testpak/tests/`.
///
/// One walk rather than two, so the red side and the green side can never be
/// judging different trees — the same rule the repository's walker exists to
/// enforce. The green population is the red one narrowed, and that containment
/// is a fact of this function rather than a hope about two of them.
///
/// Only the top-level sources are read as text, because the seat question is
/// asked of them alone. A fixture beneath them is answered by its placement and
/// its contents are none of this reader's business — several of them do not
/// parse, and do not compile, by design.
fn testpak_populations(root: &Path) -> Result<JudgeTree, String> {
    let tests = root.join(JUDGE_DIRECTORY).join("tests");
    if !tests.is_dir() {
        return Ok(JudgeTree {
            reversals: ReversalPopulation(Vec::new()),
            seats: SeatPopulation(Vec::new()),
            unparsable: Vec::new(),
        });
    }
    let mut reversals = Vec::new();
    let mut top_level = Vec::new();
    visit_files(&tests, &mut |path| {
        if is_rust_file(path) {
            let spelled = relative_slash_path(root, path);
            if path.parent() == Some(tests.as_path()) {
                let text =
                    fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
                top_level.push((spelled.clone(), text));
            }
            reversals.push(spelled);
        }
        Ok(())
    })?;
    let (seats, unparsable) = seat_population(&top_level);
    Ok(JudgeTree {
        reversals: ReversalPopulation(reversals),
        seats,
        unparsable,
    })
}

/// The seats among the top-level sources, and the ones whose seat question could
/// not be answered.
///
/// Pure over `(repository-relative path, source text)` pairs, so the reversal
/// for the narrowing is a source held in memory: the leg that decides what
/// counts as a positive control is never proven by writing an empty test file
/// into the judge.
fn seat_population(sources: &[(String, String)]) -> (SeatPopulation, Vec<String>) {
    let mut seats = Vec::new();
    let mut unparsable = Vec::new();
    for (path, text) in sources {
        match syn::parse_file(text) {
            Ok(file) => {
                if declares_a_runnable_test(&file.items) {
                    seats.push(path.clone());
                }
            }
            Err(error) => unparsable.push(format!(
                "{path} sits directly under `testpak/tests/` and is not parseable Rust, so whether \
                 it declares a test that RUNS is unknown rather than false: {error}"
            )),
        }
    }
    (SeatPopulation(seats), unparsable)
}

/// Whether one parsed source declares at least one test cargo will RUN.
///
/// The question is about ITEMS: that a function is declared, that the test
/// harness's own attribute sits ON it, and that nothing beside that attribute
/// tells the harness to skip it. A text search for `#[test]` answers a
/// different question and answers it wrongly in both directions — it says yes to
/// the attribute written inside a doc comment, a string literal, or a
/// commented-out block, each of which is a file with no test in it, and it is
/// the class of reader this repository has already replaced twice. There is one
/// reading, and it is the parse.
///
/// An inline module is entered, because cargo's harness collects a test wherever
/// it is declared inside the binary.
///
/// # An ignored test is not a test that runs
///
/// `#[ignore]` is not a comment on a test; it is an instruction to the harness.
/// A plain `cargo test` collects the function, reports it as ignored, and
/// finishes having executed it zero times — reaching it takes `--ignored` or
/// `--include-ignored`, and no stage of this repository's entry bar passes
/// either. A file whose every test carries the attribute therefore builds a
/// binary that runs nothing, which is the empty-binary defect one attribute
/// deeper: the route naming it reads as a positive control that executes while
/// nothing executes. The attribute is a fact about the FUNCTION it sits on, so a
/// file carrying an ignored test beside a live one is still a seat.
///
/// # A condition false in EVERY build is not a condition
///
/// A test the compiler removes from every binary it could ever produce runs
/// nowhere, and the file holding only such tests builds a binary that runs zero
/// tests — the empty-binary defect again, one attribute further out. Two
/// spellings say it without asking anything about the build, and both are read
/// here, on the test itself or on a module around it, because a module the
/// compiler removes takes its tests with it.
///
/// # Two ceilings, and they fail in OPPOSITE directions
///
/// A `mod name;` reaching a SEPARATE file is not followed, and no macro is
/// expanded, so a seat whose only tests arrive that way is not admitted here.
/// That direction fails CLOSED: the route is refused, its author is told exactly
/// what is missing, and nothing reads as proven that is not.
///
/// The claim's ceiling, and it fails OPEN: a CONDITIONAL `cfg` is not evaluated,
/// on the precedent the coupling reader states. A
/// `#[cfg(feature = "…")] #[test]` compiled out of the binary is still read here
/// as declared, so a route may resolve to a seat whose only test does not exist
/// in the binary that runs — and a `#[cfg_attr(…, ignore)]` is not read as a
/// skip for the same reason. That direction admits what it cannot establish,
/// which is why it is written down here rather than left to a reader. It stays
/// open for a reason that has not changed: evaluating a conditional `cfg` means
/// resolving features, targets and profiles against the build that will actually
/// run, and a second, weaker evaluator written inside a check is a reader nobody
/// could trust either.
///
/// What NARROWED is the part that needed no build to decide. `any()` and the
/// literal `false` resolve nothing — no feature, no target, no profile, no
/// evaluator — so refusing them is a reading of the attribute rather than a
/// second opinion about a build, and this reader is not a `cfg` evaluator after
/// it. Everything the ceiling ever covered that a build DECIDES is still open,
/// and `the_cfg_ceiling_is_open_and_says_so` is where that is stated with an
/// assertion on it.
///
/// What closes the rest is a stronger seat than a parse can reach: a green route
/// naming the test FUNCTION it is controlled by, resolved against the roster a
/// qualification run EXECUTED. That reading answers both ceilings at once — a
/// test arriving through a separate file or a macro appears in the roster, one
/// compiled out does not, and an ignored one is reported as ignored — and it
/// needs a row that can name a function and a run that publishes what it ran.
/// Neither exists today. It is the versioned claim and evidence schema's opening
/// condition, and it is not built here.
///
fn declares_a_runnable_test(items: &[syn::Item]) -> bool {
    items.iter().any(|item| {
        if let syn::Item::Fn(declared) = item {
            !compiled_out_of_every_build(&declared.attrs) && runs_under_the_harness(&declared.attrs)
        } else if let syn::Item::Mod(module) = item {
            !compiled_out_of_every_build(&module.attrs)
                && module
                    .content
                    .as_ref()
                    .is_some_and(|(_, inner)| declares_a_runnable_test(inner))
        } else {
            false
        }
    })
}

/// Whether one item's attributes remove it from every build there is.
///
/// Asked of a FUNCTION and of a MODULE, through one reading, because the
/// compiler removes both the same way and a module removed takes every test
/// inside it along. Asking it only of the function would seat a file whose whole
/// test module is compiled out, which is the same empty binary reached one
/// nesting level up.
fn compiled_out_of_every_build(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(states_no_build_at_all)
}

/// Whether one attribute is a `cfg` whose predicate is false in every build.
///
/// Two spellings, and the boundary around them is the whole point of this
/// reader. Both are false by DEFINITION of what is written, so neither asks a
/// question about the build that will run:
///
/// - `any()` is the disjunction over no alternatives. `any` holds when one of
///   its alternatives holds, so over none of them it holds nowhere; there is no
///   flag that turns it on, and passing `--cfg any` does not, because `any` is
///   the operator rather than a name. Measured on this repository's pinned
///   toolchain: the item is gated out and a build naming it does not resolve.
/// - `false` is the boolean literal. rustc states its own reading of it —
///   "makes this predicate evaluate to `false` unconditionally" — and there is
///   no command line that makes a literal something else.
///
/// A file whose only tests carry either one compiles clean under this
/// repository's exact lint wall and runs `0 tests`, which is the state this
/// narrowing exists to refuse.
///
/// # What is deliberately NOT read, and why
///
/// `#[cfg(FALSE)]` is a bare cfg NAME, not a false predicate, and it is measured
/// rather than assumed: a build passing `--cfg FALSE` compiles the item in.
/// Reading it as unconditionally false would be this reader claiming a fact
/// about the build, which is exactly the ceiling above. It is not silence
/// either — the workspace denies warnings, cargo's own `--check-cfg` reports an
/// unexpected condition name for it, and the build refuses it before any check
/// runs. A reversal already owned elsewhere is not one this join re-owns.
///
/// Nothing composed is read. `all(any())`, `not(all())` and their relatives are
/// false too, and reading them is where a reader stops reading an attribute and
/// starts evaluating a predicate language. The line is drawn at the predicate
/// written in the attribute, and it stays there.
fn states_no_build_at_all(attribute: &syn::Attribute) -> bool {
    if !attribute_is(attribute, CONDITION_ATTRIBUTE) {
        return false;
    }
    let syn::Meta::List(condition) = &attribute.meta else {
        return false;
    };
    let literally_false =
        syn::parse2::<syn::LitBool>(condition.tokens.clone()).is_ok_and(|stated| !stated.value);
    literally_false
        || syn::parse2::<syn::Meta>(condition.tokens.clone()).is_ok_and(|stated| match stated {
            syn::Meta::List(alternatives) => {
                alternatives.path.is_ident(EMPTY_DISJUNCTION) && alternatives.tokens.is_empty()
            }
            syn::Meta::Path(_) | syn::Meta::NameValue(_) => false,
        })
}

/// Whether one function's attributes make it a test the harness EXECUTES.
///
/// Two facts about one function, and the second is not the negation of the
/// first: the harness's own attribute puts the function in the binary, and the
/// skip leaves it there and stops it running. A reading that took only the first
/// counts a function nothing executes.
fn runs_under_the_harness(attributes: &[syn::Attribute]) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute_is(attribute, HARNESS_ATTRIBUTE))
        && !attributes
            .iter()
            .any(|attribute| attribute_is(attribute, SKIP_ATTRIBUTE))
}

/// Whether one attribute is the named one.
///
/// Read by the path's LAST SEGMENT, on the precedent the coupling reader sets:
/// `#[test]` and `#[core::prelude::v1::test]` are one attribute, and a longer
/// name ending in the word is not. ONE reading serves the harness's attribute
/// and the harness's skip rather than two readers spelled alike: the pair is
/// only meaningful read on the same terms, and a second reader is where the two
/// would drift apart.
///
/// The attribute's VALUE is never looked at, and that is what makes `#[ignore]`
/// and `#[ignore = "reason"]` one attribute here — syn hands the first as a bare
/// path and the second as that same path carrying a value, so a reason string
/// hides nothing.
fn attribute_is(attribute: &syn::Attribute, word: &str) -> bool {
    attribute
        .path()
        .segments
        .last()
        .is_some_and(|last| last.ident == word)
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
        JudgeTree, OWED_PREFIX, ReversalPopulation, SeatPopulation, declared_laws,
        double_claimed_offences, drifted_claim_offences, phantom_green_routes, red_twin_ledger,
        seat_population, testpak_populations, tooling_rows, unreadable_green_offences,
    };
    use crate::repository::readme::{
        classify_green_rows, home_readmes, red_twin_rows, tooling_red_rows,
    };
    use crate::repository::types::GreenRow;
    use crate::repository::walk::{relative_slash_path, repo_root};
    use std::fs;
    use std::path::{Path, PathBuf};

    /// One synthetic `laws.rs` declaring exactly one law.
    const ONE_LAW: &str = "mod root {\n    #[test]\n    fn a_law_somebody_wrote() {}\n}\n";

    /// One synthetic README's rows, attributed to a fixture file name.
    fn rows(readme_text: &str) -> Vec<(String, String)> {
        red_twin_rows(readme_text)
            .into_iter()
            .map(|value| (value, String::from("FIXTURE.md")))
            .collect()
    }

    /// The claims one README's green rows make, read the one way the join reads
    /// them and attributed to the home that wrote them.
    fn claims_in(readme_text: &str, home: &str) -> Vec<(String, String, String)> {
        classify_green_rows(readme_text)
            .into_iter()
            .filter_map(|row| match row {
                GreenRow::CompileTimeSeat { module, law } => {
                    Some((module, law, String::from(home)))
                }
                GreenRow::Disposition | GreenRow::Route(_) | GreenRow::Unreadable(_) => None,
            })
            .collect()
    }

    /// The claims one synthetic README's green rows make, attributed to a
    /// fixture file name.
    fn seat_claims(readme_text: &str) -> Vec<(String, String, String)> {
        claims_in(readme_text, "FIXTURE.md")
    }

    /// Every claim the real repository's home READMEs make, attributed exactly
    /// as the join attributes them.
    fn real_claims(root: &Path) -> Vec<(String, String, String)> {
        let mut claimed = Vec::new();
        for readme in home_readmes(root).unwrap_or_default() {
            let text = fs::read_to_string(&readme).unwrap_or_default();
            claimed.extend(claims_in(&text, &relative_slash_path(root, &readme)));
        }
        claimed
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

    /// The real judge tree, or empty populations where it could not be read.
    fn real_tree(root: &Path) -> JudgeTree {
        testpak_populations(root).unwrap_or_else(|_| JudgeTree {
            reversals: red_population(&[]),
            seats: green_population(&[]),
            unparsable: Vec::new(),
        })
    }

    /// One synthetic top-level source, at a path directly under
    /// `testpak/tests/`.
    fn top_level(name: &str, text: &str) -> (String, String) {
        (format!("testpak/tests/{name}"), text.to_string())
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
        let reversals = real_tree(&root).reversals;
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

    /// Planted reversal: a declared tooling ledger that is not there. Stepped
    /// over, its rows leave the published tooling denominator and the emptiness
    /// guard never fires, because the other declared ledger keeps the population
    /// non-empty.
    ///
    /// Read against a directory that is not the repository, which is every
    /// declared ledger missing at once — the same reading the first missing one
    /// gets, since the leg refuses on the first.
    #[test]
    fn a_missing_tooling_ledger_is_a_violation() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let elsewhere = root.join("xtask").join("src");
        let found = tooling_rows(&elsewhere);
        assert!(found.is_err(), "{found:?}");
        assert!(
            found
                .err()
                .is_some_and(|offence| offence.contains("tooling obligation ledger")),
        );
    }

    /// The real tooling READMEs declare a non-empty denominator, and every row
    /// naming a reversal resolves to one that exists.
    #[test]
    fn the_real_tooling_ledger_names_only_reversals_that_exist() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let reversals = real_tree(&root).reversals;
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
        let JudgeTree {
            reversals, seats, ..
        } = real_tree(&root);

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

    /// Planted reversal: top-level sources that declare NO test, offered as
    /// green routes.
    ///
    /// The defect a depth reading cannot see, and the one a path resolver never
    /// reaches. Each file sits exactly where a seat sits, is spelled exactly as
    /// a seat is spelled, and is built by cargo into a test binary of its own —
    /// a binary that runs zero tests. Narrowed by depth alone, all four stood as
    /// positive controls, so the obligations naming them read as controlled by
    /// something that executes while nothing executed.
    ///
    /// The third and fourth are why the reading is a PARSE. Both spell the
    /// harness attribute in their bytes and neither declares a test: one inside
    /// a doc comment, one inside a string literal. Any substring search for
    /// `#[test]` seats both of them, which is the same class of reader the
    /// coupling law replaced and the same class this repository has now paid for
    /// twice.
    #[test]
    fn a_top_level_file_declaring_no_test_is_no_seat() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "a_helper_only_file.rs",
                "fn shared_helper() -> u8 {\n    7\n}\n",
            ),
            top_level("an_empty_file.rs", ""),
            top_level(
                "a_documented_file.rs",
                "/// Sketching what a seat here would look like:\n\
                 ///\n\
                 /// ```text\n\
                 /// #[test]\n\
                 /// fn one_day() {}\n\
                 /// ```\n\
                 pub fn nothing_runs() {}\n",
            ),
            top_level(
                "a_quoting_file.rs",
                "const SPELLED: &str = \"#[test]\\nfn one_day() {}\";\n\
                 fn quoted() -> &'static str {\n    SPELLED\n}\n",
            ),
        ]);
        assert!(seats.0.is_empty(), "{:?}", seats.0);
        assert!(unparsable.is_empty(), "{unparsable:?}");

        let offered = phantom_green_routes(
            &named(&[
                "testpak/tests/a_helper_only_file.rs",
                "testpak/tests/an_empty_file.rs",
                "testpak/tests/a_documented_file.rs",
                "testpak/tests/a_quoting_file.rs",
            ]),
            &seats,
        );
        assert_eq!(offered.len(), 4, "{offered:?}");
        assert!(
            offered
                .iter()
                .all(|offence| offence.contains("no test in it")),
            "{offered:?}"
        );
    }

    /// The positive control: a top-level source that declares a test IS a seat,
    /// whether the declaration stands at the top of the file or inside a module
    /// the file writes, and a route naming it is lawful. A reading that seated
    /// nothing would satisfy the reversal above and be worthless.
    #[test]
    fn a_top_level_file_declaring_a_test_is_a_seat() {
        let (seats, unparsable) = seat_population(&[
            top_level("a_plain_seat.rs", "#[test]\nfn the_behaviour_holds() {}\n"),
            top_level(
                "a_seat_inside_a_module.rs",
                "mod behaviour {\n\
                 \x20   #[test]\n\
                 \x20   fn the_behaviour_holds() {}\n\
                 }\n",
            ),
        ]);
        assert_eq!(seats.0.len(), 2, "{:?}", seats.0);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        let found = phantom_green_routes(
            &named(&[
                "testpak/tests/a_plain_seat.rs",
                "testpak/tests/a_seat_inside_a_module.rs",
            ]),
            &seats,
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// Planted reversal: a top-level source whose only test is `#[ignore]`d,
    /// written the ways an author writes it — bare, with a reason, with the two
    /// attributes in either order, and inside a module the file declares.
    ///
    /// The empty-binary defect one attribute deeper, and the one a reader
    /// counting `#[test]` cannot see: cargo collects every function below,
    /// reports it as ignored, and finishes having executed nothing. Reaching any
    /// of them takes `--ignored` or `--include-ignored`, which no stage of this
    /// repository's entry bar passes, so a route naming one of these files reads
    /// as a positive control that executes while nothing executes.
    ///
    /// The reason string is the half a value-reading reader would miss. `syn`
    /// hands `#[ignore]` as a bare path and `#[ignore = "…"]` as that path
    /// carrying a value, and only a reading that ignores the value sees one
    /// attribute in both.
    #[test]
    fn a_top_level_file_whose_every_test_is_ignored_is_no_seat() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "an_ignored_seat.rs",
                "#[test]\n#[ignore]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "an_ignored_seat_with_a_reason.rs",
                "#[test]\n\
                 #[ignore = \"owed until the roster lands\"]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "an_ignore_written_first.rs",
                "#[ignore]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "an_ignored_seat_inside_a_module.rs",
                "mod behaviour {\n\
                 \x20   #[test]\n\
                 \x20   #[ignore = \"owed until the roster lands\"]\n\
                 \x20   fn the_behaviour_holds() {}\n\
                 }\n",
            ),
        ]);
        assert!(seats.0.is_empty(), "{:?}", seats.0);
        assert!(unparsable.is_empty(), "{unparsable:?}");

        let offered = phantom_green_routes(
            &named(&[
                "testpak/tests/an_ignored_seat.rs",
                "testpak/tests/an_ignored_seat_with_a_reason.rs",
                "testpak/tests/an_ignore_written_first.rs",
                "testpak/tests/an_ignored_seat_inside_a_module.rs",
            ]),
            &seats,
        );
        assert_eq!(offered.len(), 4, "{offered:?}");
        assert!(
            offered
                .iter()
                .all(|offence| offence.contains("carries `#[ignore]`")),
            "{offered:?}"
        );
    }

    /// The positive control for that narrowing: a file carrying an ignored test
    /// BESIDE a live one is still a seat, and so is one whose live test stands
    /// in a module while the ignored one does not.
    ///
    /// A reader that refused a file for containing the skip anywhere would
    /// satisfy the reversal above and would throw away real seats. The attribute
    /// is a fact about the FUNCTION it sits on, and a file is a seat when the
    /// harness runs SOMETHING in it.
    #[test]
    fn a_live_test_beside_an_ignored_one_is_still_a_seat() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "a_mixed_seat.rs",
                "#[test]\n\
                 #[ignore = \"owed until the roster lands\"]\n\
                 fn the_slow_road() {}\n\
                 \n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_mixed_seat_across_modules.rs",
                "#[test]\n\
                 #[ignore]\n\
                 fn the_slow_road() {}\n\
                 \n\
                 mod behaviour {\n\
                 \x20   #[test]\n\
                 \x20   fn the_behaviour_holds() {}\n\
                 }\n",
            ),
        ]);
        assert_eq!(seats.0.len(), 2, "{:?}", seats.0);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        let found = phantom_green_routes(
            &named(&[
                "testpak/tests/a_mixed_seat.rs",
                "testpak/tests/a_mixed_seat_across_modules.rs",
            ]),
            &seats,
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// The reader's ceiling, stated as a test because this one fails OPEN — and
    /// stated now on exactly the part of it that is still open.
    ///
    /// What CLOSED is the part no build decides. A `cfg` spelled `any()` or
    /// `false` is false by definition of what is written, so refusing it takes
    /// no features, no target and no profile, and the reversal that pins it is
    /// `a_test_compiled_out_of_every_build_is_no_seat`.
    ///
    /// What remains open is everything a BUILD decides, and every case below is
    /// one of those:
    ///
    /// - a feature nobody enables here — but somebody's build might, and this
    ///   reader is not told which build will run;
    /// - a target predicate, true on one machine and false on the next;
    /// - `#[cfg(FALSE)]`, which reads like a closed case and is not one. It is a
    ///   bare cfg NAME, and a build passing `--cfg FALSE` compiles the item in;
    ///   that is measured rather than assumed. It is also not unwatched: this
    ///   workspace denies warnings and cargo's own `--check-cfg` reports an
    ///   unexpected condition name, so the build refuses the spelling before any
    ///   check reads it;
    /// - `#[cfg_attr(…, ignore)]`, where the skip itself is conditional, so a
    ///   test ignored under a condition still seats its file.
    ///
    /// This is written as an assertion rather than left in prose so the ceiling
    /// is where the reader ACTUALLY stands rather than where a comment says it
    /// does. Closing the rest is a route resolved against the roster a
    /// qualification run executed, which the versioned claim and evidence schema
    /// opens, and a pass that closes it fails this test by name and repairs it
    /// in one place.
    #[test]
    fn the_cfg_ceiling_is_open_and_says_so() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "a_test_compiled_out_by_a_feature.rs",
                "#[cfg(feature = \"a-feature-nobody-enables\")]\n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_test_compiled_out_by_a_target.rs",
                "#[cfg(target_os = \"an-os-nobody-builds-for\")]\n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_test_gated_on_a_bare_name.rs",
                "#[cfg(FALSE)]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_test_skipped_under_a_condition.rs",
                "#[test]\n\
                 #[cfg_attr(windows, ignore)]\n\
                 fn the_behaviour_holds() {}\n",
            ),
        ]);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        assert_eq!(
            seats.0.len(),
            4,
            "the cfg ceiling has moved: this reader admits what a BUILD decides, and the doc that \
             states so must move with it: {:?}",
            seats.0
        );
    }

    /// Planted reversal: a top-level source whose only test is removed from
    /// every build there is, written the ways it gets written — the two
    /// spellings, either attribute order, and the gate on an enclosing module
    /// rather than on the test.
    ///
    /// The empty-binary defect one attribute further out than `#[ignore]`, and
    /// the more convincing of the two: an ignored test at least appears in the
    /// run as ignored, while these are not in the binary at all. Each file below
    /// compiles clean under this repository's exact lint wall and its harness
    /// reports `0 tests`, so a route naming one read as a positive control that
    /// executes while the compiled harness contained nothing to execute.
    ///
    /// The module cases are why the reading is asked of items rather than of
    /// functions. A gate on a module removes every test inside it, so a reader
    /// looking only at the test's own attributes seats a file whose whole test
    /// module the compiler threw away.
    #[test]
    fn a_test_compiled_out_of_every_build_is_no_seat() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "an_empty_disjunction.rs",
                "#[cfg(any())]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_literal_false.rs",
                "#[cfg(false)]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_condition_written_second.rs",
                "#[test]\n#[cfg(any())]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_module_compiled_out.rs",
                "#[cfg(any())]\n\
                 mod behaviour {\n\
                 \x20   #[test]\n\
                 \x20   fn the_behaviour_holds() {}\n\
                 }\n",
            ),
            top_level(
                "a_module_compiled_out_by_a_literal.rs",
                "#[cfg(false)]\n\
                 mod behaviour {\n\
                 \x20   #[test]\n\
                 \x20   fn the_behaviour_holds() {}\n\
                 }\n",
            ),
        ]);
        assert!(seats.0.is_empty(), "{:?}", seats.0);
        assert!(unparsable.is_empty(), "{unparsable:?}");

        let offered = phantom_green_routes(
            &named(&[
                "testpak/tests/an_empty_disjunction.rs",
                "testpak/tests/a_literal_false.rs",
                "testpak/tests/a_condition_written_second.rs",
                "testpak/tests/a_module_compiled_out.rs",
                "testpak/tests/a_module_compiled_out_by_a_literal.rs",
            ]),
            &seats,
        );
        assert_eq!(offered.len(), 5, "{offered:?}");
        assert!(
            offered
                .iter()
                .all(|offence| offence.contains("compiled out of every build")),
            "{offered:?}"
        );
    }

    /// The positive control for that narrowing, in both directions it can go
    /// wrong.
    ///
    /// A file carrying a compiled-out test BESIDE a live one is still a seat,
    /// and so is one whose live test stands inside a module while a compiled-out
    /// module sits beside it: the condition is a fact about the item it is
    /// written on, and a file is a seat when the harness runs SOMETHING in it.
    ///
    /// The last two are the over-reach this reader must not commit. `all()` is
    /// the conjunction over no requirements and is therefore vacuously TRUE, and
    /// `true` is the literal that says so; a reader that saw an empty predicate
    /// list and refused would throw away tests that run in every build. A reader
    /// that flagged everything, or that read `all()` as `any()`, would satisfy
    /// the reversal above and be worse than worthless.
    #[test]
    fn a_live_test_beside_a_compiled_out_one_is_still_a_seat() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "a_mixed_seat.rs",
                "#[cfg(any())]\n\
                 #[test]\n\
                 fn the_road_not_built() {}\n\
                 \n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_mixed_seat_across_modules.rs",
                "#[cfg(false)]\n\
                 mod not_built {\n\
                 \x20   #[test]\n\
                 \x20   fn the_road_not_built() {}\n\
                 }\n\
                 \n\
                 mod behaviour {\n\
                 \x20   #[test]\n\
                 \x20   fn the_behaviour_holds() {}\n\
                 }\n",
            ),
            top_level(
                "an_empty_conjunction_is_true.rs",
                "#[cfg(all())]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_literal_true.rs",
                "#[cfg(true)]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
        ]);
        assert_eq!(seats.0.len(), 4, "{:?}", seats.0);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        let found = phantom_green_routes(
            &named(&[
                "testpak/tests/a_mixed_seat.rs",
                "testpak/tests/a_mixed_seat_across_modules.rs",
                "testpak/tests/an_empty_conjunction_is_true.rs",
                "testpak/tests/a_literal_true.rs",
            ]),
            &seats,
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// Planted reversal: a top-level source that is not parseable Rust.
    ///
    /// Whether it declares a test is UNKNOWN, and the two honest halves are both
    /// taken: it is not seated, and the hole is named. Reading it as "no test
    /// here" and saying nothing would put the reader's own failure into the
    /// green population as a silent verdict about the file.
    #[test]
    fn an_unparsable_top_level_source_is_named_not_assumed() {
        let (seats, unparsable) = seat_population(&[top_level(
            "a_broken_seat.rs",
            "#[test]\nfn the_behaviour_holds( {\n",
        )]);
        assert!(seats.0.is_empty(), "{:?}", seats.0);
        assert_eq!(unparsable.len(), 1, "{unparsable:?}");
        assert!(
            unparsable
                .first()
                .is_some_and(|offence| offence.contains("unknown rather than false")),
            "{unparsable:?}"
        );
    }

    /// Planted reversal: a green row no lawful spelling reads, in each of the
    /// four ways a row goes unreadable.
    ///
    /// Every one of these was dropped by a reader that filtered to the rows it
    /// could use, so none of them ever reached the phantom-route leg. The offence
    /// names the row and the README that declared it, because the repair is made
    /// in the file that wrote it.
    #[test]
    fn an_unreadable_green_row_is_an_offence_against_its_readme() {
        let found = unreadable_green_offences(&named(&[
            "testpak/tests/stamp_row_ceiling.r",
            "testpak/tests/stamp_row_ceiling.txt",
            "sometimes",
            "",
        ]));
        assert_eq!(found.len(), 4, "{found:?}");
        assert!(
            found
                .iter()
                .all(|offence| offence.starts_with("FIXTURE.md:")),
            "{found:?}"
        );
        assert!(
            found
                .first()
                .is_some_and(|offence| offence.contains("stamp_row_ceiling.r`")),
            "{found:?}"
        );
        // An emptied row has no spelling to quote, so it is reported by what it
        // is rather than by an empty pair of backticks nobody can search for.
        assert!(
            found
                .last()
                .is_some_and(|offence| offence.contains("no value at all")),
            "{found:?}"
        );
    }

    /// The positive control: a reader that classified every row as unreadable
    /// would satisfy the reversal above and be worthless, so the lawful
    /// spellings are read through the same leg and produce nothing.
    #[test]
    fn the_lawful_green_spellings_are_no_offence() {
        let text = "    green: laws.rs root::a_seat_that_exists\n\
                    \x20   green: none — the type's nonexistence is what refuses\n\
                    \x20   green: owed — executable when the roster lands\n\
                    \x20   green: structural (a phantom makes the handle !Send)\n\
                    \x20   green: testpak/tests/stamp_row_ceiling.rs\n";
        let mut routes = Vec::new();
        let mut unreadable = Vec::new();
        for row in classify_green_rows(text) {
            match row {
                GreenRow::Route(named) => routes.push((named, String::from("FIXTURE.md"))),
                GreenRow::Unreadable(value) => unreadable.push((value, String::from("FIXTURE.md"))),
                GreenRow::CompileTimeSeat { .. } | GreenRow::Disposition => {}
            }
        }
        assert_eq!(routes.len(), 1, "{routes:?}");
        assert!(unreadable_green_offences(&unreadable).is_empty());
        let seats = green_population(&["testpak/tests/stamp_row_ceiling.rs"]);
        assert!(phantom_green_routes(&routes, &seats).is_empty());
    }

    /// Planted reversal: a red row whose value was emptied. It is refused rather
    /// than counted, and rather than dropped — a row that stops being read
    /// shrinks a denominator this repository publishes on every run.
    #[test]
    fn an_emptied_red_row_is_refused_not_counted() {
        let ledger = red_twin_ledger(
            &rows("    red:\n    red: owed-to-testpak\n"),
            &red_population(&["testpak/tests/planted_defect.rs"]),
        );
        assert_eq!(ledger.owed, 1);
        assert_eq!(ledger.discharged, 0);
        assert_eq!(ledger.offenders.len(), 1, "{:?}", ledger.offenders);
        assert!(
            ledger
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("no value at all")),
            "{:?}",
            ledger.offenders
        );
    }

    /// The real repository holds: every green row a home README declares is
    /// read as one of the three lawful spellings, every route resolves to an
    /// EXECUTABLE seat that exists, and nothing is dropped on the way.
    ///
    /// The last of those is the load-bearing one and it is why the count is
    /// taken twice. The rows are counted first by the READER, then by the raw
    /// line prefix the rows are written with, and the two numbers must agree:
    /// that is the statement that no green row left the population unclassified,
    /// stated over the real tree rather than over a fixture.
    #[test]
    fn the_real_green_rows_are_all_read() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let seats = real_tree(&root).seats;
        let mut routes = Vec::new();
        let mut unreadable = Vec::new();
        let mut seated = 0usize;
        let mut disposed = 0usize;
        let mut written = 0usize;
        let readmes = home_readmes(&root).unwrap_or_default();
        assert!(!readmes.is_empty(), "no home READMEs found");
        for readme in &readmes {
            let text = fs::read_to_string(readme).unwrap_or_default();
            let name = relative_slash_path(&root, readme);
            written = written.saturating_add(
                text.lines()
                    .filter(|line| line.trim().starts_with("green:"))
                    .count(),
            );
            for row in classify_green_rows(&text) {
                match row {
                    GreenRow::Route(named) => routes.push((named, name.clone())),
                    GreenRow::Unreadable(value) => unreadable.push((value, name.clone())),
                    GreenRow::CompileTimeSeat { .. } => seated = seated.saturating_add(1),
                    GreenRow::Disposition => disposed = disposed.saturating_add(1),
                }
            }
        }
        let offences = unreadable_green_offences(&unreadable);
        assert!(offences.is_empty(), "{offences:?}");
        assert!(
            !routes.is_empty(),
            "no green route found; the route leg would be guarding nothing"
        );
        // All three spellings are actually written here. A classifier is only
        // worth something over a population that exercises it.
        assert!(seated > 0, "no green row names a compile-time seat");
        assert!(disposed > 0, "no green row declares a disposition");
        let read = routes
            .len()
            .saturating_add(unreadable.len())
            .saturating_add(seated)
            .saturating_add(disposed);
        assert_eq!(read, written, "a green row was written and not read");
        let found = phantom_green_routes(&routes, &seats);
        assert!(found.is_empty(), "{found:?}");
    }

    /// The real repository holds: the two populations are genuinely two.
    ///
    /// The separation is only worth something if the tree actually carries
    /// fixtures the green side must refuse. If testpak ever flattened — every
    /// reversal sitting directly under `testpak/tests/` — the narrowing would
    /// still be enforced but nothing would exercise it, and this test says so
    /// out loud rather than passing quietly.
    ///
    /// Every top-level source is also READ, and read successfully. A file the
    /// parser could not open is not a file with no test in it; it is a seat
    /// question nobody answered, and it is reported rather than counted either
    /// way.
    #[test]
    fn the_real_populations_are_named_apart() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let JudgeTree {
            reversals,
            seats,
            unparsable,
        } = real_tree(&root);
        assert!(unparsable.is_empty(), "{unparsable:?}");
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
        let claimed = real_claims(&root);
        assert!(!claimed.is_empty(), "no green obligations found");
        let found = double_claimed_offences(&claimed);
        assert!(found.is_empty(), "{found:?}");
    }

    /// Planted reversal: an obligation claiming a law nobody wrote, spelled the
    /// four ways a SECOND reader matching `"green: laws.rs "` could not see —
    /// no space after the colon, two spaces, a tab before `laws.rs`, and a tab
    /// between `laws.rs` and its target.
    ///
    /// Read end to end, classifier into join leg, because the defect lived in
    /// the gap between them and nowhere else. Each row was dropped by the strict
    /// reader before the join, and seated by the classifier at the same time, so
    /// the obligation qualified while naming a law that does not exist and the
    /// count of rows read still matched the count of rows written. There is one
    /// reader now, and each of these arrives at the leg that refuses it.
    #[test]
    fn a_seat_the_strict_reader_dropped_still_reaches_the_join() {
        let existing = declared_laws(ONE_LAW);
        assert_eq!(existing.len(), 1, "{existing:?}");
        for spelled in [
            "    green:laws.rs root::a_law_nobody_wrote\n",
            "    green:  laws.rs root::a_law_nobody_wrote\n",
            "    green:\tlaws.rs root::a_law_nobody_wrote\n",
            "    green: laws.rs\troot::a_law_nobody_wrote\n",
        ] {
            let claimed = seat_claims(spelled);
            assert_eq!(claimed.len(), 1, "{spelled:?} was not seated: {claimed:?}");
            let found = drifted_claim_offences(&claimed, &existing);
            assert!(
                found
                    .iter()
                    .any(|offence| offence.contains("claims root::a_law_nobody_wrote")),
                "{spelled:?}: {found:?}"
            );
        }
    }

    /// Planted reversal: a seat row carrying a token AFTER its target, read end
    /// to end — classifier into join leg — because the defect lived in the gap
    /// between them.
    ///
    /// The trailing token used to be discarded. The row then resolved the real
    /// law, satisfied the drift leg in both directions, and qualified while
    /// stating something no reader in this repository accounts for — a second
    /// target somebody meant to add, a note, half of a rename. It arrives now at
    /// the leg that names it against the README that wrote it, and the law it
    /// used to claim is left claimed by nobody, which the join refuses from the
    /// other side. One row spelled wrong is answered twice rather than passing
    /// twice.
    #[test]
    fn a_seat_row_carrying_more_than_its_target_reaches_the_join() {
        let spelled = "    green: laws.rs root::a_law_somebody_wrote and a word nobody read\n";
        let claimed = seat_claims(spelled);
        assert!(
            claimed.is_empty(),
            "the trailing token was discarded and the row still claimed a law: {claimed:?}"
        );

        let unreadable: Vec<(String, String)> = classify_green_rows(spelled)
            .into_iter()
            .filter_map(|row| match row {
                GreenRow::Unreadable(value) => Some((value, String::from("FIXTURE.md"))),
                GreenRow::CompileTimeSeat { .. } | GreenRow::Disposition | GreenRow::Route(_) => {
                    None
                }
            })
            .collect();
        let found = unreadable_green_offences(&unreadable);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found
                .first()
                .is_some_and(|offence| offence.contains("a word nobody read")),
            "{found:?}"
        );

        // The law the discarded token used to leave claimed is now claimed by
        // nobody, and the drift leg says so rather than letting the README shrink
        // quietly.
        let drifted = drifted_claim_offences(&claimed, &declared_laws(ONE_LAW));
        assert!(
            drifted
                .iter()
                .any(|offence| offence.contains("claimed by no obligation")),
            "{drifted:?}"
        );
    }

    /// The real repository holds: the tooling ledgers' name-then-prose rows all
    /// resolve, and there are still several of them.
    ///
    /// Stated apart from the ledger test above because this is the convention the
    /// green side deliberately does NOT share. A green compile-time target is one
    /// token and a second token makes the row unreadable; a red row names its
    /// reversal and then says what the reversal does. A pass that carried the
    /// green rule across to the red side would unresolve every row counted here
    /// and shrink a denominator this repository publishes on every run, so the
    /// asymmetry is held down by a control rather than left to be tidied up.
    #[test]
    fn the_real_tooling_rows_name_a_reversal_before_their_prose() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let reversals = real_tree(&root).reversals;
        let continued: Vec<(String, String)> = tooling_rows(&root)
            .unwrap_or_default()
            .into_iter()
            .filter(|(value, _)| {
                !value.starts_with(OWED_PREFIX) && value.split_whitespace().count() > 1
            })
            .collect();
        assert!(
            continued.len() > 1,
            "the tooling ledgers carry {} row naming a reversal before its prose; the convention \
             this control pins is gone and the control is guarding nothing",
            continued.len()
        );
        let ledger = red_twin_ledger(&continued, &reversals);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
        assert_eq!(
            ledger.discharged,
            continued.len(),
            "a tooling row naming its reversal and then speaking prose stopped resolving"
        );
    }

    /// Planted reversal: the other direction of the same drift — a law standing
    /// in `laws.rs` that no obligation claims, which is a proof outliving the
    /// claim it was written for.
    #[test]
    fn a_law_no_obligation_claims_is_a_violation() {
        let found = drifted_claim_offences(&[], &declared_laws(ONE_LAW));
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found
                .first()
                .is_some_and(|offence| offence.contains("claimed by no obligation")),
            "{found:?}"
        );
    }

    /// The positive control: a claim and the law that answers it are no offence
    /// in either direction. A leg that flagged everything would satisfy both
    /// reversals above and be worthless.
    #[test]
    fn a_claim_and_the_law_answering_it_are_lawful() {
        let claimed = seat_claims("    green: laws.rs root::a_law_somebody_wrote\n");
        assert_eq!(claimed.len(), 1, "{claimed:?}");
        let found = drifted_claim_offences(&claimed, &declared_laws(ONE_LAW));
        assert!(found.is_empty(), "{found:?}");
    }

    /// The real repository holds: the seats its READMEs write and the laws
    /// `laws.rs` declares are ONE population, counted through the one reader the
    /// join uses.
    ///
    /// The number that used to be a coincidence, stated out loud. Two readers
    /// with two prefixes agreed on their counts only because every row in this
    /// tree happens to be spelled with exactly one space in each place; the
    /// first row spelled otherwise would have left the strict reader's
    /// population with nothing saying so. There is one reader now, so this
    /// states what it found against the file it is joined to, and a seat that
    /// stops resolving moves the number rather than hiding behind it.
    #[test]
    fn the_real_seats_are_the_real_laws() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let claimed = real_claims(&root);
        let laws = fs::read_to_string(root.join("src").join("laws.rs")).unwrap_or_default();
        let existing = declared_laws(&laws);
        assert!(!existing.is_empty(), "laws.rs declares no law");
        assert_eq!(
            claimed.len(),
            existing.len(),
            "{} seats claimed, {} laws declared",
            claimed.len(),
            existing.len()
        );
        let found = drifted_claim_offences(&claimed, &existing);
        assert!(found.is_empty(), "{found:?}");
    }
}
