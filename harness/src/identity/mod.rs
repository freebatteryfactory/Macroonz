#![doc = include_str!("README.md")]

mod encode;
mod types;

pub use encode::{encode_bytes, encode_length};
pub use types::{
    ContentAddress, DomainTag, HARNESS_IDENTITY_PROFILE, IdentityProfile, IdentityProfileVersion,
};
