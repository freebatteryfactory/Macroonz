# fault — typed adapters scheduled over command sequences

A fault is an adopter-owned typed adapter whose port operation returns the adopter's refusal or altered behavior. `TestPak` does not invent a universal port interface: it carries those typed adapter values through a named campaign, selects one schedule, and injects its values at declared positions in an ordinary command sequence.

The schedule is data, never ambient state. Positions are zero-based command coordinates; several faults may be stacked at one position, and their authored order is retained. Selection refuses a name the campaign did not declare, and injection refuses a scheduled position outside the supplied sequence rather than dropping it.

The adopter owns what an adapter does and what its refusal leaves true. A consumer observes those facts through the ordinary property and runner roads, so this home does not grow a second verdict, report, port trait, registry, global hook, or mutation orchestrator.

Out-of-memory is a seam fault by construction: an adopter can supply a bounded-capacity port adapter that refuses at its declared bound. No allocator hook is required in a safe-Rust harness because the product's own bounded port is the injection point.
