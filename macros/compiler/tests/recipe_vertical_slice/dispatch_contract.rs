//! Dispatch disclosure, exact-Rust custody, identity, and refusal claims.

use super::{
    COMPANION_RECIPE, DOOR, EXACT_DISPATCH_RECIPE, EXACT_EFFECT_RECIPE, bake, emitted_bytes,
    refusal_summary,
};
use macroonz_compiler::recipe::{
    HarnessPosture, LoweringSource, RecipeRelationPayload, RecipeRole, RecipeTransitionEffect,
};
use macroonz_compiler::{GeneratedTree, TextCapture};

#[test]
fn dispatch_discloses_preset_configuration_and_exact_rust_on_one_seat() -> Result<(), ()> {
    let preset_source = COMPANION_RECIPE.replace("companions;", "dispatch;");
    let preset = bake(&preset_source)?;
    let preset_effective = preset
        .projection()
        .plan()
        .content()
        .effective(RecipeRole::Dispatch)
        .ok_or(())?;
    assert_eq!(preset_effective.source(), LoweringSource::Preset);
    assert_eq!(preset_effective.name(), None);
    assert_eq!(preset_effective.exact_rust(), None);

    let configured_source = COMPANION_RECIPE.replace("companions;", "dispatch(apply);");
    let configured = bake(&configured_source)?;
    let configured_effective = configured
        .projection()
        .plan()
        .content()
        .effective(RecipeRole::Dispatch)
        .ok_or(())?;
    assert_eq!(configured_effective.source(), LoweringSource::Configuration);
    assert_eq!(configured_effective.name(), Some("apply"));
    assert_eq!(configured_effective.exact_rust(), None);

    let exact = bake(EXACT_DISPATCH_RECIPE)?;
    let exact_effective = exact
        .projection()
        .plan()
        .content()
        .effective(RecipeRole::Dispatch)
        .ok_or(())?;
    assert_eq!(exact_effective.source(), LoweringSource::ExactRust);
    assert_eq!(exact_effective.name(), Some("advance"));
    let exact_readback = exact_effective
        .exact_rust()
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    for fragment in [
        "# [ inline ]",
        "pub fn advance < 'a >",
        "current : State",
        "stimulus : Event",
        "where State : 'a",
    ] {
        assert!(
            exact_readback.contains(fragment),
            "the exact readback omitted {fragment}: {exact_readback}"
        );
    }
    let emitted = exact
        .emit()
        .tokens()
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    assert!(emitted.contains("use super :: State"));
    assert!(emitted.contains("use super :: Event"));
    assert!(
        emitted.contains("match ( current , stimulus )"),
        "the generated body did not use the exact bindings: {emitted}"
    );
    assert!(emitted.contains("pub fn advance < 'a >"));
    Ok(())
}

#[test]
fn exact_dispatch_signature_material_moves_recipe_identity() -> Result<(), ()> {
    let first = bake(EXACT_DISPATCH_RECIPE)?;
    let changed = EXACT_DISPATCH_RECIPE.replace("current: State", "source: State");
    let second = bake(&changed)?;

    assert_ne!(
        first.projection().plan().identity(),
        second.projection().plan().identity()
    );
    assert_ne!(
        first.projection().identity(),
        second.projection().identity()
    );
    assert_ne!(emitted_bytes(&first), emitted_bytes(&second));
    Ok(())
}

#[test]
fn exact_row_rust_and_selected_dispatch_bindings_share_one_account() -> Result<(), ()> {
    let baked = bake(EXACT_EFFECT_RECIPE)?;
    let recipe = baked.projection().plan().content();
    let effective = recipe.effective(RecipeRole::Dispatch).ok_or(())?;
    assert_eq!(effective.dispatch_bindings(), Some(["current", "event"]));
    let relation = recipe.transition_relation().ok_or(())?;
    let row = relation.rows().next().ok_or(())?;
    let RecipeRelationPayload::Transition { effect, .. } = row.payload() else {
        return Err(());
    };
    let RecipeTransitionEffect::ExactRust {
        target_binding,
        body,
    } = effect
    else {
        return Err(());
    };
    assert_eq!(
        target_binding,
        &macroonz_compiler::GeneratedToken::word("target")
    );
    let body = body.inspected();
    assert!(body.contains("context . calls"), "{body}");
    assert!(body.contains("Ok ( target )"), "{body}");

    let emitted = baked
        .emit()
        .tokens()
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    assert!(
        emitted.contains("let target = super :: State :: Open"),
        "{emitted}"
    );
    assert!(emitted.contains("match ( current , event )"), "{emitted}");
    assert!(
        emitted.contains("context : & mut super :: Context"),
        "{emitted}"
    );
    Ok(())
}

