# testpak — ThreadPak's testing harness

testpak is a property-based, descriptor-driven, mutation-pressured testing harness. It is ThreadPak's own judge and a standalone product by architecture: workspace and path use during construction do not make it part of the machine, and publication is the release mechanism through which another crate adopts it with `cargo add threadpak-testpak`.

The library's inherited dependency tree stays deliberately tiny — the manifest is the roster. Hand-written descriptors are a lawful producer; the generation services are an optional producer of the same inputs. No secret second language, pointed at ourselves.

## The spine — the harness in seven nouns

Declared trials become selected executions become evidence. Each arrow is an honest constructor, and the products say what a value actually requires:

```text
Row + executable attachment        →  Binding
Bindings                           →  Table       (complete; refuses duplicates)
Binding × Invocation               →  TrialReport   (run_one)
Table × Selection × Invocation     →  RunReport     (run_all)
RunReport × RunReport              →  ReportDiff
```

Both engine arrows carry a typed report posture for every table, selection, and accounting state the runner owns. A row commits to its canonical bytes when it is built, so nothing at run time can fail to name a row's revision, and a selection that matched nothing is a fact the report STATES rather than a reason to state no report at all. A run carries what it expects its selection to match — at least one row, unless a caller declares otherwise and says why — and the seat reading turns an unmet expectation into a failing seat rather than a green one that ran nothing.

Row is pure descriptor data — it carries a check reference, never a callable. Binding pairs one Row with one callable through the executable attachment, whose constructor structurally verifies the row's references match. Table is the complete set of Bindings — the world; its constructor refuses duplicate trial identities. Selection is what one invocation chooses FROM that world — the world itself never shrinks; what a run hands the engine is that selection joined to what the run expects it to match. TrialReport is one execution; RunReport is complete-table-plus-selection accounting; ReportDiff is a pure comparison of two reports.

Three vocabularies ride beside the spine without being spine nouns: the Invocation (both engine calls take it; Selection remains its own argument), the TableView (the one sealed read surface an authored table and a staged candidate view share), and the identity family (`src/report/` owns it; the derivation SUBSTRATE — the one domain-separated profile every identity kind derives through — is root-admitted at `src/identity.rs` as generic mechanism, so no instrument grows an identity island). Six seats feed the spine: `generate/` (populations into rows), the depots and the error bank (facts into rows and diagnostics), `fault/` (typed adversity values into selected schedules), `src/corpus/` (content-addressed warm starts into generation), the benches (their own row family and executor), and the dependency bill.

## The dependency direction

testpak's production library is standalone: `arbitrary`, `blake3`, and no crate of this workspace. Its qualification surface depends inward on `macroonz`, `threadpak-macroc`, and `threadpak-macros`, and uses `syn` for structural challenges, all as dev-dependencies reached from `tests/`, where its own qualification runs.

Nothing in production depends on testpak. Production never depends on its judge, and it is never published onto a production dependency path. The consumer crate dev-depends on it under a rename, which is a road that proves the public surface rather than one that ships.

Outside-consumer parity has its crate: `consumer/` is a workspace member that renames BOTH crates (`tp` for the machine, `harness` for this one), applies the expansion shell's derive to a documented public family of its own, and reads the compiled surface back beside a hand-realized twin. It stays deliberately ordinary — no judge machinery inside it and only consumer-shaped evidence. A packaged true-outsider proof is required before the stronger outsider claim is admitted.

## The uniform test model

Every test is a nextest-visible trial; what varies is where its content comes from — a property, a descriptor row, or a fixture. Execution is answered by the check reference and the subject route — sealed by being this crate's types, so a new mechanism is structurally a law change. A row's roles and tags are open, namespaced classification: regression, boundary, metamorphic, malformed-input, smoke, and their siblings are labels on a descriptor row, not directories and not mechanisms, so one runner runs everything and one report describes it. A row carries exactly one execution suite — the aggregate seat it runs under by default — so no row ever runs through two default aggregates. Every descriptor names the claim it serves — a test that cannot say why it exists is a value nobody can build.

