# Macroonz 0.2 direct-harness composition census

This Git-tracked receipt records the first Q1 composition-before-creation specimen against the published Macroonz 0.1 harness.

## Authority

- Campaign branch: `codex/macroonz-0.2-release-line`.
- Entering repository snapshot: `b71ddb2de088203489df5e99f4be10f49c137a1e`.
- Published dependency: `macroonz-harness = "=0.1.0"` from crates.io with default features disabled.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Toolchain: stable Rust 1.98.0 with Cargo 1.98.0.
- Scratch home: `target/qualification/0.2-q1-direct-harness-20260829`.
- This pass performed no product-source edit, dependency edit, feature edit, push, merge, ref rewrite, publication, or registry mutation.

## Question

Can a hand-written adopter target use the published harness batteries to state and judge a bounded temporal property without a proc-macro projection, generated scaffold, Loom, or a new framework abstraction?

## Specimen

- The subject is a small bounded state machine with declared byte inputs.
- The lawful history `[1, 2]` is judged through the published `TransitionContract`, `TemporalClaim`, `TemporalDemand::Always`, and `holds_over_history` surfaces.
- The hostile history `[4]` produces the exact `FailureClass::PropertyDisagreement` result and the exact cause `("census", "state-remains-bounded")`.
- A claim-free contract refuses as `ContractRefusal::NoClaimDeclared`, proving the pass is not vacuous.
- The dependency is development-only, uses no default feature, and does not activate Loom or the preemption home.

## Scratch custody

- `Cargo.toml`: `83803C34527FAE07CDDCEE4BCD8B9008D6FCA7691006BA346E3CD62D02F6A2DA`.
- `Cargo.lock`: `A7A4F771DC161AAD0F32A7A694ABEA6F20E13BE24F22FC0DD4C9FF94FE7506D8`.
- `rust-toolchain.toml`: `3EBDB2005CC3A5B3558C25ABA129043FE701ECCE1F2360B8B6672A97B55E810E`.
- `src/lib.rs`: `3D718E6C92801068DF4144B2B0F8C4CAD1D1CCBA3FA004E334F2A53D74F81009`.
- `tests/temporal.rs`: `8EBCC1550485A6A5C91631C980BB89CABB0B253ACE5DDDB47A1EF1A9A8768F47`.
- The sealed scratch tree contained 353 files and 159,740,606 bytes before cleanup, including disposable Cargo build output.
- These are the complete owner-agent final hashes; a coordinator draft briefly carried stale intermediate values and was corrected before Q1 acceptance.

## Stable qualification

- `cargo +1.98.0 fmt --all -- --check` passed.
- `cargo +1.98.0 check --all-targets --locked` passed.
- `cargo +1.98.0 clippy --all-targets --locked -- -D warnings` passed.
- `cargo +1.98.0 test --all-targets --locked` passed two of two semantic tests with no failure or ignored test.
- The coordinator read the authored specimen and lockfile and reran the complete declared wall independently after the owner agent completed.

## Disposition

- The temporal behavior is composition-complete on the published 0.1 surface.
- This specimen earns no new product home, public type, feature, dependency, crate, projection layer, or adapter.
- The bank's nested Cargo project is an input specimen, not authorized tracked repository architecture.
- Reduction and replay were outside this specimen's declared question and remain unproved here; that boundary is not evidence of a missing temporal-composition seam.
- This Windows x64 observation does not establish Linux, macOS, Wasm, generated-road, package, registry-delivered, performance, or human acceptance claims.

## Custody boundary

The exact authored specimen is retained below in this one readable receipt rather than as a tracked qualification package.
The lockfile hash above records the resolved graph; every non-root dependency is an immutable registry package named by that lockfile and the published package checksum recorded by Q0.
The scratch project and its build products remain disposable and may be removed after this receipt is committed and verified.

### `Cargo.toml`

```toml
[package]
name = "macroonz-direct-harness-census"
version = "0.0.0"
edition = "2024"
rust-version = "1.98.0"
publish = false

[dev-dependencies]
macroonz-harness = "=0.1.0"

[lints.rust]
unsafe_code = "forbid"
warnings = "deny"

[workspace]
```

### `rust-toolchain.toml`

```toml
[toolchain]
channel = "1.98.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

### `src/lib.rs`

```rust
//! Ordinary handwritten source used by the direct-harness census.

/// One neutral state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State(pub u8);

/// One neutral input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Input(pub u8);

/// The opening state.
#[must_use]
pub const fn opening() -> State {
    State(0u8)
}

/// Apply one input.
#[must_use]
pub const fn apply(state: &State, input: &Input) -> State {
    State(state.0.saturating_add(input.0))
}
```

### `tests/temporal.rs`

```rust
//! A direct harness journey over handwritten source and no procedural macro.

use macroonz_direct_harness_census::{Input, State, apply, opening};
use macroonz_harness::properties::{
    ContractRefusal, Holding, TemporalClaim, TemporalDemand, TransitionContract, holds_over_history,
};
use macroonz_harness::report::{FailureClass, FindingCause, TrialConclusion};

const BOUND_CAUSE: FindingCause = FindingCause::named("census", "state-remains-bounded");

fn bounded(state: &State) -> Holding {
    if state.0 <= 3u8 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

fn bounded_contract() -> Result<TransitionContract<State, Input>, ContractRefusal> {
    TransitionContract::declared(
        opening,
        apply,
        vec![TemporalClaim::declared(
            BOUND_CAUSE,
            TemporalDemand::Always(bounded),
        )],
    )
}

#[test]
fn handwritten_source_is_a_complete_harness_subject() -> Result<(), ContractRefusal> {
    let contract = bounded_contract()?;

    assert_eq!(
        holds_over_history(&contract, &[Input(1u8), Input(2u8)]),
        TrialConclusion::Passed
    );

    let TrialConclusion::Refused(finding) = holds_over_history(&contract, &[Input(4u8)]) else {
        panic!("the hostile history should break the declared bound");
    };
    assert_eq!(finding.class(), FailureClass::PropertyDisagreement);
    assert_eq!(finding.cause(), BOUND_CAUSE);
    Ok(())
}

#[test]
fn claim_free_contract_is_refused_before_a_vacuous_pass() {
    assert!(matches!(
        TransitionContract::<State, Input>::declared(opening, apply, Vec::new()),
        Err(ContractRefusal::NoClaimDeclared)
    ));
}
```
