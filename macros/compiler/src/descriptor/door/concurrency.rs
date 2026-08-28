//! The road one `concurrency!` declaration walks: a captured body in, the direct exploration-module expansion out.

use super::walk::{helper_refused, unit_tree};
use crate::descriptor::Grammar;
use crate::descriptor::concurrency::{self, ConcurrencyModule};
use crate::diagnostic::Diagnostic;
use crate::expansion::Expansion;
use crate::kind::SoleRole;
use crate::request::Door;
use crate::request::Request;
use crate::token::CapturedInput;

/// Walk one concurrency declaration to the sealed expansion carrying its exploration module.
///
/// # Errors
///
/// Returns one [`Diagnostic`], composed under the door: the grammar's refusal at the token it was established at, and every downstream road's refusal about the declaration as a whole.
pub fn concurrency(
    body: CapturedInput,
    grammar: Grammar,
    door: &Door,
) -> Result<Expansion<ConcurrencyModule>, Diagnostic> {
    let read = concurrency::declared(&body, grammar)
        .map_err(|refusal| helper_refused(&refusal, refusal.refusal().at(), door))?;
    Request::<ConcurrencyModule>::over(body, read, door)
        .answering(Vec::new())
        .render(|plan, out| {
            let module = unit_tree(concurrency::rendered(plan.content()))?;
            out.unit(SoleRole::Sole, module)
        })
}
