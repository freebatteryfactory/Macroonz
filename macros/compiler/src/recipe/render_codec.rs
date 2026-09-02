//! Standard codec projection through the existing codec owner.

use super::{ProjectionError, Recipe};
use crate::token::GeneratedTree;

pub(super) fn codec(recipe: &Recipe) -> Result<GeneratedTree, ProjectionError> {
    let mut rendered = GeneratedTree::assembled(Vec::new()).map_err(ProjectionError::Tokens)?;
    let mut observed = false;
    for declaration in recipe.codecs() {
        observed = true;
        let next =
            crate::codec::codec_surface(declaration.content()).map_err(ProjectionError::Tokens)?;
        rendered = rendered.joined(&next).map_err(ProjectionError::Tokens)?;
    }
    if !observed {
        return Err(ProjectionError::Render(
            crate::render::RenderError::NothingRendered,
        ));
    }
    Ok(rendered)
}
