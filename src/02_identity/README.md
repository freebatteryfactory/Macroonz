# 02_identity — the six-class calculus

Band 02. Imports band 00 (the first cross-band import in the machine: order
comparison refuses through a refusal family). Owns the identity *shapes* and
their laws; every concrete identity lives with its owner home and instantiates
them. One class law, not one register document.

## The two-column law

Every identity declares two independent columns — its class (which question it
answers) and its creation law (how an instance is minted) — as machine-readable
constants (`IdentityRole`). The class never implies the creation law: Class-D
rows differ in creation law while answering the same kind of question. An
identity is designed by classification plus one named minting rule, never by
taste.

## A declaration becomes a machine fact by admission

`IdentityRole` stays open and derivable, and nothing in the type system makes a
declared pair of columns coherent. `AdmittedIdentityRole<T>` is the join, opaque
and constructor-free, and `AdmittedIdentityColumns` — the road that turns the two
columns into a value that travels — is reachable only from it. The projection is
what erases the role, so the projection is what the name follows: every value of
it was read off a witness, and it is named for that rather than for the
declaration it came from. The witness keeps its own name and its `T`, because a
witness is exactly a statement about one role.

The join the home's own declarations support is narrow and is stated narrowly:
four of the seven creation laws name a class in their own declaration
(`CreationLaw::declared_class`), and a role declaring one of those under a
different class refuses. This reads creation → class and only there; the
two-column law reads class → creation and stands untouched, which is exactly why
the three class-open creation laws admit under any class.

Admission establishes nothing about the derived-seat law's two seats — those are
facts about a deployment's design, not about a pair of constants — and nothing
about whether a concrete minter follows the creation law it declared. That claim
is behavioral, it is owed, and it opens when minters exist.

## The six classes

| Class | Shape | Question | Guard |
| --- | --- | --- | --- |
| A | `Commitment<Domain>` | same meaning? | domain-tagged in type and preimage; keyed-when-protected (keying lands with `KeyScope`) |
| B | `ByteIdentity<Role>` | same bytes? | never substitutable for A |
| C | `AuthorityPosition<Scope>` | what position in one authority's order? | no `Ord`; `try_cmp_same_scope` only; cross-scope order is a cut vector; scope may be a tuple |
| D | `Occurrence<Role>` | which happening? | readers parse no structure; derived-seat law gates derived minting |
| E | `TypedRef<To>` | which referent, at which version? | equality is exactly that pair; availability/integrity ride alongside |
| F | `ApplicationScope` (trait) | application-composed scope | machine mints none; normal form authored at the authority home |

## The derived-seat law

Derived minting earns two seats or does not exist: a named consumer of
convergence, and preimage custody. Where either seat is empty the identity is
fresh — an absent preimage is a design answer that makes the computed-identity
attack class unrepresentable rather than defended. A/B stay computable by
design under their own guards.

## Seams and envelopes

Internal seams speak refusal family bodies (`OrderComparison` here); the
universal envelope is the publication form, minted only where reasons are
registered. Canonical refusal ≠ released refusal — one fact, two projections.

## The declarative stamp

This home owns the Class-C shape and its guard law, so this home stamps it:
`scope_guard_version!` writes the version newtype over `AuthorityPosition<Scope>`
from one explicit typed invocation. The caller states the docs, the visibility,
the type name, and the scope type; the stamp infers nothing and derives no
ordering — no `Ord`, no `PartialOrd`, one `try_cmp_same_scope` forwarding to the
machinery this home already owns. Rust exports `macro_rules!` at the crate root;
that is Rust's macro namespacing rule and not a root admission of a semantic
noun, because the stamp declares no type of its own.

The stamped position is private, and the stamp emits one road in and none out:
`positioned` reads a position the caller already holds under this role, and no
accessor hands it back. A role whose representation could be taken out and
re-entered under another role would be a label rather than a wall, so both
directions refuse — and the refusal is proven over ONE scope type, where nothing
about the scope is helping.

**The stamp writes into a module of its own, and that is what makes "none out" a
fact rather than an audit.** Rust's privacy is module-scoped: a `macro_rules!`
expansion lands in the invoking module, so a stamp that wrote the newtype
straight into a home's `types.rs` put the seat within reach of every other type
and implementation in that file. The invocation now names the module — `pub
struct FrameVersion over ReferenceFrameId, seated in mod frame_version;` — and
the stamp emits the guard into it and re-exports the type out. The module's
entire content is the transcriber's output, because nothing hand-written can be
added to a module that exists only inside an expansion. So the complete set of
roads out of a stamped guard is the set the stamp writes, and `rustc` is what
establishes it: from the invoking module `version.0` is `E0616` and
`FrameVersion(position)` is `E0423`.

The module name is the caller's argument because `macro_rules!` cannot build an
identifier from another identifier on stable and this repository carries no
dependency that can. It is `snake_case` because a module named after its type
trips `non_snake_case`, which the lint wall denies — no attribute suppresses
anything, and two stamps naming one module in one file collide as a duplicate
definition.

The caller-coordinate re-export is the guard's canonical exported spelling.
The caller's visibility appears there exactly once. The front grammar also
transports the same reach one module into the private generated child: private
and `self` become `super`, `super` gains one `super` segment, and absolute paths
and `pub` keep their coordinate-independent meaning. The type and both methods
carry that transported reach. Code already inside the authorized scope may name
the generated child path, but no generated item is broad enough for a wider
same-coordinate re-export, type alias, or public signature.

