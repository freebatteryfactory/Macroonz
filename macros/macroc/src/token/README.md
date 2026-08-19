# token — the typed token seam

What the services read, and what they write.

## The services' own token vocabulary

`proc_macro` is a proc-macro-crate-only API. A crate that is not compiled as a
proc-macro cannot name its types at all, so the services — which are ordinary
callable Rust and must stay so — cannot take a `TokenStream` and cannot hand one
back. The answer is not to fall back to strings: a string is a token stream with
its structure thrown away, and everything the capture then has to do is re-derive
structure that the compiler already had.

So the seam is typed on both sides.

**Reading.** [`CapturedTokenTree`] is what one token of a declared input is: a
payload, a **stable [`TokenPath`]** naming exactly where it sits in the tree, and
an opaque [`SpanHandle`] indexing the producer's own span table. Delimited groups
stay groups; nothing is re-lexed and no balance is re-discovered.

**Every producer walks under the same declared magnitudes.** Depth, level, and
whole-tree token count are written down once in the compiler plane's limits
roster; the capture-work budget is written down on [`CaptureWalk`], the walk that
spends it. All four are spent by every producer — the compiler shell and the text
reader alike — so "how big may a declared input be" has one answer rather than
one per road.

Each magnitude bounds the thing it is about, and only that thing. The level
bounds how wide one nesting level may be, the whole-tree count bounds how many
tokens the declaration carries in total, and a producer's span table — one entry
per handle it issued, across every level at once — stands under the whole-tree
count, because a table is not a level.

**Writing.** [`GeneratedTree`] is what a renderer produces. The human Rust text
is [`GeneratedTree::inspected`] — a projection of the tree, produced for a person
to read, never the artifact itself. The artifact is the tree.

## The opaque span handle

A [`SpanHandle`] means "the token at this index of the table the producer built
while capturing". The services never resolve one: they carry it into a diagnostic
so that whoever produced the input can map it back to the exact compiler span.
That is what puts a `compile_error!` on the offending token rather than on the
first token of the declaration. Author token identity survives capture →
plan → render → closure → diagnostics and origin inspection — the
span-fidelity law, and every post-capture diagnostic is measured against
it. The span facts that are stable on the pin: line, column, and the
display-oriented file; the file path may be remapped, so it lives only on
the location rail, and deeper span surfaces stay untouched.

## The seats

`types.rs` declares; its own child `type_guard.rs` holds every road that reaches
a private field, which is where all four magnitudes are settled. `text.rs` is the
callable text route end to end, `resolve.rs` composes every coordinate the seam
hands out — a span handle's position and a refused read's byte — `encode.rs`
writes the canonical bytes, and `inspect.rs` renders what a person is shown.
