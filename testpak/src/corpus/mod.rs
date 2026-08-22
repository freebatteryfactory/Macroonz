#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! `types.rs` declares seeds, packs, their address family, and refusal vocabulary. Its child `type_guard.rs` owns construction and readers over private fields. `encode.rs` writes the canonical body and complete envelope, `decode.rs` reads that envelope under an expected population, and `warm_start.rs` projects admitted seeds into exact caller-supplied generation origins.

mod decode;
mod encode;
mod types;
mod warm_start;

pub use decode::read;
pub use encode::pack;
pub use types::{
    SEED_PACK_FORMAT_VERSION, SEED_PACK_TAG, SeedInput, SeedInputRefusal, SeedPack,
    SeedPackAddress, SeedPackRefusal,
};
pub use warm_start::warm_start;
