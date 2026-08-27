# Token capture

This home owns the normalized declaration the compiler receives, the bounded builders that make it, literal capture, text lexing, source coordinates, and canonical captured identity bytes.

Compiler-host tokens and source text terminate at the same [`CapturedInput`] boundary.
Producer spans, spelling-only literal choices, whitespace, and ordinary comments do not cross that boundary.
Raw identifiers, punctuation adjacency, doc-comment meaning, lifetime spelling, and invisible compiler groups do cross because removing any of them would merge distinct lawful Rust token streams.

This home does not emit generated Rust and does not interpret a captured declaration's product meaning.
