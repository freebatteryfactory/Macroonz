//! The caller's nonzero predicate and its explicitly permitted comparison mutation.

use super::types::Sample;
use macroonz::harness::descriptor::{ClaimRef, MutationPointRef};
use macroonz::harness::muterprater::{
    ActivationSite, AlternativeDeclaration, DiscoveredMutationSite, EvaluationCallRefusal,
    EvaluationDirective, EvaluationFamilyRef, EvaluationObservation, MutationPermission,
    MutationPolicy, MutationSurfaceLowering, OperatorFamilyRef, OwnerClaimMapping,
    SpecimenMaterializerRefusal, discover,
};
use macroonz::harness::properties::Agreement;

const ORIGINAL: &[u8] = b"value != 0";
const SELECTED: &[u8] = b"value == 0";

pub(super) fn claim() -> Result<ClaimRef, String> {
    ClaimRef::named("nonzero", "positive-input-is-accepted").map_err(super::debug)
}

pub(super) fn point() -> Result<MutationPointRef, String> {
    MutationPointRef::named("nonzero", "comparison").map_err(super::debug)
}

pub(super) fn surface() -> Result<MutationSurfaceLowering, String> {
    let family = EvaluationFamilyRef::named("nonzero", "predicate").map_err(super::debug)?;
    let operator = OperatorFamilyRef::of_slug("comparison-boundaries")
        .ok_or("comparison-boundaries is not an operator family")?;
    let permission =
        MutationPermission::declared(claim()?, vec![operator]).map_err(super::debug)?;
    let policy = MutationPolicy::declared(family, vec![permission]).map_err(super::debug)?;
    let site = DiscoveredMutationSite::discovered(
        point()?,
        OwnerClaimMapping::Mapped(claim()?),
        ORIGINAL.to_vec(),
        vec![AlternativeDeclaration::stated(operator, SELECTED.to_vec())],
        ActivationSite::named("nonzero", "comparison-executed").map_err(super::debug)?,
    )
    .map_err(super::debug)?;
    discover::lower_discoveries(&policy, vec![site]).map_err(super::debug)
}

pub(super) fn production(sample: &Sample<'_>) -> u32 {
    predicate(sample.value)
}

pub(super) fn predicate(value: u32) -> u32 {
    u32::from(value != 0)
}

pub(super) fn evaluation(
    sample: &Sample<'_>,
    directive: EvaluationDirective<'_>,
) -> Result<EvaluationObservation<u32>, EvaluationCallRefusal> {
    sample
        .evaluations
        .set(sample.evaluations.get().saturating_add(1));
    let Some(selected) = directive.resolved() else {
        return Ok(EvaluationObservation::observed(production(sample), 0));
    };
    if selected.point().original_operation() != ORIGINAL
        || selected.alternative().operation() != SELECTED
    {
        return Err(EvaluationCallRefusal::ActiveSelectionNotImplemented(
            selected.selection(),
        ));
    }
    Ok(EvaluationObservation::observed(
        u32::from(sample.value == 0),
        1,
    ))
}

pub(super) fn same(left: u32, right: u32) -> Agreement {
    if left == right {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
}

pub(super) fn materialize(
    directive: EvaluationDirective<'_>,
) -> Result<Vec<u8>, SpecimenMaterializerRefusal> {
    let predicate = match directive.resolved() {
        None => "value != 0",
        Some(selected)
            if selected.point().original_operation() == ORIGINAL
                && selected.alternative().operation() == SELECTED =>
        {
            "value == 0"
        }
        Some(selected) => {
            return Err(SpecimenMaterializerRefusal::ActiveSelectionNotImplemented(
                selected.selection(),
            ));
        }
    };
    Ok(format!(
        r#"use std::io::{{Read, Write}};
fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let mut input = String::new();
    std::io::stdin().lock().take(12).read_to_string(&mut input)?;
    let value = input.trim().parse::<u32>()?;
    let observed = u32::from({predicate});
    writeln!(std::io::stdout(), "{{observed}}")?;
    Ok(())
}}
"#
    )
    .into_bytes())
}
