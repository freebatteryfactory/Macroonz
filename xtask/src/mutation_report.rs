//! The typed boundary over cargo-mutants' hosted report.
//!
//! cargo-mutants owns mutation execution, outcome classification, and the v27
//! report schema. This adapter owns the narrower `ThreadPak` admission question:
//! did a finalized report contain every required artifact, did its structured
//! totals agree with its record rosters, and did its exit posture agree with the
//! producer's own outcome? A normal run derives its executed population from
//! `outcomes.json`. The planted empty scope, for which v27 emits no outcome
//! document, derives silence from the empty mutant array and empty rosters. The
//! adapter never reconstructs mutation meaning from filenames.
//!
//! The adapter deliberately does not claim that a caught mutant proves the
//! right behavior, that a surviving mutant is acceptable, or that a timeout is
//! a failure. Survivors remain evidence debt. A timeout preserves and prints
//! the report, then returns red because the outcome is unknown.

use std::error::Error;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{ErrorKind, Read, Write as _};
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// Maximum bytes admitted from either structured report artifact.
const STRUCTURED_REPORT_BYTE_LIMIT: u64 = 67_108_864;

/// Maximum bytes admitted from one line-oriented outcome roster.
const ROSTER_BYTE_LIMIT: u64 = 16_777_216;

/// The cargo-mutants report schema this adapter admits.
const SUPPORTED_CARGO_MUTANTS_VERSION: &str = "27.0.0";

/// The producer report fields `ThreadPak` consumes from cargo-mutants v27.
#[derive(Deserialize)]
struct LabOutcome {
    outcomes: Vec<ScenarioOutcome>,
    total_mutants: usize,
    missed: usize,
    caught: usize,
    timeout: usize,
    unviable: usize,
    success: usize,
    end_time: Option<String>,
    cargo_mutants_version: String,
}

/// The producer identity carried by every cargo-mutants v27 output directory.
#[derive(Deserialize)]
struct LockRecord {
    cargo_mutants_version: String,
}

/// One scenario observation in cargo-mutants v27's finalized report.
#[derive(Deserialize)]
struct ScenarioOutcome {
    scenario: Scenario,
    summary: SummaryOutcome,
}

/// Whether an observation is the baseline or one named mutant.
#[derive(Deserialize)]
enum Scenario {
    Baseline,
    Mutant(MutantRecord),
}

/// The stable producer spelling shared by JSON observations and text rosters.
#[derive(Deserialize)]
struct MutantRecord {
    name: String,
}

/// cargo-mutants v27's closed scenario summary roster.
#[derive(Clone, Copy, Deserialize)]
enum SummaryOutcome {
    CaughtMutant,
    MissedMutant,
    Timeout,
    Unviable,
    Success,
    Failure,
}

/// Mutant identities derived from the per-scenario observations.
struct ClassifiedOutcomes {
    baseline: Vec<SummaryOutcome>,
    all: Vec<String>,
    caught: Vec<String>,
    missed: Vec<String>,
    timeout: Vec<String>,
    unviable: Vec<String>,
    success: Vec<String>,
    failure: Vec<String>,
}

/// Which hosted mutation claim is being admitted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReportMode {
    Run,
    Reversal,
}

impl ReportMode {
    fn parse(spelling: &str) -> Result<Self, String> {
        match spelling {
            "run" => Ok(Self::Run),
            "reversal" => Ok(Self::Reversal),
            other => Err(format!(
                "unknown mutation report mode {other:?}; expected run or reversal"
            )),
        }
    }

    const fn spelling(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Reversal => "reversal",
        }
    }
}

/// Whether a validated report may leave the hosted alarm green.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReportDisposition {
    Accepted,
    OutcomeUnknown,
}

/// The line-oriented evidence carried beside the structured counters.
struct Rosters {
    caught: Vec<String>,
    missed: Vec<String>,
    timeout: Vec<String>,
    unviable: Vec<String>,
}

/// A fully checked report and the text the hosted log publishes.
struct Inspection {
    receipt: String,
    disposition: ReportDisposition,
}

impl Inspection {
    fn finish(self) -> Result<(), String> {
        match self.disposition {
            ReportDisposition::Accepted => Ok(()),
            ReportDisposition::OutcomeUnknown => Err(String::from(
                "cargo-mutants timed out after writing a valid report; the timed-out mutant outcome is unknown",
            )),
        }
    }
}

