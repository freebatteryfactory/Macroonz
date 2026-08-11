# testpak — the qualification plane

testpak orchestrates hostile execution against the machine and its tooling, and
carries the denominators every verdict is stated over. **A verdict here is
always claim-specific and method-specific**: "the permuted rendering was
rejected by the string scan over these two declared orders" is a verdict; "the
derive works" is not one.

The dependency direction is the whole point. testpak depends inward — on
`threadpak`, on `threadpak-macroc`, and on `threadpak-macros` — and **nothing
depends on testpak**. Production never depends on its judge, and the
`no-core-tooling-edge` check enforces that absence at the root manifest under
every Cargo edge kind, with planted reversals proving the check can fail.

It is a workspace member and never published: it is dev-and-qualification
material, first-class, and never on the production dependency path.

## The seat map, and what actually occupies it

The plane is a numbered waterfall of seats, mapped by `#[path]` exactly as the
machine maps its bands: the number is visible in the tree and never in a module
name. **A seat exists only once it holds something.** No empty directory stands
here to make the tree look complete, and this table is the honest occupancy
rather than a plan restated as a structure.

Nine seats are foreseen. Four are occupied, and the four that are occupied are
seated where their content actually is — two as source homes, two as test
suites, because cargo requires executable challenge material to live under
`tests/` and moving it elsewhere would be arranging the tree against the tool.

| Seat | Name | Occupancy |
| ---- | ---- | --------- |
| 00 | plan | **seated as `src/00_plan/`** — the denominators a verdict is stated over |
| 01 | — | reserved, unnamed, empty |
| 02 | — | reserved, unnamed, empty |
| 03 | judge | **seated as `src/03_judge/`** — the readers that state a verdict over a rendered artifact |
| 04 | — | reserved, unnamed, empty |
| 05 | — | reserved, unnamed, empty |
| 06 | muterprater | **seated as `tests/planted_defect.rs`** — the mutation seat; today one planted defective expansion and the proof the checker notices |
| 07 | conformance | **seated as `tests/compile_refusals.rs` + `tests/compile-fail/`** — the compile-refusal seat, run through trybuild |
| 08 | — | reserved, unnamed, empty |

The five reserved seats carry no names here on purpose. Numbering a seat is
cheap; naming one commits the plane to a meaning, and a name authored to fill a
row in a table is exactly the kind of hand-maintained inventory this repository
refuses. They are named when their content lands.

| Path | What it carries |
| --- | --- |
| `src/00_plan/` | `RedTwinLedger`: expected against discharged, with no road past the denominator |
| `src/03_judge/` | `RenderVerdict` and the readers that produce it; `structural.rs` carries lane B |
| `src/laws.rs` | the plane's own compile-time proof surface, sectioned by seat |
| `tests/planted_defect.rs` | the planted defective expansion, the proof the checker notices, and the rehearsed false alarm |
| `tests/compile_refusals.rs` | the trybuild runner over the compile-fail fixtures |
| `tests/compile-fail/` | one fixture per discharged red twin |

## `Unreadable` is a failure class with its own alarm

`RenderVerdict::Unreadable` is not noise, not a skip, and not a softer
`Deviates`. It means one specific thing: **the judge could not find the
construct it anchors on.** Either the artifact stopped stating that construct,
or the artifact still states it and the anchor no longer matches the text.
Both are real findings.

So a test asserting that a lawful rendering conforms must FAIL on `Unreadable`,
and must never be written to accept it alongside `Conforms`. A silent
`Unreadable` is worse than a deviation: a deviation says the renderer is wrong,
while an ignored `Unreadable` says nothing at all while every assertion
downstream of it quietly stops testing anything.

**The response to a false alarm is to fix the anchor deliberately, never to
loosen the reader.** When a rendering legitimately changes shape, the anchor
constant in `src/03_judge/mod.rs` is re-stated to match the new shape, in one
place, on purpose, visible in the diff. Widening the reader until it matches
again — trimming whitespace, matching a prefix, adding a looser fallback — buys
a green run by making the judge cleverer, and a clever judge has stopped reading
the artifact and started agreeing with the renderer.

The alarm is rehearsed rather than trusted:
`a_whitespace_shifted_lawful_rendering_is_unreadable` takes a genuinely lawful
rendering, shifts blank space inside the anchored const item, and requires
`Unreadable` — with the unshifted text asserted to conform first, so the
rehearsal cannot pass over a rendering that was simply wrong.

## The planted defect

Every macro family in this repository ships with a deliberately defective
expansion that testpak must reject. The refusal-family derive's is
`PlantedDefect`, reached through a documented qualification seam on the
derivation: it changes the RENDERING and leaves the captured declaration alone,
so the defect is a disagreement between the text and the declaration it claims
to project — detectable, and detectable only from outside.

