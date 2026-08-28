# Windows facade posture cost receipt

This Git-tracked receipt retains one bounded local observation of clean release-build cost, warm no-op build cost, output size, and successful execution for three path-adopter facade postures.

It is not a portable performance baseline, a package-archive result, or a claim that one posture is universally preferable.

## Denominator

- The product source was commit `9c5279a37e5d2923001e178e46fe5fc322eec9af` on `codex/macroonz-repository-completion`.
- Concurrent worktree changes were confined to repository external tests and did not enter any adopter dependency graph measured here.
- The host was Microsoft Windows 11 Home build 26200 on `x86_64-pc-windows-msvc` with an Intel Core i7-8700 and 12 logical processors.
- The toolchain was stable Rust 1.98.0 with LLVM 22.1.8 and Cargo 1.98.0.
- Every build used release mode, `--offline`, `--locked`, and one Cargo job.
- Each clean sample began by cleaning its exact posture-specific target directory.
- Each posture received three clean samples, three warm no-op samples, one final target-byte census, and one successful execution.
- The disposable adopter projects lived under `target/qualification/wave-f-adopter-cost-windows-20260827` and the disposable build directories lived under `target/q/pf`, `target/q/pl`, and `target/q/pd`.

## Declared postures

| Posture | Facade declaration | Public path exercised | Lock packages | Manifest SHA-256 | Source SHA-256 | Lock SHA-256 |
| --- | --- | --- | ---: | --- | --- | --- |
| default-full | default features | `macroonz::compiler` and `macroonz::harness` | 47 | `39C9FF318B620E1655A0465D49775A19B9798EC663F5E6C98BFB8E3843A5E412` | `B41A90B825E022CD2BDC301537D11CF654DEE064D4B660F56768DCB16C7E3BA1` | `BE1BB1DC815E061E7F3059D99D043F4061A53D55B02CA320FD9C85337DFBC91A` |
| diet-lite | no defaults plus `harness` | `macroonz::compiler` and `macroonz::harness` | 22 | `9A68B5CC30B25AF92148333DD63816E7CF820B49F5CBDC2DB8E98586902DBB99` | `B41A90B825E022CD2BDC301537D11CF654DEE064D4B660F56768DCB16C7E3BA1` | `35CBB28D170496009BE8C3813BD2516BD61A2A876085C1C1B563EC72E6A84F3C` |
| diet | no defaults | `macroonz::compiler` | 18 | `5836A9FB0B3FCE7C436D4565F62E8207166E763E15EF3793C01E4B7B6FEE9E0E` | `E1EDAE6CFBAE496361D38A6B5E9849A5D0494AEEDD3F0B84DAFEDC44EB6029C4` | `23404C05EA1531D4DA20CD794A22CF87485C7896CF8373E3DAFA10EBC342B7D6` |

## Observation

| Posture | Clean release samples | Clean median | Warm no-op samples | Warm median | Final target bytes | Execution |
| --- | --- | ---: | --- | ---: | ---: | --- |
| default-full | 210,615 ms; 199,217 ms; 194,444 ms | 199,217 ms | 266 ms; 277 ms; 279 ms | 277 ms | 173,692,059 | passed in 325 ms |
| diet-lite | 132,882 ms; 131,569 ms; 134,402 ms | 132,882 ms | 222 ms; 212 ms; 208 ms | 212 ms | 114,818,892 | passed in 269 ms |
| diet | 58,695 ms; 65,174 ms; 61,996 ms | 61,996 ms | 190 ms; 258 ms; 360 ms | 258 ms | 77,460,453 | passed in 245 ms |

- All three lockfiles generated offline from the current path dependency graph.
- All nine clean release builds and all nine warm no-op builds completed successfully.
- Each resulting adopter binary executed successfully through the facade posture it declared.

## Evidence ceiling

- These wall readings describe this one Windows host while other activity existed and are not thresholds, regressions, or cross-host comparisons.
- Target-directory bytes include Cargo build state and are not a shipped archive-size measurement.
- The default-full and diet-lite source specimens are intentionally identical so their dependency declarations, rather than their Rust expressions, distinguish the two harness-bearing postures.
- Package-only, registry, physical-Linux, hosted-Linux, macOS, ARM64, peak-memory, and publication observations remain separate planes.
- Raw build output and the scratch adopters remain disposable and have no evidence authority once this receipt is committed.

## Cleanup

- After this receipt was committed, Cargo removed 165.6 MiB from `target/q/pf`, 109.5 MiB from `target/q/pl`, and 73.9 MiB from `target/q/pd`.
- The validated exact scratch-adopter source tree was moved recoverably to the `wave-f-adopter-cost-windows-20260827` leaf of the task-created `macroonz-night-scratch-20260827` archive outside the repository because host policy blocked a direct recursive deletion.
- None of the five live repository scratch paths remains, and the recoverable archive has no evidence authority.
