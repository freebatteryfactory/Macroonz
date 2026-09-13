# Version one mechanical defaults

This version fixes the following resource ceilings for ordinary bounded runs and native tool invocations.
Changing a value requires a new version; explicit overrides use the existing limit constructors.
MiB means 1,048,576 bytes.

| Constructor | Effective configuration |
| --- | --- |
| `input_limits()` | Input envelope 2 MiB; payload 1 MiB. |
| `process_limits()` | Execution deadline 600 seconds; cleanup budget 5 seconds; retained stdout 16 MiB; retained stderr 4 MiB. |
| `retention_limits()` | The input limits above; each archive envelope 16 MiB and framed member 2 MiB; complete-run census 4,096 rows; storage batch 258 payload artifacts and 64 MiB aggregate bytes. |

These ceilings are independent, so reaching any one can refuse an operation even when the others have room.
The storage artifact limit includes input and run members plus any retained capsules; the archive census bound does not promise storage for a capsule on every row.
Process capture bounds do not bound child memory, and process deadlines retain the [process owner's enforcement and pending-cleanup limits](../../native_process/README.md#execution-and-cleanup).

## Compose and inspect

`run` takes the complete typed table view, selection, current decoder, payload and explicit invocation, and calls [the root workflow](../../workflow/README.md#execute) with `input_limits()`.
It returns the same `InputRun`, including the original input and complete report.
To override input ceilings, call `workflow::run` with an explicitly constructed `InputLimits`.

With `native-tooling`, `process_tool` admits the supplied executable, working directory, complete environment and resource controls using `process_limits()`.
It returns the existing `ProcessTool`, so inspection and argument binding use that owner's ordinary methods.
It neither probes the selected executable nor starts a process.
The limit constructor preserves the process owner's fallible admission surface.
For other deadlines or output ceilings, pass explicit `ProcessLimits` to `ProcessTool::informed`.

`retention_limits()` returns the existing `RetentionLimits` for `InputRun::retain`, `StoredRun::load` and the existing recovery operation.
Replace its independent fields for an explicit override.
No storage directory or batch name is chosen, opened or written by configuration construction.

The [configuration owner](../README.md) names the semantic and host facts that remain independently required.
