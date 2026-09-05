//! Stable recipe readback observed through public and caller-owned projection roads.

use super::support::{CODEC_RECIPE, bake_under, bake_with};
use super::{COMPANION_RECIPE, COMPLETE_RECIPE, EVIDENCE_RECIPE, bake};
#[cfg(feature = "host")]
use macroonz_compiler::host::Emittable;
use macroonz_compiler::recipe::{
    HarnessPosture, LoweringSource, ProjectionDisposition, ProjectionError, ProjectionOffered,
    ProjectionRequest, ProjectionSink, ProjectorReplacement, RecipeProjector,
    RecipeRelationPayload, RecipeRelationPayloadKind, RecipeRole, RecipeView,
};
use macroonz_compiler::{
    AbsencePosture, CompletenessPosture, CyclePosture, DensityPosture, Destination, EmptyPosture,
    GeneratedToken, GeneratedTree, MembershipPosture, RenderError, RepetitionPosture, Role,
    SelfRelationPosture, unit_struct,
};

const POSTURE_RECIPE: &str = r"
pub mod structure {
    pub enum Left { Only }
    pub enum Right { Only }
    pub enum Stage { Draft, Published }

    bake! {
        vocabularies { Left; Right; Stage; };
        relations {
            complete(Left, Right) {
                (Only, Only);
            };
            line(Stage, Stage) {
                (Draft, Published);
            };
            policy(Left, Right) {
                (Only, Only) with(crate::policy::allow);
            };
        };
        postures {
            complete {
                empty(refused);
                repetition(allowed);
                membership(open, open);
                completeness(total, total);
                density(dense);
                absence(refused);
            };
            line {
                self_relation(refused);
                cycle(refused);
            };
            policy {
                repetition(refused);
            };
        };
        projections {
            relation_tables { complete; };
        };
    }
}
";

const ALL_STANDARD_PROJECTIONS_RECIPE: &str = r"
pub mod catalog {
    pub enum State { Closed, Open }
    pub enum Event { OpenDoor }
    pub enum Capability { Read }
    pub struct Ledger { pub value: u16 }

    bake! {
        vocabularies { State; Event; Capability; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(crate::open);
        };
        relations {
            policy(State, Capability) {
                (Closed, Read);
            };
        };
        absence(refused);
        codecs {
            ledger(Ledger) {
                direction(encode);
                refusal(LedgerError);
                assembly(assembled, total);
                members { value: u16 => count(required); };
            };
        };
        projections {
            companions;
            relation_tables { policy; };
            dispatch(apply);
            compile_contract;
            declaration_conformance;
            typestate(State);
            codec;
        };
        support(recipe_support);
    }
}
";

struct ReadbackProjector;

impl RecipeProjector for ReadbackProjector {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<ProjectionOffered, ProjectionError> {
        assert_eq!(request.role(), RecipeRole::RelationTables);
        assert_eq!(view.recipe().module_name(), "structure");

        let complete = view
            .recipe()
            .relation("complete")
            .ok_or(ProjectionError::Render(RenderError::NothingRendered))?;
        assert_eq!(complete.row_count(), 1);
        let complete_requirements = complete.requirements();
        assert_eq!(complete_requirements.empty(), Some(EmptyPosture::Refusal));
        assert_eq!(
            complete_requirements.repetition(),
            Some(RepetitionPosture::Allowed)
        );
        assert_eq!(
            complete_requirements.left_membership(),
            Some(MembershipPosture::Open)
        );
        assert_eq!(
            complete_requirements.right_membership(),
            Some(MembershipPosture::Open)
        );
        assert_eq!(
            complete_requirements.left_completeness(),
            Some(CompletenessPosture::Total)
        );
        assert_eq!(
            complete_requirements.right_completeness(),
            Some(CompletenessPosture::Total)
        );
        assert_eq!(complete_requirements.density(), Some(DensityPosture::Dense));
        assert_eq!(
            complete_requirements.absence(),
            Some(AbsencePosture::Refusal)
        );

