//! Independent complete-denominator and multi-axis observations across display formats.

use super::{
    declaration, fixture,
    specimen::{self, mapped},
};
use crate::presentation_formats::{self, field};
use macroonz::harness::bench::archive::{BenchArchiveLimits, read_report, retain_report};
use macroonz::harness::bench::{BenchInvocation, ContentionPosture, bench_verdict, run_all};
use macroonz::harness::clock::{ClockAttribution, ClockReadRefusal, HarnessClock};
use macroonz::harness::report::archive::ArchiveLimits;
use macroonz::harness::report::{TargetBinding, TargetTriple, ToolchainIdentity};
use macroonz::presentation;
use serde_json::{Value, json};

const LIMITS: BenchArchiveLimits =
    BenchArchiveLimits::declared(ArchiveLimits::declared(65536, 16384), 8, 8, 4, 32);

fn parsed(shown: &presentation::Presentation) -> Result<Value, String> {
    presentation_formats::agree(shown)?;
    mapped(serde_json::from_str(&shown.json()))
}

fn complete(value: &Value) -> bool {
    let Some(rows) = value.pointer("/record/readings").and_then(Value::as_array) else {
        return false;
    };
    let expected = [
        "qualified",
        "preflight-refused",
        "planted-worse-not-distinguished",
        "primary-work-refused",
        "planted-worse-not-distinguished",
    ];
    value.pointer("/record/denominator").and_then(Value::as_u64) == Some(5)
        && rows.len() == expected.len()
        && rows.iter().zip(expected).all(|(row, stage)| {
            row.pointer("/outcome/stage").and_then(Value::as_str) == Some(stage)
        })
        && rows.last().is_some_and(|row| {
            row.pointer("/outcome/judgment/measured/value/local")
                .and_then(Value::as_str)
                == Some("wrong-measured")
                && row
                    .pointer("/outcome/judgment/planted_worse/kind")
                    .and_then(Value::as_str)
                    == Some("satisfied")
                && row
                    .pointer("/outcome/judgment/gap/value/local")
                    .and_then(Value::as_str)
                    == Some("missing-gap")
        })
}

fn observations(value: &Value) -> Result<(), String> {
    assert!(complete(value));
    let rows = field(value, "/record/readings")?;
    assert_eq!(
        field(rows, "/0/row/measurement")?,
        &json!({
            "input_sizes": [4u64, 8u64, 16u64], "samples": 2u32, "warmups": 1u32,
            "ratio": {"numerator": 2u64, "denominator": 1u64},
            "contention": "no-declared-contention", "formula": null,
        })
    );
    for (index, measured, worse) in [(0usize, 8u64, 32u64), (1, 16, 128), (2, 32, 512)] {
        assert_eq!(
            field(rows, &format!("/0/outcome/measured/{index}/counts/0/count"))?,
            &Value::from(measured)
        );
        assert_eq!(
            field(
                rows,
                &format!("/0/outcome/planted_worse/{index}/counts/0/count")
            )?,
            &Value::from(worse)
        );
    }
    assert_eq!(
        field(rows, "/0/outcome/secondary/measurements")?,
        &json!([
            {"kind":"unavailable","value":null}, {"kind":"unavailable","value":null},
            {"kind":"unavailable","value":null}, {"kind":"unavailable","value":null},
            {"kind":"unavailable","value":null}, {"kind":"unavailable","value":null},
        ])
    );
    assert_eq!(field(rows, "/1/preflight/attempt/value/kind")?, "refused");
    for key in ["measured", "planted_worse", "judgment", "secondary"] {
        assert!(
            field(rows, &format!("/1/outcome/{key}"))?.is_null(),
            "invented unreached {key}"
        );
    }
    assert!(field(rows, "/4/outcome/secondary")?.is_null());
    Ok(())
}

