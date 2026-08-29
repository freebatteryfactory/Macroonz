# Macroonz 0.2 docs-blind custom-macro census

This Git-tracked receipt records a subject-owned proc-macro journey against the published Macroonz compiler 0.1 surface.

## Authority

- Campaign branch: `codex/macroonz-0.2-release-line`.
- Entering repository snapshot: `a6fea1f`.
- Published dependency: `macroonz-compiler = "=0.1.0"` from crates.io with default features disabled.
- Published compiler archive SHA-256: `6896202A7ECF4F4A8017029C364D707D21C6D02AFCB29A42322BE655D37C0D74`.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Toolchain: stable Rust 1.98.0 with Cargo 1.98.0.
- Scratch home: `target/qualification/0.2-q1-docs-blind-custom-macro-20260829`.
- This pass performed no product-source edit, API edit, dependency edit, feature edit, push, merge, ref rewrite, publication, or registry mutation.

## Docs-blind boundary

The actor received the public crates.io metadata and README, versioned docs.rs rustdoc, official Rust proc-macro documentation, compiler diagnostics, and the high-level subject-owned macro task.
The actor did not inspect the Macroonz repository source, internal tests, Git history, local registry source cache, docs.rs source view, or implementation overlays while solving the journey.

## Public navigation

- `https://docs.rs/macroonz-compiler/0.1.0/macroonz_compiler/` supplied the one-request and one-expansion contract plus the module map.
- `https://crates.io/api/v1/crates/macroonz-compiler/0.1.0/readme` supplied the decisive `host::expand` and `Request::over(...).render(...)` journey.
- `https://docs.rs/crate/macroonz-compiler/0.1.0/features` showed that `host` is the only opt-in feature.
- The public `kind`, `request`, `capture`, `output`, `door`, `expansion`, `plan`, `membership`, `role`, `diagnostic`, and `host` pages supplied every constructor and projection used by the witness.
- `https://doc.rust-lang.org/proc_macro/` and the Rust procedural-macro reference supplied the ordinary proc-macro package and function boundary.
- No first necessary semantic fact was absent from public documentation.
- The published README sketch was not copy-paste complete, so the actor assembled its undefined subject placeholders from public item pages.
- `Diagnostic` does not implement `Display`; its public `summary()` and `phase()` projections resolved the first assertion wrinkle without source access.

## Composition

The external workspace contains a normal `subject` package and a normal subject-owned `subject-macros` proc-macro package.
Only the proc-macro package activates the compiler's `host` feature.
The subject owns its tiny input grammar, canonical content, renderer, and ordinary Rust meaning.
Macroonz owns capture, request planning, render closure, expansion, diagnostic placement, and token emission.

The lawful invocation `declare_answer!(lawful)` emits this ordinary item:

```rust
pub const GENERATED_ANSWER: u64 = 42;
```

The downstream subject invokes the generated constant and returns `42`.
A direct callable compiler test uses `TextCapture` and the same compile function, then proves equality of canonical bytes and inspected generated structure at the common boundary.

## Controls

- Lawful control: the proc crossing emits its one planned declaration-site unit and the downstream test observes `42`.
- Direct crossing: the callable compiler and proc-macro roads agree on the planned membership, canonical bytes, and inspected generated tree.
- Hostile reversal: `declare_answer!(omit)` produces no planned unit and fails at the macro invocation with `subject: the renderer did not produce the planned unit: the renderer materialized no unit at all`.
- Non-vacuity: removing the macro invocation while referring to `GENERATED_ANSWER` fails with ordinary Rust error `E0425`.

## Exact custody

- `Cargo.lock`: 3,503 bytes, SHA-256 `148A015113830A73FA65193BEE5FA15A1EE52FACB91A671DFF69819A4E05C83A`.
- `Cargo.toml`: 305 bytes, SHA-256 `95F038347722B963FA3E82E785943F3009B735575388E95B0B75AB7A80AC53DC`.
- `rust-toolchain.toml`: 86 bytes, SHA-256 `E879701472A698533F43F54989BBF5F3101EB0503472159D32633F3BB8A13FDF`.
- `subject-macros/Cargo.toml`: 261 bytes, SHA-256 `2C488B3AD54B31FA4C33BF361D91E492A76BC6FCD6FFDF7A429C24E70F9EC0D2`.
- `subject-macros/src/lib.rs`: 3,251 bytes, SHA-256 `D4412080DC254586D6A162E123D0AC874A61BF061E621055554FC947C3089E37`.
- `subject/Cargo.toml`: 441 bytes, SHA-256 `B884A4C9F584643E14FF49A23622CA61FE28169ED66AE80F3906F364A17391D8`.
- `subject/src/bin/hostile.rs`: 118 bytes, SHA-256 `7DD8C56EEAF637AB4C942517CA07DF623601698A411A7FE6790606790A82A240`.
- `subject/src/bin/non_vacuity.rs`: 88 bytes, SHA-256 `E3E3D72A330A03FBA4C5288BF3F4F4C0661DEB6A2A18737F4A46C2BD2786AF7A`.
- `subject/src/lib.rs`: 407 bytes, SHA-256 `FBCD1C0CE2AF8C7AA106B6D88FCABA864222FE9B73F351AC0A759C6B79D4D7E3`.
- The nine non-build files totaled 8,460 bytes.
- The sealed scratch tree contained 761 files and 96,766,637 bytes before cleanup, including disposable Cargo output.
- `specimen.md` retains every authored input as one readable non-Cargo packet and has SHA-256 `7F6361B99D9CF44C6A4669A2E9A3B1C69DD40349C229FE27D76C1F00F4A49B12`.
- The lockfile hash and published archive checksum identify the immutable registry graph.

## Stable qualification

- `cargo +1.98.0 fmt --all -- --check` passed.
- `cargo +1.98.0 test --workspace --locked` passed three of three tests.
- `cargo +1.98.0 clippy --workspace --all-targets --locked -- -D warnings` passed.
- `cargo +1.98.0 tree --workspace --locked -e features` confirmed compiler 0.1.0 and the `host` feature only.
- `cargo +1.98.0 check -p subject --bin hostile --features hostile --locked` failed as required with the Macroonz rendering refusal at the invocation.
- `cargo +1.98.0 check -p subject --bin non-vacuity --features non-vacuity --locked` failed as required with `E0425`.
- The coordinator read all nine files and the complete lockfile and independently reran every final lawful and refusal command.

## Disposition

- Product-semantic disposition: 1, existing composition solves.
- No constructor, public path, deterministic projection, checked join, semantic primitive, or owner is missing for the narrowed compiler and proc crossing.
- No product API, dependency, feature, package, first-party proc entry, or compiler-harness edge is earned.
- The bank's full multi-role algebra was unnecessary to answer this question and remains untested rather than authorized.
- The incomplete README sketch and high assembly cost remain Q2 documentation ergonomics input; whether one smaller public composition should be promoted is a documentation-owner decision, not evidence for product semantics.

## Ceilings

- Only exact registry compiler 0.1.0 with `host` enabled in the proc member was tested.
- Only native Windows `x86_64-pc-windows-msvc` and debug compilation were tested.
- Multi-role, test, bench, mutation, publication, carrier, doubled, foreign, misdelivered, harness-parity, packaged-consumer, hosted, Linux, macOS, and Wasm claims remain unproved.

## Custody boundary

The readable specimen packet, exact hashes, lockfile identity, commands, and failure text retain the accepted observation without tracking another Cargo workspace.
The scratch project and its build products may be removed after this receipt and specimen packet are committed and verified.