/// Validate and publish one cargo-mutants report.
pub(crate) fn run(
    repository_root: &Path,
    mut arguments: impl Iterator<Item = String>,
) -> Result<(), Box<dyn Error>> {
    let mode_spelling = required_argument(&mut arguments, "mode")?;
    let exit_spelling = required_argument(&mut arguments, "exit code")?;
    let directory_spelling = required_argument(&mut arguments, "report directory")?;
    if let Some(extra) = arguments.next() {
        return Err(format!("unexpected mutation-report argument: {extra}").into());
    }

    let mode = ReportMode::parse(&mode_spelling)?;
    let exit_code = exit_spelling
        .parse::<i32>()
        .map_err(|source| format!("invalid cargo-mutants exit code {exit_spelling:?}: {source}"))?;
    let supplied_directory = PathBuf::from(directory_spelling);
    let report_directory = if supplied_directory.is_absolute() {
        supplied_directory
    } else {
        repository_root.join(supplied_directory)
    };
    let inspection = inspect(mode, exit_code, &report_directory)?;
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(inspection.receipt.as_bytes())?;
    stdout.flush()?;
    drop(stdout);
    inspection.finish().map_err(Into::into)
}

fn required_argument(
    arguments: &mut impl Iterator<Item = String>,
    role: &str,
) -> Result<String, String> {
    arguments
        .next()
        .ok_or_else(|| format!("mutation-report requires a {role}"))
}

fn inspect(
    mode: ReportMode,
    exit_code: i32,
    report_directory: &Path,
) -> Result<Inspection, String> {
    let producer_version = read_producer_version(report_directory)?;
    let mutant_inventory = read_mutant_inventory(report_directory)?;
    let rosters = read_rosters(report_directory)?;
    let (outcome, disposition) = match mode {
        ReportMode::Run => {
            let run_outcome = read_outcome(&report_directory.join("outcomes.json"))?
                .ok_or_else(|| String::from("the mutation run wrote no outcomes.json"))?;
            validate_final_outcome(&run_outcome, &producer_version)?;
            validate_counts(&run_outcome, &mutant_inventory, &rosters)?;
            let run_disposition = validate_run(exit_code, &run_outcome)?;
            (Some(run_outcome), run_disposition)
        }
        ReportMode::Reversal => {
            let reversal_outcome = read_outcome(&report_directory.join("outcomes.json"))?;
            if reversal_outcome.is_some() {
                return Err(String::from(
                    "cargo-mutants v27 unexpectedly wrote outcomes.json for the planted empty scope",
                ));
            }
            validate_reversal(exit_code, mutant_inventory.len(), &rosters)?;
            (None, ReportDisposition::Accepted)
        }
    };
    let receipt = render_receipt(
        mode,
        &producer_version,
        outcome.as_ref(),
        mutant_inventory.len(),
        &rosters,
    )?;
    Ok(Inspection {
        receipt,
        disposition,
    })
}

fn read_producer_version(report_directory: &Path) -> Result<String, String> {
    let lock_path = report_directory.join("lock.json");
    let lock_text = read_utf8_bounded(&lock_path, STRUCTURED_REPORT_BYTE_LIMIT)?;
    let lock: LockRecord = serde_json::from_str(&lock_text).map_err(|source| {
        format!(
            "cannot decode {} as cargo-mutants v27 lock.json: {source}",
            lock_path.display()
        )
    })?;
    if lock.cargo_mutants_version.trim().is_empty() {
        return Err(String::from("lock.json carries no cargo-mutants version"));
    }
    if lock.cargo_mutants_version != SUPPORTED_CARGO_MUTANTS_VERSION {
        return Err(format!(
            "unsupported cargo-mutants report version {}; expected {SUPPORTED_CARGO_MUTANTS_VERSION}",
            lock.cargo_mutants_version
        ));
    }
    Ok(lock.cargo_mutants_version)
}

fn read_mutant_inventory(report_directory: &Path) -> Result<Vec<String>, String> {
    let mutants_path = report_directory.join("mutants.json");
    let mutants_text = read_utf8_bounded(&mutants_path, STRUCTURED_REPORT_BYTE_LIMIT)?;
    let mutants: Vec<MutantRecord> = serde_json::from_str(&mutants_text).map_err(|source| {
        format!(
            "cannot decode {} as the cargo-mutants v27 mutant array: {source}",
            mutants_path.display()
        )
    })?;
    Ok(mutants.into_iter().map(|mutant| mutant.name).collect())
}

fn read_outcome(path: &Path) -> Result<Option<LabOutcome>, String> {
    let outcome_text = match read_utf8_bounded(path, STRUCTURED_REPORT_BYTE_LIMIT) {
        Ok(text) => text,
        Err(read_reason) => {
            if path_absence(path)? {
                return Ok(None);
            }
            return Err(read_reason);
        }
    };
    let outcome = serde_json::from_str(&outcome_text).map_err(|source| {
        format!(
            "cannot decode {} as cargo-mutants v27 outcomes.json: {source}",
            path.display()
        )
    })?;
    Ok(Some(outcome))
}

fn path_absence(path: &Path) -> Result<bool, String> {
    match std::fs::metadata(path) {
        Ok(_) => Ok(false),
        Err(source) if source.kind() == ErrorKind::NotFound => Ok(true),
        Err(source) => Err(format!("cannot inspect {}: {source}", path.display())),
    }
}

