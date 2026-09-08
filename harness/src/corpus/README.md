# corpus

A seed pack carries inputs worth keeping from one search into a later one.

Those inputs may be minimized counterexamples, awkward authored cases, or byte strings that took a long search to find.
The pack preserves them without turning their storage location or discovery history into product authority.

This home answers one question: whether an ordered seed roster belongs to one declared population and can cross a content-addressed envelope intact.

## Authority crossing

The writer receives an already-declared population and informed nonempty seeds.
It preserves authored order, because changing exploration order changes what the pack claims.

The reader receives the expected population and independent envelope, framed-member and seed-count ceilings separately from the untrusted envelope.
It refuses the envelope ceiling before hashing and settles the content claim before interpreting any member.
Framed names and seed bytes are bounded before copying, and the seed count is bounded before walking or allocating its roster.
Population drift, malformed framing, empty members, duplicates, and undeclared trailing material refuse.

An admitted pack leaves with the exact envelope it crossed and the ordered seeds it informed.
Its warm-start projection hands those bytes to generation as exact supplied input; generation still owns budgets, decoding, preconditions, and case accounting.

## Trust ceiling

This home owns the canonical pack format and its content address.
The exact byte grammar belongs beside the writer and reader that implement it, where outside vectors can hold it to account.

Storage, paths, locking, eviction, and retention remain caller-owned, and none enters pack identity.
The pack does not judge a seed, mint replay authority, or become evidence merely because its bytes are content-addressed.
Verdicts and reproductions remain separately earned through their ordinary owners.
Reader limits bound admitted material, not the caller's storage reads, the already-informed writer inputs, allocator overhead or later generation work.
