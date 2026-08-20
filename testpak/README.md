# testpak — ThreadPak's testing harness

testpak is a property-based, descriptor-driven, mutation-pressured testing
harness. It is ThreadPak's own judge and a standalone product by
architecture now: workspace and path use during construction, and `publish`
flips at release — at which point any crate adopts it with
`cargo add threadpak-testpak`. The library's inherited dependency
tree stays deliberately tiny — the manifest is the roster. Hand-written
descriptors are a lawful producer; the generation services are an optional
producer of the same inputs. No secret second language, pointed at ourselves.

## The spine — the harness in seven nouns

Declared trials become selected executions become evidence. Each arrow is an
honest constructor, and the products say what a value actually requires:

```text
Row + executable attachment        →  Binding
Bindings                           →  Table       (complete; refuses duplicates)
Binding × Invocation               →  TrialReport   (run_one)
Table × Selection × Invocation     →  RunReport     (run_all)
RunReport × RunReport              →  ReportDiff
```

Both engine arrows are total. A row commits to its canonical bytes when it is
built, so nothing at run time can fail to name a row's revision, and a selection
that matched nothing is a fact the report STATES rather than a reason to state
no report at all. A run carries what it expects its selection to match — at
least one row, unless a caller declares otherwise and says why — and the seat
reading turns an unmet expectation into a failing seat rather than a green one
that ran nothing.

Row is pure descriptor data — it carries a check reference, never a callable.
Binding pairs one Row with one callable through the executable attachment,
whose constructor structurally verifies the row's references match. Table is
the complete set of Bindings — the world; its constructor refuses duplicate
trial identities. Selection is what one invocation chooses FROM that world —
the world itself never shrinks; what a run hands the engine is that selection
joined to what the run expects it to match. TrialReport is one execution;
RunReport is complete-table-plus-selection accounting; ReportDiff is a pure
comparison of two reports.

Three vocabularies ride beside the spine without being spine nouns: the
Invocation (both engine calls take it; Selection remains its own argument),
the TableView (the one sealed read surface an authored table and a staged
candidate view share), and the identity family (`src/report/` owns it; the
derivation SUBSTRATE — the one domain-separated profile every identity
kind derives through — is root-admitted at `src/identity.rs` as generic
mechanism, so no instrument grows an identity island). Six
seats feed the spine: `generate/` (populations into rows), the depots and
the error bank (facts into rows and diagnostics), `fault/` (adversity values
into campaigns), `corpus/` (warm starts into the fuzz lane), the benches
(their own row family and executor), and the dependency bill.

## The dependency direction

testpak depends inward — its manifest names its own library bill and no
participant — and nothing depends on testpak. Production never depends on
its judge. It is never published onto a production dependency path.

Outside-consumer parity has its crate: `consumer/` is a workspace member
that renames BOTH crates (`tp` for the machine, `harness` for this one),
applies the expansion shell's derive to a documented public family of its
own, and reads the compiled surface back beside a hand-realized twin. It
stays deliberately ordinary — no judge machinery inside it, consumer-shaped
evidence during construction, and a packaged true-outsider proof at the
blessing-day check.

## The uniform test model

Every test is a nextest-visible trial; what varies is where its content comes
from — a property, a descriptor row, or a fixture. Execution is answered by
the check reference and the subject route — sealed by being this crate's
types, so a new mechanism is structurally a law change. A row's roles and
tags are open, namespaced classification: regression, boundary, metamorphic,
malformed-input, smoke, and their siblings are labels on a descriptor row,
not directories and not mechanisms, so one runner runs everything and one
report describes it. A row carries exactly one execution suite — the
aggregate seat it runs under by default — so no row ever runs through two
default aggregates. Every descriptor names the claim it serves — a
test that cannot say why it exists is a value nobody can build.

A trial's path-spelled name is its SITE — where it lives, for humans and
filters — never its identity: semantic identity survives file and module
moves, and reports join both. Names carry no kind decoration.

The whole loop runs through the one pure engine: a declaration is authored
through a door and its rows are projected into the test target (or a hand
writes rows directly — equally lawful); rows land in tables; the stamp reads
them into named lenses and one aggregate seat per execution suite; nextest
runs the stamped names natively while each aggregate seat calls the engine
in-process; claim coverage computes over the authored reports; mutation
pressure runs outside the wall, where a non-kill stays inconclusive — the
word survivor is earned only where activation is observed, and every earned
one is explained; a candidate
that closes a gap executes in staging against the complete world, and only a
proposal a human admits joins the authored table — the denominator grows and
says so. Every semantic-trial run anywhere in that loop is the same engine
call with a differently selected subset of the one complete table.

## The instruments