fn validate_final_outcome(outcome: &LabOutcome, producer_version: &str) -> Result<(), String> {
    let Some(end_time) = outcome.end_time.as_deref() else {
        return Err(String::from(
            "outcomes.json is not finalized: end_time is absent or null",
        ));
    };
    if end_time.trim().is_empty() {
        return Err(String::from(
            "outcomes.json is not finalized: end_time is empty",
        ));
    }
    if outcome.cargo_mutants_version.trim().is_empty() {
        return Err(String::from(
            "outcomes.json carries no cargo-mutants version",
        ));
    }
    if outcome.cargo_mutants_version != producer_version {
        return Err(format!(
            "cargo-mutants version differs between lock.json ({producer_version}) and outcomes.json ({})",
            outcome.cargo_mutants_version
        ));
    }
    Ok(())
}

fn read_rosters(report_directory: &Path) -> Result<Rosters, String> {
    Ok(Rosters {
        caught: read_roster(&report_directory.join("caught.txt"))?,
        missed: read_roster(&report_directory.join("missed.txt"))?,
        timeout: read_roster(&report_directory.join("timeout.txt"))?,
        unviable: read_roster(&report_directory.join("unviable.txt"))?,
    })
}

fn read_roster(path: &Path) -> Result<Vec<String>, String> {
    let text = read_utf8_bounded(path, ROSTER_BYTE_LIMIT)?;
    let mut records = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            let record_number = index.checked_add(1).ok_or_else(|| {
                format!("record number overflow while reading {}", path.display())
            })?;
            return Err(format!(
                "{} contains a blank record at position {record_number}",
                path.display()
            ));
        }
        records.push(line.to_owned());
    }
    Ok(records)
}

fn read_utf8_bounded(path: &Path, byte_limit: u64) -> Result<String, String> {
    let read_limit = byte_limit
        .checked_add(1)
        .ok_or_else(|| format!("byte limit overflow while opening {}", path.display()))?;
    let file =
        File::open(path).map_err(|source| format!("cannot open {}: {source}", path.display()))?;
    let mut bytes = Vec::new();
    file.take(read_limit)
        .read_to_end(&mut bytes)
        .map_err(|source| format!("cannot read {}: {source}", path.display()))?;
    let byte_count = u64::try_from(bytes.len())
        .map_err(|source| format!("cannot represent the size of {}: {source}", path.display()))?;
    if byte_count > byte_limit {
        return Err(format!(
            "{} exceeds its admitted {byte_limit}-byte report limit",
            path.display()
        ));
    }
    String::from_utf8(bytes)
        .map_err(|source| format!("{} is not strict UTF-8: {source}", path.display()))
}

fn validate_counts(
    outcome: &LabOutcome,
    mutant_inventory: &[String],
    rosters: &Rosters,
) -> Result<(), String> {
    let classified = classify_outcomes(&outcome.outcomes);
    validate_baseline(&classified.baseline)?;
    compare_count(
        "total mutant population",
        "outcomes.json total_mutants",
        outcome.total_mutants,
        "mutants.json records",
        mutant_inventory.len(),
    )?;
    compare_count(
        "observed mutant population",
        "outcomes.json total_mutants",
        outcome.total_mutants,
        "per-scenario mutant observations",
        classified.all.len(),
    )?;
    compare_count(
        "caught",
        "aggregate",
        outcome.caught,
        "observed",
        classified.caught.len(),
    )?;
    compare_count(
        "missed",
        "aggregate",
        outcome.missed,
        "observed",
        classified.missed.len(),
    )?;
    compare_count(
        "timeout",
        "aggregate",
        outcome.timeout,
        "observed",
        classified.timeout.len(),
    )?;
    compare_count(
        "unviable",
        "aggregate",
        outcome.unviable,
        "observed",
        classified.unviable.len(),
    )?;
    compare_count(
        "success",
        "aggregate",
        outcome.success,
        "observed",
        classified.success.len(),
    )?;
    compare_records("mutant inventory", mutant_inventory, &classified.all)?;
    compare_records("caught roster", &rosters.caught, &classified.caught)?;
    compare_records("missed roster", &rosters.missed, &classified.missed)?;
    compare_records("timeout roster", &rosters.timeout, &classified.timeout)?;
    compare_records("unviable roster", &rosters.unviable, &classified.unviable)?;
    if !classified.failure.is_empty() {
        return Err(format!(
            "cargo-mutants emitted {} unclassified mutant failure observation(s)",
            classified.failure.len()
        ));
    }
    if !classified.success.is_empty() {
        return Err(format!(
            "cargo-mutants reported {} successful mutant outcome(s), which this run contract does not classify",
            classified.success.len()
        ));
    }
    Ok(())
}