        let line = view
            .recipe()
            .relation("line")
            .ok_or(ProjectionError::Render(RenderError::NothingRendered))?;
        assert_eq!(
            line.requirements().self_relation(),
            Some(SelfRelationPosture::Refusal)
        );
        assert_eq!(line.requirements().cycle(), Some(CyclePosture::Refusal));

        let policy = view
            .recipe()
            .relation("policy")
            .ok_or(ProjectionError::Render(RenderError::NothingRendered))?;
        assert_eq!(
            policy.requirements().repetition(),
            Some(RepetitionPosture::Refusal)
        );
        let Some(row) = policy.rows().next() else {
            return Err(ProjectionError::Render(RenderError::NothingRendered));
        };
        let RecipeRelationPayload::Path(path) = row.payload() else {
            return Err(ProjectionError::Render(RenderError::NothingRendered));
        };
        assert_eq!(path.inspected().trim(), "crate :: policy :: allow");

        sink.offer(GeneratedTree::assembled(unit_struct(
            GeneratedToken::word("ReadbackObserved"),
            Vec::new(),
            Vec::new(),
        ))?)
    }
}

#[test]
fn public_recipe_names_are_exact_and_stable() {
    assert_eq!(HarnessPosture::Available.name(), "available");
    assert_eq!(HarnessPosture::Unavailable.name(), "unavailable");
    assert_eq!(LoweringSource::Preset.name(), "preset");
    assert_eq!(LoweringSource::Configuration.name(), "configuration");
    assert_eq!(LoweringSource::ExactRust.name(), "exact-rust");
    assert_eq!(RecipeRelationPayloadKind::Unlabeled.name(), "unlabeled");
    assert_eq!(RecipeRelationPayloadKind::Path.name(), "path");
    assert_eq!(RecipeRelationPayloadKind::ExactRust.name(), "exact-rust");
    assert_eq!(RecipeRelationPayloadKind::Transition.name(), "transition");
    assert_eq!(ProjectionDisposition::Generated.name(), "generated");
    assert_eq!(ProjectionDisposition::NotRequested.name(), "not-requested");
    assert_eq!(
        ProjectionDisposition::FeatureUnavailable.name(),
        "feature-unavailable"
    );
    assert_eq!(
        ProjectionDisposition::TargetUnavailable.name(),
        "target-unavailable"
    );

    let names = RecipeRole::ALL
        .iter()
        .copied()
        .map(<RecipeRole as Role>::name)
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            "companions",
            "relation-tables",
            "dispatch",
            "compile-contract",
            "declaration-conformance",
            "typestate",
            "trials",
            "mutation",
            "benchmarks",
            "network",
            "concurrency",
            "codec",
        ]
    );

    let destinations = RecipeRole::ALL
        .iter()
        .copied()
        .map(Role::destination)
        .collect::<Vec<_>>();
    assert_eq!(
        destinations,
        [
            Destination::DeclarationSite,
            Destination::DeclarationSite,
            Destination::DeclarationSite,
            Destination::TestCarrier,
            Destination::TestCarrier,
            Destination::DeclarationSite,
            Destination::DeclarationSite,
            Destination::DeclarationSite,
            Destination::DeclarationSite,
            Destination::DeclarationSite,
            Destination::DeclarationSite,
            Destination::DeclarationSite,
        ]
    );
}

#[test]
fn every_declared_role_reaches_one_selected_recipe_account() -> Result<(), ()> {
    let recipes = vec![
        bake(COMPLETE_RECIPE)?,
        bake(EVIDENCE_RECIPE)?,
        bake(CODEC_RECIPE)?,
        bake(POSTURE_RECIPE)?,
    ]
    .into_boxed_slice();
    for role in RecipeRole::ALL.iter().copied() {
        assert!(recipes.iter().any(|baked| {
            baked
                .projection()
                .plan()
                .content()
                .projection_disposition(role)
                == ProjectionDisposition::Generated
        }));
    }
    Ok(())
}

