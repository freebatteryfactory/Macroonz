# Token generation

This home owns the bounded token tree a renderer writes, its pure composers, its canonical identity bytes, and its human-readable source projection.

Ordinary and raw identifiers are distinct values, and punctuation carries the adjacency the compiler host must emit.
The public paths remain rooted at `token`; this child is an implementation home, not a second public namespace.

[`keyed_roster_slice`](crate::token::keyed_roster_slice) and [`keyed_assignment_slice`](crate::token::keyed_assignment_slice) project informed caller-keyed values into conventional borrowed Rust slice expressions.
They preserve retained or denominator order and delegate every row's tokens to the caller, so the surrounding name, visibility, type, destination, and meaning remain outside this home.

The namespace and data composers project conventional Rust item framing without inventing another item model.
They cover inline modules, imports and explicit reexports, aliases, unit, tuple and named structs, enums and variants, exact generic parameters, exact where predicates, attributes, documentation and visibility prefixes.
Newtypes, markers, phantom carriers and typestate data are compositions of those ordinary item forms rather than separate compiler concepts.
The caller retains every exact Rust fragment and semantic choice, while these operations own only fixed punctuation, delimiter placement and selected order.

The behavior composers project exact function signatures and bodies, conventional consuming, shared, exclusive and pinned receiver spellings, typed parameters, and match arms or expressions.
Free functions, associated functions, constructors, synchronous or asynchronous functions, typed refusals and conversions are compositions over those operations rather than separate compiler theories.
An explicitly supplied custom receiver or qualifier remains an exact caller-owned token run, and a safe preset never infers an unsafe boundary.

[`CapturedFragment::generated`](crate::token::CapturedFragment::generated) projects an exact captured run into this vocabulary without inspecting or reparsing source text.
Ordinary and raw identifiers, punctuation adjacency, all captured literal forms, written groups, and invisible compiler groups remain distinct.
[`GeneratedLiteral`](crate::token::GeneratedLiteral) guards the exact literal forms whose older semantic constructors deliberately canonicalize differently.

This home does not capture input, retain spans, or decide what a generated declaration means.
