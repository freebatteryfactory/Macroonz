//! Descriptor-native evidence, feature posture, identity, and custom-projector claims.

use super::{
    CALLER_OWNED_TRIAL_RECIPE, COMPANION_RECIPE, CallerOwnedTrials, DOOR, EVIDENCE_RECIPE,
    TARGET_UNAVAILABLE_RECIPE, bake, emitted_bytes, refusal_summary,
};
use macroonz_compiler::recipe::{
    HarnessPosture, ProjectionDisposition, ProjectorReplacement, RecipeRole,
};
use macroonz_compiler::{GeneratedTree, TextCapture};

#[test]
fn descriptor_native_evidence_uses_the_recipe_account_and_existing_carrier_roads() -> Result<(), ()>
{
    let baked = bake(EVIDENCE_RECIPE)?;
    let recipe = baked.projection().plan().content();
    for role in [
        RecipeRole::Trials,
        RecipeRole::Mutation,
        RecipeRole::Benchmarks,
        RecipeRole::Network,
        RecipeRole::Concurrency,
    ] {
        assert_eq!(
            recipe.projection_disposition(role),
            ProjectionDisposition::Generated,
            "{} did not enter the recipe projection account",
            role.name()
        );
        assert!(recipe.evidence(role).is_some());
    }
    for role in [
        RecipeRole::CompileContract,
        RecipeRole::DeclarationConformance,
    ] {
        assert_eq!(
            recipe.projection_disposition(role),
            ProjectionDisposition::NotRequested
        );
    }
    let text = baked
        .emit()
        .tokens()
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    for spelling in [
        "recipe_trials_support",
        "recipe_mutation_support",
        "recipe_bench_support",
        "pub mod recipe_network",
        "pub mod recipe_concurrency",
        "Closed",
        "Open",
        "Locked",
    ] {
        assert!(text.contains(spelling), "the bake omitted {spelling}");
    }
    assert!(text.contains("macro_rules"));
    assert!(text.contains("declared-order-permutation"));
    assert!(text.contains(":: renamed_facade :: harness"));
    Ok(())
}

#[test]
fn evidence_movement_moves_the_existing_recipe_identity_chain() -> Result<(), ()> {
    let first = bake(EVIDENCE_RECIPE)?;
    let changed = EVIDENCE_RECIPE.replace("samples = 32", "samples = 33");
    let second = bake(&changed)?;

    assert_ne!(
        first.projection().plan().identity(),
        second.projection().plan().identity()
    );
    assert_ne!(
        first.projection().closure().identity(),
        second.projection().closure().identity()
    );
    assert_ne!(
        first.projection().identity(),
        second.projection().identity()
    );
    assert_ne!(emitted_bytes(&first), emitted_bytes(&second));
    Ok(())
}

#[test]
fn target_unavailability_and_feature_unavailability_remain_distinct() -> Result<(), ()> {
    let available = TextCapture::read(TARGET_UNAVAILABLE_RECIPE).map_err(|_| ())?;
    let target_unavailable =
        macroonz_compiler::recipe::bake(available.input(), HarnessPosture::Available, &DOOR)
            .map_err(|_| ())?;
    assert_eq!(
        target_unavailable
            .projection()
            .plan()
            .content()
            .projection_disposition(RecipeRole::Trials),
        ProjectionDisposition::TargetUnavailable
    );

    let unavailable = TextCapture::read(TARGET_UNAVAILABLE_RECIPE).map_err(|_| ())?;
    let feature_unavailable =
        macroonz_compiler::recipe::bake(unavailable.input(), HarnessPosture::Unavailable, &DOOR)
            .map_err(|_| ())?;
    assert_eq!(
        feature_unavailable
            .projection()
            .plan()
            .content()
            .projection_disposition(RecipeRole::Trials),
        ProjectionDisposition::FeatureUnavailable
    );
    Ok(())
}

#[test]
fn generated_evidence_refuses_without_the_harness_before_any_projector_runs() -> Result<(), ()> {
    let read = TextCapture::read(EVIDENCE_RECIPE).map_err(|_| ())?;
    let refusal = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Unavailable, &DOOR)
        .err()
        .ok_or(())?;
    assert!(
        refusal
            .summary()
            .contains("projection `trials` requires the facade harness feature")
    );
    Ok(())
}

#[test]
fn either_harness_projection_requires_one_declared_support_address() -> Result<(), ()> {
    for role in ["compile_contract", "declaration_conformance"] {
        let source = COMPANION_RECIPE.replace("companions", role);
        let summary = refusal_summary(&source)?;
        assert!(
            summary.contains("support address"),
            "{role} did not require its support address: {summary}"
        );
    }
    Ok(())
}

#[test]
fn property_is_not_a_recipe_projection_alias() -> Result<(), ()> {
    let source = COMPANION_RECIPE.replace("companions", "property");
    let summary = refusal_summary(&source)?;
    assert!(summary.contains("a recipe projection"));
    Ok(())
}

#[test]
fn a_caller_owned_evidence_projector_uses_the_common_sink_without_standard_privilege()
-> Result<(), ()> {
    let read = TextCapture::read(CALLER_OWNED_TRIAL_RECIPE).map_err(|_| ())?;
    let standard_refusal =
        macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
            .err()
            .ok_or(())?;
    assert_eq!(standard_refusal.phase(), macroonz_compiler::Phase::Capture);

    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &[ProjectorReplacement::for_role(
            RecipeRole::Trials,
            &CallerOwnedTrials,
        )],
    )
    .map_err(|_| ())?;
    let text = custom
        .projection()
        .emit()
        .tokens()
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    assert!(text.contains("CallerOwnedTrials"));
    assert!(!text.contains("recipe_trials_support"));
    Ok(())
}
