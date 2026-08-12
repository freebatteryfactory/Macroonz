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
- A home README is markdown prose plus fenced yaml blocks that tooling parses and
  verifies against derived facts.
- **The file grammar.** Every semantic home is its numbered folder carrying `README.md`
  + `mod.rs` + `types.rs`, and the folder's files are named by what they are about.
  `types.rs` owns the home's public types **in the core crate**: a `pub` struct, enum,
  type alias, or trait outside the owning `types.rs` refuses. Private implementation
  types live beside their algorithms and never leak into public signatures, bytes, or
  identities. As machinery earns existence the home grows exactly two more type-owned
  files:
  - `type_guard.rs` — the smart constructors and the invariant nucleus. It is declared
    inside `types.rs` as `#[path = "type_guard.rs"] mod guard;`, which makes it types'
    own child and lets it see private fields. Parse-don't-validate lives here: a value
    that exists has already been parsed into its invariant, so nothing downstream
    re-checks it.
  - `type_contract.rs` — the declarative trait implementations: refusal families,
    identity roles, register participation.

  Everything else is a role-named pure-function module — `validate.rs`, `encode.rs`,
  `decode.rs`, `project.rs`, `transition.rs` — consuming types that are already
  informed, so no function re-establishes what its argument's type already promised. A
  file about the types it carries takes the `type_` prefix and sorts beside `types.rs`;
  a file about an operation does not, and the sort order is the reading order. Files
  exist only when they have content. The numbered-home convention is unchanged. The
  tooling crates carry the same grammar inside their earned directories, under the same
  machine-enforced module order — `macros/macroc/src/derive_refusal/` is the worked
  precedent.
- One root `laws.rs` in the core crate carries the compile-time green laws, sectioned
  by home. It is non-public and outside the derived ABI surface. A law that cannot fail
  is not a law; every law is proven non-vacuous by reversal. It is the residue seat —
  see **The strongest seat** — so a law that remains there states why no stronger seat
  can hold its claim.
- Every obligation declared in a README carries a stable id, a typed `challenge_kind`,
  one positive control, one law-negating challenge, and an activation route. trybuild
  covers compile refusals only — it is one challenge kind, never the universal one.
- No hand-maintained inventories: counts, dependency maps, status tables, and public
  surface listings are derived by tooling, never authored.
- rustdoc is a spec surface: public items are documented at the declaration; the README
  carries home-level narrative; nothing is written twice.

## The strongest seat

Every claim lives at the strongest seat that can establish it, and no weaker seat
restates it. Local invalid states are unrepresentable in types; closed mappings live in
one typed register with generated projections; repository closure lives in xtask joins;
behavioral claims live in testpak; external mechanism claims live in evidence. The proof
surface (`laws.rs`) holds only the residue: a claim drains to types first, then the
compiler's own lints, then xtask/testpak/macros, and a law that remains must state why
no stronger seat can hold it.

A claim restated at a weaker seat is worse than one stated once, because the weaker
statement keeps passing after the stronger one is removed. That is why the drain runs
downward and never the other way: a type that makes the wrong move unrepresentable
retires the law that asserted the move was wrong, and the law goes.

## No naked surfaces

A new crate, subsystem, module family, macro family, generator, register, checker, host
adapter, or public contract is not admitted merely because it compiles or because its
positive laws pass. The same boundary must carry: its exact owner; its explicit claims
and nonclaims; its positive control; an independently owned challenge route; a planted
reversal proving that route activates; its expected and executed denominator entry; its
origin and invalidation relationships where applicable; its diagnostic and lawful repair
route where it may refuse; its toolchain, dependency, and trust posture.

A structural claim may not be deferred when its reversal is expressible now. A
behavioral claim may remain owed only when the relevant machinery does not yet exist and
the exact opening condition is named. No boundary may be described as complete, closed,
standing, or proven beyond the strongest claim its executed evidence supports.

## Reports and commit prose

Completion language names its plane and method. "Complete", "closed", "stands", and
"proven" are never written bare: each names the exact plane it holds on and the method
that established it there, and neither reaches past what was executed. A report leads
with what remains unproven, then the denominators, then the artifact that was built —
in that order, because a reader who stops after the first paragraph must be left with
the debts rather than the achievements.

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
  every member; no member may weaken it; `#[allow]` is forbidden. `#[expect]` is the one
  hatch: it names the exact lint, carries a reason stating the structural fact that makes
  the lint wrong there, and sits on the narrowest item that contains the concern.
  `clippy.toml` states the thresholds this repository CHOOSES, including the ones that
  restate a tool default — an inherited number is a number nobody decided. The lints the
  wall refuses are named on the record in the manifest beside it, because an unnamed
  absence reads as an oversight.
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
