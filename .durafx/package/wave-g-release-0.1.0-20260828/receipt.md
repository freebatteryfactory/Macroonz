# Macroonz 0.1.0 release receipt

## Standing

- Published source revision: `eb4e6c0855f943065905c93eb98fbf7dfd40fe53`.
- Release-preparation revision: `3268515b6e270e76952911f829888816ab1237c3`.
- The published source is an explicit merge with first parent `a7c74156a7347072ec827e95bb716e2c346c51e8` and second parent `3268515b6e270e76952911f829888816ab1237c3`.
- The merge tree is byte-identical to the qualified release-preparation tree.
- Campaign-plan snapshot used to interpret this observation: SHA-256 `4C5CD359B565858B7D194510EFFBB461B483A71C1E181B55787DC2705727871A`.
- `Cargo.lock` SHA-256: `471BD8BF8BAA28392CA4B14CB49C175877E6D8E06CD778E20D1AD71C21E3D586`.
- Product and qualification toolchain: stable Rust 1.98.0 with Cargo 1.98.0.
- This receipt records a qualified hosted, published, and registry-delivered observation pending final owner acceptance of the release plane.

## Hosted source crossing

- Manual workflow `Hosted pulse`, identifier `344858202`, ran as `33214837419` against exact source `eb4e6c0`.
- Run URL: `https://github.com/freebatteryfactory/Macroonz/actions/runs/33214837419`.
- The run began at `2026-08-28T21:58:10Z`, reached terminal success at `2026-08-28T22:02:51Z`, and had no retry.
- Blacksmith Linux x64 complete wall, job `98996043463`, completed green in 1 minute 58 seconds.
- Blacksmith Linux ARM64 host crossing, job `98996043609`, completed green in 3 minutes 2 seconds.
- GitHub macOS ARM64 host crossing, job `98996043469`, completed green in 3 minutes 5 seconds.
- GitHub Windows x64 host crossing, job `98996043314`, completed green in 4 minutes 36 seconds.
- Every seat passed native graph checking, strict Clippy, 423 of 423 external tests across 60 binaries, four compiler doctests, stable-rustc coverage, and tracked-tree mutation refusal.
- Linux x64 additionally passed formatting, cargo-deny dependency policy, warnings-denied documentation, and all-feature `wasm32-unknown-unknown` checking.
- The ten intentional skips were `identity_process_determinism::child_process_reports_address`, `interleave_exploration::a_long_exhaustive_campaign_walks_every_counted_schedule`, `interleave_exploration::a_long_sampled_campaign_repeats_without_claiming_the_space`, `network_transcript::custody::a_long_network_campaign_reproduces_and_exhausts_exactly`, `preemption_exploration::supported::ambient_early_stops_are_overwritten_child`, `preemption_exploration::supported::branch_exhaustion_is_typed_child`, `preemption_exploration::supported::invalid_environment_is_typed_child`, `preemption_exploration::supported::the_longer_fused_counter_holds_over_its_bounded_space`, `runner_evidence::panic_boundary::abort_claim_child`, and `runner_evidence::panic_boundary::hook_claim_child`.

## Dependency-ordered publication

- The owner explicitly authorized the irreversible publication dance.
- Each package passed `cargo +1.98.0 publish --locked --dry-run` from exact source `eb4e6c0` immediately before its live upload.
- Publication proceeded in dependency order: `macroonz-compiler`, `macroonz-harness`, `macroonz-macros`, then `macroonz`.
- Cargo waited for each new version to become available in crates.io before the next dependent dry-run began.
- No token value, credential file, or authorization header entered repository output or this receipt.

| Package | Files | Archive bytes | Registry and local SHA-256 |
| --- | ---: | ---: | --- |
| `macroonz-compiler 0.1.0` | 304 | 393,098 | `6896202A7ECF4F4A8017029C364D707D21C6D02AFCB29A42322BE655D37C0D74` |
| `macroonz-harness 0.1.0` | 366 | 441,544 | `D4047EA57BAD927E8CBA4913F6A54DD9DF095114F3E3723A62EAFD2C7D919BAA` |
| `macroonz-macros 0.1.0` | 32 | 20,825 | `61950E4BDF0BBD46211413B7A461A0BD24F2F2381E150DCB8C960D4E1DB7BEF7` |
| `macroonz 0.1.0` | 20 | 128,355 | `DBE80FF4EB844508DC9FF69A4938729C6CAD98F145C895D9A7D14FF57FB255FC` |

- crates.io reported all four versions as `0.1.0` and unyanked.
- Every crates.io checksum exactly matched the SHA-256 of the corresponding locally uploaded `.crate` archive.
- Every package carried Rust 1.98.0, `MIT OR Apache-2.0`, the repository URL, its declared description, its distinct keywords, and its declared categories through the registry projection.
- Every package archive carried its README, Git source metadata, Apache-2.0 text, and MIT text.
- The proc-macro package reported that repository-only test `generated_support_crossing` is excluded from publication.
- Inspection of its normalized packaged manifest proved `autotests = false` with only the four included integration targets explicitly named, so no published target points at the excluded crossing.

## Registry-delivered adopter

- A fresh disposable binary package declared only `macroonz = "=0.1.0"` and its own empty workspace boundary.
- Its generated locked graph named all four Macroonz packages at version `0.1.0` with crates.io registry sources and the exact checksums in the publication table.
- After one registry fetch, `cargo +1.98.0 run --locked --offline` compiled all four delivered packages and executed successfully.
- The executable observed the facade-owned compiler projection and composed a facade-qualified network procedural declaration through the facade-qualified harness.
- No workspace path dependency, Git dependency, unpublished source, or repository build output participated in that execution.

## Documentation delivery

- `https://docs.rs/macroonz-compiler/0.1.0/macroonz_compiler/` returned success.
- `https://docs.rs/macroonz-harness/0.1.0/macroonz_harness/` returned success.
- `https://docs.rs/macroonz-macros/0.1.0/macroonz_macros/` returned success.
- `https://docs.rs/macroonz/0.1.0/macroonz/` returned success.
- Direct feature-gated pages `macroonz_compiler::host` and `macroonz_harness::preemption` also returned success, proving docs.rs built the declared all-feature posture.

## Cleanup status

- After the first receipt commit, Cargo removed 435 files and 133.2 MiB from exact target `target/qualification/release-eb4e6c0-compiler`.
- Cargo removed 512 files and 171.3 MiB from exact target `target/qualification/release-eb4e6c0-harness`.
- Cargo removed 175 files and 56.9 MiB from exact target `target/qualification/release-eb4e6c0-macros`.
- Cargo removed 404 files and 319.9 MiB from exact target `target/qualification/release-eb4e6c0-facade`.
- The adopter's own Cargo clean removed 413 compiled files and 380.3 MiB from its nested build target.
- The patch mechanism then removed only the generated adopter manifest, lockfile, and source, and non-recursive removal deleted the two empty directories.
- All five exact `target/qualification/release-eb4e6c0-*` paths named by this receipt are absent.

## Plane limits

- This receipt does not claim a supply-chain attestation, signed package, reproducible-build proof across hosts, trusted-publishing configuration, immutable GitHub Release, download population, or final owner acceptance.
- Annotated tag `v0.1.0` remains pending at exact published source `eb4e6c0` until the receipt and cleanup truth are committed.
- README adjacency positioning, the facade storefront doctest, announcement posts, and release promotion remain later work rather than hidden conditions of this registry-delivery observation.