fn classify_outcomes(outcomes: &[ScenarioOutcome]) -> ClassifiedOutcomes {
    let mut classified = ClassifiedOutcomes {
        baseline: Vec::new(),
        all: Vec::new(),
        caught: Vec::new(),
        missed: Vec::new(),
        timeout: Vec::new(),
        unviable: Vec::new(),
        success: Vec::new(),
        failure: Vec::new(),
    };
    for observation in outcomes {
        let mutant = match &observation.scenario {
            Scenario::Baseline => {
                classified.baseline.push(observation.summary);
                continue;
            }
            Scenario::Mutant(mutant) => mutant,
        };
        classified.all.push(mutant.name.clone());
        let destination = match observation.summary {
            SummaryOutcome::CaughtMutant => &mut classified.caught,
            SummaryOutcome::MissedMutant => &mut classified.missed,
            SummaryOutcome::Timeout => &mut classified.timeout,
            SummaryOutcome::Unviable => &mut classified.unviable,
            SummaryOutcome::Success => &mut classified.success,
            SummaryOutcome::Failure => &mut classified.failure,
        };
        destination.push(mutant.name.clone());
    }
    classified
}

fn validate_baseline(baselines: &[SummaryOutcome]) -> Result<(), String> {
    match baselines {
        [SummaryOutcome::Success] => Ok(()),
        [] => Err(String::from(
            "cargo-mutants wrote no baseline observation; the unmodified suite is unproven",
        )),
        [_] => Err(String::from(
            "cargo-mutants baseline observation was not successful",
        )),
        _ => Err(format!(
            "cargo-mutants wrote {} baseline observations; exactly one is required",
            baselines.len()
        )),
    }
}

fn compare_count(
    role: &str,
    first_seat: &str,
    first: usize,
    second_seat: &str,
    second: usize,
) -> Result<(), String> {
    if first == second {
        Ok(())
    } else {
        Err(format!(
            "cargo-mutants {role} is {first} in {first_seat} but {second} in {second_seat}"
        ))
    }
}

fn compare_records(role: &str, first: &[String], second: &[String]) -> Result<(), String> {
    let mut first_sorted = first.to_vec();
    let mut second_sorted = second.to_vec();
    first_sorted.sort_unstable();
    second_sorted.sort_unstable();
    if first_sorted == second_sorted {
        Ok(())
    } else {
        Err(format!(
            "cargo-mutants {role} identities disagree across its structured and record surfaces"
        ))
    }
}

fn validate_run(exit_code: i32, outcome: &LabOutcome) -> Result<ReportDisposition, String> {
    if outcome.total_mutants == 0 {
        return Err(String::from(
            "the mutation run examined no mutants; its configured scope is silent",
        ));
    }
    if outcome.caught == 0 {
        return Err(String::from(
            "the mutation run caught no mutant; its challenge route has not activated",
        ));
    }

    match (exit_code, outcome.missed, outcome.timeout) {
        (0, 0, 0) => Ok(ReportDisposition::Accepted),
        (2, missed, 0) if missed > 0 => Ok(ReportDisposition::Accepted),
        (3, _, timeout) if timeout > 0 => Ok(ReportDisposition::OutcomeUnknown),
        _ => Err(format!(
            "cargo-mutants exit {exit_code} disagrees with missed={} and timeout={} in outcomes.json",
            outcome.missed, outcome.timeout
        )),
    }
}

fn validate_reversal(exit_code: i32, mutant_count: usize, rosters: &Rosters) -> Result<(), String> {
    let roster_count = [
        rosters.caught.len(),
        rosters.missed.len(),
        rosters.timeout.len(),
        rosters.unviable.len(),
    ]
    .into_iter()
    .try_fold(0_usize, |subtotal, count| {
        subtotal
            .checked_add(count)
            .ok_or_else(|| String::from("reversal roster counters overflowed"))
    })?;
    if exit_code != 0_i32 || mutant_count != 0 || roster_count != 0 {
        return Err(format!(
            "the planted empty-scope reversal exited {exit_code} with listed={mutant_count} and rostered={roster_count}; it no longer establishes silence"
        ));
    }
    Ok(())
}

fn render_receipt(
    mode: ReportMode,
    producer_version: &str,
    outcome: Option<&LabOutcome>,
    mutant_count: usize,
    rosters: &Rosters,
) -> Result<String, String> {
    let total = outcome.map_or(mutant_count, |present| present.total_mutants);
    let caught = outcome.map_or(rosters.caught.len(), |present| present.caught);
    let missed = outcome.map_or(rosters.missed.len(), |present| present.missed);
    let timeout = outcome.map_or(rosters.timeout.len(), |present| present.timeout);
    let unviable = outcome.map_or(rosters.unviable.len(), |present| present.unviable);
    let success = outcome.map_or(0_usize, |present| present.success);
    let mut receipt = format!(
        "mutation.mode={}\nmutation.cargo_mutants_version={}\nmutation.total={}\nmutation.caught={}\nmutation.missed={}\nmutation.timeout={}\nmutation.unviable={}\nmutation.success={}\n",
        mode.spelling(),
        producer_version,
        total,
        caught,
        missed,
        timeout,
        unviable,
        success
    );
    append_roster(&mut receipt, "missed.evidence_debt", &rosters.missed)?;
    append_roster(&mut receipt, "timeout.outcome_unknown", &rosters.timeout)?;
    Ok(receipt)
}

