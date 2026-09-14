//! Public benchmark composition preserves real execution and independent adverse outcomes.

#[path = "../../../examples/benchmark_workflow/declaration.rs"]
mod declaration;
#[path = "../../../examples/benchmark_workflow/specimen.rs"]
mod specimen;
mod execution;
#[cfg(all(feature = "native-tooling", any(unix, windows)))]
mod retention;
mod fixture;
mod presentation;
mod presentation_archive;
