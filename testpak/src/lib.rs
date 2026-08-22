//! `threadpak-testpak` is ThreadPak's testing harness.
//! What the harness is, what its instruments own, and the laws they answer to
//! are the crate README's; each instrument's README is its module page.
//!
//! A verdict here is always claim-specific and method-specific.
//! "The artifact declares a trait path the caller's declaration did not name,
//! read out of a parse nobody here wrote" is a verdict; "the derive works" is
//! not one.
//!
//! # The instruments
//!
//! [`clock`] owns `TestPak`'s caller-declared wall-measurement boundary,
//! [`descriptor`] owns the rows every producer writes into, [`report`] owns
//! the records a run leaves and the identity rails, [`oracle`] is the
//! independence annex for claims where shared producer logic would make
//! self-agreement vacuous, [`runner`] is the report engine that turns
//! descriptor tables and typed invocations into typed reports,
//! [`properties`] carries the algebraic suites, [`muterprater`] is
//! the proof-pressure engine, [`generate`] owns the generation contract,
//! [`depot`] is the harness's own fact bank, [`fault`] schedules adopter-owned
//! fault adapters, [`corpus`] carries content-addressed generation warm starts,
//! and [`mod@bench`] owns primary work judgment plus secondary wall observation.
//! [`identity`] is the derivation substrate every identity kind in this crate
//! derives through.
//!
//! Executable challenge material sits under `tests/`, where cargo requires
//! it to live. Pack persistence is caller-owned; [`corpus`] owns the public
//! in-memory pack format and reader.
//!
//! # The instruments are reached at their own homes
//!
//! Nothing is re-exported at this root. An instrument's vocabulary is spelled
//! through the module that owns it — `oracle::StructuralVerdict`,
//! `muterprater::ARTIFACT_MUTATIONS` — so a reader of a call site sees which
//! home made the claim rather than a flat surface that hides it.
//!
//! # The dependency direction
//!
//! testpak depends inward, and production never depends on its judge: no
//! production dependency table in this workspace names it. The one manifest
//! that names it at all is the consumer parity seat's, as a dev-dependency
//! under a consumer-chosen name — which is that seat's whole point. What
//! this package asks for, and why, is the README's; which versions, the
//! workspace manifest's.

pub mod identity;

pub mod bench;
pub mod clock;
pub mod depot;
pub mod descriptor;
pub mod corpus;
pub mod fault;
pub mod generate;
pub mod muterprater;
pub mod oracle;
pub mod properties;
pub mod report;
pub mod runner;
