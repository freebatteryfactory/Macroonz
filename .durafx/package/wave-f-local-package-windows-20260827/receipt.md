# Local Windows package receipt

This Git-tracked receipt retains the package observation available before Macroonz's first dependency-ordered registry publication.

## Denominator

- The source snapshot was `21a57336bcef06d6646cf2da355980c6c03e44be` on `codex/macroonz-repository-completion` with a clean tracked worktree.
- The host was Microsoft Windows 11 Home build 26200 on `x86_64-pc-windows-msvc`.
- The toolchain was stable Rust 1.98.0 with Cargo 1.98.0.
- `Cargo.lock` had SHA-256 `471BD8BF8BAA28392CA4B14CB49C175877E6D8E06CD778E20D1AD71C21E3D586`.

## Observation

- `cargo +1.98.0 package -j1 -p macroonz-compiler --locked --offline` packaged and verified successfully.
- The compiler archive contained 300 entries and was 385,076 bytes.
- `macroonz-compiler-0.1.0.crate` had SHA-256 `68D93FF361E9092635C209C5B9C4770478EF11F9B89E971E774CA2B5A70FCB39`.
- The archive contained its normalized `Cargo.toml` and `src/lib.rs`.
- `cargo +1.98.0 package -p macroonz --locked --list` completed successfully with 17 entries.
- The facade package list retained `examples\rustc_coverage.rs` and `examples\support\rustc_coverage_subject.rs`.
- The facade package list had SHA-256 `1BEB1252C45C74DF2727B7DF1926BE89439BBA3EE5295171776801403CB24F55`.

## Evidence ceiling

- `macroonz-harness`, `macroonz-macros`, and `macroonz` cannot complete dependency-verifying package or publish-dry-run qualification until the first-publication registry order makes `macroonz-compiler = 0.1.0` resolvable.
- This is a local source-package observation, not registry delivery, publication, hosted CI, attestation, or post-publication adopter evidence.
- The archive and package-list scratch under `target/package` remain disposable after their hashes and observations are committed.
