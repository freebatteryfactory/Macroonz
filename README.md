# ThreadPak

ThreadPak is a host-neutral semantic machine written in safe Rust. Programs are typed
data, not text: a builder constructs typed declarations, and the machine validates,
seals, executes, and remembers them. Any frontend — Rust today, others later — enters
through the same public declaration path.

## The spine

```text
typed declarations
  → Semantic Form           normalized meaning, checked constructor
  → Execution Form          bounded operator graph, independently re-lowered
  → ProgramImage (.tpk)     binds both forms; the machine's inspectable bytecode
  → PakVM                   bounded synchronous execution, no ambient anything
  → runtime                 the Stitch: one observation in, next state and effect intents out
  → Bvisor                  the physical membrane: admission, capabilities, ports
  → accepted history (.tlog) durable append, crash recovery, authorized removal
```

Hosts live in other repositories and pin an exact ThreadPak revision. The machine never
knows which host is running it.

## Workspace

| Crate       | Role                                                    |
| ----------- | ------------------------------------------------------- |
| `threadpak` | the machine — root package at the repository root       |
| `xtask`     | repository law checks and tooling — `cargo xtask check` |

The repository root is itself the `threadpak` package; its `src/` carries the machine.
`macros/`, `testpak/`, `xtask/`, and `hosts/` are unnumbered: first-class, but never on
the production dependency path.

## The band map

Numbered directories are dependency bands: band N imports only bands lower than N.
This order is compiler-verified — every cross-home public type reference points
downward. Homes materialize only when their specification content lands; no directory
exists empty.

| Band | Home        | Owns |
| ---- | ----------- | ---- |
| —    | `src/types.rs` | root calculus: generic composition shapes + promoted axes |
| —    | `src/laws.rs`  | the one compile-time proof surface, sectioned by home |
| 00   | refusal     | refusal envelope, families, handling, `ReasonId` |
| 01   | logic       | three-valued logic, truth tables, finality primitives |
| 02   | identity    | the six identity classes, minting, scope guards |
| 03   | value       | bounded values, closed algebra, absence |
| 04   | numeric     | exact numeric families, intervals, quantization |
| 05   | bounds      | budget classes carrying typed magnitudes |
| 06   | authority   | capability and `KeyScope` value algebra, meet, attenuation, protected resolution |
| 07   | bytes       | frame grammar, codecs, digest domains, byte roles, content regions |
| 08   | schema      | schema model, refinements, canonical profiles, migration |
| 09   | time        | clock observations (the tick), deadlines, chronology |
| 10   | history     | accepted history, append, recovery, removal, partitions, federation |
| 11   | navigation  | frames, axes, addresses, `Fix`, source closure |
| 12   | port        | port contract algebra and host-obligation shapes |
| 13   | declaration | the shared authoring algebra: fragments, linker, facets |
| 14   | semantic    | the judgment and Semantic Form |
| 15   | execution   | the operator register, lowering, agreement, Execution Form |
| 16   | image       | `ProgramImage`, `.tpk`, entrypoints, admission |
| 17   | pakvm       | the executor: values, arenas, the step machine |
| 18   | bvisor      | the boundary supervisor: Attempts, reservation, ports, observations |
| 19   | runtime     | the Turn, the Stitch, checkpoints, replay, reconciliation |
| 20   | derived     | DataBlocks, masks, materialization |
| 21   | application | composition, interfaces, Serve semantics |
| 22   | security    | protection machinery: sealed extents, shred, revocation distribution |
| 23   | evidence    | receipts, verification, denominators, the evidence graph |

## Phase

Architecture closure: the repository receives its complete specification before any
machine implementation. Implementation opens per home only on explicit authorization.

```yaml
phase: architecture-closure
toolchain: "1.97.1"
workspace_members:
  - xtask
```
