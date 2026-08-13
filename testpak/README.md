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
| 01 | corpus | reserved — `src/01_corpus/README.md` states the question, the filling condition, and the nonclaims |
| 02 | arena | reserved — `src/02_arena/README.md` states the question, the filling condition, and the nonclaims |
| 03 | judge | **seated as `src/03_judge/`** — the readers that state a verdict over a rendered artifact: lane A in `byte_profile.rs`, lane B in `structural.rs`, the damage in `mutation.rs`, everything they may say in `types.rs` |
| 04 | simulation | reserved — `src/04_simulation/README.md` states the question, the filling condition, and the nonclaims |
| 05 | fault | reserved — `src/05_fault/README.md` states the question, the filling condition, and the nonclaims |
| 06 | muterprater | **seated as `tests/planted_defect.rs`, `tests/failed_seat_refusals.rs`, `tests/declared_magnitudes.rs`, and `tests/compiled_behaviour.rs`** — the mutation seat: the damaged artifacts and the proof each lane notices what it owns, the killed repairs restored one at a time, the declared magnitudes driven both directions, and the two mutants that need a compiler |
| 07 | conformance | **seated as `tests/compile_refusals.rs` + `tests/compile-fail/`** — the compile-refusal suite, run through trybuild — and as `tests/independent_identity_transcript.rs`, the independent transcript lane |
| 08 | evidence | reserved — `src/08_evidence/README.md` states the question, the filling condition, and the nonclaims |

The five reserved names restore the plane's original nine-seat design; they were
placed on this map by decision, not authored to fill it. **A reserved name fixes the
intended question of a seat. It does not claim that the seat exists, has an
owner implementation, or has satisfied admission** — name reserved ≠ home
materialized ≠ implementation admitted ≠ qualification established. Content that
does not fit its reserved name comes back for an explicit decision instead of
being normalized into the nearest drawer.

Each reserved seat is a directory carrying exactly one file: a README stating the
seat's question, that its state is reserved, that it currently contains nothing,
the exact condition that would fill it, and what the reservation explicitly does
not claim. That is a coordinate carrying its own honest specification, and it is
the opposite of an empty directory dressed up to look occupied. **No `mod.rs`, no
type, no API, and no obligation row stands for a reserved seat** — `lib.rs`
declares no module at any of these five coordinates, so nothing in this package
can reach one, and the map says so out loud rather than leaving a reader to
discover it.

