//! `threadpak-testpak`: the qualification plane.
//!
//! The plane orchestrates hostile execution against the machine and its tooling
//! and carries the denominators every verdict is stated over. **A verdict here
//! is always claim-specific and method-specific**: "the permuted rendering was
//! rejected by the string scan over these two declared orders" is a verdict;
//! "the derive works" is not one.
//!
//! # The seats
//!
//! The plane is a numbered waterfall of seats, mapped by `#[path]` exactly as
//! the machine maps its bands, so a seat's number is visible in the tree and
//! never in a module name. A seat holds a MODULE only once it holds something:
//! the module declarations below are the source-home population, and each reserved seat's own
//! directory carries one README stating its question, its filling condition, and
//! its nonclaims. No module is declared at a reserved coordinate, so nothing
//! here can reach one.
//!
//! Two seats are open here as source homes — `00_plan` and `03_judge`. Two more
//! are occupied by test suites rather than source (the mutation seat, and
//! `tests/compile_refusals.rs` with its fixtures), because their material is
//! genuinely all executable challenge and cargo requires it to live under
//! `tests/`. They are seated where their content is, not where a symmetrical
//! tree would look tidier.
//!
//! # The dependency direction is the whole point
//!
//! testpak depends inward — on `threadpak`, on `threadpak-macroc`, and on
//! `threadpak-macros` — and **nothing depends on testpak**. Production never
//! depends on its judge, and the `no-core-tooling-edge` check enforces that
//! absence at the root manifest under every Cargo edge kind.

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
pub use plan::RedTwinLedger;

#[cfg(test)]
mod laws;
