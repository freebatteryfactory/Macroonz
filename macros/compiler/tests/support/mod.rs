//! Shared mechanics for compiler integration tests.

mod rustc_specimen;

pub(crate) use rustc_specimen::observe_rustc;
