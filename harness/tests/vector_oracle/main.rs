//! External observations of the oracle methods, organized by the claim each method can make.
//!
//! Each module authors its control independently of the operation under judgement and states the evidence ceiling where provenance remains outside the harness.

#[path = "../support/archive_process.rs"]
mod archive_process;

mod archive;
mod artifact_declarations;
mod compiled_behavior;
mod golden_vectors;
mod identity_transcript;