| Path | What it carries |
| --- | --- |
| `src/00_plan/` | `RedTwinLedger`: expected against discharged, with no road past the denominator |
| `src/03_judge/types.rs` | everything the seat can say: both verdicts, everything lane B recovers, what a caller declares against, and the fifteen-mutation roster |
| `src/03_judge/type_contract.rs` | the roster's closed tables: which lane owns catching each mutation, and the sentence each one is shown as |
| `src/03_judge/byte_profile.rs` | lane A: the three exact anchors and the readers that scan for them |
| `src/03_judge/structural.rs` | lane B: the walk over the tree `syn` hands back, and the coarse-to-fine comparison against the caller's declaration |
| `src/03_judge/mutation.rs` | the damage itself: string surgery that makes a lawful artifact lie |
| `src/laws.rs` | the plane's own compile-time proof surface, sectioned by seat |
| `tests/planted_defect.rs` | the planted defective expansion, the proof the checker notices, and the rehearsed false alarm |
| `tests/failed_seat_refusals.rs` | each repair the failure-path law killed, restored by this plane and shown to be about another subject |
| `tests/declared_magnitudes.rs` | the four magnitudes a captured input stands under, both directions each, and the killed depth-and-index coordinate restored |
| `tests/compiled_behaviour.rs` | lane C's mutant seats: the materialized artifacts, their verified provenance, and the constants read back as values |
| `tests/compiled-mutant/` | the materialized artifacts lane C compiles — checked in, provenance stated and verified |
| `tests/compile_refusals.rs` | the trybuild runner over the compile-fail fixtures |
| `tests/compile-fail/` | one fixture per discharged red twin |
| `tests/independent_identity_transcript.rs` | the independent transcript lane: a second encoder, written from the published specification, re-deriving the services' own identities |
| `tests/stamp_row_ceiling.rs` | the closed-register stamp's row ceiling, spent to its last position through the public export — the positive control whose reversal is the fixture beside it |

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
constant in `src/03_judge/byte_profile.rs` is re-stated to match the new shape,
in one place, on purpose, visible in the diff. Widening the reader until it
matches again — trimming whitespace, matching a prefix, adding a looser fallback
— buys a green run by making the judge cleverer, and a clever judge has stopped
reading the artifact and started agreeing with the renderer.

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
| `a-production-scope-guard-cannot-be-laundered.rs` | the seal holds on a guard the machine SHIPS, not only on roles a fixture stamps for itself: from outside the crate, `FrameVersion(position)` and `version.0` each refuse on their own |
| `a-consumed-image-rung-cannot-be-reused.rs` | the image ladder is affine in the types and not only in its prose: a rung handed to a road that takes it by value cannot be handed over a second time |
| `a-malformed-refusal-declaration-refuses.rs` | a shape word outside the machine's roster reaches the compiler as a refusal, not as a silent empty expansion |
| `a-closed-expansion-without-a-closure.rs` | the only constructor of a receipt is crate-internal, so the plan, the origin graph, the trace, the invalidation set, the explanation, and the closure are all seats a caller cannot omit — they are arguments to a function nobody outside can call |
| `a-rendering-taken-off-the-membership-only-draft.rs` | the frontage road is closed: the membership-only draft carries no rendering method at all |
| `a-closure-minted-without-proving.rs` | every field of the proof is private and `proved` is the only road to one, so a closure assembled field by field does not compile |
| `a-post-proof-join-outside-the-closure.rs` | joining the rendered units is crate-internal with one caller — the proof — so there is no public road to a joined tree outside it |
| `a-materialized-malformed-mutant.rs` | lane C's `MalformedRust` seat: the mutated artifact text, checked in with its provenance stated, does not compile |
| `a-cause-identity-cut-from-one-string.rs` | a cause identity is the pair of a family and a local key, so the retired string road — one literal cut by convention — does not compile |

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
  owner: testpak/src/03_judge/byte_profile.rs
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
    implementation target, trait path, how the implementation is written, the
    attributes it carries, every member it carries, the constructor paths its
    values are built through, body shape, cause rows in order, output
    cardinality, duplicate items, unplanned items — and catches every one of the
    nine mutations the ownership ledger records as structural, while the lawful
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

tooling-obligation: testpak.lane-c-compiles-the-mutants-it-owns
  claim: >
    Both mutations the ownership ledger records as compiled behaviour have a
    compiled seat: the malformed artifact is handed to `rustc` and fails to
    compile, the shape-altered artifact compiles and its trait constants read
    back as VALUES disagreeing with the shape this plane declares, and the
    lawful artifact compiled the same way reads back as declared. Every
    materialized artifact's provenance is re-derived on every run.
  owner: testpak/src/03_judge/type_contract.rs
  positive: testpak/tests/compiled_behaviour.rs
  method: compiled-behaviour
  activation: cargo test -p threadpak-testpak --test compiled_behaviour
  tooling-red: testpak/tests/compiled_behaviour.rs
  nonclaims: >
    It does not claim the LAWFUL artifact composes for an outside consumer; that
    is the consumer fixtures' claim, made from outside both participants, and a
    materialized text compiled inside the judge does not stand in for it. It does
    not claim the mutants are the only ones `rustc` would catch.
