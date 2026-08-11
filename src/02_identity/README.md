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

The stamped position is private, exactly as a hand-written guard's always was,
and the stamp emits one road in and none out: `positioned` reads a position the
caller already holds under this role, and no accessor hands it back. A role
whose representation could be taken out and re-entered under another role would
be a label rather than a wall, so both directions refuse from outside the
module the stamp expanded in — and the refusal is proven over ONE scope type,
where nothing about the scope is helping.

Every scope-guard already written by hand stays written by hand. The stamp is
proven against one of them, not substituted for them.

## Delegated by ruling

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
  - id: identity.stamped-representation-cannot-be-laundered
    challenge_kind: compile-refusal
    green: laws.rs identity::a_stamped_scope_guard_matches_its_hand_written_twin
    red: testpak/tests/compile-fail/a-stamped-representation-cannot-be-laundered.rs
```
