# 03_value — the value plane's laws

Band 03. Imports the root calculus only. This home is mostly laws made
machine-readable; its ore is dense in rules and deliberately thin in types.

What is deliberately elsewhere: the seven byte roles live in `07_bytes`; the
concrete validated-value model (the 21-variant algebra) lands with `08_schema`,
which instantiates this home's laws. And one thing is absent *by law*: **there is
no universal null type** — a Nullable field's null is that schema's declared
meaning in its own value domain, never a shared sentinel. The sentinel's
nonexistence is the design.

## The six absence worlds

`ShapeOptional · ValueNull · Unauthorized · Unmaterialized · Pending ·
OutcomeUnknown` (ore spellings kept by ruling). Every foreign absence is
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

## Open ruling carried

`BoundedText` normalization profile (only paths pinned: NFC, refuse
non-canonical). No constructor until ruled.

## Obligations

```yaml
home: 03_value
obligations:
  - id: value.absence-worlds-are-closed-and-six
    challenge_kind: compile-law
    green: laws.rs value::absence_worlds_are_closed_and_six
    red: owed-to-testpak
  - id: value.pre-authority-ladder-is-ordered
    challenge_kind: compile-law
    green: laws.rs value::pre_authority_ladder_is_ordered
    red: owed-to-testpak
  - id: value.inbound-path-has-eight-unmerged-stages
    challenge_kind: compile-law
    green: laws.rs value::inbound_path_has_eight_unmerged_stages
    red: owed-to-testpak
  - id: value.lossy-operations-stay-distinct
    challenge_kind: compile-law
    green: laws.rs value::lossy_operations_stay_distinct
    red: owed-to-testpak
  - id: value.bounded-text-carries-its-limit-family
    challenge_kind: compile-refusal
    green: laws.rs value::bounded_text_carries_its_limit_family
    red: owed-to-testpak
  - id: value.no-universal-null-sentinel
    challenge_kind: repository-structure
    green: none — the type's nonexistence is the law
    red: owed-to-xtask-and-testpak
```
