//! An independent test harness: describe a subject once, and the harness spends its time trying to make that description false.
//!
//! Every verdict is claim-specific and method-specific and carries the evidence that earned it; a reduction-minted replay capsule is a second, separately earned value, joined to its report on the one execution key they share.
//! The harness belongs in test and tooling code, and it depends on nothing it judges.
//! No type is re-exported at this root, so a call site spells the home that made the claim — `oracle::StructuralVerdict`, `muterprater::ARTIFACT_MUTATIONS`.
//! The four stamps are the exception, because Rust puts an exported macro at the crate root: [`generated_support!`], [`trial_table!`], [`bench_table!`], and the [`ensure_ok!`] battery beside it.
//!
//! # The instruments
//!
//! [`descriptor`] owns the rows a producer writes into, and [`runner`] turns those rows and a typed invocation into typed reports.
//! [`generate`] owns the generation contract, [`corpus`] carries content-addressed warm starts for it, and [`properties`] holds the algebraic laws a subject can be held to.
//! [`interleave`] explores the orders concurrent parties' commands can merge in, with the schedule itself a generated input, and [`network`] is the deterministic message-passing sim whose command-shaped deliveries feed that exploration.
//! The feature-gated `preemption` module explores instruction-level interleavings and the memory model through a target-qualified backend: the feature enables Loom where the pinned scheduler supports the target and retains typed backend unavailability everywhere else.
//! [`fault`] schedules owner-declared adversity, [`clock`] is the caller-declared wall-measurement boundary, and [`mod@bench`] judges work under a pinned profile.
//! [`fuzz`] owns the Macroonz campaign shell around stable rustc coverage profiles: active toolchain readiness, root-independent coverage observations, novelty, and interesting-byte handoff into reduction and replay.
//! [`muterprater`] is the mutation-pressure engine, and [`oracle`] is the independence annex for claims where self-agreement would be vacuous.
//! [`report`] owns what a run leaves behind, [`depot`] is the harness's own fact bank, and [`identity`] is the derivation substrate every identity here goes through.
//! Executable challenge material sits under `tests/`, where cargo requires it to live.

pub mod identity;

pub mod bench;
pub mod clock;
pub mod corpus;
pub mod depot;
pub mod descriptor;
pub mod fault;
pub mod fuzz;
pub mod generate;
pub mod interleave;
pub mod muterprater;
pub mod network;
pub mod oracle;
#[cfg(feature = "preemption")]
pub mod preemption;
pub mod properties;
pub mod report;
pub mod runner;