A trial's path-spelled name is its SITE — where it lives, for humans and filters — never its identity: semantic identity survives file and module moves, and reports join both. Names carry no kind decoration.

The whole loop runs through the one execution engine: a declaration is authored through a door and its rows are projected into the test target (or a hand writes rows directly — equally lawful); rows land in tables; the stamp reads them into named lenses and one aggregate seat per execution suite; nextest runs the stamped names natively while each aggregate seat calls the engine in-process; claim coverage computes over the authored reports; mutation pressure runs outside the wall, where a non-kill stays inconclusive — the word survivor is earned only where activation is observed, and every earned one is explained; a candidate that closes a gap executes in staging against the complete world, and only a proposal a human admits joins the authored table — the denominator grows and says so. Every semantic-trial run anywhere in that loop is the same engine call with a differently selected subset of the one complete table.

## The instruments

| Instrument | Owns |
| --- | --- |
| `src/clock/` | TestPak's caller-declared wall source, admitted ticks, checked elapsed readings, unavailability, and typed measurement failure |
| `src/descriptor/` | the typed descriptor vocabulary — the public interface every producer writes into |
| `src/report/` | the harness record vocabulary: what ran, what was inspected, what was skipped and why |
| `src/oracle/` | the independence annex: independently authored structural, compiled-value, transcript, and vector comparisons |
| `src/runner/` | the execution engine: descriptor table and typed invocation in, typed reports out |
| `src/bench/` | benchmark rows, callable binding, primary work judgment, secondary caller-clock readings, and complete benchmark reports |
| `src/properties/` | the algebraic property suites: roundtrip, idempotence, conservation, the metamorphic shapes |
| `src/muterprater/` | mutation policy, exact production/evaluation pairing, compiled and interpreted evidence, proof planning, survivor explanation, and proposals |
| `src/fault/` | adopter-owned typed behavior/postcondition pairs, named campaigns, validated selection, and sequence injection |
| `src/corpus/` | content-addressed seed packs, typed reading, and exact supplied-input warm starts |
| `src/generate/` | the generation contract: typed dispositions, generation and reduction plans, the shared sequence driver, fingerprint-preserving minimization |
| `src/depot/` | the harness's own fact bank: doc-commented Rust-const rows — operator families, swap-pair populations, admitted replay capsules |
| `tests/` | executable entry points, compile-refusal fixtures, refusal-artifact reading seats |

The instruments form an honest acyclic graph over one vocabulary — clock owns wall measurement without importing a semantic instrument, descriptor and report own their facts, the runner uses those homes, and generate serves properties and muterprater alike — and nobody forms a cycle, so no instrument carries a number. Nothing is re-exported at the crate root either: a vocabulary is spelled through the home that owns it, so a call site says which instrument made the claim.

## Verdicts are typed refusals

A failed check is a typed refusal value carrying its evidence and its source location — the harness fails the way the machine refuses. The runner represents each failure it handles as a returned value, and its README owns the exact effect and unwind ceiling. A subject panic that crosses the guarded callable boundary becomes a verdict with its location — catch is the harness's own word for this; containment belongs to another band's vocabulary and is never used here. An overflow check that fires is the machine's own bounds working, and the harness records it as the finding it is. The depot is authored specification; an observed panic is runtime evidence; no failed execution mutates specification.

## Reports, coverage, and the parity receipts

The harness's records are reports. The denominator of a run is the descriptor table itself, so claim coverage — did every declared claim, hostile case, and mutation row get exercised — is computed from reports, never hand-counted; it admits authored-posture reports only, so a staged candidate run never enters coverage.

"Receipt" is the machine's word: only the parity records the machine consumes before trusting an optimized component carry it.

## Muterprater

Muterprater keeps production free of mutation directives. No-mutation evaluation delegates to production, while active evaluation receives the exact point and alternative resolved from a surface-issued selection and returns the separately produced alternative meaning. Point-catalog posture and parity qualification remain independent: a point-free surface is lawful but cannot mint an active selection, and no interpreted result becomes trusted until generic compiled suite bite, exact-input no-mutation parity, and separately compiled selection-scoped projection pressure all exist under their own evidence ceilings.

