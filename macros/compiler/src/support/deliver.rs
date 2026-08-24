//! The one road from a verified assembly to an emitted carrier expansion.
//!
//! A carrier is rendered from a [`SupportAssembly`] by [`SupportShell::assembled`], and the shell needs the carrier PLAN — the mangled name is the plan's identity at full width — so the two meet inside a request's own render step, where the plan exists and nothing can forge one.
//! This road walks that request whole: the assembly rides as the carrier kind's content, the renderer assembles the shell over the plan it is called with, and what comes back is an ordinary sealed expansion whose one declaration-site unit is the exported carrier.
//!
//! The shell road's own refusal crosses honestly.
//! A renderer answers in the render vocabulary, which has no seat for a shell refusal, so the refusal is parked beside the walk and converted to the diagnostic it deserves — the render-level abort it rides out on is never what a person reads.

use super::types::{ShellError, SupportAssembly, SupportCarrier, SupportShell};
use crate::diagnostic::{Diagnostic, Door, Placement};
use crate::expansion::Expansion;
use crate::identity::{self, Identity};
use crate::kind::SoleRole;
use crate::render::RenderError;
use crate::request::Request;
use crate::token::CapturedInput;

/// Walk one carrier request from a verified assembly to the sealed expansion its exported carrier rides out of.
///
/// The capture is the declaration the assembly stands over, captured again by the door that holds both; the dependencies are the further captures the door read content from, and an empty set states there were none.
///
/// # Errors
///
/// Returns one [`Diagnostic`], composed under the door: the shell road's own refusal where the plan and the assembly are not one declaration's or the composed carrier outgrows the token magnitude, and any refusal of the request road itself on the same terms as every other expansion.
pub fn delivered(
    capture: CapturedInput,
    dependencies: Vec<Identity<identity::CapturedDeclaration>>,
    assembly: SupportAssembly,
    door: &Door,
) -> Result<Expansion<SupportCarrier>, Diagnostic> {
    let mut parked: Option<ShellError> = None;
    let walked = Request::<SupportCarrier>::over(capture, assembly, door)
        .depending_on(dependencies)
        .render(
            |plan, out| match SupportShell::assembled(plan, plan.content(), door) {
                Ok(shell) => out.unit(SoleRole::Sole, shell.into_tree()),
                Err(refusal) => {
                    parked = Some(refusal);
                    Err(RenderError::NothingRendered)
                }
            },
        );
    if let Some(refusal) = parked {
        return Err(Diagnostic::refused(
            &refusal,
            door,
            &Placement::WholeDeclaration,
        ));
    }
    walked
}
