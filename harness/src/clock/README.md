# clock — `TestPak` measurement without a machine clock

This home owns `TestPak`'s wall-measurement boundary. A caller declares one `HarnessClock`; beginning a measurement reads that source once and returns an opaque `MeasurementStart`, and finishing that start reads the same source again. The resulting `MeasurementReading` distinguishes an observed duration, including a real zero, from declared unavailability and typed failure.

Ticks and elapsed time are separate. A `MeasurementTick` is one admitted reading on the caller's own origin, while elapsed time is the pure checked difference between the opening and closing ticks retained behind one start. A backwards reading refuses as regression rather than saturating to zero, and no public operation can close a start with another clock.

The source is a capture-free function pointer because clocks are declared in generated and hand-written test targets. That shape excludes captured closure state but does not make the caller function pure. An ordinary Rust unwind from a source read is caught and reported at the opening or closing boundary; aborts remain host process behavior. A fallible source may instead return `ClockReadRefusal` without unwinding.

This clock measures and nothing else. Each `TestPak` consumer decides what to do with its own work and retains the reading it earned; clock state never enters a semantic identity, selection, verdict, budget policy, or mutation control. ThreadPak chronology, HLC admission, scheduling, sleeping, and deadlines remain owned by the machine homes that give those operations meaning, so this harness boundary creates no universal clock and no dependency edge into S.