Muterprater plans which proof pressure is worth running, records what each backend can actually establish, explains every earned survivor, and suggests candidates. A candidate becomes authored only through a proposal a human admits; runtime evidence never writes authored specification by itself.

Fuzzing is structure-aware over `arbitrary`; a content-addressed seed pack supplies exact warm-start bytes but is never a judgment population or replay account. A subject finding reaches the ordinary run report and fingerprint; it reaches a proposal only through a separately lawful proposal ground. Compiled mutation runs through `cargo-mutants`, retained for high-assurance passes.

## Tests gate benches

A failing correctness preflight withholds every benchmark callable. An owner-bound work judge then reads measured and deliberately worse work-count curves under the row's exact formula, complexity claim, budgets, and ratio: the worse curve must refuse, the measured curve must satisfy, and the declared gap must distinguish them before the host reads its clock. Wall duration is a secondary report reading and cannot repair a failed semantic stage; a renderer receives only the finished immutable report and owns no verdict.

## The third-party mechanisms this package asks for

Which version and which feature cut is the workspace's one decision, in the root manifest's `[workspace.dependencies]` table. What this package owes is the reason it reaches for each mechanism.

**`arbitrary`, for generation** — admitted with the instruments that consume it: the shared vocabulary for structure-aware input generation — the trait alone, with no derive feature admitted — and the same vocabulary a coverage-guided fuzzer consumes.

**`syn`, for the structural challenge.** Whether an artifact DECLARES an implementation, what it targets, and whether a constant is a member of it are not questions about bytes. The refusal-artifact test sends real macroc output to a decoder that owes the producer nothing: `parsing`, without which there is no text-to-tree road, and `full`, without which items and their associated constants are not in the tree. The decoder reads and maps the foreign tree into the oracle's public vocabulary; it never writes and never runs inside a macro. `syn` is therefore a dev-dependency owned by the challenge side, while the production oracle library owns only vocabulary and typed comparison.

**And that reason settles less than it sounds like.** What a manifest ASKS FOR, what the resolved graph HOLDS, and what one compiled unit is HANDED are three different facts. The paragraph above states the first; `deny.toml` settles the second against the graph itself; the third has no seat in this tree, so no sentence here claims it.

**`blake3`, the family identity substrate — a library mechanism.** The harness is a citizen of the workspace identity family rather than an identity island: the generated-support schema identity, the revision identities, failure fingerprints, replay-capsule identities, proposal identities, the execution key and its rerun-cache keys, and seed-pack content addresses all use one `derive_key` mechanism and one shared profile stem; domain separation and compatibility position are owned independently by each preimage family through its own domain tag, and every hand-rolled checksum, deep compare, and second identity mechanism the harness would otherwise invent dies against it.

- The independent transcript lanes keep their own discipline on top: each authors the published family, position, subject, role, anchoring, content, and context facts without importing them from the producer, then the generic oracle frames and derives those facts through one implementation. The digest is the one mechanism shared with the producer, deliberately, because whether the specification says enough for somebody else to re-derive the value is what is under judgement. `blake3`'s build machinery is a disclosed mechanism of the admitted dependency, not a purity claim.

Dev-side mechanisms — `trybuild` for compile refusals, and the bench and snapshot tooling admitted at their first real use — never reach an adopter's tree.

## Extending the harness

Consumers extend through data and functions, never through this crate's source: new populations, suites, property functions, and fault adapters all flow through the descriptor and report vocabularies. Mechanisms are sealed — a new mechanism is a law change; roles, tags, and populations are open — a new population is a Tuesday.

The tri-use doctrine: every stateless semantic function is legal in three worlds without adaptation — at compile time validating declarations inside the generator, at runtime validating live data, and here as a property subject. One validator, three uses, zero glue — doctrine, not coincidence.
