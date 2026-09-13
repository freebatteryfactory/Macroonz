//! Qualified correspondence cannot be assembled from arbitrary retained meanings and a positive count.

use core::num::NonZeroU32;
use macroonz_harness::muterprater::{CompiledMutationObservation, QualifiedMutation};
use macroonz_harness::properties::{Agreement, SharedSubstrate};

fn mint<'scope>(compiled: &'scope CompiledMutationObservation<'scope, (), ()>, meaning: &'scope (), firings: NonZeroU32, substrate: SharedSubstrate) -> QualifiedMutation<'scope, (), ()> {
    QualifiedMutation {
        compiled,
        baseline: meaning,
        selected: meaning,
        compiled_baseline: meaning,
        compiled_selected: meaning,
        firings,
        substrate,
        difference: Agreement::Differs,
    }
}

fn main() {}
