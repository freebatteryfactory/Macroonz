# capsules

A human admits a finding, and the run that produced it has to outlive the admission.

This home owns the entry that admission stores: the proposal it came from, the capsule that reproduces the run, and the content-derived reference tying the two together.
The reference is minted from the capsule's own bytes at a private seat, so nobody pairs a proposal with one capsule and a reference to another.

## Where the storage is

Not here.

`ReplayDepotSink` is the caller's.
It receives an entry that is already assembled and already immutable, and it may store the entry or refuse under its own durability ceiling; on success it returns a location carrying the same replay reference, so an admission can tell its own entry from a neighbor's.
The sink chooses nothing about what it is handed — not the proposal, not the identity, not the capsule bytes — and it never decides whether the admission happens.

## What does not write here

A run does not.
Reduction can produce a capsule, but the operation that stores one is explicit and human, and evidence never grows authored specification merely by existing.

An admission that discharges an obligation stores nothing here: its admitted row is the durable record, and rerunning it regenerates the behavioral evidence.
