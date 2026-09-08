#![doc = include_str!("README.md")]

mod conclude;
mod types;

pub(crate) use types::first_difference;

pub use types::{
    ByteDifference, VECTOR_PACK_MAGIC, VECTOR_PACK_VERSION, VectorDisagreement, VectorEntry,
    VectorPack, VectorPackRefusal, VectorSubject, VectorVerdict,
};
