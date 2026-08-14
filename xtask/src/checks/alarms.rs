//! The two alarms standing beside the entry bar are counted where a machine
//! reads.
//!
//! `cargo xtask qualify` is the entry bar and the only spelling of it. Two
//! surfaces stand BESIDE it rather than inside it, each for a reason written at
//! its own home: the second harness, which runs the workspace's tests one
//! process per test where the bar's `tests` stage runs them as threads in one,
//! and the mutation alarm, which damages one decision at a time and asks whether
//! any test notices. Neither is a gate, neither becomes one, and both are
//! committed as files a hosted job reads:
//!
//!   - `.config/nextest.toml` and `.github/workflows/harness.yml`;
//!   - `.cargo/mutants.toml` and `.github/workflows/mutation.yml`.
//!
//! # Admission requirement six, unsatisfied, and this is it
//!
//! Until this law was registered those four files stood outside every
//! denominator this repository publishes. The red ledgers in
//! `crate::checks::obligations` count testpak's tests and its compile-fail
//! fixtures, and a configuration beside a hosted workflow is neither; the
//! mutation run derives a denominator of its own, but a run publishes it rather
//! than a law joining it, so nothing refused when the file that scoped the run
//! disappeared. DELETING ANY ONE OF THE FOUR MOVED NO NUMBER AND FAILED NO
//! STAGE.
//!
//! Both files said so about themselves, in their own prose, and writing a gap
//! down does not close it — prose keeps reading as true after the thing it
//! describes is gone. They also named the exact shape that would close it: a
//! repository law of the shape
//! `crate::checks::supply_chain::check_dependency_gate_artifacts`, reading these
//! two configurations and the two workflows that call them, refusing when one is
//! deleted, when one is emptied, or when the `reversal` profile stops departing
//! from `default`. This is that law, in that shape, and it claims what that shape
//! claims and no more.
//!
//! # What is claimed here
//!
//! Six facts about FILES: the four artifacts are committed, the two
//! configurations state something, and the harness configuration's planted
//! `reversal` profile is visibly distinct from the `default` profile it stands
//! against. The alarms' artifacts are PRESENT and VISIBLY DISTINCT — which is
//! what the roster entry is called, in those words.
//!
//! # The reading is the decoder's, not a scan's
//!
//! A configuration that states nothing is read as a DECODED DOCUMENT declaring
//! no key, and a profile that stopped departing is read as two decoded tables
//! that are equal. Both questions are questions about TOML, so the decoder that
//! owns TOML answers them: a file emptied to whitespace, a file emptied to
//! comments, and a reversal resynchronized with the default through a different
//! spelling of the same values are one answer here rather than three cases a
//! text scan would have to be taught one at a time.
//!
//! # The ceiling, named here because the name would otherwise imply past it
//!
//! **It does not claim any hosted step still RUNS them.** Whether
//! `.github/workflows/harness.yml` still invokes the reversal profile, and
//! whether `.github/workflows/mutation.yml` still counts what it examined, is
//! unestablished here — not false, not true, unread — and this law FAILS OPEN on
//! it: a workflow file somebody emptied of every step still satisfies every
//! reading below. That ceiling is executed as a planted test rather than only
//! written in this paragraph, because a sentence is what this whole law exists to
//! stop trusting.
//!
//! The opening condition is the same one the dependency gate names and does not
//! reach: a hosted run publishing the roster of what it EXECUTED. A step that ran
//! appears in such a roster and a step somebody deleted does not, which answers
//! the question by reading an output rather than by guessing at a source. Until
//! then a deleted STEP is caught by review and by nothing else, and a deleted
//! FILE is caught here.
//!
//! **It does not claim the departure is the PLANTED one.** That the `reversal`
//! profile differs from `default` is established here; that it differs by the
//! one-second kill threshold against a seventeen-second test it was written for
//! is established by the harness job's own requirement of exit 100 AND a timed-out
//! test, and that requirement stands exactly as long as that step does.
//!
//! **It does not claim either tool is installed anywhere.** Both are separately
//! installed binaries, which is why they stand beside the bar rather than in it,
//! and nothing here starts either one.

