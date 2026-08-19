# runner — descriptor tables become runs

The runner is a pure engine with two calls: `run_one` takes a binding and a
typed invocation and returns one trial report; `run_all` takes the sealed
table view, a selection, and a typed invocation and returns a run report.
The row is pure data and cannot execute — the binding carries the callable,
so no hidden row-to-function registry can exist. The table is always the
complete world; the selection chooses from it, and the run report accounts
on two owned axes: the selection disposition first (selected, or
not-selected with its reason), then a run attempt only where selection
admitted one — a caller can narrow a run, never the denominator, and
not-selected can never impersonate a failed execution. Invocation is a
parameter and results are values. The runner touches no
process boundary: it reads no arguments, prints nothing, and exits nothing,
so the wall holds over it without a single exception.

This is the harness's only semantic-trial execution engine: the compiler
executes compile refusals, external mutation tooling runs its campaigns
outside the wall, the bench executor measures, and the standard harness
hosts trials — none of those renders a semantic verdict. Every semantic
run anywhere in the loop — aggregate seats, candidate proving,
mutant-scoped subsets, fuzz batches, chaos schedules — is the same call
with a differently selected subset of the one complete table.

Hosting is the caller's. The stamp spelling gives every row a named test
function, so the standard harness carries listing, filtering, and per-trial
visibility natively — no protocol code lives in this tree; process fan-out
and parallelism are the host harness's, natively, and sharding is its
partition flags — external, zero machinery here. Default execution has its
physics stated: aggregate seats are ordinary test functions and run by
default; named lenses are ignored-by-default — clickable and
filter-runnable, never paid twice. The stamps live with the descriptor
vocabulary they read; their expansion names this engine through the
defining crate's own path, resolved where the consumer invokes them, so the
descriptor home gains no edge to this one. A standalone
adopter hosts the same engine from their own entry point under their own
lints; a custom-harness shell is a documented recipe, not machinery here.

The census delta is a pure report operation — the comparison lives with the
report vocabulary, under its one typed-baseline statement — so the runner
never grows memory. Selective re-run is designed-for now and arrives with
the identity rails: skip trials whose execution key matches the last
report, eligibility governed by the attachment postures per the report
instrument's one statement.

Discovery is pure and in memory — a trial table is constructed from typed
descriptors, so nothing scans and nothing spawns. A subject panic is CAUGHT
at the trial boundary — one unwind catch plus one chained panic hook
installed once, the hook process-global, correlated per trial — and the
safe payload and location are copied into the typed finding. Aborts and
stack overflow are honestly uncaught; process isolation is a caller's
hosting recipe, not machinery here. The runner itself has no panic path.