Activation is observed, not assumed. The test asserts both directions: the
defective rendering IS rejected, and the lawful rendering passes. A checker that
rejected everything would satisfy the first half and fail the second.

The judge does not ask the services what the declared order was. It states the
order itself, beside the declaration it wrote, so the comparison is between two
independent statements rather than between a value and itself.

## The compile-fail fixtures

trybuild covers compile refusals only — one challenge kind among several, never
the universal one. Each fixture here discharges a red twin some green law names:

| Fixture | The reversal it proves |
| --- | --- |
| `singleton-under-a-zero-maximum-family.rs` | `NonEmptyBounded::singleton` under a limit family declaring `MAX = 0` does not compile |
| `cross-scope-comparison-on-a-stamped-guard.rs` | two stamped scope guards over different scopes are different types; comparing them does not compile |
| `a-stamped-representation-cannot-be-laundered.rs` | two stamped guards over ONE scope: taking role A's position out and re-entering it under role B does not compile, in either direction |
| `a-malformed-refusal-declaration-refuses.rs` | a shape word outside the machine's roster reaches the compiler as a refusal, not as a silent empty expansion |

## Owed, and counted out loud

The great majority of the red twins the green laws name are still owed. The
`red:` rows in every home README are the ledger, and `cargo xtask check` prints
the denominator on every run — `red twins: N discharged / M owed` — so the
accounting is visible rather than inferred, and a shrinking numerator has
nowhere to hide. A row spelled `owed-to-…` is a lawful debt; a row NAMING a
reversal must resolve to a file in this directory, and the check refuses it if
it does not.

