//! Execute the public example and independently read its complete displayed journeys.

use super::presentation_formats::{field, read_formats};
use serde_json::{Value, json};
use std::path::Path;

#[test]
fn public_trial_example_displays_selection_and_real_disagreement_in_every_format()
-> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let scratch = super::check::scratch()?;
    let output = super::check::cargo(
        root,
        root,
        &scratch,
        "presentation-example",
        &[
            "run",
            "--example",
            "trial_workflow",
            "--no-default-features",
            "--features",
            "harness",
            "--locked",
            "--offline",
            "-j1",
        ],
    )?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = std::str::from_utf8(&output.stdout).map_err(|error| error.to_string())?;
    let mut sections = text
        .strip_prefix("--- ")
        .ok_or("missing first example section")?
        .split("\n--- ");
    for (label, payload, selected, conclusion) in [
        ("selected", "010203", 1, "passed"),
        ("all", "010203", 2, "passed"),
        ("refused", "010204", 1, "refused"),
    ] {
        let mut formats = Vec::new();
        for format in ["JSON", "Markdown", "HTML"] {
            let section = sections.next().ok_or("missing example format")?;
            formats.push(
                section
                    .strip_prefix(&format!("{label} {format} ---\n"))
                    .ok_or("wrong example format order")?
                    .trim_end(),
            );
        }
        let [json, markdown, html] = formats.as_slice() else {
            return Err("incomplete example format set".to_owned());
        };
        let value = read_formats(json, markdown, html)?;
        assert_eq!(field(&value, "/kind")?, &json!("input-run"));
        assert_eq!(field(&value, "/record/input/payload")?, &json!(payload));
        assert_eq!(field(&value, "/record/report/denominator")?, &json!(2_u64));
        census(&value, selected, conclusion)?;
    }
    assert!(sections.next().is_none());
    Ok(())
}

fn census(value: &Value, selected: usize, conclusion: &str) -> Result<(), String> {
    let rows = field(value, "/record/report/census")?
        .as_array()
        .ok_or("census absent")?;
    assert_eq!(rows.len(), 2);
    let mut observed = 0usize;
    for row in rows {
        match field(row, "/disposition/kind")?.as_str() {
            Some("selected") => {
                observed = observed.saturating_add(1);
                assert_eq!(
                    field(row, "/disposition/value/attempt/kind")?,
                    &json!("executed")
                );
                assert_eq!(
                    field(row, "/disposition/value/attempt/value/kind")?,
                    &json!(conclusion)
                );
                if conclusion == "refused" {
                    assert_eq!(
                        field(
                            row,
                            "/disposition/value/attempt/value/value/fingerprint/class"
                        )?,
                        &json!("property-disagreement")
                    );
                }
            }
            Some("not-selected") => {
                assert_eq!(field(row, "/disposition/value")?, &json!("suite-not-run"));
            }
            other => return Err(format!("unexpected trial disposition {other:?}")),
        }
    }
    assert_eq!(observed, selected);
    Ok(())
}
