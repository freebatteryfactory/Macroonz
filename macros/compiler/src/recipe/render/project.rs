//! The built-in projector catalog and the dispatch of one selected role through a projector.

use super::super::types::StandardProjector;
use super::super::{
    PreparedEvidence, ProjectionError, ProjectionRequest, ProjectionSink, Recipe, RecipeProjector,
    RecipeRole, RecipeView,
};
use super::codec::codec;
use super::companions::companions;
use super::dispatch::dispatch;
use super::evidence::{compile_contract, declaration_conformance};
use super::relation_tables::relation_tables;
use super::typestate::typestate;
use crate::token::GeneratedTree;

impl<'evidence> StandardProjector<'evidence> {
    /// Bind the built-in catalog to the descriptor outputs prepared for this recipe walk.
    pub(in crate::recipe) const fn over(evidence: &'evidence PreparedEvidence) -> Self {
        Self { evidence }
    }
}

impl RecipeProjector for StandardProjector<'_> {
    fn project(
        &self,
        view: RecipeView<'_>,
        request: ProjectionRequest<'_>,
        sink: ProjectionSink<'_, '_>,
    ) -> Result<super::super::ProjectionOffered, ProjectionError> {
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

pub(in crate::recipe) fn project(
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
