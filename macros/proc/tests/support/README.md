# Proc test support

This home owns mechanics that several proc integration crates must execute identically without turning those mechanics into a product API.

`scratch.rs` owns the repository-root location, exclusive scratch-root custody beneath Cargo's target-owned temporary directory, the complete TOML basic-string path escaper, the Rust 1.98 Cargo invocation with a scratch-owned target, and the refusal rendering of one failed subprocess.
Its explicit-target road separates archive assembly at the checkout manifest from compilation of the extracted delivery, and places a Nextest verb before that tool's manifest argument.
Standalone adopters seed their lockfile from the repository and use Cargo's workspace-only update to reconcile the fixture package while retaining already locked external dependencies.
Subsequent compilation is locked and offline; an offline cache alone is not dependency-graph custody.
Each lane keeps its own manifests, producer and consumer sources, expected outcomes, and assertions, and supplies its own label to the scratch owner.

`capture-observer/` is a separate Rust-required fixture package that observes span custody from inside a real proc host, not a shared module of this crate set.
