//! `threadpak-testpak` is the qualification plane: hostile execution against
//! the machine and its tooling, and the denominators every verdict is stated
//! over.
//!
//! A verdict here is always claim-specific and method-specific.
//! "The permuted rendering was rejected by the string scan over these two
//! declared orders" is a verdict; "the derive works" is not one.
//!
//! # The seats
//!
//! The plane is a numbered waterfall of seats, mapped by `#[path]` exactly as
//! the machine maps its bands, so a seat's number is visible in the tree and
//! never in a module name.
//! A seat holds a MODULE only once it holds something: the declarations below
//! are the source-home population, and a reserved seat's own directory carries
//! one README stating its question, its filling condition, and its nonclaims.
//! No module is declared at a reserved coordinate, so nothing here can reach
//! one.
//!
//! A seat whose material is genuinely all executable challenge is seated under
//! `tests/`, where cargo requires it to live. Material sits where its content
//! is, never where a symmetrical tree would look tidier.
//!
//! # The dependency direction
//!
//! testpak depends inward — on `threadpak`, on `threadpak-macroc`, and on
//! `threadpak-macros` — and nothing depends on testpak: no manifest in this
//! workspace names it in a dependency table of any kind. Production never
//! depends on its judge.

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
