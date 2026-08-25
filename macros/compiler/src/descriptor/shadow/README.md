# shadow

One declaration, two worlds.

A crate whose concurrency should be explorable under a preemption scheduler needs its synchronization primitives to resolve to the scheduler's shadow types when the exploration build asks, and to the standard library the rest of the time.
The ecosystem's convention for that switch is the `loom` configuration flag — and the ceremony it usually costs an author is a hand-written pair of `cfg`-gated imports for every primitive, in every crate, forever.

This home absorbs that ceremony.
A shadow declaration chooses names from a stated roster, and the rendering writes the two faces the author would have written by hand:

```rust,ignore
shadow! {
    loom = renamed_loom,
    names = [Arc, Mutex, AtomicUsize, thread],
}
```

becomes, for each chosen name, exactly

```rust,ignore
#[cfg(not(loom))]
pub use std::sync::Arc;
#[cfg(loom)]
pub use loom::sync::Arc;
```

The `loom` clause is the physical path this scope uses for the shadow vocabulary.
The adapter contributes the roster suffix, while the declaration supplies the dependency alias or facade path that roots it.

Write the declaration once in a module of the production crate, import through that module everywhere, and the crate is explorable under the shadow scheduler for the rest of its life — no gate is hand-spelled again.

## The roster is the contract

The covered names live in one stated table: each row is the chosen spelling, its standard-library path, and its shadow path.
A name outside the roster refuses at its own token with a typed cause, which is the honesty a hand-rolled re-export could never give — a typo there is a bare resolution error three crates away, a typo here is this grammar naming the roster.

The roster grows a row when the shadow library covers the primitive, and not before: a row nothing realizes would be a declaration that compiles into a lie under the flag.

Two rows are macros rather than types — `thread_local` and the `mpsc` module's siblings ride the same `pub use` road, because Rust re-exports a macro by path like anything else.
One spelling caveat rides the `thread_local` row: the `const { … }` initializer block belongs to the standard macro alone, so a declaration meant to live under both faces states the classic initializer.

## What stays the author's

Two declared rows in the adopting crate's own manifest, exactly as the shadow library's documentation prescribes, and the declaration's matching `loom` binding:

- `[target.'cfg(loom)'.dependencies]` naming the shadow library — a row that compiles into nothing under every ordinary and release build;
- `[lints.rust] unexpected_cfgs = { level = "warn", check-cfg = ["cfg(loom)"] }` — the statement that `loom` is a configuration this crate knows.

This home writes imports; it neither owns the dependency nor hides it, which is the declared-input doctrine applied to the manifest itself.

The expansion is direct declaration-site items — nothing here is inert, nothing rides a carrier, and no consumption target is involved.
What the chosen names are used *for* is the production crate's own business.
