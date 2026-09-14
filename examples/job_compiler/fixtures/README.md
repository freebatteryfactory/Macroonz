# Job compiler inputs

The lawful binary independently requires Draft plus Queue to produce Queued.
The wrong-state binary passes `7u8` where a Stage is required.
Its exact error code and coordinates are independently declared in [the caller checks](../checks.rs).
The manifest selects these files as binaries so ordinary workspace builds do not try to accept the deliberately refused input.
The coverage reader passes stdin bytes to the same Job decoder and reaches distinct accepted/refused functions.
The [Job coverage caller](../../job_coverage/README.md) instruments that binary and independently checks the literal input sequence and its source-attributed novelty.
