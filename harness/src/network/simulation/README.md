# simulation

Deterministic message passing is a declared value.

This home owns topology, logical time, link disciplines, campaign selection, sends, deliveries, and the census that accounts for every send.
A selected schedule and an owner-built topology open one `SimNet`; `send` and `advance` are its only moves.

Fault precedence is structural.
An open partition takes a send before positional shaping, a drop takes it next, and delay plus duplication compose only for traffic that remains live.
Deliveries are ordered by due tick and then by the sequence in which they were scheduled.

The sim retains successful caller actions and delivery history for the sibling transcript owner.
That internal crossing projects already-informed values only; it creates no second simulation machine and exposes no public child path.
