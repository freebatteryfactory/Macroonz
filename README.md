# ThreadPak

[![qualify](https://github.com/freebatteryfactory/ThreadPak/actions/workflows/qualify.yml/badge.svg)](https://github.com/freebatteryfactory/ThreadPak/actions/workflows/qualify.yml)

ThreadPak is a host-neutral semantic machine written in safe Rust. Programs are typed
data, not text: a builder constructs typed declarations, and the machine validates,
seals, executes, and remembers them. Any frontend — Rust today, others later — enters
through the same public declaration path.

## The spine

```text
typed declarations
  → Semantic Form           normalized meaning, checked constructor
  → Execution Form          bounded operator graph, independently re-lowered
  → ProgramImage (.tpk)     binds both forms; the machine's inspectable bytecode
  → PakVM                   bounded synchronous execution, no ambient anything
  → runtime                 the Stitch: one observation in, next state and effect intents out
  → Bvisor                  the physical membrane: admission, capabilities, ports
  → accepted history (.tlog) durable append, crash recovery, authorized removal
```

Hosts live in other repositories and pin an exact ThreadPak revision. The machine never
knows which host is running it.

## Workspace

| Crate                             | Role                                                                          |
| --------------------------------- | ----------------------------------------------------------------------------- |
| `threadpak`                       | the machine — root package at the repository root                             |
| `macros/macroc`                   | the metaprogramming services — package `threadpak-macroc`                     |
| `macros/proc`                     | the Rust-facing expansion shell — package `threadpak-macros`                  |
| `testpak`                         | the qualification plane — package `threadpak-testpak`                         |
| `xtask`                           | repository law and the qualification road — `cargo xtask qualify`             |
| `xtask/fixtures/macro-consumer`   | the outside consumer fixture — package `threadpak-macro-consumer`             |
| `xtask/fixtures/renamed-consumer` | the renamed-dependency consumer fixture — `threadpak-renamed-consumer`        |

The repository root is itself the `threadpak` package; its `src/` carries the machine.
`macros/`, `testpak/`, and `xtask/` are unnumbered: first-class, but never on the
production dependency path. Hosts are unnumbered in the same sense and live one step
further out — in other repositories, as the spine above says — so this repository has
no `hosts/` directory, and it gains one only by an explicit decision. `macros/` is a plain
subsystem directory rather than a package: it holds the services (`macros/macroc`) and
the one Rust-facing expansion shell over them (`macros/proc`).
The metaprogramming edges run one way and inward —
`macros/proc` → `macros/macroc` → `threadpak` — and the machine depends on neither, an
absence the `no-core-tooling-edge` check enforces across every Cargo edge kind.

That check enforces a second absence in the same breath: the services never depend on
their frontend surfaces either, not even for tests. Composition is proven from outside
the participants instead — `xtask/fixtures/macro-consumer` depends on the machine and on
the expansion shell, exactly as an application would, and holds the tests that apply the
shell's derive and prove one derived implementation equal to a hand-written twin.

The same check enforces a third absence: **nothing depends on testpak**. testpak depends
inward on the machine and on the tooling, and production never depends on its judge.

## The band map

Numbered directories are dependency bands: band N imports only bands lower than N.
This order is compiler-verified — every cross-home public type reference points
downward. Homes materialize only when their specification content lands; no directory
exists empty.

| Band | Home        | Owns |
| ---- | ----------- | ---- |
| —    | `src/types.rs` | root calculus: generic composition shapes + root-admitted axes |
| —    | `src/laws.rs`  | the one compile-time proof surface, sectioned by home |
| 00   | refusal     | refusal envelope, families, handling, `ReasonId` |
| 01   | logic       | three-valued logic, truth tables, finality primitives |
| 02   | identity    | the six identity classes, minting, scope guards |
| 03   | value       | bounded values, closed algebra, absence |
| 04   | numeric     | exact numeric families, intervals, quantization |
| 05   | bounds      | budget classes carrying typed magnitudes |
| 06   | authority   | capability and `KeyScope` value algebra, meet, attenuation, protected resolution |
| 07   | bytes       | frame grammar, codecs, digest domains, byte roles, content regions |
| 08   | schema      | schema model, refinements, canonical profiles, migration |
| 09   | time        | clock observations (the tick), deadlines, chronology |
| 10   | history     | accepted history, append, recovery, removal, partitions, federation |
| 11   | navigation  | frames, axes, addresses, `Fix`, source closure |
| 12   | port        | port contract algebra and host-obligation shapes |
| 13   | declaration | the shared authoring algebra: fragments, linker, facets |
| 14   | semantic    | the judgment and Semantic Form |
| 15   | execution   | the operator register, lowering, agreement, Execution Form |
| 16   | image       | `ProgramImage`, `.tpk`, entrypoints, admission |
| 17   | pakvm       | the executor: values, arenas, the step machine |
| 18   | bvisor      | the boundary supervisor: Attempts, reservation, ports, observations |
| 19   | runtime     | the Turn, the Stitch, checkpoints, replay, reconciliation |
| 20   | derived     | DataBlocks, masks, materialization |
| 21   | application | composition, interfaces, Serve semantics |
| 22   | security    | protection machinery: sealed extents, shred, revocation distribution |
| 23   | evidence    | receipts, verification, denominators, the evidence graph |

## Phase

Architecture closure: the repository receives its complete specification before any
machine implementation. Implementation opens per home only on explicit authorization.

```yaml
phase: architecture-closure
toolchain: "1.97.1"
workspace_members:
  - macros/macroc
  - macros/proc
  - testpak
  - xtask
  - xtask/fixtures/macro-consumer
  - xtask/fixtures/renamed-consumer
```

## A declared magnitude becomes a machine fact by admission under a profile

`Limit` and `ConstLimit` stay open: any home, and any frontend outside this
crate, declares a limit family and states its own magnitude, and the compiler
checks nothing about the number. `AdmittedLimit` is what a declaration passes
through before a road treats it as a fact — opaque, family-tagged, profile-tagged,
and minted by a const road that establishes exactly one thing at compile time:
the magnitude stands under the ADMITTING PROFILE's ceiling.

**The machine owns the algebra; a profile owns the number.** `LimitAdmissionProfile`
is a plane's declared ceiling, and the machine declares no production profile of
its own. There is no single number that is right for every plane, and one seated
here for convenience would become the ceiling everything downstream inherits
without deciding anything. The generic roads are instantiated with the
downstream profile type, so the machine needs no edge to any plane that declares
one — the authoring plane's `AuthoringLimitProfile` lives in `macros/macroc`,
and the `no-core-tooling-edge` check is what keeps that direction one-way. Root
seats only a profile-independent algebra, bounds the representation itself
imposes, and narrowly named `cfg(test)` profiles its own laws stand under.

**Positivity is a second witness, not a stronger first one.** `AdmittedLimit`
proves the upper bound and nothing else, so a family declaring `MAX = 0` mints
it lawfully — the empty-only bound is a real bound and `Bounded::empty` under it
is an honest empty collection. `PositiveLimit` proves the same ceiling fact AND
that the family admits an item, and it is what `NonEmptyBounded::admitted_const`
demands, because that road promises an inhabitant no zero-maximum family can
supply. It proves the ceiling fact by CONTAINING the base witness rather than by
restating it: one field, minted by `AdmittedLimit::under_profile`, so the
comparison and its diagnostic have a single owner and the stronger witness
cannot quietly stop being the stronger form of the weaker one.

Each construction road states its own claim class, and the classes do not
substitute for one another:

| Road | Claim class | Evidence consumed |
| ---- | ----------- | ----------------- |
| `Bounded::empty` | no magnitude evidence at all | none; `L::MAX` is never read |
| `Bounded::from_array<N>` | local arity | const `N <= L::MAX`, proven at the call |
| `Bounded::admitted_const` | admitted family magnitude | `AdmittedLimit<L, P>` |
| `Bounded::admitted` | schema-minted runtime magnitude | `LimitWitness<L>` |
| `NonEmptyBounded::singleton` | local positivity | const `L::MAX >= 1`, proven at the call |
| `NonEmptyBounded::from_array<N>` | local arity and local positivity | const `N < L::MAX`, plus the separate first item |
| `NonEmptyBounded::admitted_const` | admitted family magnitude, and it must be inhabited | `PositiveLimit<L, P>` |
| `AdmittedPrefix::examined_completely` | admitted family magnitude, reported rather than refused | `PositiveLimit<L, P>` |
| `AdmittedPrefix::stopped_early` | admitted family magnitude, and it must be inhabited | `PositiveLimit<L, P>` |
| `NonEmptyBounded::admitted` | schema-minted runtime magnitude | `LimitWitness<L>` |

`examined_completely` is the one road that neither refuses nor claims
completeness. Refusing is right for material that is meaningless in part — a
trail, a membership, a ceiling — and wrong for a REPORT: the issues an over-bound
pass established are each true on their own, so refusing the body would leave a
caller with no findings at all. The road carries the prefix the admitted
magnitude holds and states, in the same value, how much it did not carry. It is
band 00's, because what it hands back is a refusal body and the posture that body
reports its own coverage with; the truncating mechanics stay here in the root
calculus as a crate-internal seam with that one consumer.

`stopped_early` is the halted road beside it, and it refuses where the other one
reports: `ReportTruncated` carries a seat for what it dropped and `EarlyStopped`
carries none, so a halted body handed more than its magnitude holds could only
drop the remainder silently. It has no caller today, because no scan in the
machine halts; it exists so that the first family whose examination honestly
stops early meets the same coupled seat every other family meets. Its honesty
ceiling is written at the road: the constructor couples the body to the posture
and does not prove that any external examination truly halted.

The two halves are one value because the pair is where the lie lives. A road
handing back a carry and a count hands a caller two things it may pair freely, so
the body one pass truncated could be reported under the count another pass
dropped — both halves individually honest, the pair false, and nothing in the
types noticing. `AdmittedPrefix` has private seats, no `into_parts`, and no owned
carry, so a report claiming it dropped seven issues is a report whose own body
dropped seven. Every collection-shaped refusal family in the machine and in the
services carries that one seat and reads it back through `issues()` and
`posture()`; `cargo xtask check`'s `collection-bodies-are-coupled` derives that
population from the sources rather than from a list anybody maintains.

The const-proven total roads read `L::MAX` bare by decision. `from_array([one,
two])` proves that two elements fit under a type-level maximum; it cannot prove
that maximum was a sensible declaration, because proving that needs a profile
and no profile is involved. Each proves exactly the local fact it needs and
claims no admission, which is why each stays total and has no refusal to return.

The ceiling on the claim: admission says nothing about whether a magnitude is the
right one for its domain. That is the owner's declaration, no road can check it,
and no witness pretends to.

## The closed-register stamp

`closed_register!` is the root's, and it is the one piece of root material that
is not a type. One declaration of a closed roster's rows writes the enum, the
roster constant in declared order, each row's position, each row's declared
stable name, and the prose a person is shown — five readings of one statement,
so the hand-kept pair they replace (a roster array beside a `match` returning
numbers) has no second place left to drift in. Band 13's authoring algebra, the
services crate's eight rosters, and this crate's own proof surface all stamp
with it; it declares no type, reaches no band's material, and belongs to no
band, which is why it sits at the root rather than in one of them. It is seated
in `lib.rs` — the root calculus's module surface — on the precedent
`scope_guard_version!` set in band 02's `mod.rs`.

The stamp carries a declared supply of positions and pairs each row of a
declaration with exactly one of them. There is no arithmetic in the expansion at
all, so nothing there can overflow, saturate, or disagree with a number written
elsewhere: the ceiling IS the length of the supply, and
`CLOSED_REGISTER_ROW_CEILING` is that same supply read out as a value, from the
arm the pairing walk spends. The supply is written in one place and the number
is written in none, which is why this paragraph names the constant rather than
the count. A row past the last position finds the supply spent and refuses with
the stamp's own sentence; the rule that matches that state matches the remaining
rows without recursing over them, so expansion stops at the first row past the
supply whatever the declaration's length and a long roster never dies against
the compiler's recursion limit instead. **The ceiling is this stamp
implementation's current authoring-profile ceiling, not a semantic cap on any
vocabulary**; raising it means extending the supply, once that is qualified, and
what a longer supply costs — an append within the `u8` a position is answered
as, or a new versioned encoding profile past it — is stated at the constant. The
claim is proved at the DECLARATION, from outside the crate that exports the
stamp: a roster spending the supply to its last position compiles with exact
positions, and one row past it refuses. The same boundary read through the
lifecycle facade arrives with the lifecycle specimen, because that facade does
not exist yet.

## Root calculus obligations

The root calculus (`src/types.rs` and the `closed_register!` stamp in
`src/lib.rs`) is the one home without a numbered directory; its obligations live
here.

```yaml
home: root
obligations:
  - id: root.cut-families-are-caller-supplied
    challenge_kind: compile-law
    green: laws.rs root::cut_families_are_caller_supplied
    red: owed-to-testpak
  - id: root.no-coordinate-forecloses-stale
    challenge_kind: compile-law
    green: laws.rs root::no_coordinate_forecloses_stale
    red: owed-to-testpak
  - id: root.completeness-domains-do-not-unify
    challenge_kind: compile-refusal
    green: laws.rs root::completeness_domains_do_not_unify
    red: owed-to-testpak
  - id: root.limit-families-do-not-unify
    challenge_kind: compile-refusal
    green: laws.rs root::limit_families_do_not_unify
    red: owed-to-testpak
  - id: root.dispatch-is-owner-refusal-generic
    challenge_kind: compile-law
    green: laws.rs root::dispatch_is_owner_refusal_generic
    red: owed-to-testpak
  - id: root.evidence-ref-identity-is-referent-and-version
    challenge_kind: compile-law
    green: laws.rs root::evidence_ref_identity_is_referent_and_version
    red: owed-to-testpak
  - id: root.bounded-construction-is-a-seam
    challenge_kind: compile-law
    green: laws.rs root::bounded_construction_is_a_seam
    red: testpak/tests/compile-fail/singleton-under-a-zero-maximum-family.rs
  - id: root.admission-precedes-a-trusted-magnitude
    challenge_kind: compile-refusal
    green: laws.rs root::admission_precedes_a_trusted_magnitude
    red: testpak/tests/compile-fail/a-magnitude-past-the-authoring-ceiling.rs
  - id: root.positivity-is-the-stronger-witness
    challenge_kind: compile-refusal
    green: laws.rs root::positivity_is_the_stronger_witness
    red: testpak/tests/compile-fail/a-zero-maximum-family-cannot-mint-a-positive-limit.rs
  - id: root.the-positive-witness-carries-the-admitted-one
    challenge_kind: compile-refusal
    green: laws.rs root::the_positive_witness_carries_the_admitted_one
    red: testpak/tests/compile-fail/a-past-ceiling-family-cannot-mint-a-positive-limit.rs
  - id: root.a-prefix-road-reports-what-it-did-not-carry
    challenge_kind: compile-refusal
    green: laws.rs root::a_prefix_road_reports_what_it_did_not_carry
    red: testpak/tests/compile-fail/a-remainder-married-to-another-body.rs
  - id: root.an-admission-does-not-cross-profiles
    challenge_kind: compile-refusal
    green: laws.rs root::an_admission_does_not_cross_profiles
    red: testpak/tests/compile-fail/a-cross-profile-admission.rs
  - id: root.reading-is-not-gaining
    challenge_kind: compile-law
    green: laws.rs root::reading_is_not_gaining
    red: owed-to-testpak
  - id: root.closure-bar-is-implementable
    challenge_kind: compile-law
    green: laws.rs root::closure_bar_is_implementable
    red: owed-to-testpak
  - id: root.a-stamped-roster-declares-its-own-ceiling
    challenge_kind: compile-refusal
    green: testpak/tests/stamp_row_ceiling.rs
    red: testpak/tests/compile-fail/a-roster-past-the-stamp-ceiling.rs
```

## License

ThreadPak is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
