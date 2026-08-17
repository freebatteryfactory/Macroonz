# runner — descriptor tables become nextest trials

The runner enumerates descriptor tables into named trials and speaks the
nextest custom-harness protocol itself: list mode emits one stable
`module::path::trial_name` per line; exact mode runs one trial; failure is a
returned typed value, never a panic. The protocol is small enough to own
outright, which is what keeps an adopter's inherited tree tiny.

The invocation arguments and the list stream are this binary's one declared
host port — the single seam where the harness touches the process boundary,
and the wall's spelling for that seam is recorded at the seam itself.

Discovery is pure and in memory — a trial table is constructed from typed
descriptors, so per-trial process spawning costs nothing quadratic. Subject
panics are contained at the trial boundary and converted into verdicts with
their locations; the runner itself has no panic path.

Execution is sequential or scoped-parallel; nothing here owns a thread pool,
an ambient clock, or an environment read.
