# 03_value — the value plane's laws

Band 03. Imports the root calculus only. This home is mostly laws made
machine-readable; its content is dense in rules and deliberately thin in types.

What is deliberately elsewhere: the seven byte roles live in `07_bytes`; the
concrete validated-value model (the 21-variant algebra) lands with `08_schema`,
which instantiates this home's laws. And one thing is absent *by law*: **there is
no universal null type** — a Nullable field's null is that schema's declared
meaning in its own value domain, never a shared sentinel. The sentinel's
nonexistence is the design.

## The six absence worlds

`ShapeOptional · ValueNull · Unauthorized · Unmaterialized · Pending ·
OutcomeUnknown` (the original spellings, kept by decision). Every foreign absence is
classified exactly once, at decode, into its typed axis; after admission,
unclassified null does not exist. The enum is a classification namespace that
routes to owning axes — `Pending` routes to the `Truth` knowledge axis,
`OutcomeUnknown` to the runtime's outcome knowledge — it is not a result axis.

## The pre-authority ladder

Readers validate lengths, counts, offsets, expansion, and role — in that order —
before any allocation or authority. Declared as `PRE_AUTHORITY_LADDER`; cited by
the bytes home's readers, never restated.

## The canonical inbound path

Eight stages from carrier bytes to derived materialization, declared as
`CANONICAL_INBOUND_PATH`. Stages pipeline, never merge: field-name similarity, a
valid transport message, or a successful decode chooses no transformation and
grants no admission.

## Lossy operations

Seven, closed, distinct: quantization · redaction · summarization · projection ·
sampling · truncation · selection. Never one generic transform; each owner
performing one owes its own disclosure row.

## Unicode mechanism — Decision (2026-08-10), in tiers

1. **Contraband refusal is IN-HOUSE owned code, not a mechanism**: bidi
   ordering controls, raw controls, surrogates, and noncharacters refuse
   against hardcoded scalar sets (a stable dozen ranges) — no crate, no
   tables. This is the data-as-instruction firewall's text tier.
2. **NFC validation: the `unicode-normalization` crate family — ADMITTED.**
   Protects digest-identity honesty (one meaning, one byte spelling, one
   commitment) wherever a schema declares a normalized text refinement, and
   unblocks `BoundedText`'s real constructor when the text machinery lands.
   Admitted under the mechanism-standing law: admission is not
   qualification; swappable behind the machine-owned text role contract.
3. **Confusables / identifier hygiene (UTS #39 tables): NOT ADMITTED.** The
   live Rust-declaration frontend's names are already vetted by rustc, so no
   core home carries the heavy tables. A frontend that needs confusable
   detection carries it itself, on its own side of the declaration path.
