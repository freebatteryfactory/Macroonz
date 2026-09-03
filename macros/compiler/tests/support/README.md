# Compiler test support

This home owns mechanics that several compiler integration crates must execute identically without turning those mechanics into a product API.

The Rustc specimen road writes one caller-supplied source beneath Cargo's target-owned temporary directory, invokes stable Rust 1.98 with caller-supplied extra arguments, executes an admitted binary, and removes its exact scratch root whether observation succeeds or refuses.

Callers retain the source, arguments, expected compile posture, and every behavioral or diagnostic assertion.
Dependency-bearing Cargo specimens remain with the claim that requires their separate package boundary.

`captured_tokens.rs` owns the one pre-order walk over captured token trees that the recipe slice's span finders and the declared-magnitude route census both read.

`attribute_specimens.rs` owns the lawful trial, mutation, and benchmark declaration bodies that the attribute-road and descriptor-content lanes drive.

Each integration crate is compiled on its own, so a crate includes exactly the files it consumes through `#[path]` rather than one door that would leave unused mechanics behind the warning wall.

