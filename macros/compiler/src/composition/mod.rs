#![doc = include_str!("README.md")]

mod establish;
mod type_contract;
mod types;

pub use types::{
    CompositionIssueLimit, CompositionRoot, CompositionRootDeclaration, CompositionRootIssue,
    DESCRIPTOR_KINDS, DescriptorKind, DescriptorProvider, DescriptorProviderLimit,
};
