# 04_numeric — exact families, intervals, quantize, honesty

Band 04. Imports logic (Truth), refusal (families), and the root calculus. The
largest foundation home: ten exact families (nine named + the schema-admitted
extension slot), twelve refusal families, interval decisions, quantization,
rounding, admitted approximation, and the numeric-honesty layer — including
`Finality`, sited here with its first caller.

## The ladder

`unit → scale → range → witness coherence` (`CONSTRUCTOR_AXIS_LADDER`). This IS
every family's deterministic cause-selection rule; Rust variant order is never
the rule; repair direction is the ladder itself. A family that lists a cause for
an axis it does not carry has copied a neighbour rather than stated itself.
Every numeric constructor family is a closed single-cause enum — no
`CompletionPosture`, no issue carrier, no exhaustiveness claim.

## Not here

No changelog of executed decisions lives here: every decision this home received
is restated at the declaration it governs. Change history is the repository
ledger's, never a section of a specification.

## Declared incomplete (owed to owners)

Currency/time-unit/unit-domain designation members (schema home); cause payload
fields (schema identities); `QuantizeEvidence`'s seven remaining facts; the
interval family roster (which makes the truth tables executable); the
`DistributionEstimate` shape; the wide-exact seam carrier; the `decide`
operation itself (a machine algorithm — closed until implementation opens).

## Obligations

```yaml
home: 04_numeric
obligations:
  - id: numeric.constructor-axis-ladder-is-ordered
    challenge_kind: compile-law
    green: laws.rs numeric::constructor_axis_ladder_is_ordered
    red: owed-to-testpak
  - id: numeric.families-are-single-cause-with-declared-orders
    challenge_kind: compile-law
    green: laws.rs numeric::families_are_single_cause_with_declared_orders
    red: owed-to-testpak
  - id: numeric.rounding-modes-are-six
    challenge_kind: compile-law
    green: laws.rs numeric::rounding_modes_are_six
    red: owed-to-testpak
  - id: numeric.float-classes-are-six
    challenge_kind: compile-law
    green: laws.rs numeric::float_classes_are_six
    red: owed-to-testpak
  - id: numeric.requirement-disposition-has-six-terminals
    challenge_kind: compile-law
    green: laws.rs numeric::requirement_disposition_has_six_terminals
    red: owed-to-testpak
  - id: numeric.finality-is-generic-over-cut
    challenge_kind: compile-law
    green: structural (Finality<Cut> takes the owner's cut as a type parameter
      and names no cut of its own, so a universal finality cut has nowhere to be
      written)
    red: owed-to-testpak — hard-wiring one cut type must break the signature
  - id: numeric.interval-relations-are-six
    challenge_kind: compile-law
    green: laws.rs numeric::interval_relations_are_six
    red: owed-to-testpak
  - id: numeric.knowledge-axis-selection-order-is-declared
    challenge_kind: compile-law
    green: laws.rs numeric::knowledge_axis_selection_order_is_declared
    red: owed-to-testpak
  - id: numeric.quantize-evidence-binds-nine-facts
    challenge_kind: compile-law
    green: laws.rs numeric::quantize_evidence_binds_nine_facts
    red: owed-to-testpak
  - id: numeric.designations-do-not-unify
    challenge_kind: compile-refusal
    green: laws.rs numeric::designations_do_not_unify
    red: owed-to-testpak
  - id: numeric.interval-truth-tables-cell-for-cell
    challenge_kind: runtime-positive
    green: owed — executable when the interval family roster lands
    red: owed-to-testpak
```
