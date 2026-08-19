# descriptor — the typed vocabulary every producer writes into

A descriptor is one row of the harness's denominator: a typed statement of one
test — the claim it serves, the roles that classify it, the subject it
exercises, the check that judges it, and where it came from. A claim names behavior,
never structure: it is stated in inputs, outputs, and laws, so a lawful
refactor of the subject cannot break it. A test that would break under a
refactor that preserves meaning is coupled to an owner, not to a law — and
owner-coupled tests are how structure ossifies and intent flattens. Where
bytes are declared canonical, the bytes are part of the meaning: a check
anchored on a canonical encoding is coupled to a declared contract, not to
an owner, because changing those bytes renames identities and is no
refactor at all. The runner enumerates
descriptor tables into trials; coverage is computed over the same tables; a
test that exists without a row is a value nobody can build.

testpak owns these types. Producers — the generation services, a hand, an
admitted proposal — emit data conforming to this vocabulary; no
producer's own types are imported, so no producer's shape can quietly become
the interface. Admitting what a producer constructed is itself a lane.

Execution is answered by the check reference and the subject route — sealed
by being this crate's types, so a new mechanism is structurally a law
change. A role is honest, open classification: namespaced, multi-valued,
never an execution roster. The initial role vocabulary: anomaly, boundary,
malformed-input, regression, metamorphic, fault, crash-recovery, mutation,
smoke, end-to-end, performance. A new role is a label; a new mechanism is a
law change; a new population inside a role is a Tuesday.

The fields of a row, closed: the claim served (a typed identity); the
execution suite — exactly one, the aggregate seat the row runs under by
default; roles and tags — open, multi-valued classification; the subject
route — a typed selection of what is under test; the check reference —
which property suite or oracle lane judges the subject (the row references
its check; only a Binding carries a callable); the population — which
generated population supplies the row's inputs; and the origin, whose arms
carry exactly what they earn: hand-written | generated, with producer
facts | candidate, with synthesis facts — lawful only in a staged view,
never authored | admitted-replay, citing its proposal's content identity,
the admission facts, and the replay reference into the depot capsule the
admission act authored | admitted-discharge, citing its proposal's content
identity and the admission facts, with no replay seat at all. The schema
identity is not a row field: it rides the generated Binding and Table
provenance, so hand-written rows never touch it and row identity never
churns when a producer-facing schema changes.

