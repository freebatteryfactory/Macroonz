//! Projection capabilities and baked-recipe readback.

use super::{
    EffectiveProjection, LoweringSource, ProjectionError, ProjectionOffered, ProjectionRequest,
    ProjectionSink, RELATION_TABLE_LIMIT, Recipe, RecipeBake, RecipeError, RecipeIssue,
    RecipeProjection, RecipeRole, RecipeShell, RecipeShellContent, RecipeView,
    RelationTableProjection,
};
use crate::bounded::Bounded;
use crate::expansion::Expansion;
use crate::kind::Role;
use crate::render::Output;
use crate::token::{GeneratedTree, SpanHandle};

impl EffectiveProjection {
    pub(in crate::recipe) fn effective(
        role: RecipeRole,
        name: Option<String>,
        subject: Option<String>,
        source: LoweringSource,
    ) -> Self {
        Self {
            role,
            name,
            subject,
            source,
            exact_rust: None,
            exact_dispatch_bindings: None,
            exact_dispatch_imports: None,
            relation_tables: None,
        }
    }

    pub(in crate::recipe) fn exact_dispatch(
        name: String,
        exact_rust: GeneratedTree,
        bindings: [crate::token::GeneratedToken; 2],
        imports: [bool; 2],
    ) -> Self {
        Self {
            role: RecipeRole::Dispatch,
            name: Some(name),
            subject: None,
            source: LoweringSource::ExactRust,
            exact_rust: Some(exact_rust),
            exact_dispatch_bindings: Some(bindings),
            exact_dispatch_imports: Some(imports),
            relation_tables: None,
        }
    }

    pub(in crate::recipe) fn with_relation_tables(
        tables: Bounded<RelationTableProjection, RELATION_TABLE_LIMIT>,
    ) -> Self {
        Self {
            role: RecipeRole::RelationTables,
            name: None,
            subject: None,
            source: LoweringSource::Configuration,
            exact_rust: None,
            exact_dispatch_bindings: None,
            exact_dispatch_imports: None,
            relation_tables: Some(Box::new(tables)),
        }
    }

    /// Reads the selected role.
    #[must_use]
    pub const fn role(&self) -> RecipeRole {
        self.role
    }

    /// Reads the effective public spelling where this role declares one.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Reads the informed structural subject selected for this projection.
    #[must_use]
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_deref()
    }

    pub(in crate::recipe) fn select_subject(&mut self, subject: String) {
        self.subject = Some(subject);
    }

    /// Reads where the effective value came from.
    #[must_use]
    pub const fn source(&self) -> LoweringSource {
        self.source
    }

    /// Reads the exact caller-authored Rust that replaced this mechanical seat.
    #[must_use]
    pub const fn exact_rust(&self) -> Option<&GeneratedTree> {
        self.exact_rust.as_ref()
    }

    pub(in crate::recipe) const fn exact_dispatch_bindings(
        &self,
    ) -> Option<&[crate::token::GeneratedToken; 2]> {
        self.exact_dispatch_bindings.as_ref()
    }

    pub(in crate::recipe) const fn exact_dispatch_imports(&self) -> Option<&[bool; 2]> {
        self.exact_dispatch_imports.as_ref()
    }

    /// Reads every selected relation-table surface in declaration order.
    pub fn relation_tables(&self) -> impl Iterator<Item = &RelationTableProjection> {
        self.relation_tables.iter().flat_map(|tables| tables.iter())
    }
}

impl RelationTableProjection {
    pub(in crate::recipe) fn informed(
        relation: String,
        function: String,
        source: LoweringSource,
        exact_rust: Option<GeneratedTree>,
        bindings: Option<[crate::token::GeneratedToken; 2]>,
        imports: Option<[bool; 2]>,
    ) -> Self {
        Self {
            relation,
            function,
            source,
            exact_rust,
            bindings,
            imports,
        }
    }

    /// Reads the caller-named relation selected by this table.
    #[must_use]
    pub fn relation(&self) -> &str {
        self.relation.as_str()
    }