| Instrument | Owns |
| --- | --- |
| `src/descriptor/` | the typed descriptor vocabulary — the public interface every producer writes into |
| `src/report/` | the harness record vocabulary: what ran, what was inspected, what was skipped and why |
| `src/oracle/` | the independence annex: reference decoding where bytes are the spec, and the vector parser |
| `src/runner/` | the pure execution engine: descriptor table and typed invocation in, typed reports out |
| `src/properties/` | the algebraic property suites: roundtrip, idempotence, conservation, the metamorphic shapes |
| `src/muterprater/` | the proof-pressure engine: mutation, fuzz, and chaos lanes, survivor explanation, the proposal road |
| `src/fault/` | refusing adapters — typed values implementing port contracts — and campaign shapes |
| `src/generate/` | the generation contract: typed dispositions, generation and reduction plans, the shared sequence driver, fingerprint-preserving minimization |
| `src/depot/` | the harness's own fact bank: doc-commented Rust-const rows — operator families, swap-pair populations, admitted replay capsules |
| `corpus/` | compressed seed-packs for warm-start fuzzing |

The instruments form an honest acyclic graph over one vocabulary — the
descriptor vocabulary at the bottom, report beside it, the runner using
both, generate serving properties and muterprater alike — and nobody forms
a cycle, so no instrument carries a number. Nothing is re-exported at the
crate root either: a vocabulary is spelled through the home that owns it, so
a call site says which instrument made the claim.

## Verdicts are typed refusals

A failed check is a typed refusal value carrying its evidence and its source
location — the harness fails the way the machine refuses. That is the
instruments' law: the runner reports failure as a returned value, and no
instrument fails by panicking. A panic from the subject under test is
CAUGHT at the trial boundary and converted into a verdict with its location —
catch is the harness's own word for this; containment belongs to another
band's vocabulary and is never used here: an overflow check
that fires is the machine's own bounds working, and the harness records it as
the finding it is. The depot is authored specification; an observed panic is
runtime evidence; no failed execution mutates specification.

## Reports, coverage, and the parity receipts

The harness's records are reports. The denominator of a run is the descriptor
table itself, so claim coverage — did every declared claim, hostile case, and
mutation row get exercised — is computed from reports, never hand-counted;
it admits authored-posture reports only, so a staged candidate run never
enters coverage.
"Receipt" is the machine's word: only the parity records the machine consumes
before trusting an optimized component carry it.

## Muterprater

Muterprater plans which proof pressure is worth running, runs it under budget,
explains every survivor, and SUGGESTS — a candidate becomes authored only
through a proposal a human admits. A proposal earns admission on one of three
structural grounds: it kills a real mutant, pins a named claim, or discharges
a claim declared owed; runtime evidence never writes authored specification
by itself. No report of the run, no trust in its result.
Fuzzing is structure-aware over `arbitrary`; a minimized find rides the same
proposal road, carrying its replay capsule. Compiled mutation
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
implementation, what it targets, and whether a constant is a member of it are
not questions about bytes. The text goes to a decoder that owes this
repository nothing: `parsing`, without which there is no text-to-tree road,
and `full`, without which items and their associated constants are not in the
tree. The lane reads, never writes, and never runs inside a macro — and its
home is the challenge side: structural decoding of this repository's rendered
artifacts is qualification-era work, while the oracle library owns vocabulary
and comparison. The decoder stands isolated in one oracle file today, and the
manifest carries the opening condition exactly: the move happens when a
challenge-side caller exists to call it, and that same move retires this from
the library's dependency table. A decoder relocated ahead of its caller would
be unreachable code wearing the shape of a move.

**And that reason settles less than it sounds like.** What a manifest ASKS
FOR, what the resolved graph HOLDS, and what one compiled unit is HANDED are
three different facts. The paragraph above states the first; `deny.toml`
settles the second against the graph itself; the third has no seat in this
tree, so no sentence here claims it.

**`blake3`, the family identity substrate — a library mechanism.** The
harness is a citizen of the workspace identity family rather than an
identity island: the generated-support schema identity, the revision
identities, failure fingerprints, replay-capsule identities, proposal
identities, the execution key and its rerun-cache keys, and seed-pack
content addresses are all content addresses under one domain-separated,
versioned profile — `derive_key` gives every identity kind its own domain
tag — and every hand-rolled checksum, deep compare, and second identity
mechanism the harness would otherwise invent dies against it. The
independent transcript lane keeps its own discipline on top: it re-derives
a published identity from its published specification, writing out every
encoding decision itself and importing none — the digest is the one thing
it shares with the producer, deliberately, because whether the
specification says enough for somebody else to re-derive the value is what
is under judgement. The adopter's standing bill is two library dependencies,
`arbitrary` and `blake3` — blake3's build machinery is a disclosed mechanism
of the admitted dependency, not a purity claim. Today an adopter inherits
`syn` beside them, because the decoder above has not moved yet; the manifest
states that as the present tense rather than as an aspiration.

Dev-side mechanisms admitted at their first real use — bench tooling at the
first bench surface — never reach an adopter's tree.

## Extending the harness

Consumers extend through data and functions, never through this crate's
source: new populations, suites, property functions, and fault adapters all
flow through the descriptor and report vocabularies. Mechanisms are sealed —
a new mechanism is a law change; roles, tags, and populations are open — a
new population is a Tuesday.

The tri-use doctrine: every stateless semantic function is legal in three
worlds without adaptation — at compile time validating declarations inside
the generator, at runtime validating live data, and here as a property
subject. One validator, three uses, zero glue — doctrine, not coincidence.
