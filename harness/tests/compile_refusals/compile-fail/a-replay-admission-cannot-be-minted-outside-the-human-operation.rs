//! An outside caller cannot mint replay-bearing admitted provenance directly.
//!
//! The public road is the mutation home's explicit human-admission operation, which first binds proposal custody and the caller-owned replay depot.

use macroonz_harness::descriptor::{
    ExecutionSuite, ProposalId, ReplayAdmission, ReplayBearingGround, ReplayRef,
};

fn bypass_human_admission(
    proposal: ProposalId,
    ground: ReplayBearingGround,
    destination: ExecutionSuite,
    replay: ReplayRef,
) -> ReplayAdmission {
    ReplayAdmission::admitted(proposal, ground, destination, replay)
}

fn main() {}
