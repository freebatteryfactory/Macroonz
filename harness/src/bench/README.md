# bench — a number that means the same thing tomorrow

A wall-clock number on its own is a rumor.

Run the same code twice and the two timings disagree.
Run it on another host, or beside a noisy neighbor, and they disagree more.
So this home decides nothing from the clock.

It counts the work a subject actually does, at several input sizes, and asks a judge you wrote whether that curve is the shape you claimed.
The clock is read afterwards, and only for a row that has already qualified.

## What a row declares

A benchmark row is data.
It cannot run a subject, consult a host, or decide a verdict.

It names a workload, an axis of at least two distinct input sizes, a correctness preflight, a deliberately worse control, exact sample and warmup budgets, a contention posture, an optional formula spelled in bytes the owner chose, and a complexity claim nothing here interprets.
Each of those names is a `NamespacedName` you declared, and each ratio is an exact pair of integers, because a benchmark that rounds is a benchmark that drifts.

`BenchRowKey` is derived from that whole declaration.
Change any of the eight facts and it is a different row with a different key — which is the point, since a number is only comparable to a number from the same declaration.

## The row's preimage

Read `u32be(n)` as an integer in four big-endian bytes, `u64be(n)` as one in eight, `bytes(x)` as `u64be(x.len())` followed by `x`, and `name(x)` as `bytes(namespace)` followed by `bytes(stem)`.

The key is derived over exactly this, under `BENCH_ROW_KEY_TAG`, with no separators and no padding:

```text
name(workload)
u64be(size count)
u64be(size)                repeated, in authored order
name(preflight)
name(planted worse)
u32be(samples)
u32be(warmups)
u64be(ratio numerator)
u64be(ratio denominator)
u8(contention tag)
u8(formula present)
bytes(formula)             only where present
name(complexity)
```

The one contention posture is tag zero, and formula presence is zero or one.

## The planted-worse control

Every row must also name a callable that is deliberately worse than the one being measured.

A benchmark that only exercises the good road cannot tell you whether it is measuring anything at all.
If the judge cannot tell the planted-worse curve apart from the measured curve, by the exact ratio the row declared, the row does not qualify and the measured curve's own result is never reached.
A control that fails to look bad is a broken instrument, not a passing benchmark.

## The order a run takes

For each row, in this order and never out of it:

1. The correctness preflight runs as an ordinary trial, because a subject that is wrong is not slow, it is wrong.
2. The measured callable records its work curve across the axis.
3. The planted-worse callable records its own curve across the same axis.
4. One judge — yours, a plain function pointer — reads both curves together and answers three things: the measured conclusion, the planted-worse conclusion, and whether the declared gap was observed.
5. Only if the control was refused *and* distinguished, and the measured curve satisfied, does the timed pass run: warmups discarded, then one clock measurement per sample.
6. The same judge reads the timed pass's own curve, and a row that stops qualifying under the clock publishes no report at all.

Before any of that, every row's preflight invocation is checked against the run's declared target and toolchain.
A mismatch refuses before one line of caller code executes, because comparing a number from one target against a number from another is the mistake this home exists to prevent.

## What the judge may see

`WorkJudgmentInput` carries the formula, the complexity claim, the budgets, and the two curves.

It carries no duration, no reading, and no clock, and that is settled by its type rather than by a rule someone has to remember.
Qualification is decided from work.
The clock only ever describes a row that already qualified.

## The recorder

A benchmark callable is handed a `WorkRecorder` scoped to exactly the observations its binding declared.

`record` refuses a name outside that roster, and refuses a count that would overflow.
A callable cannot invent an observation mid-run, and it cannot quietly wrap a counter.

## The report

`BenchReport` holds one reading per authored binding, in authored order, and only this home can construct one.

A reading keeps the complete row, the target it stood on, the preflight report, and a stage-shaped outcome: refused preflight, undistinguished control, refused primary work, or qualified.
`bench_verdict` folds those readings down to the first row that did not qualify.

A renderer may take a `&BenchReport` and do whatever it likes with it.
What it cannot do is build one, reach a benchmark callable or the judge, or change a stage or a denominator.

## The private owners

The public owner remains this `bench` home, and every public item stays at `macroonz_harness::bench`.

`declaration/` owns the row facts and the one canonical preimage from which `BenchRowKey` is derived.
`work/` owns scoped recording, work curves, owner judgment, and the executable work attachment.
The parent owns preflight binding, table admission, host execution, timed observation, report minting, verdict, and stamped refusal composition across those children.

The child modules are private implementation homes and add no public path.

## What this home will not grow

There is no benchmark backend here, and there will not be one.

It runs no statistics, fits no curve, and holds no opinion about what "fast" means — the formula bytes belong to the owner and pass through unread.
It reads no host fact of its own: the target, the toolchain, the clock, and the contention posture all arrive at the invocation, declared by whoever is running.
