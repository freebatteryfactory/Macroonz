# testpak — ThreadPak's testing harness

testpak is a property-based, descriptor-driven, mutation-pressured testing
harness. It is ThreadPak's own judge and a standalone product: any crate adopts
it with `cargo add threadpak-testpak`, and the library's inherited dependency
tree stays deliberately tiny — the manifest is the roster. Hand-written
descriptors are a lawful producer; the generation services are an optional
producer of the same inputs. No secret second language, pointed at ourselves.

## The dependency direction

testpak depends inward — on `threadpak`, `threadpak-macroc`, and
`threadpak-macros`, all as dev-dependencies reached only from `tests/` — and
nothing depends on testpak. Production never depends on its judge. It is
never published onto a production dependency path.

Outside-consumer parity is still a seat with no crate: no package in this
workspace applies the expansion shell's derive as an ordinary outside
consumer or reaches the machine under a renamed dependency binding, so no
lane here claims any of that.

## The uniform test model

Every test is a nextest-visible trial; what varies is where its content comes
from — a property, a descriptor row, or a fixture. Test kinds are data, not
directories: regression, boundary, metamorphic, malformed-input, smoke, and
their siblings are fields on a descriptor row, so one runner runs everything
and one report describes it. Every descriptor names the claim it serves — a
test that cannot say why it exists is a value nobody can build.

Trial names are stable across runs, spelled `module::path::trial_name`, and
carry no kind decoration.

## The instruments

| Instrument | Owns |
| --- | --- |
| `src/descriptor/` | the typed descriptor vocabulary — the public interface every producer writes into |
| `src/report/` | the harness record vocabulary: what ran, what was inspected, what was skipped and why |
| `src/oracle/` | the independence annex: reference decoding where bytes are the spec, and the vector parser |
| `src/runner/` | the nextest-protocol runner enumerating descriptor tables into trials |
| `src/properties/` | the algebraic property suites: roundtrip, idempotence, conservation, the metamorphic shapes |
| `src/muterprater/` | the proof-pressure engine: mutation, fuzz, and chaos lanes, survivor explanation, promotion |
| `src/fault/` | refusing adapters — typed values implementing port contracts — and campaign shapes |
| `tests/` | executable entry points, compile-refusal fixtures, compiled-behaviour seats |
| `corpus/` | compressed seed-packs for warm-start fuzzing |

The instruments are order-free peers over one vocabulary — no instrument
imports another, so no instrument carries a number. The one numbered seat,
`src/03_judge/`, is the pre-redesign machinery the oracle and muterprater
absorb.

## Verdicts are typed refusals

A failed check is a typed refusal value carrying its evidence and its source
location — the harness fails the way the machine refuses. That is the
instruments' law: the runner reports failure as a returned value, and no
instrument fails by panicking. The standing seat still asserts the old way;
its absorption is what retires that. A panic from the subject under test is contained at the trial
boundary and converted into a verdict with its location: an overflow check
that fires is the machine's own bounds working, and the harness records it as
the finding it is. The depot is authored specification; an observed panic is
runtime evidence; no failed execution mutates specification.

## Reports, coverage, and the parity receipts

The harness's records are reports. The denominator of a run is the descriptor
table itself, so claim coverage — did every declared claim, hostile case, and
mutation row get exercised — is computed from reports, never hand-counted.
"Receipt" is the machine's word: only the parity records the machine consumes
before trusting an optimized component carry it.

## Muterprater

Muterprater plans which proof pressure is worth running, runs it under budget,
explains every survivor, and promotes only candidates that kill a real mutant
or pin a named invariant. No oracle, no promotion; no killed mutant or new
proof delta, no promotion; no report of the run, no trust in its result.
Fuzzing is structure-aware over `arbitrary`; a minimized find is promoted into
a regression descriptor row carrying its reproduction seed. Compiled mutation
runs through `cargo-mutants`, retained for high-assurance passes.

## Tests gate benches

A failing operation is never benchmarked. The vacuity gate is itself a trial:
a reference implementation and a deliberately worse one are timed across input
sizes, and the gate asserts the growth classes separate — its sample counts
and thresholds are declared beside it. Stated honestly, the gate is robust,
not deterministic: it asserts a growth class, never a time. Bench output is a
human report, never fails a build, and the bench home arrives with the first
bench — the manifest admits its tooling then.

## The third-party mechanisms this package asks for

Which version and which feature cut is the workspace's one decision, in the
root manifest's `[workspace.dependencies]` table. What this package owes is
the reason it reaches for each mechanism.

**`arbitrary`, for generation** — admitted with the instruments that consume
it: the shared vocabulary for structure-aware input generation, derivable for
closed algebraic types, and the same vocabulary a coverage-guided fuzzer
consumes.

**`syn`, for the structural oracle.** Whether an artifact DECLARES an
implementation, what it targets, and whether an anchored constant is a member
of it are not questions about bytes. The text goes to a decoder that owes this
repository nothing: `parsing`, without which there is no text-to-tree road,
and `full`, without which items and their associated constants are not in the
tree. The lane reads, never writes, and never runs inside a macro.

**And that reason settles less than it sounds like.** What a manifest ASKS
FOR, what the resolved graph HOLDS, and what one compiled unit is HANDED are
three different facts. The paragraph above states the first; `deny.toml`
settles the second against the graph itself; the third has no seat in this
tree, so no sentence here claims it.

**`blake3`, for the independent transcript oracle — a dev-side mechanism,
reached from `tests/`.** That lane re-derives a
published identity from its published specification, writing out every
encoding decision itself and importing none. The digest is the one thing it
shares with the producer, deliberately: whether the specification says enough
for somebody else to re-derive the value is what is under judgement.

Dev-side mechanisms — `trybuild` for compile refusals, and the bench and
snapshot tooling admitted at their first real use — never reach an adopter's
tree.

## Extending the harness

Consumers extend through data and functions, never through this crate's
source: new populations, suites, property functions, and fault adapters all
flow through the descriptor and report vocabularies. The kind roster is
sealed — a new kind is a law change; a new population is a Tuesday.
