# Declared codecs

This example writes and reads a caller-owned record through the root recipe entrance, checking independently authored bytes and typed refusals.
It needs the ordinary library posture:

```sh
cargo run --example codec_workflow --no-default-features --locked
```

Successful execution prints:

```text
codec: exact bytes and round trip agree; malformed bytes and zero count refuse
```

The record uses count, bytes, text, closed-choice and nested members, with required, optional and repeated cardinalities.
The nested record requests `round_trip` with total assembly.
The outer record deliberately declares separate `encode` and `decode` roads; ordinary two-way use can replace those matching declarations with one `round_trip` declaration.
The decoder calls the author's checked assembly, which refuses a zero count.
The [recipe grammar](../../macros/compiler/src/recipe/README.md#clause-forms) and [codec owner](../../macros/compiler/src/codec/README.md) own the declaration and member-method contracts.

The checks require exact bytes for a populated record and round-trip behavior for absent optional and empty repeated members.
They separately challenge invalid presence, UTF-8, a foreign choice slot, an overlong frame, invalid nested bytes, an oversized count, trailing bytes, truncation and the caller's assembly refusal.
The expected byte vector is independent of the generated writer.

Decode errors identify the field that needs repair; `NotAssembled(ZeroCount)` comes from the caller's rule rather than a rule invented by Macroonz.
The writer encodes supplied fields without calling the assembly function.
Applications that require only admitted in-memory records should keep their fields private and enforce that rule in their own constructor.
Neither canonical framing nor a successful round trip establishes that a caller's wire choices or domain rules are correct.
