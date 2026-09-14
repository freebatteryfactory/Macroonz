//! Complete historical assessments from controlled host readings, without compiler certification.

use super::fixture;
use crate::presentation_formats::{field, parsed};
use crate::presentation_parity::{
    Datum, binding, datum, encoder, invocation, mapped, odd, pair_for,
};
use macroonz::harness::descriptor::archive::BindingArchiveLimits;
use macroonz::harness::muterprater::discovery_archive::SurfaceArchiveLimits;
use macroonz::harness::muterprater::interpret::{
    observe_mutation, observe_witness, qualify_execution, qualify_witness,
};
use macroonz::harness::muterprater::interpretation_archive::{
    AssessmentArchiveLimits, read_assessment, retain_assessment,
};
use macroonz::harness::muterprater::specimen;
use macroonz::harness::muterprater::{MutationWitness, SpecimenMaterializerBinding};
use macroonz::harness::properties::SharedSubstrate;
use macroonz::harness::report::{TrialConclusion, archive::ArchiveLimits};
use macroonz::presentation;
use serde_json::{Value, json};

const BYTES: ArchiveLimits = ArchiveLimits::declared(262_144, 131_072);
const LIMITS: AssessmentArchiveLimits = AssessmentArchiveLimits::declared(
    BYTES,
    SurfaceArchiveLimits::declared(BYTES, 8, 8),
    BindingArchiveLimits::declared(8192, 1024, 8),
    BYTES,
    128,
    128,
    8,
);
const PASS: fn(&Datum) -> TrialConclusion = |_value| TrialConclusion::Passed;

#[test]
fn assessment_display_preserves_five_roads_and_a_surviving_difference() -> Result<(), String> {
    let surface = fixture::surface()?;
    let pair = pair_for(&surface, fixture::KILLED)?;
    let selection = surface
        .selections()
        .last()
        .copied()
        .ok_or("selection missing")?;
    let input = datum(1);
    let invocation = invocation();
    let observed = mapped(observe_mutation(
        &surface,
        &pair,
        &input,
        selection,
        &invocation,
    ))?;
    let compiled = mapped(specimen::observe_mutation(
        &observed,
        &SpecimenMaterializerBinding::bound(&pair, fixture::MATERIALIZER),
        fixture::HOST,
    ))?;
    let qualified = mapped(qualify_execution(
        &compiled,
        SharedSubstrate::DeclaredIndependent,
    ))?;
    for (check, verdict, selected_conclusion) in
        [(PASS, "survived", "passed"), (odd, "killed", "refused")]
    {
        let binding = binding()?;
        let check_ref = binding.row().check();
        let witness = mapped(MutationWitness::bound(binding, check_ref, check))?;
        let reading = mapped(observe_witness(&qualified, witness, &invocation))?;
        let assessment =
            qualify_witness(reading).map_err(|rejected| format!("{:?}", rejected.cause()))?;
        let retained = mapped(retain_assessment(
            &assessment,
            &encoder("input")?,
            &encoder("meaning")?,
            LIMITS,
        ))?;
        let record = mapped(read_assessment(retained.encoded(), LIMITS))?;
        let shown = parsed(&presentation::archived_assessment(&record))?;
        inspect(&shown, verdict, selected_conclusion)?;
        let live = parsed(&presentation::mutation_record(assessment.mutation()))?;
        assert_eq!(field(&live, "/record/equivalence")?, "refuted");
        assert_eq!(field(&live, "/record/outcome/kind")?, verdict);
    }
    Ok(())
}

fn inspect(shown: &Value, verdict: &str, selected: &str) -> Result<(), String> {
    assert_eq!(field(shown, "/standing")?, "historical-unauthenticated");
    assert_eq!(field(shown, "/record/difference")?, "differs");
    assert_eq!(field(shown, "/record/mutation/equivalence")?, "refuted");
    assert_eq!(field(shown, "/record/mutation/outcome/kind")?, verdict);
    assert_eq!(field(shown, "/record/input/bytes")?, "ff01");
    assert_eq!(field(shown, "/record/baseline_content/bytes")?, "");
    assert_eq!(field(shown, "/record/selected_content/bytes")?, "00ff0d");
    assert_eq!(
        field(shown, "/record/mutation/activation/value/firings")?,
        &json!(7u32)
    );
    let roads = field(shown, "/record/roads")?
        .as_array()
        .ok_or("roads missing")?;
    assert_eq!(roads.len(), 5);
    for (road, (role, bytes, conclusion)) in roads.iter().zip([
        ("production", "ff01", "passed"),
        ("baseline-evaluation", "ff01", "passed"),
        ("compiled-baseline", "ff01", "passed"),
        ("compiled-selected", "ff02", selected),
        ("selected-evaluation", "ff02", selected),
    ]) {
        assert_eq!(field(road, "/role")?, role);
        assert_eq!(field(road, "/meaning/bytes")?, bytes);
        assert_eq!(field(road, "/report/attempt/value/kind")?, conclusion);
        assert_eq!(field(road, "/meaning/convention/name/stem")?, "meaning");
    }
    super::inspect_surface(field(shown, "/record/surface")?)?;
    for field_name in [
        "archive_address",
        "pair",
        "selection",
        "witness",
        "substrate",
    ] {
        assert!(!field(shown, &format!("/record/{field_name}"))?.is_null());
    }
    Ok(())
}
