# Macroonz 0.2 exact compile-contract implementation

This Git-tracked receipt seals the first bounded implementation lot authorized by the compile-contract owner ruling.

## Authority and history

- Campaign branch: `codex/macroonz-0.2-release-line`.
- Owner-ruling commit: `9fb0d75757ecfefd1761644efb578e2d293a33e7`.
- Implementation commit: `a108244ac1d1b6c4681ea8515dd654f77d5ebe19`.
- Independent-QA correction commit: `56187b3260ce9460d33a86b63991e63e297ae399`.
- This lot authorizes no push, merge, ref rewrite, publication, registry mutation, or later self-host implementation.

## Product movement

The existing `oracle::compiled` owner now carries an additive exact compilation road beside its unchanged coarse read-back road.

- `RustcErrorCode` informs only `E` followed by exactly four ASCII digits.
- `RelativeSourcePath` informs only nonempty slash-separated logical paths relative to a declared challenge root.
- `SourcePosition` informs one-based line and column coordinates.
- `PrimarySourceSpan` preserves rustc's one-based inclusive lines, one-based inclusive start column, and one-based exclusive end column.
- `DiagnosticAnchor` binds one stable rustc error code to one normalized primary span without claiming the compiler emitted no other diagnostic.
- `DeclaredCompilation` and `ObservedCompilation` separately state acceptance or one exact refusal anchor.
- `compared_compilation` distinguishes acceptance posture, error-code disagreement, and primary-span disagreement.
- `CompilationVerdict::concluded` reuses the existing `FailureClass::OracleDisagreement`, oracle cause family, and ordinary report conclusion rail.

Every new value has private representation and an informing constructor.
The nine pre-existing public declaration blocks in `compiled/types.rs` were independently compared against the owner-ruling snapshot and remained byte-equivalent.
No existing exhaustive enum gained a variant and no 0.1 meaning was reinterpreted.

## External host crossing

The existing `compile_refusals` integration target owns the effectful package and compiler work.
It creates disposable package-shaped challenges, invokes stable Cargo 1.98 with locked and offline structured JSON output, normalizes physical paths beneath the declared challenge root, and submits only the established typed observation to the product oracle.

The host does not filter diagnostics by the declared error code.
It requires exactly one relevant primary anchor at the independently declared challenge locus.
Zero anchors, multiple anchors, missing codes, malformed structured output, outside-root paths, unavailable toolchains, and spawn failures keep distinct refusal or infrastructure standing.
The lawful challenge carries only a neutral source locus; only the hostile challenge carries the independently declared E0308 expectation.

## Independent controls

- Lawful compilation conforms to an acceptance declaration.
- A deliberate stable E0308 refusal conforms at its exact normalized primary span.
- Unexpected acceptance and unexpected refusal deviate.
- The same span under a different error code and the same code at a different span deviate independently.
- Relocated scratch roots produce the same logical source identity.
- Empty, rooted, backslash-bearing, traversal, current-directory, empty-segment, and ambiguous physical paths refuse.
- Zero, multiple, code-less, and ambiguous relevant anchors cannot silently become observations.
- A same-line reversed column refuses while a later-line span may carry a smaller end column.
- Missing toolchain and process-spawn failure remain host standing rather than compiler refusal.
- Malformed Cargo JSON remains capture infrastructure rather than a subject verdict.
- Three external compile-fail witnesses prove callers cannot bypass the diagnostic-code, source-position, source-path, span, or anchor informing boundaries.

## Independent qualification

All commands used `CARGO_TARGET_DIR=target/qualification/0.2-compile-contract-impl-20260829` and ran serially from the repository root.

- `cargo +1.98.0 fmt --all -- --check` passed.
- `cargo +1.98.0 test -p macroonz-harness --test vector_oracle --all-features --locked` passed 12 of 12 tests.
- `cargo +1.98.0 test -p macroonz-harness --test compile_refusals --all-features --locked` passed 7 of 7 tests, including the real package-shaped compiler crossings and all external compile-fail witnesses.
- `cargo +1.98.0 clippy -p macroonz-harness --all-targets --all-features --locked -- -D warnings` passed.
- `RUSTDOCFLAGS=-D warnings cargo +1.98.0 doc -j1 -p macroonz-harness --all-features --no-deps --locked` passed.
- `cargo +1.98.0 test -p macroonz-harness --doc --all-features --locked` passed with zero doctests present.
- `cargo +1.98.0 package -p macroonz-harness --locked` packaged and verified 372 files totaling 1.9 MiB and 439.5 KiB compressed.
- `git diff --check 9fb0d75..56187b3` passed.
- The changed-file identity and forbidden-hatch scan found no personal path, personal name, `unsafe` block, `#[allow]`, or `#[expect]`.
- The manifest and lockfile diff was empty.

The toolchain was `rustc 1.98.0 (88d9e12ae 2026-08-18)` with LLVM 22.1.8 and `cargo 1.98.0 (797e8a9bc 2026-08-05)` on `x86_64-pc-windows-msvc`.

## Graph and custody

No package, dependency, feature, manifest edge, lockfile entry, qualification crate, facade edge, generated identity framing, or report failure class changed.
The effectful host remains external test code and creates no product observation channel beyond the public typed values.

At the sealed point, the exact disposable target contained 14,837 files totaling 4,956,476,985 bytes.
It may be deleted only after this receipt is committed and independently verified.

## Ceiling and next obligation

This receipt establishes one local Windows stable-Rust exact diagnostic contract and its package-shaped host crossing.
It does not establish Linux, macOS, Wasm, hosted, or registry-delivered 0.2 behavior.
It does not yet establish the complete generalized compile-contract road through checked runner admission, `HostTrialRecord`, Muterprater, generated API consumers, or the one-surface facade.
It does not add a diagnostic roster, preserve complete rustc output as identity, or implement the future Macroonz self-hosting goal.

The next Section A lot must compose this exact oracle result through the existing runner, report, and Muterprater owners and prove one generated API consumer without moving compiler-emission semantics into the harness or harness-judgment semantics into the compiler.
