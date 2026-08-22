# question — the explanation protocol's closed question roster

The questions every generated thing must be able to answer, and the typed answer to "does this kind admit that question at all". Nothing else lives here:

**the roster is a vocabulary, not machinery.**

## Why the roster is its own home

Both ends of the protocol need the questions. A projection kind declares its roster while it is being PLANNED, before any explanation exists; the explanation machinery reads that roster while it is being CHECKED, after the plan exists.

Left in the machinery module, that pair of needs is a cycle — planning importing explanation, explanation importing planning — and a cycle is a dependency order nobody can state. Seated here, the roster is a leaf both sides import, and the order is a straight line again.

The home names nothing from this crate, and that is the point:

a closed roster of names is the one thing in the plane with no machinery to depend on. The one thing it takes from the machine is the authoring stamp that writes a closed roster down.

A stamp decides no meaning, carries no semantic noun, and reaches no band's material, so taking it costs the leaf nothing it was protecting — what the leaf protects is the absence of an edge to another module of THIS crate, and that absence is exactly as complete as it was.

## The seats

`types.rs` declares, and that is the whole home. There is no `type_guard.rs` because no declaration here has a private field to guard, and no `type_contract.rs` because the roster's own table is written by the authoring stamp rather than restated.

A file exists here only when it has content.