use crate::repository::cargo::table_at;
use crate::repository::snapshot::RepositorySnapshot;
use crate::repository::types::Read;

/// The second harness's configuration, in path segments so the join spells no
/// separator of its own.
const HARNESS_CONFIGURATION: [&str; 2] = [".config", "nextest.toml"];

/// The hosted seat the second harness runs in.
const HARNESS_WORKFLOW: [&str; 3] = [".github", "workflows", "harness.yml"];

/// The mutation alarm's configuration, which scopes what a run examines.
const MUTATION_CONFIGURATION: [&str; 2] = [".cargo", "mutants.toml"];

/// The hosted seat the mutation alarm runs in.
const MUTATION_WORKFLOW: [&str; 3] = [".github", "workflows", "mutation.yml"];

/// The table the harness configuration's profiles hang beneath.
const PROFILE_TABLE: &str = "profile";

/// The profile a lawful run reads.
const LAWFUL_PROFILE: &str = "default";

/// The profile that is deliberately wrong, kept on purpose.
const REVERSAL_PROFILE: &str = "reversal";

/// The two standing alarms' committed artifacts are present and visibly
/// distinct.
///
/// # Errors
///
/// Returns the offences together rather than one per run, because they are
/// independent of one another and a repairer who deleted two parts should learn
/// about both at once. A configuration that is present and cannot be decoded is
/// a failure of the run itself rather than an offence: whether it states
/// anything is then unknown rather than false.
pub(crate) fn check_alarm_artifacts(snapshot: &RepositorySnapshot) -> Result<(), String> {
    let mut offences = Vec::new();
    committed(
        snapshot,
        &spelled(&HARNESS_WORKFLOW),
        &mut offences,
        "the second harness's hosted seat is not committed: no file in this repository names a run \
         that would read `.config/nextest.toml`, so the configuration below it is a decision \
         nobody executes",
    );
    committed(
        snapshot,
        &spelled(&MUTATION_WORKFLOW),
        &mut offences,
        "the mutation alarm's hosted seat is not committed: no file in this repository names a run \
         that would read `.cargo/mutants.toml`, so the scope it declares selects sources for \
         nobody",
    );
    committed(
        snapshot,
        &spelled(&HARNESS_CONFIGURATION),
        &mut offences,
        "the second harness's configuration is not committed: nothing states which nextest a run \
         requires, which profile it reads, or that a planted reversal exists at all, so the second \
         reading of the test population has stopped being a second reading of anything",
    );
    committed(
        snapshot,
        &spelled(&MUTATION_CONFIGURATION),
        &mut offences,
        "the mutation alarm's configuration is not committed: nothing scopes what a run examines, \
         and a run with no scope takes the root package alone, finds no mutant, and exits zero — \
         which a job reads as success",
    );
    if let Some(harness) = decoded(snapshot, &spelled(&HARNESS_CONFIGURATION))? {
        judge_harness(harness, &mut offences);
    }
    if let Some(mutation) = decoded(snapshot, &spelled(&MUTATION_CONFIGURATION))?
        && mutation.is_empty()
    {
        offences.push(String::from(
            "`.cargo/mutants.toml` states no key at all: a configuration that scopes nothing is a \
             run that examines the root package alone and reports zero mutants as success",
        ));
    }
    if offences.is_empty() {
        Ok(())
    } else {
        Err(offences.join("; "))
    }
}

