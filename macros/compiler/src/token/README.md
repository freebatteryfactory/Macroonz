# token — the seam on both sides

What a producer hands the compiler, and what a renderer hands back.

## Why it is typed

`proc_macro` is a proc-macro-crate-only API.
A crate that is not compiled as a proc macro cannot name its types at all, so the compiler — which is ordinary callable Rust and must stay so — can neither take a `TokenStream` nor hand one back.

The answer is not to fall back to strings.
A string is a token stream with its structure thrown away, and everything the capture then has to do is re-derive structure the compiler already had.

So the seam is typed on both sides.

## Reading

[`CapturedTokenTree`] is one token of a declared input: a payload, a stable [`TokenPath`] naming exactly where it sits in the tree, and an opaque [`SpanHandle`] indexing the producer's own table.
Delimited groups stay groups; nothing is re-lexed and no balance is re-discovered.

A payload carries a literal's **value** and never its spelling.
`"x"` and `r"x"` are one text, `"a\nb"` is three characters, and which prefix a producer read is not a fact the tree keeps.
[`capture_literal`] is where a lexed spelling becomes that value, and it refuses a form it has no row for rather than filing it under a neighbouring one.

Every producer walks under the same five magnitudes — depth, level, whole tree, capture work, and the width of a generated level — declared as plain constants beside the capacities they govern.
A [`SpanHandle`] means "the token at this index of the table the producer built while capturing".
The compiler never resolves one; it carries the handle into a diagnostic so that whoever produced the input can map it back to the exact compiler span, which is what puts a `compile_error!` on the offending token rather than on the first token of the declaration.

[`TextCapture::read`] is the third producer.
A compiler is one, a test is another, and text is the third — it exists so that the reproduction route a diagnostic names is a real road and not a promise.

## Writing

[`GeneratedTree`] is what a renderer produces.
A renderer states a literal's value and never its spelling here too: the quoting, the escaping, and the absence of a suffix belong to the tree.
That is what keeps `b"…"` from being assembled out of a word and a quoted string — two tokens where the address reading it matches one — and what lets one count be written into a `u32` seat, a `u64` seat, and a `usize` seat, because an unsuffixed literal is typed by the position it lands in.

`compose.rs` is the rest of what a renderer needs: paths, calls, method chains, bindings, constants, functions, attributes, rosters.
A renderer states what it means and never assembles punctuation by hand.

The written roster grows only at its end.
Each arm's slot lives in `encode.rs`, a slot is a byte of the tree's canonical bytes, and those bytes are what a rendered unit's identity is derived over.

## What it is not

The Rust source text a person reads is [`GeneratedTree::inspected`] — a projection of the tree, produced for a person, never the artifact.
Nothing parses it back and no identity is derived from it.

Nothing here knows what a declaration means.
The seam carries tokens with their structure and their spans intact; the grammar written in them is the caller's.
