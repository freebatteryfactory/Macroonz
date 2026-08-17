# descriptor — the typed vocabulary every producer writes into

A descriptor is one row of the harness's denominator: a typed statement of one
test — the claim it serves, the kind it belongs to, the subject it exercises,
the check that judges it, and where it came from. The runner enumerates
descriptor tables into trials; coverage is computed over the same tables; a
test that exists without a row is a value nobody can build.

testpak owns these types. Producers — the generation services, a hand, a
promotion from a fuzz find — emit data conforming to this vocabulary; no
producer's own types are imported, so no producer's shape can quietly become
the interface. Parsing what a producer emitted is itself a lane.

The fields of a row: the claim served (a typed identity); the kind, from the
sealed kind roster; the subject route — a typed selection of what is under
test, resolving to a runnable check; the check binding — which property suite
or oracle lane judges the subject; suite tags; the origin (hand-written,
generated, or promoted); and, for promoted rows, the reproduction seed — the
minimized input bytes that reproduce the find.

Two spellings read these tables and neither may drift from the other: the
runner enumerates them at run time, and the stamp expands them into named
test functions — which is also what gives every row an editor test lens, since
a named test function is what editors know how to run. The tables are the
single source of truth for both.
