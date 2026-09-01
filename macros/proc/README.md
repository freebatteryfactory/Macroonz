# macroonz-macros

The hand that loads the oven.

A proc-macro crate may export nothing but macros, so this one is small on purpose: token conversion, span custody, one call into the compiler, diagnostic placement, emission.
It owns no grammar, no roster, no identity rule, no planning decision, and no judgment.

The root `macroonz::recipe!` entrance reaches its compiler-owned recipe door through this carrier.
Built-in projectors and exact declarative configuration may run there because they ship with Macroonz, while an arbitrary downstream projection algorithm runs through `macroonz-compiler` in a caller-owned compiler or proc host.
Both hosts consume the same informed recipe and constrained output protocol; this crate gains no plugin registry or second compiler model.

Rust requires the function-like proc carrier behind the hygienic facade wrapper to be public.
Exactly one such carrier exists, it is hidden from generated documentation, and `macroonz::recipe!` is the only supported entrance.
Direct invocation of `__macroonz_recipe_carrier!` is outside the compatibility contract and may change or break in any release without notice.

---

## Procedural declaration families

The item-preserving attributes emit an inert carrier beside the decorated item.
The direct declarations emit ordinary Rust items at the declaration site.

| Entry | On | Renders |
| --- | --- | --- |
| `#[trials(...)]` | a type or a module | A trial table for `macroonz-harness`: suites and rows, each row a claim, subject, check, and population you name. |
| `#[bench(...)]` | a type or a module | A neutral bench table and one typed seat for the target's `fn(&BenchReport)` reader. |
| `#[mutations(...)]` | an enum | A mutation surface pressing the enum's declared order: the policy you state, and one adjacent-transposition alternative per neighboring pair of variants. |
| `shadow! { ... }` | item position | Both faces of every chosen synchronization name, rooted at the Loom path the declaration supplies. |
| `network! { ... }` | item position | The builder module for a declared topology and its fault schedules, rooted at the harness path the declaration supplies. |
| `concurrency! { ... }` | item position | One generic function per declared exploration row, rooted at the harness path the declaration supplies. |

Each attribute expands to one exported carrier, then re-emits the item token stream it received.
The direct declarations are ordinary Rust where they stand, inert inside nothing, because a face and a builder are not cargo.
Their `loom` or `harness` clause supplies the physical path that declaration scope resolves, so a renamed dependency and a facade re-export use the same grammar and the renderer hardcodes neither.

The carrier is a hidden `macro_rules!` definition — plus the alias you chose in the `support` clause — holding its cargo inert.
An ordinary build compiles the definition and nothing inside it.
Your test or bench target invokes the alias, supplies its own host facts and callables there, and the carrier's gate checks the schema pin before a single constructor reaches type checking.

A descriptor carrier composed through `macroonz::recipe!` also requires the invocation target to state the declaring crate path.
The wrapper's `$crate` belongs to the facade that defined the wrapper, so the explicit path is what lets the existing carrier reach its hidden helper without guessing package topology or adding ambient discovery.

Each attribute walks the road any derive built on `macroonz-compiler` walks — capture, request, render, close, explain, bind, emit.
The grammar each one reads is the compiler's `descriptor` home's; the road from a reading to a sealed carrier expansion is the same home's `door`; the carrier itself is the compiler's `support` home's.
What lives here is one thin function per procedural entry, and every sentence a refusal shows you was composed inside the compiler at the token it is about.

---

## Caller-owned projection algorithms

Do not add one here merely because a recipe needs a custom projection.

An arbitrary recipe projection algorithm is ordinary caller-owned compiler code or lives in the caller's proc-macro crate on `macroonz-compiler` with the `host` feature:

```rust
impl macroonz_compiler::recipe::RecipeProjector for MyProjector {
    fn project(
        &self,
        view: macroonz_compiler::recipe::RecipeView<'_>,
        request: macroonz_compiler::recipe::ProjectionRequest<'_>,
        sink: macroonz_compiler::recipe::ProjectionSink<'_, '_>,
    ) -> Result<macroonz_compiler::recipe::ProjectionOffered, macroonz_compiler::recipe::ProjectionError> {
        let tree = project_my_role(view.recipe(), request.effective())?;
        sink.offer(tree)
    }
}
```

`project_my_role` is caller-owned placeholder code returning one `GeneratedTree`.
The complete compiling example is shipped by `macroonz-compiler` as `examples/custom_recipe_projector.rs`.

That is the complete capability boundary a caller-owned recipe projector receives.
It consumes the same informed recipe contract as the paved recipe road and returns output through the same planned role.
The distinction is execution host rather than semantic model: an already compiled Macroonz proc macro cannot invoke arbitrary code defined later in a downstream crate.
