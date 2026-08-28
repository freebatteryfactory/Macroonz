# Hosted CI design and local qualification

## Standing

- Source base: `ffdd8b7bd42807be912c49b9db5122ad13ae0696`.
- Qualification branch: `codex/macroonz-hosted-ci`.
- Workflow implementation commit: `ac31fb6269efa78651af579a7fbea9dbaeee210e`.
- Campaign-plan SHA-256: `FCB402268443AD802D48E4E5A49E86095B45634591268F3ABC6C94EA3726B96D`.
- This receipt records a locally qualified workflow design pending owner acceptance.
- No hosted workflow, runner, operating system, architecture, duration, cache hit, or cloud verdict is claimed.

## Declared hosted topology

- Blacksmith Linux x64 on `blacksmith-4vcpu-ubuntu-2404` owns the complete hosted wall.
- Blacksmith Linux ARM64 on `blacksmith-4vcpu-ubuntu-2404-arm` owns one native architecture crossing.
- GitHub Windows x64 on `windows-2025` owns one independent provider and operating-system crossing.
- GitHub macOS ARM64 on `macos-15` owns one independent provider, operating-system, and architecture crossing.
- The workflow is manual-dispatch only, read-only, secret-free, artifact-free, publication-free, timeout-bounded, and protected by same-ref cancellation.
- The cache contains Cargo registry packages and Git database material only and carries no compiled target, qualification scratch, verdict, or accepted evidence.
- The complete Linux seat carries source-wide formatting, dependency policy, warnings-denied rustdoc, and the all-feature Wasm posture once.
- Every seat carries native graph checking, strict Clippy, the complete external test wall, doctests, and the stable-rustc coverage example.

## Immutable inputs

- `actions/checkout` v6.0.2 resolved to `de0fac2e4500dabe0009e67214ff5f5447ce83dd` through the GitHub API.
- `actions/cache` v5.0.3 resolved to `cdf6c1fa76f9f475f3d7449005a359c84ca0f306` through the GitHub API.
- `taiki-e/install-action` v2.86.3 resolved to `5b4d68e2e660441203ab128a23676f1e4faf1532` through the GitHub API.
- The workflow requests stable Rust 1.98.0, cargo-nextest 0.9.132, and cargo-deny 0.19.0.
- Dependency resolution is locked, and Cargo becomes offline after one declared fetch.

## Workflow analysis

- Zizmor 1.29.0 reported no finding under its pedantic, low-severity, strict-collection audit.
- The released Windows archive used for that audit had SHA-256 `68A6BC6888F10BF0D53658C75885E7C1B7A0588D4C1FBC3F0CA280AD7324BF06` and passed GitHub attestation verification against `zizmorcore/zizmor`.
- Actionlint 1.7.12 was verified against its published checksum as SHA-256 `6E7241B51E6817EA6A047693D8E6FED13B31819C9A0DD6C5A726E1592D22F6E9`.
- Actionlint's first pass named only the two declared third-party Blacksmith runner labels as unknown to its built-in GitHub label roster.
- A second pass admitting only runner labels beginning with the exact `blacksmith-4vcpu-ubuntu-2404` prefix reported no remaining finding.
- No repository ignore, suppression, analyzer configuration, or weaker workflow posture was introduced to obtain either result.

## Local execution

- Cargo-nextest 0.9.132 accepted the `ci` profile and its exact-version requirement.
- The complete `ci` profile ran 423 tests across 60 binaries, passed all 423, and reported nine intentional skips.
- The profile wrote a parseable 95,690-byte JUnit document with 56 suites, 423 cases, zero failures, and zero errors.
- Stable Rust 1.98 passed formatting, workspace every-target and all-feature checking, strict Clippy, four doctests, cargo-deny policy, warnings-denied rustdoc, all-feature `wasm32-unknown-unknown` checking, and the facade `rustc_coverage` example.
- The coverage example removed its exact disposable run and left no live example scratch.
- `git diff --check` reported no whitespace error.

## Tooling obstruction and custody

- A disposable source build of zizmor 1.29.0 reached its final link and then refused because the active Windows linker search could not locate `windows.0.52.0.lib`.
- That refusal belonged to the external analyzer build, changed no repository source, and did not become a reason to alter the Macroonz toolchain.
- The independently released and attested analyzer binary supplied the completed workflow audit instead.
- Guarded Cargo cleanup removed the exact 3,540-file, 1,128,064,818-byte analyzer build tree, and guarded direct cleanup removed the exact 17-file, 41,421,321-byte released-tool tree after this receipt retained their identities and results.
- The Playwright MCP process trees were terminated, but an unidentified live Windows handle still holds one empty pre-existing Mermaid-shot leaf.
- That empty leaf contains no file, verdict, or evidence authority and was not relabeled as this task's scratch.

## Unproved planes and boundaries

- The Blacksmith GitHub App installation was observed through the organization installation API, but no ephemeral runner has yet accepted a Macroonz job.
- Cloud Linux x64, cloud Linux ARM64, cloud Windows x64, cloud macOS ARM64, cache behavior, JUnit ingestion, machine metrics, and run duration remain unexecuted.
- GitHub accepts manual workflow dispatch only after the workflow exists on default `main`.
- Pushing and merging this branch is therefore one owner boundary, and dispatching the first hosted pulse is a second owner boundary.
- Automatic triggers, required checks, branch governance, security administration, physical-host qualification, publication, attestations, and registry delivery remain outside this receipt.
