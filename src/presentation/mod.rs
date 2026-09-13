#![doc = include_str!("README.md")]

mod admission;
mod benchmark;
mod clock;
mod context;
mod corpus;
mod coverage;
mod descriptor;
mod descriptor_refusal;
mod diagnostic;
mod historical;
mod legacy;
mod mutation;
mod network;
#[cfg(feature = "native-tooling")]
mod native;
mod oracle;
mod outcome;
mod path;
mod proposal;
mod record;
mod reduction;
mod render;
mod replay;
mod types;
mod value;
mod workflow;

pub use admission::{archive_refusal, input_refusal};
pub use benchmark::{
    archived_benchmark, benchmark, benchmark_archive_refusal, benchmark_refusal, benchmark_verdict,
};
pub use corpus::{seed_pack, seed_pack_refusal};
pub use coverage::{
    coverage_admission, coverage_admission_refusal, coverage_corpus, coverage_preflight_refusal,
    coverage_profile_refusal, coverage_readiness, coverage_result,
};
pub use diagnostic::compiler_diagnostic;
pub use historical::{archived_capsule, archived_run, archived_trial};
pub use legacy::{legacy_comparison, legacy_join_refusal, legacy_record};
pub use mutation::{
    archived_backend, archived_interpreted, archived_mutation, archived_mutation_run,
    archived_parity, archived_projection, backend_manifest, backend_manifest_refusal,
    backend_reading, backend_reading_refusal, mutation_discovery, mutation_record, mutation_run,
};
#[cfg(feature = "native-tooling")]
pub use native::{
    bake_error, bake_output, benchmark_retention_refusal, compiler_error, coverage_preflight_error,
    coverage_profile_error, mutation_error, native_compilation, native_coverage, native_mutation,
    native_process, pending_compilation, pending_mutation, publication_destination_check,
    publication_destination_error, publication_format_error, publication_format_output,
    publication_format_run, publication_staging_error, publication_staging_run, read_back_error,
    retention_refusal, storage_error, stored_run,
};
pub use network::{
    network_replay_exhaustion, network_replay_incomplete, network_replay_join_refusal,
    network_reproduced_replay, network_reproduction, network_transcript,
    network_transcript_refusal,
};
pub use oracle::{
    archived_oracle, compilation_comparison, compilation_observation, compiled_comparison,
    compiled_observation,
};
pub use proposal::archived_proposal;
pub use record::{capsule, run, trial};
pub use reduction::{archived_reduction, reduction};
pub use replay::{replay_comparison, replay_join_refusal};
pub use types::Presentation;
pub use workflow::{input_run, suite_selection_refusal};
