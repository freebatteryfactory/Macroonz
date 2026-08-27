# Macroonz

<p align="center">
  <img src="assets/logo.webp" alt="Macroonz — Bake. Build. Delight." width="480">
</p>

**Code generation that can prove what it made, and a test harness that tries to break it.**

Ferris bakes macaroons here.
Every batch starts from a written recipe, every tray is checked against that recipe before it leaves the oven, and a taste tester with no loyalty to the baker gets the first bite.

> Macroonz knows nothing about your domain.
> Your types, your errors, your identities, your bounds — all yours.
> It only knows how to bake exactly what you asked for, and how to find out whether it's any good.

---

## Why

Writing a derive macro means parsing a token stream, emitting another one, and hoping.
Nobody can say afterwards what was generated, why, or what would change it.
Testing a library means writing the examples you thought of.
The bug is in the one you didn't.

Macroonz replaces both hopes with records.

| You have | You get |
| --- | --- |
| A derive that emits tokens | An expansion that names every unit it produced, proves the set matches its plan, and explains each decision — before a byte reaches `rustc` |
| A handful of example tests | Generated inputs, injected faults, a controlled clock, mutants of your own code, and the reproduction material each road earns — generation keeps its seed when generation drove the run, reduction mints the smallest witness it reached, and the replay capsule carries it |

---

## The bakery

One storefront opens onto the oven, the hand that loads it, and the taste tester without pouring their vocabularies into one bowl.

| Crate | Directory | What it is |
| --- | --- | --- |
| **`macroonz`** | `/` | The storefront. It preserves the owners as `compiler`, `macros`, and feature-gated `harness` modules. This is the crate you add. |
| **`macroonz-compiler`** | `macros/compiler/` | The compiler, as ordinary functions. Capture a declaration, build a request, plan, render, close, explain, bind, emit. |
| **`macroonz-macros`** | `macros/proc/` | The thin procedural host. Item-preserving attributes and direct declarations call compiler-owned descriptor doors, then carry token conversion, span custody, diagnostic placement, and emission. It owns no grammar. |
| **`macroonz-harness`** | `harness/` | The judge. Descriptors, generation, properties, oracles, faults, corpus, fuzz composition, mutation, benches, reports, replay. The default storefront includes it; the diet posture removes it from a shipping graph. |

```mermaid
flowchart LR
    YOU["your crate"] --> F["macroonz"]
    F --> C["macroonz-compiler"]
    F --> PROC["macroonz-macros"]
    PROC --> C
    F -. harness feature .-> H["macroonz-harness"]
```

Arrows point at dependencies.
The compiler depends on nothing in this workspace.
The proc crate reaches the harness only from its tests, and the harness reaches the compiler only from its tests.

---

## Your recipe, your kinds

A **kind** is a thing you can ask Macroonz to generate: one `impl` block, a codec pair, a test carrier, a documentation page.
You define it.
A kind says what content it is rendered from, which roles its output units play, and which questions its explanation has to answer.
The compiler is generic over all of that; it has never heard of your kind and does not need to.

```rust
use macroonz::compiler::{Kind, NoQuestions, Request};

/// One `impl Greet` for the declared type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GreetImpl;

impl Kind for GreetImpl {
    const NAME: &'static str = "greet.impl";
    type Content = Greeting;
    type Role = GreetRole;
    type Question = NoQuestions;
}

pub fn greet(input: TokenStream) -> TokenStream {
    macroonz::compiler::host::expand(input, |capture| {
        let greeting = Greeting::read(&capture)?;
        Request::<GreetImpl>::over(capture, greeting, &GREET_DOOR)
            .render(|plan, out| out.unit(GreetRole::Impl, plan.content().impl_tokens()))
    })
}
```

`Greeting::read` is yours: your grammar, your rules, your refusals.
`Greeting` implements `CanonicalContent`, giving the kind-specific facts one complete semantic encoding that changes whenever a fact the renderer may read changes.
`GREET_DOOR` is the one value that says who is asking — the diagnostic spellings, rendered crate binding, and producer qualification each compiler surface reads.
Macroonz never parses your declaration for you and never decides what it means.
This is the same split `serde` makes — `serde_derive` ships with `serde`, not with `syn` — and it is the only split that keeps a generator honest.

---

## The road

Every request walks the same eight steps, whatever the kind.
Each step hands the next a value it cannot forge.

