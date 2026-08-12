#![doc = include_str!("README.md")]

mod establish;
mod type_contract;
mod types;

pub use types::{
    CompositionRoot, CompositionRootDeclaration, CompositionRootIssue, DESCRIPTOR_KINDS,
    DescriptorKind, DescriptorProvider,
};
