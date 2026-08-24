# descriptor — how a test is written down

A test nobody wrote down is a test nobody can count.

This home is the vocabulary a test is written down in.
One row states what it claims, what it exercises, what judges it, where its inputs come from, and where the row itself came from — and nothing else.
The runner enumerates rows into trials and coverage is read over the same rows, so a test with no row is a value nobody can build.

## What a row says

| Field | What it names |
| --- | --- |
| claim | the behavior the test exists to hold |
| execution suite | the one aggregate seat the row runs under |
| roles and tags | open, multi-valued classification |
| subject route | what is under test |
| check | which property suite or oracle lane renders the verdict |
| population | where the inputs come from |
| origin | who wrote the row |

A claim names behavior, never structure.
Stated in inputs, outputs, and laws, it cannot be broken by a refactor that preserves meaning.
Where bytes are declared canonical the bytes are part of the meaning, because changing them renames identities and is no refactor at all.

A role is a label and never an execution roster: nothing selects a mechanism by reading one.
The vocabulary this home shipped with — anomaly, boundary, malformed-input, regression, metamorphic, fault, crash-recovery, mutation, smoke, end-to-end, performance — is convention.
A new role is a label, a new mechanism is a law change, and a new population inside a role is a Tuesday.

## Where a row comes from

Five origins, each carrying exactly what it earns.

- **Hand-written** earns nothing beyond having been written, so naming the arm is the whole of it.
- **Generated** names the door and the projection a producer emitted through.
- **Candidate** names the opening a synthesis cut the row for, and is lawful in a staged view only.
- **Admitted on a replay-bearing ground** cites the proposal, the ground, the destination suite, and the capsule entry that admission authored.
- **Admitted on a discharge ground** cites the proposal and the destination, and has no replay seat at all.

Each arm carries its own payload type with one lawful constructor, so an incoherent origin is unwritable rather than refused.

## Row, attachment, table

A row is pure data and cannot execute.
It names its check; the callable arrives separately on an executable attachment, so no hidden row-to-function registry can exist.
A `Binding` marries the two and verifies that both name the same subject and the same check.

A row commits to its canonical bytes as it is born.
The encoder runs once, at the constructor, so every identity derived later is a reading over bytes that already exist.

Two table names and one relationship.
An authored table owns authored bindings and refuses the candidate arm outright; a staged view borrows a complete authored parent and overlays candidates on it.
Both refuse two rows stating one trial, so a denominator can never read two where one thing is measured.
One authored world, ever.

## The wall

A producer writes letters to this address; it does not own the mailbox.

Three crossings under one law: the vocabulary is owned here, producer output targets public constructors, both sides carry the same schema pin, and this side decides admission.

1. Generated support becomes bindings and tables.
2. Producer mutation discovery arrives in the mutation-discovery vocabulary, before this crate's own admission.
3. Benchmark rows arrive in the bench vocabulary.

One root schema declaration holds all three field rosters, so one pin governs all three crossings.
Its canonical bytes are the preimage of the generated-support schema identity — never hand-bumped, never a hash of source text — and a change to any member moves the identity.

## The gate

`generated_support!` is the door.
It matches the producer's copy of the published identity against this crate's copy as tokens, before either side reaches type checking, and releases both seats of a delivery or neither.

The pin crosses as thirty-two decimal byte values, and the base is what makes the comparison sound rather than merely conventional.
A macro arm matches tokens, a byte string has many spellings of one value, and the producer's side is written by the compiler's own literal writer.
An unsuffixed integer has exactly one rendering, so the two sides are one token by construction.

The gate never reads the deferred seat.
It transports the cargo or it withholds it, because a door that parsed the cargo would be a second authority over a vocabulary it does not own.

What the gate catches is pair incoherence: a version-mixed consumer, a partial rewrite, a hand edit to one side.
What it cannot catch is a jointly stale pair, where the declaration moved and neither literal was rewritten — two old values agree just as loudly.
That one belongs to `harness/tests/published_schema_currency/`, which derives the identity from the current declaration and requires both published spellings to equal it.
The refusing arm itself tells whoever meets it how the pin is rewritten.

## The stamp

`trial_table!` turns one declaration into the authored world, one ordinary test per execution suite, and one ignored named lens per row.
The lens is what makes every row clickable in an editor without being paid for twice in an ordinary run.
Both spellings call the same engine, so a verdict cannot differ between them.

## What this home does not claim

It imports no producer's types, so no producer's shape can quietly become the interface.
It says nothing about what a trial concluded — a table that was never built ran nothing, and a verdict belongs to the record home.
It reads no host fact: a target and a clock are declared at the invocation, because a cache key with a guessed toolchain in it is a lie with a digest.
