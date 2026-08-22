# capsules — human-admitted replay custody

This home owns the exact entry a replay-bearing human admission stores: the proposal identity, the content-derived replay reference, and the run-bound capsule that reference names.

The storage implementation belongs to the caller. `ReplayDepotSink` receives the already-assembled entry and may refuse under its own durability ceiling; success returns a location bound to the same replay reference. The sink neither decides admission nor edits the capsule.

The proof-pressure engine owns the explicit human-admission operation. It first replaces the candidate row's origin with the admitted proposal/reference pair, then asks the caller's depot sink to store the exact entry, and only after both stand returns the admission receipt. Runtime reduction can produce a capsule but cannot reach this operation implicitly, so evidence never grows authored specification merely by existing.

An obligation-discharge admission creates no capsule entry. Its admitted row is the durable record, and the proof-pressure engine owns that separate human operation.
