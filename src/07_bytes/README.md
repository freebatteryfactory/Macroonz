# 07_bytes — the shared byte primitives

Band 07. Imports identity, refusal, and the root calculus. Owns what every
binary role shares: the one frame grammar (14-byte header, 32-byte digest
trailer, one `TPAK` magic for every binary file), the domain-tag register's
shape and its four-projections law, the digest-family law, the width
conventions, the text-form scheme, the eight commitment roles, content regions,
and the sixteen-maxima bounded-reader roster. **Role-specific frame profiles are
their owner homes' rows** (EventFrame at history, image components at image,
cursors at their owners) — primitives shared, container identity/version
line/recovery law per role, and a payload codec never versions its enclosing
frame.

## The two-bound law

The u32 `length` is the physical bound; the semantic bound is the capacity
profile checked before decode; anything larger is a content region by
construction — the tiering law expressed as a field width.

## The domain-tag register

`threadpak/<tag-version>/<family>/<role>/<schema-version>`. Each row emits four
projections — derive-key context, text-form prefix, frame role id, docs table —
from one row, by the macros crate, so wire id, human prefix, and hash domain
cannot drift. Two layers stay distinct: the logical preimage
(algorithm-independent) and the digest transcript (algorithm-specific).

## Mechanism admissions (RULED by the repository owner, 2026-08-10)

1. **Digest family: blake3-256 — ADMITTED.** Hash / keyed-hash-under-KeyScope
   / derive-key-as-domain-tag map natively onto settled law; the tree
   structure makes verified slice reads of content regions realizable (a
   capability with its own qualification surface, never free). An admitted
   mechanism under the mechanism-standing law: admission is not
   qualification, qualification is not a support promise, and the family
   remains swappable behind the machine-owned digest role contract.
2. **Text-form checksum: bech32m — ADMITTED.** The strict role-covering
   scheme (checksum domain includes the role prefix; mixed case refuses).
   Gates only the human-surface text rendering of identities — persistence
   and the event store are binary and never touch it. Same standing law
   applies.

## Obligations

```yaml
home: 07_bytes
obligations:
  - id: bytes.frame-header-is-fourteen-and-trailer-thirty-two
    challenge_kind: compile-law
    green: laws.rs bytes::frame_header_is_fourteen_and_trailer_thirty_two
    red: owed-to-testpak
  - id: bytes.frame-decode-ladder-is-declared
    challenge_kind: compile-law
    green: laws.rs bytes::frame_decode_ladder_is_declared
    red: owed-to-testpak
  - id: bytes.commitment-roles-are-eight
    challenge_kind: compile-law
    green: laws.rs bytes::commitment_roles_are_eight
    red: owed-to-testpak
  - id: bytes.text-form-ladder-is-declared
    challenge_kind: compile-law
    green: laws.rs bytes::text_form_ladder_is_declared
    red: owed-to-testpak
  - id: bytes.tag-projections-are-four
    challenge_kind: compile-law
    green: laws.rs bytes::tag_projections_are_four
    red: owed-to-testpak
  - id: bytes.decode-maxima-are-sixteen
    challenge_kind: compile-law
    green: laws.rs bytes::decode_maxima_are_sixteen
    red: owed-to-testpak
  - id: bytes.width-conventions-are-eight
    challenge_kind: compile-law
    green: laws.rs bytes::decode_maxima_are_sixteen
    red: owed-to-testpak
  - id: bytes.content-region-declares-two-columns
    challenge_kind: compile-law
    green: laws.rs bytes::content_region_declares_two_columns
    red: owed-to-testpak
  - id: bytes.golden-frame-vectors
    challenge_kind: golden-bytes
    green: owed — encoder/decoder machinery lands with the digest mechanism
    red: owed-to-testpak
```