#[test]
fn every_standard_projection_fits_one_recipe_selection() -> Result<(), ()> {
    let baked = bake(ALL_STANDARD_PROJECTIONS_RECIPE)?;
    let recipe = baked.projection().plan().content();
    let selected = [
        RecipeRole::Companions,
        RecipeRole::RelationTables,
        RecipeRole::Dispatch,
        RecipeRole::CompileContract,
        RecipeRole::DeclarationConformance,
        RecipeRole::Typestate,
        RecipeRole::Codec,
    ];
    for role in selected {
        assert_eq!(
            recipe.projection_disposition(role),
            ProjectionDisposition::Generated
        );
    }
    assert_eq!(
        baked.projection().closure().rendered().count(),
        selected.len()
    );
    Ok(())
}

#[test]
fn generated_roles_retain_their_declared_root_and_baked_placement_order() -> Result<(), ()> {
    let baked = bake(EVIDENCE_RECIPE)?;
    let text = baked.emit().tokens().ok_or(())?.inspected();
    let trials = token_position(&text, "recipe_trials_support")?;
    let mutation = token_position(&text, "recipe_mutation_support")?;
    let benchmarks = token_position(&text, "recipe_bench_support")?;
    let recipe_module = token_position(&text, "pub mod door")?;
    let baked_module = token_position(&text, "pub mod baked")?;
    let companions = token_position(&text, "STATE_VARIANTS")?;
    let network = token_position(&text, "recipe_network")?;
    let concurrency = token_position(&text, "recipe_concurrency")?;

    assert!(trials < mutation);
    assert!(mutation < benchmarks);
    assert!(benchmarks < recipe_module);
    assert!(recipe_module < baked_module);
    assert!(baked_module < companions);
    assert!(companions < network);
    assert!(network < concurrency);
    Ok(())
}

#[test]
fn caller_owned_projection_reads_every_declared_relation_requirement() -> Result<(), ()> {
    bake_with(
        POSTURE_RECIPE,
        &[ProjectorReplacement::for_role(
            RecipeRole::RelationTables,
            &ReadbackProjector,
        )],
    )
    .map(|_| ())
}

#[test]
fn unavailable_harness_roles_keep_their_exact_public_dispositions() -> Result<(), ()> {
    let baked = bake_under(COMPANION_RECIPE, HarnessPosture::Unavailable)?;
    let recipe = baked.projection().plan().content();
    let expected_dispositions = [
        ProjectionDisposition::Generated,
        ProjectionDisposition::NotRequested,
        ProjectionDisposition::NotRequested,
        ProjectionDisposition::FeatureUnavailable,
        ProjectionDisposition::FeatureUnavailable,
        ProjectionDisposition::NotRequested,
        ProjectionDisposition::FeatureUnavailable,
        ProjectionDisposition::FeatureUnavailable,
        ProjectionDisposition::FeatureUnavailable,
        ProjectionDisposition::FeatureUnavailable,
        ProjectionDisposition::FeatureUnavailable,
        ProjectionDisposition::NotRequested,
    ];
    assert_eq!(RecipeRole::ALL.len(), expected_dispositions.len());
    for (role, expected_disposition) in RecipeRole::ALL.iter().copied().zip(expected_dispositions) {
        assert_eq!(recipe.projection_disposition(role), expected_disposition);
    }
    Ok(())
}

#[test]
#[cfg(feature = "host")]
fn projection_errors_and_host_cargos_retain_their_public_cause_chain() -> Result<(), ()> {
    let error = ProjectionError::Render(RenderError::NothingRendered);
    assert!(core::error::Error::source(&error).is_some());

    let baked = bake(COMPANION_RECIPE)?;
    let cargos = Emittable::cargos(&baked).collect::<Vec<_>>();
    let [cargo] = cargos.as_slice() else {
        return Err(());
    };
    assert!(core::ptr::eq(*cargo, baked.emit()));

    let supported = bake(COMPLETE_RECIPE)?;
    let emitted = supported.emit().tokens().ok_or(())?.inspected();
    assert!(emitted.contains("door_recipe_support"));
    Ok(())
}

fn token_position(text: &str, needle: &str) -> Result<usize, ()> {
    text.find(needle).ok_or(())
}
