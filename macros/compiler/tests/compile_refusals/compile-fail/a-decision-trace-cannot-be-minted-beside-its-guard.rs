//! A decision trace can be minted only by the constructor that establishes its non-empty bounded record.

use macroonz_compiler::identity::{Identity, Role, Traced, Transcript};
use macroonz_compiler::{
    DecisionTrace, NonEmpty, TRACE_ENTRY_LIMIT, TraceDecision, TraceEntry,
};

fn main() {
    let subject: Identity<Traced> =
        Identity::derived(Transcript::rooted(Role::Plan, b"subject", 0));
    let entries = NonEmpty::<TraceEntry, TRACE_ENTRY_LIMIT>::one(TraceEntry {
        subject,
        decision: TraceDecision::NotRun,
    });
    let _trace = DecisionTrace { entries };
}
