# `pattern_stamp` — the account a declarative stamp owes

Planning the generation of the machine's `scope_guard_version!` stamp: which
authored pattern was instantiated, with which typed arguments, on whose declared
facts, producing which complete output set, and what would make that account
stale.

## The plan, not the stamp

The stamp itself is a `macro_rules!` in the machine's identity home — the home
that owns the Class-C shape stamps it, and nothing here writes a byte of it. The
account is a `ProjectionPlan` over
[`PatternStampProjection`](crate::planning::PatternStampProjection) like any
other, and building one here is what shows the plan family carries a
*declarative* stamp and not only a derive: the pattern kind is in the sealed
roster, and this home shows the roster meant it.

## Anchors in, nothing minted

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
the one road: it reads the anchors and states the account — the complete output
set, the origin trail, the decisions in selection order, and the watch set the
plan context derives — or refuses with the planning family, naming the magnitude
it could not fit inside.
