# `declaration` — one benchmark row before anything can run

This private child owns the facts an authored benchmark row states and the identity derived from those facts.

A row names its workload, correctness preflight, planted-worse control, complexity claim, input-size axis, exact budgets, contention posture, optional formula bytes, and table identity.
It cannot carry a callable, a clock, a judgment, or a report.

`BenchRowKey` is derived once from the complete declaration under `BENCH_ROW_KEY_TAG`.
The encoder preserves authored axis order and the exact byte grammar stated by the parent bench home.

`types.rs` owns this vocabulary, `type_guard.rs` owns its constructors and readers, and `encode.rs` owns the canonical row preimage.

The child is private.
The parent [`crate::bench`] door preserves every public path.
