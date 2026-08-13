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

`types.rs` declares. Every seat of a diagnostic is READABLE, because a diagnostic
that hid a seat would be a diagnostic that sometimes says less than it knows.

Readable is not the same as writable, and the difference is what earns this home
its `type_guard.rs`. Two values here — `RelatedSet` and `RelatedSetTruncation` —
seat facts about an ACT the services performed rather than values a caller
holds: which identities a set-building road kept, how many it left behind, and
which body those identities are about. A caller able to write them could state
that identities were dropped by a set that dropped none, or seat one refusal's
coarse commitment over another refusal's issues, and both halves would read as
honestly derived. So those seats are private, the guard holds the one road that
reaches them, and the road takes the issue MATERIAL and derives the body's
identity and the per-issue identities together — there is no half to mispair
because there is no half a caller ever holds.

`type_contract.rs` states the home's one declarative table: how a span table's
answer becomes a site posture, and how that posture reads back.

This home's qualification obligations live in the crate README's tooling-obligation
blocks.
