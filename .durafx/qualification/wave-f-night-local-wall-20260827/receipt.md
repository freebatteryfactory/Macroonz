# Wave F night local wall receipt

This Git-tracked receipt retains the cumulative stable local qualification that followed the night-shift observer and evidence commits.

## Denominator

- The qualified source snapshot was `21a57336bcef06d6646cf2da355980c6c03e44be` on `codex/macroonz-repository-completion` with a clean tracked worktree.
- The host was Microsoft Windows 11 Home build 26200 on `x86_64-pc-windows-msvc`.
- The toolchain was stable Rust 1.98.0 with LLVM 22.1.8 and Cargo 1.98.0.
- `Cargo.lock` had SHA-256 `471BD8BF8BAA28392CA4B14CB49C175877E6D8E06CD778E20D1AD71C21E3D586`.
- No nightly toolchain, CI, cross-host execution, targeted mutation, publication, push, merge to `main`, or ref movement entered this denominator.

## Observation

- `cargo +1.98.0 check -j1 --workspace --all-targets --all-features --locked` passed.
- `cargo +1.98.0 clippy -j1 --workspace --all-targets --all-features --locked -- -D warnings` passed.
- `cargo +1.98.0 nextest run -j1 --workspace --all-features --locked --no-fail-fast` passed 411 of 411 directly enumerated tests across 60 binaries, with nine intentional child or ignored tests outside direct nextest enumeration.
- `cargo +1.98.0 fmt --all -- --check` passed.
- `cargo deny --workspace check` passed advisories, bans, licenses, and sources.
- `RUSTDOCFLAGS=-Dwarnings cargo +1.98.0 doc -j1 --workspace --all-features --no-deps --locked` passed.
- `cargo +1.98.0 check -j1 --workspace --all-features --target wasm32-unknown-unknown --locked` passed.
- `cargo +1.98.0 run -j1 --example rustc_coverage --features harness --locked` passed without retaining a new example scratch directory.

## Evidence ceiling

- The local wall reports this Windows source snapshot and does not establish hosted, physical-Linux, macOS, ARM64, registry, publication, or as-retrieved behavior.
- The separately executed ignored schedule campaigns, stable coverage censuses, adopter-cost runs, and package observation retain their own receipts and denominators.
- The descriptor empty-composition diagnostic contradiction remains an owner decision and is not repaired or hidden by this green wall.
