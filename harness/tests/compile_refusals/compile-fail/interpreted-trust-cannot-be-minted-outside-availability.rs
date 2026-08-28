//! Claim: interpreted trust can be minted only by availability after every required evidence join succeeds.
//!
//! Subject: the private fields of `InterpretedTrust` at the public crate boundary.
//! Population: all three retained authorities offered directly through one outside struct literal.
//! Hostile control: the fixture supplies a surface, generic suite pressure, and exact projection pressure while bypassing availability.
//! Denominator: every retained trust member.
//! Evidence ceiling: compiler privacy proves outside unwritability and does not establish the runtime truth of the supplied evidence.
//! Retained regression: trybuild records the Rust 1.98 private-field refusals.

use macroonz_harness::muterprater::{
    CompiledProjectionPressure, CompiledSuitePressure, EvaluationSurface, InterpretedTrust,
};

fn remint<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning>(
    surface: &'surface EvaluationSurface,
    suite: &'suite CompiledSuitePressure,
    projection: &'projection CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning>,
) -> InterpretedTrust<'surface, 'suite, 'projection, 'parity, 'pair, 'input, Input, Meaning> {
    InterpretedTrust {
        surface,
        suite,
        projection,
    }
}

fn main() {}
