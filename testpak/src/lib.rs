//! `threadpak-testpak` is ThreadPak's testing harness: property-based,
//! descriptor-driven, mutation-pressured, and standalone — the library's one
//! inherited dependency is `arbitrary`.
//!
//! A verdict here is always claim-specific and method-specific.
//! "The permuted rendering was rejected by the string scan over these two
//! declared orders" is a verdict; "the derive works" is not one.
//! A failed check is a typed refusal value carrying its evidence and its
//! source location; the harness fails the way the machine refuses.
//!
//! # The instruments
//!
//! The homes below are order-free peers over one vocabulary: [`descriptor`]
//! owns the rows every producer writes into, [`report`] owns the records a
//! run leaves, [`oracle`] is the independence annex for claims where bytes
//! are the spec, [`runner`] turns descriptor tables into nextest trials,
//! [`properties`] carries the algebraic suites, [`muterprater`] is the
//! proof-pressure engine, and [`fault`] holds the refusing adapters.
//! Each home's README is its module page.
//!
//! Executable challenge material sits under `tests/`, where cargo requires
//! it to live; seed-packs sit under `corpus/`; performance reports under
//! `benches/`.
//!
//! # The standing seats
//!
//! Two numbered seats carry the pre-redesign machinery: [`plan`] and
//! [`judge`], whose three readings and mutation roster are the oracle's and
//! muterprater's seed material.
//!
//! # The dependency direction
//!
//! testpak reaches the machine and the generation services only from
//! `tests/`, as dev-dependencies, and nothing depends on testpak: no manifest
//! in this workspace names it in a dependency table of any kind. Production
//! never depends on its judge.

pub mod descriptor;
pub mod fault;
pub mod muterprater;
pub mod oracle;
pub mod properties;
pub mod report;
pub mod runner;

#[path = "00_plan/mod.rs"]
pub mod plan;

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
