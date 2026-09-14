# Benchmark execution and retention

Run `cargo run --example benchmark_workflow --features native-tooling` from the package root.
The example counts nonzero values, checks literal independent correctness cases, measures visited elements, and executes a repeated-scan control.
The work judge compares exact counts against a separately written linear claim and the declared gap.
Native time is recorded only as a secondary observation.

`specimen` owns this example's subject and independent judgments; `declaration` supplies its explicit names, revisions, target, budgets and bindings.
`main` runs the table and uses the root workflow to retain and load its complete historical account.
The example creates its fixed disposable output directory exclusively and removes it on success.
An existing directory refuses rather than overwriting a prior interrupted example.
The target label identifies this neutral caller, while the toolchain label is an explicit declaration rather than runtime discovery.

This is an executable composition example, not a claim that counted visits prove every possible cost or that one timing run establishes stable performance.
