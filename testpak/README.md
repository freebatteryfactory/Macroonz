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
| `src/03_judge/` | `RenderVerdict` and the readers that produce it |
| `src/laws.rs` | the plane's own compile-time proof surface, sectioned by seat |
| `tests/planted_defect.rs` | the planted defective expansion, the proof the checker notices, and the rehearsed false alarm |
| `tests/compile_refusals.rs` | the trybuild runner over the compile-fail fixtures |
| `tests/compile-fail/` | one fixture per discharged red twin |

## Two lanes, and a verdict belongs to exactly one of them

**The fast lane is a string scan, and it is deliberately dumb.** The readers in
`src/03_judge/` find one declared construct in the rendered text and report what
they found. They read text `rustc` never touched, which is exactly their value:
they catch a renderer emitting the wrong bytes before those bytes are ever
offered to a compiler, and they cost a string search. A cleverer reader would
start agreeing with the renderer about what the text means, so the dumbness is
the design and not a stage on the way to something better.

**The authoritative lane compiles the rendered artifact and reads its trait
constants as values.** There, `rustc` is the independent decoder: it parses the
source by its own rules, with no anchor of ours anywhere in the path, and hands
back typed values rather than substrings. That lane is the consumer-fixture
parity tests at `xtask/fixtures/macro-consumer` — a crate owning neither the
machine nor the shell, applying the shell's derive and comparing the derived
`SHAPE`, `SELECTION_ORDER`, and `DECLARED_ORDER` against a hand-written twin,
value for value and position for position.

Neither lane subsumes the other and neither is a weaker version of the other.
**A verdict is method-specific**, exactly as the machine's evidence law
requires. Reporting a fast-lane verdict as if it came from the authoritative
lane — or the reverse — is the collapse this plane exists to refuse.

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
