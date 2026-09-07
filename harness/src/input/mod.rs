#![doc = include_str!("README.md")]

mod decode;
mod encode;
mod types;

pub use decode::read;
pub use encode::pack;
pub use types::{
    BoundInput, INPUT_CASE_TAG, INPUT_FORMAT_VERSION, InputBinding, InputCaseId, InputEnvelope,
    InputLimits, InputProfile, InputRefusal,
};
