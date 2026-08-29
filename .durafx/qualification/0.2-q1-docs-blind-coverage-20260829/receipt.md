# Macroonz 0.2 docs-blind coverage census

This Git-tracked receipt records a docs-blind stable-rustc coverage journey against the published Macroonz 0.1 facade.

## Authority

- Campaign branch: `codex/macroonz-0.2-release-line`.
- Entering coordinator snapshot: `cdc9cd9`.
- Published dependency: `macroonz = "=0.1.0"` from crates.io with default features disabled and only `harness` enabled.
- Published facade archive SHA-256: `DBE80FF4EB844508DC9FF69A4938729C6CAD98F145C895D9A7D14FF57FB255FC`.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Compiler: Rust 1.98.0 commit `88d9e12ae178fab0fb5cc050a94da85685d449ea` with LLVM 22.1.8.
- Cargo: 1.98.0 commit `797e8a9bc`.
- Coverage tools: matching `llvm-cov` and `llvm-profdata` 22.1.8-rust-1.98.0-stable from the Rust toolchain sysroot.
- Scratch home: `target/qualification/0.2-q1-docs-blind-20260829`.
- This pass performed no product-source edit, API edit, dependency edit, feature edit, push, merge, ref rewrite, publication, or registry mutation.

## Docs-blind boundary

The actor received published packages, crates.io metadata and README content, versioned docs.rs rustdoc, public docs.rs package source, and official Rust 1.98 coverage documentation.
The actor did not inspect the Macroonz repository implementation, internal tests, receipts, Git history, bank overlays, or local registry source cache while solving the journey.

## Public navigation trail

1. `https://crates.io/crates/macroonz/0.1.0` identified the facade, Rust 1.98 posture, and the `harness` feature.
2. `https://docs.rs/macroonz/0.1.0/macroonz/` exposed the feature-gated `macroonz::harness` route.
3. `https://docs.rs/macroonz-harness/0.1.0/macroonz_harness/` routed to the `fuzz` owner.
4. `https://docs.rs/macroonz-harness/0.1.0/macroonz_harness/fuzz/index.html` documented the stable-rustc composition and advertised the runnable example.
5. That page emitted `../../../examples/rustc_coverage.rs`, which resolves to `https://docs.rs/macroonz-harness/examples/rustc_coverage.rs` and returned HTTP 400 during both owner and coordinator observation.
6. The actor recovered through `https://docs.rs/crate/macroonz/0.1.0/source/` and obtained the facade example plus its three support files from the versioned public package source.
7. `https://doc.rust-lang.org/1.98.0/rustc/instrument-coverage.html` confirmed the compiler instrumentation and matching-tool requirements.

## Exact witness input

- `Cargo.toml`: 291 bytes, SHA-256 `994D8CE9E1414CE1EB60A12DC3EAABE9D6271F76DC412EE8650A37BF00DFF1B8`.
- `Cargo.lock`: 4,855 bytes, SHA-256 `C05CCF4D23C268E69B2C31224F37B757A24EF1EFC21170CEF099CF7B01E26877`.
- `examples/rustc_coverage.rs`: 5,776 bytes, SHA-256 `C0B09366E9A4F5E58AB17ACEDD2EA5AC5890BA1D904276A942A2F9F4E0953B60`.
- `examples/support/rustc_coverage_host.rs`: 2,363 bytes, SHA-256 `1594A307DAFC5E14898A3A09DA3BA78F3E5ED24356F3B8053A1B2B995BC5D8D4`.
- `examples/support/rustc_coverage_replay.rs`: 5,209 bytes, SHA-256 `7DF3F4C4B10520A015C630A2DC028DED877B284B6FCBF2878E6EB2232D5623B4`.
- `examples/support/rustc_coverage_subject.rs`: 555 bytes, SHA-256 `F889B7D95749186538F3A164ACA207856BC7BD076B578DFD9D56EF41A7CC7EAD`.
- The six input files totaled 19,049 bytes.
- The sealed scratch tree contained 462 files, 77 directories, and 301,017,153 bytes before cleanup, including disposable Cargo output.
- Both complete runs left zero per-case qualification directories because the example cleaned its own transient run material.

The complete authored manifest was:

```toml
[package]
name = "macroonz-docs-blind-coverage"
version = "0.0.0"
edition = "2024"
rust-version = "1.98.0"
publish = false

[workspace]

[dependencies]
macroonz = { version = "=0.1.0", default-features = false, features = ["harness"] }

[lints.rust]
unsafe_code = "forbid"
warnings = "deny"
```

