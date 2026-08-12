# 00_refusal — how the machine says no

Band 00. It stands below every band, because every checked constructor in every
later band refuses through it, and it imports nothing from any of them. Its one
edge runs the other way: `PrefixRemainder`, the root calculus's witness to a
truncating construction, because the posture a truncated report writes is
selected by that witness rather than by a number a caller chose. The root
calculus imports nothing at all and is the floor under this home.

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

## A halted examination and a truncated report are different facts

`CompletionPosture` names three states, not two, because a reader acts
differently on each. `EarlyStopped` says the EXAMINATION halted at a declared
bound: nothing is known about the sites past it, so a caller who repairs what is
reported must run the pass again to learn whether anything remains.
`ReportTruncated` says the examination covered every declared site and the BODY
does not have room for everything it established — the count is known exactly and
is carried, because "some were dropped" is a claim nobody can act on.

The distinction is minted rather than declared. `ReportTruncation` is opaque with
no public constructor, and `CompletionPosture::examined_completely` is its only
road. That road takes no number. It takes a `PrefixRemainder` — the root
calculus's witness to a truncation that actually happened — and selects the
posture from it, so a body that carried everything cannot claim it truncated and
a body that dropped issues cannot claim completeness. Neither is a discipline a
site has to remember, because neither is a value a site can build.

The witness is what makes the count belong to the body. A `usize` parameter would
have made the posture accurate only by convention: a body that dropped nothing
could still state that seven issues stand outside it, and the type would be
recording an assertion rather than an act. `NonEmptyBounded::admitted_prefix`
mints the witness and has no rival — it is the only construction road in the
machine that truncates at all — so the count a reader acts on is the count that
truncation performed. Writing a remainder by hand does not compile, and neither
does handing the posture road a bare number; both reversals are testpak's.

## The order is typed; the text is its projection

A cause has a stable identity (`CauseId`) that is not its Rust spelling, not its
display text, not prose, and not its position. The identity is a PAIR — the
`RefusalFamilyId` that declares the cause and the `LocalCauseKey` it answers to
inside that family — so family ownership travels in the value and is read rather
than parsed. Two families may declare the same local key; that is a shared word,
and the family seat is what keeps the two identities apart. The canonical text
form `<family>.<local>` is composed from the two seats on demand and is never
stored: a stored join is a third value that can disagree with the two it came
from, and two identities that render alike are still two identities.

`DeclaredCauseOrder` states the canonical order as `DeclaredCause` rows —
identity plus today's spelling — and mints the position (`CauseOrdinal`) out of
its own layout, so no position can disagree with the order it belongs to. Two
consequences are law:

- renaming a Rust variant moves the spelling and moves neither identity nor
  order;
- changing a cause's meaning mints a different identity, and cannot hide behind
  an unchanged spelling.

`RefusalFamily::SELECTION_ORDER` stays exactly what it was and is now named for
what it is: the **textual projection** of that typed order, joined to it by
`DeclaredCauseOrder::projects_to`. One fact, two forms. A family that has not yet
been given stable cause identities does not implement `CauseOrderDeclaration` at
all — an absent declaration is visible where a defaulted one would be a claim
nobody made.

Reason granularity is shape-determined law: single-cause families map every
inhabited cause value to its own stable `ReasonId`; collection families map the
envelope reason at the **family** level — issue identities stay inside the
family value, and no owner elects a "primary issue". No implementation may match
on a cause *spelling* rather than a family *type*.

## A declaration becomes a machine fact by admission

`RefusalFamily` stays open and derivable: any home, and any consumer outside this
crate, declares a family and states its own shape and selection order. Nothing in
the type system makes those two agree, so a road that reads either constant and
acts on it is trusting a pair of declarations nobody joined.
`AdmittedRefusalFamily` is that join, and it is opaque and constructor-free —
holding one IS the evidence. `admit_shape` establishes that the selection order
is non-empty exactly when the shape is single-cause; `admit_order` establishes
that and the typed order's projection, and is available only where a family
declares one.

**Which road ran is a type parameter, not a field.** The witness carries its
coverage as `ShapeCoherent` or `OrderProjected` under a sealed implication
hierarchy: every `OrderAdmission` coverage is a `ShapeAdmission` coverage, and
the reverse does not hold. A consumer states the strength it needs as a bound —
publication takes any `ShapeAdmission` coverage because it acts on the shape
alone; `cause_order`, which hands back the order a caller is about to rank causes
by, hangs off `OrderAdmission`. So the weaker admission passing for the stronger
is unrepresentable rather than checked, and no runtime read stands between the
two. `FamilyAdmissionCoverage` survives as that type's inspection
projection — what a receipt writes down — and is never the axis enforcement
rides.

The envelope's one mint demands the witness. Publication is the act that hands a
refusal to a reader who will act on the family's shape and order without
re-reading them, so an unjoined declaration does not reach it. The coverage the
witness carried in its type is projected onto the envelope, so a refusal
published under coherence alone and one published under coherence and projection
are not the same receipt. The road's reach today is this crate's, because
`ReasonId` carries no public mint until the evidence home registers reasons.

What admission does NOT establish: whether the declared order is the right
selector for the family's checks, anything about the family's Rust body, and
family uniqueness across a whole program — that join stays the composition
root's.

## Variant spelling

Family variants spell themselves one of four ways: negated adjective
(`NotCanonical`), `Not`-prefix on the failed requirement, the prohibited act
itself, or one of exactly two bounds spellings (`Unbounded`, and
`<thing>BoundsMissing` — always plural). Never a third invented form: no `Un-`
prefix on a positively stated property, no `-Dependent` antonym, no `-Mismatch`.
Cross-family spelling collisions over distinct types are deliberate and lawful —
a shared word is shared vocabulary, never a shared type.

## Not here

No seed roster of refusal triggers lives here: every trigger this home once
listed as authoring seed has materialized as a family in its owner home, and
what remains to be written is work-tracking, not specification.

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
  - id: refusal.a-truncated-report-is-not-a-halted-examination
    challenge_kind: compile-refusal
    green: laws.rs refusal::a_truncated_report_is_not_a_halted_examination
    red: testpak/tests/compile-fail/a-truncation-count-with-no-truncation-behind-it.rs
  - id: refusal.cause-identity-outlives-its-spelling
    challenge_kind: compile-law
    green: laws.rs refusal::cause_identity_outlives_its_spelling
    red: owed-to-testpak
  - id: refusal.cause-identity-is-a-family-and-a-local-key
    challenge_kind: compile-refusal
    green: laws.rs refusal::cause_identity_is_a_family_and_a_local_key
    red: testpak/tests/compile-fail/a-cause-identity-cut-from-one-string.rs
  - id: refusal.selection-order-projects-the-typed-order
    challenge_kind: compile-law
    green: laws.rs refusal::selection_order_projects_the_typed_order
    red: owed-to-testpak
  - id: refusal.admission-coverage-is-a-type-parameter
    challenge_kind: compile-refusal
    green: laws.rs refusal::admission_coverage_is_a_type_parameter
    red: testpak/tests/compile-fail/an-admitted-family-minted-bare.rs
  - id: refusal.order-admission-implies-shape-admission
    challenge_kind: compile-refusal
    green: laws.rs refusal::order_admission_implies_shape_admission
    red: testpak/tests/compile-fail/a-weak-admission-at-a-strong-consumer.rs
  - id: refusal.publication-requires-an-admitted-family
    challenge_kind: compile-law
    green: laws.rs refusal::publication_requires_an_admitted_family
    red: owed-to-testpak
```
