# Network deliveries and command orders

This example declares network pressure and concurrency budgets through the root recipe entrance, then supplies its own account commands, transitions and independent no-overdraft rule.
It uses the harness without native tooling or a scheduler backend:

```sh
cargo run --example schedule_workflow --no-default-features --features harness --locked
```

Successful execution prints:

```text
network: quiet and four fault forms agree with exact deliveries
concurrency: withdraw-first fails and replays; guarded histories hold
evidence: exhaustive and sampled standings stay distinct; empty pressure and zero work refuse
```

The [declaration](types.rs) names three nodes, two directed links, an empty control schedule and a pressure schedule using drop, duplicate, delay and partition.
The [network caller](network.rs) selects each schedule, sends real simulated traffic and checks exact payloads, link addresses, send ordinals, logical times, duplicate markings and the complete send census.
The delayed deposit arrives twice at tick three, another deposit drops, and a withdrawal placed during the partition drops before a later withdrawal travels at the healing boundary.
A campaign containing only the empty control refuses to claim pressure, and an undeclared selection refuses.

The quiet deliveries become one command strand per link in the [concurrency caller](concurrency.rs).
Each strand preserves its own delivery order while the explorer considers the two cross-link orders.
The caller's vulnerable transition lets the balance go negative when withdrawal precedes deposit; the independent temporal demand catches that intermediate state even though the final balance returns to zero.
Encoding and interpreting the counterexample reconstructs the same commands and refusal without another search.
Replay checks the failure class and cause while each actual check keeps its own diagnostic location.

The caller separately supplies a guarded transition that ignores a withdrawal the balance cannot cover.
That amended behavior passes the saved order and both orders in the exhaustive space.
The same guarded contract also runs under a smaller exhaustive ceiling, producing repeatable sampled evidence with its narrower standing.
A declaration that admits zero interleavings returns the harness's typed bound refusal.

The [network declaration](../../macros/compiler/src/descriptor/network/README.md#authored-grammar), [concurrency declaration](../../macros/compiler/src/descriptor/concurrency/README.md#authored-grammar), [network owner](../../harness/src/network/README.md), [interleaving owner](../../harness/src/interleave/README.md) and [property owner](../../harness/src/properties/README.md) own their respective contracts.
The example supplies application meaning and expected results; Macroonz supplies the existing builders, simulator, schedule interpreter and temporal judgment.

Logical ticks are declared simulation time, and commands are atomic at the interleaving floor.
This run establishes neither real network behavior nor instruction-level preemption or memory-model behavior.
The pressure run checks delivery mechanics; the command-order exploration uses the quiet deliveries and does not claim to explore every combination of network faults and schedules.
Ignoring an uncovered withdrawal is this caller's explicit amendment, and the no-overdraft demand alone does not prove fairness, eventual withdrawal or every accounting rule.
