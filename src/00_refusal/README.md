# 00_refusal — how the machine says no

Band 00. Imports nothing — this home must stand below everything, because every
checked constructor in every later band refuses through it.

Four observables never collapse: **success ≠ refusal ≠ uncertainty ≠ failure**.
A refusal is a typed, lawful "no" from a check that ran. It is never silent
normalization, never a panic, never an untyped default, and never a claim about
checks that did not run. Uncertainty belongs to the knowledge axes; failure
(infrastructure breakage) belongs to the runtime and evidence planes; neither is
spelled here.

## The three body shapes

A refusal family is a concrete Rust type shaped one of exactly three ways. The
selector is structural — how the checks relate — never taste:

| Shape | When | The lie it makes unrepresentable |
| --- | --- | --- |
| Single cause | dependent checks: each meaningful only after the last passed | claiming results from checks that never ran |
| Issue collection | independent, co-establishable facts | hiding co-true defects; a zero-issue refusal |
| Inseparable pair | exactly two questions, neither meaningful alone | splitting an answer whose halves are nonsense apart |

Single-cause families declare a canonical **selection order** — a selector over
established conditions, never an execution schedule — as a machine-readable
constant on the family. Issue collections ride `NonEmptyBounded` under a declared
limit family and carry `CompletionPosture` as an **instance value** (single-cause
families carry no posture at all). Pairs have exactly two seats; separable
questions must separate.

Reason granularity is shape-determined law: single-cause families map every
inhabited cause value to its own stable `ReasonId`; collection families map the
envelope reason at the **family** level — issue identities stay inside the
family value, and no owner elects a "primary issue". No implementation may match
on a cause *spelling* rather than a family *type*.

## Variant spelling

Family variants spell themselves one of four ways: negated adjective
(`NotCanonical`), `Not`-prefix on the failed requirement, the prohibited act
itself, or one of exactly two bounds spellings (`Unbounded`, and
`<thing>BoundsMissing` — always plural). Never a third invented form: no `Un-`
prefix on a positively stated property, no `-Dependent` antonym, no `-Mismatch`.
Cross-family spelling collisions over distinct types are deliberate and lawful —
a shared word is shared vocabulary, never a shared type.

## Seed trigger roster

The old foundation chapter names these refusal triggers; families landing in later
homes must cover them (roster preserved as authoring seed, not as families):
malformed · noncanonical · wrong-role decode · unknown required meaning ·
pre-allocation ladder breach (length/count/offset/expansion/role) · unlawful
combination · bounds unenforceable at admission · bound exceeded under metering ·
crossing-gain without the named morphism · unmatched (state, input) · undeclared
ambiguity · unlawful chronology advancement · foreign claim unadmitted ·
completeness unproven · configuration unadmitted · atomicity unproven · authority
not earned.

## Obligations

```yaml
home: 00_refusal
obligations:
  - id: refusal.envelope-is-family-generic
    challenge_kind: compile-law
    green: laws.rs refusal::envelope_is_family_generic
    red: owed-to-testpak
  - id: refusal.zero-issue-collection-unrepresentable
    challenge_kind: compile-refusal
    green: laws.rs refusal::issue_collections_are_nonempty_bounded
    red: owed-to-testpak
  - id: refusal.handling-carries-do-not-retry
    challenge_kind: compile-law
    green: laws.rs refusal::handling_carries_do_not_retry
    red: owed-to-testpak
  - id: refusal.selection-order-is-family-declared
    challenge_kind: repository-structure
    green: laws.rs refusal::selection_order_is_family_declared
    red: owed-to-testpak
  - id: refusal.posture-is-a-collection-instance-value
    challenge_kind: compile-law
    green: laws.rs refusal::posture_is_a_collection_instance_value
    red: owed-to-testpak
```
