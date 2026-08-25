# network

A topology you can read at a glance.

The harness's network sim is declared through values — nodes, links, schedules, campaigns — and the values are right, but a topology spelled as constructor calls reads like plumbing.
This home is the declaration grammar over those values: name the nodes, draw the links, state each schedule's discipline as fault phrases, and the rendering writes the builder module an author would have written by hand.

```text
network! {
    harness = renamed_macroonz::harness,
    module = net,
    namespace = "app",
    nodes = [client, server],
    link forward = client to server,
    link back = server to client,
    schedule quiet = [],
    schedule outage = [
        drop forward at 0,
        delay forward at 1 by 2,
        partition forward from 0 until 3,
    ],
}
```

The `harness` clause is the physical path this scope uses for the harness vocabulary.
A direct expansion compiles immediately, so an adopter states its Cargo alias or facade re-export here instead of the renderer guessing a package name.

becomes one module — `net` — holding `topology()`, one function per schedule, and one generated fault enum their refusals travel in.
Assembling schedules into a campaign stays the author's one line, because which schedules ride together is a run's decision, not a topology's.

## What is settled at the declaration

Everything the tokens can know refuses at its own token, before any code exists: a repeated node or link name, a link drawn to a node never declared, a phrase naming a link never drawn, a phrase this grammar cannot read.
What the harness's own guards refuse — an empty topology, an empty partition interval, a zero delay — still refuses there, at runtime, through the generated functions' honest results: this grammar compresses spelling, it does not stand in for the value roads' judgment.

## The fault phrases

One phrase per fault, in the sim's own vocabulary: `drop <link> at <n>`, `delay <link> at <n> by <n>`, `duplicate <link> at <n>`, and `partition <link> from <n> until <n>`.
Phrases on one link gather into that link's discipline in authored order, and an empty roster is the quiet control, exactly as the sim home states it.