#[test]
fn presentation_preserves_all_reached_axes_and_first_refusal_without_execution()
-> Result<(), String> {
    let report = fixture::report(HarnessClock::unavailable())?;
    let first = bench_verdict(&report)
        .err()
        .ok_or("fixture unexpectedly qualified")?;
    let retained = mapped(retain_report(&report, LIMITS))?;
    let loaded = mapped(read_report(retained.encoded(), LIMITS))?;
    fixture::reset();
    let current = parsed(&presentation::benchmark(&report))?;
    let mut historical = parsed(&presentation::archived_benchmark(&loaded))?;
    let verdict = parsed(&presentation::benchmark_verdict(&first))?;
    assert_eq!(fixture::calls(), (0, 0, 0, 0));
    observations(&current)?;
    observations(&historical)?;
    assert_eq!(field(&verdict, "/kind")?, "benchmark-verdict-refusal");
    assert_eq!(field(&verdict, "/record/stage")?, "preflight-refused");
    assert_eq!(
        field(&verdict, "/record/row")?,
        field(&current, "/record/readings/1/row/key")?
    );
    assert_eq!(
        field(&historical, "/standing")?,
        "historical-unauthenticated"
    );
    let record = historical
        .get_mut("record")
        .and_then(Value::as_object_mut)
        .ok_or("historical object absent")?;
    record.remove("archive_address");
    for row in record
        .get_mut("readings")
        .and_then(Value::as_array_mut)
        .ok_or("readings absent")?
    {
        row.get_mut("preflight")
            .and_then(Value::as_object_mut)
            .ok_or("preflight absent")?
            .remove("archive_address");
    }
    assert_eq!(field(&current, "/record")?, field(&historical, "/record")?);
    let mut omitted = current.clone();
    omitted
        .pointer_mut("/record/readings")
        .and_then(Value::as_array_mut)
        .ok_or("rows absent")?
        .remove(1);
    assert!(
        !complete(&omitted),
        "the denominator check accepted a dropped failure"
    );
    let mut masked = current;
    *masked
        .pointer_mut("/record/readings/4/outcome/judgment/measured")
        .ok_or("measured judgment absent")? = json!({"kind":"satisfied","value":null});
    assert!(
        !complete(&masked),
        "the multi-axis check accepted a hidden refusal"
    );
    Ok(())
}

#[test]
fn presentation_keeps_secondary_zero_and_failed_readings_distinct() -> Result<(), String> {
    for (clock, expected, attribution) in [
        (
            HarnessClock::reading_as(|| 0, ClockAttribution::Synthetic),
            json!({"kind":"observed","value":0u64}),
            "synthetic",
        ),
        (
            HarnessClock::fallible_as(
                || Err(ClockReadRefusal::Refused),
                ClockAttribution::Monotonic,
            ),
            json!({"kind":"failed","value":"opening-refused"}),
            "monotonic",
        ),
    ] {
        let report = fixture::report(clock)?;
        let retained = mapped(retain_report(&report, LIMITS))?;
        for shown in [
            presentation::benchmark(&report),
            presentation::archived_benchmark(&retained),
        ] {
            let value = parsed(&shown)?;
            assert!(complete(&value));
            let secondary = field(&value, "/record/readings/0/outcome/secondary")?;
            assert_eq!(field(secondary, "/clock_attribution")?, attribution);
            let measures = field(secondary, "/measurements")?
                .as_array()
                .ok_or("measurements absent")?;
            assert_eq!(measures.len(), 6);
            assert!(measures.iter().all(|measure| measure == &expected));
        }
    }
    Ok(())
}

#[test]
fn presentation_escapes_caller_names_and_refuses_to_invent_a_report() -> Result<(), String> {
    let hostile = "qualified <em>x</em> | [go](https://bad.invalid) &";
    let table = declaration::table(vec![declaration::binding(
        hostile,
        specimen::measured,
        specimen::worse,
        specimen::judge,
        specimen::preflight,
    )?])?;
    let report = mapped(run_all(
        &table,
        &declaration::invocation(HarnessClock::unavailable()),
    ))?;
    let shown = presentation::benchmark(&report);
    let value = parsed(&shown)?;
    assert_eq!(
        field(&value, "/record/readings/0/row/workload/stem")?,
        hostile
    );
    assert!(!shown.html().contains("<em>"));
    assert!(!shown.markdown().contains("[go](https://bad.invalid)"));
    let invocation = BenchInvocation::declared(
        TargetBinding::bound(
            TargetTriple::declared("wrong-target"),
            ToolchainIdentity::declared("1.98.1"),
        ),
        HarnessClock::unavailable(),
        ContentionPosture::NoDeclaredContention,
    );
    let refusal = run_all(&table, &invocation)
        .err()
        .ok_or("wrong target ran")?;
    let rejected = parsed(&presentation::benchmark_refusal(&refusal))?;
    assert_eq!(field(&rejected, "/kind")?, "benchmark-run-refusal");
    assert_eq!(
        field(&rejected, "/record/kind")?,
        "preflight-target-mismatch"
    );
    assert_eq!(
        field(&rejected, "/record/value/mismatch")?,
        &json!({"kind":"target","value":{
            "benchmark":"wrong-target","preflight":"neutral-counting-host",
        }})
    );
    assert!(rejected.pointer("/record/readings").is_none());
    assert!(rejected.pointer("/record/denominator").is_none());
    Ok(())
}
