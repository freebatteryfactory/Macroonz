# src — the root calculus

The root is the floor under every band: the generic composition shapes the
whole crate instantiates, and nothing else. It imports nothing; band 00 is its
first consumer. `lib.rs` is its module surface (the root has no `mod.rs`) and
`types.rs` holds its shapes. The shapes carry the guarantee themselves: a type
that makes a wrong move unrepresentable leaves nothing for a separate assertion
to say.

## What the root owns

- The limit calculus: `Limit` families and their two capacity roads
  (declared and evidence-selected), the admission witnesses (`AdmittedLimit`,
  `PositiveLimit`, `LimitWitness`, `PositiveLimitWitness`), and the bounded
  collections (`Bounded`, `NonEmptyBounded`). There is no public unbounded
  collection anywhere in the machine.
- The transition grammar: `TransitionSystem` and `Dispatch` — the closure bar
  each owner's own state machine proves against, never a universal state type.
- The typed-reference shape (`EvidenceRef`) and the non-erasable-domain
  completeness shape (`Completeness<D>`): owners instantiate it under their own
  names, so a complete query can never masquerade as complete verification.
- Two root-admitted axes, by explicit decision: freshness (`Current` / `Stale`,
  types not variants) and `ProofDisposition`. Both are evidence facts, not
  knowledge axes — neither can say "not yet".
- `closed_register!`, the composition stamp every closed roster is declared
  through, and `CLOSED_REGISTER_ROW_CEILING`, the one value it projects.
- The depot, by explicit root admission: the bank of data-shaped truth —
  error prose, golden vectors, hostile and fault shapes, roster tables — as
  doc-commented constants. Data only, never behavior; its own README carries
  its law. It sits at the root because every band and every crate on the
  machine may read a fact, and a fact has no band.

## What the root refuses to own

A semantic noun lives here only by an explicit root admission decision;
everything else has an owner home. No production default limit profile: a
plane's ceiling is declared where that plane's seats are declared, and a
default seated here for convenience would become a ceiling nobody decided. No
universal status, result, uncertainty, or receipt type: each operation composes
only the axes it answers.

## Crate-wide laws the root states once

- **The structural spine.** A compile-time shape makes the wrong move
  unrepresentable, and a runtime-validated fact carried in the value's own
  canonical bytes enforces the right move at the operation boundary. The shape
  never decides — it makes bypassing the runtime check impossible.
- **The opaque-newtype obligations.** Every role-distinct public type is
  opaque; minted only by its owner; `Eq`/`Hash`; no `Ord` beyond a declared
  raw-byte storage order; serialized through an explicit codec only; no public
  constructor, no `Default`, no cross-family `From`; wrong-role construction
  does not compile, and wrong-role decode refuses.
- **Crossings never gain.** At every boundary crossing, uncertainty only
  widens, budgets only shrink, authority only attenuates, and information
  classification only restricts. Each reverse direction is a named,
  authority-bearing morphism that consumes new evidence and leaves a receipt.
- **Result conventions.** `ASK` is pure and publishes nothing; `DO` admits a
  bounded effect batch after required evidence and decisions pass; `REQUEST`
  durably admits an asynchronous effect intent; `PEND` admits the same durable
  intent and performs one immediate bounded attempt. `bool` is a result axis
  only for questions decidable-total from data in hand. Only the knowledge
  axes (`Truth`, `CommitKnowledge`, `OutcomeKnowledge`) may say "not yet"; an
  owed-but-not-performed posture spells itself `Outstanding` or `Unresolved`,
  never `Pending`. `Freshness` and `ProofDisposition` are evidence facts, not
  knowledge axes — neither can say "not yet".
- **Standing prohibitions.** No universal uncertainty wrapper and no parallel
  belief store. One owner per public type: every public type has exactly one
  owning home defining its body; all others reference it. A projection may
  adapt syntax, transport, or presentation; it may never change identity,
  schemas, authority, capabilities, bounds, effects, results, refusals, or
  evidence meaning.

## The band map

The numbered homes under this directory are dependency bands: band N imports
only bands lower than N. The repository README carries the full band map; each
home's README carries its own narrative.
