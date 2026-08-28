# macroonz-macros

The hand that loads the oven.

A proc-macro crate may export nothing but macros, so this one is small on purpose: token conversion, span custody, one call into the compiler, diagnostic placement, emission.
It owns no grammar, no roster, no identity rule, no planning decision, and no judgment.

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

Each attribute walks the road any derive built on `macroonz-compiler` walks — capture, request, render, close, explain, bind, emit.
The grammar each one reads is the compiler's `descriptor` home's; the road from a reading to a sealed carrier expansion is the same home's `door`; the carrier itself is the compiler's `support` home's.
What lives here is one thin function per procedural entry, and every sentence a refusal shows you was composed inside the compiler at the token it is about.

---

## Writing your own derive

Do not write it here.

A derive that produces your types lives in your repository, in your proc-macro crate, on `macroonz-compiler` with the `host` feature:

```rust
#[proc_macro_derive(Greet, attributes(greet))]
pub fn greet(input: TokenStream) -> TokenStream {
    macroonz_compiler::host::expand(input, |capture| {
        let greeting = Greeting::read(&capture)?;
        Request::<GreetImpl>::over(capture, greeting, &GREET_DOOR)
            .render(|plan, out| out.unit(GreetRole::Impl, plan.content().impl_tokens()))
    })
}
```

That is the whole crate a derive needs.
`serde_derive` ships with `serde`; your derive ships with you.
A derive that also wants to deliver descriptor cargo composes its own carrier on the same public roads these attributes walk — one vehicle may carry a trial table beside a mutation module, which is a composition the standalone attributes deliberately keep apart.
