# Macroonz 0.2 local facade-posture economics

This receipt retains one bounded local Windows observation that ordinary executable tests cannot preserve.

## Authority

- Measurement repository revision: `deee363331ce295d28d1dd737e399ed23083fc3f`.
- Product package source revision: `73ffa61b8555290deebee2d463f207ba10651036`.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Toolchain: stable Rust and Cargo `1.98.0`.
- Cargo posture: locked, offline, release profile, one job, and one fresh exact target at a time.
- Consumer: the exact renamed-facade package-shaped consumer previously qualified by Wave H.

The facade, compiler, harness, and proc-macro archives hash respectively to `B3E5BDF0490E11A580961A6C5438ABB27D3B9A1305E9536A95F4F6284E5A5B6D`, `5C4D7A7BC70E19B547ACD1857D38CE8646A93B9EF3BAFBAAF57CD9D0F114D070`, `6CB879B11168691CA44E74BD590790060D039040A2B2CD08BEFAFC637BC0157B`, and `4731975F381C8339B4B8343F7DC2EDC0B1A708244B185F4BED4FECC09B2204AF`.

The four archives total `1,018,172` bytes under every posture because feature selection changes the compiled graph rather than the packaged input bytes.

## Typed work and controls

| Posture | Required surface | Resolved packages | Macroonz packages | Active feature entries |
| --- | --- | ---: | ---: | ---: |
| `diet` | compiler and proc macro | 17 | 3 | 7 |
| `diet-lite` | compiler, proc macro, and harness | 21 | 4 | 11 |
| `default-full` | compiler, proc macro, harness, and preemption | 45 | 4 | 64 |

The same core consumer target compares `diet` with `diet-lite`.

The same harness consumer target compares `diet-lite` with `default-full`.

An exact safe-Rust adapter through the existing `macroonz-harness::bench` owner qualified both narrow-to-broader controls and refused an identical control before timing.

This establishes that each broader posture performs strictly more declared package and feature work for the corresponding narrower journey.

It does not establish that `default-full` is globally worse because full owns additional requested capability and no broader supported facade posture exists.

## Local host observations

| Posture | Target files | Target bytes | Cold samples ms | Warm samples ms | Harness samples ms |
| --- | ---: | ---: | --- | --- | --- |
| `diet` | 248 | 79,968,230 | 50,977; 52,426; 50,694 | 251; 147; 141 | unavailable |
| `diet-lite` | 291 | 118,011,695 | 106,839; 109,905; 107,105 | 285; 275; 170 | 710; 952; 698 |
| `default-full` | 485 | 176,911,465 | 158,725; 155,695; 157,476 | 200; 232; 210 | 948; 822; 775 |

Cold medians are `50,977`, `107,105`, and `157,476` milliseconds for `diet`, `diet-lite`, and `default-full` respectively.

All three fresh samples for each posture produced identical target file and byte counts.

All nine cold executions, all nine warm no-op executions, and every applicable semantic test passed.

Every sample ended with Cargo-native cleanup and an absent exact target directory.

The complete sample table had SHA-256 `FE2AEA5FEACA228A945CA4955AA5721B5BD23AA3FA747F5C458294C8C377638A`, and the aggregate table had SHA-256 `5D0E00872062252427206E9A3DF17A575DC1AC70B7DBD698FAE599F23F123B47` at observation time.

## Qualification and custody

The disposable adapter passed formatting, locked and offline all-target checking, strict all-target Clippy, its complete two-test wall, and a repeated focused test on stable Rust `1.98.0`.

Cargo then removed its exact `811.3` MiB target.

This receipt is the complete retained evidence; no source payload, package copy, scratch laboratory, raw log, executable, debug database, or build output accompanies it.

The source and package graph remain reconstructable from Git and the fixed archive hashes above.

## Ceiling

These timings are secondary observations from one interactive Windows host and do not establish cold-machine, Linux, macOS, Wasm, registry-delivered, physical-host, peak-memory, universal-threshold, or release-acceptance claims.

Target bytes are rebuildable Cargo state rather than shipped archive size.

Source mutation is inapplicable to consumer feature selection, so this observation adds no Muterprater sensitivity claim.

No product source, public API, feature, dependency, product package, benchmark owner, cost schema, tracked qualification package, push, merge, ref movement, hosted action, registry action, tag, or publication changed.
