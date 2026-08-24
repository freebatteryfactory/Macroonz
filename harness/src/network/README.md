# network

The network is an input.

A distributed bug is an ordering bug wearing a transport: a duplicate that double-applies, a drop that never retries, a partition that heals into two truths.
Chasing those against a real wire means chasing an adversary that never deals the same hand twice.
This home deals the same hand every time: a deterministic message-passing simulator, declared as a value, whose every delivery, drop, and delay follows from the declared topology, the declared discipline, and nothing else.

## The vocabulary

A **topology** is adopter-named nodes and the directed links between them.
A payload is whatever type the adopter carries — this home never learns what a message means, and it defines **no port trait**: the adopter's own port wraps the sim inside their adapter, exactly as the fault home's adversity stays the adopter's.

Time is **logical ticks**, owned by the sim.
A send is placed at the current tick; a delivery comes due a tick later, plus whatever the discipline adds.
No wall clock participates anywhere — the clock home stays the measurement boundary it always was.

**Discipline** is per-link adversity in the fault home's shape: named schedules gathered into a campaign, a quiet control beside the hostile ones, selected by name.
The faults are this home's own closed roster, applied by send ordinal on the link:

| Fault | What the link does |
| --- | --- |
| drop at a position | that send is lost |
| delay at a position | that send's delivery comes due later, by declared ticks |
| duplicate at a position | that send is delivered twice |
| partition over an interval | every send placed while the interval is open is lost |

Reordering is a delay that crosses: hold an earlier send longer than a later one and their deliveries change order.
That is how real networks reorder — latency variation, not a sorting demon — so no separate reorder fault exists to pretend otherwise.

## The road

Declare a topology, declare schedules, gather them into a campaign, select one by name, and open the sim over the pair.
Then drive it: `send` places a payload on a link and hands back a receipt naming the send's fate — scheduled, and for when, or dropped, and by what; `advance` moves the tick and hands back every delivery that came due, in deterministic order.
The census counts what became of every send, so a schedule that quietly dropped half the traffic cannot read as a calm run.

A **delivery is a command-shaped event**: link, ordinal, payload, sent and delivered ticks, and whether it is the original or a duplicate.
That shape is the keystone — deliveries feed a [`crate::properties`] transition contract directly, and per-link delivery sequences stand as [`crate::interleave`] strands, so delivery orders, party schedules, and link faults explore in one seeded, replayable space.

## Records and replays

A run worth keeping becomes a **transcript pack**: the provenance, the topology, and every delivery with its whole lineage — link, ordinal, payload bytes, sent and delivered ticks, original or duplicate — in one content-addressed envelope, the address derived over the whole body and written ahead of it, so a reader re-derives the claim before it believes a single row.

The same pack has two writers and one meaning:

- **`Simulated`** — the rows came out of a sim run, fully declared and re-derivable from the run's own inputs.
- **`RecordedLive`** — the rows were witnessed on a real network by an adopter's adapter, outside this crate.
  A transcript entry is openly mintable for exactly this reason: an adapter writes down what it observed, and what the pack can claim is bounded by its provenance, not by who spelled the rows.
  A claim graded over recorded-live material takes the honest ceiling posture — witnessed once, not re-derivable — and nothing may quietly promote it.

A pack opens as a **replay**: exactly the recorded deliveries at exactly their recorded ticks, no sends taken, no discipline consulted.
Live traffic becomes a deterministic regression input in one move, and the replayed deliveries are ordinary command-shaped events, judged on the same temporal road as everything else.

Live end-to-end runs themselves execute outside — a real client, a real server, a real wire — and their observations enter evidence through the runner's recording road, where a host cannot author evidence it never observed.
This home ships the pack writer and reader; it never owns a socket.

## What it refuses

- A topology with no node, no link, a repeated node or link, or a link naming a node never declared.
- A schedule with two disciplines on one link, or a partition interval that closes before it opens.
- A campaign with no schedule, a repeated name, or nothing but quiet controls — pressure declared and applied nowhere.
- Selecting a name the campaign never declared, opening a sim whose schedule disciplines a link outside its topology, and sending on an undeclared link.

## What this home will not tell you

The sim is a value, not a socket.
Nothing here binds a port, spawns a task, or touches an operating system; running real traffic is the adopter's act, outside, and its observations enter evidence through the runner's ordinary recording road.

The fate a receipt names is the experimenter's truth, not the subject's: a real sender never learns its packet died.
What the subject under test is allowed to see is the adopter's port's decision, and this home does not police it.

Delivery order among same-tick deliveries is the scheduling order, deterministically — not a claim that real networks order anything.
The discipline is where disorder is declared, and the quiet control is what it is measured against.
