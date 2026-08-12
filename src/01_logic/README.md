# 01_logic — three-valued logic in general

Band 01. Imports band 00 only. Owns the canonical truth values, their K3 (strong
Kleene) connectives, and the decision algebra sited beside them. The six
interval-comparison truth tables stay with the numeric home, which imports `Truth`
from here: logic owns what truth *is*; owners downstream own what *produces* it.

## The values

- **`Truth { True, False, Pending }`** — exactly one three-valued truth in the
  machine; no second `True`/`False` enum exists anywhere; `bool` is never a result
  axis. `Truth` is a knowledge axis, which is why `Pending` is lawful here and in
  no other enum.
- **`Decision { Allow, Deny, Defer }`** — never `Truth` wearing different names:
  no conversion exists in either direction. A decided value carries no authority.

## The connectives (authored: strong Kleene, the standard K3 reading)

This home reads K3 as strong Kleene and writes the connective tables here —
the reading is a design decision of this home, not an inherited one. The
load-bearing consequence: `False` dominates
conjunction, so a lagging answer can never hide a known failure —
`Pending AND False = False`.

## Non-collapse

Eight states are never silently collapsed into false, failure, absence, or
success: missing · pending · unavailable · unauthorized · invalid · refused ·
shredded · outcome-unknown. Missing acknowledgement proves nothing and authorizes
no retry. Two-variant results are lawful only for questions decidable-total from
data in hand (the gate is stated at the crate root; cited, not restated).

## Obligations

```yaml
home: 01_logic
obligations:
  - id: logic.truth-tables-cell-for-cell
    challenge_kind: compile-law
    green: laws.rs logic::truth_tables_cell_for_cell
    red: owed-to-testpak
  - id: logic.pending-cannot-hide-known-failure
    challenge_kind: compile-law
    green: laws.rs logic::pending_cannot_hide_known_failure
    red: owed-to-testpak
  - id: logic.double-negation-is-identity
    challenge_kind: compile-law
    green: laws.rs logic::double_negation_is_identity
    red: owed-to-testpak
  - id: logic.decision-is-not-truth
    challenge_kind: compile-refusal
    green: laws.rs logic::decision_is_not_truth
    red: owed-to-testpak
```