This directory is where those reversals land, and it is nearly empty. Also
owed: the hostile corpora (permutation, determinism, and ambient-pathway
hostiles the services' own docs name), and the mutation machinery beyond the one
planted defect. Sequenced, not deferred.

## Tooling qualification obligations

A verdict about a TOOL is not a verdict about the machine, and the two
denominators are never added. Each block binds the claim, the owner, the
positive control, the reversal, the activation route, the method, and the
nonclaims.

```yaml
tooling-obligation: testpak.lane-a-catches-what-lane-a-owns
  claim: >
    The byte-profile scan catches every mutation whose ownership it is recorded
    as holding, and the lawful artifact passes it.
  owner: testpak/src/03_judge/mod.rs
  positive: testpak/tests/planted_defect.rs
  method: mutation
  activation: cargo test -p threadpak-testpak --test planted_defect
  tooling-red: testpak/tests/planted_defect.rs
  nonclaims: >
    It claims nothing structural. A decoy in a comment and an unplanned output
    are both invisible to it, and the same test asserts that they are.

tooling-obligation: testpak.the-anchor-alarm-is-rehearsed
  claim: >
    A lawful artifact whose anchored form has shifted reads as `Unreadable`, and
    the unshifted control conforms first — so the alarm is known to sound before
    anyone has to interpret one.
  owner: testpak/src/03_judge/types.rs
  positive: testpak/tests/planted_defect.rs
  method: mutation
  activation: cargo test -p threadpak-testpak --test planted_defect
  tooling-red: testpak/tests/planted_defect.rs
  nonclaims: >
    It does not claim the anchor is stable across renderer changes; it claims the
    opposite, and fixes the response.

tooling-obligation: testpak.lane-b-structural-reader
  claim: >
    An independent parse of the rendered artifact recovers what it DECLARES —
    implementation target, trait path, body shape, cause rows in order, output
    cardinality, duplicate items, unplanned items — and catches every one of the
    four mutations the ownership ledger records as structural, while the lawful
    artifact conforms.
  owner: testpak/src/03_judge/structural.rs
  positive: testpak/tests/planted_defect.rs
  method: structural-read
  activation: cargo test -p threadpak-testpak --test planted_defect
  tooling-red: testpak/tests/planted_defect.rs
  nonclaims: >
    It claims nothing about compilation. Not that the artifact typechecks, not
    that its paths resolve, not that the trait it names exists, not that any
    constant evaluates to the value its spelling suggests. A wrong trait path
    reads as a different path than the one declared, never as no such contract.
```

## The three lanes, and what each one may claim

**Lane A — the byte-profile scan** (`src/03_judge/mod.rs`). Its claim is exactly
*the rendered text contains this exact declared textual form* and never anything
structural. It reads text `rustc` never touched, which is exactly its value: it
catches a renderer emitting the wrong bytes before those bytes are ever offered
to a compiler, and it costs a string search.

**Lane B — the structural read** (`src/03_judge/structural.rs`). Its claim is
exactly *the artifact DECLARES these implementations, of these traits, for this
target, carrying these members* — what item is this, what does it target, which
contract does it realize, are the cause rows the declared ones, did an item
nobody planned come along, was one emitted twice. It answers those by parsing the
text with a parser nobody here wrote, and it claims nothing about whether any of
it compiles.

**Lane C — compiled behaviour** (`xtask/fixtures/macro-consumer`,
`xtask/fixtures/renamed-consumer`). `rustc` compiles the derived artifact and the
tests read its trait constants as VALUES, comparing them against hand-written
twins. The renamed-consumer fixture reaches the machine only under `tp`, so a
renderer hardcoding the default binding would fail to compile there.

**Why the dumb reader stays dumb.** A cleverer reader has to decide what the text
MEANS, and the only way to decide that is to implement the same understanding the
renderer already has. Two implementations of one understanding, written by the
same hands against the same document, agree because they SHARE THE CHALLENGED
IMPLEMENTATION — not because either of them understands Rust. Correlated evidence
about a renderer is not independent of that renderer. Lanes B and C escape this
because their decoders — `syn` and `rustc` — are decoders nobody here wrote.

## The structural lane's admitted dependency

**Why the string lane cannot answer this claim.** Lane A finds bytes. Told that
`const SELECTION_ORDER : … = &["A", "B"]` is present, it has established that
those bytes are somewhere in the text — not that the artifact declares an
implementation, not that the implementation targets the right type, not that the
constant is a MEMBER of that implementation, not that a second implementation
nobody planned is sitting beside it, and not that a comment put the bytes there.
Making the scan answer any of that means teaching it what an item, a path, a
member, and a comment are — which is writing a Rust parser, by hand, inside the
judge, from the same understanding the renderer was written from. Two readings of
one understanding agree because they share it. So the claim is not reachable by
making lane A cleverer; it is reachable only by handing the text to a decoder
that owes this repository nothing.

**The dependency.** `syn`, pinned exact at `=3.0.3` — the version the lockfile
already carries — with `default-features = false` and exactly two features:
`parsing`, without which there is no text-to-tree road at all, and `full`,
without which items and their associated constants are not in the tree. Printing,
folding, visiting, derive input, extra traits, and the proc-macro bridge are all
off: the lane reads and never writes, and never runs inside a macro.

**What it reads out of the tree.** Per item: whether the item is a trait
implementation at all, the trait path (segments and leading `::`), the target
type path, and each associated constant by name — `SHAPE`'s variant word,
`SELECTION_ORDER`'s string list in order, and `DECLARED_ORDER`'s cause rows as
the pairs they are, each row's stable identity and its spelling, in order. Across
items: how many were declared, whether one trait-and-target pair was implemented
twice, and whether anything that is not a declared trait implementation came
along.

**What it shares with the producer: nothing.** Not the capture, not the plan, not
the renderer, not `GeneratedTree` or any token type the renderer writes, not the
projection that turns that tree into text, and no constant, anchor, or spelling
imported from `threadpak-macroc`. The declaration it compares against is written
out in full in `tests/planted_defect.rs`, beside the declaration handed to the
services and independent of it. The parse is `syn`'s, by Rust's rules.

**What it does NOT claim.** Nothing about compilation. Not that the artifact
typechecks, not that its paths resolve, not that the trait it names exists, not
that the target type exists, not that the implementation is coherent, and not
that any constant evaluates to the value its spelling suggests.
`::threadpak::refusal::SomethingElse` reads here as *a different path than the
one declared* and never as *no such contract*. All of that is lane C's, where
`rustc` compiles the artifact and the constants come back as values. And
`Unparsable` is a failure class of its own, exactly as lane A's `Unreadable` is:
never a skip, never a softer `Deviates`.

**The planted mutations it must catch**, all four recorded as structural in
`src/03_judge/mutation.rs`, each landing on a different structural fact:

| Mutation | What lane B reports |
| --- | --- |
| `ImplTargetAltered` | the implementation targets a type the declaration did not name |
| `TraitPathWrong` | the implementation realizes a trait path the declaration did not name |
| `UnplannedOutputAdded` | the artifact declares more implementations than the declaration names |
| `DecoyInComment` | the selection order the artifact DECLARES is not the declared roster — the comment carrying the anchored bytes is not in the tree at all |

The decoy is the pair that makes the split visible in one line: lane A reports
`Conforms` on it and lane B reports the reversed order, and both are right, in
their own methods, about different questions.

## Mutation is the judge's job

The services no longer carry a road that renders a deliberately defective
artifact. A generator writing its own exam is rehearsed only against the defects
it already imagined. `src/03_judge/mutation.rs` declares ten mutations, damages
lawful artifacts itself, and records per mutation WHICH LANE owns catching it.
Ownership is the seat of the CLAIM rather than a boast of exclusivity: lane B
notices a permuted order too and says nothing about it, because that verdict is
stated over lane A's method and belongs to lane A's row.
