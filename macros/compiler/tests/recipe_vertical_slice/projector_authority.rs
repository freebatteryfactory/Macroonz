//! Caller-owned projectors observed against the standard projector authority ceiling.

use super::support::{
    CODEC_RECIPE, COMPANION_RECIPE, COMPLETE_RECIPE, EVIDENCE_RECIPE, EXACT_EFFECT_RECIPE,
    MirroredCodec, MirroredCompanions, MirroredDispatch, MirroredRelationTables, MirroredTypestate,
    bake, bake_with, bake_with_refusal, emitted_bytes,
};
use macroonz_compiler::recipe::{
    PROJECTION_LIMIT, ProjectionError, ProjectionOffered, ProjectionRequest, ProjectionSink,
    ProjectorReplacement, RecipeBake, RecipeProjector, RecipeRole, RecipeView,
};
use macroonz_compiler::{CanonicalContent, GeneratedToken, GeneratedTree, unit_struct};
use std::cell::RefCell;

/// The standard bake and the caller-owned bake of one recipe, whose projection, plan, and closure identities and emitted bytes must agree.
fn same_authority(
    source: &str,
    replacements: &[ProjectorReplacement<'_>],
) -> Result<(RecipeBake, RecipeBake), ()> {
    let standard = bake(source)?;
    let custom = bake_with(source, replacements)?;
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
    Ok((standard, custom))
}

struct RecordingProjector<'projector> {
    observed: &'projector RefCell<Vec<RecipeRole>>,
    delegate: &'projector dyn RecipeProjector,
}

struct CatalogProjector<'projector> {
    observed: &'projector RefCell<Vec<RecipeRole>>,
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

impl RecipeProjector for CatalogProjector<'_> {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        let role = request.role();
        assert_eq!(request.effective().role(), role);
        assert!(!view.recipe().module_name().is_empty());
        self.observed.borrow_mut().push(role);
        let name = match role {
            RecipeRole::Companions => "CatalogCompanions",
            RecipeRole::RelationTables => "CatalogRelationTables",
            RecipeRole::Dispatch => "CatalogDispatch",
            RecipeRole::CompileContract => "CatalogCompileContract",
            RecipeRole::DeclarationConformance => "CatalogDeclarationConformance",
            RecipeRole::Typestate => "CatalogTypestate",
            RecipeRole::Trials => "CatalogTrials",
            RecipeRole::Mutation => "CatalogMutation",
            RecipeRole::Benchmarks => "CatalogBenchmarks",
            RecipeRole::Network => "CatalogNetwork",
            RecipeRole::Concurrency => "CatalogConcurrency",
            RecipeRole::Codec => "CatalogCodec",
            _ => "CatalogFutureRole",
        };
        sink.offer(GeneratedTree::assembled(unit_struct(
            GeneratedToken::word(name),
            Vec::new(),
            Vec::new(),
        ))?)
    }
}

#[test]
fn a_caller_owned_projector_has_the_standard_clients_exact_authority() -> Result<(), ()> {
    let (standard, custom) = same_authority(
        COMPANION_RECIPE,
        &[ProjectorReplacement::for_role(
            RecipeRole::Companions,
            &MirroredCompanions,
        )],
    )?;
    assert_eq!(
        standard.projection().explain().identity(),
        custom.projection().explain().identity()
    );
    Ok(())
}

#[test]
fn a_caller_owned_dispatch_projector_uses_the_same_behavior_kernel_and_authority() -> Result<(), ()>
{
    let source = COMPANION_RECIPE.replace("companions;", "dispatch(apply);");
    let (standard, custom) = same_authority(
        &source,
        &[ProjectorReplacement::for_role(
            RecipeRole::Dispatch,
            &MirroredDispatch,
        )],
    )?;
    assert_eq!(
        standard.projection().explain().identity(),
        custom.projection().explain().identity()
    );
    Ok(())
}

#[test]
fn exact_row_effects_give_standard_and_caller_owned_dispatch_equal_authority() -> Result<(), ()> {
    same_authority(
        EXACT_EFFECT_RECIPE,
        &[ProjectorReplacement::for_role(
            RecipeRole::Dispatch,
            &MirroredDispatch,
        )],
    )
    .map(|_| ())
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
    same_authority(
        source,
        &[ProjectorReplacement::for_role(
            RecipeRole::RelationTables,
            &MirroredRelationTables,
        )],
    )
    .map(|_| ())
}

