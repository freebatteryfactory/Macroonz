# testpak — the qualification plane

testpak orchestrates hostile execution against the machine and its tooling, and
carries the denominators every verdict is stated over. **A verdict here is
always claim-specific and method-specific**: "the permuted rendering was
rejected by the projection check over these two declared orders" is a verdict;
"the derive works" is not one.

The dependency direction is the whole point. testpak depends inward — on
`threadpak`, on `threadpak-macroc`, and on `threadpak-macros` — and **nothing
depends on testpak**. Production never depends on its judge, and the
`no-core-tooling-edge` check enforces that absence at the root manifest under
every Cargo edge kind, with planted reversals proving the check can fail.

It is a workspace member and never published: it is dev-and-qualification
material, first-class, and never on the production dependency path.

## What lives here

| Path | What it carries |
| --- | --- |
| `src/lib.rs` | the judges: readers that check a rendered projection against an independently stated declared order |
| `tests/planted_defect.rs` | the planted defective expansion, and the proof that the checker notices |
| `tests/compile_refusals.rs` | the trybuild runner over the compile-fail fixtures |
| `tests/compile-fail/` | one fixture per owed red twin |

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

## Owed

The great majority of the red twins the green laws name are still owed. The
`red: owed-to-testpak` rows in every home README are the ledger; this directory
is where they land, and it is nearly empty. Also owed: the hostile corpora
(permutation, determinism, and ambient-pathway hostiles the services' own docs
name), and the mutation machinery. Sequenced, not deferred.
