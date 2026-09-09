# network

A direct network declaration becomes the harness builders its declaration site compiles.

## Boundary

This child is the compiler descriptor adapter for one question: what conforming harness constructor data does the declaration state?
Capture admits the physical harness path, names, directed links, schedule membership, and fault material that the authored tokens can settle.
Rendering uses only that informed declaration to emit one module containing the topology and schedule builders.
The generated functions still call the harness's public smart constructors, so harness-owned value refusals remain harness-owned rather than being predicted here.

The physical harness path is declared input because a direct expansion compiles where it stands.
A renamed dependency and a facade re-export therefore cross through the same binding instead of a guessed crate name.

## Composition

Capture preserves authored order and resolves each schedule phrase to the link it names before a declaration can exist.
Canonical content commits to that ordered meaning and the physical binding before planning.
The descriptor door walks the generic request road and delivers the rendered module as one declaration-site unit.

The declaration chooses no campaign, opens no simulation, and judges no network behavior.
Those decisions remain with the adopter and the harness network home.

## Authored grammar

```text
<helper>! {
    harness = <dependency path>,
    module = <ident>,
    namespace = "<owner>",
    nodes = [<ident>, ...],
    link <ident> = <node> to <node>,
    schedule <ident> = [<fault phrase>, ...],
}
```

Clause order is free and is read by key; roster order is meaning and is preserved.
The reading walks the clauses in passes — the names first, then the links against the nodes, then the schedules against the links — so every clause may stand wherever its author put it.

The fault phrases are `drop link at ordinal`, `duplicate link at ordinal`, `delay link at ordinal by span`, and `partition link from tick until tick`.
Each phrase names a declared link; ordinals and delay spans fit `u32`, and partition ticks fit `u64`.
The binding, module, namespace, nonempty node roster and at least one link are required; schedules may be absent, and `schedule quiet = []` declares a schedule with no faults.
