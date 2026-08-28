# Green four-seat hosted pulse

## Standing

- Source revision: `be990fb11b8968ab13944c2aee746b200ce929c8`.
- Workflow: `Hosted pulse`, identifier `344858202`.
- Run: `33207350287`.
- Run URL: `https://github.com/freebatteryfactory/Macroonz/actions/runs/33207350287`.
- Event: manual `workflow_dispatch` against default branch `main`.
- Started: `2026-08-28T20:14:06Z`.
- Completed: `2026-08-28T20:19:14Z`.
- Conclusion: success.
- Campaign-plan snapshot used to interpret the observation: SHA-256 `C92C9D0C32FBD129BA674F7427D978380A5645DB233EF2318BCB8DCA093E8C1E`.
- This receipt records a qualified hosted observation pending final owner acceptance of the plane.

## Declared seats

- Blacksmith Linux x64 complete wall, job `98971670130`, completed green in 1 minute 54 seconds.
- GitHub macOS ARM64 host crossing, job `98971670155`, completed green in 2 minutes 7 seconds.
- Blacksmith Linux ARM64 host crossing, job `98971670022`, completed green in 3 minutes 4 seconds.
- GitHub Windows x64 host crossing, job `98971670143`, completed green in 5 minutes 1 second.
- All four jobs reached terminal success, and no automatic retry or additional dispatch followed.

## Common denominator

- Every seat installed stable Rust 1.98.0 commit `88d9e12ae178fab0fb5cc050a94da85685d449ea` with Cargo 1.98.0 commit `797e8a9bca276c1c9f9f738d2a20f484fa4eea9d` and LLVM 22.1.8.
- Every seat checked every native target and feature and passed the strict Clippy wall.
- Every seat ran 423 external tests across 60 binaries, passed all 423, and reported ten intentional skips.
- Every seat passed the four compiler doctests while the other three packages retained zero doctests.
- Every seat passed the stable-rustc coverage crossing.
- Every seat passed the final tracked-tree mutation refusal.
- The Linux x64 complete-wall seat additionally passed formatting, cargo-deny dependency policy, warnings-denied documentation, and the all-feature `wasm32-unknown-unknown` posture.

## Hosted Windows closure

- The Windows workflow retained its ordinary `RUST_BACKTRACE=1` posture.
- `supported::branch_exhaustion_stays_infrastructure_unresolved` passed as test 317 of 423.
- The same parent crossing had aborted with stack-overflow exit `0xc00000fd` in runs `33201896588` and `33205616869` before exact child-process isolation was integrated.
- The green crossing therefore closes that contradiction for exact source `be990fb` on the declared GitHub Windows Server 2025 seat without removing the test, lowering its branch budget, changing its typed assertions, or disabling diagnostics globally.

## Plane limits

- This is a cloud-host observation for the declared Blacksmith and GitHub runner seats, not a physical-host observation.
- The run does not establish physical Linux, registry delivery, crate publication, attestation, release acceptance, private-vulnerability-reporting configuration, branch protection, or required-check governance.
- No product source, public API, dependency, feature, identity, encoded byte, workflow trigger, cache posture, artifact upload, secret, or publication authority changed during the pulse.
