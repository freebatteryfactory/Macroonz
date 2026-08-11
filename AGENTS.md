# ThreadPak — Working Law

ThreadPak is a host-neutral semantic machine in safe Rust: programs are typed data, not
text. This file is the binding law for any person, model, or agent working in this
repository. `AGENTS.md` and `CLAUDE.md` are byte-identical; the parity is
machine-checked by `cargo xtask check`.

## The spine

```text
typed declarations → Semantic Form → Execution Form → ProgramImage (.tpk) → PakVM
→ runtime (the Stitch) → Bvisor → accepted history (.tlog)
```

Hosts live in other repositories and pin an exact ThreadPak revision. The machine never
knows which host is running it.

## Format law

- The repository is the specification. There is no separate book, and there will be no
  separate book: the review artifact is ASSEMBLED from the owner surfaces rather than
  authored. `cargo xtask book` is the command that will assemble it; it is owned by
  xtask and lands with the typed repository model, in the macroc era. It does not
  exist yet, and nothing may cite it as though it does. No semantic fact is manually
  restated in two places — cite the owner, never copy.
- Numbered directories are dependency bands: band N imports only bands lower than N.
  Numbers live on directories only; module names stay clean via `#[path]`.
- The crate root owns generic composition shapes only. A semantic noun lives at the
  root solely by an explicit root admission ruling from the repository owner;
  otherwise it has an owner home. The root is never a shared-noun drawer.
- Every semantic home is `README.md` + `mod.rs` + `types.rs`. A home README is markdown
  prose plus fenced yaml blocks that tooling parses and verifies against derived facts.
- `types.rs` owns the home's public types **in the core crate**: a `pub` struct, enum,
  type alias, or trait outside the owning `types.rs` refuses. Private implementation
  types live beside their algorithms and never leak into public signatures, bytes, or
  identities. The rule governs semantic homes; it is not a workspace-wide file law. The
  tooling crates carry the same discipline in the form their material admits —
  machine-enforced module order, and types discoverable at the module that owns them —
  and it is formalized under policy-scoped surfaces when the typed repository model
  lands.
- One root `laws.rs` in the core crate carries the compile-time green laws, sectioned
  by home. It is non-public and outside the derived ABI surface. A law that cannot fail
  is not a law; every law is proven non-vacuous by reversal.
- Every obligation declared in a README carries a stable id, a typed `challenge_kind`,
  one positive control, one law-negating challenge, and an activation route. trybuild
  covers compile refusals only — it is one challenge kind, never the universal one.
- No hand-maintained inventories: counts, dependency maps, status tables, and public
  surface listings are derived by tooling, never authored.
- rustdoc is a spec surface: public items are documented at the declaration; the README
  carries home-level narrative; nothing is written twice.

## Vocabulary

- The runtime transition is the **Stitch**: one typed observation in, next state and
  effect intents out. `Stitch` is reserved exclusively for the runtime transition —
  never reused for linking, joining, merging, or causal edges. `tick` is the clock's
  tick and nothing else. `step` is PakVM's inner step. `Turn` is an identity. PEND is
  never spelled AWAIT.
- Canonical verbs: define, parse, decode, encode, validate, resolve, compile, lower,
  plan, execute, apply, commit, project, render, inspect, explain, dispose, sample,
  fork, merge, append, acknowledge, checkpoint, resume, pack.
- Banned in prose and identifiers: construction-lifecycle vocabulary (factory,
  candidate, promotion, self-hosting); "law" as an ordinary noun (the `laws.rs`
  filename is the accepted exception); the acronym TCB.

## Hard rules

- Safe Rust only. The workspace lint wall is declared once at the root and inherited by
  every member; no member may weaken it; `#[allow]` is forbidden.
- NO PYTHON in this repository, ever. All tooling is Rust (xtask; trybuild).
- The metaprogramming services project contracts; they never decide meaning and are
  never their own oracle. The services live in `macros/macroc/` (ordinary callable
  Rust — planning, rendering, inspection, explanation); the proc-macro crate
  (`macros/proc/`) is one thin Rust-facing expansion surface over them, and `threadc`
  is reserved for a future language frontend. Every macro family ships with a planted
  defective expansion that testpak must reject. Expansion is deterministic from its
  declared input: no network, no filesystem scans, no environment reads, no clock, no
  entropy — and testpak carries hostiles proving those pathways are unused. The proc
  shell is semantically empty, ambient-free, thin, and dependency-minimized: a
  third-party dependency enters it only by explicit mechanism admission, when a real
  shell obligation has earned it. The core package carries no dependency edge to the
  metaprogramming tooling under any Cargo edge kind — ordinary, renamed, dev, build,
  or target-specific — and the `no-core-tooling-edge` gate enforces that absence.
  Compiler services never depend on their frontend surfaces, even for tests;
  composition is proven from an outside consumer fixture. A tooling type may
  summarize, reference, plan, explain, or project an owner fact; it may never create
  a second value that independently answers the owner's semantic question.
- testpak depends inward on core; nothing depends on testpak. Production never depends
  on its judge.
- Probes (throwaway compiler experiments) never enter this repository.
- LF line endings everywhere; no symlinks; no `build.rs`; no environment-variable
  dependence — anything env-dependent is a bug.
- Frontends plug in from outside through the public declaration path with zero core
  changes. If a frontend needs a core change to exist, the declaration contract is
  wrong.

## Phase gate

The repository is in architecture closure: every home receives its spec files with zero
product-runtime implementation — no machine algorithms, no host behavior. Architecture
tooling is real code by design: xtask checks, macros, testpak harness and fixtures,
compile-time laws, and bounded probes are executable and must never sit empty.
Implementation of the machine opens per home only on the repository owner's explicit
authorization. Commits are defined by the repository owner — never initiated by an
agent on its own.