#[test]
fn a_caller_owned_typestate_projector_uses_the_same_item_kernel_and_authority() -> Result<(), ()> {
    let source = COMPANION_RECIPE.replace("companions;", "typestate(State);");
    same_authority(
        &source,
        &[ProjectorReplacement::for_role(
            RecipeRole::Typestate,
            &MirroredTypestate,
        )],
    )
    .map(|_| ())
}

#[test]
fn a_caller_owned_codec_projector_uses_the_existing_codec_owner_and_same_authority()
-> Result<(), ()> {
    same_authority(
        CODEC_RECIPE,
        &[ProjectorReplacement::for_role(
            RecipeRole::Codec,
            &MirroredCodec,
        )],
    )
    .map(|_| ())
}

#[test]
fn every_catalog_role_accepts_the_same_caller_owned_projection_capability() -> Result<(), ()> {
    let observed = RefCell::new(Vec::new());
    let projector = CatalogProjector {
        observed: &observed,
    };
    observe_catalog_roles(
        COMPLETE_RECIPE,
        &[
            RecipeRole::Companions,
            RecipeRole::Dispatch,
            RecipeRole::CompileContract,
            RecipeRole::DeclarationConformance,
            RecipeRole::Typestate,
        ],
        &projector,
    )?;
    observe_catalog_roles(
        EVIDENCE_RECIPE,
        &[
            RecipeRole::Companions,
            RecipeRole::Trials,
            RecipeRole::Mutation,
            RecipeRole::Benchmarks,
            RecipeRole::Network,
            RecipeRole::Concurrency,
        ],
        &projector,
    )?;
    observe_catalog_roles(CODEC_RECIPE, &[RecipeRole::Codec], &projector)?;
    observe_catalog_roles(
        r"
pub mod door {
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
",
        &[RecipeRole::RelationTables],
        &projector,
    )?;

    let observed = observed.into_inner();
    for role in RecipeRole::ALL {
        assert!(
            observed.contains(role),
            "{} did not accept the common caller-owned projection capability",
            role.name()
        );
    }
    Ok(())
}

fn observe_catalog_roles(
    source: &str,
    roles: &[RecipeRole],
    projector: &dyn RecipeProjector,
) -> Result<(), ()> {
    let replacements = roles
        .iter()
        .copied()
        .map(|role| ProjectorReplacement::for_role(role, projector))
        .collect::<Vec<_>>();
    bake_with(source, replacements.as_slice()).map(|_| ())
}

#[test]
fn a_caller_owned_projector_cannot_replace_an_unselected_role() -> Result<(), ()> {
    let refusal = bake_with_refusal(
        COMPANION_RECIPE,
        &[ProjectorReplacement::for_role(
            RecipeRole::Dispatch,
            &MirroredCompanions,
        )],
    )?;
    assert!(refusal.summary().contains("unselected role `dispatch`"));
    Ok(())
}

#[test]
fn several_caller_owned_projectors_share_one_role_order_and_authority_ceiling() -> Result<(), ()> {
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
    let (standard, custom) = same_authority(COMPLETE_RECIPE, &replacements)?;

    assert_eq!(
        observed.borrow().as_slice(),
        [
            RecipeRole::Companions,
            RecipeRole::Dispatch,
            RecipeRole::Typestate,
        ]
    );
    assert_eq!(
        standard.projection().explain().identity(),
        custom.projection().explain().identity()
    );
    Ok(())
}

#[test]
fn caller_owned_projector_rosters_refuse_duplicate_and_unbounded_roles_before_projection()
-> Result<(), ()> {
    let observed = RefCell::new(Vec::new());
    let projector = RecordingProjector {
        observed: &observed,
        delegate: &MirroredCompanions,
    };
    let replacement = ProjectorReplacement::for_role(RecipeRole::Companions, &projector);
    let repeated = bake_with_refusal(COMPANION_RECIPE, &[replacement, replacement])?;
    assert!(
        repeated
            .summary()
            .contains("caller-owned projector role `companions` is replaced more than once")
    );
    assert!(observed.borrow().is_empty());

    for replacements in [
        vec![replacement; PROJECTION_LIMIT - 1],
        vec![replacement; PROJECTION_LIMIT],
    ] {
        let refusal = bake_with_refusal(COMPANION_RECIPE, replacements.as_slice())?;
        assert!(
            refusal
                .summary()
                .contains("caller-owned projector role `companions` is replaced more than once")
        );
    }
    assert!(observed.borrow().is_empty());

    let unbounded = [replacement; PROJECTION_LIMIT + 1];
    let unbounded_refusal = bake_with_refusal(COMPANION_RECIPE, &unbounded)?;
    assert!(
        unbounded_refusal
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
