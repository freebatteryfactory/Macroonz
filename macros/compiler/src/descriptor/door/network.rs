//! The road one `network!` declaration walks: a captured body in, the direct builder-module expansion out.

use super::walk::{helper_refused, unit_tree};
use crate::descriptor::Grammar;
use crate::descriptor::network::{self, NetworkModule};
use crate::diagnostic::Diagnostic;
use crate::expansion::Expansion;
use crate::kind::SoleRole;
use crate::request::Door;
use crate::request::Request;
use crate::token::CapturedInput;

/// Walk one network declaration to the sealed expansion carrying its builder module.
///
/// # Errors
///
/// Returns one [`Diagnostic`], composed under the door: the grammar's refusal at the token it was established at, and every downstream road's refusal about the declaration as a whole.
pub fn network(
    body: CapturedInput,
    grammar: Grammar,
    door: &Door,
) -> Result<Expansion<NetworkModule>, Diagnostic> {
    let read = network::declared(&body, grammar)
        .map_err(|refusal| helper_refused(&refusal, refusal.refusal().at(), door))?;
    Request::<NetworkModule>::over(body, read, door)
        .answering(Vec::new())
        .render(|plan, out| {
            let module = unit_tree(network::rendered(plan.content()))?;
            out.unit(SoleRole::Sole, module)
        })
}
