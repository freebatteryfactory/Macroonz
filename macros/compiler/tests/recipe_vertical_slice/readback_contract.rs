//! Stable recipe readback observed through public and caller-owned projection roads.

use super::{COMPANION_RECIPE, COMPLETE_RECIPE, DOOR, bake};
use macroonz_compiler::host::Emittable;
use macroonz_compiler::recipe::{
    HarnessPosture, LoweringSource, ProjectionDisposition, ProjectionError, ProjectionOffered,
    ProjectionRequest, ProjectionSink, ProjectorReplacement, RecipeProjector,
    RecipeRelationPayload, RecipeRole, RecipeView,
};
use macroonz_compiler::{
    AbsencePosture, CompletenessPosture, CyclePosture, DensityPosture, EmptyPosture,
    GeneratedToken, GeneratedTree, MembershipPosture, RenderError, RepetitionPosture,
    SelfRelationPosture, TextCapture, unit_struct,
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
        .map(<RecipeRole as macroonz_compiler::Role>::name)
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            "companions",
            "relation-tables",
            "dispatch",
            "compile-contract",
            "property",
            "typestate",
            "trials",
            "mutation",
            "benchmarks",
            "network",
            "concurrency",
            "codec",
        ]
    );
}

#[test]
fn caller_owned_projection_reads_every_declared_relation_requirement() -> Result<(), ()> {
    let read = TextCapture::read(POSTURE_RECIPE).map_err(|_| ())?;
    macroonz_compiler::recipe::bake_with(
        read.input(),
        HarnessPosture::Available,
        &DOOR,
        &[ProjectorReplacement::for_role(
            RecipeRole::RelationTables,
            &ReadbackProjector,
        )],
    )
    .map(|_| ())
    .map_err(|_| ())
}

#[test]
fn unavailable_harness_roles_keep_their_exact_public_dispositions() -> Result<(), ()> {
    let read = TextCapture::read(COMPANION_RECIPE).map_err(|_| ())?;
    let baked = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Unavailable, &DOOR)
        .map_err(|_| ())?;
    let recipe = baked.projection().plan().content();
    assert_eq!(
        recipe.projection_disposition(RecipeRole::Companions),
        ProjectionDisposition::Generated
    );
    assert_eq!(
        recipe.projection_disposition(RecipeRole::CompileContract),
        ProjectionDisposition::FeatureUnavailable
    );
    assert_eq!(
        recipe.projection_disposition(RecipeRole::Property),
        ProjectionDisposition::FeatureUnavailable
    );
    assert_eq!(
        recipe.projection_disposition(RecipeRole::Dispatch),
        ProjectionDisposition::NotRequested
    );
    Ok(())
}

#[test]
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
