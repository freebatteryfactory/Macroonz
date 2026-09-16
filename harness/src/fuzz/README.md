# fuzz

This home turns stable rustc source coverage into a bounded search signal while leaving Macroonz's existing corpus, generation, reporting, reduction, and replay owners in charge of their own semantics.

## Claim

[`CoverageCampaign`] declares the population, subject revision, coverage interpretation, and every resource ceiling before a host is consulted.
[`preflight_ready`] joins that declaration and its selected execution target to the identity of the exact stable Rust 1.98 compiler that owns the matching LLVM tools.
The resulting [`ReadyPreflight`] is the only door into execution.

`InstrumentedTarget::declared` selects that compiler's host triple, while `InstrumentedTarget::for_target` carries an explicit caller-declared execution triple.
The compiler host remains the LLVM discovery coordinate even when the execution target differs.
Neither target selection authenticates the executable's compilation provenance or establishes that this host can execute it.
`ReadyPreflight` exposes the selected compiler path, its reported release, host, sysroot and LLVM version, the derived tool paths and their exact shared version string.
Its retained request keeps the original target selection, arguments, source-root declarations and scratch path beside the canonical roots established by preflight.
Only the pinned release and matching LLVM versions establish readiness; another release returns `PreflightIncomplete::RustcRelease` rather than acquiring an unqualified compatibility claim.

[`observe_rustc_profile`] owns the join from exact candidate bytes to one supervised process outcome and its canonical coverage observation.
The caller cannot substitute bytes, a campaign, a target, or a toolchain after that observation exists.
[`CoverageCorpus`] accepts only a joined result from its own qualified standing and mints [`InterestingBytes`] only when a successful execution adds a previously unseen point.
An adopter cannot manufacture that admission directly.

Coverage points use the caller-declared logical source root and paths relative to its canonical physical root.
Moving equivalent source between physical roots therefore preserves coverage identity while absolute checkout paths never become novelty.
`CoverageSourceRoots` admits a nonempty collection with distinct logical names and nonoverlapping declared paths.
`RustcProfileRequest::mapped` declares multiple roots, and preflight repeats admission after filesystem canonicalization.
`read_lcov_mapped` requires each source record, including a record with no executed points, to match exactly one normalized root.
The single-root request and reader entrances retain the same identity and refusal rules.

## Ownership

Stable rustc and its matching `llvm-profdata` and `llvm-cov` binaries supply source-coverage mechanism.
The caller-supplied supervisor supplies deadline and operating-system resource policy and returns the typed execution class.
`preflight_ready_with` and `observe_rustc_profile_with` pass every compiler, LLVM and target invocation to a declared executor.
That executor supplies complete tool output or a target classification and owns actual execution, deadlines and resource enforcement.
An executor failure remains separate from the owning preflight or profile refusal.
When it can still own a child, its returned `CoverageCaseCleanup` preserves the task-created directory until the executor has finished using it.
The ordinary entrypoints use the default compiler/LLVM process road and the existing target-supervisor callback; they do not acquire native deadlines from the presence of an executor seam.
This home supplies the qualified join, canonical point identity, novelty frontier, and deterministic neighboring-byte exploration.
The ordinary [`crate::corpus`] home owns retained seed packs and warm starts.
The ordinary [`crate::generate`] home owns generated streams, reduction, and replay.
The ordinary [`crate::report`] home owns execution reports and target facts.

The Macroonz-owned implementation is safe Rust and adds no native instrumentation library, FFI wrapper, general fuzz-engine dependency, nightly toolchain, feature, or Cargo package.

## Resource closure

[`CoverageBudgets`] closes target attempts, cumulative candidate bytes, per-case coverage-export bytes, accumulated canonical points, retained cases, and retained bytes.
A refusal spends only work already attempted and never partially advances the novelty frontier.
Each case has one task-created directory beneath the declared scratch root, and execution removes that exact directory after completed observation or semantic refusal.
An executor failure transfers explicit case cleanup rather than deleting files an unfinished child may still use.
Raw profiles and compiled campaign subjects remain disposable build output under `target/qualification`.

[`neighboring_inputs`] expands a retained input through a deterministic, caller-bounded sequence of safe byte operations.
Its budget selects an exact priority prefix and does not imply fairness among mutation families.

## Composition

[`read_lcov`] retains executed line and branch rows that the stable toolchain actually exports.
This is stable source-coverage guidance, not an AFL-style edge-coverage claim.
Explicit rustc branch-coverage modes remain outside the stable denominator.

[`compose_reduce_replay`] hands coverage-earned bytes to an already-qualified [`crate::generate::ReductionProbeBinding`].
Coverage guides search, while the existing failure fingerprint remains the authority for reduction and replay.

![Stable rustc coverage feedback][diagram-harness-fuzz]

Diagram source: `assets/diagrams/harness-fuzz.mmd` in this crate's source package.

[diagram-harness-fuzz]: ../../assets/diagrams/harness-fuzz.svg

## Runnable road

The current facade package source ships [`examples/rustc_coverage.rs`](https://docs.rs/crate/macroonz/latest/source/examples/rustc_coverage.rs) with its complete [`examples/support/`](https://docs.rs/crate/macroonz/latest/source/examples/support/) directory as a target that compiles a small Rust subject, proves coverage novelty and repeatability, retains a seed pack, and crosses one coverage-earned input into reduction and replay.
It uses the same `macroonz` package an adopter installs and no separate qualification package.
Adding `macroonz` as a dependency does not install the facade package's example target into the adopter's package.
Obtain the main file and complete support directory in that layout as an example target, and enable the dependency's `harness` feature before running this command from the package that owns the target.

```sh
cargo run --example rustc_coverage
```

## Limits

Fresh target and LLVM-tool processes trade throughput for simple isolation and exact per-case custody.
Timeout and resource-exhaustion values are supervisor classifications unless a separate operating-system crossing proves causal enforcement.