```mermaid
flowchart LR
    A["1 · account"] --> I["2 · intent"] --> X["3 · context"] --> P["4 · plan"]
    P --> R["5 · render"] --> CL["6 · close"] --> E["7 · explain"] --> B["8 · bind"]
```

1. **Account.** The kind-specific content bound to its exact captured declaration and owner-qualified kind, plus every independent captured dependency it declares.
2. **Intent.** What it means: an identity over the owner-qualified kind and content commitment. Two callers who meant the same thing derive the same intent.
3. **Context.** Which profile and which generator version are answering.
4. **Plan.** The complete output set, named before a byte of syntax exists — each unit's role, semantic key, destination, origin, expected profile, and digest contract — plus the invalidation set, the decision trace, and the nonclaims.
5. **Render.** Typed tokens into rendered units, each digested over its own canonical bytes.
6. **Close.** The membership is rebuilt from what was rendered and proved equal to the plan, role by role. The units are partitioned by the destination each one declared.
7. **Explain.** Every question the kind owes is answered once, over that plan and that closure, under an identity derived from both.
8. **Bind.** Plan, closure, and explanation are sealed together, after the compiler establishes that the three name one another.

The sealed expansion is the one value emission is read from.
`emit()` hands a proc macro its tokens; a test carrier, a bench carrier, or a publication step reads its own partition from the same value.

> A request that cannot walk the whole road is refused whole.
> There is no partial output.
> A refusal is never a smaller success.

Expansion is a function of its declared input.
No network, no filesystem scan, no environment, no clock, no entropy — there is no seat where one could enter.

---

## The taste test

You describe a subject once: what it takes, what it returns, what it refuses, what must hold.
The harness hands you the instruments — each independently callable, composed by your own tests rather than by one button:

- **Generates** inputs against the description, structure-aware, from a seed it records.
- **Injects** faults on a declared schedule, and measures against a clock the caller declares, so the subject is judged under pressure and not on a sunny day.
- **Reduces** a failure to the smallest witness reached under the declared reducers and budget, and mints a replay capsule over it.
- **Hands off** coverage-admitted bytes into that same reduction and replay road when a selected native backend is in use.
- **Mutates** the subject's own code and runs the trials against each mutant, to prove the trials can tell right from wrong.
- **Benchmarks** with the same receiver and the same pinned profile, so a number means the same thing tomorrow.
- **Reports** each verdict with its standing, its site, and its complete denominator — joined to its replay capsule, where a reduction earned one, on one execution key.

Descriptors, trials, mutations, and benches live in your tests — written through the generic `macroonz::macros` attributes, through your own attributes, or by hand.
The harness owns how they are judged, never what they mean.

---

## Getting started

1. Pick a posture from [The three postures](#the-three-postures).
2. Add `macroonz` with that command.
3. Follow the compiler, proc, or harness crate README for the road you are on.

Contribution procedure lives in [`CONTRIBUTING.md`](CONTRIBUTING.md).
Security reporting lives in [`SECURITY.md`](SECURITY.md).

---

## The three postures

Cargo features are additive, so the lighter postures are selected by turning off the default before adding back only the door wanted.

| Posture | Command | Surface |
| --- | --- | --- |
| **full** | `cargo add macroonz` | Compiler, proc declarations, harness, and the target-qualified preemption backend. This is the default. |
| **diet-lite** | `cargo add macroonz --no-default-features --features harness` | Compiler, proc declarations, and harness without Loom. |
| **diet** | `cargo add macroonz --no-default-features` | Compiler and proc declarations only. |

The `preemption` feature always implies `harness`.
On a native target supported by the pinned Loom backend, enabling `preemption` installs that backend.
On every other target, including Wasm, the same harness result plane remains available and reports typed backend unavailability instead of trying to compile Loom.

The `fuzz-frida` feature is a separate opt-in.
It forwards to `macroonz-harness/fuzz-frida` and stays out of every posture above.

---

## Working here

[`AGENTS.md`](AGENTS.md) is the working law for anyone — person, model, or agent — who edits this repository, and it owns what enforcement means here.
The wall it names, run locally:

```sh
cargo check  --workspace --all-targets --features full
cargo clippy --workspace --all-targets --features full
cargo nextest run --workspace --features full --run-ignored all
cargo test -p macroonz-harness --features preemption --run-ignored all
cargo fmt --all -- --check
cargo deny --workspace check
```

The ordinary wall exercises the default product postures, including the target-qualified Loom-backed `preemption` road through `full`.
It does not enable `fuzz-frida`; that opt-in road is qualified separately under stable Rust 1.98.

---

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
