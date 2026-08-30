# token — the public seam on both sides

What a producer hands the compiler, and what a renderer hands back.

The private `capture/` and `generation/` homes own the two directions while this module keeps the established public `token` paths unchanged.

## Why it is typed

`proc_macro` is a proc-macro-crate-only API.
A crate that is not compiled as a proc macro cannot name its types at all, so the compiler — which is ordinary callable Rust and must stay so — can neither take a `TokenStream` nor hand one back.

The answer is not to fall back to strings.
A string is a token stream with its structure thrown away, and everything the capture then has to do is re-derive structure the compiler already had.

So the seam is typed on both sides.

## Reading

[`CapturedTokenTree`] is one token of a declared input: a payload, a stable [`TokenPath`] naming exactly where it sits in the tree, and an opaque [`SpanHandle`] indexing the producer's own table.
Delimited groups stay groups, including a compiler's invisible group.
Compiler-host input is never re-lexed.

[`CaptureBuilder`] is the only mint of a complete captured input.
A producer supplies its own source position and a [`CapturedAtom`], or opens a nested group; the builder issues the path and handle, spends every declared magnitude, retains positions in handle order, and derives the final denominator from that roster.
No producer can state the resulting capture facts as sibling arguments.
A capture operation consumes its open [`CaptureLevel`], and only a successful operation returns the level, so a refused partial tree has no road to [`CaptureLevel::finish`].
The refused attempt's positions remain available for its diagnostic; opening a fresh level rolls back that attempt while preserving handles issued by earlier successful captures in the same table.
Where a producer refuses one token after issuance, the refusal retains both the declaration-local path and the producer-local handle.

A payload carries a literal's **value** and never its spelling.
`"x"` and `r"x"` are one text, `"a\nb"` is three characters, and which prefix a producer read is not a fact the tree keeps.
[`capture_literal`] is where a lexed spelling becomes that value, and it refuses a form it has no row for rather than filing it under a neighbouring one.

Token distinctions that change a proc macro's input remain distinct: ordinary and raw identifiers have different rows, punctuation retains whether it joins what follows, lifetimes remain quote-plus-identifier tokens, and invisible compiler groups remain groups.
Whitespace and ordinary comments do not enter the normalized declaration.
Doc comments enter as the `doc` attributes the compiler presents to a proc macro.

Every producer walks under the same captured-input magnitudes — depth, level, whole tree, and capture work — declared as plain constants beside the capacities they govern.
A producer that skips or backtracks charges that observation through [`CaptureLevel::examined`], so work discarded before capture does not disappear from the budget.
A [`SpanHandle`] means "the token at this index of the table the producer built while capturing".
The compiler never resolves one; it carries the handle into a diagnostic so that whoever produced the input can map it back to the exact compiler span, which is what puts a `compile_error!` on the offending token rather than on the first token of the declaration.

[`TextCapture::read`] is the third producer.
A compiler is one, a test is another, and text is the third — it exists so that the reproduction route a diagnostic names is a real road and not a promise.
It uses a pinned low-level compiler lexer for token boundaries and slices every spelling from the original source by the lexer-reported byte range.
Its source-byte magnitude is checked before lexing and stands independently of the capture tree and work magnitudes.
The [`CapturedInput`] it returns is the shared normalization boundary against which the compiler-token producer is observed.

## Writing

[`GeneratedTree`] is what a renderer produces.
A renderer states a literal's value and never its spelling here too: the quoting, the escaping, and the absence of a suffix belong to the tree.
That is what keeps `b"…"` from being assembled out of a word and a quoted string — two tokens where the address reading it matches one — and what lets one count be written into a `u32` seat, a `u64` seat, and a `usize` seat, because an unsuffixed literal is typed by the position it lands in.

The private generation home's composers are the rest of what a renderer needs: paths, calls, method chains, bindings, constants, functions, attributes, rosters.
A renderer states what it means and never assembles punctuation by hand.
The keyed slice projectors walk an informed [`KeyedRoster`](crate::KeyedRoster) or [`KeyedRosterAssignment`](crate::KeyedRosterAssignment) in its structural order while the renderer supplies every row's tokens and the ordinary Rust item surrounding the slice.

The written roster grows only at its end.
Each arm's stable slot is one byte of the tree's canonical encoding, and those bytes are what a rendered unit's identity is derived over.
An ordinary identifier and a raw identifier occupy distinct rows, while every pre-existing row keeps its occupied slot.

## What it is not

The Rust source text a person reads is [`GeneratedTree::inspected`] — a projection of the tree, produced for a person, never the artifact.
Nothing parses it back and no identity is derived from it.

Nothing here knows what a declaration means.
The seam carries tokens with their structure and their spans intact; the grammar written in them is the caller's.
