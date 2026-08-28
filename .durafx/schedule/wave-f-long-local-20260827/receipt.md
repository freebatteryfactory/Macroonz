# Windows long-schedule receipt

This Git-tracked receipt retains the bounded Windows schedule observations introduced by source commit `d4a9dc3ac4145a269b3ad8a5af56c896b13cbfee`.

Git supplies history, hashing, replication, and change detection for this receipt.

No build output, mutable campaign directory, or raw scheduler exhaust is retained here.

## Denominator

- The parent source commit was `1f81f7491eebb85f9ec7b9c74d322d9675c390da`.
- The host was Microsoft Windows 11 Home build 26200 on `x86_64-pc-windows-msvc` with an Intel Core i7-8700 and 12 logical processors.
- The compiler was stable Rust 1.98.0 commit `88d9e12ae178fab0fb5cc050a94da85685d449ea` with LLVM 22.1.8.
- Cargo was 1.98.0 commit `797e8a9bc`.
- `Cargo.lock` had SHA-256 `471BD8BF8BAA28392CA4B14CB49C175877E6D8E06CD778E20D1AD71C21E3D586`.
- Every campaign was deterministic, bounded, opt-in, and retained as an external claim test rather than a default-wall cost.

## Observations

- `cargo +1.98.0 test -j1 -p macroonz-harness --test interleave_exploration --locked -- --ignored --nocapture` passed two of two explicit campaigns.
- The exhaustive campaign walked all 369,600 schedules of four three-command strands and earned `SpaceExhaustedAllHold`.
- The sampled campaign repeated exactly under one declared seed, explored 4,096 schedules from a counted space of 1,832,624,140,942,590,534, and retained `SampledAllHold` rather than exhaustive standing.
- `cargo +1.98.0 test -j1 -p macroonz-harness --test network_transcript --locked -- --ignored --nocapture` passed one of one explicit campaign.
- The network campaign drove 2,048 sends through drop, duplicate, and partition adversity, retained an exact 1,984-row census, repeated its address and bytes, preserved same-tick original-before-duplicate ordering, reproduced the transcript, exhausted replay, and joined both authorities under one address.
- `cargo +1.98.0 test -j1 -p macroonz-harness --test preemption_exploration --features preemption --locked -- supported::the_longer_fused_counter_holds_over_its_bounded_space --exact --ignored --nocapture` passed one of one explicit campaign.
- The preemption campaign used three yielded workers under `AtMost(4)` preemptions and a 100,000-branch ceiling and earned the backend's bounded all-interleavings-held result.
- `cargo +1.98.0 clippy -j1 -p macroonz-harness --test bench_receiver --test interleave_exploration --test network_transcript --test preemption_exploration --features preemption --locked -- -D warnings` passed.

## Evidence ceiling

- These are local Windows observations and establish no WSL, physical-Linux, cloud-Linux, macOS, ARM64, or hosted result.
- The sampled campaign makes no claim about schedules outside its exact sample.
- The preemption campaign makes no claim beyond its declared preemption and branch ceilings.
- The existing interleave counterexample proves direct replay and a one-byte reduction, but this slice does not yet establish generic schedule reduction through a replay capsule.
- The default Cargo target remains disposable and is not evidence authority.
