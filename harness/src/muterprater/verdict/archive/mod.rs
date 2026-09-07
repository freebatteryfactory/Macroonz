#![doc = include_str!("README.md")]

mod encode;
mod types;

pub(crate) use encode::{activation_size, target_size, write_activation, write_target};
pub use types::{
    ArchivedActivation, ArchivedActivationReading, ArchivedMutationIdentity, ArchivedMutationSite,
    ArchivedMutationTarget,
};
pub(crate) use types::{read_activation, read_target};
