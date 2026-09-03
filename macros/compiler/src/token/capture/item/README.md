# item

This home is the shallow structural lens over one supported complete caller-authored Rust item.

The declarations hold the recognized item families, the borrowed lens, and the refusal a lens read establishes with the exact producer span available at the site.
The lens reads only the outer envelope: attributes, visibility, qualifiers, item family, optional name, generic run, where-clause run, signature run, body group, and an explicit item-level `unsafe` token where one is written.
Every fragment the lens hands out borrows the same captured material as the preserved reading, so nothing is copied and nothing is reparsed.
The contracts render every refusal as one sentence about a declared item boundary.

The lens is not a Rust AST.
It never scans for hidden unsafe behavior and leaves syntax, type, lifetime, ownership, and coherence judgments to Rustc.
