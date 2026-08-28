# Local Windows package refresh receipt

This Git-tracked receipt refreshes the local source-package plane after the Wave F fuzz-owner correction and targeted mutation closure.

## Denominator

- Source snapshot: `9884702` on `codex/macroonz-repository-completion` with a clean tracked worktree.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Toolchain: stable Rust 1.98.0 with Cargo 1.98.0.
- `Cargo.lock` SHA-256: `471BD8BF8BAA28392CA4B14CB49C175877E6D8E06CD778E20D1AD71C21E3D586`.
- Every command was offline, locked, and serial.
- Package-list hashes below are SHA-256 over the exact Cargo-emitted roster joined with one LF after every entry.

## Package rosters

| Package | Entries | LF-roster SHA-256 |
| --- | ---: | --- |
| `macroonz-compiler` | 300 | `C82EAE9E137E17309D25046C7014BB1F87B6C8B6B888CED8F38E3FDC24CBA802` |
| `macroonz-harness` | 364 | `D0386A63F2317BE55E52C4FEF2186754D3651D4F48467F1205E0142EF9EFE087` |
| `macroonz-macros` | 30 | `B8233E6B4A0E6035B34CD99D20A074FD282C87FAE28A5376373A6AF435D37B10` |
| `macroonz` | 19 | `95B3D6C59DA4539720C91118FE765DC17DFAB77137EC3E102792E7DD2E4EAF05` |

- The harness roster contains the complete `src/fuzz/` home, the 18-case external fuzz crossing, and the outside novelty-mint compile refusal.
- The facade roster contains `examples/rustc_coverage.rs`, `examples/support/rustc_coverage_host.rs`, `examples/support/rustc_coverage_replay.rs`, and `examples/support/rustc_coverage_subject.rs`.
- The narrow compiler-boundary `capture-observer` fixture and its one repository-only generated-support crossing remain excluded from the consumer archive by the existing proc-macro package law.

## Verified compiler archive

- `cargo +1.98.0 package -j1 -p macroonz-compiler --locked --offline --target-dir target/qualification/wave-f-package-refresh-20260828` packaged and verified successfully.
- The archive contained 300 entries and was 385,076 bytes.
- `macroonz-compiler-0.1.0.crate` had SHA-256 `04E6D4AB616238AF1E82A3549BD095D4516BDA4B8A41C0B43CEF2385EA44A460`.
- The byte size and entry count match the earlier compiler package observation.
- The archive hash moves with Cargo's embedded current Git source metadata and therefore is recorded for this exact source snapshot rather than treated as a source-payload regression.

## Dependency-order ceiling

- Offline `--no-verify` package attempts for `macroonz-harness`, `macroonz-macros`, and `macroonz` each refused before archive materialization because crates.io does not yet contain `macroonz-compiler = 0.1.0`.
- `--no-verify` does not bypass dependency resolution and therefore cannot lawfully simulate first-publication registry order.
- The proc-macro attempt also reported that the repository-only generated-support crossing is excluded from the published archive, which matches its declared repository-test role.
- Dependent archive materialization, package verification, publish dry runs, registry retrieval, and as-retrieved adopter tests remain behind the explicit first-publication human boundary.

## Evidence ceiling

- This is local Windows source-package evidence, not a hosted, published, registry-delivered, attested, physical-Linux, macOS, or ARM64 result.
- Package material remained disposable beneath `target/qualification/wave-f-package-refresh-20260828` until this receipt was committed.