The admitted direct-token population is Rust's private and public forms,
`pub(self)`, `pub(super)`, `pub(crate)`, the equivalent `pub(in self)`,
`pub(in super)`, and `pub(in crate)` forms, relative `super` chains, absolute
`crate` paths, and an outer macro's coordinate-invariant `$crate` path. A whole
visibility forwarded as a captured `$vis:vis` is opaque: the stamp refuses it
instead of treating an unknown reach as crate-wide.

Visibility selection does not duplicate the guard. One transcriber owns the
type, private seat, and both methods; the front arms choose only the transported
internal reach and the caller-coordinate re-export.
The `@transcribe` arm follows the root register stamp's internal-arm precedent:
Rust exports the arm, so a direct invocation is hand-authored authority over a
new guard's two visibilities and is outside the front grammar's transport claim.
It still cannot change an existing guard because the module or item name
collides, and it cannot add a constructor or accessor because the shared body
still contains the same private field and complete method set.

The machine's production scope guards are stamped. Nine of them were tuple
structs whose position field was `pub`, which is both a public constructor and a
public accessor — so the road out the stamp refuses to emit was standing open
beside it, and this home's own guard law was false of the machine while being
true of the stamp. Writing them through the stamp is what closes that, and it is
a deletion rather than an addition: twelve hand-written declarations and their
derive lines are gone, and the shape they all now have is generated from one
place. The hand-written twin survives only on the proof surface, where the law
below needs something to compare the stamp against.

**The absence of a road out is now the compiler's statement, and no check makes
it.** A repository law used to read every stamped guard and every implementation
beside it, asking whether any of them handed the position back. It failed
repeatedly, and each failure was a Rust shape the reader had not been taught: a
receiver of a different type, a `Box`, a `Vec`, a tuple, an opaque iterator, a
type alias, a nested `Result`, a free function, an implementation for a
reference. The question was *did a person write a leak anywhere in a file
containing dozens of other types*, and it is not answerable without being a
compiler.

Seating the guard in its own module answers it structurally. The set of roads is
the expansion, and nothing else is inside the wall — so
`stamped-guards-seal-their-position` is deleted rather than repaired, and the
claim it used to make
is `E0616`, `E0423` and `E0603` on the two laundering fixtures. That is the
drain running downward: a type that makes the wrong move unrepresentable retires
the law that asserted the move was wrong, and the law goes.

## Delegated by decision

Text forms (Display/FromStr/prefixes): identity SEMANTICS live here; text
REPRESENTATION is delegated to 07_bytes and its role-prefixed scheme, whose
checksum domain includes the role prefix — mechanism admitted, one owner.
Fresh-minting layout selection (fully random, time-prefixed, writer-counter,
or anything else) is host and admission policy: no roster lives in core, and
the reader contract on `OccurrenceForm` carries the whole semantics — 16
opaque bytes, no structure parsed. Class F's composition normal form:
authored at 06_authority beside `KeyScope`.

## Obligations

```yaml
home: 02_identity
obligations:
  - id: identity.two-column-law-is-machine-readable
    challenge_kind: compile-law
    green: laws.rs identity::two_column_law_is_machine_readable
    red: owed-to-testpak
  - id: identity.scope-mismatch-refuses
    challenge_kind: compile-law
    green: laws.rs identity::scope_mismatch_refuses
    red: owed-to-testpak
  - id: identity.class-c-has-no-ord
    challenge_kind: compile-refusal
    green: laws.rs identity::scope_tuples_are_lawful
    red: owed-to-testpak
  - id: identity.typed-ref-equality-is-referent-and-version
    challenge_kind: compile-law
    green: laws.rs identity::typed_ref_equality_is_referent_and_version
    red: owed-to-testpak
  - id: identity.commitment-domains-do-not-unify
    challenge_kind: compile-refusal
    green: laws.rs identity::commitment_domains_do_not_unify
    red: owed-to-testpak
  - id: identity.stamped-scope-guard-matches-its-hand-written-twin
    challenge_kind: compile-law
    green: laws.rs identity::a_stamped_scope_guard_matches_its_hand_written_twin
    red: testpak/tests/compile-fail/cross-scope-comparison-on-a-stamped-guard.rs
  - id: identity.scope-guard-visibility-is-caller-relative
    challenge_kind: compile-refusal
    green: testpak/tests/scope_guard_visibility.rs
    red: testpak/tests/compile-fail/a-scope-guard-reexport-cannot-widen-reach.rs
  - id: identity.scope-guard-alias-cannot-widen
    challenge_kind: compile-refusal
    green: testpak/tests/scope_guard_alias_visibility.rs
    red: testpak/tests/compile-fail/a-scope-guard-alias-cannot-widen-reach.rs
  - id: identity.scope-guard-signature-cannot-widen
    challenge_kind: compile-refusal
    green: testpak/tests/scope_guard_signature_visibility.rs
    red: testpak/tests/compile-fail/a-scope-guard-signature-cannot-widen-reach.rs
  - id: identity.admission-joins-creation-to-class
    challenge_kind: compile-refusal
    green: laws.rs identity::admission_joins_creation_to_class
    red: testpak/tests/compile-fail/an-admitted-role-minted-bare.rs
  - id: identity.stamped-representation-cannot-be-laundered
    challenge_kind: compile-refusal
    green: laws.rs identity::a_stamped_representation_cannot_be_laundered
    red: testpak/tests/compile-fail/a-stamped-representation-cannot-be-laundered.rs
```
