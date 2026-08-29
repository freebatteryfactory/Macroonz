# Macroonz 0.2 coverage-documentation routing cure

This Git-tracked receipt records the Q2 correction earned by the accepted docs-blind coverage census.

## Authority

- Campaign branch: `codex/macroonz-0.2-release-line`.
- Entering census receipt: `.durafx/qualification/0.2-q1-docs-blind-coverage-20260829/receipt.md` at `d330b0f`.
- Documentation correction: `2613440d563b8e036afad1b817545c11b6cba6ff`.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Toolchain: stable Rust 1.98.0 with Cargo 1.98.0.
- This pass performed no product-source, API, dependency, feature, package, example-source, or qualification-package change.
- This pass performed no push, merge, ref rewrite, publication, or registry mutation.

## Earned correction

The accepted docs-blind census proved that the published facade already supplies the complete stable-rustc coverage journey.
It also proved that the fuzz rustdoc's relative example link resolved beneath the harness package and returned HTTP 400.

Only `harness/src/fuzz/README.md` changed.
The correction:

- routes the runnable road to the current facade package's public example source;
- routes the required support layout to the same public package source;
- states that adding a dependency does not install another package's example target;
- names the required main-file, support-directory, and `harness`-feature execution context;
- retains the existing command and semantic claim without copying the example body;
- labels the `latest` route as current source rather than a frozen same-version permalink.

The final public routes are:

- `https://docs.rs/crate/macroonz/latest/source/examples/rustc_coverage.rs`;
- `https://docs.rs/crate/macroonz/latest/source/examples/support/`.

## Stable qualification

- `RUSTDOCFLAGS=-Dwarnings cargo +1.98.0 doc -j1 -p macroonz-harness --all-features --no-deps --locked` passed.
- The rendered `target/doc/macroonz_harness/fuzz/index.html` contained each final route exactly once.
- The rendered page contained the superseded `../../../examples/rustc_coverage.rs` route zero times.
- Both final public routes returned HTTP 200 during owner-agent and coordinator observations.
- GitHub Markdown rendering retained both absolute routes and the runnable command during the owner-agent observation.
- `cargo +1.98.0 fmt --all -- --check` passed.
- `git diff --check d330b0f..2613440` passed.
- The coordinator read the complete owning README and its module door and independently reran the docs, rendering, HTTP, formatting, diff, and identity checks.

## Disposition

- Reuse-first disposition 2 is closed for this coverage-road friction.
- No convenience API, new public path, semantic owner, dependency, feature, package, or duplicated documentation implementation was earned.
- The public route now explains the one unavoidable Cargo fact instead of hiding it: dependency installation and example-target ownership are distinct.
- The runnable behavior itself was already accepted by Q1 and was not reopened by this docs-only lot.

## Ceilings

- This receipt proves local Rust 1.98 rustdoc rendering and live public URL availability on the observation date.
- The unversioned `latest` routes intentionally point to the current facade package source and do not claim historical same-version pinning.
- Future published 0.2 docs.rs rendering, hosted link checking, cross-platform execution, registry delivery, and human release acceptance remain unproved.
