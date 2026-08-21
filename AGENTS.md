# ThreadPak — Working Law

ThreadPak is an embedded, sync-first, event-native database and runtime in safe Rust,
named for the logical thread it preserves; the README owns the full product statement
and the machine-in-one-view.
This file is the binding law for any person, model, or agent working in this
repository, and it is the only one.
`CLAUDE.md` carries a pointer here and nothing semantic:
a law stated twice is a law that can disagree with itself,
and one that did is what cost this repository a week.

There is no CI and no gate.
The enforcement surface is the toolchain at the root, run locally:
the lint wall in `Cargo.toml`, `clippy.toml`, `deny.toml`, `.cargo/config.toml`,
the compiler itself, and the executed lanes.
Checks report; a human decides.

## The spine

The README owns the machine in one view — both columns: the program pipeline
(declarations → Semantic Form → Execution Form → ProgramImage → PakVM → the Stitch →
Bvisor) and the data authority (accepted events → durable history → derived
projections → receipts, replay, reconciliation). Hosts live in other repositories and
pin an exact ThreadPak revision; the machine never knows which host is running it.

## Format law

- The repository is the specification. There is no separate book. No semantic fact is
  manually restated in two places — cite the owner, never copy.
- The specification states what IS.
  No home carries a ban list, a negative-space ledger,
  or an inventory of the forbidden and the pending:
  what cannot happen is carried by the type that makes it unwritable.
  A dead decision leaves the tree; git history is its only tombstone.
  This file and the construction-phase rule pack are the two exceptions,
  because they address the author rather than the machine.
  A working law states prohibitions, and retires them when they stop being true.
- Numbered directories are dependency bands: band N imports only bands lower than N.
  Numbers live on directories only; module names stay clean via `#[path]`.
- The crate root owns generic composition shapes only. A semantic noun lives at the
  root solely by an explicit root admission decision; otherwise it has an owner home.
- A home's README is owner prose: why the boundary exists, what it claims, what it
  does not. No yaml blocks, no machine-parsed ledgers, no status tables.
- **The file grammar.** Every semantic home is its numbered folder carrying `README.md`
  + `mod.rs` + `types.rs`, files named by what they are about. `types.rs` owns the
  home's public types: a `pub` struct, enum, alias, or trait outside the owning
  `types.rs` refuses. Private implementation types live beside their algorithms and
  never leak into public signatures, bytes, or identities. As machinery earns existence
  the home grows exactly two more type-owned files:
  - `type_guard.rs` — smart constructors and the invariant nucleus, declared inside
    `types.rs` as `#[path = "type_guard.rs"] mod guard;` so it sees private fields.
    Parse-don't-validate lives here.
  - `type_contract.rs` — declarative trait implementations: refusal families, identity
    roles, register participation.

  Everything else is a role-named pure-function module — `validate.rs`, `encode.rs`,
  `decode.rs`, `project.rs`, `transition.rs` — consuming types already informed, so no
  function re-establishes what its argument's type already promised. A file about types
  takes the `type_` prefix and sorts beside `types.rs`; sort order is reading order.
  Files exist only when they have content.
- A reserved architectural coordinate is a directory carrying exactly one README that
  admits the seat is empty, states its question, and names the exact condition that
  fills it. No `mod.rs`, no placeholder types, no stubs dressing an empty seat as
  occupied.
- No test corpus is specification.
  A structural invariant lives in the type and its smart constructor:
  where a type can reach a claim,
  the type that makes its violation unwritable is the only statement of it.
  A lane observes what no type can state —
  behavior, cross-crate composition, expansion under renamed dependencies,
  encodings, diagnostics, external tools,
  and the compile refusal showing a violation really is unwritable from outside.
  There is no `laws.rs` and no proof-surface module:
  a file collecting assertions across unrelated homes is the defect,
  whatever it is called, and a lane never legislates meaning.
  Lanes are named for the behavior they exercise,
  live in the `tests/` of the crate that owns the road,
  and are reached only through the public surface.
  A `#[cfg(test)]` item inside the library is a road with an audience of one,
  and it refuses.
  A claim that is neither structural nor observed is not claimed.