```

## The three lanes, and what each one may claim

**Lane A — the byte-profile scan** (`src/03_judge/byte_profile.rs`). Its claim
is exactly *the rendered text contains this exact declared textual form* and
never anything structural. It reads text `rustc` never touched, which is exactly
its value: it catches a renderer emitting the wrong bytes before those bytes are
ever offered to a compiler, and it costs a string search.

**Lane B — the structural read** (`src/03_judge/structural.rs`). Its claim is
exactly *the artifact DECLARES these implementations, of these traits, for this
target, written this way, carrying these members and no others* — what item is
this, what does it target, which contract does it realize, is it written
`unsafe`, negative, `default`, or generic, does it exist at all under some `cfg`,
are the cause rows the declared ones and are they built through the declared
constructors, did a member nobody planned come along, was one stated twice, did
an item nobody planned come along, was one emitted twice. It answers those by
parsing the text with a parser nobody here wrote, and it claims nothing about
whether any of it compiles.

**Lane C — compiled behaviour** (`xtask/fixtures/macro-consumer`,
`xtask/fixtures/renamed-consumer`, `tests/compiled_behaviour.rs`). `rustc`
compiles the artifact and the tests read its trait constants as VALUES. The
LAWFUL artifact's seat is the consumer fixtures, and it has to be: they compare
the derived implementation against hand-written twins from outside both
participants, and the renamed-consumer fixture reaches the machine only under
`tp`, so a renderer hardcoding the default binding would fail to compile there.
The MUTANTS' seats are this package's, because a mutant is this plane's own
damage and no participant is grading itself when the judge hands its own damaged
text to a compiler.

**Why the dumb reader stays dumb.** A cleverer reader has to decide what the text
MEANS, and the only way to decide that is to implement the same understanding the
renderer already has. Two implementations of one understanding, written by the
same hands against the same document, agree because they SHARE THE CHALLENGED
IMPLEMENTATION — not because either of them understands Rust. Correlated evidence
about a renderer is not independent of that renderer. Lanes B and C escape this
because their decoders — `syn` and `rustc` — are decoders nobody here wrote.

## The independent transcript lane

The services derive every plane identity from a transcript whose specification
they publish. A specification is a promise that somebody else can re-derive the
value; `tests/independent_identity_transcript.rs` is that somebody else.

**What it shares with the producer: nothing that encodes.** The length framing,
the field order, the domain-string grammar, the subject names, the role names,
the role slots, the anchoring discriminants, the profile stem, the profile
version, the generator name, and the generator schema version are all written out
in full in the test, from the specification, exactly as the planted-defect lane
writes out the declared order it judges against. Not one encoding function or
spelling is imported from `threadpak-macroc`.

**What it does share, deliberately: the digest.** Both sides call BLAKE3, pinned
exact at the same version with the same minimal feature set. A lane that
reimplemented the hash would be judging an arithmetic exercise; the hash is not
what is under judgement. What is under judgement is whether the specification says
enough — which is the question a reader of a published receipt actually has.

**What it derives.** A real captured-declaration identity, over a declaration the
services actually read, plus rooted and anchored transcripts across three
subjects. **And it rehearses its own reversal twice**: an encoder that drops the
content's length prefix must disagree, and a context assembled with the subject
and the role transposed must disagree. A match that could not fail would be
evidence of nothing.

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
the rows they are — the four constructor paths each row is built through, the two
seats of the stable identity it mints, and the spelling it states, in order. Across
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

**The planted mutations it must catch**, all nine recorded as structural in
`src/03_judge/type_contract.rs`, each landing on a different structural fact:

| Mutation | What lane B reports |
| --- | --- |
| `ImplTargetAltered` | the implementation targets a type the declaration did not name |
| `TraitPathWrong` | the implementation realizes a trait path the declaration did not name |
| `UnplannedOutputAdded` | the artifact declares more implementations than the declaration names |
| `DecoyInComment` | the selection order the artifact DECLARES is not the declared roster — the comment carrying the anchored bytes is not in the tree at all |
| `ImplMemberDuplicated` | one expected constant is stated twice, and the second reading is recorded rather than written over the first |
| `ImplMemberUnexpected` | the implementation carries a member the declaration did not name, described by what it is |
| `ConstructorPathAltered` | a cause row carries the declared values through one of the four constructors the declaration did not name |
| `ImplPostureAltered` | the implementation is written `unsafe`, where the declaration names none of the four postures |
| `MeaningBearingAttributeAdded` | the implementation carries an attribute that decides something — a doc comment decides nothing and is not one |

The decoy is the pair that makes the split visible in one line: lane A reports
`Conforms` on it and lane B reports the reversed order, and both are right, in
their own methods, about different questions.

## Mutation is the judge's job

The services no longer carry a road that renders a deliberately defective
artifact. A generator writing its own exam is rehearsed only against the defects
it already imagined. `src/03_judge/types.rs` declares fifteen mutations,
`src/03_judge/mutation.rs` damages lawful artifacts itself, and
`src/03_judge/type_contract.rs` records per mutation WHICH LANE owns catching
it — three files because a roster, a cut, and a ledger are three different
statements. Ownership is the seat of the CLAIM rather than a boast of
exclusivity: lane B notices a permuted order too and says nothing about it,
because that verdict is stated over lane A's method and belongs to lane A's row.

Every recorded ownership now has evidence under it. Lane A and lane B are held to
exactly their own rows in `tests/planted_defect.rs`; lane C's two rows are held
in `tests/compiled_behaviour.rs`, and the test that enumerates them fails if a
third mutation is ever recorded as compiled behaviour without a compiled seat.
