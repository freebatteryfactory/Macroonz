# Enum order through deferred carriers

This caller declares enum order through the direct mutation attribute and through a recipe, then invokes both deferred carriers through the root facade.
The permitted priority order yields two adjacent transpositions, while the recipe's discovered pipeline order remains visible with no executable selections because its claim has no permission.

Copy this directory's Rust files into a new caller-owned directory.
Copy [consumer.toml](consumer.toml) there as `Cargo.toml` and replace `/absolute/path/to/Macroonz` with the exact source checkout being evaluated, using forward slashes on Windows.
Run from that directory:

```text
cargo +1.98.1 generate-lockfile
cargo +1.98.1 run --bin enum-mutation-workflow --locked
```

The program checks every generated alternative against independently stated order values and prints:

```text
enum: unchanged order agrees; two alternatives disagree; unpermitted point stays discoverable
```

[types.rs](types.rs) owns the declarations and permissions, [observation.rs](observation.rs) owns independent expectations, and [consumer.rs](consumer.rs) invokes the carriers exported by the library's [declaration.rs](declaration.rs).
Both the attribute and the recipe mutation block choose their support addresses explicitly.
The library and consuming binary are separate Rust crates within the adopter's package because these exported carriers must be consumed across that boundary.
The template adds no Macroonz workspace package and observes the selected checkout rather than registry or release provenance.
The [mutation owner](../../macros/compiler/src/descriptor/mutation/README.md#authored-grammar) owns the declaration clauses, and the [support owner](../../macros/compiler/src/support/README.md#invocation) owns carrier invocation.

The selected alternatives are order values supplied by generated evaluation functions.
Their firings count those selections; they do not rewrite the enum's representation or establish execution of a mutated application, semantic equivalence, a behavioral kill or survival.
Use the [structural compiler example](../structural_mutation/README.md) for actual generated-program compilation and the [assessment example](../mutation_assessment/README.md) for separately qualified observations and witness judgment.

If no selections appear, inspect the discovery disposition and the caller's fact-to-claim mapping and permission for that claim and operator family.
A permission for an unrelated claim does not authorize this point.
If the attribute is placed on an item with no enum order, compilation refuses at that item; apply it to an enum or use the callable structural producer appropriate to the declared subject.
