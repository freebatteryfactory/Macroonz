#![doc = include_str!("README.md")]

mod compiler;
mod coverage;
mod mutation;
mod mutation_refusal;
mod process;
mod publication;
mod refusal;
mod retention;
mod storage;

pub use compiler::{native_compilation, pending_compilation};
pub use coverage::{coverage_preflight_error, coverage_profile_error, native_coverage};
pub use mutation::{native_mutation, pending_mutation};
pub use mutation_refusal::mutation_error;
pub use process::native_process;
pub use publication::{
    bake_error, bake_output, publication_destination_check, publication_destination_error,
    publication_format_error, publication_format_output, publication_format_run,
    publication_staging_error, publication_staging_run,
};
pub use refusal::{compiler_error, read_back_error};
pub use retention::{benchmark_retention_refusal, retention_refusal, stored_run};
pub use storage::storage_error;
