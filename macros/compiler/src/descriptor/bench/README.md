# `bench` — neutral benchmark declarations completed by a target

One declaration produces two coupled units: a stamped benchmark table and a typed report-reader value.

They travel through one carrier and one generated-support pin because either unit without the other is an incomplete delivery.
The table is constructor-shaped cargo for `macroonz-harness`.
The report reader is a target-supplied `fn(&BenchReport)` value carried in a small generated module.

## Declaration-owned facts

[`BenchmarkDeclaration`] owns only stable authored facts:

- the support address;
- the generated table function and the table's namespaced identity;
- authored row order;
- each workload, preflight, planted-worse, and complexity reference;
- each input-size axis;
- sample count and warmup count;
- the exact ratio numerator and denominator;
- the optional formula bytes;
- the work-observation references;
- the report-reader module name.

The four budget values are named fields rather than a positional account.
No denominator is inferred, and no complexity reference is interpreted as a judge.

An axis carries at least two distinct sizes because a growth relation is read from a curve rather than one point.
Each row carries at least one distinct observation reference because the harness recorder cannot qualify work it was never permitted to count.

## Target-owned facts

The carrier invocation supplies each row's measured callable, planted-worse callable, real `WorkJudgeBinding`, and complete `PreflightTrial`.
It also supplies the report reader.

Those values are expressions rather than rendered paths, so they resolve in the target's own scope and remain valid when dependencies are renamed or reached through a facade.
The compiler does not rebuild a preflight from a callable, infer a judge from a complexity label, or acquire a clock.

The target constructs its `BenchInvocation`, including target, toolchain, contention posture, and `HarnessClock`, then hands the generated table to the harness's `run_all` host.
The generated cargo never becomes a second host.

## One renderer

The table renderer calls the harness's public `BenchReferences`, `BenchMeasurement`, `BenchRow`, `BenchAttachment`, and `BenchBinding` constructors in their declared relationship order.
The same declared observation roster becomes the attachment's recorder scope.

The report reader is stored as a function pointer and is never wrapped in a backend protocol.
It receives a finished `BenchReport`, owns no verdict, and cannot reach the callables or clock that produced the report.

## A benchmark is evidence

The declaration states one measurement contract and nothing about another realization.
Measurements and verdicts belong to the harness execution road, not to generated syntax.

## The seats

`types.rs` declares the kind, the table and reporter roles, the question it owes, and the neutral declaration vocabulary.

`type_guard.rs` owns the informed row and declaration constructors.

`type_contract.rs` owns the kind, role, question, canonical-content, and refusal contracts.

`capture.rs` reads the closed helper grammar.

`render.rs` projects the informed declaration into the current harness constructors and the typed report-reader seat.
