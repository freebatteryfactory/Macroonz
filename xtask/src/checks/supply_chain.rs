//! The dependency gate's committed artifacts are counted where a machine reads.
//!
//! `deny.toml` settles what the resolved dependency graph is allowed to be,
//! `deny-reversal.toml` is the deliberately wrong configuration that proves the
//! rule still refuses, and `.github/workflows/dependencies.yml` is where both
//! are run. Those three files ARE the gate as this repository commits it, and
//! until this law was registered they stood outside every denominator the
//! repository publishes: the red ledgers in `crate::checks::obligations` count
//! testpak's tests and its compile-fail fixtures, and a configuration beside a
//! hosted workflow is neither. Deleting the reversal moved no number and failed
//! no check, so qualification kept reading whole after the checker had
//! disappeared.
//!
//! That gap was already WRITTEN DOWN, in the reversal's own prose, and writing
//! a gap down does not close it. Prose keeps reading as true after the thing it
//! describes is gone — which is the single failure the whole obligations join
//! was built to refuse, arriving one file to the side of the join.
//!
//! # What is claimed here, and the name that came down to claim it
//!
//! Four facts about FILES: the workflow file is committed, the rule set is
//! committed, the reversal is committed, and the reversal states something that
//! is not byte-for-byte the rule set. The gate's artifacts are PRESENT and
//! VISIBLY DISTINCT. That is the whole claim, and the roster entry now says so
//! in those words.
//!
//! It was registered first as `dependency-gate-carries-its-reversal`, and that
//! name asserted a hosted-execution fact this body never reads. That a gate
//! CARRIES its reversal is a statement about a RUN; nothing here starts one, and
//! no reading below comes near one. A name claiming more than its body
//! establishes is the exact defect this law was registered to end, and the first
//! spelling of the cure committed it, one line into the roster.
//!
//! Two repairs exist for that shape and only one of them was available here. A
//! control that CAN be widened to cover its name is widened, and the name
//! stands. This one cannot be: establishing that a workflow still invokes the
//! reversal means reading the workflow, that means a YAML reader this repository
//! has not admitted, and the alternative — matching text in a workflow file — is
//! the class of reader `crate::checks::obligations` has now replaced four
//! separate times, each replacement made because a semantic category recognized
//! by a convenient spelling is a category the next spelling walks past. So the
//! claim came down to the body rather than the body being sent up to the claim,
//! and what this law is called is now what this law establishes.
//!
//! # Where the gate is counted, and why here
//!
//! In the roster `main.rs` holds, which that file states is the only statement
//! of how many repository laws there are. The gate occupies one line of it:
//! present, the run prints one more PASS than it did; deleted along with this
//! law, the roster is one shorter and the deletion is a line of a diff nobody
//! can miss; deleted on its own, this law FAILS and takes stage four of
//! `cargo xtask qualify` with it. Renaming the law moved no line: the roster is
//! one seat shorter or longer only when a law arrives or goes, and a claim
//! narrowed to its evidence is neither.
//!
//! The reversal LEDGER was the other seat considered, and it lost on what it
//! would have cost. Admitting a committed artifact that is not a testpak test
//! into the red population means loosening the resolver that ledger's exactness
//! rests on — the population is drawn from one walk of `testpak/tests/`, and
//! every other reader in that module is deliberately narrow and says so — and
//! the loosening would apply to every `red:` row in every home, not to the one
//! row that wanted it. It would also have to be written in a tooling ledger
//! whose home does not own the gate: `macros/macroc` and `testpak` are the two
//! declared tooling ledgers, and the supply chain belongs to neither. A claim
//! seated in a home that does not own it is the duplicate authority this
//! repository is eliminating, bought at the price of a weaker join over a
//! published number.
//!
//! # What this law reads, and it is not a parse
//!
//! Three files are present, the reversal states something, and the reversal is
//! not byte-for-byte the rule set it stands against. That is the whole reading:
//! four facts about files, no grammar entered, no line matched, no structure
//! inferred. What a departure MEANS is deliberately not asked here — see the
//! nonclaims below.
//!
//! # The two roads a gate disappears by, and which one this reaches
//!
//! **The committed artifacts go.** The reversal deleted, the rule set deleted,
//! the workflow file deleted, the reversal emptied, or the reversal quietly
//! synchronized with the rule set until it is no longer wrong. Every one of
//! those is reached here, and each is a planted reversal below.
//!
//! **The step goes while the workflow stays.** Someone edits the reversal step
//! out of `.github/workflows/dependencies.yml` and leaves the file, the rule
//! set, and the reversal exactly where they are. Whether the hosted step still
//! INVOKES the reversal is unestablished here — not established false, not
//! established true, unread — and this law FAILS OPEN on it: everything here
//! goes on passing while the refusal nobody watches has stopped being watched
//! again. The reversals below pin that ceiling as an executed fact rather than
//! as a sentence, because a sentence is what this whole law exists to stop
//! trusting.
//!
//! The opening condition is the versioned claim and evidence schema, and
//! specifically the same half of it every ceiling in the obligations join is
//! waiting on: a hosted run publishing the roster of what it EXECUTED. A step
//! that ran appears in such a roster and a step somebody deleted does not, which
//! answers the question by reading an output rather than by guessing at a
//! source. That schema owns hosted evidence; a second, weaker opinion about it
//! seated in this module would be the duplicate authority the seat argument
//! above refuses. Until then the step's removal is caught by review and by
//! nothing else, and this module says so rather than letting the roster entry
//! imply otherwise.
//!
//! # Nonclaims
//!
//! It does not claim the hosted step still INVOKES the reversal, and that is the
//! claim the old name made. The step is read by nobody here.
//!
//! It does not claim the gate RUNS. Nothing here starts cargo-deny, and
//! `deny.toml` states why the entry bar cannot: a separately installed binary
//! would make qualification depend on what a machine happens to have.
//!
//! It does not claim the departure is the PLANTED one. That the reversal
//! differs from the rule set is established here; that it differs by the
//! `[[bans.features]]` mismatch it was written for is established by the hosted
//! step's own requirement of exit 2 AND the `exact-features-mismatch`
//! diagnostic — and that requirement stands exactly as long as the step does.
//!
//! It does not claim anything about the four rules `deny-reversal.toml` names
//! as uncovered. Those stand on a positive invocation alone, the reversal says
//! so, and counting the reversal does not discharge them.

