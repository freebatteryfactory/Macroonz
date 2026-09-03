//! Projection through the one capability shared by standard and caller-owned projectors.

mod codec;
mod companions;
mod dispatch;
mod evidence;
mod relation_tables;
mod tokens;
mod typestate;

use super::types::StandardProjector;
use super::{EffectiveProjection, RecipeTransitionEffect};
use super::{
    PreparedEvidence, ProjectionError, ProjectionRequest, ProjectionSink, Recipe, RecipeProjector,
    RecipeRole, RecipeView,
};
use crate::token::GeneratedTree;
use codec::codec;
use companions::companions;
use dispatch::dispatch;
use evidence::{compile_contract, declaration_conformance};
use relation_tables::relation_tables;
use typestate::typestate;

impl<'evidence> StandardProjector<'evidence> {
    /// Bind the built-in catalog to the descriptor outputs prepared for this recipe walk.
    pub(super) const fn over(evidence: &'evidence PreparedEvidence) -> Self {
        Self { evidence }
    }
}

impl RecipeProjector for StandardProjector<'_> {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<super::ProjectionOffered, ProjectionError> {
        let tree = match request.role() {
            RecipeRole::Companions => companions(view.recipe())?,
            RecipeRole::RelationTables => relation_tables(view.recipe(), request.effective())?,
            RecipeRole::Dispatch => dispatch(view.recipe(), request.effective())?,
            RecipeRole::CompileContract => compile_contract(view.recipe())?,
            RecipeRole::DeclarationConformance => declaration_conformance(view.recipe())?,
            RecipeRole::Typestate => typestate(view.recipe())?,
            RecipeRole::Codec => codec(view.recipe())?,
            RecipeRole::Trials
            | RecipeRole::Mutation
            | RecipeRole::Benchmarks
            | RecipeRole::Network
            | RecipeRole::Concurrency => self.evidence(request.role())?,
        };
        sink.offer(tree)
    }
}

impl StandardProjector<'_> {
    fn evidence(&self, role: RecipeRole) -> Result<GeneratedTree, ProjectionError> {
        self.evidence
            .tree(role)
            .cloned()
            .ok_or(ProjectionError::Render(
                crate::render::RenderError::NothingRendered,
            ))
    }
}

pub(super) fn project(
    recipe: &Recipe,
    role: RecipeRole,
    sink: ProjectionSink<'_, '_>,
    projector: &dyn RecipeProjector,
) -> Result<(), ProjectionError> {
    let Some(effective) = recipe.effective(role) else {
        return Err(ProjectionError::Render(
            crate::render::RenderError::SeatUnplanned { role: role.name() },
        ));
    };
    projector
        .project(
            RecipeView::over(recipe),
            ProjectionRequest::selected(effective),
            sink,
        )
        .map(|_| ())
}
