#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! `types.rs` declares the entry, storage reference, refusal, and caller-owned
//! sink. Its invariant nucleus is `type_guard.rs`.

mod types;

pub use types::{ReplayCapsuleEntry, ReplayDepotRefusal, ReplayDepotSink, StoredReplayEntryRef};