    /// Reads the effective function name inside the relation-named module.
    #[must_use]
    pub fn function(&self) -> &str {
        self.function.as_str()
    }

    /// Reads where this function surface came from.
    #[must_use]
    pub const fn source(&self) -> LoweringSource {
        self.source
    }

    /// Reads the exact caller-authored function signature where one was supplied.
    #[must_use]
    pub const fn exact_rust(&self) -> Option<&GeneratedTree> {
        self.exact_rust.as_ref()
    }

    pub(in crate::recipe) const fn bindings(&self) -> Option<&[crate::token::GeneratedToken; 2]> {
        self.bindings.as_ref()
    }

    pub(in crate::recipe) const fn imports(&self) -> Option<&[bool; 2]> {
        self.imports.as_ref()
    }
}

impl<'recipe> RecipeView<'recipe> {
    pub(in crate::recipe) const fn over(recipe: &'recipe Recipe) -> Self {
        Self { recipe }
    }

    /// Reads the informed recipe without plan or mutation authority.
    #[must_use]
    pub const fn recipe(self) -> &'recipe Recipe {
        self.recipe
    }
}

impl<'recipe> ProjectionRequest<'recipe> {
    pub(in crate::recipe) const fn selected(effective: &'recipe EffectiveProjection) -> Self {
        Self { effective }
    }

    /// Reads the exact selected role this invocation answers.
    #[must_use]
    pub const fn role(self) -> RecipeRole {
        self.effective.role()
    }

    /// Reads the complete effective mechanical configuration for this invocation.
    #[must_use]
    pub const fn effective(self) -> &'recipe EffectiveProjection {
        self.effective
    }

    /// Reads the destination owned by the selected role.
    #[must_use]
    pub fn destination(self) -> crate::kind::Destination {
        self.role().destination()
    }
}

impl<'output, 'plan> ProjectionSink<'output, 'plan> {
    pub(in crate::recipe) const fn bound(
        output: &'output mut Output<'plan, RecipeProjection>,
        role: RecipeRole,
    ) -> Self {
        Self { output, role }
    }

    /// Offers one tree under the exact role bound into this one-use capability.
    ///
    /// # Errors
    ///
    /// Returns the existing output refusal when the plan does not admit the role or the rendered bytes exceed their bound.
    pub fn offer(self, tree: GeneratedTree) -> Result<ProjectionOffered, ProjectionError> {
        self.output
            .unit(self.role, tree)
            .map_err(ProjectionError::Render)?;
        Ok(ProjectionOffered { _private: () })
    }
}

impl RecipeError {
    pub(in crate::recipe) const fn at(issue: RecipeIssue, at: Option<SpanHandle>) -> Self {
        Self { issue, at }
    }

    /// Reads the exact recipe issue.
    #[must_use]
    pub(in crate::recipe) const fn issue(&self) -> &RecipeIssue {
        &self.issue
    }

    /// Reads the captured producer span available for this issue.
    #[must_use]
    pub(in crate::recipe) const fn token(&self) -> Option<SpanHandle> {
        self.at
    }
}

impl RecipeBake {
    pub(in crate::recipe) const fn baked(
        projection: Expansion<RecipeProjection>,
        emitted: Expansion<RecipeShell>,
    ) -> Self {
        Self {
            projection,
            emitted,
        }
    }

    /// Reads the selected projection expansion before final module assembly.
    pub const fn projection(&self) -> &Expansion<RecipeProjection> {
        &self.projection
    }

    /// Reads the proved declaration-site cargo emitted by the paved proc host.
    pub fn emit(&self) -> &crate::closure::PartitionCargo {
        self.emitted.emit()
    }
}

impl RecipeShellContent {
    pub(in crate::recipe) const fn composed(
        recipe: crate::identity::ClosedExpansionId,
        support: Option<crate::identity::ClosedExpansionId>,
    ) -> Self {
        Self { recipe, support }
    }
}