- No hand-maintained inventories: counts, dependency maps, status tables, and public
  surface listings are derived, never authored.
- rustdoc is a spec surface: public items are documented at the declaration; the README
  carries the home narrative; nothing is written twice.

## The strongest seat

Every claim lives at the strongest seat that can establish it,
and no weaker seat restates it:
types first, then the compiler's own lints, then generated registers,
then one executed lane observing what no seat above it can state.
A claim no seat can establish is not claimed.
A claim restated at a weaker seat is worse than one stated once,
because the weaker statement keeps passing after the stronger one is removed.
The drain runs downward only:
a type that makes the wrong move unrepresentable retires the law that asserted it,
and the law goes.

## No naked surfaces

A new crate, subsystem, module family, macro family, generator, register,
or public contract is not admitted merely because it compiles.
The boundary carries: its exact owner; its claims and nonclaims;
the types that make its violations unwritable;
where a type cannot reach, the lane that observes the behavior instead;
its dependency and trust posture.
And it names what it deletes:
a component that deletes nothing and retires nothing
is refused until it says why it deserves to exist anyway.

A structural claim may not be deferred
when a type can make its violation unwritable now.
A behavioral claim may remain owed only when the machinery does not yet exist,
and the exact opening condition is named.
No boundary is described as complete, closed, or proven
beyond what its types actually enforce.

## Reports and findings

Completion language names its plane and method — "complete", "closed", and "proven" are
never written bare. A report leads with what remains unproven, then the denominators,
then what was built, in that order.

Green is not a goal; it is a side effect of the software being true. A finding stands
red until the owner rules on its substance — fix, regenerate, or delete the subject. No
agent edits code to silence a finding.

## Rustdoc law

- Every public item begins with one plain product-language sentence naming what
  it is. Additional sections are earned by the item's actual contract, never
  templated: document construction, authority, bounds, ordering, refusals,
  errors, or examples only where the caller needs the distinction.
- Headings are noun phrases — `# Construction`, `# Authority`, `# Bounds`,
  `# Errors`, `# Examples` — never sentence fragments or clauses.
- A nonclaim is written only where a reader could acquire false authority — a
  receipt read as proof of more, a capability read as a grant, a DataBlock read
  as accepted truth — never as furniture on ordinary items.
- Remove on sight: current population counts and "today/currently" status
  prose; migration and origin history; explanations of the defect that caused
  the type to exist; repository procedure; restatements of a linked owner's
  contract. Git owns history; the tree owns the census.
- One statement, one owner: a mechanism documented at its owner is linked,
  never restated.
- Length is a consequence, never a target: an ordinary shape is one to three
  sentences; a genuinely risk-bearing type may be long.
- Keep complete thoughts together: prefer one sentence per source line, and
  when a sentence must wrap, break at punctuation or a clause boundary — never
  inside a type name, a link, or an inline-code span.

## Vocabulary

- The runtime transition is the **Stitch**: one typed observation in, next state and
  effect intents out — reserved exclusively for the runtime transition, never reused
  for linking, joining, merging, or causal edges. `tick` is the clock's tick. `step` is
  PakVM's inner step. `Turn` is an identity. PEND is never spelled AWAIT.
- Canonical verbs: define, parse, decode, encode, validate, resolve, compile, lower,
  plan, execute, apply, commit, project, render, inspect, explain, dispose, sample,
  fork, merge, append, acknowledge, checkpoint, resume, pack.
- Ordinary computer-science vocabulary stays ordinary: `law`, `lawful`. The
  defect is never a noun; it is one claim holding two proof authorities, or a weaker
  proof outliving its owner.
