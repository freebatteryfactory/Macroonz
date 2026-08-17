# runner — descriptor tables become nextest trials

The runner enumerates descriptor tables into named trials and speaks the
nextest custom-harness protocol itself: list mode emits one stable
`module::path::trial_name` per line; exact mode runs one trial; failure is a
returned typed value, never a panic. The protocol is small and this crate
owns its ~four hundred lines, so an adopter's inherited tree stays one crate.

Discovery is pure and in memory — a trial table is constructed from typed
descriptors in microseconds, so per-trial process spawning costs nothing
quadratic. Subject panics are contained at the trial boundary and converted
into verdicts with their locations; the runner itself has no panic path.

Execution is sequential or scoped-parallel; nothing here owns a thread pool,
an ambient clock, or an environment read.
