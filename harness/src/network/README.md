# network

The network home composes deterministic simulation with addressed transcript custody behind one public road.

The private [simulation](simulation/README.md) child owns topology, logical time, link faults, schedule and campaign selection, sends, deliveries, and census accounting.
The private [transcript](transcript/README.md) child owns source-specific packs, exact encoding and reading, simulation reproduction, replay exhaustion, and the same-address evidence join.
Their vocabulary is reexported only through `macroonz_harness::network`, so recursive ownership adds no new public path.

## Composition

A simulation retains its declared topology, selected schedule, successful caller actions, and delivery history.
The transcript writer projects those already-informed values to bytes and later reproduces them through the same public simulation operations.
No second simulator, fault roster, schedule semantics, or decoded-name mint exists in the transcript child.

Deliveries remain command-shaped values for the adopter to feed into the properties and interleave instruments.
Host-observed live traffic remains the adopter's input and may enter the generic runner recording road as an ordinary host observation.
Instruction-level preemption remains the separate target-qualified preemption floor.

## Boundary

This home owns no socket, port trait, task, operating-system interaction, wall clock, or product protocol.
A live adapter may mint transcript entries for what it observed, but only the transcript roads can admit a pack or mint reproduction and replay standing.
The experimenter's send receipt is not a claim about what a real subject can observe.
