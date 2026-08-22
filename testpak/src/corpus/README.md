# corpus — content-addressed seed packs for generation warm starts

A seed pack is dependency-free, length-prefixed binary exploration state for one declared population. The writer preserves authored seed order, derives the pack address over the complete encoded body, and places that claim ahead of the body; the reader re-derives it before interpreting the body, refuses a population other than the caller expected, and refuses exact duplicate seeds rather than silently narrowing the search roster.

## Canonical envelope

The primitives are `u32be(n)` and `u64be(n)`, the integer in four or eight big-endian bytes, and `bytes(x)`, which is `u64be(x.len()) || x`. The addressed body is exactly `u32be(SEED_PACK_FORMAT_VERSION) || bytes(population namespace UTF-8) || bytes(population stem UTF-8) || u64be(seed count) || bytes(seed)` repeated in authored seed order; there are no other separators or padding bytes. `SeedPackAddress` is `ContentAddress::derived(SEED_PACK_TAG, body)`, and the complete envelope is `address.as_bytes() || body`, with the leading address excluded from its own preimage.

The public home owns bytes-to-pack and pack-to-supplied-input roads only. Storage paths, persistence, locking, eviction, and retention belong to the caller, and no filesystem fact enters the pack's content identity.

Packs warm-start generation; they do not judge it. A find that matters reaches the ordinary `TestPak` report and fingerprint roads, and may reach a proposal only through a separately lawful proposal ground. A pack address is neither a replay capsule nor evidence, and no seed is judged against the corpus that supplied it.