#[test]
fn selected_dispatch_binding_and_exact_row_body_move_identity() -> Result<(), ()> {
    let first = bake(EXACT_EFFECT_RECIPE)?;
    let binding =
        EXACT_EFFECT_RECIPE.replace("dispatch(current, event)", "dispatch(renamed, event)");
    assert!(
        refusal_summary(&binding)?.contains(
            "exact dispatch selector `renamed` does not name one simple parameter binding"
        )
    );
    let body = EXACT_EFFECT_RECIPE.replace("Ok(target)", "Ok({ let _ = target; current })");
    let second = bake(&body)?;
    assert_ne!(
        first.projection().plan().identity(),
        second.projection().plan().identity()
    );
    assert_ne!(emitted_bytes(&first), emitted_bytes(&second));
    Ok(())
}

#[test]
fn selected_dispatch_bindings_are_distinct_and_exact_target_bindings_are_local() -> Result<(), ()> {
    let repeated =
        EXACT_EFFECT_RECIPE.replace("dispatch(current, event)", "dispatch(current, current)");
    assert!(
        refusal_summary(&repeated)?.contains("two distinct dispatch bindings"),
        "the same parameter cannot own both structural coordinates"
    );

    let path_binding = EXACT_EFFECT_RECIPE.replace("with(target) {", "with(crate::target) {");
    assert!(
        refusal_summary(&path_binding)?.contains("one declared-target binding"),
        "an exact effect body must receive one local target binding"
    );
    Ok(())
}

#[test]
fn commas_inside_exact_parameter_types_do_not_invent_parameter_rows() -> Result<(), ()> {
    let nested_type = EXACT_DISPATCH_RECIPE.replace(
        "current: State",
        "current: core::result::Result<State, TransitionRefusal>",
    );
    let baked = bake(&nested_type)?;
    let exact = baked
        .projection()
        .plan()
        .content()
        .effective(RecipeRole::Dispatch)
        .and_then(|effective| effective.exact_rust())
        .map(GeneratedTree::inspected)
        .ok_or(())?;
    assert!(exact.contains("Result < State , TransitionRefusal >"));
    Ok(())
}

#[test]
fn fully_qualified_exact_types_do_not_emit_unneeded_vocabulary_imports() -> Result<(), ()> {
    let qualified = EXACT_DISPATCH_RECIPE
        .replace("current: State", "current: crate::door::State")
        .replace("stimulus: Event", "stimulus: crate::door::Event")
        .replace(
            "Result<State, TransitionRefusal>",
            "Result<crate::door::State, TransitionRefusal>",
        )
        .replace("State: 'a", "crate::door::State: 'a");
    let baked = bake(&qualified)?;
    let emitted = baked
        .emit()
        .tokens()
        .map(GeneratedTree::inspected)
        .ok_or(())?;

    assert!(!emitted.contains("use super :: State"), "{emitted}");
    assert!(!emitted.contains("use super :: Event"), "{emitted}");
    Ok(())
}

#[test]
fn exact_dispatch_refusals_name_the_owned_repair() -> Result<(), ()> {
    let not_function = EXACT_DISPATCH_RECIPE.replace(
        "#[inline]\n                pub fn advance<'a>(\n                    current: State,\n                    stimulus: Event,\n                ) -> Result<State, TransitionRefusal>\n                where\n                    State: 'a;",
        "pub const ADVANCE: usize = 1;",
    );
    assert!(refusal_summary(&not_function)?.contains(
        "exact dispatch braces must contain one semicolon-terminated Rust function signature"
    ));

    let with_body = EXACT_DISPATCH_RECIPE.replace("State: 'a;", "State: 'a { unreachable!() }");
    assert!(
        refusal_summary(&with_body)?.contains("exact dispatch cannot carry a caller-authored body")
    );

    let one_parameter = EXACT_DISPATCH_RECIPE.replace(
        "                    current: State,\n                    stimulus: Event,",
        "                    current: State,",
    );
    assert!(
        refusal_summary(&one_parameter)?
            .contains("exact dispatch requires two parameters but the signature states 1")
    );

    let missing_type = EXACT_DISPATCH_RECIPE.replace("current: State", "current:");
    assert!(
        refusal_summary(&missing_type)?
            .contains("exact dispatch parameter 1 must use one simple identifier binding")
    );

    let pattern = EXACT_DISPATCH_RECIPE.replace("current: State", "(current, _): (State, State)");
    let read = TextCapture::read(&pattern).map_err(|_| ())?;
    let refusal = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .err()
        .ok_or(())?;
    assert!(
        refusal
            .summary()
            .contains("exact dispatch parameter 1 must use one simple identifier binding")
    );
    let repair = refusal.repairs().first().ok_or(())?;
    assert!(
        repair
            .description
            .shown()
            .contains("write `dispatch { fn apply")
    );
    Ok(())
}
