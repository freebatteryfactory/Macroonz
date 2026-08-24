//! An independent test harness: describe a subject once, and the harness spends its time trying to make that description false.
//!
//! Every verdict is claim-specific and method-specific, and carries the evidence and the replay that reproduce it.
//! The harness is a dev-dependency, and it depends on nothing it judges.
//! No type is re-exported at this root, so a call site spells the home that made the claim — `oracle::StructuralVerdict`, `muterprater::ARTIFACT_MUTATIONS`.
//! The four stamps are the exception, because Rust puts an exported macro at the crate root: [`generated_support!`], [`trial_table!`], [`bench_table!`], and the [`ensure_ok!`] battery beside it.
//!
//! # The instruments
//!
//! [`descriptor`] owns the rows a producer writes into, and [`runner`] turns those rows and a typed invocation into typed reports.
//! [`generate`] owns the generation contract, [`corpus`] carries content-addressed warm starts for it, and [`properties`] holds the algebraic laws a subject can be held to.
//! [`fault`] schedules owner-declared adversity, [`clock`] is the caller-declared wall-measurement boundary, and [`mod@bench`] judges work under a pinned profile.
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
pub mod generate;
pub mod muterprater;
pub mod oracle;
pub mod properties;
pub mod report;
pub mod runner;
