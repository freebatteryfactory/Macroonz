# Recipe postures

This example changes caller-declared structural requirements and observes admission or refusal through the root facade's callable compiler.
The [recipe clause forms](../../macros/compiler/src/recipe/README.md#clause-forms) and [relation vocabulary](../../macros/compiler/src/relation/types.rs) own the syntax and meanings.

Run from the repository root:

```sh
cargo run --example recipe_postures --no-default-features
```

Successful execution is silent.
The executable admits both answer forms of every posture, checks each endpoint of membership and completeness independently, preserves authored order and repeated rows when permitted, and requires the contradictory declarations to refuse.
It informs and emits recipes; it does not compile those emitted programs or establish a domain property.
The [first recipe](../recipe.rs) executes generated lookups, codec methods and dispatch with independent expected values.

`membership(open, open)` still refuses a row naming an undeclared member.
The declared vocabulary bounds the enumerated rows under either membership choice.
`absence(allowed)` in this example is a stored relation requirement; no dispatcher is requested.
The standard transition dispatcher requires `absence(refused)` and returns its typed absent case.

To repair a refused declaration, correct the rows or the explicitly intended posture: supply the missing endpoint coverage or pairs, remove unwanted repetition, self edges or cycles, or explicitly permit them when that is the caller's intended contract.
An unknown endpoint must be corrected or added to the authored vocabulary, and a posture question must have one answer.
Changing a declared requirement is a caller decision, not an automatic repair by the compiler.

The example's executable control can also be selected directly:

```sh
cargo nextest run -p macroonz --example recipe_postures --no-default-features --no-tests fail
```
