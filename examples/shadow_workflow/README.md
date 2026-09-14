# Ordinary execution and shadow exploration

The same caller-owned increment operations compile against standard-library or Loom synchronization through one generated import module.
An independently authored check requires two completed increments to leave exactly two.
Separate load/store operations satisfy that check sequentially but can lose an update concurrently; the caller's separate repair uses one atomic increment.

## Run the adopter

Copy this directory's Rust files, preserving the `synchronization` directory, into a new caller-owned directory.
Copy [consumer.toml](consumer.toml) there as `Cargo.toml` and replace `/absolute/path/to/Macroonz` with the exact source checkout being evaluated, using forward slashes on Windows.
Run these commands from that directory on a target supported by the [preemption owner](../../harness/src/preemption/README.md):

```text
cargo +1.98.1 generate-lockfile
cargo +1.98.1 run --locked --bin shadow-workflow
```

The ordinary build prints:

```text
ordinary: sequential check and concurrent repair agree
```

Build the same program with the shadow configuration selected for the adopter binary:

```text
cargo +1.98.1 rustc --locked --bin shadow-workflow -- --cfg loom
```

Run `target/debug/shadow-workflow` directly, or `target\debug\shadow-workflow.exe` on Windows.
If Cargo output was explicitly redirected, use that target directory instead.
This execution prints:

```text
shadow: sequential check holds; lost update caught; repair holds within bounds
```

The shadow program requires the sequential control to hold, a concurrent lost update to be caught, and the independently authored atomic repair to hold under the same check and bounds.
Unexpected outcomes return a nonzero exit with the actual result.
Running plain `cargo run` again selects the ordinary configuration, so execute the binary produced by the shadow build directly.

## Ownership and limits

[increment.rs](increment.rs) owns the counter operations, while [observation.rs](observation.rs) independently states the expected result.
Both builds use those same files and the imports from [synchronization](synchronization/README.md).
The adopter declares its direct Loom dependency at the harness's exact pin because the generated shadow imports name that dependency; Macroonz does not declare an adopter's dependencies.
The manifest is an executable consumer template, not an additional Macroonz workspace package.
This example observes an explicitly selected checkout, not registry-package or release provenance.

The [shadow declaration owner](../../macros/compiler/src/descriptor/shadow/README.md) owns import selection, and the preemption owner owns exploration and its result distinctions.
The ordinary run checks one sequential execution and one concurrent repaired execution; it makes no claim that an ordinary scheduler will expose the broken operation.
The shadow run explores this two-worker model with at most two preemptions and a branch budget of 1,000 per execution.
A held result is bounded model evidence, not a proof for every thread count, program or target.
An unavailable or unresolved backend remains an incomplete result and makes this example fail visibly.

For an unknown selected name, repair the declaration against the shadow owner's roster.
For an unresolved `loom` path or incompatible type, check the physical dependency binding and exact pin in the adopting manifest.
For an incomplete backend result, inspect that result and the preemption owner's target and bounds contract before changing the check.
