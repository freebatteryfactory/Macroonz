//! The road one `shadow!` declaration walks: a captured body in, the direct two-faced expansion out.
//!
//! The shortest road a door here takes, because a shadow face rides no carrier: the reading chooses roster rows, the rendering writes each row's two `cfg`-gated faces, and the one rendered unit lands at the declaration site as ordinary items.

use super::walk::{helper_refused, unit_tree};
use crate::descriptor::Grammar;
use crate::descriptor::shadow::{self, ShadowFace};
use crate::diagnostic::Diagnostic;
use crate::expansion::Expansion;
use crate::kind::SoleRole;
use crate::request::Door;
use crate::request::Request;
use crate::token::CapturedInput;

/// Walk one shadow declaration to the sealed expansion carrying both faces of every chosen name.
///
/// # Errors
///
/// Returns one [`Diagnostic`], composed under the door: the grammar's refusal at the token it was established at, and every downstream road's refusal about the declaration as a whole.
pub fn shadow(
    body: CapturedInput,
    grammar: Grammar,
    door: &Door,
) -> Result<Expansion<ShadowFace>, Diagnostic> {
    let read = shadow::chosen(&body, grammar)
        .map_err(|refusal| helper_refused(&refusal, refusal.refusal().at(), door))?;
    Request::<ShadowFace>::over(body, read, door)
        .answering(Vec::new())
        .render(|plan, out| {
            let face = unit_tree(shadow::faces(plan.content()))?;
            out.unit(SoleRole::Sole, face)
        })
}