use std::fs;
use std::path::{Path, PathBuf};

/// The hosted workflow the dependency gate runs in, in path segments so the
/// join spells no separator of its own.
const GATE_WORKFLOW: [&str; 3] = [".github", "workflows", "dependencies.yml"];

/// The configuration that settles what the resolved graph is allowed to be.
const RULE_SET: &str = "deny.toml";

/// The deliberately wrong configuration that proves the rule still refuses.
const PLANTED_REVERSAL: &str = "deny-reversal.toml";

/// The dependency gate's committed artifacts are present and visibly distinct.
///
/// Four offences, reported together rather than one per run, because they are
/// independent of one another and a repairer who deleted two parts should learn
/// about both at once.
///
/// Every offence is a statement about a committed file, because a committed file
/// is the whole of what this reading opens. What a hosted run does with these
/// files is named where it is conditional and claimed nowhere.
pub(crate) fn check_dependency_gate_artifacts(root: &Path) -> Result<(), String> {
    let mut offences = Vec::new();
    if !gate_workflow(root).is_file() {
        offences.push(format!(
            "the dependency gate's hosted seat is not committed: `{}` is not there, so no file in \
             this repository names a run that would read `{RULE_SET}` or the reversal beside it, \
             and what the resolved graph is allowed to be is settled by nobody",
            spelled_workflow()
        ));
    }
    let rule_set = committed_bytes(root, RULE_SET)?;
    if rule_set.is_none() {
        offences.push(format!(
            "`{RULE_SET}` is not there: it is the rule set the committed reversal stands against, \
             and a reversal standing against a rule set nobody wrote departs from nothing"
        ));
    }
    match committed_bytes(root, PLANTED_REVERSAL)? {
        None => offences.push(format!(
            "`{PLANTED_REVERSAL}` is not there: it is this gate's planted reversal, the one \
             committed artifact by which the rule's REFUSAL can be watched at all, and nothing in \
             the tree now carries a deliberately wrong configuration"
        )),
        Some(reversal) => {
            if !states_something(&reversal) {
                offences.push(format!(
                    "`{PLANTED_REVERSAL}` states nothing at all: a configuration carrying no rule \
                     cannot be wrong, so cargo-deny succeeds against it wherever it is run, and \
                     the artifact has stopped being a reversal"
                ));
            }
            if rule_set.as_deref() == Some(reversal.as_slice()) {
                offences.push(format!(
                    "`{PLANTED_REVERSAL}` is byte-for-byte `{RULE_SET}`: the planted reversal is a \
                     copy of the rule it stands against, so it is no longer distinct from it and \
                     carries nothing wrong left to refuse"
                ));
            }
        }
    }
    if offences.is_empty() {
        Ok(())
    } else {
        Err(offences.join("; "))
    }
}