fn append_roster(receipt: &mut String, role: &str, records: &[String]) -> Result<(), String> {
    writeln!(receipt, "mutation.{role}.begin")
        .map_err(|source| format!("cannot render mutation receipt: {source}"))?;
    for record in records {
        writeln!(receipt, "{record}")
            .map_err(|source| format!("cannot render mutation receipt: {source}"))?;
    }
    writeln!(receipt, "mutation.{role}.end")
        .map_err(|source| format!("cannot render mutation receipt: {source}"))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use serde_json::json;

    use super::{Inspection, ReportDisposition, ReportMode, inspect, read_utf8_bounded};

    static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct Fixture {
        root: PathBuf,
    }

    impl Fixture {
        fn new(label: &str) -> Result<Self, String> {
            let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "threadpak-mutation-report-{}-{sequence}-{label}",
                std::process::id()
            ));
            fs::create_dir_all(&root)
                .map_err(|source| format!("cannot create {}: {source}", root.display()))?;
            Ok(Self { root })
        }

        fn write(&self, relative: &str, bytes: impl AsRef<[u8]>) -> Result<(), String> {
            let path = self.root.join(relative);
            fs::write(&path, bytes)
                .map_err(|source| format!("cannot write {}: {source}", path.display()))
        }

        fn write_report(&self, report: &FixtureReport) -> Result<(), String> {
            let outcome = json!({
                "outcomes": report.scenario_outcomes(),
                "total_mutants": report.total,
                "missed": report.missed.len(),
                "caught": report.caught.len(),
                "timeout": report.timeout.len(),
                "unviable": report.unviable.len(),
                "success": report.success.len(),
                "start_time": "2026-08-15T00:00:00Z",
                "end_time": report.end_time,
                "cargo_mutants_version": "27.0.0"
            });
            self.write(
                "outcomes.json",
                serde_json::to_vec(&outcome)
                    .map_err(|source| format!("cannot encode fixture outcome: {source}"))?,
            )?;
            self.write(
                "lock.json",
                br#"{"cargo_mutants_version":"27.0.0","start_time":"2026-08-15T00:00:00Z","hostname":"fixture","username":"fixture"}"#,
            )?;
            let mutants = report
                .mutant_names()
                .into_iter()
                .map(|name| json!({ "name": name }))
                .collect::<Vec<_>>();
            self.write(
                "mutants.json",
                serde_json::to_vec(&mutants)
                    .map_err(|source| format!("cannot encode fixture mutants: {source}"))?,
            )?;
            self.write("caught.txt", report.caught.join("\n"))?;
            self.write("missed.txt", report.missed.join("\n"))?;
            self.write("timeout.txt", report.timeout.join("\n"))?;
            self.write("unviable.txt", report.unviable.join("\n"))
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            drop(fs::remove_dir_all(&self.root));
        }
    }

    struct FixtureReport {
        total: usize,
        caught: Vec<&'static str>,
        missed: Vec<&'static str>,
        timeout: Vec<&'static str>,
        unviable: Vec<&'static str>,
        success: Vec<&'static str>,
        end_time: Option<&'static str>,
    }

    impl FixtureReport {
        fn mutant_names(&self) -> Vec<&str> {
            self.caught
                .iter()
                .chain(&self.missed)
                .chain(&self.timeout)
                .chain(&self.unviable)
                .chain(&self.success)
                .copied()
                .collect()
        }

        fn scenario_outcomes(&self) -> Vec<serde_json::Value> {
            let mut outcomes = vec![json!({
                "scenario": "Baseline",
                "summary": "Success"
            })];
            Self::append_outcomes(&mut outcomes, &self.caught, "CaughtMutant");
            Self::append_outcomes(&mut outcomes, &self.missed, "MissedMutant");
            Self::append_outcomes(&mut outcomes, &self.timeout, "Timeout");
            Self::append_outcomes(&mut outcomes, &self.unviable, "Unviable");
            Self::append_outcomes(&mut outcomes, &self.success, "Success");
            outcomes
        }

        fn append_outcomes(outcomes: &mut Vec<serde_json::Value>, names: &[&str], summary: &str) {
            outcomes.extend(names.iter().map(|name| {
                json!({
                    "scenario": { "Mutant": { "name": name } },
                    "summary": summary
                })
            }));
        }

        fn run() -> Self {
            Self {
                total: 3,
                caught: vec!["caught one", "caught two"],
                missed: vec!["survived one"],
                timeout: Vec::new(),
                unviable: Vec::new(),
                success: Vec::new(),
                end_time: Some("2026-08-15T00:01:00Z"),
            }
        }

        fn reversal() -> Self {
            Self {
                total: 0,
                caught: Vec::new(),
                missed: Vec::new(),
                timeout: Vec::new(),
                unviable: Vec::new(),
                success: Vec::new(),
                end_time: Some("2026-08-15T00:01:00Z"),
            }
        }
    }

    fn inspected(
        label: &str,
        mode: ReportMode,
        exit_code: i32,
        report: &FixtureReport,
    ) -> Result<(Fixture, Inspection), String> {
        let fixture = Fixture::new(label)?;
        fixture.write_report(report)?;
        let result = inspect(mode, exit_code, &fixture.root)?;
        Ok((fixture, result))
    }

    fn refusal(result: Result<Inspection, String>) -> Result<String, String> {
        match result {
            Ok(_) => Err(String::from("the planted malformed report was admitted")),
            Err(reason) => Ok(reason),
        }
    }

    #[test]
    fn a_valid_run_preserves_survivors_as_evidence_debt() -> Result<(), String> {
        let (_fixture, inspection) =
            inspected("valid-run", ReportMode::Run, 2, &FixtureReport::run())?;
        assert_eq!(inspection.disposition, ReportDisposition::Accepted);
        assert!(inspection.receipt.contains("mutation.total=3"));
        assert!(inspection.receipt.contains("survived one"));
        inspection.finish()
    }

    #[test]
    fn a_missing_baseline_refuses() -> Result<(), String> {
        let fixture = Fixture::new("missing-baseline")?;
        fixture.write_report(&FixtureReport::run())?;
        let outcome_text = fs::read_to_string(fixture.root.join("outcomes.json"))
            .map_err(|source| format!("cannot read fixture outcome: {source}"))?;
        let mut outcome: serde_json::Value = serde_json::from_str(&outcome_text)
            .map_err(|source| format!("cannot decode fixture outcome: {source}"))?;
        let Some(outcomes) = outcome
            .get_mut("outcomes")
            .and_then(serde_json::Value::as_array_mut)
        else {
            return Err(String::from("fixture outcome has no scenario array"));
        };
        outcomes.retain(|observation| observation.get("scenario") != Some(&json!("Baseline")));
        fixture.write(
            "outcomes.json",
            serde_json::to_vec(&outcome)
                .map_err(|source| format!("cannot encode fixture outcome: {source}"))?,
        )?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("no baseline"));
        Ok(())
    }

    #[test]
    fn a_failed_baseline_refuses() -> Result<(), String> {
        let fixture = Fixture::new("failed-baseline")?;
        fixture.write_report(&FixtureReport::run())?;
        let outcome_text = fs::read_to_string(fixture.root.join("outcomes.json"))
            .map_err(|source| format!("cannot read fixture outcome: {source}"))?;
        let mut outcome: serde_json::Value = serde_json::from_str(&outcome_text)
            .map_err(|source| format!("cannot decode fixture outcome: {source}"))?;
        let Some(baseline) = outcome
            .get_mut("outcomes")
            .and_then(serde_json::Value::as_array_mut)
            .and_then(|outcomes| {
                outcomes.iter_mut().find(|observation| {
                    observation
                        .get("scenario")
                        .and_then(serde_json::Value::as_str)
                        == Some("Baseline")
                })
            })
        else {
            return Err(String::from("fixture outcome has no baseline"));
        };
        let Some(summary) = baseline.get_mut("summary") else {
            return Err(String::from("fixture baseline has no summary"));
        };
        *summary = json!("Failure");
        fixture.write(
            "outcomes.json",
            serde_json::to_vec(&outcome)
                .map_err(|source| format!("cannot encode fixture outcome: {source}"))?,
        )?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("not successful"));
        Ok(())
    }

    #[test]
    fn duplicate_baselines_refuse() -> Result<(), String> {
        let fixture = Fixture::new("duplicate-baseline")?;
        fixture.write_report(&FixtureReport::run())?;
        let outcome_text = fs::read_to_string(fixture.root.join("outcomes.json"))
            .map_err(|source| format!("cannot read fixture outcome: {source}"))?;
        let mut outcome: serde_json::Value = serde_json::from_str(&outcome_text)
            .map_err(|source| format!("cannot decode fixture outcome: {source}"))?;
        let Some(outcomes) = outcome
            .get_mut("outcomes")
            .and_then(serde_json::Value::as_array_mut)
        else {
            return Err(String::from("fixture outcome has no scenario array"));
        };
        outcomes.push(json!({ "scenario": "Baseline", "summary": "Success" }));
        fixture.write(
            "outcomes.json",
            serde_json::to_vec(&outcome)
                .map_err(|source| format!("cannot encode fixture outcome: {source}"))?,
        )?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("exactly one"));
        Ok(())
    }

    #[test]
    fn the_empty_scope_reversal_is_distinct() -> Result<(), String> {
        let fixture = Fixture::new("empty-reversal")?;
        fixture.write_report(&FixtureReport::reversal())?;
        assert!(
            refusal(inspect(ReportMode::Reversal, 0, &fixture.root))?
                .contains("unexpectedly wrote outcomes.json")
        );
        fs::remove_file(fixture.root.join("outcomes.json"))
            .map_err(|source| format!("cannot remove fixture outcome: {source}"))?;
        let inspection = inspect(ReportMode::Reversal, 0, &fixture.root)?;
        assert!(inspection.receipt.contains("mutation.mode=reversal"));
        assert!(
            inspection
                .receipt
                .contains("mutation.cargo_mutants_version=27.0.0")
        );
        inspection.finish()
    }

    #[test]
    fn an_unterminated_final_record_is_counted() -> Result<(), String> {
        let (fixture, inspection) = inspected(
            "unterminated-final-record",
            ReportMode::Run,
            2,
            &FixtureReport::run(),
        )?;
        let caught = fs::read_to_string(fixture.root.join("caught.txt"))
            .map_err(|source| format!("cannot read fixture: {source}"))?;
        assert!(!caught.ends_with('\n'));
        assert!(inspection.receipt.contains("mutation.caught=2"));
        Ok(())
    }

    #[test]
    fn malformed_or_missing_structured_artifacts_refuse() -> Result<(), String> {
        let fixture = Fixture::new("malformed-json")?;
        fixture.write_report(&FixtureReport::run())?;
        fixture.write("outcomes.json", b"{")?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("cannot decode"));

        fixture.write_report(&FixtureReport::run())?;
        fixture.write("lock.json", b"{")?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("lock.json"));

        fixture.write_report(&FixtureReport::run())?;
        fixture.write("mutants.json", b"{}")?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("mutant array"));

        fixture.write_report(&FixtureReport::run())?;
        fs::remove_file(fixture.root.join("outcomes.json"))
            .map_err(|source| format!("cannot remove fixture artifact: {source}"))?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("wrote no outcomes"));
        Ok(())
    }

    #[test]
    fn unsupported_and_disagreeing_producer_versions_refuse() -> Result<(), String> {
        let fixture = Fixture::new("unsupported-version")?;
        fixture.write_report(&FixtureReport::run())?;
        fixture.write("lock.json", br#"{"cargo_mutants_version":"28.0.0"}"#)?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("unsupported"));

        fixture.write_report(&FixtureReport::run())?;
        let outcome_text = fs::read_to_string(fixture.root.join("outcomes.json"))
            .map_err(|source| format!("cannot read fixture outcome: {source}"))?;
        let mut outcome: serde_json::Value = serde_json::from_str(&outcome_text)
            .map_err(|source| format!("cannot decode fixture outcome: {source}"))?;
        let Some(producer_version) = outcome.get_mut("cargo_mutants_version") else {
            return Err(String::from("fixture outcome has no producer version"));
        };
        *producer_version = json!("28.0.0");
        fixture.write(
            "outcomes.json",
            serde_json::to_vec(&outcome)
                .map_err(|source| format!("cannot encode fixture outcome: {source}"))?,
        )?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("version differs"));
        Ok(())
    }

    #[test]
    fn a_missing_required_field_refuses() -> Result<(), String> {
        let fixture = Fixture::new("missing-field")?;
        fixture.write_report(&FixtureReport::run())?;
        let outcome_text = fs::read_to_string(fixture.root.join("outcomes.json"))
            .map_err(|source| format!("cannot read fixture outcome: {source}"))?;
        let mut outcome: serde_json::Value = serde_json::from_str(&outcome_text)
            .map_err(|source| format!("cannot decode fixture outcome: {source}"))?;
        let Some(object) = outcome.as_object_mut() else {
            return Err(String::from("fixture outcome is not an object"));
        };
        object.remove("outcomes");
        fixture.write(
            "outcomes.json",
            serde_json::to_vec(&outcome)
                .map_err(|source| format!("cannot encode fixture outcome: {source}"))?,
        )?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("missing field"));
        Ok(())
    }

    #[test]
    fn blank_and_mismatched_rosters_refuse() -> Result<(), String> {
        let fixture = Fixture::new("blank-roster")?;
        fixture.write_report(&FixtureReport::run())?;
        fixture.write("caught.txt", b"caught one\n\ncaught two")?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("blank record"));

        fixture.write_report(&FixtureReport::run())?;
        fixture.write("caught.txt", b"caught one")?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("caught roster"));

        fixture.write_report(&FixtureReport::run())?;
        fixture.write("caught.txt", b"caught one\ncaught wrong")?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("caught roster"));
        Ok(())
    }

    #[test]
    fn inconsistent_and_unclassified_totals_refuse() -> Result<(), String> {
        let fixture = Fixture::new("inconsistent-total")?;
        let mut inconsistent = FixtureReport::run();
        inconsistent.total = 4;
        fixture.write_report(&inconsistent)?;
        assert!(
            refusal(inspect(ReportMode::Run, 2, &fixture.root))?
                .contains("total mutant population")
        );

        let successful = FixtureReport {
            total: 4,
            success: vec!["successful"],
            ..FixtureReport::run()
        };
        fixture.write_report(&successful)?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("successful mutant"));
        Ok(())
    }

    #[test]
    fn candidate_and_observation_populations_must_reconcile() -> Result<(), String> {
        let fixture = Fixture::new("candidate-mismatch")?;
        fixture.write_report(&FixtureReport::run())?;
        fixture.write(
            "mutants.json",
            br#"[{"name":"caught one"},{"name":"survived one"}]"#,
        )?;
        assert!(
            refusal(inspect(ReportMode::Run, 2, &fixture.root))?
                .contains("total mutant population")
        );

        fixture.write_report(&FixtureReport::run())?;
        fixture.write(
            "mutants.json",
            br#"[{"name":"caught one"},{"name":"caught wrong"},{"name":"survived one"}]"#,
        )?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("mutant inventory"));
        Ok(())
    }

    #[test]
    fn aggregate_and_per_scenario_classifications_must_reconcile() -> Result<(), String> {
        let fixture = Fixture::new("classification-mismatch")?;
        fixture.write_report(&FixtureReport::run())?;
        let outcome_text = fs::read_to_string(fixture.root.join("outcomes.json"))
            .map_err(|source| format!("cannot read fixture outcome: {source}"))?;
        let mut outcome: serde_json::Value = serde_json::from_str(&outcome_text)
            .map_err(|source| format!("cannot decode fixture outcome: {source}"))?;
        let Some(first_outcome) = outcome
            .get_mut("outcomes")
            .and_then(serde_json::Value::as_array_mut)
            .and_then(|outcomes| {
                outcomes.iter_mut().find(|observation| {
                    observation
                        .get("scenario")
                        .is_some_and(serde_json::Value::is_object)
                })
            })
        else {
            return Err(String::from("fixture outcome carries no scenario"));
        };
        let Some(summary) = first_outcome.get_mut("summary") else {
            return Err(String::from("fixture scenario carries no summary"));
        };
        *summary = json!("MissedMutant");
        fixture.write(
            "outcomes.json",
            serde_json::to_vec(&outcome)
                .map_err(|source| format!("cannot encode fixture outcome: {source}"))?,
        )?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("caught"));
        Ok(())
    }

    #[test]
    fn unfinished_reports_refuse() -> Result<(), String> {
        let fixture = Fixture::new("unfinished")?;
        let unfinished = FixtureReport {
            end_time: None,
            ..FixtureReport::run()
        };
        fixture.write_report(&unfinished)?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("not finalized"));
        Ok(())
    }

    #[test]
    fn silent_and_uncaught_run_controls_refuse() -> Result<(), String> {
        let fixture = Fixture::new("silent-run")?;
        fixture.write_report(&FixtureReport::reversal())?;
        assert!(
            refusal(inspect(ReportMode::Run, 0, &fixture.root))?.contains("examined no mutants")
        );

        let uncaught = FixtureReport {
            total: 1,
            caught: Vec::new(),
            missed: vec!["survived"],
            ..FixtureReport::reversal()
        };
        fixture.write_report(&uncaught)?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("caught no mutant"));
        Ok(())
    }

    #[test]
    fn exit_and_report_disagreement_refuses() -> Result<(), String> {
        let fixture = Fixture::new("exit-mismatch")?;
        fixture.write_report(&FixtureReport::run())?;
        assert!(refusal(inspect(ReportMode::Run, 0, &fixture.root))?.contains("exit 0 disagrees"));
        Ok(())
    }

    #[test]
    fn timeout_evidence_is_preserved_but_the_outcome_stays_unknown() -> Result<(), String> {
        let timeout_report = FixtureReport {
            total: 2,
            caught: vec!["caught"],
            missed: Vec::new(),
            timeout: vec!["timed out mutant"],
            ..FixtureReport::reversal()
        };
        let (_fixture, inspection) =
            inspected("timeout-unknown", ReportMode::Run, 3, &timeout_report)?;
        assert_eq!(inspection.disposition, ReportDisposition::OutcomeUnknown);
        assert!(inspection.receipt.contains("timed out mutant"));
        assert!(inspection.finish().is_err());
        Ok(())
    }

    #[test]
    fn report_bytes_are_bounded_and_strict_utf8() -> Result<(), String> {
        let fixture = Fixture::new("bounded-utf8")?;
        fixture.write("tiny", b"four")?;
        assert!(read_utf8_bounded(&fixture.root.join("tiny"), 3).is_err());
        fixture.write("tiny", [0xff_u8])?;
        assert!(read_utf8_bounded(&fixture.root.join("tiny"), 3).is_err());
        Ok(())
    }
}
