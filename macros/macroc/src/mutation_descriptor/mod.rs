#![doc = include_str!("README.md")]

mod capture;
mod encode;
pub(crate) mod render;
mod types;

pub use capture::{MUTATION_ATTRIBUTE, captured_mutations};
pub use types::{
    GeneratedMutationFamily, MutationDeclaration, MutationDeclarationCause,
    MutationDeclarationRefusal, MutationModuleName, MutationOwnerFact, OwnerClaimDeclaration,
    OperatorPermissionDeclaration,
};
