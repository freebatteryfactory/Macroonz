# src — the root calculus

The root is the floor under every band: the generic composition shapes the
whole crate instantiates, and nothing else. It imports nothing; band 00 is its
first consumer. `lib.rs` is its module surface (the root has no `mod.rs`),
`types.rs` holds its shapes, and `laws.rs` is the crate's one compile-time
proof surface, sectioned by home. It shrinks: a type that makes a wrong move
unrepresentable retires the law that asserted the move was wrong.

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

## What the root refuses to own

A semantic noun lives here only by an explicit root admission decision;
everything else has an owner home. No production default limit profile: a
plane's ceiling is declared where that plane's seats are declared, and a
default seated here for convenience would become a ceiling nobody decided. No
universal status, result, uncertainty, or receipt type: each operation composes
only the axes it answers.

## The band map

The numbered homes under this directory are dependency bands: band N imports
only bands lower than N. The repository README carries the full band map; each
home's README carries its own narrative.
