# generate

Where the inputs come from, and where a failure goes to get smaller.

A bug found with a random input is a story.
A bug found with a recorded seed, at a named case, shrunk to the smallest bytes that still break the same way, is a thing you can hold.
This home is the difference between the two.

## Two roads

**Generation** starts from a plan: a population, a generation profile, a seed or exact bytes, a case budget, a byte budget, a rejection allowance, and a size ramp.
The plan names a byte stream, and the stream is seekable — chunk N is derived from the plan's address and N alone, with no state carried between chunks.
Two runs of one plan draw identical bytes, and a longer run reproduces every case a shorter one produced, because growing a budget re-windows the same stream rather than renaming it.

One driver turns those bytes into command sequences.
It is the only loop of its kind here, so a lane that needs a structured input drives through it instead of growing a loop of its own.

**Reduction** starts from a failing input and a probe bound to the exact report that failed.
Semantic reducers you declared propose candidates first; the generic byte reducer follows, removing and zeroing windows at halving widths until a whole round admits nothing.
Every candidate, from either source, is admitted on one ground and one only: it still fails, under the same fingerprint.

## The census is the point

Six things can become of one request for a case, and all six have a seat.

| Disposition | What happened |
| --- | --- |
| generated | a case was produced and the precondition admitted it |
| bytes-insufficient | the source held less than the ramp asked for |
| precondition-rejected | a case was produced and the precondition turned it away |
| generator-refused | the decoder declined the bytes and produced nothing |
| generator-contract-violated | the decoder reported a command while consuming no bytes |
| generation-budget-exhausted | the byte budget was already spent when the case came up |

A rejection is counted, never skipped.
One that burned budget without being counted would shrink the denominator, and a reader of the census would see a smaller world than the drive actually walked.

The census and the halt answer different questions.
The census says what became of every case the drive reached; the halt says which bound or contract event ended it.
Neither is derivable from the other.

## What this home will not tell you

The generation axis is generation.
What an execution did with a case belongs to the runner, and what a check concluded belongs to the check — three owned axes, never one status blob.

A reduced input is the smallest one the reduction *reached*, under its budget and its reducer.
It is not the smallest input that reproduces the failure, and `ReductionHalt::BudgetExhausted` is what says so.

Nothing here reads a clock, an environment, or an operating-system random source.
Nothing here executes a subject, judges one, or knows what a command means.

## What stays yours

Four seams, all of them bare function pointers.
No trait to implement, no bound to satisfy, no captured state riding in.

| Seam | You supply |
| --- | --- |
| `CommandDecode` | a case's bytes, decoded into one command |
| `SequencePrecondition` | whether a decoded sequence belongs to your population |
| `SemanticReducerCall` | candidates strictly shorter than the input you were handed |
| `FingerprintProbe` | whether a candidate still fails, and under which fingerprint |

A command type that derives `Arbitrary` reaches the first seam through `decode_arbitrary` without writing one.
A population with no precondition drives under `admit_every_sequence` rather than under an absent one.

The strict descent a semantic reducer's candidates must show is checked before anything is probed.
That is what keeps the shared engine terminating no matter what a reducer knows about the bytes.
