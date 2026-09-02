//! Caller-owned projectors observed against the standard projector authority ceiling.

use super::support::{
    CODEC_RECIPE, COMPANION_RECIPE, COMPLETE_RECIPE, DOOR, EXACT_EFFECT_RECIPE, MirroredCodec,
    MirroredCompanions, MirroredDispatch, MirroredRelationTables, MirroredTypestate, bake,
    emitted_bytes,
};
use macroonz_compiler::recipe::{
    HarnessPosture, PROJECTION_LIMIT, ProjectionError, ProjectionOffered, ProjectionRequest,
    ProjectionSink, ProjectorReplacement, RecipeProjector, RecipeRole, RecipeView,
};
use macroonz_compiler::{CanonicalContent, TextCapture};
use std::cell::RefCell;

struct RecordingProjector<'projector> {
    observed: &'projector RefCell<Vec<RecipeRole>>,
    delegate: &'projector dyn RecipeProjector,
}

impl RecipeProjector for RecordingProjector<'_> {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        self.observed.borrow_mut().push(request.role());
        self.delegate.project(view, request, sink)
    }
}

#[test]
fn a_caller_owned_projector_has_the_standard_clients_exact_authority() -> Result<(), ()> {
    let read = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &[ProjectorReplacement::for_role(
            RecipeRole::Companions,
            &MirroredCompanions,
        )],
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
        &[ProjectorReplacement::for_role(
            RecipeRole::Dispatch,
            &MirroredDispatch,
        )],
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
fn exact_row_effects_give_standard_and_caller_owned_dispatch_equal_authority() -> Result<(), ()> {
    let read = TextCapture::read(EXACT_EFFECT_RECIPE).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &[ProjectorReplacement::for_role(
            RecipeRole::Dispatch,
            &MirroredDispatch,
        )],
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
    assert_eq!(emitted_bytes(&standard), emitted_bytes(&custom));
    Ok(())
}

#[test]
fn a_caller_owned_relation_table_uses_the_same_account_and_authority() -> Result<(), ()> {
    let source = r"
pub mod graph {
    pub enum Stage { Draft, Published }

    bake! {
        vocabularies { Stage; };
        relations {
            evolution(Stage, Stage) {
                (Draft, Published);
            };
        };
        projections {
            relation_tables { evolution; };
        };
    }
}
";
    let read = TextCapture::read(source).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &[ProjectorReplacement::for_role(
            RecipeRole::RelationTables,
            &MirroredRelationTables,
        )],
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
    assert_eq!(emitted_bytes(&standard), emitted_bytes(&custom));
    Ok(())
}

#[test]
fn a_caller_owned_typestate_projector_uses_the_same_item_kernel_and_authority() -> Result<(), ()> {
    let source = COMPANION_RECIPE.replace("companions;", "typestate(State);");
    let read = TextCapture::read(&source).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &[ProjectorReplacement::for_role(
            RecipeRole::Typestate,
            &MirroredTypestate,
        )],
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
fn a_caller_owned_codec_projector_uses_the_existing_codec_owner_and_same_authority()
-> Result<(), ()> {
    let read = TextCapture::read(CODEC_RECIPE).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &[ProjectorReplacement::for_role(
            RecipeRole::Codec,
            &MirroredCodec,
        )],
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
        &[ProjectorReplacement::for_role(
            RecipeRole::Dispatch,
            &MirroredCompanions,
        )],
    )
    .err()
    .ok_or(())?;
    assert!(refusal.summary().contains("unselected role `dispatch`"));
    Ok(())
}

#[test]
fn several_caller_owned_projectors_share_one_role_order_and_authority_ceiling() -> Result<(), ()> {
    let read = TextCapture::read(COMPLETE_RECIPE).map_err(|_| ())?;
    let standard = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let observed = RefCell::new(Vec::new());
    let typestate = RecordingProjector {
        observed: &observed,
        delegate: &MirroredTypestate,
    };
    let companions = RecordingProjector {
        observed: &observed,
        delegate: &MirroredCompanions,
    };
    let dispatch = RecordingProjector {
        observed: &observed,
        delegate: &MirroredDispatch,
    };
    let replacements = [
        ProjectorReplacement::for_role(RecipeRole::Typestate, &typestate),
        ProjectorReplacement::for_role(RecipeRole::Companions, &companions),
        ProjectorReplacement::for_role(RecipeRole::Dispatch, &dispatch),
    ];
    let custom = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &replacements,
    )
    .map_err(|_| ())?;

    assert_eq!(
        observed.borrow().as_slice(),
        [
            RecipeRole::Companions,
            RecipeRole::Dispatch,
            RecipeRole::Typestate,
        ]
    );
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
fn caller_owned_projector_rosters_refuse_duplicate_and_unbounded_roles_before_projection()
-> Result<(), ()> {
    let read = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let observed = RefCell::new(Vec::new());
    let projector = RecordingProjector {
        observed: &observed,
        delegate: &MirroredCompanions,
    };
    let replacement = ProjectorReplacement::for_role(RecipeRole::Companions, &projector);
    let repeated = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &[replacement, replacement],
    )
    .err()
    .ok_or(())?;
    assert!(
        repeated
            .summary()
            .contains("caller-owned projector role `companions` is replaced more than once")
    );
    assert!(observed.borrow().is_empty());

    let at_limit = [replacement; PROJECTION_LIMIT];
    let refusal = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &at_limit,
    )
    .err()
    .ok_or(())?;
    assert!(
        refusal
            .summary()
            .contains("caller-owned projector role `companions` is replaced more than once")
    );
    assert!(observed.borrow().is_empty());

    let unbounded = [replacement; PROJECTION_LIMIT + 1];
    let refusal = macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &unbounded,
    )
    .err()
    .ok_or(())?;
    assert!(
        refusal
            .summary()
            .contains("caller-owned projectors were supplied where at most 12 fit")
    );
    assert!(observed.borrow().is_empty());
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