/// Where the gate's hosted seat sits under one root.
fn gate_workflow(root: &Path) -> PathBuf {
    let mut path = root.to_path_buf();
    for segment in GATE_WORKFLOW {
        path.push(segment);
    }
    path
}

/// The workflow as this repository spells paths — relative, forward slashes —
/// so a run on any machine names the same file.
fn spelled_workflow() -> String {
    GATE_WORKFLOW.join("/")
}

/// The bytes one committed part carries, or nothing where no file is there.
///
/// A part that is ABSENT is an offence this law reports; a part that is present
/// and unreadable is a failure of the run itself, because whether it departs
/// from anything is then unknown rather than false.
fn committed_bytes(root: &Path, relative: &str) -> Result<Option<Vec<u8>>, String> {
    let path = root.join(relative);
    if !path.is_file() {
        return Ok(None);
    }
    fs::read(&path)
        .map(Some)
        .map_err(|e| format!("{}: {e}", path.display()))
}

/// Whether a committed part states anything at all: one byte that is not
/// whitespace.
///
/// Read as bytes and nothing further. What the file says is the hosted run's
/// question and cargo-deny's to answer; that it says SOMETHING is a fact a
/// reader can establish without opening a grammar.
fn states_something(bytes: &[u8]) -> bool {
    bytes.iter().any(|byte| !byte.is_ascii_whitespace())
}

/// Planted reversals for a law whose subject is the tree.
///
/// The gate is a set of files, so its reversals are planted against a scratch
/// root outside the repository: the law that counts the gate's artifacts is
/// never proven by deleting them. The last two tests read the ceiling and the
/// real tree, and each states what it found.
#[cfg(test)]
mod tests {
    use super::{GATE_WORKFLOW, PLANTED_REVERSAL, RULE_SET, check_dependency_gate_artifacts};
    use crate::checks::scratch::Scratch;
    use crate::repository::walk::repo_root;
    use std::fs;
    use std::path::PathBuf;

    /// A fixture rule set, standing for `deny.toml`.
    const RULE_SET_FIXTURE: &str = "[bans]\nmultiple-versions = \"deny\"\n";

    /// A fixture reversal, standing for `deny-reversal.toml`: the same shape,
    /// deliberately not the same bytes.
    const REVERSAL_FIXTURE: &str =
        "[[bans.features]]\ncrate = \"syn\"\nallow = [\"fold\"]\nexact = true\n";

    /// A fixture workflow, standing for the gate's hosted seat.
    const WORKFLOW_FIXTURE: &str = "name: dependencies\n";

    /// One scratch root carrying a whole, lawful gate.
    fn planted(name: &str) -> Scratch {
        let scratch = Scratch::named(name);
        scratch.write(&GATE_WORKFLOW.join("/"), WORKFLOW_FIXTURE);
        scratch.write(RULE_SET, RULE_SET_FIXTURE);
        scratch.write(PLANTED_REVERSAL, REVERSAL_FIXTURE);
        scratch
    }

    /// The positive control: a gate carrying all three of its artifacts, with a
    /// reversal that states something and departs from the rule it stands
    /// against, is lawful. A law that refused everything would satisfy every
    /// reversal below and be worthless.
    #[test]
    fn present_and_distinct_artifacts_are_lawful() {
        let scratch = planted("gate-whole");
        let found = check_dependency_gate_artifacts(scratch.root());
        assert!(found.is_ok(), "{found:?}");
    }

    /// Planted reversal: the reversal artifact deleted. This is the failure the
    /// law exists for — the one committed thing by which the rule's refusal can
    /// be watched is gone, and nothing else in the tree notices.
    #[test]
    fn a_deleted_reversal_is_a_violation() {
        let scratch = planted("gate-reversal-deleted");
        let _removed = fs::remove_file(scratch.root().join(PLANTED_REVERSAL));
        let found = check_dependency_gate_artifacts(scratch.root());
        assert!(
            found.is_err_and(
                |reason| reason.contains(PLANTED_REVERSAL) && reason.contains("is not there")
            ),
            "a deleted reversal passed the law that counts it"
        );
    }

