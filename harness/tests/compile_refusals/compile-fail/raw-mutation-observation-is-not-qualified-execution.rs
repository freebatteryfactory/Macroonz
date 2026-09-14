//! Retained raw observations cannot substitute for qualified execution correspondence.

use macroonz_harness::muterprater::{CompiledMutationObservation, QualifiedMutation};

fn elevate<'scope>(raw: CompiledMutationObservation<'scope, (), ()>) -> QualifiedMutation<'scope, (), ()> {
    raw
}

fn main() {}
