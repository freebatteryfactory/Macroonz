# 10_history — accepted history

Band 10. Imports identity, refusal, schema, value, and the root calculus. The
machine's memory: the four-object split, exact local order under one writer,
lineage transitions, partitions and affine handoff, federation cuts, commit
knowledge, durability, the storage port's typestate roots, authenticated
history, the complete authorized-removal model, and `.tlog` recovery.

## Two band-forced seatings and one executed rename

1. **`SourceClosure` seats here** (navigation, one band up, imports it for
   `Fix`) — one owner, navigation a consumer. Band math decides the seat.
2. **Executed rename**: the cut's ordinal member is `ceiling` — the
   watermark-family word is banned vocabulary; the substance ("the visible
   ceiling of published history") is unchanged.

## Firsts

The machine's first **composite-pair refusal family** (`LineageRefusal`:
reason + partial evidence, neither droppable) and the first production
`EvidenceCut` instantiation (`HistoryCut`), first `Completeness` instantiation
(`SourceClosure`), plus four more identity-class instantiations and two more
scope-guarded orders (`AuthorityGeneration`, `WriteAuthorityEpoch`).

## The 30-scenario crash-recovery denominator (binding; campaigns are testpak's)

acknowledged-durable-events-survive · partial batch invisible/rolled-back/
refused · never a shorter successful batch · partial tail discarded with
receipt · bytes beyond last receipt discarded-with-receipt · invalid at-or-
before boundary refuse-and-hold · middle deletion detected · reordering
detected · duplicate-payload splice detected · foreign predecessor detected ·
cross-lineage substitution detected (accumulator: scope identity ≠ world
identity) · unauthorized genesis detected · derived-state reconstruction is a
category error · unreferenced durable extent reclaimed · committed event
referencing nondurable bytes = Corrupt-not-absence · compaction inputs survive
until replacement durable · idempotency survives reopen (anchored to the
commitment chain, never a bare integer a diverged sibling could replay) ·
durable content without namespace entry = not published · partial group ack
loses no member, replay duplicates none · interrupted import retryable without
duplication · self-ingest prevented by frozen source ceiling · non-matching
projection event = lawful filter · failing decode/apply = typed terminal ·
four recovery paths agree on first failure · restored older generation =
valid-but-stale · two accumulator roots at one position = detected
equivocation · restored material earns standing only after witness comparison ·
Required witness absent = refuse not weaken · lost ack lowers receipt
completeness only · per-record verdicts speak K3, narrowing fail-closed.

## Not here

The storage port's method roster and its postcondition rows land with the
storage adapter contract in host space, not in this home. What is law here is
the durability CLAIM algebra: twelve typed postcondition axes and the profile
that carries them, so no adapter can substitute a weaker guarantee.

## Resolved: implementation-swap identity stability (discharged by structure)

An implementation swap changing implementation authority while preserving
lineage and semantic target identity needs no row here: its guarantee is
structural in this machine. Identity is a commitment over
meaning and the realization is never in the preimage, so an implementation swap
cannot change identity arithmetic; accepted history never contains
realizations, so a swap cannot touch it; and a reuse key spanning a swap
already must digest its complete input set by the derived-seat law. If any
future band finds it needs this row as prose, that band has leaked realization
detail into an identity preimage — fix the preimage, not the prose.

## Obligations

```yaml
home: 10_history
obligations:
  - id: history.four-object-split-and-knowledge-axes
    challenge_kind: compile-law
    green: laws.rs history::four_object_split_and_knowledge_axes
    red: owed-to-testpak
  - id: history.lineage-refusal-is-a-composite-pair
    challenge_kind: compile-law
    green: laws.rs history::lineage_refusal_is_a_composite_pair
    red: owed-to-testpak
  - id: history.federation-composition-is-a-seam
    challenge_kind: compile-law
    green: laws.rs history::federation_composition_is_a_seam
    red: owed-to-testpak
  - id: history.order-roles-do-not-unify
    challenge_kind: compile-refusal
    green: laws.rs history::order_roles_do_not_unify
    red: owed-to-testpak — a From/Into bridge between order roles must not compile
  - id: history.reading-has-three-orthogonal-axes
    challenge_kind: compile-law
    green: laws.rs history::reading_has_three_orthogonal_axes
    red: owed-to-testpak
  - id: history.removal-families-hold-their-rosters
    challenge_kind: compile-law
    green: laws.rs history::removal_families_hold_their_rosters
    red: owed-to-testpak
  - id: history.recovery-has-three-endings-and-five-steps
    challenge_kind: compile-law
    green: laws.rs history::recovery_has_three_endings_and_five_steps
    red: owed-to-testpak
  - id: history.epoch-and-cut-succession-are-typed
    challenge_kind: compile-law
    green: laws.rs history::epoch_and_cut_succession_are_typed
    red: owed-to-testpak
  - id: history.crash-recovery-denominator
    challenge_kind: crash-recovery
    green: owed — the 30-scenario campaigns land with testpak
    red: owed-to-testpak
```
