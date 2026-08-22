#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! [`types`] declares the two static bank-row shapes. [`operator_families`] and
//! [`swap_pairs`] carry those authored families. [`capsules`] owns the exact
//! entry and caller-supplied storage seam used only by human admission.

pub mod capsules;
pub mod operator_families;
pub mod swap_pairs;
pub mod types;
