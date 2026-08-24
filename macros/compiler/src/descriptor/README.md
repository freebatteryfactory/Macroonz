# `descriptor` — the three kinds that speak to the harness

Most of this compiler knows nothing about what it is generating. This home is the exception, and it is the exception on purpose: `macroonz-harness` is this workspace's own harness, so the compiler is allowed to know its vocabulary, and it writes it down in one place rather than scattering it through three renderers.

Three kinds live here.

| Kind | What one declaration produces | Where it lands |
| --- | --- | --- |
| [`TrialTable`](trial::TrialTable) | A stamped module of declared rows | The declaration site, inert inside the carrier's stamped seat |
| [`BenchTable`](bench::BenchTable) | A bench table and the one file that binds it to a measurement backend | The table beside the trial table's; the adapter as the consumer's bench-target cargo |
| [`MutationSurface`](mutation::MutationSurface) | The module a mutation harness lowers and invokes | The consumer's test target |

A stamped seat lands differently from an opaque one because it is a different material: stamp grammar is not Rust, so a stamped module delivered as opaque target cargo would be a syntax error in the consumer's build, and it rides the carrier's stamped seat instead — which the gate forwards to its stamp, never to the compiler.

## Nothing here is a constant a door cannot change

The helper attribute each grammar reads is a [`Grammar`] the caller declares, not a word baked in. A door registers the attribute it wants and hands the same value to the reading, so a refusal names what an author actually typed.

The producer's own act — the namespace it spells its facts under, the producer that emitted a table, and the door a declaration came through — is an [`Emitter`] the caller declares. A rendering composes those three into every row's origin and every table's provenance, and no authored declaration has a seat it could sign one with.

## What crosses, and what does not

What crosses the wall is conforming DATA in the harness's declared field shape. Not one harness type is imported, and the constructor-calling expressions a rendering writes name the address through the caller-supplied binding rather than through a dependency edge.

The producer writes letters to an address; it does not own the mailbox.

## Two crate bindings, and no crate name

Every rendered path is rooted at one of two [`Binding`] rows — the declaring crate whose operations a row measures or challenges, and the harness whose vocabulary the row is spelled in. Each is written as a metavariable the carrier's invocation binds, so a consumer that renamed either dependency gets its own name back and nothing here learns what the crate is called.

A callable living in the consumer's own target needs no binding at all: it arrives as an expression at the invocation, where that target's own hygiene reaches its own items.

## One vocabulary for a declaration's values

[`Name`], [`SupportName`], [`ModuleName`], [`TypeName`], [`FunctionName`], and [`BoundPath`] are what a declaration's values are. Every one of them refuses at construction, so a name that names nothing and a spelling a consumer's compiler would read as something else are values nobody can hold.

[`DeclarationError`] is how they refuse: seven shapes over one [`Seat`] roster, because what refuses is the SHAPE of the disagreement and which seat it was about is the other half of the same sentence. A bounded seat admitted later is one row on that roster rather than three rows of a refusal.

## Two helper readings, told apart by position

A declaration may carry two helper bodies — one trial, one mutation — and they are separated by the position each reading stands at, never by two roles. The families their diagnostics derive in are separated the same way, which is why each grammar's refusal is its own type: a diagnostic's family tag is a fact about the type, and one type for both readings would derive one related identity for two unrelated observations.

## Composition

[`Composition`] is the one declaration of which providers of descriptor material exist, so a consumer that must see all of them at once has a single declared set to read.

It is a declaration and not an inventory: naming, in one place, exactly which providers compose is a statement somebody made and can be held to, while an unchecked list is right when it is written and refuses nothing when it stops being right.

Duplicate-free by construction — the scan runs before the value exists — and structurally non-empty, because a composition with no provider is not a composition, it is silence.

## The homes

`vocabulary/` is what the harness publishes, in two tables.

`trial/`, `bench/`, and `mutation/` are the three carrier kinds, each with its own grammar, its own guard, and its own rendering; `shadow/` is the one direct kind, whose grammar chooses names off a stated roster and whose rendering writes both `cfg`-gated faces of each.

`door/` is the roads a generic entry walks, one per grammar: a captured body in, the sealed expansion out — a carrier with its axes composed from what each kind's own terminal proved for the three, and direct declaration-site items for the shadow face.

`types.rs` holds what the kinds share; `type_guard.rs` is its own child and holds every road that reaches a private field; `type_contract.rs` states how a refusal reads; `composition.rs` is the duplicate scan.
