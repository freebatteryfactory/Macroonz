# Retained benchmark workflow

This home joins complete benchmark reports to bounded native storage.
It is available with `native-tooling`.
Run the caller's admitted table with [`harness::bench::run_all`](../../../harness/src/bench/README.md), selecting its target, contention posture and clock explicitly.
The runnable `benchmark_workflow` example supplies actual measured work, an independently written correctness preflight and work judge, and an executed worse control.

Pass the returned `BenchReport` to `retain` with an explicit storage root, batch name and independent archive/storage limits.
Every row and every reached failure axis enters the existing benchmark archive, including the complete primary judgments and secondary clock outcomes.
Retention does not call `bench_verdict`, discard refused rows or turn unavailable timing into observed zero.
Execution refusals that prevent a complete report remain the [benchmark owner's](../../../harness/src/bench/README.md#the-report-boundary) result; this home does not manufacture a partial report for them.

The batch has exactly one mechanical member, `benchmark`, containing the existing canonical benchmark envelope.
Archive and storage bounds are checked before reserving a new batch.
`load` checks the complete member roster and delegates byte admission to the existing archive reader, returning its `ArchivedBenchReport` directly.
Loading performs no preflight, work, judge or clock calls and grants no current execution or performance claim.

A failed write leaves the caller's live report available.
`recover_retention` explicitly replaces an unpublished attempt from that report under the [storage owner's](../../native_storage/README.md) existing custody rules.
Published batches cannot be overwritten or recovered.
Storage errors remain separate from archive refusals and subject conclusions.

No clock, runner, archive grammar, performance threshold or report model is owned here.
The `benchmark_workflow` example runs with `cargo run --example benchmark_workflow --features native-tooling`.
Its declared output directory is disposable under `target/qualification/benchmark-workflow-example`.
