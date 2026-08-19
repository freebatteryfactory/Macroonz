# benches — a reserved seat, and the contract it fills under

This seat is empty: no bench surface exists yet. The question it holds is
"which operations declare a growth class worth gating." It fills at the
first bench surface — generated or hand-written, since standalone benches
are lawful too — and the manifest admits the bench tooling then. The
contract below is what any filling must satisfy; it is stated here because
this seat is its owner.

Tests gate benches: a failing operation is never benchmarked, and the gate
asserts a growth class, robust not deterministic — never a time.

The bench-row field set: the workload identity; the input-size axis; the
correctness preflight; the planted-worse falsifier; declared budgets; the
declared CONTENTION POSTURE — a measurement under an undeclared contention
posture is inadmissible; and the declared work formula where the operation
states one — the gate measures WORK COUNTS, not wall time; wall time is the
secondary human observation. Gate tolerances — sample counts, warmup, the
ratio threshold — are declared constants in the descriptor: spec, not
vibes. The row owns a NEUTRAL complexity-claim reference — a standalone
public type never names a product type; the machine's integration maps its
own complexity contract into that neutral seat, product-owned on the
product side.

A bench row is pure data and cannot measure. The bench binding closes the
same hidden-callable seam the descriptor's binding closes: the row, the
measured callable, the planted-worse callable, the correctness-preflight
trial — a binding plus its invocation, because a binding does not pass by
itself — and the work-observation bindings. The host order is law: the
preflight trial passes; the planted-worse gate proves the measurement
distinguishes the declared class; only then is the measurement backend
invoked; the backend reports, never verdicts. The bench host's own typed
outcome record carries the preflight result, the planted-worse gap, the
work counts against the declared formula, and the declared budgets.

Bench generation is backend-agnostic — the reporter is a one-file swap, a
stated constraint on the benchmark projection's design. The generated bench
surface is backend-neutral, and the generation services render the one-file
reporter shell binding the neutral table to the chosen backend inside the
generated bench target; the consumer adds the backend as a dev dependency
at that target, one stated line; standalone hand authors get the same shell
as a documented recipe. The current backend choice is divan, under a named
one-maintainer risk, with criterion the fallback; instruction-count lanes
stay on the CI-era shelf.
