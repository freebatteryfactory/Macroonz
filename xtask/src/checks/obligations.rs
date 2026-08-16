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

use crate::repository::markdown::{ObligationLedger, obligation_ledger, tooling_reversal_rows};
use crate::repository::rust::DeclaredFunction;
use crate::repository::snapshot::{JUDGE_DIRECTORY, MACHINE_DIRECTORY, RepositorySnapshot};
use crate::repository::types::{CanonicalPath, GreenRow, ObligationRecord, Read};

/// The one compile-time proof surface the green seats are joined against.
const PROOF_SURFACE: &str = "src/laws.rs";

/// The document a home states its obligations in.
const HOME_LEDGER: &str = "README.md";

/// The READMEs that carry tooling qualification obligations.
///
/// A distinct population from the machine's homes: these are claims about the
/// TOOLS — what a service refuses, what a check catches, what a judge is
/// rehearsed against — and their reversals are counted on their own denominator.
const TOOLING_READMES: [&str; 2] = ["macros/macroc/README.md", "testpak/README.md"];

/// The prefix a lawful debt is spelled with: `owed-to-testpak`,
/// `owed-to-xtask-and-testpak`, and any other named creditor.
const OWED_PREFIX: &str = "owed-to";

/// The attribute the test harness collects a function by, in the harness's own
/// bare spelling.
const HARNESS_ATTRIBUTE: &str = "test";

/// The attribute that leaves a collected test in the binary and stops a plain
/// run from executing it. One lawful path spelling, and the compiler is what
/// closes the set — see [`skips_the_harness_run`].
const SKIP_ATTRIBUTE: &str = "ignore";

/// The attribute a build reads a condition off. One lawful path spelling,
/// protected by name — see [`is_a_build_condition`]. This reader asks whether it
/// is WRITTEN and never what it says; see [`stands_under_a_condition`].
const CONDITION_ATTRIBUTE: &str = "cfg";

/// The crate roots that re-export the harness attribute through a prelude.
const PRELUDE_ROOTS: [&str; 2] = ["core", "std"];

/// The module those roots publish their prelude under.
const PRELUDE_MODULE: &str = "prelude";

/// The prelude editions each root publishes the harness attribute in. Every one
/// of them was EXECUTED on the pinned toolchain before it was written down here;
/// see [`is_the_harness_attribute`] for what that measurement was and why an
/// unmeasured row would be the same defect pointing the other way.
const PRELUDE_EDITIONS: [&str; 5] = ["v1", "rust_2015", "rust_2018", "rust_2021", "rust_2024"];

/// The attribute that applies another attribute where a condition holds. The
/// second of the two the compiler protects BY NAME, and read on the same terms —
/// see [`is_a_conditional_application`].
const CONDITIONAL_ATTRIBUTE: &str = "cfg_attr";

/// The attribute a documentation comment arrives as. `///` above an item is
/// `#[doc = "…"]` ON that item, one attribute per line written, which is what
/// makes a control marker readable as a fact about a FUNCTION rather than as a
/// line in a file.
const DOCUMENTATION_ATTRIBUTE: &str = "doc";

/// The word a control marker opens with, and it is the obligation row's own
/// word. A test says which claim it controls in the vocabulary the claim is
/// written in — `green: <the obligation's id>` — so the two ends of the join are
/// spelled the same way in both files.
const CONTROL_MARKER: &str = "green:";

