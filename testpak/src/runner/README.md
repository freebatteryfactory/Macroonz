# runner — descriptor tables become runs

The runner is a pure engine: it takes a descriptor table and a typed
invocation — which trials, what budgets — and returns typed reports.
Invocation is a parameter and results are values. The runner touches no
process boundary: it reads no arguments, prints nothing, and exits nothing,
so the wall holds over it without a single exception.

Hosting is the caller's. The stamp spelling gives every row a named test
function, so the standard harness carries listing, filtering, and per-trial
visibility natively — no protocol code lives in this tree. A standalone
adopter hosts the same engine from their own entry point under their own
lints; a custom-harness shell is a documented recipe, not machinery here.

Discovery is pure and in memory — a trial table is constructed from typed
descriptors, so nothing scans and nothing spawns. Subject panics are
contained at the trial boundary and converted into verdicts with their
locations; the runner itself has no panic path.
