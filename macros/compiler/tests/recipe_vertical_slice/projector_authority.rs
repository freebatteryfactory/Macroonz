//! Caller-owned projectors observed against the standard projector authority ceiling.

use super::support::{
    COMPANION_RECIPE, DOOR, MirroredCompanions, MirroredDispatch, MirroredTypestate, bake,
    emitted_bytes,
};
use macroonz_compiler::recipe::{HarnessPosture, RecipeRole};
use macroonz_compiler::{CanonicalContent, TextCapture};

#[test]
fn a_caller_owned_projector_has_the_standard_clients_exact_authority() -> Result<(), ()> {
    let read = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Companions,
        &MirroredCompanions,
    )
    .map_err(|_| ())?;

    assert_eq!(
        standard.projection().identity(),
        custom.projection().identity()
    );
    assert_eq!(
        standard.projection().plan().identity(),
        custom.projection().plan().identity()
    );
    assert_eq!(
        standard.projection().closure().identity(),
        custom.projection().closure().identity()
    );
    assert_eq!(
        standard.projection().explain().identity(),
        custom.projection().explain().identity()
    );
    assert_eq!(emitted_bytes(&standard), emitted_bytes(&custom));
    Ok(())
}

#[test]
fn a_caller_owned_dispatch_projector_uses_the_same_behavior_kernel_and_authority() -> Result<(), ()>
{
    let source = COMPANION_RECIPE.replace("companions;", "dispatch(apply);");
    let read = TextCapture::read(&source).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Dispatch,
        &MirroredDispatch,
    )
    .map_err(|_| ())?;

    assert_eq!(
        standard.projection().identity(),
        custom.projection().identity()
    );
    assert_eq!(
        standard.projection().plan().identity(),
        custom.projection().plan().identity()
    );
    assert_eq!(
        standard.projection().closure().identity(),
        custom.projection().closure().identity()
    );
    assert_eq!(
        standard.projection().explain().identity(),
        custom.projection().explain().identity()
    );
    assert_eq!(emitted_bytes(&standard), emitted_bytes(&custom));
    Ok(())
}

#[test]
fn a_caller_owned_typestate_projector_uses_the_same_item_kernel_and_authority() -> Result<(), ()> {
    let source = COMPANION_RECIPE.replace("companions;", "typestate;");
    let read = TextCapture::read(&source).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Typestate,
        &MirroredTypestate,
    )
    .map_err(|_| ())?;

    assert_eq!(
        standard.projection().identity(),
        custom.projection().identity()
    );
    assert_eq!(
        standard.projection().closure().identity(),
        custom.projection().closure().identity()
    );
    assert_eq!(emitted_bytes(&standard), emitted_bytes(&custom));
    Ok(())
}

#[test]
fn a_caller_owned_projector_cannot_replace_an_unselected_role() -> Result<(), ()> {
    let read = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let refusal = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        RecipeRole::Dispatch,
        &MirroredCompanions,
    )
    .err()
    .ok_or(())?;
    assert!(refusal.summary().contains("unselected role `dispatch`"));
    Ok(())
}

#[test]
fn semantic_effect_movement_moves_identity_even_when_rendered_companions_do_not() -> Result<(), ()>
{
    let first = bake(COMPANION_RECIPE)?;
    let changed = COMPANION_RECIPE.replace("crate::effects::open", "crate::effects::unlock");
    let second = bake(&changed)?;

    assert_eq!(emitted_bytes(&first), emitted_bytes(&second));
    assert_ne!(
        first
            .projection()
            .plan()
            .content()
            .canonical_content_bytes(),
        second
            .projection()
            .plan()
            .content()
            .canonical_content_bytes()
    );
    assert_ne!(
        first.projection().plan().identity(),
        second.projection().plan().identity()
    );
    assert_ne!(
        first.projection().identity(),
        second.projection().identity()
    );
    Ok(())
}
