# `pattern_stamp` — the pre-magic story of a declarative stamp

Planning the account the machine's `scope_guard_version!` pattern owes.

## Why a declarative stamp still owes a plan

The stamp itself is a `macro_rules!` in the machine's identity home — the home
that owns the Class-C shape stamps it, and nothing here writes a byte of it. What
the services owe is the account: which authored pattern was instantiated, with
which typed arguments, on whose declared facts, producing which complete output
set, and what would make that account stale.

That account is a `ProjectionPlan` over
[`PatternStampProjection`](crate::planning::PatternStampProjection) like any
other, and building one here is the proof that the plan family carries a
*declarative* stamp and not only a derive: the pattern kind was already in the
sealed roster, and this home shows the roster meant it.

## The services mint nothing

Every identity a stamp plan carries names something the machine owns — the closed
graph, the profile, the declaration that caused it, the authored pattern, the
instantiation, the typed arguments, the generated unit. The caller supplies them
as [`ScopeGuardStampAnchors`]; this home reads them and adapts none. Nothing here
observes the stamp's expansion, and nothing here decides what the stamp means.

## The seats

`types.rs` declares the two anchor records. Every seat on both is public and
required — a stamp plan that could omit its pattern, its instantiation, or its
arguments would be an account that sometimes says less than it knows — so there
is no private field, no invariant nucleus, and no `type_guard.rs`. `plan.rs` is
the one road: it reads the anchors and returns the plan, or the planning family
naming the magnitude it could not fit inside.
