# clock — wall measurement without a clock of its own

Time is an input.

This home turns one caller-declared source into a typed elapsed reading.
It owns the boundary discipline around that source and no theory of what time means to the caller.
Nothing here asks the host what time it is, because a duration nobody declared is a number a report cannot stand behind.

## One source, one measurement

Opening a measurement retains the declared source behind an opaque value.
Finishing consumes that value, so the second read cannot be redirected, repeated, or separated from the first.
A source that is unavailable or fails while opening still leaves a finishable value, allowing the caller's work to run before the measurement posture is published.

The two admitted readings share the caller's own origin.
Their checked difference is a duration only when the closing reading does not precede the opening reading.

## Honest outcomes

An observed duration, declared unavailability, and a failed measurement remain different facts.
Zero is an observation rather than an alias for unavailability.
A failure retains whether opening or closing refused or unwound, and a backwards pair retains both readings rather than saturating to zero.

The source boundary catches an ordinary Rust unwind and records it without claiming that a caller function is pure, monotonic, terminating, or abort-safe.
Those properties remain the caller's evidence.

## Composition and ceiling

Runner, benchmark, and mutation operations may record the reading beside their own evidence.
The reading never enters an identity, selection, conclusion, budget decision, mutation classification, or owner judgment.

This home schedules nothing, sleeps nowhere, owns no deadline, and creates no shared notion of now.
Declaring a source here creates no clock for the rest of an adopter's system to consult.
