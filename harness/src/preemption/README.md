# preemption

The preemption order is an input.

The [`interleave`](crate::interleave) home explores orders between whole commands; below that floor live the bugs inside a step — a load and a store that interleave across threads, an ordering the memory model permits and the author never imagined.
Exploring that space needs a scheduler that owns every context switch and a model of the memory orderings themselves.
That machine exists, is battle-hardened, and should not be rebuilt: this home wraps **loom**, pinned exact, as a declared backend — the same posture as wrapping the mutation console, at the API instead of a console grammar.

## The road

Write the model against loom's shadow types — `loom::sync`, `loom::thread` — and hand it to `explored` with declared bounds: how many preemptions per execution the search may spend, and how many branches one execution may take.
Loom then runs the model under its controlled scheduler, once per reachable interleaving under the bound, exhaustively.

Back comes a typed reading:

- **all interleavings held** — every execution under the declared bounds completed with every assertion standing;
- **the model broke** — some execution did not complete cleanly, with loom's own report carried as bounded foreign text.

A clean pass is a statement about the bounded space, exhaustively walked — stronger than any number of lucky runs on real threads, and deterministic under fixed bounds, so a wall that was green stays green.

`concluded` reads the verdict into one ordinary trial conclusion — a pass, or a refusal classed as the subject's own panic with loom's report riding as foreign text — so a preemption row concludes in the same report vocabulary as every other check.

## What composes inside a model

A model function is loom's world, and everything loom offers there composes with this road without a wrapper: the shadow synchronization types and channels, `loom::thread` whole — spawn, park, thread locals through loom's own `thread_local!` — and `loom::future::block_on`, so an async block is explored like any other operation (the workspace enables loom's `futures` face and the lane drives one).
Loom's exploration-guidance calls — `explore`, `stop_exploring`, `skip_branch` — are model-side statements too, and a model that makes them is narrowing its own search, declared in its own source.
`loom::sync::Notify` has no standard-library twin, so it rides no shadow row; a model may still use it directly.

A model runs at most `loom::MAX_THREADS` concurrent threads — five, in the pinned version — and that ceiling is loom's own.

## Every seat set, none left ambient

The builder's every seat is written explicitly by this road: the two the declared bounds own, and every other at the value a clean environment would give it — `max_permutations` none, `max_duration` none (a wall clock, which no verdict in this harness may read), no checkpoint file (the filesystem is not a declared port of this harness today), the pinned default checkpoint interval, `location` off (in loom's own words, very expensive — a declared opt-in seat is the road for it if evidence ever needs it), `log` off, and loom's own thread ceiling.
Seats forced rather than left unset, because an unset seat is loom's environment speaking: an ambient permutation ceiling or duration could end exploration before a single schedule ran and still return cleanly — a zero-execution run wearing the exhaustive claim.
A seat that graduates to meaning does so as a declared input on [`PreemptionBounds`], never as an ambient default.

## What this wrap will not claim

The verdict's cause vocabulary is the boundary's, not loom's: a broken model means *an execution did not complete cleanly* — an assertion failed, a deadlock was found, or the exploration overran a declared bound — and loom's report says which in its own words.
Nothing here parses that text back into types, because a wrap that guessed at a foreign report would be manufacturing evidence.

Loom reports no iteration count through its API, so none is claimed here.
The bounds are the declared statement of how far the search was allowed to go.

The catch at this boundary holds what the standard library can catch, and that ceiling is stated rather than papered: a model that panics while a loom thread or synchronization value is still live can panic loom again during unwind cleanup, and that second break escapes the typed reading and takes the process; the process panic hook also runs before the catch, so an expected finding is written to stderr on its way to becoming a value.
Neither effect mints false evidence — both are loud — and full containment would be a process-isolation ruling, not a wider catch.

Loom parses its `LOOM_*` environment variables inside its own constructor, before this road holds a builder to correct: a valid value there changes nothing — every seat is overwritten — but an unparseable spelling breaks the process ahead of this boundary's catch.
That break is loud rather than false; no verdict is minted from it.

The pin is exact — [`LOOM_PIN`](crate::preemption::LOOM_PIN) mirrors the manifest's `=`-requirement, and the lane holds the two spellings together — because a scheduler is a semantics, and a semantics that can drift under a caret is not a declared input.

## The one declaration an adopter writes

Production code cannot name loom's types directly without gating every import by hand.
The `macroonz-macros` crate's `shadow!` declaration absorbs that ceremony: one choice of names from a stated roster, in one module of the production crate, expands to both `cfg`-gated faces of each — and the crate is explorable under this home's road for the rest of its life.
The roster's shadow paths are witnessed by this home's own lane at the pinned version, under the ordinary wall.

## Where it sits beside its siblings

One home per floor: [`interleave`](crate::interleave) explores command orders with no threads at all, this home explores instruction-level preemption and the memory model with loom's shadow threads, and the [`network`](crate::network) home explores delivery orders between nodes.
The three compose by construction, because each one's schedule is a declared input to its own floor.