- Trust is owner-scoped, never ambient: a trust claim names its typed boundary,
  members, authority, nonclaims, and evidence.

## Hard rules

- Safe Rust only. The lint wall is declared once at the root and inherited by every
  member; no member weakens it. `#[allow]` is forbidden; `#[expect]` is the one hatch —
  it names the exact lint, carries a reason stating the structural fact that makes the
  lint wrong there, and sits on the narrowest item that contains the concern.
  `clippy.toml` carries only chosen, live settings.
- NO PYTHON in this repository, ever. All tooling is Rust.
- The metaprogramming services project contracts; they never decide meaning and are
  never their own oracle. `macros/macroc/` is ordinary callable Rust — planning,
  rendering, inspection, explanation; `macros/proc/` is one thin, semantically empty,
  dependency-minimized expansion shell over it. Expansion is deterministic from its
  declared input: no network, no filesystem scan, no environment, no clock, no entropy.
  Composition is proven from an outside consumer that compiles against the public
  road, never from inside a participant.
- testpak depends inward on core; nothing depends on testpak. Production never depends
  on its judge.
- Probes (throwaway compiler experiments) never enter this repository.
- LF everywhere; no symlinks; no `build.rs`.
- Semantic compilation, projection, identity construction, and generated meaning are
  ambient-free: no host fact influences a result unless an owner-declared input carries
  it. A test harness may read the host facts it needs to run; they never enter a
  semantic identity or decision.
- Frontends plug in from outside through the public declaration path with zero core
  changes. If a frontend needs a core change to exist, the declaration contract is
  wrong.

## Phase gate

**Under construction — the concrete is not hard yet.** During the build-out, the
spec files carry two kinds of text and say which is which: ground rules and
mental models (the door law, the process arrow, the depot pattern, the zero-
compile phase — hard enough to keep anyone from walking into an already-walked
rake), and drying structure that hardens only at the owner's blessing. Text
describing machinery mid-absorption is read as the standing state, not as
finished law; the hard-law versions land when the things they govern exist.
While the concrete dries, authority runs one way: the landed READMEs and
types are the spec, and standing code that disagrees with them is the defect
being drained — never a counterexample to the spec, and never a precedent to
imitate.

The generation system is the product line: families are authored through front doors
and their contracts are generated. The hand-written contract code standing in `src/`
today is scaffolding awaiting re-authoring once the doors exist. The door law is
settled: every door is a thin shell over the one callable engine, and equivalent
declarations through different doors produce equivalent contracts. Core never
carries a compile-time dependency edge to the proc-macro crate. Core-local
declarative stamps are standing law (`closed_register!` and `scope_guard_version!`
are the exemplars); generation beyond a stamp's reach lands in core as published
source under a receipt; the derive is the outside consumer's door.

The harness (testpak) and the generation services (macros) were built in parallel
as one phase under zero compilation:
implementers wrote complete code end to end —
no stubs, no placeholder bodies, no compiler-appeasement —
and QA was read-only review.
That phase closed at first toolchain contact,
which was quiet and corrective rather than a ceremony.
The toolchain now stands and the lanes execute;
a red lane is evidence about the code, never an instruction to edit the lane.

Next comes the trust opening:
baseline qualification, then wrap-first mutation pressure,
then the mandatory no-mutation parity —
only then are interpreted-mutation results and rewrite-produced descriptors
admitted as evidence.
The harness's ladder runs against this repository there,
the one subject that costs something to grade.
Then the migration pass, where the generator runs its own migration, never a hand;
then the per-home source pass with the owner in the loop.
The blessing-day ceremony — the native-clone dual-target runs
and the packaged outsider check — crowns the COMPLETE machine after all of that,
and CI is designed fresh, last.
Approvals and dial-downs are recorded in the crates' own READMEs and types,
because the repository is the spec.

No product-runtime implementation opens in any home without explicit human
authorization. Commits are decided by a human — never initiated by an agent.
