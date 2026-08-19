#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! [`types`] declares the shape of a row and nothing else. [`operator_families`]
//! and [`swap_pairs`] are the two families whose entries have landed. The
//! `capsules/` seat carries only its own README, which states the one act that
//! authors an entry there.

pub mod operator_families;
pub mod swap_pairs;
pub mod types;
