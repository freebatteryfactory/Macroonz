//! Reached search facts survive presentation without executing another search.

use super::fixture::{self, mapped};
use crate::presentation_formats::{self, field};
use macroonz::harness::generate::reduce::archive::{
    ReductionArchiveLimits, read_reduction, retain_reduction,
};
use macroonz::harness::generate::{
    ByteReducerId, FingerprintPreservation, ReductionBudget, ReductionPlan,
    SemanticCandidateRefusal, SemanticCandidates, SemanticReducerBinding, SemanticReducerId,
};
use macroonz::harness::report::{MinimizationProfile, archive::ArchiveLimits};
use macroonz::presentation;
use serde_json::{Value, json};

fn offered(input: &[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal> {
    SemanticCandidates::proposed(input, vec![vec![1], Vec::new()])
}

fn unreachable(_input: &[u8]) -> Result<SemanticCandidates, SemanticCandidateRefusal> {
    std::panic::resume_unwind(Box::new("unreached reducer executed"))
}

#[test]
fn presentation_keeps_reproduction_and_wrong_witness_join_distinct() -> Result<(), String> {
    let original = fixture::run(&[1, 2])?;
    let retained = mapped(macroonz::harness::report::archive::retain_capsule(
        &fixture::capsule(&original)?,
        ArchiveLimits::declared(65536, 16384),
    ))?;
    let current = fixture::run(&[1])?;
    let reading = mapped(macroonz::harness::report::replay::compare(
        &retained,
        fixture::selected(&current)?,
        current.input(),
    ))?;
    fixture::reset();
    let comparison = presentation::replay_comparison(&reading);
    presentation_formats::agree(&comparison)?;
    let value: Value = mapped(serde_json::from_str(&comparison.json()))?;
    assert_eq!(fixture::observations(), (0, 0, 0));
    assert_eq!(field(&value, "/record/outcome/kind")?, "defect-reproduced");
    assert_eq!(field(&value, "/record/lineage")?, "reached-witness");
    assert_eq!(
        field(&value, "/record/historical")?,
        &json!({"kind":"comparable","value":"exact-derived"})
    );
    assert_eq!(
        field(&value, "/record/movement")?,
        &json!({
            "trial":"same","subject":"same","check":"same","profile":"same","schema":"same",
            "decoder":"same","target":"same","toolchain":"same","invocation":"same",
        })
    );
    let refusal = macroonz::harness::report::replay::compare(
        &retained,
        fixture::selected(&current)?,
        original.input(),
    )
    .err()
    .ok_or("wrong witness compared")?;
    let rejected = presentation::replay_join_refusal(refusal);
    presentation_formats::agree(&rejected)?;
    let failed: Value = mapped(serde_json::from_str(&rejected.json()))?;
    assert_eq!(field(&failed, "/kind")?, "replay-join-refusal");
    assert_eq!(field(&failed, "/record/cause")?, "witness-bytes-differ");
    assert!(failed.pointer("/record/outcome").is_none());
    Ok(())
}

fn plan(budget: u32, semantic: Vec<SemanticReducerBinding>) -> Result<ReductionPlan, String> {
    mapped(ReductionPlan::declared(
        MinimizationProfile::declared("display-search", 1),
        ByteReducerId::ChunkRemovalAndZeroing,
        semantic,
        FingerprintPreservation::Required,
        ReductionBudget::declared(budget),
    ))
}

fn shown(evidence: &macroonz::harness::generate::ReductionEvidence) -> Result<Value, String> {
    let limits = ReductionArchiveLimits::declared(ArchiveLimits::declared(65536, 16384), 4);
    let retained = mapped(retain_reduction(evidence, limits))?;
    let loaded = mapped(read_reduction(retained.encoded(), limits))?;
    fixture::reset();
    let current = presentation::reduction(evidence);
    let historical = presentation::archived_reduction(&loaded);
    presentation_formats::agree(&current)?;
    presentation_formats::agree(&historical)?;
    let present: Value = mapped(serde_json::from_str(&current.json()))?;
    let mut past: Value = mapped(serde_json::from_str(&historical.json()))?;
    assert_eq!(fixture::observations(), (0, 0, 0));
    assert_eq!(field(&present, "/standing")?, "recorded");
    assert_eq!(field(&past, "/standing")?, "historical-unauthenticated");
    let record = past
        .get_mut("record")
        .and_then(Value::as_object_mut)
        .ok_or("historical record absent")?;
    for key in [
        "archive_address",
        "capsule_archive_address",
        "capsule_identity",
    ] {
        assert!(record.remove(key).is_some());
    }
    assert_eq!(field(&present, "/record")?, field(&past, "/record")?);
    assert!(present.pointer("/record/original_bytes").is_none());
    assert!(present.pointer("/record/global_minimum").is_none());
    Ok(present)
}

#[test]
fn presentation_keeps_budget_halt_and_only_invoked_semantic_reducers() -> Result<(), String> {
    let original = fixture::run(&[1, 2])?;
    let calls: [(&str, macroonz::harness::generate::SemanticReducerCall); 2] =
        [("reached <reducer>", offered), ("unreached", unreachable)];
    let reducers = calls
        .into_iter()
        .map(|(name, call)| {
            Ok(SemanticReducerBinding::bound(
                mapped(SemanticReducerId::named("display-search", name))?,
                fixture::revision(),
                call,
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let value = shown(&fixture::reduction(&original, &plan(1, reducers)?)?)?;
    assert_eq!(field(&value, "/record/budget")?, 1u32);
    assert_eq!(field(&value, "/record/outcome/halt")?, "budget-exhausted");
    assert_eq!(field(&value, "/record/outcome/input")?, "01");
    assert_eq!(
        field(&value, "/record/outcome/census")?,
        &json!({
            "accepted":1u32,"fingerprint_moved":0u32,"no_failure":0u32,"probes":1u32,
        })
    );
    assert_eq!(
        field(&value, "/record/byte_reducer/kind")?,
        "not-reached-because-budget-spent"
    );
    let reached = field(&value, "/record/semantic_reducers")?
        .as_array()
        .ok_or("reducers absent")?;
    let [invoked] = reached.as_slice() else {
        return Err("invented or lost reducer".to_owned());
    };
    assert_eq!(field(invoked, "/name/stem")?, "reached <reducer>");
    assert_eq!(field(invoked, "/candidates")?, 2u64);
    assert_eq!(field(invoked, "/probes")?, 1u64);
    Ok(())
}

#[test]
fn presentation_keeps_a_generic_fixed_point_distinct_from_budget_exhaustion() -> Result<(), String>
{
    let original = fixture::run(&[1, 2])?;
    let value = shown(&fixture::reduction(&original, &plan(32, Vec::new())?)?)?;
    assert_eq!(
        field(&value, "/record/outcome/halt")?,
        "fixed-point-reached"
    );
    assert_eq!(field(&value, "/record/outcome/input")?, "01");
    assert_eq!(
        field(&value, "/record/byte_reducer")?,
        &json!({"kind":"executed","value":"chunk-removal-and-zeroing"})
    );
    assert_eq!(field(&value, "/record/semantic_reducers")?, &json!([]));
    assert!(
        field(&value, "/record/outcome/census/no_failure")?
            .as_u64()
            .is_some_and(|count| count > 0)
    );
    assert_eq!(
        field(&value, "/record/outcome/census/fingerprint_moved")?,
        0u32
    );
    Ok(())
}
