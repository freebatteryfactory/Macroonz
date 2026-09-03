# Token capture

This home owns the normalized declaration the compiler receives, the bounded builders that make it, literal capture, text lexing, source coordinates, and canonical captured identity bytes.

Compiler-host tokens and source text terminate at the same [`CapturedInput`] boundary.
Producer spans, spelling-only literal choices, whitespace, and ordinary comments do not cross that boundary.
Raw identifiers, punctuation adjacency, doc-comment meaning, lifetime spelling, and invisible compiler groups do cross because removing any of them would merge distinct lawful Rust token streams.

[`CaptureCursor`] provides the bounded mechanical reading layer over that normalized tree.
It can require exact words, ordinary or raw identifiers, numbers, punctuation adjacency, groups, arrows, complete consumption, and trailing-separated rows while retaining the exact available producer span for a refusal.
A subject composes those operations into its own grammar and still owns every noun, precedence rule, clause meaning, empty posture, and diagnostic policy.

[`CapturedFragment`] borrows one exact run from a complete capture or nested group and retains its captured token structure, canonical bytes, and available source-span boundaries.
[`CaptureCursor::fragment`] couples a caller-owned structural read to the exact run it consumed, including an intentionally empty exact seat.

[`AuthoredItem`] recognizes only the outer envelope of one supported complete caller-authored Rust item: attributes, visibility, qualifiers, item family, optional name, generic run, where-clause run, signature run, body group, and an explicit item-level `unsafe` token where written.
It preserves one token reading rather than reconstructing a public Rust AST, never scans for hidden unsafe behavior, and leaves complete Rust legality to Rustc.

This home does not emit generated Rust and does not interpret a captured declaration's product meaning.

## Seats

The root of this home is the normalized declaration itself: the magnitudes, the token tree with its path and span handle, the checked builder, literal capture, canonical bytes, borrowed fragments, and the span table with every coordinate it answers in.
`cursor/` is the mechanical reading layer over that tree.
`item/` is the structural lens over one complete caller-authored item.
`text/` is the text producer that reads source into the same tree.
Each sub-home reaches the root through its public roads, and none reaches another.
