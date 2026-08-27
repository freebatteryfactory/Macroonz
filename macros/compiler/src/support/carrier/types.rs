//! Carrier declarations.
use crate::identity::{self, Identity};
use crate::token::GeneratedTree;
#[path = "type_guard.rs"]
mod guard;
/// The full-width plan-keyed exported name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShellName {
    spelling: String,
}
/// A carrier-shell refusal.
#[must_use = "a shell refusal names why no carrier was rendered for this pair"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellError {
    /// Plan and assembly stand over different declarations.
    NotOneDeclaration {
        /// The assembly declaration.
        stated: Identity<identity::CapturedDeclaration>,
        /// The plan declaration.
        planned: Identity<identity::CapturedDeclaration>,
    },
    /// The rendered tree exceeds its magnitude.
    TreeUnbounded {
        /// The bound.
        bound: usize,
        /// The observed count.
        observed: usize,
    },
}
/// The rendered inert exported shell.
#[must_use = "a carrier is the exported definition a consumption target invokes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SupportShell {
    name: ShellName,
    tree: GeneratedTree,
}
/// The carrier request kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SupportCarrier;
