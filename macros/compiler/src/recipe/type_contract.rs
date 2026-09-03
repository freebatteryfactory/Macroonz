//! Recipe kind, role, lowering, refusal, and host-emission contracts.

#[cfg(feature = "host")]
use super::RecipeBake;
use super::types::{RecipeShell, RecipeShellContent};
use super::{ProjectionError, Recipe, RecipeProjection, RecipeRole};
use crate::bounded::Overflow;
use crate::kind::{Destination, Kind, NoQuestions, Role, SoleRole};
use crate::render::RenderError;
use core::fmt;

impl Role for RecipeRole {
    const ALL: &'static [Self] = Self::ALL;

    fn name(self) -> &'static str {
        Self::name(self)
    }

    fn destination(self) -> Destination {
        self.profile().output.destination
    }
}

impl Kind for RecipeProjection {
    const NAME: &'static str = "recipe-projection";
    type Content = Recipe;
    type Role = RecipeRole;
    type Question = NoQuestions;
}

impl Kind for RecipeShell {
    const NAME: &'static str = "recipe-emission";
    type Content = RecipeShellContent;
    type Role = SoleRole;
    type Question = NoQuestions;
}

impl fmt::Display for ProjectionError {
    fn fmt(&self, into: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tokens(overflow) => write!(into, "{overflow}"),
            Self::Render(refusal) => write!(into, "{refusal}"),
        }
    }
}

impl core::error::Error for ProjectionError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Tokens(overflow) => Some(overflow),
            Self::Render(refusal) => Some(refusal),
        }
    }
}

impl From<Overflow> for ProjectionError {
    fn from(overflow: Overflow) -> Self {
        Self::Tokens(overflow)
    }
}

impl From<ProjectionError> for RenderError {
    fn from(refusal: ProjectionError) -> Self {
        match refusal {
            ProjectionError::Tokens(overflow) => Self::TokensUnbounded {
                bound: overflow.capacity,
                observed: overflow.offered,
            },
            ProjectionError::Render(render) => render,
        }
    }
}

#[cfg(feature = "host")]
impl crate::host::Emittable for RecipeBake {
    fn cargos(&self) -> impl Iterator<Item = &crate::closure::PartitionCargo> {
        core::iter::once(self.emit())
    }
}