/// Judges the harness configuration: it states something, and its planted
/// reversal profile departs from the profile it stands against.
fn judge_harness(document: &toml::Table, offences: &mut Vec<String>) {
    if document.is_empty() {
        offences.push(String::from(
            "`.config/nextest.toml` states no key at all: a configuration carrying no profile \
             cannot be departed from, so the planted reversal has stopped being a reversal",
        ));
        return;
    }
    let lawful = table_at(document, &[PROFILE_TABLE, LAWFUL_PROFILE]);
    let planted = table_at(document, &[PROFILE_TABLE, REVERSAL_PROFILE]);
    let (Some(lawful), Some(planted)) = (lawful.known(), planted.known()) else {
        offences.push(format!(
            "`.config/nextest.toml` does not state both `[{PROFILE_TABLE}.{LAWFUL_PROFILE}]` and \
             `[{PROFILE_TABLE}.{REVERSAL_PROFILE}]`: the planted reversal is the one committed \
             artifact by which it can be established that this file is READ at all, and a profile \
             that is not there is refused by nextest rather than watched by anybody"
        ));
        return;
    };
    if planted.is_empty() {
        offences.push(format!(
            "`[{PROFILE_TABLE}.{REVERSAL_PROFILE}]` states nothing: a profile that overrides no \
             setting inherits every one of them, so a run under it is the lawful run wearing \
             another name and it passes exactly where the reversal was supposed to refuse"
        ));
    }
    if **lawful == **planted {
        offences.push(format!(
            "`[{PROFILE_TABLE}.{REVERSAL_PROFILE}]` states what \
             `[{PROFILE_TABLE}.{LAWFUL_PROFILE}]` states: the planted reversal has been \
             synchronized with the profile it stands against, so it is no longer distinct from it \
             and carries nothing wrong left to refuse"
        ));
    }
}

/// One artifact's presence, with the words its absence is reported in.
fn committed(
    snapshot: &RepositorySnapshot,
    relative: &str,
    offences: &mut Vec<String>,
    said: &str,
) {
    if snapshot.files().get(relative).is_none() {
        offences.push(format!("`{relative}`: {said}"));
    }
}

/// One configuration as the decoder that owns TOML resolved it, or nothing
/// where the tree carries no such file.
///
/// An absence is not reported here — the presence reading above already reported
/// it, and one disappearance stated twice reads as two. A file that IS there and
/// does not decode refuses the whole law, because whether it states anything is
/// then unknown rather than false.
fn decoded<'snapshot>(
    snapshot: &'snapshot RepositorySnapshot,
    relative: &str,
) -> Result<Option<&'snapshot toml::Table>, String> {
    match snapshot.cargo().document(relative) {
        Read::Known(document) => Ok(Some(document)),
        Read::DeclaredAbsent(_) => Ok(None),
        Read::Unreadable(failure) => Err(format!("{relative} could not be decoded: {failure}")),
    }
}

/// One artifact as this repository spells paths — relative, forward slashes —
/// so a run on any machine names the same file.
fn spelled(segments: &[&str]) -> String {
    segments.join("/")
}

/// Planted reversals for a law whose subject is the tree.
///
/// The alarms are a set of files, so their reversals are planted against a
/// scratch root outside the repository: the law that counts the artifacts is
/// never proven by deleting them. The last two tests read the ceiling and the
/// real tree, and each states what it found.
#[cfg(test)]
mod tests {
    use super::{
        HARNESS_CONFIGURATION, HARNESS_WORKFLOW, MUTATION_CONFIGURATION, MUTATION_WORKFLOW,
        check_alarm_artifacts, spelled,
    };
    use crate::checks::scratch::Scratch;
    use crate::repository::snapshot::repository_snapshot;

    /// A fixture harness configuration: a lawful profile and a planted reversal
    /// that departs from it.
    const HARNESS_FIXTURE: &str = "nextest-version = \"0.9.132\"\n\n\
                                   [profile.default]\n\
                                   fail-fast = false\n\
                                   retries = 0\n\n\
                                   [profile.reversal]\n\
                                   default-filter = 'binary(compile_refusals)'\n";

    /// A fixture mutation configuration, standing for the scope a run examines.
    const MUTATION_FIXTURE: &str = "examine_globs = [\"xtask/**/*.rs\"]\ncap_lints = true\n";

    /// A fixture workflow, standing for either alarm's hosted seat.
    const WORKFLOW_FIXTURE: &str = "name: harness\n";

