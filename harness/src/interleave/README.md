# interleave

The schedule is an input.

Two parties that each behave alone can still break together, and the break lives in nothing but the order their steps merged in.
This home explores those orders: every way a set of declared command sequences can interleave, each merged history judged under one transition contract, with the same seeds, census honesty, and replay every other input here carries.
No thread runs and no clock ticks — a schedule is bytes, like any other input, which is what makes a concurrency bug a thing you can hold.

## The vocabulary

A **strand** is one named party: the commands it will issue, in its own program order.
The word is Cilk's, for a serial chain of steps, and it is chosen over "thread" precisely because no operating-system thread is involved — a strand is as at home in async code, in multi-node logic, or in two business workflows racing over one account.

A **strand set** is the parties together — at least two, uniquely named, no more than one choice byte can address.

An **interleaving** is one merge order, spelled as the canonical choice string: which strand stepped next, one ordinal per step.
Within it, every strand keeps its own program order; across strands, anything goes.
That is the whole space this home explores.

**Material** is an interleaving written in interpretation coordinates — the bytes a byte stream supplies.
At each step, the byte picks among the strands that still hold commands, so *every* byte string denotes exactly one lawful interleaving: interpretation is total, missing tail bytes read as the first live strand, and surplus bytes go unread.
Totality is the shrink story — a reducer can remove or zero any window of material and still hold a lawful schedule whose failure a fingerprint probe can judge.

## The road

`explored` walks the space one way or the other, and says which:

- **Exhaustive**, while the counted space fits the declared bound: every interleaving, enumerated in ascending position order, each driven through [`crate::properties::holds_over_history`].
  An all-pass here is a statement about the *whole* space.
- **Sampled**, beyond the bound: choice bytes are drawn through the one shared sequence driver ([`crate::generate::drive`]) under a seeded plan this home sizes exactly, so a sampled schedule carries a seed and a case like any generated input.
  An all-pass here is a statement about the sampled schedules and nothing more — the standing spells the difference, and no reader can mistake one for the other.

The reading carries the counted space, the mode with its generation census, how many interleavings were judged, and the standing.
A counterexample carries the site it was found at, the canonical interleaving, and the typed finding — `encoded` turns the interleaving back into material, `interpreted` realizes material into the merged history, and the two compose into replay with nothing hidden between them.

A directed check is the same two calls in the other direction: author the choice string yourself, encode it, realize it, judge it — the exact schedule that once failed, pinned as a regression.

`concluded` reads a whole exploration into one ordinary trial conclusion: a counterexample as the refusal its own finding states, an exhausted space as a pass over the whole space, a clean sample as a pass of the declared exploration exactly when its drive met its declared budget — and as a refusal where it stopped short, because an all-pass over fewer schedules than were declared is unexercised evidence.
The reading stays the owner of the replay; the conclusion is the verdict alone, and it rides the report vocabulary every fingerprint and rerun selection already reads.

## Faults compose

Adversity is per-party: inject a [`crate::fault`] schedule into one strand's commands *before* declaring the strand, and explore the injected commands like any others.
Each party's adversity stays that party's, and the exploration multiplies schedules by orders without either home learning the other's vocabulary.

## What it refuses

- A strand with no commands: a declared party that never acts is vacuous, and vacuity is refused where it is written.
- A set with fewer than two strands — an exploration needs something to reorder — or with a repeated name, or with more strands than a choice byte can address, or whose step total no address can hold.
- A bound with no interleaving seat or no sample seat.
- Encoding an interleaving foreign to the set: wrong step count, an ordinal no strand owns, a strand drawn past its length — each named at the step it broke.

## What this home will not tell you

A strand's command is one atomic step.
Instruction-level preemption and the memory model live below this floor: what happens *inside* a step is the subject's own, and a claim here says nothing about it.

Two interleavings that merge commuting commands are both counted and both driven.
No partial-order reduction is performed in this vocabulary — exhaustive-under-bound means exactly what it says, and an equivalence over schedules would be a claim needing its own evidence.

Sampling finds presence, never absence.
The standing after an all-pass sample is worded so that nothing downstream can read it as the exhausted space.
