# corpus

A seed pack is a search that already paid for itself.

Generation eventually reaches an input worth keeping: a shrunk counterexample, an awkward case someone wrote by hand, a byte string that cost a thousand tries to find.
Write those bytes into a pack and the next run starts where the last one finished.

This home owns the pack format and the two roads across it — writing one, and reading one that arrived from somewhere else.

## What a pack holds

One declared population, and its seeds, in the order they were authored.

Order is kept because it is exploration order, and exploration order is part of what the pack claims.
The pack's address is derived over its whole encoded body and written ahead of that body, so a reader re-derives the claim before it believes a single member of it.

## The envelope

Read `u32be(n)` as an integer in four big-endian bytes, `u64be(n)` as one in eight, and `bytes(x)` as `u64be(x.len())` followed by `x`.

The addressed body is exactly this, with no separators and no padding:

```text
u32be(SEED_PACK_FORMAT_VERSION)
bytes(population namespace)
bytes(population stem)
u64be(seed count)
bytes(seed)                       repeated, in authored order
```

The address is that body under `SEED_PACK_TAG`, and the complete envelope is the address followed by the body it addresses.
An address never covers itself.

## What this home refuses to grow

It stores nothing.
Paths, files, locking, eviction, and retention are the caller's, and no fact about where a pack lived may enter what the pack is.

It judges nothing.
A pack warms a search up; the verdict still comes from the ordinary report and fingerprint roads, and no seed is judged against the corpus that handed it over.
A pack address is not a replay capsule, and it is not evidence.
