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

/// The producer report fields `ThreadPak` consumes from cargo-mutants v27.
#[derive(Deserialize)]
struct LabOutcome {
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
    let mutant_count = read_mutant_count(report_directory)?;
    let rosters = read_rosters(report_directory)?;
    let (outcome, disposition) = match mode {
        ReportMode::Run => {
            let run_outcome = read_outcome(&report_directory.join("outcomes.json"))?
                .ok_or_else(|| String::from("the mutation run wrote no outcomes.json"))?;
            validate_final_outcome(&run_outcome, &producer_version)?;
            validate_counts(&run_outcome, &rosters)?;
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
            validate_reversal(exit_code, mutant_count, &rosters)?;
            (None, ReportDisposition::Accepted)
        }
    };
    let receipt = render_receipt(
        mode,
        &producer_version,
        outcome.as_ref(),
        mutant_count,
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
    Ok(lock.cargo_mutants_version)
}

fn read_mutant_count(report_directory: &Path) -> Result<usize, String> {
    let mutants_path = report_directory.join("mutants.json");
    let mutants_text = read_utf8_bounded(&mutants_path, STRUCTURED_REPORT_BYTE_LIMIT)?;
    let mutants: serde_json::Value = serde_json::from_str(&mutants_text)
        .map_err(|source| format!("cannot decode {} as JSON: {source}", mutants_path.display()))?;
    let Some(rows) = mutants.as_array() else {
        return Err(format!(
            "{} is not the cargo-mutants v27 mutant array",
            mutants_path.display()
        ));
    };
    Ok(rows.len())
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

fn validate_counts(outcome: &LabOutcome, rosters: &Rosters) -> Result<(), String> {
    compare_count("caught", outcome.caught, rosters.caught.len())?;
    compare_count("missed", outcome.missed, rosters.missed.len())?;
    compare_count("timeout", outcome.timeout, rosters.timeout.len())?;
    compare_count("unviable", outcome.unviable, rosters.unviable.len())?;

    let classified = [
        outcome.caught,
        outcome.missed,
        outcome.timeout,
        outcome.unviable,
        outcome.success,
    ]
    .into_iter()
    .try_fold(0_usize, |subtotal, count| {
        subtotal.checked_add(count).ok_or_else(|| {
            String::from("cargo-mutants outcome counters overflowed while deriving the population")
        })
    })?;
    if classified != outcome.total_mutants {
        return Err(format!(
            "cargo-mutants classified {classified} of {} total mutants; the report contains an unclassified or inconsistent outcome",
            outcome.total_mutants
        ));
    }
    if outcome.success != 0 {
        return Err(format!(
            "cargo-mutants reported {} successful mutant outcome(s), which this run contract does not classify",
            outcome.success
        ));
    }
    Ok(())
}

fn compare_count(role: &str, structured: usize, roster: usize) -> Result<(), String> {
    if structured == roster {
        Ok(())
    } else {
        Err(format!(
            "cargo-mutants {role} count is {structured} in outcomes.json but {roster} in {role}.txt"
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
                "outcomes": [],
                "total_mutants": report.total,
                "missed": report.missed.len(),
                "caught": report.caught.len(),
                "timeout": report.timeout.len(),
                "unviable": report.unviable.len(),
                "success": report.success,
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
            self.write("mutants.json", b"[]")?;
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
        success: usize,
        end_time: Option<&'static str>,
    }

    impl FixtureReport {
        fn run() -> Self {
            Self {
                total: 3,
                caught: vec!["caught one", "caught two"],
                missed: vec!["survived one"],
                timeout: Vec::new(),
                unviable: Vec::new(),
                success: 0,
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
                success: 0,
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
    fn producer_version_disagreement_refuses() -> Result<(), String> {
        let fixture = Fixture::new("version-disagreement")?;
        fixture.write_report(&FixtureReport::run())?;
        fixture.write("lock.json", br#"{"cargo_mutants_version":"28.0.0"}"#)?;
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
        object.remove("caught");
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
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("caught count"));
        Ok(())
    }

    #[test]
    fn inconsistent_and_unclassified_totals_refuse() -> Result<(), String> {
        let fixture = Fixture::new("inconsistent-total")?;
        let mut inconsistent = FixtureReport::run();
        inconsistent.total = 4;
        fixture.write_report(&inconsistent)?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("classified 3 of 4"));

        let successful = FixtureReport {
            total: 4,
            success: 1,
            ..FixtureReport::run()
        };
        fixture.write_report(&successful)?;
        assert!(refusal(inspect(ReportMode::Run, 2, &fixture.root))?.contains("successful mutant"));
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
