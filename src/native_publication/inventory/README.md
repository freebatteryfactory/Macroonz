# Publication inventory

This home pairs one sealed expansion's complete published-unit roster with caller-declared destination paths.
Every logical publication address has exactly one binding, and no binding may name an absent unit.
Bindings are retained in the compiler's proved unit order rather than the caller's mapping order.
An expansion with no publication units admits only an empty binding set.

Adding a stamp compares its planned unit, digest contract and canonical definition bytes with the actual sealed unit.
Every named landing receives exactly one explicit path, in the stamp's declared order.
The inventory cannot accept a second stamp for that unit, a missing or extra landing, or a definition rendered under another record.
It retains the original expansion and stamp values; file views borrow their token trees and parent rendered units.

## Paths and bounds

`PublicationPath` composes the oracle's existing relative-path admission with a portable publication filename grammar.
Segments contain ASCII letters, digits, underscore, hyphen or dot, are at most 240 bytes, and do not end with a dot.
The complete spelling is at most 4,096 bytes.
Windows device stems and segments beginning with `.macroonz-` refuse; the latter reserve the publication control namespace.
No spelling is normalized.
The complete file set refuses ASCII-case aliases and file/directory prefix collisions on every host, so moving the same inventory between case-sensitive and case-insensitive filesystems does not create an admitted alias.
Actual filesystem containment and availability are separate native-operation questions.

`PublicationLimits` bounds the complete file count and aggregate unformatted source projection, including one terminal LF per file.
Adding landings rechecks the complete set rather than giving each stamp a separate allowance.
An informed inventory has no duplicate destination and no partial stamp.

## Canonical material and publication bytes

The compiler's rendered unit retains its original canonical digest and identity.
A file's `canonical_digest` separately commits to its exact borrowed tree's canonical bytes, including for a landing that is not a separate planned unit.
`published_digest` commits to the supplied physical bytes, which may have been formatted.
Neither operation parses formatted text back into the compiler's account.

Both use the existing compiler identity owner, its transcript framing and BLAKE3 derivation.
The profile is `macroonz/native-publication/file-bytes` at version one, role `Bundle`, rooted with position zero, and the entire byte slice is its material.
The distinct subjects `canonical-token-bytes` and `published-bytes` use stem `macroonz/native-publication`.
Paths, host facts and formatting-tool versions do not enter these byte commitments.
They identify bytes, not semantic equivalence, compilation, ownership of an existing file or an installation receipt.
