# Security

## Reporting

Report privately, through GitHub's private vulnerability reporting on this repository's Security tab.
If that road is closed to you, open an ordinary issue that says only that you have something to report and asks for a private channel — no detail, no reproduction.

You will get an acknowledgement.
There is no response-time promise, because a number nobody can hold to is worse than none.

## Supported versions

The most recent published release is supported.
A report names the exact version or revision it is against.

## In scope

| Road | What it looks like |
| --- | --- |
| Dependencies | The resolved graph differs from what the manifests and `deny.toml` declare |
| Ambient input | Expansion or judgment influenced by anything but declared input — network, filesystem, environment, clock, entropy |
| Exhaustion | A request or a descriptor an attacker controls that exhausts CPU, memory, stack, or I/O |
| Escaped output | A generated unit reaching a destination its plan did not name, or a member the closure never proved |
| False evidence | A harness verdict that misattributes, drops, or fabricates what it observed |

## Out of scope

- Anything that needs `unsafe`. The lint wall forbids it, so the report is about a different repository.
- A scanner identifier on its own. A report names the road: what the attacker controls, what they reach, what they get.
- The semantics, runtime, or deployment of a library that uses Macroonz. Those are reports to that library.

## What helps

The exact revision, toolchain, and command, and the smallest input that shows the behaviour.
