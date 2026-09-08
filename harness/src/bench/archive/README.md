# archive — complete historical benchmark reports

This home retains the complete account a benchmark report already holds.
It does not run a preflight, work callable, judge or clock, and loading cannot mint an executable binding or current qualification.

A report retains its table name and producer standing, every row in authored order, the complete row declaration, target, preflight report and all evidence for the stage reached.
A returned `BenchRunRefusal` is outside that report boundary because the execution owner publishes no partial report.

## Framing

The envelope begins with its thirty-two-byte address under `historical-benchmark-report/v1`.
Its body begins with big-endian `u32` format one, kind one and historical custody zero.
Names use the descriptor owner's namespace/stem framing.
Every framed byte member has a big-endian `u64` byte length.

The body retains the table name, framed descriptor provenance, then a big-endian `u64` reading count and that many readings.
Each reading retains a framed canonical benchmark-row preimage, framed target and toolchain text, a framed complete trial archive and one stage byte followed by that stage's evidence.
Row identity is derived from the existing declaration preimage; table identity remains separate.

Stage zero is preflight refused and carries no work.
Stages one, two and three are undistinguished control, refused primary work and qualified.
Each of these stages carries the measured curve, planted-worse curve and complete primary judgment.
Stage three additionally carries the secondary curve, secondary judgment, source-attribution byte, measurement count and ordered measurement readings.

A curve contains a `u64` point count followed by points.
Each point contains its `u64` input size, `u64` observation count and ordered name/`u64` count pairs.
A judgment retains measured conclusion, planted-worse conclusion and gap standing in that order.
Conclusion zero means satisfied, and one carries framed UTF-8 cause family and local text.
Gap zero means distinguished, and one carries the same cause fields.
Empty cause text remains lawful under the finding owner.

Secondary attribution is unspecified zero, synthetic one or caller-declared monotonic two.
Measurements reuse the [trial archive](../../report/archive/README.md#trial-envelope) grammar and preserve unavailable, observed zero and every failure.
Preflight attribution remains part of its independent nested trial record.

## Admission and ceiling

Writers admit total, field and population bounds before allocating canonical buffers.
Readers bound complete bytes, each field, row count, input axis, observations and secondary samples independently before walking or copying them.
The declaration owners admit axis uniqueness, budgets and formula presence.
Curves preserve the row's exact axis and one nonempty unique observation roster across all retained passes.
Secondary measurement population equals axis population times samples using checked arithmetic.

Rows are nonempty and unique by canonical identity, targets agree across the report and each preflight, and the retained preflight and work judgments agree with the stage.
These are historical consistency checks; they do not repeat the owner judge or authenticate a claimed execution.
Producer/schema labels and source attribution remain historical claims.
The exact original envelope is retained for caller-owned storage, and every nested trial keeps its own canonical bytes and authority ceiling.