The host and replay files were byte-identical to the published facade example files.
The actor added only strict crate lints, four observation lines, an explicit final success, and the default-rustfmt line wrap shown by this exact unified diff against the published main and subject files:

```diff
diff --git a/examples/rustc_coverage.rs b/witness/examples/rustc_coverage.rs
index 46dc395..98d0f3f 100644
--- a/examples/rustc_coverage.rs
+++ b/witness/examples/rustc_coverage.rs
@@ -0,0 +1,3 @@
+#![forbid(unsafe_code)]
+#![deny(warnings)]
+
@@ -75,0 +79 @@ fn exercise(run: &Path) -> Result<(), ExampleFailure> {
+    println!("lawful: [0] succeeded and opened the coverage frontier");
@@ -83,0 +88 @@ fn exercise(run: &Path) -> Result<(), ExampleFailure> {
+    println!("hostile: [1, 2, 3] succeeded as a process and added distinct coverage");
@@ -91,0 +97 @@ fn exercise(run: &Path) -> Result<(), ExampleFailure> {
+    println!("non-vacuous: repeated [0] was known, not novel");
@@ -94 +100,4 @@ fn exercise(run: &Path) -> Result<(), ExampleFailure> {
-    replay::reduce_and_replay(&second, target_facts, campaign.revision()).map_err(ExampleFailure)
+    replay::reduce_and_replay(&second, target_facts, campaign.revision())
+        .map_err(ExampleFailure)?;
+    println!("retained: two novel seeds; hostile input reduced to exact replay [1]");
+    Ok(())
diff --git a/examples/support/rustc_coverage_subject.rs b/witness/examples/support/rustc_coverage_subject.rs
index 02521f9..66a1acd 100644
--- a/examples/support/rustc_coverage_subject.rs
+++ b/witness/examples/support/rustc_coverage_subject.rs
@@ -1,0 +2 @@
+#![deny(warnings)]
```

The lockfile and published archive checksums identify the complete immutable registry graph without creating a tracked qualification package.

## Stable qualification

- `cargo +1.98.0 fmt --all -- --check` passed after one default-rustfmt line wrap in scratch.
- `cargo +1.98.0 check --locked --all-targets` passed.
- `cargo +1.98.0 clippy --locked --all-targets -- -D warnings` passed.
- `cargo +1.98.0 run --locked --example rustc_coverage` passed twice.
- `cargo +1.98.0 tree -e features --locked` confirmed only the facade `harness` feature and no Loom or `preemption` feature.
- The coordinator read all six input files and the complete lockfile and reran the declared wall and executable twice after the owner agent completed.
- Both executions printed identical semantic observations.

## Semantic observations

- Lawful input `[0]` succeeded and opened the coverage frontier as interesting.
- Hostile input `[1, 2, 3]` succeeded at the process plane, carried the example's distinct semantic failure identity, added new coverage, and reduced to exact replay `[1]`.
- Repeated lawful input `[0]` was known rather than novel, proving the coverage admission was non-vacuous.
- Two novel seeds entered the seed pack.
- Both executions reproduced the same classification and exact replay result.

## Disposition

- Lowest truthful reuse-first disposition: 2, documentation or example routing missing.
- The stable coverage capability itself is composition-complete through the published facade.
- The broken runnable-example link and missing dependency-only file-layout guidance are the first exact friction.
- The narrow earned cure is to route public docs to the facade-owned example and state its execution context.
- This census earns no product API, new semantic owner, dependency, feature, package, coverage implementation, or qualification workspace.
- Any bank candidate for another fuzz framework or product primitive loses to the working published composition.
- The callable-compiler and future compile-contract docs-blind journeys remain untested.

## Ceilings

- Only exact registry `macroonz = 0.1.0` with the `harness` feature was tested.
- Only native Windows `x86_64-pc-windows-msvc` and one standalone instrumented Rust subject were tested.
- Three candidate executions prove the advertised semantic crossing, not throughput, scheduling quality, or scalability.
- No coverage-point total, branch or MC/DC denominator, durable on-disk depot, causal timeout, crash, cross-platform, Wasm, hosted, or real adopter-defect claim is established here.

## Custody boundary

The published facade archive, its immutable checksum, the complete manifest, the exact overlay, and the input hashes preserve the executable witness without retaining another Cargo island.
The scratch project and its build products may be removed after this receipt is committed and verified.
