# presentation — readable projections of admitted records

This home displays records through their existing public readers.
The executable [trial example](../../examples/trial_workflow/README.md) shows the public call and all formats, and the [publication example](../../examples/publication_workflow/README.md) displays native command success and failure.
`input_run` displays the [workflow owner's](../workflow/README.md) original admitted input beside its complete run report.
`suite_selection_refusal`, `input_refusal` and `archive_refusal` retain selection, decoder and report-format failures without inventing an execution or the caller's read/write phase.
The [report owner](../../harness/src/report/README.md) owns execution meaning and the [archive owner](../../harness/src/report/archive/README.md) owns historical admission.
`run`, `trial` and `capsule` read their recorded values; `archived_run`, `archived_trial` and `archived_capsule` read integrity-admitted historical values.
`compiler_diagnostic` displays the [compiler diagnostic](../../macros/compiler/src/diagnostic/README.md), retaining its family, stable class, phase, source-coordinate role, expected contract, observed difference, related-set capping and cited repairs.
`benchmark` and `archived_benchmark` retain [complete benchmark observations](benchmark/README.md), while `benchmark_refusal` and `benchmark_verdict` distinguish an absent report from the first refusal within a report.
`reduction` and `archived_reduction` retain the [reduction account](../../harness/src/generate/reduction/README.md): only invoked reducers, the complete candidate census, the reached witness, halt and participant ceilings.
They do not reconstruct original bytes, uninvoked plans or individual candidate observations, and a reached witness is not a claim of global minimality.
`replay_comparison` retains the [joined replay reading](../../harness/src/report/replay/README.md), keeping its outcome, coordinate movement, lineage and historical ceiling independent.
`replay_join_refusal` displays an unsuccessful witness join and establishes no replay outcome.
`legacy_record` preserves [sparse historical source](../../harness/src/report/legacy/README.md) and every missing, null or present field without filling absent headers or interpreting outcome prose.
`legacy_comparison` retains the existing field-by-field claim relations and unverifiable ceiling; `legacy_join_refusal` records why no such comparison was admitted.
Compiler [observations and comparisons](oracle/README.md) retain exact diagnostics and compiled-value differences without reinterpreting them.
[Mutation projections](mutation/README.md) retain complete discovery and backend populations, independent outcome axes, and historical parity and interpreted pressure.
[Coverage projections](coverage/README.md) retain readiness, complete observations and frontiers, novelty decisions, budgets and phase-specific refusals without assigning semantic meaning to coverage.
`seed_pack` displays the [corpus owner's](../../harness/src/corpus/README.md) population, exact envelope and authored seed order without assigning a verdict or replay standing.
`seed_pack_refusal` retains an admission failure without inventing whether the caller was reading or writing.
[Network transcript projections](network/README.md) preserve source claims, complete simulation manifests and delivery rosters alongside separately established reproduction, playback and joins.
`archived_proposal` retains the [historical offer](../../harness/src/muterprater/proposal/archive/README.md), its complete candidate, destination, concrete ground and comparison.
Proposal identity and evidence address remain distinct, repeated prior failures retain their order, and absent claim or discharge-trial joins are not inferred.
Opt-in [native projections](native/README.md) retain process, compiler interpretation, read-back failure and unfinished cleanup standing.

Each operation produces one immutable `Presentation` whose `json`, `markdown` and `html` methods show the same fields.
JSON preserves structure; Markdown and HTML show every container and leaf in a path/value table, including empty containers and explicit nulls.
Array positions retain source order, and the complete census appears whether or not a row was selected or exercised.
There is no pass-only rendering or caller-supplied replacement census.

The envelope names `macroonz-presentation-v1`, the record kind, its semantic owner and its standing.
`recorded` means the supplied typed record; it does not assert that execution occurred in the current process or authenticate a host claim.
`historical-unauthenticated` retains an archive's weaker standing even when its recorded outcome says passed.
Presentation neither admits evidence nor reconstructs callables, performs replay, establishes current source agreement or authorizes a proposal.
Canonical identity, storage and readback continue through the record's owning format; presentation text is not a canonical archive.

Foreign material retains its exact admitted bytes as lowercase hexadecimal alongside the shown text, fidelity and original truncation counts.
Text never determines an outcome, cause or measurement.
Names, paths and foreign text are data in every format: Markdown punctuation and HTML markup are escaped, and no supplied text becomes a link, attribute or script.
JSON is a standalone JSON document, not an HTML script fragment.
Projection retains all supplied data without display truncation; its memory and output grow with the admitted record.
