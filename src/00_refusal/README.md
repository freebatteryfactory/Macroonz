# 00_refusal — how the machine says no

Band 00. It stands below every band, because every checked constructor in every
later band refuses through it, and it imports nothing from any of them. Its one
edge runs the other way, into the root calculus: a collection-shaped refusal body
is a `NonEmptyBounded` and this home owns the posture that body reports its own
coverage with, so this home is where the two are married and `AdmittedPrefix`
lives. The root calculus imports nothing at all and is the floor under this home.

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
constant on the family. Issue collections ride `AdmittedPrefix`, which is a
`NonEmptyBounded` under a declared limit family married to the
`CompletionPosture` its own construction selected, carried as an **instance
value** (single-cause families carry no posture at all). Pairs have exactly two
seats; separable questions must separate.

## A halted examination and a truncated report are different facts

`CompletionPosture` names three states, not two, because a reader acts
differently on each. `EarlyStopped` says the EXAMINATION halted at a declared
bound: nothing is known about the sites past it, so a caller who repairs what is
reported must run the pass again to learn whether anything remains.
`ReportTruncated` says the examination covered every declared site and the BODY
does not have room for everything it established — the count is known exactly and
is carried, because "some were dropped" is a claim nobody can act on.

The distinction is minted rather than declared. `ReportTruncation` is opaque with
no public constructor, and `AdmittedPrefix::examined_completely` is its only
road. That road takes no number. It takes the material and performs the
truncation, so the count it writes down is the count it just dropped: a body that
carried everything cannot claim it truncated and a body that dropped issues
cannot claim completeness. Neither is a discipline a site has to remember,
because neither is a value a site can build.

The halted posture has its own road, `AdmittedPrefix::stopped_early`, which
couples the carry a halted pass handed over to the bound it stated it stopped at
in one construction. Its honesty ceiling is stated where it lives: the
constructor structurally couples the body to the posture, and it does not prove
that an external examination truly halted — the family owner's algorithm
establishes the behavioral claim, and its proof lives outside this crate. It
refuses where the truncating road
truncates, because `ReportTruncated` has a seat to record what it dropped and
`EarlyStopped` has none, so material past the declared bound could only be
dropped silently. No caller exists today, because no scan in the machine halts;
the road exists so the first honestly halting family is coupled rather than
pushed back onto a loose body beside a loose posture.

Performing the act is what makes the count belong to the body. A `usize`
parameter would have made the posture accurate only by convention: a body that
dropped nothing could still state that seven issues stand outside it, and the
type would be recording an assertion rather than an act.

And the count leaves that road married to the carry, because provenance alone is
not enough. A count minted by the one road that truncates is a count some
truncation really performed — but two values handed to a caller are two values a
caller may pair, so the body one pass truncated could be reported under the count
another pass dropped, with both halves honest and the pair a lie. So a report
body IS an `AdmittedPrefix`: private seats, no `into_parts`, no owned carry, and
one construction behind both readings.

The home carries the grammar's guard seat for exactly that marriage: `types.rs`
declares `AdmittedPrefix` and `ReportTruncation`, and its own child
`type_guard.rs` owns every road that touches their seats — the two mints, the
truncation a mint performs, and the two readers that hand the carry and the
posture back — so the marriage is performed in one file and no seam elsewhere in
the crate can build either half.

**The claim's exact reach: every collection-shaped refusal family in the machine
and in the services.** Not this home's bodies and the tooling plane's six — all
of them, the upper-band declaration families included. A family's body is one
`AdmittedPrefix` seat read back through `issues()` and `posture()`, and the two
seats those families used to spell are gone rather than deprecated. The coupling
is the declaration's own shape: one seat answers both readers, so there is no
second seat for either half to drift from.

What the reach does NOT include: a construction road for any upper-band family.
No migrated family carries a constructor of any kind, because none of them has an
enumerating pass to build one from. Seven of them stand one step further back
still: their limit families declare no compile-time magnitude, and every
`AdmittedPrefix` mint consumes one, so those seven cannot produce the value their
own seat holds until a magnitude is declared or a runtime-witnessed prefix road
exists. Coupled seats with no road to them is the honest state of a declaration
nobody has written a pass for; it is not a coupling the road would fail.

Writing a truncation posture by hand does not compile; neither does marrying one
report's carry to another report's completion; and neither does assembling a
migrated family out of a carry and a posture. All three are unrepresentable: the
seats are private and `type_guard.rs` owns every road that touches them.

**What those three establish is representation privacy, and each says so at its
own head.** The seats are not a caller's to write, so neither the cross-wired
pair nor the fabricated posture is a value that can be assembled. That an
`into_parts`, an owned carry, or a second mint does not EXIST is a fact about the
surface as it stands and is held by review: adding one would leave every one of
those three errors exactly where it is. Deriving that absence needs a
machine-readable declaration of which package is sealed and which mint is the
one, and this home has none — `CauseId` has two private seats it hands back on
purpose, beside a public mint that takes them, so a reader condemning the one
would condemn the other. The stamped scope guards are derived precisely because
their stamp IS that declaration.

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

No roster of refusal triggers lives here. Every family is a concrete Rust type
in the home whose checks establish it; what this home owns is the shapes those
bodies take, the identities they declare, and the join that admits a
declaration — never a list of what the machine refuses by.
