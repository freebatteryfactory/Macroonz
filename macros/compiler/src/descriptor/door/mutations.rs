//! The road one `mutations` attribute walks: a captured body and the captured item it sits on in, the carrier expansion out.

use super::types::{SOLE_READING_FACT, TRIALS_FORM_FACT};
use super::walk::{helper_refused, proved_off, support_address, unit_tree, whole};
use crate::descriptor::mutation::{self, MutationCaptureError, MutationSurface, SurfaceRole};
use crate::descriptor::{CaptureCause, Grammar};
use crate::diagnostic::Diagnostic;
use crate::expansion::Expansion;
use crate::kind::{Destination, Disposition};
use crate::request::Door;
use crate::request::{self, Request};
use crate::support::{self, AxisCargo, CargoAxis, SupportAxes, SupportCarrier};
use crate::token::{CapturedInput, CapturedTokenTree, SpanHandle};

/// Walk one mutation declaration to the sealed carrier expansion its module rides out inside.
///
/// The body is read through the mutation grammar and completed from the item — the declared order is the item's own variant list — so the item's identity rides the request as a dependency.
/// The rendered module is proved into the test-carrier delivery and composes as the deferred axis; the stamped seat is honestly empty, and the carrier still writes the trials form.
///
/// A standalone attribute owns its carrier, so the `support` clause is required here even though the grammar admits its absence for a declaration whose carrier another helper already addressed.
///
/// # Errors
///
/// Returns one [`Diagnostic`], composed under the door: the grammar's refusal at the token it was established at, the absent support address as the grammar's own absent-clause cause, and every downstream road's refusal about the declaration as a whole.
pub fn mutations(
    body: CapturedInput,
    item: &CapturedInput,
    grammar: Grammar,
    door: &Door,
) -> Result<Expansion<SupportCarrier>, Diagnostic> {
    let trees: Vec<&CapturedTokenTree> = body.trees().iter().collect();
    let read = mutation::captured(&trees, SpanHandle::at(0), grammar)
        .map_err(|refusal| helper_refused(&refusal, refusal.refusal().at(), door))?;
    drop(trees);
    let item_trees: Vec<&CapturedTokenTree> = item.trees().iter().collect();
    let surface = mutation::completed(read, &item_trees, grammar)
        .map_err(|refusal| helper_refused(&refusal, refusal.refusal().at(), door))?;
    drop(item_trees);

    let Some(spelling) = surface.address().support.as_ref() else {
        let refusal = MutationCaptureError::grammar_refused(
            grammar,
            CaptureCause::ClauseAbsent,
            SpanHandle::at(0),
        );
        return Err(helper_refused(&refusal, SpanHandle::at(0), door));
    };
    let address = support_address(spelling.spelling(), door)?;

    let dependency = request::committed(item);
    let module = Request::<MutationSurface>::over(body.clone(), surface, door)
        .depending_on(vec![dependency])
        .render(|plan, out| {
            let rendered = unit_tree(mutation::generated_module(plan.content()))?;
            out.unit(SurfaceRole::Module, rendered)
        })?;

    let proved = proved_off(&module, CargoAxis::Deferred, Destination::TestCarrier, door)?;
    let axes = SupportAxes {
        declared: AxisCargo::Absent {
            because: Disposition::NotRequested {
                because: SOLE_READING_FACT,
            },
        },
        deferred: AxisCargo::Carried(proved),
        bench: AxisCargo::Absent {
            because: Disposition::NotApplicable {
                because: TRIALS_FORM_FACT,
            },
        },
    };
    let root = module.plan().account().commitment();
    let assembly = support::SupportAssembly::assembled(root, Some(address), axes)
        .map_err(|refusal| whole(&refusal, door))?;
    support::delivered(body, vec![dependency], assembly, door)
}
