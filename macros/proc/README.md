# macroonz-macros

The hand that loads the oven.

A proc-macro crate may export nothing but macros, so this one is small on purpose: token conversion, span custody, one call into the compiler, diagnostic placement, emission.
It owns no grammar, no roster, no identity rule, no planning decision, and no judgment.

---

## Three attributes and one declaration

| Entry | On | Renders |
| --- | --- | --- |
| `#[trials(...)]` | a type or a module | A trial table for `macroonz-harness`: suites and rows, each row a claim, subject, check, and population you name. |
| `#[bench(...)]` | a type or a module | A bench table and its one-file reporter adapter, bound to a measurement backend you name. |
| `#[mutations(...)]` | an enum | A mutation surface pressing the enum's declared order: the policy you state, and one adjacent-transposition alternative per neighboring pair of variants. |
| `shadow! { ... }` | item position | Both faces of every chosen synchronization name from the compiler's stated roster: the ordinary face behind `#[cfg(not(loom))]`, the shadow face behind `#[cfg(loom)]` — written once, where the declaration stands. |
| `network! { ... }` | item position | The builder module for a declared topology and its fault schedules: `topology()`, one function per schedule, and one generated fault enum their refusals travel in. |
| `concurrency! { ... }` | item position | One generic function per declared exploration row, taking the strand set and the transition contract and handing back the exploration reading beside its concluded trial verdict. |

Each attribute expands to exactly two things: one exported carrier, and the item you wrote, untouched.
The three declarations are direct emissions — their items are ordinary Rust where they stand, inert inside nothing, because a face and a builder are not cargo.

The carrier is a hidden `macro_rules!` definition — plus the alias you chose in the `support` clause — holding its cargo inert.
An ordinary build compiles the definition and nothing inside it.
Your test or bench target invokes the alias, supplies its own host facts and callables there, and the carrier's gate checks the schema pin before a single constructor reaches type checking.

Each attribute walks the road any derive built on `macroonz` walks — capture, request, render, close, explain, bind, emit.
The grammar each one reads is the compiler's `descriptor` home's; the road from a reading to a sealed carrier expansion is the same home's `door`; the carrier itself is the compiler's `support` home's.
What lives here is one function per attribute, and every sentence a refusal shows you was composed inside the compiler at the token it is about.

---

## Writing your own derive

Do not write it here.

A derive that produces your types lives in your repository, in your proc-macro crate, on `macroonz` with the `host` feature:

```rust
#[proc_macro_derive(Greet, attributes(greet))]
pub fn greet(input: TokenStream) -> TokenStream {
    macroonz::host::expand(input, |capture| {
        let greeting = Greeting::read(&capture)?;
        Request::<GreetImpl>::over(capture, greeting, &GREET_DOOR)
            .render(|plan, out| out.unit(GreetRole::Impl, plan.content().impl_tokens()))
    })
}
```

That is the whole crate a derive needs.
`serde_derive` ships with `serde`; your derive ships with you.
A derive that also wants to deliver descriptor cargo composes its own carrier on the same public roads these attributes walk — one vehicle may carry a trial table beside a mutation module, which is a composition the standalone attributes deliberately keep apart.
