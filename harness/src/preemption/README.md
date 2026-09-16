# preemption

The schedule inside one operation is an input.

## Boundary

The [`interleave`](crate::interleave) home explores orders between whole commands, while this home explores instruction-level schedules and memory-model behavior inside those commands.
It wraps a pinned external scheduler as a declared backend rather than rebuilding scheduler semantics inside Macroonz.
The model remains the adopter's Rust, written against the backend's shadow vocabulary, and the model reports its own check as a typed return value.

![Preemption backend result road][diagram-harness-preemption]

Diagram source: `assets/diagrams/harness-preemption.mmd` in this crate's source package.

[diagram-harness-preemption]: ../../assets/diagrams/harness-preemption.svg

## Result plane

[`PreemptionReading`] retains the declared [`PreemptionBounds`] beside the strongest [`PreemptionOutcome`] the qualified backend established.
A completed outcome is either [`PreemptionVerdict::AllInterleavingsHeld`] over the bounded space the backend walked or [`PreemptionVerdict::ModelBroke`] from the model's explicit typed refusal.
An incomplete outcome states backend unavailability, initialization failure, or unresolved execution without turning infrastructure behavior into a subject verdict.

[`attempted`] projects the completed and incomplete arms onto the harness report vocabulary.
Completed model evidence becomes an executed conclusion, while incomplete exploration remains an infrastructure failure.

## Composition

The model may compose the pinned backend's synchronization, thread, and async vocabulary directly because those operations participate in the same controlled schedule.
The compiler's separate shadow declaration road gives production code ordinary and modeled faces of its chosen synchronization names without importing scheduler policy into the compiler.
The manifest owns the exact backend pin, feature, and target qualification, while the external preemption lane holds that posture against the compiled dependency and generated shadow roster.

The neighboring concurrency floors remain separate.
[`interleave`](crate::interleave) owns command-order schedules without threads, and [`network`](crate::network) owns deterministic delivery schedules between nodes.
Composition happens through adopter-owned models and command values rather than by flattening those owners into this one.

## Evidence ceiling

The declared bounds state the search space the backend was allowed to walk, and this home claims no iteration count the backend does not report.
Only [`PreemptionModelFailure`] can establish the model-broke verdict.
Foreign unwinds remain bounded diagnostic material on an incomplete infrastructure rail, even when their text resembles a model refusal.

The backend may be unavailable on a compilation target, may refuse during initialization, or may begin execution without returning a typed verdict.
Those ceilings stay visible in the result type and in the corresponding report projection.
On unsupported targets the public result plane remains present and the supplied model is not invoked.

The qualified backend owns process-level effects that an in-process unwind boundary cannot promise to contain.
A stronger containment claim would require a separately owned process-isolation road.
