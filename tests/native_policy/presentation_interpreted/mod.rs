#![doc = include_str!("README.md")]

mod fixture;
mod proposal;
mod proposal_fixture;

use crate::presentation_formats::{field, parsed};
use crate::presentation_parity::{binding, datum, encoder, invocation, mapped, odd, pair_for};
use macroonz::harness::muterprater::{
    InterpreterAvailability, MutationWitness, NoMutationParityStanding,
    SpecimenMaterializerBinding,
    interpret::{availability, execute_active, observe_no_mutation, qualify_no_mutation},
    interpretation_archive::{
        read_interpreted, retain_interpreted, retain_interpreted_with_material,
    },
    specimen::demonstrate_compiled_projection,
};
use macroonz::presentation;
use serde_json::{Value, json};

#[test]
fn presentation_keeps_complete_interpreted_grounds_without_promoting_history() -> Result<(), String>
{
    for (evaluate, expected) in [(fixture::KILLED, "killed"), (fixture::SURVIVED, "survived")] {
        let surface = fixture::surface()?;
        let pair = pair_for(&surface, evaluate)?;
        let input = datum(1);
        let binding = binding()?;
        let check = binding.row().check();
        let witness = mapped(MutationWitness::bound(binding, check, odd))?;
        let invocation = invocation();
        let NoMutationParityStanding::Qualified(parity) = qualify_no_mutation(mapped(
            observe_no_mutation(&pair, witness, &input, &invocation),
        )?) else {
            return Err("declared no-mutation pair did not qualify".into());
        };
        let selection = surface
            .selections()
            .last()
            .copied()
            .ok_or("selection missing")?;
        let projection = mapped(demonstrate_compiled_projection(
            &surface,
            &parity,
            &SpecimenMaterializerBinding::bound(&pair, fixture::MATERIALIZER),
            selection,
            &invocation,
            fixture::HOST,
        ))?;
        let suite = fixture::suite()?;
        let InterpreterAvailability::Available(trust) =
            availability(Some(&surface), Some(&suite), Some(&projection))
        else {
            return Err("declared trust inputs did not join".into());
        };
        let evidence = mapped(execute_active(&trust, &invocation))?;
        for originals in [false, true] {
            let input_encoder = encoder("input")?;
            let meaning = encoder("meaning")?;
            let archive = mapped(if originals {
                retain_interpreted_with_material(
                    &evidence,
                    &input_encoder,
                    &meaning,
                    fixture::CONSOLE,
                    fixture::SOURCES,
                    &fixture::LIMITS,
                )
            } else {
                retain_interpreted(&evidence, &input_encoder, &meaning, &fixture::LIMITS)
            })?;
            let archive = mapped(read_interpreted(archive.encoded(), &fixture::LIMITS))?;
            let shown = parsed(&presentation::archived_interpreted(&archive))?;
            inspect(&shown, expected, originals.then_some("00ff736f75726365"))?;
            let nested = parsed(&presentation::archived_projection(
                archive.trust().projection(),
            ))?;
            assert_eq!(
                field(&nested, "/record")?,
                field(&shown, "/record/trust/projection")?
            );
            let parity_display = parsed(&presentation::archived_parity(
                archive.trust().projection().parity(),
            ))?;
            assert_eq!(
                field(&parity_display, "/record")?,
                field(&nested, "/record/parity")?
            );
            let backend = parsed(&presentation::archived_backend(
                archive.trust().suite().manifest(),
            ))?;
            assert_eq!(
                field(&backend, "/record")?,
                field(&shown, "/record/trust/suite/manifest")?
            );
        }
    }
    Ok(())
}

