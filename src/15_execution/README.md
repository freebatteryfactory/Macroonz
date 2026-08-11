# 15_execution — the operator register and Execution Form

Band 15. Imports semantic (BoundDimensionRow), bounds, identity, refusal, and
the root calculus. The portable execution plane: the authored v1 operator
register, the per-operator declaration, Execution Form and its fifteen-cause
family, the agreement seam, well-founded recursion, the effect batch as pure
data, and the five kernel contract types with their three families.

## The operator register (v1 — AUTHORED here)

38 operators, enumerated here because the set has one owner: the four
boundary forms · fold/unfold (the only bounded-lane traversal) · fourteen
iteration operations (including `group` under the grouping ruling and
`relation_expansion` as the separate multi-membership operation) · the three
settled traversal spellings (seek/children/descendants) · `truncate`/`page`
(the old query TAKE renamed `truncate` — iteration owns lowercase `take`, and
one spelling never carries two meanings) · `resolve` · `decide` · the five
derived-data operations (derivation pure, four publications effect nodes) ·
the six owner-specific publication boundaries. Adding, removing, or changing
one advances `ExecutionFormVersion` (the sixth scope-guard instantiation).

## Resolved here: EffectBatch's owner

The intent DATA shape and its composition family seat at 15 (the
atomic-planning lane builds it as pure data); execution, receipts, and
recovery are the runtime home's. No result or receipt member is representable
inside the unsubmitted intent — a shape law with no refusal variant, and none
may be added later.

## Band-forced move executed: SemanticWork → 05_bounds

The seven-record register's `SemanticWork` was authored at 11_navigation for
`Fix`; the canonical record now lives at 05_bounds (both consumers import) —
one type, one owner. The remaining six records: PhysicalEstimate /
ResourceReservation / ReservationEvidence / PhysicalObservation are the
physical membrane's (18); the calibration pair is evidence's (23).

## Owed upward (named owners, band order)

`PlanTemplate`/`PlanBinding` and the ephemeral bound specialization → 20
(the full key law is read and banked: exact-equality static mechanism facts
vs per-admitted-use binding; neither key captures a dynamic fact as static).
The trust-boundary-widening disclosure profile family (the one place unsafe
is even discussed) → 22 owns the law. The reference execution route and
terminals → 17_pakvm. The image validation ladder and admission pipeline →
16_image.

## Obligations

```yaml
home: 15_execution
obligations:
  - id: execution.operator-register-holds-and-versions
    challenge_kind: compile-law
    green: laws.rs execution::operator_register_holds_and_versions
    red: owed-to-testpak
  - id: execution.form-family-holds-fifteen
    challenge_kind: compile-law
    green: laws.rs execution::form_family_holds_fifteen
    red: owed-to-testpak
  - id: execution.effect-batch-composes-as-data
    challenge_kind: compile-law
    green: laws.rs execution::effect_batch_composes_as_data
    red: owed-to-testpak — a result or receipt member on EffectBatch must not
      compile
  - id: execution.recursion-witness-records-eleven
    challenge_kind: compile-law
    green: laws.rs execution::recursion_witness_records_eleven
    red: owed-to-testpak
  - id: execution.kernels-partition-not-duplicate
    challenge_kind: compile-law
    green: laws.rs execution::kernels_partition_not_duplicate
    red: owed-to-testpak — literal construction of a binding-policy arm must
      not compile
  - id: execution.agreement-seam-lists-hold
    challenge_kind: compile-law
    green: laws.rs execution::agreement_seam_lists_hold
    red: owed-to-testpak
```
