# Compiler test support

This home owns mechanics that several compiler integration crates must execute identically without turning those mechanics into a product API.

The Rustc specimen road writes one caller-supplied source beneath Cargo's target-owned temporary directory, invokes stable Rust 1.98 with caller-supplied extra arguments, executes an admitted binary, and removes its exact scratch root whether observation succeeds or refuses.

Callers retain the source, arguments, expected compile posture, and every behavioral or diagnostic assertion.
Dependency-bearing Cargo specimens remain with the claim that requires their separate package boundary.

