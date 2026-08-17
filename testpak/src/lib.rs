//! `threadpak-testpak` is ThreadPak's testing harness.
//! What the harness is, what its instruments own, and the laws they answer to
//! are the crate README's; each instrument's README is its module page.
//!
//! A verdict here is always claim-specific and method-specific.
//! "The permuted rendering was rejected by the string scan over these two
//! declared orders" is a verdict; "the derive works" is not one.
//!
//! # The instruments
//!
//! [`descriptor`] owns the rows every producer writes into, [`report`] owns
//! the records a run leaves, [`oracle`] is the independence annex for claims
//! where bytes are the spec, [`runner`] turns descriptor tables into nextest
//! trials, [`properties`] carries the algebraic suites, [`muterprater`] is
//! the proof-pressure engine, and [`fault`] holds the refusing adapters.
//!
//! Executable challenge material sits under `tests/`, where cargo requires
//! it to live; seed-packs sit under `corpus/`.
//!
//! # The standing seat
//!
//! One numbered seat carries the pre-redesign machinery: [`judge`], whose
//! three readings and mutation roster are the oracle's and muterprater's
//! seed material.
//!
//! # The dependency direction
//!
//! testpak depends inward and nothing depends on testpak: no manifest in
//! this workspace names it in a dependency table of any kind. Production
//! never depends on its judge. What this package asks for, and why, is the
//! README's; which versions, the workspace manifest's.

pub mod descriptor;
pub mod fault;
pub mod muterprater;
pub mod oracle;
pub mod properties;
pub mod report;
pub mod runner;

#[path = "03_judge/mod.rs"]
pub mod judge;

pub use judge::{
    ARTIFACT_MUTATIONS, ArtifactMutation, ArtifactStructure, CauseRow, DeclaredStructure,
    ImplPosture, ImplementationStructure, LaneOwnership, RenderVerdict, StructuralDisagreement,
    StructuralVerdict, cause_identities_in, judge_declared_order, judge_structure, mutated,
    selection_order_in, structure_of,
};

#[cfg(test)]
mod laws;
