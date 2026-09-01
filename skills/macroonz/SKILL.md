---
name: macroonz
description: Author or extend Macroonz 0.2 Rust recipes, choose built-in versus caller-owned projections, and keep generated behavior separate from independent harness judgment.
---

# Macroonz recipes

Use this skill when working from the packaged `macroonz` facade.
Read the [facade README](../../README.md) for product posture and run the complete [first recipe](../../examples/recipe.rs) before inventing syntax.

## Write one recipe

Start with ordinary Rust inside `macroonz::recipe!`.
Keep `bake!` last in the inline module.

```rust
macroonz::recipe! {
    pub mod door {
        pub enum State { Closed, Open }
        pub enum Event { OpenDoor }

        bake! {
            vocabularies(State, Event);
            transitions {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
            };
        }
    }
}
```

The caller owns the Rust items, relations, effect paths, postures, and requested projections.
Use an ordinary Rust path unless Macroonz must enumerate the referenced members.
Read generated declaration-site companions under the recipe module's `baked` child; do not assume a crate-root reexport.

## Add precision without changing modes

- Use `dispatch;` for the documented conventional name.
- Use `dispatch(apply);` for one flat configured name.
- Use braces for exact Rust material:

```rust
dispatch {
    /// Applies one caller-declared transition or returns typed absence.
    pub fn advance(
        current: State,
        event: Event,
    ) -> Result<State, TransitionRefusal>;
};
```

The exact dispatch seat accepts one semicolon-terminated function signature with two simple caller-named parameter bindings.
Macroonz supplies only the transition-row-accounted body.
Use a caller-owned projector when the algorithm or body itself is custom.
Preserve explicit caller-authored unsafe syntax exactly; never ask Macroonz to infer, widen, or justify it.

## Route custom algorithms

Implement `macroonz::compiler::recipe::RecipeProjector` and call `recipe::bake_with` from ordinary compiler code or a caller-owned proc-macro host.
Consume only `RecipeView`, `ProjectionRequest`, and the one-use `ProjectionSink`; offer one `GeneratedTree` for the already selected role.
Use the packaged `macroonz-compiler` example `custom_recipe_projector.rs` as the complete reference.
Do not try to name downstream executable projector code inside `macroonz::recipe!`; Macroonz's already compiled proc carrier cannot execute it.

## Judge independently

Generated Rust is not its own oracle.
Exercise it from an external test and compare it with a handwritten model, a direct `macroonz::harness` property or oracle, or an explicitly invoked generated evidence carrier.
The packaged `macroonz-harness` example `temporal_property.rs` is the direct judgment reference.
An uninvoked carrier is inert material, not evidence.

## Respect package posture

- Default `macroonz` includes the harness.
- `--no-default-features --features harness` keeps harness judgment without Loom preemption.
- `--no-default-features` keeps the recipe/compiler/proc road and makes harness-owned bakes typed unavailable.

Treat a Macroonz diagnostic as the owning repair instruction.
Let rustc judge paths, visibility, types, borrowing, lifetimes, coherence, exhaustiveness, and unsafe obligations.
