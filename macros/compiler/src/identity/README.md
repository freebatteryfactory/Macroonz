# identity

Thirty-two bytes that say which thing this is, and a transcript that says why.

An identity here is never a handle, a counter, or a name somebody picked.
It is a BLAKE3 derivation over a complete preimage — a transcript carrying the grammar it stands in, the subject it names, the seat it fills, what it hangs off, and the material that varies.
Two runs on one machine, or on two, derive the same bytes from the same transcript, and nothing derives them from anything else.

## Subjects are yours

`Subject` is an open trait with two constants: the name a subject is spelled by, and the stem of whoever owns it.

```ignore
macroonz::subjects! {
    stem = "my-crate/identity";
    /// One obligation a trial row challenges.
    Obligation = "obligation",
}
```

The compiler's own subjects are declared in this home under `MACROONZ_STEM`.
Yours are declared in your crate under your stem, and the two cannot collide: the stem opens the derive-key context, so `"obligation"` under your stem and `"obligation"` under anyone else's are unrelated key spaces rather than neighbouring names.

A projection kind is qualified by the producer namespace and name on its request door before the kind's own declared name is derived.
A projection-content commitment is then derived from that kind identity and the content's complete canonical bytes under the exact captured declaration, so changing any of those three facts moves the binding before a plan exists.

A roster is checked while it compiles — every name inside the grammar, and no name declared twice.
The grammar is lowercase ASCII letters and digits in `-`-joined segments, with no leading, trailing, or doubled separator.

## One version per grammar

A `Profile` is one preimage grammar: a stem, a name, and a `Version`.

A version is what renames.
It rides the derive-key context and the transcript, so moving it renames every identity derived under that grammar — and under no other, which is why each grammar carries its own.
One shared position would rename a plan the day a rendered tree grew a token arm, and the equivalence an intent comparison answers would stop answering it.

A position moves when the grammar's preimage moves **and somebody holds a name derived under the old one**.
Where no reader holds one, a widening is an edit to the position that stands.

## The ten members

A transcript is the exact byte string handed to the digest, and this is the whole of it.
`u32be(n)` and `u64be(n)` are the integer in four or eight big-endian bytes; `bytes(x)` is `u64be(x.len())` followed by the bytes of `x`, so no two member sequences can be cut at a different boundary and produce one string.

| # | member | encoding |
| - | ------ | -------- |
| 1 | profile stem | `bytes(utf8)` |
| 2 | profile name | `bytes(utf8)` |
| 3 | profile version | `u32be` |
| 4 | subject | `bytes(utf8)` of `Subject::NAME` |
| 5 | role | `bytes(utf8)` of `Role::name` |
| 6 | role slot | one byte |
| 7 | anchoring | one byte |
| 8 | anchor | `bytes(…)` — empty when rooted, else the full thirty-two |
| 9 | material | `bytes(…)` — the full material, never a fold |
| 10 | position | `u32be` |

The derive-key context is `<profile stem>/<profile name>/v<version>/<subject stem>/<subject name>/<role>`, and the identity is `blake3::derive_key(context, transcript)`.
The subject's stem is a segment of the context and is not restated as a member, so two subjects spelled alike under different stems derive under different keys rather than under one key with different bytes.

Nothing is folded on the way in: the anchor crosses at its full thirty-two bytes and the material at its full length, so the thirty-two-byte output is the only compression anywhere in the derivation.

## What this home will not do

It mints nothing for a consumer, and it reads nothing back.

`OwnerIdentity` is how a consumer cites thirty-two bytes it minted itself.
Nothing here checks them; holding one says the compiler refers to that identity and says nothing else.

`OwnerFact` is how anyone cites a fact by the home that declared it and the name that home wrote down.
Every selection, omission, and non-applicability in the compiler cites one, because a bare boolean says a decision happened without saying whose fact decided it.

`HumanProjection` carries bytes a caller may show a person.
Nothing here reads one back, matches on one, or decides from one.

## The generator is provenance

Which generator produced a value rides `Provenance`.
What the value **is** rides its own transcript, and no grammar names the generator, so a shape bump renames nothing and the same exact bytes stay the same artifact across the producers that emitted them.

Where a generator's shape genuinely changes what something is, the change reaches identity through the seat that states it — a plan whose role roster grew declares a different membership, and a membership is a plan's own transcript member.

## The seats

`types.rs` declares the subject roster, the role roster, one profile constant per grammar, and the two citation shapes; its own child `type_guard.rs` holds every road that reaches a private field.
`type_contract.rs` states the constant answers the closed rosters settle, `encode.rs` owns the one length framing every canonical encoding in the crate is written through, and `transcript.rs` is the ten-member specification as code.
