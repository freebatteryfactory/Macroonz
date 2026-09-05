//! A declared compiled campaign must account for exact mutant identities and terminal outcomes.

use macroonz_harness::muterprater::{
    AnnouncedRoster, ExecutionAxis, MaterializationAxis, MutantId, MutationIdentity,
    MutationVerdict, SourceCoordinate, WrapReading,
};
use std::collections::BTreeSet;

pub(super) fn expected(text: &str) -> Result<BTreeSet<MutantId>, String> {
    let mut identities = BTreeSet::new();
    for row in text.lines() {
        let fields = row.splitn(4, '\t').collect::<Vec<_>>();
        let [file, line, column, damage] = fields.as_slice() else {
            return Err("expected mutant row must have four declared fields".to_owned());
        };
        if file.is_empty() || damage.is_empty() {
            return Err("expected mutant coordinate and damage must be nonempty".to_owned());
        }
        let line = line.parse::<u32>().map_err(|error| error.to_string())?;
        let column = column.parse::<u32>().map_err(|error| error.to_string())?;
        let coordinate =
            SourceCoordinate::reported(file, line, column).map_err(|error| format!("{error:?}"))?;
        if !identities.insert(MutantId::over(&coordinate, damage.as_bytes())) {
            return Err("expected mutant roster repeats an identity".to_owned());
        }
    }
    if identities.is_empty() {
        return Err("expected mutant roster is empty".to_owned());
    }
    Ok(identities)
}

pub(super) fn reconcile(
    reading: &WrapReading,
    declared: &BTreeSet<MutantId>,
) -> Result<(), String> {
    let count = u32::try_from(declared.len()).map_err(|error| error.to_string())?;
    if count == 0 || reading.announced() != AnnouncedRoster::Stated(count) {
        return Err("announced and declared nonempty populations disagree".to_owned());
    }
    let mut observed = BTreeSet::new();
    for report in reading.run().reports() {
        let MutationIdentity::External(identity) = report.target().identity() else {
            return Err("compiled campaign reported a nonexternal identity".to_owned());
        };
        if !observed.insert(identity) {
            return Err("compiled campaign repeats a mutant identity".to_owned());
        }
        match (
            report.materialization(),
            report.execution(),
            report.verdict(),
        ) {
            (MaterializationAxis::Built, ExecutionAxis::Completed, MutationVerdict::Killed)
            | (
                MaterializationAxis::Unviable,
                ExecutionAxis::InfrastructureFailed,
                MutationVerdict::Inconclusive,
            ) => {}
            _ => return Err("compiled campaign contains an unresolved outcome".to_owned()),
        }
    }
    if observed != *declared {
        return Err("reported and declared mutant identities disagree".to_owned());
    }
    Ok(())
}

#[test]
fn exact_identity_accounting_refuses_missing_repeated_and_substituted_reports() -> Result<(), String>
{
    use macroonz_harness::muterprater::wrap::read_output;
    use macroonz_harness::muterprater::{BackendVersion, BackendVersionPosture};

    let declared = expected("subject.rs\t1\t1\treplace first\nsubject.rs\t2\t1\treplace second\n")?;
    let cases = [
        (
            "Found 2 mutants\nok Unmutated baseline\ncaught subject.rs:1:1: replace first\nunviable subject.rs:2:1: replace second\n",
            true,
        ),
        (
            "Found 2 mutants\nok Unmutated baseline\ncaught subject.rs:1:1: replace first\n",
            false,
        ),
        (
            "Found 2 mutants\nok Unmutated baseline\ncaught subject.rs:1:1: replace first\ncaught subject.rs:1:1: replace first\n",
            false,
        ),
        (
            "Found 2 mutants\nok Unmutated baseline\ncaught subject.rs:1:1: replace first\nunviable subject.rs:2:1: replace different\n",
            false,
        ),
        (
            "Found 1 mutants\nok Unmutated baseline\ncaught subject.rs:1:1: replace first\nunviable subject.rs:2:1: replace second\n",
            false,
        ),
    ];
    for (console, accepted) in cases {
        let reading = read_output(
            console,
            BackendVersionPosture::Stated(
                BackendVersion::stated("27.0.0").map_err(|error| format!("{error:?}"))?,
            ),
            |_| None,
            |_, _| None,
        )
        .map_err(|error| format!("{error:?}"))?;
        assert_eq!(reconcile(&reading, &declared).is_ok(), accepted);
    }
    Ok(())
}

#[test]
fn missed_timed_out_and_failed_variants_are_not_credited_as_compiler_refusals() -> Result<(), String>
{
    use macroonz_harness::muterprater::wrap::read_output;
    use macroonz_harness::muterprater::{BackendVersion, BackendVersionPosture};

    let declared = expected("subject.rs\t1\t1\treplace first\n")?;
    for outcome in ["missed", "timeout", "failed"] {
        let console = format!(
            "Found 1 mutants\nok Unmutated baseline\n{outcome} subject.rs:1:1: replace first\n"
        );
        let reading = read_output(
            &console,
            BackendVersionPosture::Stated(
                BackendVersion::stated("27.0.0").map_err(|error| format!("{error:?}"))?,
            ),
            |_| None,
            |_, _| None,
        )
        .map_err(|error| format!("{error:?}"))?;
        assert_eq!(
            reconcile(&reading, &declared),
            Err("compiled campaign contains an unresolved outcome".to_owned())
        );
    }
    Ok(())
}

#[test]
fn the_expected_roster_refuses_empty_malformed_and_duplicate_declarations() {
    for material in [
        "",
        "subject.rs\t1\t1",
        "\t1\t1\treplace first",
        "subject.rs\tbad\t1\treplace first",
        "subject.rs\t1\tbad\treplace first",
        "subject.rs\t1\t1\t",
        "subject.rs\t1\t1\treplace first\nsubject.rs\t1\t1\treplace first\n",
    ] {
        assert!(expected(material).is_err(), "accepted {material:?}");
    }
}
