# Native process execution

This home executes explicitly declared commands with monotonic deadlines, bounded pipe capture and retained cleanup ownership.
Enable the root `native-tooling` feature and call `run` with an informed `ProcessRequest` and either an already opened stdin file or `None` for null input.
`ProcessTool` binds a reusable explicit tool configuration to individual argument lists through `invocation`; `ProcessRequest::informed` is the equivalent single-invocation entrance.
The existing compiler, coverage, mutation and publication owners interpret the observations; this home supplies no semantic verdict, tool discovery, command shell, runner or measurement clock.

## Declared inputs

The executable and working directory must be absolute.
Arguments retain their order and `std::process::Command` platform semantics; this home supplies no shell command-string grammar.
The environment is cleared before the caller's explicit entries are installed.
Environment keys are nonempty ASCII without equals or NUL, and duplicate keys refuse under the target's case rules.
The operating system may still supply its own process runtime facts; this is not an environment sandbox or a reproducible-build claim.

`ProcessLimits` admits a positive execution duration, a cleanup duration and separate stdout/stderr retained-byte bounds.
Zero output bounds retain no bytes and still detect an attempted write.
Zero cleanup duration submits termination and returns pending custody without polling for completion.
Memory, CPU-time, network-denial and group-escape controls currently return `Unsupported` before execution when requested.
The declared pipe bounds are not a process-memory quota.

## Execution and cleanup

Windows starts a suspended process in a job before allowing it to execute, preserving the hidden-console creation flag.
Linux and macOS create a process group before execution and retain the waitable leader until group termination has been submitted, so cleanup does not signal a recycled group identity.
Other targets return `Unavailable` without starting a process.

Stdout and stderr are read concurrently into separately bounded prefixes.
A reader stops at EOF, a read failure, or the first bytes beyond its bound; overflowing output closes that reader and triggers group termination.
Every capture preserves whether EOF, truncation or failure was observed, including when another stop condition takes precedence.

The execution deadline starts before allocation and spawn and uses the root's permitted native monotonic clock access.
After a leader exit, deadline, capture failure or output overflow, the supervisor submits termination to the controlled job or group, polls direct-child reaping and joins only readers that have finished.
The separate cleanup budget bounds polling and reader joining rather than silently turning unfinished cleanup into success.
`ProcessRun::Pending` retains those resources and exposes `finish` for a further explicit budget.
Dropping pending custody attempts termination and a nonblocking reap; it is not a successful cleanup observation and cannot guarantee that unfinished readers or processes have ended.

`Finished` means the termination request succeeded, the direct child was reaped and both pipe readers joined.
It does not certify that every descendant has exited: job termination is asynchronous, Unix descendants are not necessarily children this process can reap, and an adversarial Unix descendant can escape its group.
Operating-system spawn, termination and I/O calls and scheduler delays are not preemptible by this safe library, so duration bounds are enforcement deadlines rather than hard real-time guarantees.
No process identity, measurement, exit status or captured text becomes a semantic judgment in this home.

The [working law](../../AGENTS.md) owns native-effect permission and the [local wall](../../CONTRIBUTING.md#local-wall) owns qualification.