    /// One scratch root carrying both alarms whole.
    fn planted(name: &str) -> Scratch {
        let scratch = Scratch::named(name);
        scratch.write(&spelled(&HARNESS_WORKFLOW), WORKFLOW_FIXTURE);
        scratch.write(&spelled(&MUTATION_WORKFLOW), WORKFLOW_FIXTURE);
        scratch.write(&spelled(&HARNESS_CONFIGURATION), HARNESS_FIXTURE);
        scratch.write(&spelled(&MUTATION_CONFIGURATION), MUTATION_FIXTURE);
        scratch
    }

    /// The positive control: both alarms carrying all four artifacts, with a
    /// reversal profile that states something and departs from the profile it
    /// stands against, is lawful. A law that refused everything would satisfy
    /// every reversal below and be worthless.
    #[test]
    fn present_and_distinct_artifacts_are_lawful() -> Result<(), String> {
        let scratch = planted("alarms-whole");
        let found = check_alarm_artifacts(&scratch.read()?);
        assert!(found.is_ok(), "{found:?}");
        Ok(())
    }

    /// Planted reversal: the second harness's configuration deleted. This is one
    /// half of the failure the law exists for — before it, deleting this file
    /// moved no number and failed no stage.
    #[test]
    fn a_deleted_harness_configuration_is_a_violation() -> Result<(), String> {
        let scratch = planted("alarms-harness-configuration-deleted");
        scratch.remove(&spelled(&HARNESS_CONFIGURATION));
        let found = check_alarm_artifacts(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains(".config/nextest.toml")
                && reason.contains("is not committed")),
            "a deleted harness configuration passed the law that counts it"
        );
        Ok(())
    }

    /// Planted reversal: the mutation alarm's configuration deleted, which is
    /// the other half. A run with no scope examines the root package alone,
    /// finds nothing, and exits zero.
    #[test]
    fn a_deleted_mutation_configuration_is_a_violation() -> Result<(), String> {
        let scratch = planted("alarms-mutation-configuration-deleted");
        scratch.remove(&spelled(&MUTATION_CONFIGURATION));
        let found = check_alarm_artifacts(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains(".cargo/mutants.toml")
                && reason.contains("is not committed")),
            "a deleted mutation configuration passed"
        );
        Ok(())
    }

    /// Planted reversal: the harness workflow deleted, leaving a configuration
    /// no committed file names a run for.
    #[test]
    fn a_deleted_harness_workflow_is_a_violation() -> Result<(), String> {
        let scratch = planted("alarms-harness-workflow-deleted");
        scratch.remove(&spelled(&HARNESS_WORKFLOW));
        let found = check_alarm_artifacts(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("harness.yml")
                && reason.contains("hosted seat is not committed")),
            "a deleted harness workflow passed"
        );
        Ok(())
    }

    /// Planted reversal: the mutation workflow deleted.
    #[test]
    fn a_deleted_mutation_workflow_is_a_violation() -> Result<(), String> {
        let scratch = planted("alarms-mutation-workflow-deleted");
        scratch.remove(&spelled(&MUTATION_WORKFLOW));
        let found = check_alarm_artifacts(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("mutation.yml")
                && reason.contains("hosted seat is not committed")),
            "a deleted mutation workflow passed"
        );
        Ok(())
    }

    /// Planted reversal: a configuration emptied rather than deleted — the same
    /// disappearance written one edit shallower, and the one a listing of the
    /// tree reports as present. Emptied to COMMENTS rather than to whitespace,
    /// because that is the spelling a text scan would read as content and the
    /// decoder reads as the empty document it is.
    #[test]
    fn an_emptied_mutation_configuration_is_a_violation() -> Result<(), String> {
        let scratch = planted("alarms-mutation-configuration-emptied");
        scratch.write(
            &spelled(&MUTATION_CONFIGURATION),
            "# every decision that was here is gone\n",
        );
        let found = check_alarm_artifacts(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("states no key at all")),
            "an emptied mutation configuration passed"
        );
        Ok(())
    }

    /// Planted reversal: the reversal profile deleted out of a configuration
    /// that is otherwise whole. Every artifact is present and the one committed
    /// thing by which this file can be shown to be READ is gone.
    #[test]
    fn a_harness_configuration_with_no_reversal_profile_is_a_violation() -> Result<(), String> {
        let scratch = planted("alarms-reversal-profile-deleted");
        scratch.write(
            &spelled(&HARNESS_CONFIGURATION),
            "[profile.default]\nfail-fast = false\n",
        );
        let found = check_alarm_artifacts(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("does not state both")),
            "a configuration carrying no reversal profile passed"
        );
        Ok(())
    }

    /// Planted reversal: the reversal profile synchronized with the profile it
    /// stands against until it is no longer wrong.
    ///
    /// The most expensive spelling of the defect, because every artifact is
    /// present and a listing of the tree says both alarms are whole. A reversal
    /// that agrees with the profile it departs from is not distinct from it, and
    /// the one thing the profile was committed to be — a configuration that is
    /// wrong on purpose — has quietly stopped being true of it.
    #[test]
    fn a_reversal_profile_that_stopped_departing_is_a_violation() -> Result<(), String> {
        let scratch = planted("alarms-reversal-profile-synchronized");
        scratch.write(
            &spelled(&HARNESS_CONFIGURATION),
            "[profile.default]\nfail-fast = false\n\n[profile.reversal]\nfail-fast = false\n",
        );
        let found = check_alarm_artifacts(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("has been synchronized")),
            "a reversal profile that is a copy of the default passed"
        );
        Ok(())
    }

    /// Planted reversal: an emptied reversal profile, which is the same
    /// disappearance one edit shallower again. A profile that overrides nothing
    /// inherits everything, so a run under it is the lawful run under another
    /// name.
    #[test]
    fn an_emptied_reversal_profile_is_a_violation() -> Result<(), String> {
        let scratch = planted("alarms-reversal-profile-emptied");
        scratch.write(
            &spelled(&HARNESS_CONFIGURATION),
            "[profile.default]\nfail-fast = false\n\n[profile.reversal]\n",
        );
        let found = check_alarm_artifacts(&scratch.read()?);
        assert!(
            found.is_err_and(|reason| reason.contains("states nothing")),
            "an emptied reversal profile passed"
        );
        Ok(())
    }

    /// The ceiling, executed rather than only written down: workflows that
    /// invoke nothing at all still pass this law.
    ///
    /// It proves nothing about either alarm, and it is not a positive control.
    /// What it holds in place is the NAME. The fixture workflows here carry no
    /// job and no step, exactly as workflows somebody edited every step out of
    /// carry none, and this law is content with both — which is the fail open the
    /// module documentation states. A later reader who widens this law to read
    /// what a hosted run executed has to delete this test to do it, and that
    /// deletion is the line of the diff where the claim is allowed to grow back.
    #[test]
    fn workflows_that_invoke_nothing_still_pass() -> Result<(), String> {
        let scratch = planted("alarms-workflows-invoke-nothing");
        let empty = "name: harness\non:\n  schedule:\n    - cron: '0 0 * * 0'\njobs: {}\n";
        scratch.write(&spelled(&HARNESS_WORKFLOW), empty);
        scratch.write(&spelled(&MUTATION_WORKFLOW), empty);
        let found = check_alarm_artifacts(&scratch.read()?);
        assert!(
            found.is_ok(),
            "this law reads files and not steps; a verdict here would mean it had started reading \
             steps without its name saying so: {found:?}"
        );
        Ok(())
    }

    /// The real repository holds: both alarms' four artifacts are committed, the
    /// two configurations state something, and the planted reversal profile
    /// departs from the profile it stands against.
    #[test]
    fn the_real_alarm_artifacts_are_present_and_distinct() -> Result<(), String> {
        let found = check_alarm_artifacts(repository_snapshot()?);
        assert!(found.is_ok(), "{found:?}");
        Ok(())
    }
}
