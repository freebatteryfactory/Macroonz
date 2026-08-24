# concurrency

The exploration's identity, declared once.

An interleaving exploration is four declared facts — a population, a bound, a sample count, a seed — and a plumbing sentence repeating them at every call.
This home is the declaration grammar over that sentence: name each exploration, state its facts as a row, and the rendering writes one function per row that runs the exploration and reads it into a trial conclusion.

```rust,ignore
concurrency! {
    module = explorations,
    namespace = "app",
    transfer_never_overdraws {
        population = "transfer-orders",
        interleavings = 16,
        samples = 32,
        seed = 11,
    },
}
```

becomes one module — `explorations` — holding, per row, a generic function taking the strand set and the transition contract and handing back the exploration reading beside its concluded verdict, with every refusal traveling in one generated fault enum.

## What a row declares, and what it does not

A row declares the facts that make a finding replayable and nameable: which population the schedules are drawn under, the exhaustive ceiling and the sample count, and the seed.
Those are spellings a table should pin, because a seed that drifts is a counterexample nobody can find again.

The strand set and the contract stay call-side values, because they are the adopter's living types: a declaration that tried to spell them would be a second place their meaning could drift.
The generated function is generic over both, exactly as the exploration road is.

## What comes back

The pair, not a flattened verdict: the harness's `ExplorationReading` — the space, the mode, the census, the counterexample with its replay — beside the `TrialConclusion` the conclusion road read off it.
The verdict rides the ordinary report vocabulary, so a fingerprint, a rerun selection, and a reduction bind to it like to any other conclusion; the reading stays the owner of the evidence.