A Row is pure data and cannot execute. The EXECUTABLE ATTACHMENT carries the
typed subject reference, the typed check reference, one posture-bearing
revision binding for each, and the callable; `Binding` pairs one Row with
one attachment, and its constructor structurally verifies that the row's
references match the attachment's. Each revision binding carries its honest
posture with a stated claim ceiling: derived (generated from an owned
declaration); declared (a hand author's explicit commitment — the ceiling is
the author's word); untracked (no stable commitment — lawful). What a
posture means for the cache and for replay — including the meet law for the
attachment's two bindings — is the report instrument's one statement.

The table constructor refuses two rows with one trial identity — a
duplicated trial cannot exist in a constructed table, so a denominator can
never read two where one thing is measured. Three table names, one declared
relationship: an AUTHORED TABLE owns authored bindings and its constructor
refuses the candidate origin arm outright; a STAGED VIEW borrows its
complete authored parent and overlays candidate bindings, enforcing trial
uniqueness across parent and overlay; both present the one sealed read
surface the runner takes. One authored world, ever.

## The seam

The generation services emit against this vocabulary; this instrument
decides admission. The producer writes letters to an address; it does not
own the mailbox.

The spines meet at ONE WALL with exactly three named crossings, every
crossing under the same law — public vocabulary owned here, producer emits
against it via public constructors, the two-sided schema pin, this side
decides admission: (1) generated support → the construction contract →
Binding and Table; (2) the mutation-evaluation surface → the
mutation-point vocabulary; (3) benchmark rows → the bench target's row
vocabulary. One GENERATED SUPPORT SCHEMA declaration — the root, whose
members are the descriptor, mutation-point, and bench schemas — covers
every producer-facing vocabulary, so one pin governs all three crossings.
The object that physically crosses is the GENERATED SUPPORT SHELL: a
deferred token carrier the door emits at the declaration site (a macro
invoked in a test target sees only its own invocation tokens, so the
declaration's structure must travel as tokens), holding its cargo inert —
constructor-calling expressions for the row vocabularies, the evaluation
copy for the mutation crossing — naming no testpak type until expansion and
executing nothing in the normal build. The consumer's consumption targets
invoke it: row expressions for crossing one and the evaluation copy land in
the test target; bench rows land in the bench target. Nothing crosses
unnamed.

The root schema declaration's canonical bytes are the PREIMAGE of the
generated-support schema identity, which is derived from them — never
hand-bumped, never a hash of source text, and bytes are never "the id"; a
change to ANY member moves the id, so the one pin governs all three
crossings mechanically. The producer holds its own independently held
expectation of that identity — two values in two crates, whose independence
is across upgrade time. Both checked-in sides are written by one explicit
publication operation at schema-change time, git-visible, human-committed,
under a receipt; the first pair is hand-authored under a declared-bootstrap
posture (the author's word, claiming pair coherence only) and becomes
verified-derived at the first toolchain contact, the flip itself a
receipted, human-committed publication act. What the pin's comparison
detects is pair-coherence failure: a version-mixed consumer, a partial
publication, or a hand edit to one side. Inside one workspace, where both
sides move together, the pin's live protection is the last two only — that
limit is stated, not hidden. What it cannot detect, stated: a
jointly stale pair — the schema changed and publication never ran, so the
two old literals still agree and the gate opens. Pair currency is the
conformance trial's job: the executed admission trial derives the current
schema's id and checks the published literal against it. The disposal
routes, exactly: pair incoherence dies at the gate; joint staleness dies by
whichever tripwire the drift reaches first — changed constructor shape is
rejected by the compiler as ordinary type errors before any trial runs; a
stale surface that still typechecks is rejected by the conformance trial.
Every drift dies; only the gate's own claim is narrow.

The pre-typecheck gate makes the loud break a mechanism, not a hope: this
instrument owns a local `generated_support!` gate that token-matches the
producer's traveling expectation against the checked-in published literal
BEFORE releasing the constructor body into type checking — a mismatch
expands to one owned diagnostic and the constructors never reach the
compiler. Admission's first check is producer-expected against the
published harness id — one precise loud break when the two published ids
disagree, never a cascade of field errors on that road; a jointly stale
pair sits outside this comparison's claim. Generated expressions call
public constructors, never struct literals; the conformance lane is
admission exercised as one executed trial, never a registry. The rename
twins hold: a consumer may rename both crates, and generated code honors
both names.

The stamp battery lives here with the vocabulary it reads: `trial_table!`
expands rows into the table, the named lenses, and one aggregate seat per
execution suite — invoked by `generated_support!` on the generated road and
directly by hand authors; `ensure_*` sugar lives with the property
combinators it wraps. One refusal family closes that road: a declared row
expression builds its own parts through the public constructors and writes
the language's own `?` on each, so every construction that can refuse — a
name, a classification roster, a row, the published root schema declaration,
its derived identity, the binding itself — has exactly one declared discharge
into the stamp's family, stated once and never a variant a producer invented
inside a vocabulary it does not own. A macro body is tokens: `$crate` names the crate that
defined the macro, expansion occurs at the invocation site, and this home
gains no dependency edge from either stamp.

Two spellings read these tables — the runner enumerates them at run time,
and the stamp expands them into named test functions, which is also what
gives every row an editor test lens. Both roads call the same engine, so
verdict drift between them is unrepresentable by construction; the
spellings-parity trial exercises what the roads do NOT share — selection,
aggregation, and suite wiring — and names run_one as its shared substrate,
because agreement across a shared substrate is silence about that
substrate. The tables are the single source of truth for both.