/// The obligations join, in five legs.
///
/// **Whole records, not loose rows.** Every row this join reads is read through
/// the obligation RECORD that declared it, and every record states exactly one
/// `green:` row and exactly one `red:` row. The rows used to be gathered by two
/// independent scans of the whole file, so nothing bound a row to the obligation
/// it belonged to and a record could lose one in silence: with its `green:` line
/// deleted an obligation named no positive control, so none was resolved and the
/// record qualified on its red row alone; with its `red:` line deleted the
/// published core denominator shrank by one and no leg had anything to say. A
/// `laws.rs` claim is exposed from the other side when it vanishes — the law it
/// named is then claimed by nobody — and a ROUTE has no other side, which is why
/// nothing caught this. A record carries its rows or the record is refused.
///
/// **Green, both ways.** Every README obligation naming a `laws.rs` green law
/// points at a law that exists, and every law in `laws.rs` is claimed by some
/// obligation — the READMEs and the laws never drift apart.
///
/// **Green, exactly once, and on BOTH green sides.** No law is claimed by two
/// obligations, and no executable seat is routed to by two. Evidence named
/// twice is a proof standing in for a claim it does not make, and it reads as
/// discharged from both rows. The two sides are one rule stated over two
/// populations rather than one rule and a habit: refusing a doubled law while a
/// doubled route passed was the same defect surviving in the branch the rule was
/// not written in, which is how every other defect in this reader has arrived.
/// The RED side is deliberately NOT held to it, and [`double_routed_offences`]
/// states why.
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
/// attribute deeper, and a file whose every test stands under a CONDITION runs
/// however many tests a build nobody named decides it runs, which is not a
/// positive control at all. The parse is where all of those are read.
///
/// A green route pointing at a file nobody wrote is worse than an unproven
/// claim, because it reads as proven; a green route pointing at its own red
/// twin, or at an empty test binary, is worse still, because the evidence it
/// names establishes nothing at all.
///
/// **Green, at the CONTROL the route names and not merely the file.** A routed
/// seat must hold a test the harness runs whose own documentation names the
/// obligation that routed to it. The seat question is an EXISTENTIAL over a file
/// — at least one test in there runs — so any unrelated test in the file
/// answered it, and the declared positive control could be renamed or deleted
/// with the route still qualifying on a neighbour's back. The control marker was
/// already written; nothing joined it. Now it is one of the two ends of a join,
/// and neither end can move without the other.
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
pub(crate) fn check_obligations_join(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let laws = snapshot
        .rust()
        .functions_in(&CanonicalPath::spelled(PROOF_SURFACE))
        .taken(PROOF_SURFACE)?;
    let existing = declared_laws(&laws);
    let mut claimed = Vec::new();
    let mut rows = Vec::new();
    let mut routes = Vec::new();
    let mut unreadable = Vec::new();
    let mut offenders = Vec::new();
    for home in home_readmes(snapshot) {
        let document = snapshot.markdown().document(&home).taken(home.as_str())?;
        let spelled = home.to_string();
        let ledger = obligation_ledger(document, &spelled)
            .taken(&format!("{spelled}'s obligation ledger"))?;
        offenders.extend(
            document
                .unjoinable_data_blocks()
                .into_iter()
                .map(|offence| format!("{spelled}: {offence}")),
        );
        let declared = home_rows(&ledger, &spelled);
        offenders.extend(declared.offences);
        claimed.extend(declared.claimed);
        routes.extend(declared.routes);
        unreadable.extend(declared.unreadable);
        rows.extend(declared.red);
    }
    offenders.extend(drifted_claim_offences(&claimed, &existing));
    offenders.extend(double_claimed_offences(&claimed));
    let judge = testpak_populations(snapshot);
    let ledger = red_twin_ledger(&rows, &judge.reversals);
    offenders.extend(phantom_green_routes(&routes, &judge.seats));
    offenders.extend(uncontrolled_green_routes(&routes, &judge.seats));
    offenders.extend(double_routed_offences(&routes));
    offenders.extend(unreadable_green_offences(&unreadable));
    offenders.extend(judge.unparsable);

    let tooling_rows = tooling_rows(snapshot)?;
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

/// Every home README the join reads: the root one, and one per numbered band.
///
/// Derived from the one reading rather than from a directory listing of its
/// own, in canonical path order, so the population is stable on every machine
/// and cannot differ from the population any other law is about.
fn home_readmes(snapshot: &RepositorySnapshot) -> Vec<CanonicalPath> {
    let mut homes = vec![CanonicalPath::spelled(HOME_LEDGER)];
    homes.extend(
        snapshot
            .files()
            .under(MACHINE_DIRECTORY)
            .map(|(path, _)| path)
            .filter(|path| {
                path.file_name() == HOME_LEDGER && path.as_str().matches('/').count() == 2
            })
            .cloned(),
    );
    homes
}

/// Everything one home README declared, read through the obligation records
/// that declared it.
///
/// The home's twin of [`JudgeTree`], and it stands here for the same reason: one
/// reading of one file produces every population that file contributes, so no
/// two of them can be about different records. What used to be here was four
/// accumulators filled from two whole-file scans, and the gap between those
/// scans is where a record could lose a row without anything noticing.
struct HomeRows {
    /// `(module, law, README)` for every compile-time seat the home claims.
    claimed: Vec<(String, String, String)>,
    /// Every green route the home declares, carrying its obligation.
    routes: Vec<GreenRoute>,
    /// `(value, README)` for every green row no lawful spelling reads.
    unreadable: Vec<(String, String)>,
    /// `(value, README)` for every red row, for the published ledger.
    red: Vec<(String, String)>,
    /// What the record reading itself refused: a record missing or doubling a
    /// row, and a row no record owns.
    offences: Vec<String>,
}

/// Everything one home README declares, sorted into the populations the join
/// resolves.
///
/// Every row here arrives through the record that wrote it, which is what makes
/// the two record legs above possible at all: a row's obligation is known
/// because the row was never separated from it. A green route carries that
/// obligation onward, because the route leg has to ask a question about the
/// CONTROL and not merely about the file.
fn home_rows(ledger: &ObligationLedger, home: &str) -> HomeRows {
    let records = ledger.records();
    let mut offences = record_field_offences(records, home);
    offences.extend(ledger.offences().iter().cloned());
    let mut declared = HomeRows {
        claimed: Vec::new(),
        routes: Vec::new(),
        unreadable: Vec::new(),
        red: Vec::new(),
        offences,
    };
    for record in records {
        let id = &record.id;
        declared.red.extend(
            record
                .red
                .iter()
                .map(|row| (row.clone(), String::from(home))),
        );
        for row in &record.green {
            match *row {
                GreenRow::CompileTimeSeat {
                    ref module,
                    ref law,
                } => {
                    declared
                        .claimed
                        .push((module.clone(), law.clone(), String::from(home)));
                }
                GreenRow::Route(ref named) => declared.routes.push(GreenRoute {
                    named: named.clone(),
                    readme: String::from(home),
                    id: id.clone(),
                }),
                GreenRow::Unreadable(ref value) => {
                    declared
                        .unreadable
                        .push((value.clone(), String::from(home)));
                }
                GreenRow::Disposition => (),
            }
        }
    }
    declared
}

/// One green route, carried whole: the file the row named, the home that
/// declared it, and the OBLIGATION whose record owns the row.
///
/// The id is here because a route with no obligation attached is a route that
/// can only be resolved against a file. That is what let the seat check be an
/// existential — the route asked "does anything in there run", because there was
/// nothing in the row saying which thing. Carrying the id makes the narrower
/// question askable, and carrying it in the same value as the path makes asking
/// the looser one alone a decision somebody has to write rather than the default.
#[derive(Debug)]
struct GreenRoute {
    /// The file the row named, as the row spelled it.
    named: String,
    /// The home README that declared the row.
    readme: String,
    /// The obligation whose record the row was written in.
    id: String,
}

/// Every obligation record whose IDENTITY or whose rows do not stand, one
/// offence per defect.
///
/// # The identity leg, which is B3's rule kept here rather than owed
///
/// An obligation's id is the join key on three sides at once: the `green:` and
/// `red:` rows are attributed to it, [`uncontrolled_green_routes`] resolves a
/// routed seat's control marker against it, and a repair is made by finding the
/// record it names. So a record whose id is EMPTY has rows nothing can attribute
/// and a marker nothing can name, and two records in one home sharing an id are
/// two obligations one marker discharges — the doubled-evidence defect this
/// module refuses on both green sides, arriving through the key instead of
/// through the row.
///
/// Neither is stated anywhere else, so leaving them to a later phase would leave
/// the join reading keys it never checked. Uniqueness is asked WITHIN one home,
/// which is the scope an id actually has: the id is written in a home's ledger,
/// resolved against that home's rows, and two homes may lawfully write the same
/// word.
///
/// # And the row leg, unchanged
///
/// Every record states exactly one `green:` row and exactly one `red:` row.
///
/// The leg the row readers could not have. A row-shaped reader answers questions
/// about the rows it was given, and a deleted row is the one thing it is never
/// given — so an obligation that lost its `green:` line stated no positive
/// control, resolved against nothing, and qualified; one that lost its `red:`
/// line took itself out of a denominator this repository publishes on every run,
/// and the count simply came back one smaller. The `laws.rs` side never had this
/// hole, because a claim that vanishes leaves its law claimed by nobody and the
/// drift leg reports it from the other side. A route has no other side. Nothing
/// in testpak knows an obligation was supposed to point at it.
///
/// Doubled is refused on the same rule and for the reason the doubled-evidence
/// legs are: two `green:` rows in one record state two positive controls for one
/// claim, and nothing says which of them the claim rests on. Two `red:` rows put
/// two entries on the published ledger for one obligation.
///
/// The offence names the id AND the README, because an id is unique to a home
/// rather than to the repository, and the repair is made in the file that wrote
/// it.
///
/// Pure over its records, so the leg is proven against fixture records rather
/// than by deleting a row from a README the repository stands on.
fn record_field_offences(records: &[ObligationRecord], readme: &str) -> Vec<String> {
    let mut offences = Vec::new();
    let mut reported: Vec<&str> = Vec::new();
    for record in records {
        let id = &record.id;
        if id.is_empty() {
            offences.push(format!(
                "{readme}: an obligation record opens with `id:` and states no identity. The id is \
                 the key its own rows are attributed to, the key a routed seat's control marker \
                 names back, and the key a repair is found by — a record with none has rows \
                 nothing can attribute and a marker nothing can name"
            ));
        } else if !reported.contains(&id.as_str())
            && records
                .iter()
                .filter(|other| other.id == *id)
                .count()
                .gt(&1)
        {
            reported.push(id.as_str());
            offences.push(format!(
                "{readme}: obligation `{id}` is declared by {} records, and an id names one \
                 obligation. Two records sharing a key are two claims one control marker \
                 discharges, and nothing says which of them the evidence is about",
                records.iter().filter(|other| other.id == *id).count()
            ));
        }
        if record.green.len() != 1 {
            let stated = record.green.len();
            offences.push(format!(
                "{readme}: obligation `{id}` states {stated} `green:` rows, and an obligation \
                 states exactly one. A record with none names no positive control, so none is \
                 resolved and the obligation qualifies on its red row alone; a record with two \
                 says nowhere which of them the claim rests on"
            ));
        }
        if record.red.len() != 1 {
            let stated = record.red.len();
            offences.push(format!(
                "{readme}: obligation `{id}` states {stated} `red:` rows, and an obligation states \
                 exactly one. A record with none leaves the published core denominator with \
                 nothing saying so; a record with two puts one obligation on that ledger twice"
            ));
        }
    }
    offences
}

/// Every `#[test]` law `laws.rs` declares, as `(module, law)` in declaration
/// order.
///
/// A law is a function carrying the harness's own attribute, and its identity is
/// the pair: the module path it is declared inside, and the name it is declared
/// under. Both come off the ONE parse the snapshot already holds — item, module
/// path, complete attribute set — through the same
/// [`is_the_harness_attribute`] roster the seat population is read with, so
/// `laws.rs` and `testpak/tests/` are asked the same question by the same reader.
///
/// # What the line reader this replaced could not see
///
/// It required the attribute to be alone on the previous LINE and the function
/// to open the next one, so every shape below vanished from the denominator
/// while cargo went on executing it — and vanishing from this denominator is
/// silent in the direction that matters, because a law nobody claims is only
/// reported when the join knows the law exists:
///
/// - `#[test]` followed by `#[should_panic]`, or by any other attribute, before
///   the function.
/// - `#[test]` written after `#[cfg]`, `#[expect]`, or a documentation comment,
///   which arrives as `#[doc = "…"]` on the same item.
/// - Anything declared inside a nested inline module, whose `mod` line the
///   reader matched only when it was written flush at the file's left edge, so a
///   nested law was attributed to the module ABOVE it.
///
/// It also read laws that do not exist: `#[test]` written on its own line inside
/// a multi-line string literal opened a phantom law, which the drift leg then
/// reported as claimed by nobody. The parse answers both directions at once,
/// because a parse is about items rather than about lines.
///
/// # The ceiling, and which way it falls
///
/// This establishes what `laws.rs` DECLARES, not what a run executed. A law
/// carrying `#[ignore]`, or standing under a `#[cfg]` no build enables, is
/// declared and is counted here — deliberately, because the join's question is
/// whether the READMEs and the proof surface name the same population, and a law
/// dropped from this side stands unclaimed with nothing saying so. That a
/// declared law also RUNS is a claim the qualification run's test stage
/// establishes and this reader does not; it opens where every other ceiling in
/// this module opens, at the roster a run publishes of what it executed.
fn declared_laws(declared: &[DeclaredFunction<'_>]) -> Vec<(String, String)> {
    declared
        .iter()
        .filter(|function| function.attributes().iter().any(is_the_harness_attribute))
        .map(|function| (function.module().to_owned(), function.name().to_owned()))
        .collect()
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

/// Every executable seat named by more than one green route, one offence per
/// seat.
///
/// The route side of the doubled-evidence refusal, and it is here because the
/// seat side standing alone was an asymmetry rather than a rule. Two obligations
/// naming one seat is two claims answered by one positive control, and the
/// control answers at most one of them: either the pair states one claim, in
/// which case it is one obligation, or it states two, in which case the second
/// one's green half is evidence another row is already spending.
///
/// # The RED side lawfully shares, and this side may not
///
/// The red ledger permits one reversal to answer two rows, deliberately and on
/// the record. `macros/macroc/README.md` names
/// `testpak/tests/compile-fail/a-magnitude-past-the-authoring-ceiling.rs` and
/// states in that row's own prose that the fixture is shared with the machine's
/// `root.admission-precedes-a-trusted-magnitude` — one program that fails to
/// compile falsifies both readings, and a second copy of it would be a second
/// thing to keep true. That permission stands, and nothing here touches the red
/// side: this leg is over green routes and no other population.
///
/// The two sides differ in what their evidence ESTABLISHES rather than in how
/// generously each is read. A reversal's fact is about the WHOLE file — the
/// fixture compiles or it does not — so two claims naming it are answered by
/// that one fact undivided, and the row that shares it says which common
/// refusal it is spending. A route's fact is an EXISTENTIAL over the file: at
/// least one test in it is a test the harness runs. Two claims naming one route
/// are each answered by "something in there runs", and nothing in either row
/// says the something is not the same single test both times, so the second
/// claim's positive control may not exist while the row reads as controlled.
/// That is the seat leg's reasoning arriving one population over.
///
/// # The ceiling, and it fails CLOSED
///
/// A green row names a FILE and cannot name the test function it is controlled
/// by. So a file genuinely holding two controls, one per claim, is refused here
/// — and there is no spelling in this repository for saying which is which. That
/// direction costs a lawful pair rather than admitting an unproven one, which is
/// the only direction a file-granular reading may fail in. It opens where every
/// other reader in this module opens: a route resolved against the roster a
/// qualification run EXECUTED, where the row names the function and two
/// obligations naming one file are two obligations naming two functions. This
/// leg's subject moves with that row.
///
/// Pure over its rows — `(named seat, declaring README)` pairs — so the law is
/// proven against fixture rows rather than against the tree it guards.
fn double_routed_offences(routes: &[GreenRoute]) -> Vec<String> {
    let mut offences = Vec::new();
    let mut reported: Vec<String> = Vec::new();
    for GreenRoute { named, .. } in routes {
        if reported.contains(named) {
            continue;
        }
        let declared: Vec<&str> = routes
            .iter()
            .filter(|route| &route.named == named)
            .map(|route| route.readme.as_str())
            .collect();
        if declared.len() > 1 {
            reported.push(named.clone());
            offences.push(format!(
                "green route `{named}` is named by {} obligations ({}): one executable seat \
                 controls one claim",
                declared.len(),
                declared.join(", ")
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
fn phantom_green_routes(rows: &[GreenRoute], seats: &SeatPopulation) -> Vec<String> {
    let mut offences = Vec::new();
    for GreenRoute { named, readme, .. } in rows {
        if !seats.carries(named) {
            offences.push(format!(
                "{readme}: green route names `{named}`, which is no executable test seat: a green \
                 route must name a file directly under `testpak/tests/` that DECLARES at least \
                 one `#[test]` a plain harness run EXECUTES, spelled as its exact \
                 repository-relative path. A compile-fail fixture can never be one; neither can a \
                 top-level file that builds a test binary with no test in it; neither can one \
                 whose every test carries `#[ignore]`, which a plain run reports as ignored while \
                 it finishes with nothing executed; and neither can one whose every test stands \
                 under a condition — any `cfg` or `cfg_attr`, on the test, on a module around it, \
                 or on the file itself, whatever the predicate says — because a control that \
                 executes only in some builds is not a positive control"
            ));
        }
    }
    offences
}

/// Every green route whose seat holds no test naming the obligation that routed
/// to it, one offence per route.
///
/// The leg that turns a route from a claim about a FILE into a claim about the
/// CONTROL it names. The seat question is an existential — does anything in this
/// file run — and an existential is answered by whatever happens to be there.
/// So `root.a-stamped-roster-declares-its-own-ceiling` could have its declared
/// positive control renamed or deleted and go on qualifying, on the strength of
/// any unrelated test left in the same file. The marker naming the obligation
/// was already written in that test's own documentation, and nothing read it.
///
/// The two ends are now one join. A route names its seat, the record names the
/// obligation, and a test in that seat names the obligation back — so the
/// control cannot be renamed, deleted, or moved to another file without one end
/// failing to find the other. Nothing here is a search through the file's lines:
/// the marker is read off the `#[doc]` attribute of a function `syn` handed back
/// as a test the harness RUNS, so a `green:` written in an ordinary comment, in a
/// string, in the file's own module documentation, or on an ignored test names
/// nothing at all. [`documented_control`] is the reading.
///
/// A route whose seat does not exist is left to [`phantom_green_routes`]. One
/// route earns one offence, and the author of a route naming a file nobody wrote
/// is told the file is missing rather than told what the file it does not have
/// fails to contain.
///
/// # The ceiling, and it fails CLOSED
///
/// The marker is prose in a doc comment rather than a field of a schema, so what
/// this establishes is that the seat DOCUMENTS itself as controlling the
/// obligation. A test whose marker says one thing and whose assertions do
/// another is not reachable from here, and no reader of a source is going to
/// reach it. What closes that is the same thing that closes every other ceiling
/// in this module: a route naming the test FUNCTION, resolved against the roster
/// a qualification run EXECUTED, under the versioned claim and evidence schema.
///
/// Pure over its rows, so the leg is proven against fixture rows rather than by
/// editing the seat it guards.
fn uncontrolled_green_routes(routes: &[GreenRoute], seats: &SeatPopulation) -> Vec<String> {
    let mut offences = Vec::new();
    for GreenRoute { named, readme, id } in routes {
        if seats.carries(named) && !seats.controls(named, id) {
            offences.push(format!(
                "{readme}: green route `{named}` is the positive control declared by obligation \
                 `{id}`, and no test in that file names it. A routed seat must hold a test a \
                 plain harness run EXECUTES whose own documentation states `{CONTROL_MARKER} \
                 {id}` — read off the test's documentation attribute, never off the file's lines. \
                 Seating a file is an existential over it, so without that naming any unrelated \
                 test in the file stands in for the control this obligation declared, and the \
                 declared one can be renamed or deleted with the route still qualifying"
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
/// This is also where a row that states MORE than its account now lands — a
/// route with a token after its path, a target with a second separator in it.
/// Those used to be truncated into the claim their first token made and joined
/// as if the rest were not written. The reader holds every green grammar to its
/// own account and nothing after it, and what that refuses arrives here, named
/// against the README that wrote it rather than silently trimmed.
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
            "{readme}: green row states {spelled}, which is no spelling this repository reads. \
             Every green row states its account and NOTHING after it: `laws.rs \
             <module>::<name>`, exactly one `::` and neither half empty; or a \
             repository-relative path to a `.rs` file, with nothing following the path; or a \
             declared disposition — `none — …`, `owed — …`, `structural (…)` — whose account is \
             the prose accounting for why no file holds a positive control, and is the one \
             account that runs as long as it needs to. A row nobody can read is an obligation \
             whose positive control nobody looks for"
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
fn tooling_rows(snapshot: &RepositorySnapshot) -> Result<Vec<(String, String)>, String> {
    let mut rows = Vec::new();
    for readme in TOOLING_READMES {
        let path = CanonicalPath::spelled(readme);
        let document = snapshot.markdown().document(&path).taken(&format!(
            "{readme}, which is declared as a tooling obligation ledger and whose rows would \
             otherwise leave the tooling denominator with nothing saying so"
        ))?;
        let declared = tooling_reversal_rows(document)
            .taken(&format!("{readme}'s tooling obligation ledger"))?;
        for row in declared {
            rows.push((row.clone(), String::from(readme)));
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
/// whose every test stands under a CONDITION is that same claim written one
/// attribute further out still: what the binary holds is then decided by a build
/// the route never named. A seat here is a file that declares a test the harness
/// RUNS unconditionally, established from a parse of it.
///
/// Each seat also carries what its runnable tests SAY they control, because
/// seating a file answers an existential and an obligation's route is not an
/// existential claim. See [`Seat`].
struct SeatPopulation(Vec<Seat>);

/// One executable seat: where it sits, and every obligation its runnable tests
/// name as the claim they control.
///
/// The second field is the whole of the second repair. A file was a seat, and a
/// route resolved to a file, so a route's positive control amounted to
/// "something in there runs" — which any test in the file answers, including one
/// written for another purpose entirely and one left behind after the declared
/// control was deleted. Which obligations the file's tests NAME is a different
/// fact about the same parse, gathered in the same walk, and it is what a route
/// is resolved against now.
///
/// Only tests the harness RUNS contribute. A marker on an ignored test, on a
/// conditionally compiled one, or on a function that is no test at all names a
/// control that does not execute, and this reader already knows which functions
/// execute.
#[derive(Debug)]
struct Seat {
    /// The file, as a repository-relative slash path.
    path: String,
    /// The obligations its runnable tests document themselves as controlling.
    controls: Vec<String>,
}

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
        self.0.iter().any(|seat| seat.path.as_str() == named)
    }

    /// Whether the seat a green route names holds a runnable test that names
    /// the obligation back.
    ///
    /// Both halves of the identity are exact: the seat is found by its exact
    /// repository-relative path, and the obligation by its exact id. A marker
    /// that merely contains the id, or an id that merely contains a marker's
    /// value, names an obligation nobody declared — which is precisely what a
    /// half-finished rename leaves behind on one side of this join.
    fn controls(&self, named: &str, id: &str) -> bool {
        self.0.iter().any(|seat| {
            seat.path.as_str() == named && seat.controls.iter().any(|stated| stated == id)
        })
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
fn testpak_populations(snapshot: &RepositorySnapshot) -> JudgeTree {
    let tests = format!("{JUDGE_DIRECTORY}/tests");
    let mut reversals = Vec::new();
    let mut top_level: Vec<(&CanonicalPath, &syn::File)> = Vec::new();
    let mut unparsable = Vec::new();
    for (path, source) in snapshot.rust().under(&tests) {
        reversals.push(path.to_string());
        if !path.sits_directly_in(&tests) {
            continue;
        }
        match *source {
            Read::Known(ref parsed) => top_level.push((path, parsed)),
            Read::DeclaredAbsent(reason) => unparsable.push(format!(
                "{path} sits directly under `{tests}/` and was not read, so whether it declares a \
                 test that RUNS is unknown rather than false: {reason}"
            )),
            Read::Unreadable(ref failure) => unparsable.push(format!(
                "{path} sits directly under `{tests}/` and is not parseable Rust, so whether it \
                 declares a test that RUNS is unknown rather than false: {failure}"
            )),
        }
    }
    JudgeTree {
        reversals: ReversalPopulation(reversals),
        seats: seat_population(&top_level),
        unparsable,
    }
}

/// The seats among the top-level sources.
///
/// Pure over `(canonical path, parsed tree)` pairs, so the reversal for the
/// narrowing is a source held in memory: the leg that decides what counts as a
/// positive control is never proven by writing an empty test file into the
/// judge. A source that did not parse never arrives here — the caller carries it
/// as an offence, because whether such a file declares a running test is UNKNOWN
/// rather than false.
///
/// The FILE's own attributes are read before its items are, because the file is
/// the outermost module a test can be enclosed by and this reader used to walk
/// straight past it. `#![cfg(any())]` written at the top of a top-level source
/// gates the whole binary: measured on the pinned toolchain, that file compiles
/// clean and its harness reports `0 tests`, while a reader looking only at items
/// saw the `#[test]` below and called it a seat. `syn` hands an inner attribute
/// back on the item it is written inside, so the file, a module and a function
/// are all asked the same question through the same reading.
fn seat_population(sources: &[(&CanonicalPath, &syn::File)]) -> SeatPopulation {
    SeatPopulation(
        sources
            .iter()
            .filter_map(|(path, parsed)| seat_of(path.as_str(), parsed))
            .collect(),
    )
}

/// The seat one parsed top-level source is, or nothing where it is none.
///
/// Two refusals and one construction, in the order the reader can afford them: a
/// file standing under a condition is no seat whatever it declares, a file
/// declaring no test the harness runs is no seat however it is placed, and what
/// is left is a seat carrying every obligation its running tests name.
///
/// The controls are gathered from the SAME walk that decided the file is a seat,
/// so a test that seats the file and a test that names an obligation can never
/// be established by two readings that disagree about which tests run.
fn seat_of(path: &str, file: &syn::File) -> Option<Seat> {
    if stands_under_a_condition(&file.attrs) {
        return None;
    }
    let executed = harness_tests(&file.items);
    if executed.is_empty() {
        return None;
    }
    Some(Seat {
        path: String::from(path),
        controls: executed.into_iter().flatten().collect(),
    })
}

/// Every test one parsed scope declares that cargo will RUN, each carrying the
/// obligations its own documentation names as the claims it controls.
///
/// One walk, two facts, because they are two readings of the same items and two
/// walks would be two chances to disagree about which functions the harness
/// runs. That a scope declares a runnable test at all is this list being
/// non-empty; which claims those tests control is what the entries carry. A test
/// naming nothing contributes an empty entry rather than no entry, so a file
/// full of unnamed tests is still a seat and still controls nothing.
///
/// Only tests that RUN are here, which is what makes the marker a fact about
/// executed evidence. A marker on a skipped test, on a conditionally compiled
/// one, or on a plain function names a control that never executes, and none of
/// them reaches this list.
///
/// The question is about ITEMS: that a function is declared, that the test
/// harness's own attribute sits ON it, that nothing beside that attribute tells
/// the harness to skip it, and that no condition stands between it and the
/// binary. A text search for `#[test]` answers a different question and answers
/// it wrongly in both directions — it says yes to the attribute written inside a
/// doc comment, a string literal, or a commented-out block, each of which is a
/// file with no test in it, and it is the class of reader this repository has
/// already replaced twice. There is one reading, and it is the parse.
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
/// # A control that runs under SOME conditions is not a positive control
///
/// A green route claims one thing: this obligation's control EXECUTES. A test
/// whose compilation is conditional cannot make that claim in any build the
/// route does not name, and a route names no build. So the question asked here
/// is not "is this predicate false" — it is whether the control is there
/// unconditionally. Any `cfg` or `cfg_attr`, on the test, on a module enclosing
/// it, or on the file itself, and the file is no seat.
///
/// That rule is CLOSED, and the enumeration it replaced was not. This reader
/// refused `cfg(any())`, then the literal `false`, then `cfg_attr(all(), ignore)`
/// and `cfg_attr(true, ignore)` — four passes, each correct, each asking whether
/// a predicate was false of a predicate language that has no last member. The
/// fifth arrived on schedule: `#[cfg(not(test))]` over a `#[test]`, which in an
/// integration test binary is in NO build cargo makes, because `test` is always
/// set there. Measured on the pinned toolchain, that file's harness reports
/// `0 tests`. A sixth was already sitting under the reader unasked —
/// `#![cfg(any())]` written at the TOP of the file, which items-only reading
/// walked straight past; measured, that binary reports `0 tests` too.
///
/// Truth is not what the rule is about, and `#[cfg(test)]` is the proof.
/// Measured, `#[cfg(test)] #[test]` in an integration test binary reports one
/// PASSED test: the predicate is TRUE there, the control does execute, and this
/// reader refuses the file anyway. It refuses it because a seat resolved through
/// a predicate is a seat whose author and whose reader must both be right about
/// a build neither of them named, and nobody tells this reader which build will
/// run. That direction costs a control somebody wrote, whose author is told
/// exactly what is missing and writes the test without the gate — which is the
/// only direction this reading may fail in.
///
/// Nothing is evaluated to reach any of that. No feature is resolved, no target
/// is decided, no profile is read, no predicate language is entered. The reading
/// is whether the attribute is written, which is why there is no next round.
///
/// # What is still open, and which way each falls
///
/// A `mod name;` reaching a SEPARATE file is not followed, so a seat whose only
/// tests arrive that way is not admitted here. That direction fails CLOSED: the
/// route is refused, its author is told exactly what is missing, and nothing
/// reads as proven that is not.
///
/// An import-renamed spelling of the harness attribute is not resolved, and that
/// fails CLOSED too. [`is_the_harness_attribute`] states the measurement.
///
/// The grammar is not the expansion, and that one falls BOTH ways — it is the
/// only ceiling left here that does. What this reading establishes is that a
/// SOURCE declares a harness test under the grammar the pinned toolchain
/// accepts. It does not establish that the compiled binary contains one. An
/// attribute macro standing above a test may delete it, rename it, wrap it, or
/// hand back something else entirely — including something wearing a `cfg` the
/// macro wrote — and no parse can say which, because the expansion runs after
/// this reader is finished, in a crate this reader never opens. A macro that
/// REMOVES a test seats a file that runs nothing, which fails open; a macro that
/// GENERATES tests from a declaration carrying no harness attribute has its file
/// refused, which fails closed. So conditional compilation can still reach a
/// green seat by exactly one road — an expansion that writes the condition — and
/// it can no longer reach one by being written down.
/// `the_expansion_ceiling_is_open_and_says_so` is where that is stated with an
/// assertion on it.
///
/// What the exact-path roster in [`is_the_harness_attribute`] closed is the
/// narrower case where a foreign attribute is the one SPELLING the category's
/// word — `#[some_crate::test]` is no longer read as a test, and
/// `#[some_crate::ignore]` is no longer read as a skip. It could not close the
/// case of a foreign attribute sitting above a genuine `#[test]`, and reading
/// harder will not reach it.
///
/// What closes the rest is a stronger seat than a parse can reach: a green route
/// naming the test FUNCTION it is controlled by, resolved against the roster a
/// qualification run EXECUTED. A roster is the OUTPUT of expansion rather than a
/// guess about it, so it answers every ceiling above at once — a test arriving
/// through a separate file or generated by a macro appears in it, one a macro
/// deleted does not, one a condition removed never existed to appear, an ignored
/// one is reported as ignored, and an aliased spelling of the harness attribute
/// is invisible to the question because the roster is not reading spellings at
/// all. That reading retires every reader above it, this one included. It needs a
/// row that can name a function and a run that publishes what it ran. Neither
/// exists today. It is the versioned claim and evidence schema's opening
/// condition, and it is not built here.
///
/// Half of the "name a function" half of that is reachable from a source, and it
/// is reached: the tests here carry the obligations their own documentation
/// names, so a route resolves to a CONTROL rather than to a file. What that does
/// not become is a roster. It says which claim a test in the source documents
/// itself as controlling, not which tests a run executed, so every ceiling above
/// stands exactly where it stood — a macro that deletes the marked test still
/// leaves the file seated by whatever it hands back, and a marker on a test the
/// expansion removed is a name nothing withdraws.
///
fn harness_tests(items: &[syn::Item]) -> Vec<Vec<String>> {
    let mut executed = Vec::new();
    for item in items {
        if let syn::Item::Fn(declared) = item {
            if !stands_under_a_condition(&declared.attrs) && runs_under_the_harness(&declared.attrs)
            {
                executed.push(documented_controls(&declared.attrs));
            }
        } else if let syn::Item::Mod(module) = item
            && !stands_under_a_condition(&module.attrs)
            && let Some((_, inner)) = module.content.as_ref()
        {
            executed.extend(harness_tests(inner));
        }
    }
    executed
}

/// Every obligation one item's DOCUMENTATION names as a claim it controls.
fn documented_controls(attributes: &[syn::Attribute]) -> Vec<String> {
    attributes.iter().filter_map(documented_control).collect()
}

/// The obligation one documentation attribute names, or nothing where it names
/// none.
///
/// The marker is read as an ATTRIBUTE and never as a line. `///` above an item
/// arrives as `#[doc = "…"]` on that item — one attribute per line written — so
/// `syn` has already decided which item the documentation belongs to and where
/// each written line begins and ends. Nothing here splits a text into lines,
/// searches a file, or looks at anything that is not attached to the function
/// under judgement.
///
/// That is the whole difference between this and a scan. A `green:` written in
/// an ordinary `//` comment is not an attribute and is invisible here; one
/// inside a string literal is a value rather than an attribute; one in the
/// file's own `//!` documentation is an attribute on the FILE, which is not a
/// test; and one above a skipped or conditionally compiled test is never asked
/// for, because [`harness_tests`] asks only the functions that run.
///
/// The marker's own grammar is the obligation row's: the word the row opens
/// with, then the id, then whatever prose the author wants. The id is the first
/// token after the word, so the sentence the marker continues into — which is
/// how the one marker in this repository is written, wrapping across three
/// documentation lines — costs the reading nothing.
fn documented_control(attribute: &syn::Attribute) -> Option<String> {
    if !attribute.path().is_ident(DOCUMENTATION_ATTRIBUTE) {
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
    let documented = written.value();
    let named = documented.trim().strip_prefix(CONTROL_MARKER)?;
    named.split_whitespace().next().map(str::to_string)
}

/// Whether one item's attributes make its compilation conditional at all.
///
/// Asked of a FUNCTION, of a MODULE, and of the FILE, through one reading,
/// because the compiler gates all three the same way and a gate on an enclosing
/// scope takes every test inside it along. Asked only of the function, a file
/// whose whole test module is gated is a seat; asked only of items, a file
/// carrying its own `#![cfg(any())]` is a seat while its binary is empty. `syn`
/// hands an inner attribute back on the item it is written inside, so one field
/// answers the question at all three sites.
///
/// The predicate is never opened. `any()`, the literal `false`, the literal
/// `true`, `all()`, `test`, `not(test)`, a feature, a target, a bare name, a
/// composition, or a spelling nobody has written yet — the answer here is the
/// same for every one of them, and that sameness is what makes this rule closed
/// rather than a list waiting for its next member. Four passes closed four
/// spellings one at a time and a fifth was found each time; this asks a question
/// with no members to enumerate.
fn stands_under_a_condition(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(is_conditional_compilation)
}

/// Whether one attribute is conditional compilation: the build's condition, or
/// the application of an attribute under one.
///
/// Both, through one reader, because both make what an item IS depend on a build
/// nobody here names. `cfg` decides whether the test is compiled. `cfg_attr`
/// decides what is WRITTEN on it — `ignore`, a `cfg`, another `cfg_attr`,
/// anything — and telling those apart means evaluating its predicate and then
/// reading what it carries, which is the pair of habits this rule exists to
/// stop. A `cfg_attr` that turns out to apply something harmless costs its file
/// nothing but an unconditional spelling.
fn is_conditional_compilation(attribute: &syn::Attribute) -> bool {
    is_a_build_condition(attribute) || is_a_conditional_application(attribute)
}

/// Whether one function's attributes make it a test the harness EXECUTES.
///
/// Two facts about one function, and the second is not the negation of the
/// first: the harness's own attribute puts the function in the binary, and the
/// skip leaves it there and stops it running. A reading that took only the first
/// counts a function nothing executes.
///
/// The two are read by DIFFERENT readers over different form sets, and that
/// separation is the law here rather than an economy. Each category is
/// recognized by the forms the pinned toolchain actually accepts FOR IT, and the
/// two sets are not the same size: twenty-one lawful spellings collect a test,
/// and one attribute skips it. Held to a single reading the pair was wrong in
/// both directions at once — `#[some_crate::test]` was counted as a test, which
/// seats a binary that runs nothing, and `#[some_crate::ignore]` was counted as a
/// skip, which throws away a test that runs. Neither of those crate-qualified
/// attributes is the harness's; both are measured in this module's reversals.
///
/// The skip is asked about at ONE site. It used to be asked at two, because a
/// `cfg_attr` could apply it wherever its predicate held, and a whole reader
/// existed to decide when that was. Nothing reaching this function carries a
/// `cfg_attr` any more: [`stands_under_a_condition`] refuses the item before the
/// harness is consulted, whatever the application would have applied.
fn runs_under_the_harness(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(is_the_harness_attribute) && !attributes.iter().any(skips_the_harness_run)
}

/// Whether one attribute is the harness's own, read by its WHOLE path against a
/// roster of spellings measured on the pinned toolchain.
///
/// The reading this replaced compared the path's LAST SEGMENT, and stated the
/// consequence in its own documentation as though it were the opposite: that "a
/// longer name ending in the word is not" the attribute. It was. Under a
/// last-segment reading `#[test]`, `#[core::prelude::v1::test]` and
/// `#[any_crate::test]` are one attribute — and the third is whatever the crate
/// defining it does, including nothing at all. `test` is not a reserved name in
/// the attribute namespace, so a no-op proc attribute called `test` is ordinary
/// Rust; measured, a file whose only test carries one compiles and its harness
/// reports `0 tests` while the function is warned about as dead code. That is
/// the empty-binary defect a fourth time, arriving through the reader built to
/// close it: a semantic category recognized by a convenient SPELLING rather than
/// by the forms the category actually has.
///
/// # The roster, and how every row got here
///
/// Each spelling below was EXECUTED on this repository's pinned toolchain before
/// it was admitted, because admitting a path on the strength of a plausible
/// reading is the same defect facing the other way — a reader claiming a fact
/// about the compiler that nobody asked the compiler for.
///
/// - The bare path `test`, and only unrooted. `#[::test]` is a different path:
///   it resolves from the extern prelude, finds no crate of that name, and does
///   not compile. `Path::is_ident` is the exact statement of the admitted form —
///   one segment, no leading `::`, no generic arguments.
/// - `core::prelude::…::test` and `std::prelude::…::test` across the five
///   prelude editions the two roots publish, with or without a leading `::`.
///   Twenty spellings, twenty measured runs.
///
/// # What this cannot reach, and which way it falls
///
/// A path is not the only way the harness attribute arrives. Measured, both of
/// these run: `use core::prelude::v1::test as harness;` with `#[harness]`, and
/// `use core::prelude::v1 as p;` with `#[p::test]`. An import rename produces a
/// running test under a spelling no roster can enumerate, and no reader that
/// does not resolve imports will ever close it.
///
/// That direction fails CLOSED. A file whose only test is spelled through an
/// alias is not seated, the route naming it is refused, and its author is told
/// exactly what is missing. Nothing reads as proven that is not.
fn is_the_harness_attribute(attribute: &syn::Attribute) -> bool {
    let path = attribute.path();
    if path.is_ident(HARNESS_ATTRIBUTE) {
        return true;
    }
    let spelled: Vec<String> = path
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    let [root, prelude, edition, collected] = spelled.as_slice() else {
        return false;
    };
    PRELUDE_ROOTS.contains(&root.as_str())
        && prelude.as_str() == PRELUDE_MODULE
        && PRELUDE_EDITIONS.contains(&edition.as_str())
        && collected.as_str() == HARNESS_ATTRIBUTE
}

/// Whether one attribute stops the harness running the function it sits on: the
/// harness's SKIP, read by its whole path.
///
/// The skip's form set is closed, and the compiler is what closes it. Measured
/// on the pinned toolchain: `core::prelude::v1::ignore`,
/// `std::prelude::v1::ignore` and `::ignore` all fail to resolve, because the
/// skip is a built-in attribute rather than a re-exported macro — there is no
/// qualified spelling to admit, so none is admitted. rustc states the remainder
/// itself: offered `#[ignore("a reason in parentheses")]` it answers that "valid
/// forms for the attribute are `#[ignore = \"reason\"]` and `#[ignore]`" and
/// denies the input outright. Those two forms are the whole population.
///
/// So the reading is `is_ident`, which is exactly the measured shape: one
/// segment, no leading `::`, no generic arguments. The attribute's VALUE is
/// never looked at, and that is what makes `#[ignore]` and `#[ignore = "…"]` one
/// attribute here — syn hands the first as a bare path and the second as that
/// same path carrying a value, so a reason string hides nothing.
///
/// # Why this may NOT be read on the harness attribute's terms
///
/// The two shared one reader, on the argument that a pair is only meaningful
/// read on the same terms and that a second reader is where they would drift.
/// They are not the same category, and the shared reading was wrong in the
/// direction that costs a seat somebody wrote. `ignore` is not reserved either:
/// a crate may define a proc attribute called `ignore`, and measured,
/// `#[test] #[some_crate::ignore]` is a test that RUNS — one passed. A reader
/// matching the last segment called that file skipped and refused a real
/// positive control. Widening this to the harness attribute's twenty-one
/// spellings would be the same over-reach with a longer list, and there is
/// nothing to widen it to: every qualified spelling was measured and none
/// resolves.
///
/// # What it no longer has to reach
///
/// This used to read a PATH rather than an attribute, because the skip was asked
/// about at two sites: written on a function, and applied to one by a `cfg_attr`
/// whose predicate held in every build. A whole reader stood beside it deciding
/// when that was, and a `cfg_attr` is now refused wherever it is written, so the
/// applied site is gone and the category keeps one reader over one form.
fn skips_the_harness_run(attribute: &syn::Attribute) -> bool {
    attribute.path().is_ident(SKIP_ATTRIBUTE)
}

/// Whether one attribute is the conditional application, read by its whole path.
///
/// One spelling, protected exactly as `cfg` is — measured rather than assumed.
/// On the pinned toolchain a proc-macro crate defining one is refused at its own
/// definition site with "name `cfg_attr` is reserved in attribute namespace";
/// `#[core::prelude::v1::cfg_attr(all(), ignore)]` and its `std` twin are both
/// "could not find `cfg_attr` in `v1`"; and `#[::cfg_attr(all(), ignore)]` is
/// "could not find `cfg_attr` in the list of imported crates". The bare path is
/// the whole population, established at the definition site rather than hoped
/// for at the use site.
///
/// This one falls both ways, and the reservation is why neither direction is
/// reachable. Reading a foreign attribute as the conditional application would
/// REFUSE a seat somebody wrote; failing to read a real one would SEAT a file
/// whose only test is written by a build rather than by its author. There is no
/// foreign `cfg_attr` to mistake for this one, and no qualified one to miss.
///
/// The exactness matters more since the rule became presence rather than truth:
/// what is read here is refused outright, so a looser reading would now throw
/// away every file carrying any attribute whose path merely ends in this word.
/// `only_the_compilers_own_condition_gates_a_seat` is where that is held down.
fn is_a_conditional_application(attribute: &syn::Attribute) -> bool {
    attribute.path().is_ident(CONDITIONAL_ATTRIBUTE)
}

/// Whether one attribute is the build condition, read by its whole path.
///
/// One spelling, and this is the first of the two the compiler protects BY NAME
/// — the conditional application beside it is the other.
/// Measured on the pinned toolchain: `#[core::prelude::v1::cfg(any())]` is
/// refused with "expected attribute, found macro", because what the prelude
/// re-exports under that name is the `cfg!` macro and not the attribute;
/// `#[::cfg(any())]` does not resolve; and a crate cannot mint a competing one
/// at all, since rustc refuses the defining proc-macro crate itself with "name
/// `cfg` is reserved in attribute namespace". The bare path is the whole
/// population, established at the definition site rather than hoped for at the
/// use site.
///
/// Which way this one falls matters more than for the harness attribute and its
/// skip, because it can fall both ways. Reading a foreign attribute as a
/// condition would REFUSE a seat somebody wrote; failing to read a real one
/// would SEAT a file whose test some build decides the existence of. The
/// reservation is why neither is reachable: there is no foreign `cfg` to mistake
/// for this one, and no qualified one to miss.
fn is_a_build_condition(attribute: &syn::Attribute) -> bool {
    attribute.path().is_ident(CONDITION_ATTRIBUTE)
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
        GreenRoute, JudgeTree, OWED_PREFIX, ReversalPopulation, Seat, SeatPopulation,
        declared_laws, double_claimed_offences, double_routed_offences, drifted_claim_offences,
        home_readmes, phantom_green_routes, record_field_offences, red_twin_ledger,
        seat_population as seats_of_trees, testpak_populations, tooling_rows,
        uncontrolled_green_routes, unreadable_green_offences,
    };
    use crate::checks::scratch::Scratch;
    use crate::repository::markdown::{
        MarkdownDocument, ObligationLedger, obligation_ledger, tooling_reversal_rows,
    };
    use crate::repository::rust::declared_functions;
    use crate::repository::snapshot::{RepositorySnapshot, repository_snapshot};
    use crate::repository::types::{CanonicalPath, GreenRow, ObligationRecord, Read};

    /// The laws one fixture SOURCE declares, read the one way the join reads
    /// them.
    ///
    /// A fixture is text, so this parses one and hands the parse to the same
    /// reader the join uses. Nothing here re-implements the reading: a reversal
    /// proven against a helper that agrees with the reader proves the helper.
    fn laws_of(text: &str) -> Result<Vec<(String, String)>, String> {
        let file = syn::parse_file(text).map_err(|error| error.to_string())?;
        Ok(declared_laws(&declared_functions(&file)))
    }

    /// The obligation ledger one fixture states.
    ///
    /// A fixture is written as the data block a home writes, as the records
    /// inside one, or as the rows inside a single record — and every one of them
    /// is read HERE through the one reader the join itself uses, so a reversal is
    /// proven against the reading rather than against a helper that agrees with
    /// it. The wrapping is what a fixture omits, never what it states.
    fn fixture_ledger(text: &str) -> ObligationLedger {
        let written = if text.contains("```") {
            String::from(text)
        } else if text.contains("- id:") {
            format!("```yaml\nhome: fixture\nobligations:\n{text}```\n")
        } else {
            format!(
                "```yaml\nhome: fixture\nobligations:\n  - id: fixture.the-one-record\n{text}```\n"
            )
        };
        match obligation_ledger(&MarkdownDocument::parse(&written), "FIXTURE.md") {
            Read::Known(ledger) => ledger,
            Read::DeclaredAbsent(_) | Read::Unreadable(_) => ObligationLedger {
                records: Vec::new(),
                offences: Vec::new(),
            },
        }
    }

    /// Every obligation record one fixture declares.
    fn obligation_records(text: &str) -> Vec<ObligationRecord> {
        fixture_ledger(text).records
    }

    /// Every green row one fixture's records declare, classified.
    fn classify_green_rows(text: &str) -> Vec<GreenRow> {
        obligation_records(text)
            .into_iter()
            .flat_map(|record| record.green)
            .collect()
    }

    /// Every red row one fixture's records declare.
    fn red_twin_rows(text: &str) -> Vec<String> {
        obligation_records(text)
            .into_iter()
            .flat_map(|record| record.red)
            .collect()
    }

    /// Every `tooling-red:` row one fixture's tooling ledger declares.
    fn tooling_red_rows(text: &str) -> Vec<String> {
        let written = if text.contains("```") {
            String::from(text)
        } else {
            format!("```yaml\ntooling-obligation: fixture.the-one-obligation\n{text}```\n")
        };
        match tooling_reversal_rows(&MarkdownDocument::parse(&written)) {
            Read::Known(rows) => rows,
            Read::DeclaredAbsent(_) | Read::Unreadable(_) => Vec::new(),
        }
    }

    /// The seat population fixture SOURCE TEXT declares, and the fixtures whose
    /// seat question could not be answered.
    ///
    /// The law is handed trees the snapshot already parsed; a fixture is text, so
    /// this parses one and reports a fixture that does not parse exactly as the
    /// reading reports a source it could not read.
    fn seat_population(sources: &[(String, String)]) -> (SeatPopulation, Vec<String>) {
        let mut parsed = Vec::new();
        let mut unparsable = Vec::new();
        for (path, text) in sources {
            match syn::parse_file(text) {
                Ok(file) => parsed.push((CanonicalPath::spelled(path), file)),
                Err(error) => unparsable.push(format!(
                    "{path} sits directly under `testpak/tests/` and is not parseable Rust, so \
                     whether it declares a test that RUNS is unknown rather than false: {error}"
                )),
            }
        }
        let trees: Vec<(&CanonicalPath, &syn::File)> =
            parsed.iter().map(|(path, file)| (path, file)).collect();
        (seats_of_trees(&trees), unparsable)
    }

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

    /// Every row the real repository's home READMEs declare, read through the
    /// obligation record that declared it and attributed exactly as the join
    /// attributes it.
    fn real_rows(snapshot: &RepositorySnapshot) -> Result<Vec<(GreenRow, String, String)>, String> {
        let mut declared = Vec::new();
        for home in home_readmes(snapshot) {
            let document = snapshot.markdown().document(&home).taken(home.as_str())?;
            let spelled = home.to_string();
            let ledger = obligation_ledger(document, &spelled).taken(&spelled)?;
            declared.extend(ledger.records.into_iter().flat_map(|record| {
                let id = record.id;
                let declaring = spelled.clone();
                record
                    .green
                    .into_iter()
                    .map(move |row| (row, declaring.clone(), id.clone()))
            }));
        }
        Ok(declared)
    }

    /// Every claim the real repository's home READMEs make.
    fn real_claims(snapshot: &RepositorySnapshot) -> Result<Vec<(String, String, String)>, String> {
        Ok(real_rows(snapshot)?
            .into_iter()
            .filter_map(|(row, home, _)| match row {
                GreenRow::CompileTimeSeat { module, law } => Some((module, law, home)),
                GreenRow::Disposition | GreenRow::Route(_) | GreenRow::Unreadable(_) => None,
            })
            .collect())
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

    /// One synthetic GREEN population whose seats name no obligation: the
    /// population the seat-existence leg is read against.
    fn green_population(paths: &[&str]) -> SeatPopulation {
        SeatPopulation(
            paths
                .iter()
                .map(|path| Seat {
                    path: (*path).to_string(),
                    controls: Vec::new(),
                })
                .collect(),
        )
    }

    /// One synthetic GREEN population whose seats name the obligation their
    /// tests control.
    fn controlled_population(seats: &[(&str, &str)]) -> SeatPopulation {
        SeatPopulation(
            seats
                .iter()
                .map(|(path, control)| Seat {
                    path: (*path).to_string(),
                    controls: vec![(*control).to_string()],
                })
                .collect(),
        )
    }

    /// One synthetic green route population, every row attributed to a fixture
    /// README and to one fixture obligation.
    fn routed(paths: &[&str]) -> Vec<GreenRoute> {
        paths
            .iter()
            .map(|path| route(path, "FIXTURE.md", "fixture.an-obligation"))
            .collect()
    }

    /// The real judge tree, or the refusal that says why it could not be read.
    fn real_tree(snapshot: &RepositorySnapshot) -> JudgeTree {
        testpak_populations(snapshot)
    }

    /// One synthetic top-level source, at a path directly under
    /// `testpak/tests/`.
    fn top_level(name: &str, text: &str) -> (String, String) {
        (format!("testpak/tests/{name}"), text.to_string())
    }

    /// Every source given is refused as a seat, and a green route naming any of
    /// them is refused for standing under a condition.
    ///
    /// Both halves, over every source, because either alone is half an answer: a
    /// file dropped from the seat population with no offence spoken is the
    /// silence this boundary exists to end, and an offence about the wrong thing
    /// tells its author to repair something else. Shared by the conditional
    /// reversals so that adding a spelling to any one of them is one line rather
    /// than a block of assertions somebody could write differently.
    fn refuses_every_one(sources: &[(String, String)]) {
        let (seats, unparsable) = seat_population(sources);
        assert!(seats.0.is_empty(), "{:?}", seats.0);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        let paths: Vec<&str> = sources.iter().map(|(path, _)| path.as_str()).collect();
        let offered = phantom_green_routes(&routed(&paths), &seats);
        assert_eq!(offered.len(), sources.len(), "{offered:?}");
        assert!(
            offered
                .iter()
                .all(|offence| offence.contains("stands under a condition")),
            "{offered:?}"
        );
    }

    /// One synthetic claim row.
    fn claim(module: &str, law: &str, readme: &str) -> (String, String, String) {
        (module.to_string(), law.to_string(), readme.to_string())
    }

    /// One synthetic green route, attributed to the home that declared it and
    /// to the obligation whose record wrote it.
    fn route(named: &str, readme: &str, id: &str) -> GreenRoute {
        GreenRoute {
            named: named.to_string(),
            readme: readme.to_string(),
            id: id.to_string(),
        }
    }

    /// Every green route the real repository's home READMEs name, attributed
    /// exactly as the join attributes them.
    fn real_routes(snapshot: &RepositorySnapshot) -> Result<Vec<GreenRoute>, String> {
        Ok(real_rows(snapshot)?
            .into_iter()
            .filter_map(|(row, home, id)| match row {
                GreenRow::Route(named) => Some(route(&named, &home, &id)),
                GreenRow::CompileTimeSeat { .. }
                | GreenRow::Disposition
                | GreenRow::Unreadable(_) => None,
            })
            .collect())
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
    fn the_real_red_ledger_names_only_reversals_that_exist() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let reversals = real_tree(snapshot).reversals;
        assert!(!reversals.0.is_empty(), "testpak carries no reversal files");
        let mut collected = Vec::new();
        let readmes = home_readmes(snapshot);
        assert!(!readmes.is_empty(), "no home READMEs found");
        for readme in &readmes {
            let document = snapshot
                .markdown()
                .document(readme)
                .taken(readme.as_str())?;
            let name = readme.to_string();
            let ledger = obligation_ledger(document, &name).taken(&name)?;
            collected.extend(
                ledger
                    .records
                    .into_iter()
                    .flat_map(|record| record.red)
                    .map(|value| (value, name.clone())),
            );
        }
        let ledger = red_twin_ledger(&collected, &reversals);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
        assert!(
            ledger.owed > 0,
            "no owed red twins found; the ledger cannot be empty here"
        );
        Ok(())
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
    /// Read against a scratch tree carrying neither declared ledger, which is
    /// every declared ledger missing at once — the same reading the first
    /// missing one gets, since the leg refuses on the first.
    #[test]
    fn a_missing_tooling_ledger_is_a_violation() -> Result<(), String> {
        let scratch = Scratch::named("tooling-ledger-missing")?;
        scratch.write("README.md", "# a tree with no tooling ledger\n")?;
        let found = tooling_rows(&scratch.read()?);
        assert!(found.is_err(), "{found:?}");
        assert!(
            found
                .err()
                .is_some_and(|offence| offence.contains("tooling obligation ledger")),
        );
        Ok(())
    }

    /// The real tooling READMEs declare a non-empty denominator, and every row
    /// naming a reversal resolves to one that exists.
    #[test]
    fn the_real_tooling_ledger_names_only_reversals_that_exist() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let reversals = real_tree(snapshot).reversals;
        let collected = tooling_rows(snapshot)?;
        assert!(!collected.is_empty(), "no tooling reversal rows found");
        let ledger = red_twin_ledger(&collected, &reversals);
        assert!(ledger.offenders.is_empty(), "{:?}", ledger.offenders);
        assert!(ledger.owed > 0, "the tooling ledger claims no debt at all");
        Ok(())
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
        let vanished = phantom_green_routes(
            &routed(&["testpak/tests/nobody-ever-wrote-this.rs"]),
            &seats,
        );
        assert_eq!(vanished.len(), 1, "{vanished:?}");
        assert!(
            vanished
                .first()
                .is_some_and(|offence| offence.contains("nobody-ever-wrote-this.rs"))
        );

        let misspelled =
            phantom_green_routes(&routed(&["testpak/tests/stamp_row_ceilings.rs"]), &seats);
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
    fn a_green_route_naming_a_fixture_is_a_violation() -> Result<(), String> {
        let JudgeTree {
            reversals, seats, ..
        } = real_tree(repository_snapshot()?);

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
            let offered = phantom_green_routes(&routed(&[fixture]), &seats);
            assert_eq!(
                offered.len(),
                1,
                "{fixture} stood as a green positive control: {offered:?}"
            );
        }
        Ok(())
    }

    /// Planted reversal: a green route spelled loosely rather than exactly —
    /// a STALE path whose tail is a real seat name, and a BARE file name.
    ///
    /// Both resolved under the containment reading this leg used to carry. Each
    /// reads as a written, running positive control and names nothing that runs.
    #[test]
    fn a_green_route_spelled_loosely_is_a_violation() {
        let seats = green_population(&["testpak/tests/stamp_row_ceiling.rs"]);

        let stale = phantom_green_routes(&routed(&["testpak/old/stamp_row_ceiling.rs"]), &seats);
        assert_eq!(stale.len(), 1, "a stale path stood as a route: {stale:?}");

        let bare = phantom_green_routes(&routed(&["stamp_row_ceiling.rs"]), &seats);
        assert_eq!(bare.len(), 1, "a bare name stood as a route: {bare:?}");

        // The mirror of the stale case: a real seat whose path CONTAINS the
        // declared spelling. `row_ceiling.rs` names no seat anyone declared.
        let contained = phantom_green_routes(&routed(&["row_ceiling.rs"]), &seats);
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
            &routed(&[
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
            &routed(&[
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
            &routed(&[
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
            &routed(&[
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
            &routed(&[
                "testpak/tests/a_mixed_seat.rs",
                "testpak/tests/a_mixed_seat_across_modules.rs",
            ]),
            &seats,
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// Planted reversal: a top-level source whose every test stands under a
    /// condition — the whole class, in one rule, refused without a predicate
    /// being evaluated.
    ///
    /// The reader closed `cfg(any())`, then the literal `false`, then two
    /// `cfg_attr` spellings, one predicate a pass — and each pass a reviewer
    /// found the next one, because "is this predicate false" is a question asked
    /// of a language with no last member. The fifth is here: `#[cfg(not(test))]`
    /// over a `#[test]`, which in an integration test binary is in NO build
    /// cargo makes, because `test` is always set there. Measured on the pinned
    /// toolchain, that file's harness reports `0 tests`.
    ///
    /// The three rows that matter most are the ones no truth-seeking reader
    /// would ever refuse. `#[cfg(test)]` is TRUE in an integration test binary
    /// and its test PASSES — measured, one passed — and it is refused here,
    /// which is the statement that this rule is about conditionality rather than
    /// about truth. `#[cfg(all())]` and `#[cfg(true)]` were POSITIVE CONTROLS a
    /// pass ago, admitted for holding in every build; they are refused now on
    /// the same rule as the ones that hold in none, because a control anybody
    /// has to reason about is not a control.
    ///
    /// Everything the old ceiling admitted and could not reach — a feature, a
    /// target, a bare cfg name — is refused by that one rule, and reaching it
    /// took no new reading, no evaluator, and nothing resolved.
    #[test]
    fn a_conditionally_compiled_test_is_no_seat() {
        let sources = [
            // The two this reader closed one predicate at a time.
            top_level(
                "an_empty_disjunction.rs",
                "#[cfg(any())]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_literal_false.rs",
                "#[cfg(false)]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            // The fifth, found by a reviewer after those.
            top_level(
                "a_negated_harness_condition.rs",
                "#[cfg(not(test))]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            // The predicates that are TRUE where this binary runs, refused on
            // the same rule as the ones that are false.
            top_level(
                "a_true_harness_condition.rs",
                "#[cfg(test)]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "an_empty_conjunction.rs",
                "#[cfg(all())]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_literal_true.rs",
                "#[cfg(true)]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            // Everything a BUILD decides, which the ceiling used to admit.
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
            // Written after the harness attribute rather than before it.
            top_level(
                "a_condition_written_second.rs",
                "#[test]\n#[cfg(any())]\nfn the_behaviour_holds() {}\n",
            ),
        ];
        refuses_every_one(&sources);
    }

    /// Planted reversal: a top-level source whose only test carries a
    /// `cfg_attr`, whatever the application would have written on it.
    ///
    /// The conditional application is refused for BEING one. Two of these used
    /// to be refused for what they applied and when — `all()` and `true`
    /// carrying the skip, each measured to report its test as ignored — and a
    /// whole reader stood behind that pair, parsing the predicate out of the
    /// attribute and then looking through the applied list for `ignore`. Three
    /// more were admitted by the same reader for being past its reach: a skip
    /// under a target, a skip under a feature, and a skip applied through a
    /// second `cfg_attr`, which is measured to report its test as IGNORED while
    /// the reader saw only that the outer application carried no `ignore`.
    ///
    /// The last one is the shape of the whole change. Nothing here asks what is
    /// applied, so composition needs no recursion to refuse and a sixth spelling
    /// needs no sixth pass. The cost is stated rather than hidden: the file
    /// applying `inline` is refused too, and its author writes `inline` without
    /// the condition.
    #[test]
    fn a_conditionally_applied_attribute_is_no_seat() {
        let sources = [
            top_level(
                "a_skip_applied_by_an_empty_conjunction.rs",
                "#[cfg_attr(all(), ignore)]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_skip_applied_by_a_literal_true.rs",
                "#[cfg_attr(true, ignore)]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_test_skipped_under_a_condition.rs",
                "#[test]\n\
                 #[cfg_attr(windows, ignore)]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_test_skipped_under_a_feature.rs",
                "#[test]\n\
                 #[cfg_attr(feature = \"a-feature-nobody-enables\", ignore)]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_skip_applied_through_a_composed_condition.rs",
                "#[cfg_attr(all(), cfg_attr(all(), ignore))]\n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "an_applied_attribute_that_is_not_the_skip.rs",
                "#[cfg_attr(all(), inline)]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
        ];
        refuses_every_one(&sources);
    }

    /// Planted reversal: a top-level source whose test is unconditional and
    /// whose ENCLOSING scope is not.
    ///
    /// A gate on a scope takes every test inside it along, so the question has
    /// to be asked of the scope as well as of the test — and it has to be asked
    /// of every scope there is. Three sit around a test in a top-level source:
    /// a module gated from outside, the same module gated from INSIDE by its own
    /// inner attribute, and the FILE, which is the outermost module of all and
    /// the one this reader walked straight past. Measured on the pinned
    /// toolchain, a top-level source opening with `#![cfg(any())]` compiles
    /// clean and its harness reports `0 tests`, while an items-only reading saw
    /// the `#[test]` below it and called the file a seat.
    ///
    /// `syn` hands an inner attribute back on the item it is written inside, so
    /// the file and the module are answered by the same field the function is,
    /// and there is no fourth site to forget.
    #[test]
    fn a_condition_around_the_test_is_no_seat() {
        let sources = [
            top_level(
                "a_module_compiled_out.rs",
                "#[cfg(any())]\n\
                 mod behaviour {\n\
                 \x20   #[test]\n\
                 \x20   fn the_behaviour_holds() {}\n\
                 }\n",
            ),
            top_level(
                "a_module_under_a_feature.rs",
                "#[cfg(feature = \"a-feature-nobody-enables\")]\n\
                 mod behaviour {\n\
                 \x20   #[test]\n\
                 \x20   fn the_behaviour_holds() {}\n\
                 }\n",
            ),
            top_level(
                "a_module_gated_from_inside.rs",
                "mod behaviour {\n\
                 \x20   #![cfg(any())]\n\
                 \n\
                 \x20   #[test]\n\
                 \x20   fn the_behaviour_holds() {}\n\
                 }\n",
            ),
            top_level(
                "a_module_under_a_conditional_application.rs",
                "#[cfg_attr(all(), cfg(any()))]\n\
                 mod behaviour {\n\
                 \x20   #[test]\n\
                 \x20   fn the_behaviour_holds() {}\n\
                 }\n",
            ),
            top_level(
                "a_file_gated_from_inside.rs",
                "#![cfg(any())]\n\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
        ];
        refuses_every_one(&sources);
    }

    /// The positive control for the closed rule, in the directions it can
    /// over-reach.
    ///
    /// The condition is a fact about the ITEM it is written on, so an
    /// unconditional test standing beside a conditional one still seats the
    /// file, and so does an unconditional module standing beside a gated one. A
    /// reader that refused a file for holding a `cfg` anywhere would satisfy the
    /// reversal above and would throw away real seats, which is the failure this
    /// control exists to catch: refusing everything is not closing a category.
    #[test]
    fn a_live_test_beside_a_conditional_one_is_still_a_seat() {
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
                "a_live_test_beside_an_applied_skip.rs",
                "#[cfg_attr(all(), ignore)]\n\
                 #[test]\n\
                 fn the_slow_road() {}\n\
                 \n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_live_test_beside_a_feature_gated_one.rs",
                "#[cfg(feature = \"a-feature-nobody-enables\")]\n\
                 #[test]\n\
                 fn the_optional_road() {}\n\
                 \n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
        ]);
        assert_eq!(seats.0.len(), 4, "{:?}", seats.0);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        let found = phantom_green_routes(
            &routed(&[
                "testpak/tests/a_mixed_seat.rs",
                "testpak/tests/a_mixed_seat_across_modules.rs",
                "testpak/tests/a_live_test_beside_an_applied_skip.rs",
                "testpak/tests/a_live_test_beside_a_feature_gated_one.rs",
            ]),
            &seats,
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// Planted reversal: a top-level source whose only "test" is an attribute
    /// that merely ENDS in the harness's word.
    ///
    /// The empty binary a fourth time, and the one a suffix reading cannot see.
    /// `test` is not a reserved name in the attribute namespace, so any crate
    /// may define a proc attribute called `test`; one that hands the item
    /// straight back leaves an ordinary function behind and the binary runs
    /// nothing. Measured on the pinned toolchain: the harness reports `running 0
    /// tests` and the compiler warns that the function is never used.
    ///
    /// Read by its LAST SEGMENT the attribute was indistinguishable from
    /// `#[test]`, so the file was a seat and the route naming it read as a
    /// positive control that executes while nothing executed. The reading is the
    /// whole path now, and each spelling below arrives at the leg that refuses
    /// it: a foreign crate's attribute, a path shaped like the prelude's whose
    /// root is somebody else's, an edition module no root publishes, and the
    /// bare word resolved from the extern prelude — which is measured to be a
    /// different path from `#[test]` and to compile nowhere.
    #[test]
    fn an_attribute_merely_ending_in_the_harness_word_is_no_seat() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "a_foreign_test_attribute.rs",
                "#[some_unrelated_macro::test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_prelude_shaped_foreign_root.rs",
                "#[not_core::prelude::v1::test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "an_unpublished_prelude_edition.rs",
                "#[core::prelude::rust_2099::test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_rooted_bare_test.rs",
                "#[::test]\nfn the_behaviour_holds() {}\n",
            ),
        ]);
        assert!(seats.0.is_empty(), "{:?}", seats.0);
        assert!(unparsable.is_empty(), "{unparsable:?}");

        let offered = phantom_green_routes(
            &routed(&[
                "testpak/tests/a_foreign_test_attribute.rs",
                "testpak/tests/a_prelude_shaped_foreign_root.rs",
                "testpak/tests/an_unpublished_prelude_edition.rs",
                "testpak/tests/a_rooted_bare_test.rs",
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

    /// The positive control for that narrowing: every spelling of the harness
    /// attribute this repository MEASURED seats its file.
    ///
    /// A reader that admitted only the bare word would satisfy the reversal
    /// above and would throw away real seats, so the roster is stated here as
    /// the twenty-one sources it was measured over — the bare path, and the two
    /// prelude roots across their five edition modules, each with and without a
    /// leading `::`. Every one of them was EXECUTED on the pinned toolchain and
    /// reported one passing test before it was written into the reader. The
    /// count is asserted rather than the membership, so a row added to the
    /// roster without a source added here moves the number and fails by name.
    #[test]
    fn every_measured_harness_spelling_is_a_seat() {
        let mut sources = vec![top_level(
            "a_bare_seat.rs",
            "#[test]\nfn the_behaviour_holds() {}\n",
        )];
        let editions = ["v1", "rust_2015", "rust_2018", "rust_2021", "rust_2024"];
        let measured = ["core", "std"]
            .into_iter()
            .flat_map(|root| editions.map(move |edition| (root, edition)))
            .flat_map(|(root, edition)| {
                [("", "unrooted"), ("::", "rooted")]
                    .map(move |(lead, spelled)| (root, edition, lead, spelled))
            });
        for (root, edition, lead, spelled) in measured {
            sources.push(top_level(
                &format!("a_{spelled}_{root}_{edition}_seat.rs"),
                &format!(
                    "#[{lead}{root}::prelude::{edition}::test]\n\
                     fn the_behaviour_holds() {{}}\n"
                ),
            ));
        }
        let (seats, unparsable) = seat_population(&sources);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        assert_eq!(
            seats.0.len(),
            sources.len(),
            "a measured harness spelling stopped seating its file: {:?}",
            seats.0
        );
        assert_eq!(seats.0.len(), 21, "{:?}", seats.0);
    }

    /// The path reader's own ceiling, stated as a test, and this one fails
    /// CLOSED.
    ///
    /// An import rename produces a running harness test under a spelling no
    /// roster can enumerate. Measured on the pinned toolchain, both sources
    /// below report one PASSED test: `use core::prelude::v1::test as harness;`
    /// with `#[harness]`, and `use core::prelude::v1 as p;` with `#[p::test]`.
    /// A reader that does not resolve imports cannot see either, and this one
    /// does not resolve imports.
    ///
    /// Neither file is seated, so a route naming one is REFUSED and its author
    /// is told exactly what is missing. That is the direction that costs a real
    /// seat rather than the direction that invents one, and it is the only
    /// direction a spelling roster can fail in. It is written as an assertion
    /// rather than left in prose so the ceiling is where the reader ACTUALLY
    /// stands, and the pass that closes it — a route resolved against the roster
    /// a qualification run executed, where no spelling is a question anybody
    /// asks — fails this test by name and repairs it in one place.
    #[test]
    fn the_aliased_spelling_ceiling_is_closed_and_says_so() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "a_renamed_import.rs",
                "use core::prelude::v1::test as harness;\n\
                 \n\
                 #[harness]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_renamed_module.rs",
                "use core::prelude::v1 as p;\n\
                 \n\
                 #[p::test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
        ]);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        assert!(
            seats.0.is_empty(),
            "the alias ceiling has moved: this reader admits a spelling it cannot resolve, and the \
             doc that states so must move with it: {:?}",
            seats.0
        );
    }

    /// The reversal that matters most: a skip reader that over-reached the way
    /// the harness reader did would throw away a test that RUNS.
    ///
    /// `ignore` is not a reserved name either, so a crate may define a proc
    /// attribute called `ignore`. Measured on the pinned toolchain,
    /// `#[test] #[some_crate::ignore]` reports one PASSED test: the foreign
    /// attribute hands the item back and the harness's own skip was never
    /// written. A reader matching the last segment called that file skipped,
    /// refused it as a seat, and told its author a positive control they had
    /// written does not run.
    ///
    /// The other two are the direction nobody can reach by widening the reader.
    /// The skip is a BUILT-IN attribute rather than a re-exported macro, so it
    /// has no qualified spelling to admit: measured, `core::prelude::v1::ignore`
    /// and `std::prelude::v1::ignore` are both "could not find `ignore` in
    /// `v1`", and `::ignore` is "could not find `ignore` in the list of imported
    /// crates". None of the three sources below can be skipped by anything, and
    /// all three are seats.
    #[test]
    fn an_attribute_merely_ending_in_the_skip_word_is_no_skip() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "a_foreign_ignore.rs",
                "#[test]\n#[some_unrelated_macro::ignore]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_prelude_spelled_ignore.rs",
                "#[test]\n#[core::prelude::v1::ignore]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_rooted_ignore.rs",
                "#[test]\n#[::ignore]\nfn the_behaviour_holds() {}\n",
            ),
        ]);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        assert_eq!(seats.0.len(), 3, "{:?}", seats.0);
        let found = phantom_green_routes(
            &routed(&[
                "testpak/tests/a_foreign_ignore.rs",
                "testpak/tests/a_prelude_spelled_ignore.rs",
                "testpak/tests/a_rooted_ignore.rs",
            ]),
            &seats,
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// The condition reader's exactness, and the measurement that makes the one
    /// direction it could hurt unreachable.
    ///
    /// A suffix reading called `#[some_crate::cfg(any())]` a build condition and
    /// threw away the seat beneath it. The path reading does not, and the file
    /// below is a seat. Nothing compiled-out is admitted by that: measured on
    /// the pinned toolchain, `cfg` is RESERVED in the attribute namespace, and a
    /// proc-macro crate declaring one is refused at its own definition site with
    /// "name `cfg` is reserved in attribute namespace". So there is no foreign
    /// condition to mistake for the real one. The qualified spellings of the
    /// real one do not resolve either — `#[core::prelude::v1::cfg(any())]` is
    /// "expected attribute, found macro", because what the prelude publishes
    /// under that name is the `cfg!` macro rather than the attribute — so there
    /// is none to MISS, which is the direction that would have failed open.
    ///
    /// The conditional application is protected the same way and is measured the
    /// same way. A proc-macro crate declaring `cfg_attr` is refused at its own
    /// definition site with "name `cfg_attr` is reserved in attribute
    /// namespace"; `#[core::prelude::v1::cfg_attr(all(), ignore)]` and its `std`
    /// twin are "could not find `cfg_attr` in `v1`"; and
    /// `#[::cfg_attr(all(), ignore)]` is "could not find `cfg_attr` in the list
    /// of imported crates". So the second file below carries an attribute that
    /// cannot be the compiler's, applies nothing, and is a seat.
    ///
    /// This control carries more weight since the rule became presence rather
    /// than truth. What is recognized here is now refused outright, with no
    /// predicate examined and no second chance downstream, so a reading that
    /// matched the last segment would throw away every seat whose author wrote
    /// any attribute ending in either word. The exactness is what stands between
    /// the closed rule and a reader that refuses real work.
    #[test]
    fn only_the_compilers_own_condition_gates_a_seat() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "a_foreign_condition.rs",
                "#[some_unrelated_macro::cfg(any())]\n#[test]\nfn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_foreign_conditional_application.rs",
                "#[some_unrelated_macro::cfg_attr(all(), ignore)]\n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
        ]);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        assert_eq!(seats.0.len(), 2, "{:?}", seats.0);
    }

    /// The reader's one remaining OPEN ceiling, stated as a test — and the
    /// subject is no longer the one this test used to hold.
    ///
    /// The conditional-compilation ceiling CLOSED. It stood here for as long as
    /// the rule was "is this predicate false", because a predicate a build
    /// decides could not be answered by a reader nobody tells which build will
    /// run. The rule is now that the control must be unconditional, which needs
    /// no answer to that question at all, and every case this test used to admit
    /// — a feature, a target, a bare cfg name, a conditional skip, a composed
    /// one — is refused by `a_conditionally_compiled_test_is_no_seat` instead.
    ///
    /// What is still open is the one thing a parse can never see: the expansion.
    /// A foreign attribute macro standing above a genuine `#[test]` is handed
    /// that item after this reader has finished, in a crate this reader never
    /// opens. It may hand the item back untouched, delete it, or return it
    /// wearing a `cfg` it wrote itself — so conditional compilation can still
    /// reach a green seat by exactly this one road, and by no other. The first
    /// source below is SEATED, and what it compiles to is not established here.
    ///
    /// It falls the other way too, and that direction is asserted beside it: a
    /// declaration carrying no harness attribute, which a macro will expand into
    /// tests, is refused. That costs a control somebody wrote rather than
    /// inventing one, which is the direction a reader of this kind may fail in.
    ///
    /// Written as an assertion rather than left in prose so the ceiling is where
    /// the reader ACTUALLY stands rather than where a comment says it does.
    /// Closing it is a route resolved against the roster a qualification run
    /// EXECUTED — a roster is the output of expansion rather than a guess about
    /// it — which the versioned claim and evidence schema opens, and a pass that
    /// closes it fails this test by name and repairs it in one place.
    #[test]
    fn the_expansion_ceiling_is_open_and_says_so() {
        let (seats, unparsable) = seat_population(&[
            top_level(
                "a_macro_standing_above_a_test.rs",
                "#[some_unrelated_macro::wrapping]\n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_macro_that_generates_the_tests.rs",
                "#[some_unrelated_macro::generating]\n\
                 mod behaviour {}\n",
            ),
        ]);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        assert_eq!(
            seats.0.len(),
            1,
            "the expansion ceiling has moved, and the doc that states so must move with it: {:?}",
            seats.0
        );
        assert!(
            seats
                .0
                .first()
                .is_some_and(|seat| seat.path.ends_with("a_macro_standing_above_a_test.rs")),
            "{:?}",
            seats.0
        );
    }

    /// The separate-file ceiling, stated as a test, and this one fails CLOSED.
    ///
    /// `mod behaviour;` reaches a file this reader does not open, so a source
    /// whose only tests arrive that way declares none as far as the parse can
    /// tell. The route naming it is REFUSED and its author is told exactly what
    /// is missing — nothing reads as proven that is not — and the repair is to
    /// write the seat where the route points.
    ///
    /// Asserted rather than described, for the same reason the other two
    /// ceilings are: a pass that starts following module declarations onto disk
    /// fails this test by name.
    #[test]
    fn the_separate_file_module_ceiling_is_closed_and_says_so() {
        let (seats, unparsable) =
            seat_population(&[top_level("a_module_in_another_file.rs", "mod behaviour;\n")]);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        assert!(
            seats.0.is_empty(),
            "the separate-file ceiling has moved: this reader admits a test it never read, and the \
             doc that states so must move with it: {:?}",
            seats.0
        );
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
                GreenRow::Route(named) => {
                    routes.push(route(&named, "FIXTURE.md", "fixture.an-obligation"));
                }
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
    /// owned by an obligation record, read as one of the three lawful
    /// spellings, and every route resolves to an EXECUTABLE seat that exists —
    /// with nothing dropped on the way.
    ///
    /// The last of those is the load-bearing one and it is why the count is
    /// taken twice. The rows are counted first through the RECORDS that declared
    /// them, then by the raw line prefix the rows are written with, and the two
    /// numbers must agree: that is the statement that no green row left the
    /// population either unclassified or unowned, stated over the real tree
    /// rather than over a fixture.
    #[test]
    fn the_real_green_rows_are_all_read() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let seats = real_tree(snapshot).seats;
        let mut routes = Vec::new();
        let mut unreadable = Vec::new();
        let mut seated = 0usize;
        let mut disposed = 0usize;
        let mut written = 0usize;
        let readmes = home_readmes(snapshot);
        assert!(!readmes.is_empty(), "no home READMEs found");
        for readme in &readmes {
            written = written.saturating_add(
                snapshot
                    .files()
                    .text(readme.as_str())
                    .taken(readme.as_str())?
                    .lines()
                    .filter(|line| line.trim().starts_with("green:"))
                    .count(),
            );
        }
        for (row, name, id) in real_rows(snapshot)? {
            match row {
                GreenRow::Route(named) => routes.push(route(&named, &name, &id)),
                GreenRow::Unreadable(value) => unreadable.push((value, name)),
                GreenRow::CompileTimeSeat { .. } => seated = seated.saturating_add(1),
                GreenRow::Disposition => disposed = disposed.saturating_add(1),
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
        Ok(())
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
    fn the_real_populations_are_named_apart() -> Result<(), String> {
        let JudgeTree {
            reversals,
            seats,
            unparsable,
        } = real_tree(repository_snapshot()?);
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
        for Seat { path, .. } in &seats.0 {
            assert!(
                reversals.0.contains(path),
                "{path} is an executable seat the reversal population does not carry"
            );
            assert_eq!(
                path.matches('/').count(),
                2,
                "{path} is not directly under testpak/tests/"
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
        let offered = phantom_green_routes(&routed(&fixtures), &seats);
        assert_eq!(
            offered.len(),
            fixtures.len(),
            "a real fixture stood as a green positive control: {offered:?}"
        );
        Ok(())
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
    fn the_real_obligations_claim_each_law_once() -> Result<(), String> {
        let claimed = real_claims(repository_snapshot()?)?;
        assert!(!claimed.is_empty(), "no green obligations found");
        let found = double_claimed_offences(&claimed);
        assert!(found.is_empty(), "{found:?}");
        Ok(())
    }

    /// Planted reversal: two obligations naming one executable seat.
    ///
    /// The doubled-evidence defect on the side that had no leg. Both rows
    /// resolve — the seat is real, its path is exact, the phantom-route leg says
    /// yes to each — and both read as controlled by a positive control that
    /// establishes one thing: something in that file runs. The second row is
    /// spending evidence the first is already spending, and nothing in either
    /// row says the two are not the same single test.
    #[test]
    fn a_route_named_by_two_obligations_is_a_violation() {
        let doubled = [
            route(
                "testpak/tests/stamp_row_ceiling.rs",
                "src/05_bounds/README.md",
                "bounds.a-stamped-roster-declares-its-own-ceiling",
            ),
            route(
                "testpak/tests/stamp_row_ceiling.rs",
                "src/05_bounds/README.md",
                "bounds.a-second-claim-spending-the-same-file",
            ),
        ];
        let found = double_routed_offences(&doubled);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found.first().is_some_and(|offence| {
            offence.contains("stamp_row_ceiling.rs") && offence.contains("2 obligations")
        }));

        // The seat is real and the route leg takes it: exactness is not what
        // refuses this, and a resolver fix alone never reaches it.
        let seats = green_population(&["testpak/tests/stamp_row_ceiling.rs"]);
        assert!(phantom_green_routes(&doubled, &seats).is_empty());

        // Two homes naming one seat is the same offence, and it is reported
        // once rather than once per home.
        let across_homes = [
            route(
                "testpak/tests/stamp_row_ceiling.rs",
                "src/23_evidence/README.md",
                "evidence.a-stamped-roster-declares-its-own-ceiling",
            ),
            route(
                "testpak/tests/stamp_row_ceiling.rs",
                "README.md",
                "root.a-stamped-roster-declares-its-own-ceiling",
            ),
        ];
        assert_eq!(double_routed_offences(&across_homes).len(), 1);
    }

    /// The positive control: distinct routes are lawful, and so is a route
    /// whose path merely SHARES a directory or a stem with another.
    ///
    /// A leg that flagged everything would satisfy the reversal above and be
    /// worthless, and one that resolved routes by anything looser than the exact
    /// path would refuse the two seats below for living beside each other.
    #[test]
    fn distinct_routes_are_lawful() {
        let distinct = [
            route(
                "testpak/tests/stamp_row_ceiling.rs",
                "src/05_bounds/README.md",
                "bounds.a-stamped-roster-declares-its-own-ceiling",
            ),
            route(
                "testpak/tests/planted_defect.rs",
                "src/07_bytes/README.md",
                "bytes.a-planted-defect-is-caught",
            ),
            route(
                "testpak/tests/stamp_row_ceilings.rs",
                "README.md",
                "root.a-neighbouring-seat",
            ),
        ];
        assert!(double_routed_offences(&distinct).is_empty());
    }

    /// The real repository holds: every executable seat its obligations route to
    /// is routed to by exactly one of them.
    ///
    /// Stated over the tree with the number it actually has. The green route
    /// population here is small, so this control is the statement that the tree
    /// is clean rather than a workout for the leg — the leg's own reversals are
    /// fixture rows, which is why they are written above and not here.
    #[test]
    fn the_real_obligations_route_to_each_seat_once() -> Result<(), String> {
        let routes = real_routes(repository_snapshot()?)?;
        assert!(
            !routes.is_empty(),
            "no green route found; the leg would be guarding nothing"
        );
        let found = double_routed_offences(&routes);
        assert!(found.is_empty(), "{found:?}");
        Ok(())
    }

    /// One synthetic `laws.rs` written in every shape the line reader could not
    /// see, and two it saw wrongly.
    ///
    /// Every law here is ordinary Rust the harness collects and runs. Not one of
    /// them reached the join's denominator while the reading was a line scan.
    ///
    /// The whole constant is fixture TEXT: `syn` parses it and no compiler ever
    /// sees it, so the `#[expect]` written below is one of the shapes under test
    /// rather than a lint hatch in this crate. `xtask` carries none of those, and
    /// the wall that forbids them is the compiler's rather than a scan's — which
    /// is the same distinction this reversal is about.
    const LAWS_A_LINE_READER_CANNOT_SEE: &str = r#"
mod root {
    /// A law whose documentation stands above its attribute.
    #[test]
    fn a_documented_law() {}

    #[test]
    #[should_panic = "the shape reversed"]
    fn a_law_that_must_panic() {}

    #[cfg(feature = "nothing-enables-this")]
    #[test]
    fn a_law_under_a_condition() {}

    #[expect(clippy::assertions_on_constants, reason = "the assertion is the law")]
    #[test]
    fn a_law_under_an_expectation() {}

    fn a_function_that_is_no_law() {}

    const SPELLED_IN_A_STRING: &str = "
#[test]
fn a_law_nobody_declared() {}
";

    mod deeper {
        #[test]
        fn a_law_one_module_further_in() {}
    }
}
"#;

    /// Planted reversal: every law shape the line reader dropped, and every
    /// non-law it would have picked up.
    ///
    /// THE defect this reader replacement exists for, and it was silent in the
    /// direction that matters. The scan required `#[test]` to stand alone on the
    /// previous line with the function opening the next one, so a second
    /// attribute, a documentation comment, or a condition written above the
    /// attribute took the law out of the denominator entirely — while cargo went
    /// on collecting and running it. A law missing from this side is a law no
    /// obligation has to claim, so the README could drop its row and the drift
    /// leg would have nothing to say: the population shrank and the join
    /// reported clean about it.
    ///
    /// The two directions it got wrong the other way are here too. A nested
    /// module's `mod` line was matched only flush at the file's left edge, so a
    /// law one module further in was attributed to the module ABOVE it and
    /// resolved against a target nobody wrote. And `#[test]` written on its own
    /// line inside a multi-line string literal opened a law that does not exist.
    ///
    /// A parse answers all five at once, because a parse is about items and
    /// their attributes rather than about lines.
    #[test]
    fn every_shape_the_line_reader_dropped_is_a_declared_law() -> Result<(), String> {
        let declared = laws_of(LAWS_A_LINE_READER_CANNOT_SEE)?;
        let spelled: Vec<String> = declared
            .iter()
            .map(|(module, law)| format!("{module}::{law}"))
            .collect();
        let found: Vec<&str> = spelled.iter().map(String::as_str).collect();
        assert_eq!(
            found,
            vec![
                "root::a_documented_law",
                "root::a_law_that_must_panic",
                "root::a_law_under_a_condition",
                "root::a_law_under_an_expectation",
                "root::deeper::a_law_one_module_further_in",
            ],
            "a law shape the harness runs is missing from the denominator"
        );
        assert!(
            !found.contains(&"root::a_law_nobody_declared"),
            "a law was read out of a string literal: {found:?}"
        );
        assert!(
            !found
                .iter()
                .any(|named| named.ends_with("a_function_that_is_no_law")),
            "a function carrying no harness attribute entered the denominator: {found:?}"
        );
        Ok(())
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
    fn a_seat_the_strict_reader_dropped_still_reaches_the_join() -> Result<(), String> {
        let existing = laws_of(ONE_LAW)?;
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
        Ok(())
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
    fn a_seat_row_carrying_more_than_its_target_reaches_the_join() -> Result<(), String> {
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
        let drifted = drifted_claim_offences(&claimed, &laws_of(ONE_LAW)?);
        assert!(
            drifted
                .iter()
                .any(|offence| offence.contains("claimed by no obligation")),
            "{drifted:?}"
        );
        Ok(())
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
    fn the_real_tooling_rows_name_a_reversal_before_their_prose() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let reversals = real_tree(snapshot).reversals;
        let continued: Vec<(String, String)> = tooling_rows(snapshot)?
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
        Ok(())
    }

    /// Planted reversal: the other direction of the same drift — a law standing
    /// in `laws.rs` that no obligation claims, which is a proof outliving the
    /// claim it was written for.
    #[test]
    fn a_law_no_obligation_claims_is_a_violation() -> Result<(), String> {
        let found = drifted_claim_offences(&[], &laws_of(ONE_LAW)?);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found
                .first()
                .is_some_and(|offence| offence.contains("claimed by no obligation")),
            "{found:?}"
        );
        Ok(())
    }

    /// The positive control: a claim and the law that answers it are no offence
    /// in either direction. A leg that flagged everything would satisfy both
    /// reversals above and be worthless.
    #[test]
    fn a_claim_and_the_law_answering_it_are_lawful() -> Result<(), String> {
        let claimed = seat_claims("    green: laws.rs root::a_law_somebody_wrote\n");
        assert_eq!(claimed.len(), 1, "{claimed:?}");
        let found = drifted_claim_offences(&claimed, &laws_of(ONE_LAW)?);
        assert!(found.is_empty(), "{found:?}");
        Ok(())
    }

    /// One fixture home README carrying two whole obligation records.
    const TWO_WHOLE_RECORDS: &str = "```yaml\n\
                                     obligations:\n\
                                     \x20 - id: bounds.budget-is-affine\n\
                                     \x20   challenge_kind: compile-refusal\n\
                                     \x20   green: laws.rs bounds::budget_is_affine\n\
                                     \x20   red: owed-to-testpak\n\
                                     \x20 - id: bounds.a-stamped-roster\n\
                                     \x20   challenge_kind: compile-refusal\n\
                                     \x20   green: testpak/tests/stamp_row_ceiling.rs\n\
                                     \x20   red: testpak/tests/compile-fail/a-roster.rs\n\
                                     ```\n";

    /// Planted reversal: an obligation whose `green:` row was deleted, and one
    /// whose `red:` row was deleted.
    ///
    /// THE defect this leg exists for, and it was silent in both directions. The
    /// rows were gathered by two independent scans of the whole file, so nothing
    /// bound a row to the obligation that wrote it and a deleted row was simply
    /// a row that was not there: the first record below stated no positive
    /// control, so none was resolved and the obligation qualified on its red row
    /// alone; the second took itself off the core denominator this repository
    /// publishes on every run and the number came back one smaller with nothing
    /// said. A `laws.rs` claim cannot vanish this way — the law it named is left
    /// claimed by nobody and the drift leg reports it from the other side — and a
    /// ROUTE has no other side at all. Nothing in testpak knows an obligation was
    /// supposed to point at it.
    #[test]
    fn a_record_that_lost_a_row_is_a_violation() {
        let text = "```yaml\n\
                    obligations:\n\
                    \x20 - id: bounds.no-positive-control\n\
                    \x20   challenge_kind: compile-law\n\
                    \x20   red: owed-to-testpak\n\
                    \x20 - id: bounds.no-reversal\n\
                    \x20   challenge_kind: compile-law\n\
                    \x20   green: laws.rs bounds::budget_is_affine\n\
                    ```\n";
        let found = record_field_offences(&obligation_records(text), "src/05_bounds/README.md");
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(
            found
                .iter()
                .all(|offence| offence.starts_with("src/05_bounds/README.md: obligation `")),
            "{found:?}"
        );
        assert!(
            found.first().is_some_and(|offence| {
                offence.contains("`bounds.no-positive-control`") && offence.contains("0 `green:`")
            }),
            "{found:?}"
        );
        assert!(
            found.last().is_some_and(|offence| {
                offence.contains("`bounds.no-reversal`") && offence.contains("0 `red:`")
            }),
            "{found:?}"
        );
    }

    /// Planted reversal: an obligation stating two `green:` rows, and one
    /// stating two `red:` rows.
    ///
    /// The other half of the same rule. Two positive controls for one claim say
    /// nowhere which of them the claim rests on, and a reader that took the
    /// first would be spending evidence the record never chose; two reversals
    /// put one obligation on a published ledger twice, which moves a number the
    /// campaign reports without an obligation behind it.
    #[test]
    fn a_record_stating_a_row_twice_is_a_violation() {
        let text = "```yaml\n\
                    obligations:\n\
                    \x20 - id: bounds.two-positive-controls\n\
                    \x20   green: laws.rs bounds::budget_is_affine\n\
                    \x20   green: testpak/tests/stamp_row_ceiling.rs\n\
                    \x20   red: owed-to-testpak\n\
                    \x20 - id: bounds.two-reversals\n\
                    \x20   green: laws.rs bounds::charge_shrinks_or_refuses\n\
                    \x20   red: owed-to-testpak\n\
                    \x20   red: testpak/tests/compile-fail/a-roster.rs\n\
                    ```\n";
        let found = record_field_offences(&obligation_records(text), "src/05_bounds/README.md");
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(
            found.first().is_some_and(|offence| {
                offence.contains("`bounds.two-positive-controls`") && offence.contains("2 `green:`")
            }),
            "{found:?}"
        );
        assert!(
            found.last().is_some_and(|offence| {
                offence.contains("`bounds.two-reversals`") && offence.contains("2 `red:`")
            }),
            "{found:?}"
        );
    }

    /// The positive control: a record stating exactly one of each row is no
    /// offence, whichever green spelling it uses.
    ///
    /// A leg that flagged everything would satisfy both reversals above and
    /// would refuse all 195 records in this tree.
    #[test]
    fn a_record_stating_one_of_each_row_is_lawful() {
        let records = obligation_records(TWO_WHOLE_RECORDS);
        assert_eq!(records.len(), 2, "{}", records.len());
        let found = record_field_offences(&records, "src/05_bounds/README.md");
        assert!(found.is_empty(), "{found:?}");
    }

    /// Planted reversal: a record with no identity at all, and two records in
    /// one home sharing one.
    ///
    /// The identity leg, and both halves are the same defect the row legs refuse
    /// arriving through the KEY instead of through the row. An id is what a
    /// record's own rows are attributed to and what a routed seat's control
    /// marker names back, so an empty one leaves rows nothing can attribute and
    /// a marker nothing can name, and a shared one lets one marker discharge two
    /// claims — which is exactly what [`double_routed_offences`] and
    /// [`double_claimed_offences`] refuse on the evidence side, unrefused on the
    /// claim side.
    ///
    /// Pure over its records, so the leg is proven against fixture records
    /// rather than by emptying an id in a README the repository stands on.
    #[test]
    fn a_record_with_no_identity_or_a_shared_one_is_a_violation() {
        let nameless = "```yaml\n\
                        home: bounds\n\
                        obligations:\n\
                        \x20 - id:\n\
                        \x20   green: laws.rs bounds::budget_is_affine\n\
                        \x20   red: owed-to-testpak\n\
                        ```\n";
        let found = record_field_offences(&obligation_records(nameless), "src/05_bounds/README.md");
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found
                .first()
                .is_some_and(|offence| offence.contains("states no identity")),
            "{found:?}"
        );

        let shared = "```yaml\n\
                      home: bounds\n\
                      obligations:\n\
                      \x20 - id: bounds.one-key-two-claims\n\
                      \x20   green: laws.rs bounds::budget_is_affine\n\
                      \x20   red: owed-to-testpak\n\
                      \x20 - id: bounds.one-key-two-claims\n\
                      \x20   green: laws.rs bounds::charge_shrinks_or_refuses\n\
                      \x20   red: owed-to-testpak\n\
                      ```\n";
        let doubled = record_field_offences(&obligation_records(shared), "src/05_bounds/README.md");
        assert_eq!(doubled.len(), 1, "one offence per shared key: {doubled:?}");
        assert!(
            doubled.first().is_some_and(|offence| {
                offence.contains("`bounds.one-key-two-claims`")
                    && offence.contains("declared by 2 records")
            }),
            "{doubled:?}"
        );
    }

    /// The real repository holds: every obligation names itself, and no two
    /// records in one home name the same thing.
    ///
    /// Stated over the tree because the identity leg is new and a leg that
    /// arrives already refusing something is a leg somebody will weaken. It
    /// found nothing: every record in every home README states an id, and every
    /// id within a home is written once.
    #[test]
    fn the_real_records_each_name_themselves_once() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let mut offences = Vec::new();
        let mut records = 0_usize;
        for path in home_readmes(snapshot) {
            let home = path.to_string();
            let document = snapshot.markdown().document(&path).taken(&home)?;
            let ledger = obligation_ledger(document, &home).taken(&home)?;
            records = records.saturating_add(ledger.records.len());
            offences.extend(record_field_offences(&ledger.records, &home));
        }
        assert!(records > 1, "the leg would be guarding nothing: {records}");
        assert!(offences.is_empty(), "{offences:?}");
        Ok(())
    }

    /// Planted reversal: rows written where no obligation record owns them.
    ///
    /// The reading's own failure mode, refused rather than trusted. This join
    /// reads rows THROUGH the record that declared them, so a row standing
    /// outside every record is a row nothing joins — which would be the original
    /// silence arriving through the repair itself. The reading names it against
    /// the README that wrote it rather than dropping it.
    #[test]
    fn a_row_no_record_owns_is_a_violation() {
        let text = "```yaml\n\
                    home: bounds\n\
                    obligations:\n\
                    \x20 green: laws.rs bounds::a_row_above_every_record\n\
                    \x20 red: owed-to-testpak\n\
                    \x20 - id: bounds.the-one-real-record\n\
                    \x20   green: laws.rs bounds::budget_is_affine\n\
                    \x20   red: owed-to-testpak\n\
                    ```\n";
        let ledger = fixture_ledger(text);
        assert_eq!(ledger.records.len(), 1, "{}", ledger.records.len());
        assert_eq!(ledger.offences.len(), 2, "{:?}", ledger.offences);
        assert!(
            ledger
                .offences
                .iter()
                .all(|offence| offence.contains("stands outside every obligation record")),
            "{:?}",
            ledger.offences
        );
    }

    /// The positive control: rows written inside the records that declared them
    /// are owned, and the reading says nothing.
    #[test]
    fn rows_written_inside_their_records_are_owned() {
        let ledger = fixture_ledger(TWO_WHOLE_RECORDS);
        assert_eq!(ledger.records.len(), 2, "{}", ledger.records.len());
        assert!(ledger.offences.is_empty(), "{:?}", ledger.offences);
    }

    /// The real repository holds: every obligation is a whole record, and every
    /// row is owned by one.
    ///
    /// The denominator is stated as a RELATION rather than as a number. One
    /// obligation record declares one `red:` row, so the core count this
    /// repository publishes on every run is exactly the number of records its
    /// home READMEs carry — and a record that lost its row, or a row that lost
    /// its record, moves the two sides apart instead of quietly moving the
    /// published figure.
    #[test]
    fn the_real_records_are_whole_and_own_every_row() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let readmes = home_readmes(snapshot);
        assert!(!readmes.is_empty(), "no home READMEs found");
        let mut declared = 0usize;
        let mut ledger_rows = 0usize;
        let mut offences = Vec::new();
        for readme in &readmes {
            let document = snapshot
                .markdown()
                .document(readme)
                .taken(readme.as_str())?;
            let home = readme.to_string();
            let ledger = obligation_ledger(document, &home).taken(&home)?;
            declared = declared.saturating_add(ledger.records.len());
            ledger_rows =
                ledger_rows.saturating_add(ledger.records.iter().fold(0usize, |total, record| {
                    total.saturating_add(record.red.len())
                }));
            offences.extend(record_field_offences(&ledger.records, &home));
            offences.extend(ledger.offences);
        }
        assert!(offences.is_empty(), "{offences:?}");
        assert!(
            declared > 0,
            "no obligation record found in any home README"
        );
        assert_eq!(
            declared, ledger_rows,
            "{declared} obligation records declare {ledger_rows} red rows; the published core \
             denominator is one row per record"
        );
        Ok(())
    }

    /// Planted reversal: a routed seat that exists, runs tests, and holds no
    /// test naming the obligation that routed to it.
    ///
    /// THE defect the seat check could not see. Seating a file answers an
    /// EXISTENTIAL — something in there runs — so any test left in the file
    /// answers it, including one written for another claim entirely and one left
    /// behind after the declared control was renamed or deleted. Both rows below
    /// resolve against a real seat and every path-shaped reading says yes; only
    /// the naming refuses them.
    #[test]
    fn a_routed_seat_naming_no_control_is_a_violation() {
        let seats = controlled_population(&[(
            "testpak/tests/stamp_row_ceiling.rs",
            "root.a-stamped-roster-declares-its-own-ceiling",
        )]);

        // The control was deleted and an unrelated test kept the file seated.
        let unnamed = green_population(&["testpak/tests/stamp_row_ceiling.rs"]);
        let deleted = uncontrolled_green_routes(
            &[route(
                "testpak/tests/stamp_row_ceiling.rs",
                "README.md",
                "root.a-stamped-roster-declares-its-own-ceiling",
            )],
            &unnamed,
        );
        assert_eq!(deleted.len(), 1, "{deleted:?}");
        assert!(
            deleted.first().is_some_and(|offence| {
                offence.contains("root.a-stamped-roster-declares-its-own-ceiling")
                    && offence.contains("no test in that file names it")
            }),
            "{deleted:?}"
        );

        // The seat's tests name a DIFFERENT obligation: half of a rename.
        let renamed = uncontrolled_green_routes(
            &[route(
                "testpak/tests/stamp_row_ceiling.rs",
                "README.md",
                "root.a-stamped-roster-declares-its-ceiling",
            )],
            &seats,
        );
        assert_eq!(renamed.len(), 1, "{renamed:?}");

        // The seat exists, so the phantom leg is silent: only this leg refuses
        // either of them, and a resolver fix never reaches it.
        assert!(
            phantom_green_routes(&routed(&["testpak/tests/stamp_row_ceiling.rs"]), &seats)
                .is_empty()
        );
    }

    /// The positive control: a route whose seat holds a test naming that exact
    /// obligation is lawful, and a route naming a seat nobody wrote is left to
    /// the phantom leg rather than answered twice.
    #[test]
    fn a_routed_seat_that_names_its_obligation_is_lawful() {
        let seats = controlled_population(&[(
            "testpak/tests/stamp_row_ceiling.rs",
            "root.a-stamped-roster-declares-its-own-ceiling",
        )]);
        let lawful = route(
            "testpak/tests/stamp_row_ceiling.rs",
            "README.md",
            "root.a-stamped-roster-declares-its-own-ceiling",
        );
        assert!(uncontrolled_green_routes(&[lawful], &seats).is_empty());

        let missing = route(
            "testpak/tests/nobody-ever-wrote-this.rs",
            "README.md",
            "root.a-stamped-roster-declares-its-own-ceiling",
        );
        assert!(
            uncontrolled_green_routes(&[missing], &seats).is_empty(),
            "a route naming a file nobody wrote earned a second offence"
        );
    }

    /// The marker is read off the test's DOCUMENTATION and never off the file's
    /// lines, and this is the test that says so.
    ///
    /// Each source below spells the marker in its bytes and none of them
    /// documents a running test with it: an ordinary comment is no attribute, a
    /// string literal is a value rather than an attribute, the file's own `//!`
    /// documentation is an attribute on the FILE, and a doc comment on a helper
    /// is an attribute on a function the harness never runs. A line scan seats
    /// every one of them as controlled — the same class of reader this module
    /// has already replaced for `#[test]` itself — and the parse names none of
    /// them.
    ///
    /// All four are seats: each declares a test that runs. What they do not do
    /// is control the obligation their bytes mention, which is the whole
    /// distinction between reading a file and reading its items.
    #[test]
    fn a_marker_outside_a_running_tests_documentation_controls_nothing() {
        let claimed = "root.a-claim-nobody-controls";
        let sources = [
            top_level(
                "a_marker_in_an_ordinary_comment.rs",
                "// green: root.a-claim-nobody-controls\n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_marker_inside_a_string.rs",
                "const SPELLED: &str = \"green: root.a-claim-nobody-controls\";\n\
                 #[test]\n\
                 fn the_behaviour_holds() {\n\
                 \x20   assert!(!SPELLED.is_empty());\n\
                 }\n",
            ),
            top_level(
                "a_marker_in_the_files_own_documentation.rs",
                "//! green: root.a-claim-nobody-controls\n\
                 \n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
            top_level(
                "a_marker_on_a_function_that_is_no_test.rs",
                "/// green: root.a-claim-nobody-controls\n\
                 fn a_helper() {}\n\
                 \n\
                 #[test]\n\
                 fn the_behaviour_holds() {}\n",
            ),
        ];
        let (seats, unparsable) = seat_population(&sources);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        assert_eq!(seats.0.len(), sources.len(), "{:?}", seats.0);
        for (path, _) in &sources {
            assert!(seats.carries(path), "{path} stopped being a seat");
            assert!(
                !seats.controls(path, claimed),
                "{path} controlled an obligation its documentation never names"
            );
        }
    }

    /// The positive control for that reading: a marker written the way this
    /// repository writes one — a documentation line on the test itself, opening
    /// with the obligation row's own word and running on into prose — names the
    /// obligation, and the route that declared it resolves.
    ///
    /// A reader that named nothing would satisfy the reversal above and would
    /// unresolve the only green route in the tree.
    #[test]
    fn a_marker_documenting_a_running_test_names_its_obligation() {
        let (seats, unparsable) = seat_population(&[top_level(
            "a_documented_control.rs",
            "/// green: root.a-stamped-roster-declares-its-own-ceiling — the stamp admits a\n\
             /// declaration that spends its declared supply of positions.\n\
             #[test]\n\
             fn a_stamped_roster_declares_its_own_ceiling() {}\n",
        )]);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        assert!(
            seats.controls(
                "testpak/tests/a_documented_control.rs",
                "root.a-stamped-roster-declares-its-own-ceiling"
            ),
            "{:?}",
            seats.0
        );
        let found = uncontrolled_green_routes(
            &[route(
                "testpak/tests/a_documented_control.rs",
                "README.md",
                "root.a-stamped-roster-declares-its-own-ceiling",
            )],
            &seats,
        );
        assert!(found.is_empty(), "{found:?}");
    }

    /// Planted reversal: the declared control is marked, and it is a test the
    /// harness never RUNS — skipped in one file, compiled out in the other —
    /// while an unrelated test keeps each file seated.
    ///
    /// The exact shape of the defect one attribute deeper. Both files are
    /// genuinely seats, both spell the marker on the function it belongs to, and
    /// in neither does the marked control execute. A marker gathered from every
    /// documented function would call both controlled; gathered only from the
    /// tests this reader already knows the harness runs, neither is.
    #[test]
    fn a_marker_on_a_test_that_never_runs_controls_nothing() {
        let claimed = "root.a-stamped-roster-declares-its-own-ceiling";
        let sources = [
            top_level(
                "a_skipped_control.rs",
                "/// green: root.a-stamped-roster-declares-its-own-ceiling — owed until the\n\
                 /// roster lands.\n\
                 #[test]\n\
                 #[ignore = \"owed until the roster lands\"]\n\
                 fn a_stamped_roster_declares_its_own_ceiling() {}\n\
                 \n\
                 #[test]\n\
                 fn something_else_entirely() {}\n",
            ),
            top_level(
                "a_conditional_control.rs",
                "/// green: root.a-stamped-roster-declares-its-own-ceiling — under a build\n\
                 /// nobody named.\n\
                 #[cfg(feature = \"a-feature-nobody-enables\")]\n\
                 #[test]\n\
                 fn a_stamped_roster_declares_its_own_ceiling() {}\n\
                 \n\
                 #[test]\n\
                 fn something_else_entirely() {}\n",
            ),
        ];
        let (seats, unparsable) = seat_population(&sources);
        assert!(unparsable.is_empty(), "{unparsable:?}");
        assert_eq!(seats.0.len(), sources.len(), "{:?}", seats.0);
        for (path, _) in &sources {
            assert!(seats.carries(path), "{path} stopped being a seat");
            assert!(
                !seats.controls(path, claimed),
                "{path} controlled an obligation through a test nothing executes"
            );
        }
    }

    /// The real repository holds: every green route resolves to the CONTROL it
    /// names, and the one route this tree writes is the one this leg was built
    /// for.
    ///
    /// The route, the obligation, and the marker are all named here rather than
    /// counted, because there is exactly one of each and the point of the leg is
    /// that the three are one identity. A rename at either end fails this test
    /// by name.
    #[test]
    fn the_real_route_resolves_to_the_control_it_names() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let seats = real_tree(snapshot).seats;
        let routes = real_routes(snapshot)?;
        assert!(
            !routes.is_empty(),
            "no green route found; the leg would be guarding nothing"
        );
        let found = uncontrolled_green_routes(&routes, &seats);
        assert!(found.is_empty(), "{found:?}");
        assert!(
            routes.iter().any(|route| {
                route.named == "testpak/tests/stamp_row_ceiling.rs"
                    && route.id == "root.a-stamped-roster-declares-its-own-ceiling"
            }),
            "the tree's one green route is not the one this control names: {routes:?}"
        );
        assert!(
            seats.controls(
                "testpak/tests/stamp_row_ceiling.rs",
                "root.a-stamped-roster-declares-its-own-ceiling"
            ),
            "the routed seat no longer documents the obligation it controls"
        );
        Ok(())
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
    ///
    /// # The denominator is PINNED, because a reader was replaced
    ///
    /// ONE HUNDRED AND EIGHTY-FOUR laws, and the number is written here rather
    /// than merely compared against the claims. The line reader this join used
    /// to call could not see a law carrying a second attribute, a documentation
    /// comment, or a nested module, and could read a law out of a string literal
    /// — so replacing it with a parse could have moved this population in either
    /// direction with both sides moving together and nothing saying so. Measured
    /// across the replacement: 183 before, 183 after, every pair identical. The
    /// reader changed and the tree did not, which is the only way a reader
    /// replacement is allowed to settle.
    ///
    /// THE NUMBER HAS MOVED ONCE SINCE, BY ONE, AND THE LAW IS NAMED. Joining
    /// the capacity-authority boundary added `root::a_family_declares_one_
    /// capacity_authority`, so this population is 184. That is what a pinned
    /// denominator is for: two boundaries settled apart, one of them grew the
    /// proof surface, and the number refused the join until somebody said which
    /// law arrived. A count that moved silently here would have been the same
    /// defect the reader replacement was written to end.
    #[test]
    fn the_real_seats_are_the_real_laws() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let claimed = real_claims(snapshot)?;
        let laws = snapshot
            .rust()
            .functions_in(&CanonicalPath::spelled(super::PROOF_SURFACE))
            .taken(super::PROOF_SURFACE)?;
        let existing = declared_laws(&laws);
        assert_eq!(
            existing.len(),
            184,
            "laws.rs declares {} laws",
            existing.len()
        );
        assert_eq!(
            claimed.len(),
            existing.len(),
            "{} seats claimed, {} laws declared",
            claimed.len(),
            existing.len()
        );
        let found = drifted_claim_offences(&claimed, &existing);
        assert!(found.is_empty(), "{found:?}");
        Ok(())
    }
}
