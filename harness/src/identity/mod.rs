#![doc = include_str!("README.md")]

mod encode;
mod read;
mod types;

pub use encode::{encode_bytes, encode_length};
pub(crate) use read::addressed_body;
pub(crate) use types::BodyReader;
pub use types::{
    ContentAddress, DomainTag, HARNESS_IDENTITY_PROFILE, IdentityProfile, IdentityProfileVersion,
};
