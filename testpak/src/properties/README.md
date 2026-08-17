# properties — the algebraic suites

A property is a law a stateless function must hold across its typed domain:
roundtrip, idempotence, conservation, monotonicity, and the metamorphic
shapes — permutation insensitivity, run-twice-one-answer determinism, and
ambient-pathway invariance. The oracle for a property is the declared algebra
itself; no second implementation is needed for a law to be checkable.

Temporal laws are the suite shape over histories: a generated command
sequence drives a transition system, and the law is asserted across the whole
history — what always holds, what never happens, what is eventually reached.
Parity is the suite shape over roads: wherever one meaning is reachable two
ways — a maintained result and its recomputed fold, a generated artifact and
its hand-written twin, two doors over one declaration — the suites drive both
roads with the same inputs and demand agreement. A parity law pins meaning
while leaving every road free to change.
The sequence driver is the fuzz lane's generation machinery; a command
sequence is a structured input like any other, and a failing sequence is a
counterexample carrying its seed like any other.

Because every semantic function is stateless and typed, it is legal here
unmodified — the suites import product functions directly and drive them with
structure-aware generation over `arbitrary`, budgeted and minimizing. A
failure is a typed counterexample carrying its seed, not a panic.

Composition owes its own laws: wiring correct operations in the wrong order
is still a defect, so composed roads carry a small named suite of the same
algebraic shapes.

Behavioral laws drain out of the machine's proof surface and land here as
properties under mutation pressure; at end state the machine keeps no
self-test corpus of its own. The suites drive declared transition systems;
the machine's own running history becomes a subject only once the machine
can run one.
