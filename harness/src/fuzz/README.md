# fuzz

Coverage-guided search is an external witness; this home owns the Macroonz campaign shell around it.

## Boundary

A selected native backend admits interesting bytes.
Macroonz owns campaign declaration, typed preflight facts the caller supplies, failure fingerprint, reduction, replay, and named ceilings.
The opt-in `fuzz-frida` feature opens the `LibAFL` plus Frida mechanism under this home: safe `EventSink` block recording, target-relative edge maps, deterministic bounded corpus evolution, and evolved [`InterestingBytes`] handoff.
That feature stays out of `default`, `full`, `diet-lite`, and `diet`.
Callers still install `macroonz` and enable `fuzz-frida` explicitly.

```mermaid
flowchart LR
    classDef input fill:#fff4d6,stroke:#9a6700,color:#3d2b00
    classDef backend fill:#e8f1ff,stroke:#3465a4,color:#102a43
    classDef held fill:#e6f4ea,stroke:#26864a,color:#123d22
    classDef ceiling fill:#f1e9ff,stroke:#7d4ab0,color:#32184f

    FACTS["declared preflight facts"]:::input --> READY{"every required<br/>capability available?"}:::held
    READY -->|no| INCOMPLETE["incomplete preflight"]:::ceiling
    READY -->|yes| SELECT["selected backend<br/>+ named ceilings"]:::backend
    SELECT --> BYTES["interesting bytes"]:::input
    BYTES --> REDUCE["generate::reduce<br/>+ capture_replay"]:::held
    REDUCE --> CAPSULE["ReplayCapsule"]:::held
```

## Composition

[`compose_reduce_replay`] runs the existing [`crate::generate::reduce`] and [`crate::generate::capture_replay`] roads under a [`crate::generate::ReductionProbeBinding`] the caller already opened from a refused report.
Interesting bytes enter as the reduction seed; Frida edges remain search compass unless the caller declares them as the fingerprint.

[`preflight_ready`] judges only the facts it is handed.
It never scans the filesystem, environment, or network.

[`run_libafl_frida`] is available only with `fuzz-frida`.
The caller supplies the subject classification; this home invents no product capture policy.

[`corpus`](crate::corpus) still owns seed packs and warm starts.
[`muterprater`](crate::muterprater) still owns pressure-lane vocabulary that names a fuzz road.
This home does not duplicate those owners.

## Evidence ceiling

Selection pins and ceilings are typed constants and values, not ambient discovery.
A host disposition of credible-unexecuted is an honest posture, not an executed receipt.
The first-party Windows runnable road remains an open seat under an existing package target; disposable campaign scratch stays under `target/qualification/fuzz-frida-windows/`.
That road does not by itself close Linux or macOS native receipts.
Hostile composition cases refuse empty interesting bytes, incomplete or contradictory preflight facts, incomplete F0 ceiling or host rosters, and reduction seeds that do not establish a baseline failure.
Branch coverage under nightly toolchains is not an accepted evidence plane; stable Rust 1.98 is the authorized toolchain.
The distilled Windows F0 selection receipt lives beside this README as [`windows_frida_f0_receipt.md`](windows_frida_f0_receipt.md).
