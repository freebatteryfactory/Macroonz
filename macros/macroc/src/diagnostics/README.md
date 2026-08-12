# diagnostics — what the services say when something disagrees

One typed value per observation, and every rendering of it is a projection of
that value.

## One value, many faithful projections

A diagnostic is one typed value. The compiler-facing rendering, the
machine-readable rendering, and the rendering an agent is handed are projections
of that one value — they may differ in shape, ordering, and verbosity, and they
may never differ in what they claim. A projection that upgrades a narrowed
suspect into an established cause, or a suggestion into an authority, has changed
the claim and is not a projection of it.

## Repairs are owner-declared, never invented

Every [`RepairAction`] cites the owner fact that declares the repair. The
services do not compose advice: they report which declared repair applies. And
the standing prohibition: no repair ever suggests deleting a declared capability
so that generation compiles. Making the machine smaller until the services stop
complaining is not a repair, it is a silent narrowing of what the program
promised.

## The seats

`types.rs` declares. Nothing here has a private field, so the home has no
invariant nucleus to guard and no `type_guard.rs` exists to hold one — every seat
of a diagnostic is public because a diagnostic that hid a seat would be a
diagnostic that sometimes says less than it knows. `type_contract.rs` states the
home's one declarative table: how a span table's answer becomes a site posture,
and how that posture reads back.

This home's qualification obligations live in the crate README's tooling-obligation
blocks.
