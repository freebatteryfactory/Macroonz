//! The road one `bench` attribute walks: a captured body and the item it exercises in, the carrier expansion out.

use super::types::BENCH_FORM_FACT;
use super::walk::{helper_refused, proved_off, support_address, unit_tree, whole};
use crate::descriptor::bench::{self, BENCH_HELPER_POSITION, BenchAnswer, BenchRole, BenchTable};
use crate::descriptor::{Emitter, Grammar};
use crate::diagnostic::Diagnostic;
use crate::expansion::Expansion;
use crate::kind::{Destination, Disposition};
use crate::request::Door;
use crate::request::Request;
use crate::support::{self, AxisCargo, CargoAxis, DeclaredCargo, SupportAxes, SupportCarrier};
use crate::token::{CapturedInput, CapturedTokenTree, GeneratedTree, SpanHandle};

/// Walk one benchmark declaration to the sealed carrier expansion its table and report reader ride out inside.
///
/// The body is read through the bench grammar beside the item it exercises, the bench terminal proves the table into its declaration-site delivery and the report reader into the bench-carrier delivery, and the carrier composes the two as the bench form: stamped table, opaque reporter.
/// The item is the semantic declaration both requests stand over, while the body is committed separately at [`BENCH_HELPER_POSITION`].
///
/// # Errors
///
/// Returns one [`Diagnostic`], composed under the door: the grammar's refusal at the token it was established at, and every downstream road's refusal about the declaration as a whole.
pub fn bench(
    body: &CapturedInput,
    item: &CapturedInput,
    grammar: Grammar,
    emitter: Emitter,
    door: &Door,
) -> Result<Expansion<SupportCarrier>, Diagnostic> {
    let trees: Vec<&CapturedTokenTree> = body.trees().iter().collect();
    let read = bench::captured(&trees, SpanHandle::at(0), grammar)
        .map_err(|refusal| helper_refused(&refusal, refusal.refusal().at(), door))?;
    drop(trees);

    let address = support_address(read.support().spelling(), door)?;
    let matched = GeneratedTree::assembled(bench::matched_clauses(&read))
        .map_err(|overflow| whole(&super::walk::overflown(overflow), door))?;
    let rows = u64::try_from(read.row_count()).unwrap_or(u64::MAX);
    let answer = BenchAnswer::MeasuringBenchmarks {
        table: read.table().clone(),
        rows,
    };

    let delivery = Request::<BenchTable>::over(item.clone(), read, door)
        .answering(vec![answer])
        .render(|plan, out| {
            let table = unit_tree(bench::bench_table(plan.content(), emitter))?;
            out.unit(BenchRole::Table, table)?;
            let reporter = unit_tree(bench::reporter(plan.content()))?;
            out.unit(BenchRole::Reporter, reporter)
        })?;

    let declared =
        DeclaredCargo::stamped_from(&delivery, matched).map_err(|refusal| whole(&refusal, door))?;
    let proved = proved_off(&delivery, CargoAxis::Bench, Destination::BenchCarrier, door)?;
    let axes = SupportAxes {
        declared: AxisCargo::Carried(declared),
        deferred: AxisCargo::Absent {
            because: Disposition::NotApplicable {
                because: BENCH_FORM_FACT,
            },
        },
        bench: AxisCargo::Carried(proved),
    };
    let assembly = support::SupportAssembly::assembled_for_helper(
        item,
        body,
        BENCH_HELPER_POSITION,
        Some(address),
        axes,
    )
    .map_err(|refusal| whole(&refusal, door))?;
    support::delivered(item.clone(), Vec::new(), assembly, door)
}