fn inspect(shown: &Value, expected: &str, original: Option<&str>) -> Result<(), String> {
    assert_eq!(field(shown, "/standing")?, "historical-unauthenticated");
    let record = field(shown, "/record")?;
    assert_eq!(field(record, "/mutation/outcome/kind")?, expected);
    assert_eq!(
        field(record, "/mutation/activation/value/firings")?,
        &json!(7u32)
    );
    assert_eq!(
        field(record, "/meaning/bytes")?,
        if expected == "killed" { "ff02" } else { "ff03" }
    );
    assert_eq!(
        field(record, "/meaning/convention")?,
        field(record, "/trust/projection/parity/evaluation/convention")?
    );
    let pressure = field(record, "/trust/projection")?;
    assert_eq!(field(pressure, "/baseline_content/bytes")?, "");
    assert_eq!(field(pressure, "/selected_content/bytes")?, "00ff0d");
    assert_ne!(
        field(pressure, "/baseline_content/identity")?,
        field(pressure, "/selected_content/identity")?
    );
    assert_eq!(
        field(pressure, "/standing/artifact")?,
        field(pressure, "/selected_content/identity")?
    );
    assert_eq!(
        field(pressure, "/baseline_report/attempt/value/kind")?,
        "passed"
    );
    assert_eq!(
        field(pressure, "/selected_report/attempt/value/kind")?,
        "refused"
    );
    assert_eq!(field(pressure, "/mutation/outcome/kind")?, "killed");
    assert_eq!(field(pressure, "/parity/disposition/kind")?, "qualified");
    assert!(pressure.get("baseline_meaning").is_none());
    assert!(pressure.get("selected_meaning").is_none());
    inspect_surface(field(record, "/trust/surface")?)?;
    let suite = field(record, "/trust/suite")?;
    assert_eq!(field(suite, "/kill_ordinal")?, &json!(1u64));
    assert_eq!(
        field(suite, "/manifest/reading/run/denominator")?,
        &json!(3usize)
    );
    assert_eq!(
        field(suite, "/kill")?,
        field(suite, "/manifest/reading/run/reports/1")?
    );
    assert_ne!(
        field(suite, "/kill")?,
        field(suite, "/manifest/reading/run/reports/2")?
    );
    assert_eq!(
        field(suite, "/manifest/original_console")?.is_string(),
        original.is_some()
    );
    assert_eq!(
        field(suite, "/manifest/sources/0/original")?,
        &json!(original)
    );
    Ok(())
}

fn inspect_surface(surface: &Value) -> Result<(), String> {
    let points = field(surface, "/points")?
        .as_array()
        .ok_or("points missing")?;
    assert_eq!(points.len(), 2);
    for (point, stem) in points.iter().zip(["first", "second"]) {
        assert_eq!(field(point, "/name/stem")?, stem);
        assert_eq!(field(point, "/original_operation")?, "ff");
        let alternatives = field(point, "/alternatives")?
            .as_array()
            .ok_or("alternatives missing")?;
        assert_eq!(alternatives.len(), 2);
        let mut operations: Vec<&str> = alternatives
            .iter()
            .filter_map(|value| value.get("operation").and_then(Value::as_str))
            .collect();
        operations.sort_unstable();
        assert_eq!(operations, ["00", "0d"]);
    }
    assert!(surface.get("discovery_denominator").is_none());
    assert!(surface.get("permissions").is_none());
    Ok(())
}

#[test]
fn presentation_keeps_withheld_discoveries_in_the_original_denominator() -> Result<(), String> {
    let lowering = fixture::lowering()?;
    assert_eq!(lowering.surface().points().len(), 2);
    let shown = parsed(&presentation::mutation_discovery(lowering.discovery()))?;
    assert_eq!(field(&shown, "/record/denominator")?, &json!(5usize));
    let entries = field(&shown, "/record/entries")?
        .as_array()
        .ok_or("discovery rows missing")?;
    assert_eq!(entries.len(), 5);
    for (entry, (stem, disposition)) in entries.iter().zip([
        ("unmapped", "owner-unmapped"),
        ("second", "mapped"),
        ("unpermitted-claim", "mapped-unpermitted"),
        ("unpermitted-family", "mapped-unpermitted"),
        ("first", "mapped"),
    ]) {
        assert_eq!(field(entry, "/site/name/stem")?, stem);
        assert_eq!(field(entry, "/disposition/kind")?, disposition);
        assert_eq!(field(entry, "/site/alternatives/0/operation")?, "00");
        assert_eq!(field(entry, "/site/alternatives/1/operation")?, "0d");
    }
    assert_eq!(
        field(&shown, "/record/entries/2/disposition/value")?,
        &json!({
            "kind":"claim", "value":{"namespace":"other", "stem":"claim"},
        })
    );
    assert_eq!(
        field(&shown, "/record/entries/3/disposition/value")?,
        &json!({
            "kind":"family", "value":{"at":1usize, "family":"boolean-operators"},
        })
    );
    Ok(())
}
