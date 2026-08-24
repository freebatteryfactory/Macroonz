# macroonz-macros

The hand that loads the oven.

A proc-macro crate may export nothing but macros, so this one is small on purpose: token conversion, span custody, one call into the compiler, diagnostic placement, emission.
It owns no grammar, no roster, no identity rule, no planning decision, and no judgment.

---

## The question

What does a proc host export, when every grammar it would read and every road that turns a reading into tokens belong to the compiler?

Three attributes are the answer it is written for.

| Attribute | On | Renders |
| --- | --- | --- |
| `#[trials(...)]` | a type or a module | A trial table for `macroonz-harness`: suites and rows, each row a claim, subject, check, and population you name. |
| `#[bench(...)]` | a type or a module | A bench table and its one-file reporter adapter, bound to a measurement backend. |
| `#[mutations(...)]` | a type or a module | A mutation surface: the points, owner-claim mappings, and operator permissions the harness lowers into executable pressure. |

Each walks the road any derive built on `macroonz` walks — capture, request, render, close, explain, bind, emit — and reads its body through `macroonz::descriptor`.

## The condition that fills it

What all three render is inert until a consumption target invokes it, so what a declaration site receives is the exported carrier and never the table inside it.

A carrier reaches a declaration site the way every other token does: as a rendered unit of a seat whose destination is the declaration site, joined and digested by the proof that closed it.
`macroonz::closure::CarriedTokens` has no public constructor, so there is no second road — a tree assembled here would be bytes no proof committed to, and `macroonz::host::emit` will not take one.
Every seat the compiler's three descriptor kinds declare delivers to a carrier, so none of them can hand this crate anything to emit.

That seat — a carrier kind whose plan `macroonz::support::SupportShell::assembled` already accepts, rendering the exported `macro_rules!` at the declaration site — is what fills this crate.
Two of the three grammars wait on one further thing each: a bench declaration has no reader in `macroonz::descriptor::bench`, and a mutation declaration is completed by the door that captured what the helper sits on.

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
