# benches

Every target here is an ordinary caller of the public [`bench`](../src/bench/README.md) receiver, with no privileges of its own.

A target builds a table, hands in the host facts, takes back one immutable report, and may pass that report to a renderer.
The receiver decides whether the work qualified.
A renderer decides nothing.

`neutral_receiver.rs` is the specimen: hand-written, backed by no benchmark framework, and sharing one declaration with the behavior lane under `tests/` so the two cannot drift apart.
A framework here would be a second execution authority, and there is only one.
