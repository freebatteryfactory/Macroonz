# `descriptor` — the kinds that speak to the harness

Most of this compiler knows nothing about what it is generating.
This home is the bounded first-party adapter: it targets the public `macroonz-harness` constructor surface and the target-qualified Loom shadow surface, and it owns that destination vocabulary in one place rather than scattering it through the neutral compiler.
It reads declared meaning and physical dependency paths, renders the destination shape, and invents no harness policy, owner fact, or default.

Six kinds live here: three carrier kinds, and three direct ones.

| Kind | What one declaration produces | Where it lands |
| --- | --- | --- |
| [`TrialTable`](trial::TrialTable) | A stamped module of declared rows | The declaration site, inert inside the carrier's stamped seat |
| [`BenchTable`](bench::BenchTable) | A neutral bench table and one typed target-supplied report reader | The table in the stamped seat; the report reader as bench-target cargo |
| [`MutationSurface`](mutation::MutationSurface) | The module a mutation harness lowers and invokes | The consumer's test target |
| [`ShadowFace`](shadow::ShadowFace) | Both `cfg`-gated faces of every chosen synchronization name | The declaration site, as ordinary items |
| [`NetworkModule`](network::NetworkModule) | The builder module for a declared topology and its fault schedules | The declaration site, as ordinary items |
| [`ConcurrencyModule`](concurrency::ConcurrencyModule) | One generic exploration function per declared row | The declaration site, as ordinary items |

A stamped seat lands differently from an opaque one because it is a different material: stamp grammar is not Rust, so a stamped payload delivered as opaque target cargo would be a syntax error in the consumer's build, and it rides the carrier's stamped seat instead — which the gate forwards to its stamp, never to the compiler.

## Nothing here is a constant a door cannot change

The helper attribute each grammar reads is a [`Grammar`] the caller declares, not a word baked in.
A door registers the attribute it wants and hands the same value to the reading, so a refusal names what an author actually typed.

The producer's own act — the namespace it spells its facts under, the producer that emitted a table, and the door a declaration came through — is an [`Emitter`] the caller declares.
A rendering composes those three into every row's origin and every table's provenance, and no authored declaration has a seat it could sign one with.

## What crosses, and what does not

What crosses the wall is conforming DATA in the harness's declared field shape.
Not one harness type is imported, and the constructor-calling expressions a rendering writes name the address through an informed binding rather than through a dependency edge.

The producer writes letters to an address; it does not own the mailbox.

## Recipe composition

The recipe home may carry declared trial, mutation, benchmark, network, and concurrency material through these same kinds.
It does not translate them into recipe-owned evidence types or reproduce their constructor vocabulary.

Each declared block is read by its existing grammar, rendered by its existing kind, and delivered through its existing direct or support-carrier road before the recipe projection accounts for the resulting output.
Compile, property, temporal, generation, fuzz, fault, schedule, package, and publication claims remain caller-owned meanings stated through trial rows rather than new compiler policies.

## Two binding postures, and no hardcoded crate name

Carrier cargo roots every harness path at the support carrier's logical harness metavariable.
Callables and other target-owned values arrive as expressions, so no declaring-crate path needs a second logical binding.
A consumer that renamed the harness gets its own name back and the declaration never learns what the crate is called.

The shadow, network, and concurrency projections compile immediately where their direct macro stands, so each declaration supplies a [`DirectBinding`] containing the physical path that scope resolves.
One segment addresses a renamed dependency; several address a facade re-export.
The shared binding reader consumes the entire path, and each direct renderer reads only that informed value.

The physical path participates in the declaration's canonical content because changing the path changes the exact Rust the projection renders.

A callable living in the consumer's own target needs no binding at all: it arrives as an expression at the invocation, where that target's own hygiene reaches its own items.

## One vocabulary for a declaration's values

[`Name`], [`SupportName`], [`ModuleName`], [`TypeName`], [`FunctionName`], and [`DirectBinding`] are what a declaration's values are.
Every one of them refuses at construction, so a name that names nothing and a spelling a consumer's compiler would read as something else are values nobody can hold.

[`DeclarationError`] is how they refuse: seven shapes over one [`Seat`] roster, because what refuses is the SHAPE of the disagreement and which seat it was about is the other half of the same sentence.
A bounded seat admitted later is one row on that roster rather than three rows of a refusal.

## Three helper readings, told apart by position

A declaration may carry three helper bodies — trial, bench, mutation — and they are separated by the position each reading stands at, never by three roles.
The families their diagnostics derive in are separated the same way, which is why each grammar's refusal is its own type: a diagnostic's family tag is a fact about the type, and one type for several readings would derive one related identity for unrelated observations.

## Composition

[`Composition`] is the one declaration of which providers of descriptor material exist, so a consumer that must see all of them at once has a single declared set to read.

It is a declaration and not an inventory: naming, in one place, exactly which providers compose is a statement somebody made and can be held to, while an unchecked list is right when it is written and refuses nothing when it stops being right.

A composition is structurally non-empty and duplicate-free, because a composition with no provider is silence and two declarations of one provider identity do not name two providers.
Absence and magnitude refuse before duplicates are explored, so pairwise work ranges only over an admitted provider set.
Every refusal carries its complete finding set and cites the descriptor meaning that repairs the disagreement.

## Ownership

The shared descriptor vocabulary owns author-spelled names, physical bindings, provider composition, and the refusal shapes common to these declarations.
The shared fault bank owns the generated arms and harness refusal paths used by more than one direct descriptor projection.
Each kind owns its own grammar, informed declaration, output shape, and rendering, while each door owns the complete crossing from captured input to one sealed expansion.
The harness owns the constructor meanings these projections address, and the proc host owns only the compiler-facing act of carrying them.
