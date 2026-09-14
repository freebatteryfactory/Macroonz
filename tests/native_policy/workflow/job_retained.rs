//! Job reduction and historical input cross native processes with unchanged independent controls.

#[path = "../../../examples/job_retained/admission.rs"]
mod admission;

use super::{example, fixture, retention};
use crate::compiler::configure::{root, spelling};
use crate::compiler::job_example::invoke;
use crate::presentation_formats::field;
use macroonz::harness::runner::trial_identity;
use macroonz::native_process::{ProcessOutput, ProcessTool};
use macroonz::native_storage::StorageName;
use macroonz::workflow::StoredRun;
use serde_json::{Value, json};
use std::path::Path;

#[test]
fn public_job_retained_reaches_four_and_reexecutes_without_reduction() -> Result<(), String> {
    let run = root()?;
    let selected = example::executable(&run, "job_retained")?;
    let retained = invoke(&selected, &run, "retained", &configuration(&run, "retain")?)?;
    let record = success(&retained)?;
    assert_eq!(field(&record, "/record/input/payload")?, "010204");
    assert_eq!(field(&record, "/record/report/denominator")?, 1usize);
    assert_eq!(
        field(
            &record,
            "/record/report/census/0/disposition/value/attempt/value/kind"
        )?,
        "refused"
    );
    assert!(text(&retained)?.ends_with(
        "retained original [1, 2, 4]; reached witness [4]; nonfailing baseline refused\n"
    ));
    let inspected = invoke(
        &selected,
        &run,
        "inspected",
        &configuration(&run, "inspect")?,
    )?;
    let historical = success(&inspected)?;
    assert_eq!(
        field(&historical, "/standing")?,
        "historical-unauthenticated"
    );
    assert_eq!(field(&historical, "/record/input/payload")?, "010204");
    assert_eq!(
        field(&historical, "/record/capsules/0/capsule/input")?,
        "04"
    );
    for (coordinate, expected) in [
        ("family", "neutral-job"),
        ("local", "bounded-work"),
        ("class", "property-disagreement"),
    ] {
        assert_eq!(
            field(
                &historical,
                &format!("/record/capsules/0/capsule/fingerprint/{coordinate}")
            )?,
            expected
        );
    }
    assert!(text(&inspected)?.ends_with("decodes=0 checks=0 probes=0\n"));
    let replayed = invoke(&selected, &run, "replayed", &configuration(&run, "replay")?)?;
    let current = success(&replayed)?;
    assert_eq!(
        field(&current, "/record/outcome/kind")?,
        "defect-reproduced"
    );
    assert_eq!(field(&current, "/record/lineage")?, "reached-witness");
    for coordinate in ["trial", "subject", "check", "profile", "schema", "decoder"] {
        assert_eq!(
            field(&current, &format!("/record/movement/{coordinate}"))?,
            "same"
        );
    }
    assert!(
        text(&replayed)?.ends_with(
            "DefectReproduced; witness: decodes=1 checks=1 probes=0; lawful [1]: passed\n"
        )
    );
    damaged(&selected, &run)
}

fn damaged(selected: &ProcessTool, run: &Path) -> Result<(), String> {
    let request = configuration(run, "inspect")?;
    for member in ["input", "run", "capsule-0"] {
        let path = run.join("item-work").join(format!("item-{member}"));
        let original = std::fs::read(&path).map_err(debug)?;
        let mut corrupted = original.clone();
        *corrupted.last_mut().ok_or("empty retained member")? ^= 1;
        std::fs::write(&path, corrupted).map_err(debug)?;
        refused(
            &invoke(selected, run, &format!("corrupt-{member}"), &request)?,
            "load the complete retained batch",
        );
        let truncated = original
            .get(..original.len().saturating_sub(1))
            .ok_or("missing retained bytes")?;
        std::fs::write(&path, truncated).map_err(debug)?;
        refused(
            &invoke(selected, run, &format!("truncated-{member}"), &request)?,
            "load the complete retained batch",
        );
        std::fs::write(&path, &original).map_err(debug)?;
    }
    success(&invoke(selected, run, "restored", &request)?)?;
    Ok(())
}

#[test]
fn job_retained_admission_refuses_when_only_population_changes() -> Result<(), String> {
    retention::observe(|root, _path| {
        let original = fixture::run(&[7, 1, 9])?;
        let name = StorageName::informed("saved").map_err(debug)?;
        original
            .retain(
                root,
                &name,
                &[fixture::capsule(&original)?],
                retention::LIMITS,
            )
            .map_err(debug)?;
        fixture::reset();
        let saved = StoredRun::load(
            root,
            &name,
            fixture::decoder()?.profile(),
            retention::LIMITS,
        )
        .map_err(debug)?;
        let selected = fixture::binding("selected", fixture::revision(), fixture::defective)?;
        let foreign = fixture::binding("foreign", fixture::revision(), fixture::defective)?;
        assert_eq!(selected.row().claim(), foreign.row().claim());
        assert_eq!(selected.row().subject(), foreign.row().subject());
        assert_eq!(selected.row().check(), foreign.row().check());
        assert_ne!(selected.row().population(), foreign.row().population());
        assert_eq!(
            admission::capsule(&saved, trial_identity(selected.row()))?.input(),
            &[1]
        );
        assert_eq!(
            admission::capsule(&saved, trial_identity(foreign.row()))
                .err()
                .ok_or("foreign population admitted")?,
            "saved witness belongs to another trial or population"
        );
        assert_eq!(fixture::observations(), (0, 0, 0));
        Ok(())
    })
}

fn configuration(run: &Path, action: &str) -> Result<Value, String> {
    Ok(json!({"action":action, "storage":spelling(run)?, "batch":"work"}))
}

fn success(output: &ProcessOutput) -> Result<Value, String> {
    assert!(
        output.status().success(),
        "{}",
        String::from_utf8_lossy(output.stderr().bytes())
    );
    let mut lines = text(output)?.lines();
    let value = serde_json::from_str(lines.next().ok_or("missing record")?).map_err(debug)?;
    assert!(lines.next().is_some());
    assert!(lines.next().is_none());
    Ok(value)
}

fn refused(output: &ProcessOutput, expected: &str) {
    assert!(!output.status().success());
    assert!(output.stdout().bytes().is_empty());
    let diagnostic = String::from_utf8_lossy(output.stderr().bytes());
    assert!(diagnostic.contains(expected), "{diagnostic}");
}

fn text(output: &ProcessOutput) -> Result<&str, String> {
    std::str::from_utf8(output.stdout().bytes()).map_err(debug)
}

fn debug(error: impl core::fmt::Debug) -> String {
    format!("{error:?}")
}
