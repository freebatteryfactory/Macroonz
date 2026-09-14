# Native clock source

This home supplies the existing harness clock with a checked native monotonic reader.

Enable `native-tooling` and pass `native_clock::source()` where a runner or benchmark accepts `HarnessClock`.
Creating the source reads no clock.
The first actual reading establishes one process-local origin shared by this adapter's readers.

## Read boundary

On Windows, Linux and macOS, each read obtains an `Instant`, checks its difference from the retained origin, and converts nanoseconds to the harness reader's `u64` range.
A reading before the origin or beyond that range refuses; it is never saturated or truncated into an observation.
An ordinary unwind reaches the harness's existing source-failure boundary.
Other targets supply declared unavailability without calling a native clock.

The [harness clock](../../harness/src/clock/README.md) owns opening, consuming finish, the checked difference between readings, attribution and measurement outcomes.
Native source classification does not certify operating-system behavior, steady ticks, suspend accounting or an independently verified physical clock.

## Authority

Readings are optional measurement evidence and never supply identities, selection or semantic judgments.
This home owns no process deadline or execution budget.
The repository's [working law](../../AGENTS.md) owns native-effect permission, and [contribution procedure](../../CONTRIBUTING.md#local-wall) owns its qualification.
