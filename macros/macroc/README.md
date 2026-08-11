# macroc — the metaprogramming services

The services are ordinary callable Rust — capture, planning, rendering, closure,
inspection, explanation — reached the same way by any caller. They depend inward
on the machine and never back outward: nothing here knows a proc-macro exists.

The crate's own doc comment carries the charter and the dependency order; this
file carries what a README owes that rustdoc does not: **the qualification
obligations this tooling stands under.**

## Tooling obligations are their own category

A core semantic obligation is a claim about the MACHINE — what a home's types
make unrepresentable, what a law proves, what a reversal breaks. Its denominator
is the red-twin ledger.

A **tooling qualification obligation** is a claim about a TOOL — what a service
refuses, what a check catches, what a judge is rehearsed against. It has its own
denominator and its own reversals, and the two are never added together. A
repository that reported "178 obligations, 5 discharged" over both populations at
once would be reporting a number nobody can act on: the two populations are
challenged by different methods, owned by different homes, and are complete on
different days. `cargo xtask check` prints them apart, always, on every run.

Each block below binds seven things: the CLAIM, the OWNER module, the POSITIVE
control, the REVERSAL, the ACTIVATION route, the METHOD, and the NONCLAIMS.

```yaml
tooling-obligation: macroc.capture-refuses-a-malformed-declaration
  claim: >
    A declaration the authored grammar does not admit reaches the compiler as a
    refusal naming the established cause and the offending token — never as a
    silent empty expansion and never as a smaller success.
  owner: macros/macroc/src/derive_refusal/capture.rs
  positive: xtask/fixtures/macro-consumer/src/lib.rs
  method: compile-refusal
  activation: cargo test -p threadpak-testpak --test compile_refusals
  tooling-red: testpak/tests/compile-fail/a-malformed-refusal-declaration-refuses.rs
  nonclaims: >
    It does not claim the grammar admits every well-formed Rust enum, and it does
    not claim the refusal text is stable across releases.

tooling-obligation: macroc.the-receipt-rich-road-is-the-only-road
  claim: >
    Tokens are emitted only from a closed expansion, and a closed expansion
    exists only once the plan, the origin graph, the trace, the rendering, the
    closure, and the explanation have all been produced and have agreed.
  owner: macros/macroc/src/derive_refusal/mod.rs
  positive: macros/macroc/src/laws.rs
  method: executable-law
  activation: cargo test -p threadpak-macroc
  tooling-red: owed-to-testpak — a controlled mutant deleting each seat in turn
  nonclaims: >
    It does not claim the rendering is correct Rust; that is lane C's claim, and
    it is made by the consumer fixtures.

tooling-obligation: macroc.the-crate-binding-travels
  claim: >
    A consumer that renamed its dependency gets a rendering naming the crate by
    the name that consumer uses, because the binding is captured, planned,
    explained, and rendered rather than assumed.
  owner: macros/macroc/src/derive_refusal/render.rs
  positive: xtask/fixtures/renamed-consumer/src/lib.rs
  method: compiled-behaviour
  activation: cargo test -p threadpak-renamed-consumer
  tooling-red: owed-to-testpak — a renderer hardcoding the default binding
  nonclaims: >
    It does not claim the machine is reachable under an arbitrary path; only
    under a crate name the consumer's own manifest declares.
```

## What the services never do

They decide no meaning. The three body shapes are band 00's; the canonical cause
key grammar is band 00's; the selection order's content is the author's. A
tooling type may summarize, reference, plan, explain, or project an owner fact;
it may never create a second value that independently answers the owner's
semantic question — which is why an unanchored diagnostic says it is unanchored
rather than carrying a minted stand-in.