    /// Planted reversal: the whole hosted workflow deleted. The gate's two
    /// configurations survive and no committed file names a run that reads them.
    #[test]
    fn a_deleted_workflow_is_a_violation() {
        let scratch = planted("gate-workflow-deleted");
        let mut workflow = scratch.root().to_path_buf();
        for segment in GATE_WORKFLOW {
            workflow.push(segment);
        }
        let _removed = fs::remove_file(&workflow);
        let found = check_dependency_gate_artifacts(scratch.root());
        assert!(
            found.is_err_and(|reason| reason.contains("hosted seat is not committed")),
            "a deleted workflow passed the law that counts the gate's artifacts"
        );
    }

    /// Planted reversal: the rule set deleted, leaving a reversal standing
    /// against nothing.
    #[test]
    fn a_deleted_rule_set_is_a_violation() {
        let scratch = planted("gate-rule-set-deleted");
        let _removed = fs::remove_file(scratch.root().join(RULE_SET));
        let found = check_dependency_gate_artifacts(scratch.root());
        assert!(
            found.is_err_and(|reason| reason.contains(RULE_SET) && reason.contains("is not there")),
            "a deleted rule set passed"
        );
    }

    /// Planted reversal: the reversal emptied rather than deleted — the same
    /// disappearance written one edit shallower, and the one a file listing
    /// would report as present.
    #[test]
    fn an_emptied_reversal_is_a_violation() {
        let scratch = planted("gate-reversal-emptied");
        scratch.write(PLANTED_REVERSAL, "\n   \n");
        let found = check_dependency_gate_artifacts(scratch.root());
        assert!(
            found.is_err_and(|reason| reason.contains("states nothing at all")),
            "an emptied reversal passed"
        );
    }

    /// Planted reversal: the reversal synchronized with the rule set until it
    /// is no longer wrong.
    ///
    /// The most expensive spelling of the defect, because every artifact is
    /// present and a listing of the tree says the gate is whole. A reversal that
    /// agrees with the rule it stands against is not distinct from it, and the
    /// one thing the file was committed to be — a configuration that is wrong on
    /// purpose — has quietly stopped being true of it.
    #[test]
    fn a_reversal_that_stopped_departing_is_a_violation() {
        let scratch = planted("gate-reversal-synchronized");
        scratch.write(PLANTED_REVERSAL, RULE_SET_FIXTURE);
        let found = check_dependency_gate_artifacts(scratch.root());
        assert!(
            found.is_err_and(|reason| reason.contains("byte-for-byte")),
            "a reversal that is a copy of its rule set passed"
        );
    }

    /// The ceiling, executed rather than only written down: a workflow that
    /// invokes nothing at all still passes this law.
    ///
    /// It proves nothing about the gate, and it is not a positive control. What
    /// it holds in place is the NAME. The fixture workflow here carries no job
    /// and no step, exactly as a workflow somebody edited the reversal step out
    /// of carries none, and this law is content with both — which is the fail
    /// open the module documentation states. A later reader who widens this law
    /// to read what a hosted run executed has to delete this test to do it, and
    /// that deletion is the line of the diff where the claim is allowed to grow
    /// back.
    #[test]
    fn a_workflow_that_invokes_nothing_still_passes() {
        let scratch = planted("gate-workflow-invokes-nothing");
        scratch.write(
            &GATE_WORKFLOW.join("/"),
            "name: dependencies\non:\n  pull_request:\njobs: {}\n",
        );
        let found = check_dependency_gate_artifacts(scratch.root());
        assert!(
            found.is_ok(),
            "this law reads files and not steps; a verdict here would mean it had started reading \
             steps without its name saying so: {found:?}"
        );
    }

    /// The real repository holds: the gate's three artifacts are committed, and
    /// the reversal states something that is not the rule set.
    #[test]
    fn the_real_gate_artifacts_are_present_and_distinct() {
        let root = repo_root().unwrap_or_else(|_| PathBuf::from("."));
        let found = check_dependency_gate_artifacts(&root);
        assert!(found.is_ok(), "{found:?}");
    }
}
