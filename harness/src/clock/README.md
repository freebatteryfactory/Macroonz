# clock — wall measurement without a clock of its own

Time is an input.

The harness measures elapsed wall time and owns no clock.
A caller declares one `HarnessClock` at the invocation, and every wall reading in that run comes from the declared source and from nowhere else.
Nothing here asks the host what time it is, because a duration nobody declared is a number a report cannot stand behind.

## One measurement

`HarnessClock::begin` reads the declared source once and hands back an opaque `MeasurementStart`.
The caller then does the work it wanted timed.
`MeasurementStart::finish` consumes that start and reads the same source a second time.

The start is opaque and single-use on purpose.
No caller can swap the clock halfway, reverse the two readings, or close one measurement twice.

## Three readings that never collapse

A `MeasurementReading` is exactly one of three things.

- `Observed` carries a duration, and a duration of zero is a real observation.
- `Unavailable` says the caller declared no clock for this run.
- `Failed` says a clock was offered, the measurement did not complete, and here is why.

A failure keeps the boundary it happened on: whether the source refused or unwound while opening, or while closing.
A closing reading that precedes its opening reading is a `Regressed` failure carrying both readings, never a difference saturated to zero.

Ticks and durations are separate types.
A `MeasurementTick` is one admitted reading on the caller's own origin, and a `RecordedDuration` is the checked difference between two of them.

## What a source may be

The source is a plain function pointer, because clocks are declared in generated and hand-written test targets where a closure cannot be spelled as a constant.
A function pointer excludes captured state; it does not make the caller's function pure.

An infallible source returns nanoseconds.
A fallible source returns `ClockReadRefusal` instead of unwinding.
Either way, an ordinary Rust unwind out of a source read is caught at the boundary it happened on and recorded as a failure.
A process abort remains the host's business.

## What this home will not grow

It measures, and that is the whole job.

A reading is recorded beside a verdict and never inside one: it enters no identity, no selection, no conclusion, no budget decision, and no mutation control.
There is no scheduling here, no sleeping, no deadline, and no shared notion of now.
Whoever adopts the harness keeps its own time semantics, and declaring a source here creates no clock for the rest of that system to consult.
