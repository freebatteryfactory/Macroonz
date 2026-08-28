//! Claim: exact compiled projection pressure can be minted only by the operation that establishes every retained join.
//!
//! Subject: the private fields of `CompiledProjectionPressure` at the public crate boundary.
//! Population: all six retained members offered directly through one outside struct literal.
//! Hostile control: the fixture supplies every member but bypasses the proof operation.
//! Denominator: parity, baseline content, selected standing, both reports, and the mutation report.
//! Evidence ceiling: compiler privacy proves outside unwritability and does not establish runtime host behavior.
//! Retained regression: trybuild records the stable private-field refusal.

use macroonz_harness::muterprater::{
    ArtifactContentId, CompiledProjectionPressure, CompiledSpecimenStanding, MutationReport,
    NoMutationParityQualification,
};
use macroonz_harness::report::TrialReport;

fn remint<'parity, 'pair, 'input, Input, Meaning>(
    parity: &'parity NoMutationParityQualification<'pair, 'input, Input, Meaning>,
    baseline_artifact: ArtifactContentId,
    standing: CompiledSpecimenStanding,
    baseline_report: TrialReport,
    selected_report: TrialReport,
    mutation: MutationReport,
) -> CompiledProjectionPressure<'parity, 'pair, 'input, Input, Meaning> {
    CompiledProjectionPressure {
        parity,
        baseline_artifact,
        standing,
        baseline_report,
        selected_report,
        mutation,
    }
}

fn main() {}
