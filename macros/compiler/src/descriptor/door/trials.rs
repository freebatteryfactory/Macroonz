//! The road one `trials` attribute walks: a captured body and the item it exercises in, the carrier expansion out.

use super::types::{SOLE_READING_FACT, TRIALS_FORM_FACT};
use super::walk::{helper_refused, support_address, unit_tree, whole};
use crate::descriptor::trial::{self, TRIAL_HELPER_POSITION, TrialAnswer, TrialRole, TrialTable};
use crate::descriptor::{Emitter, Grammar};
use crate::diagnostic::Diagnostic;
use crate::expansion::Expansion;
use crate::kind::Disposition;
use crate::request::Door;
use crate::request::Request;
use crate::support::{
    self, AxisCargo, DeclaredCargo, DeclaringBinding, SupportAxes, SupportCarrier,
};
use crate::token::{CapturedInput, CapturedTokenTree, GeneratedTree, SpanHandle};

/// Walk one trial declaration to the sealed carrier expansion its table rides out inside.
///
/// The body is read through the trial grammar beside the item it exercises, the trial terminal proves the stamped table into its declaration-site delivery, and the carrier composes that table as declared cargo with both proved axes honestly absent.
/// The item is the semantic declaration both requests stand over, while the body is committed separately at [`TRIAL_HELPER_POSITION`].
///
/// # Errors
///
/// Returns one [`Diagnostic`], composed under the door: the grammar's refusal at the token it was established at, and every downstream road's refusal about the declaration as a whole.
pub fn trials(
    body: &CapturedInput,
    item: &CapturedInput,
    grammar: Grammar,
    emitter: Emitter,
    door: &Door,
) -> Result<Expansion<SupportCarrier>, Diagnostic> {
    walked(body, item, grammar, emitter, DeclaringBinding::Absent, door)
}

/// Walk one recipe-composed trial declaration while requiring its consumer to state the declaring crate path.
pub(crate) fn trials_requiring_declaring(
    body: &CapturedInput,
    item: &CapturedInput,
    grammar: Grammar,
    emitter: Emitter,
    door: &Door,
) -> Result<Expansion<SupportCarrier>, Diagnostic> {
    walked(
        body,
        item,
        grammar,
        emitter,
        DeclaringBinding::Required,
        door,
    )
}

fn walked(
    body: &CapturedInput,
    item: &CapturedInput,
    grammar: Grammar,
    emitter: Emitter,
    declaring: DeclaringBinding,
    door: &Door,
) -> Result<Expansion<SupportCarrier>, Diagnostic> {
    let trees: Vec<&CapturedTokenTree> = body.trees().iter().collect();
    let read = trial::captured(&trees, SpanHandle::at(0), grammar)
        .map_err(|refusal| helper_refused(&refusal, refusal.refusal().at(), door))?;
    drop(trees);

    let address = support_address(read.support().spelling(), door)?;
    let matched = GeneratedTree::assembled(trial::matched_clauses(&read))
        .map_err(|overflow| whole(&super::walk::overflown(overflow), door))?;
    let rows = u64::try_from(read.row_count()).unwrap_or(u64::MAX);
    let answer = TrialAnswer::ChallengingTests {
        table: read.table().clone(),
        rows,
    };

    let table = Request::<TrialTable>::over(item.clone(), read, door)
        .answering(vec![answer])
        .render(|plan, out| {
            let stamped = unit_tree(trial::stamped_module(plan.content(), emitter))?;
            out.unit(TrialRole::Table, stamped)
        })?;

    let declared =
        DeclaredCargo::stamped_from(&table, matched).map_err(|refusal| whole(&refusal, door))?;
    let axes = SupportAxes {
        declared: AxisCargo::Carried(declared),
        deferred: AxisCargo::Absent {
            because: Disposition::NotRequested {
                because: SOLE_READING_FACT,
            },
        },
        bench: AxisCargo::Absent {
            because: Disposition::NotApplicable {
                because: TRIALS_FORM_FACT,
            },
        },
    };
    let assembly = support::SupportAssembly::assembled_for_helper_with_binding(
        item,
        body,
        TRIAL_HELPER_POSITION,
        Some(address),
        declaring,
        axes,
    )
    .map_err(|refusal| whole(&refusal, door))?;
    support::delivered(item.clone(), Vec::new(), assembly, door)
}
