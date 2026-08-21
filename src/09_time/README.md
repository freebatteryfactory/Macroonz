# 09_time — the typed temporal algebra

Band 09. Imports bounds, identity, refusal, value, and the root calculus. Owns
T1–T4, the tick (the 2026-08-09 decision, quoted in the module docs), clock
observations, the deadline split, and HLC chronology.

## The deadline split

The retired fused monotonic-deadline was asked to be durable and monotonic at
once; no type can be both. Successors: `DeadlinePolicy` (durable commitment,
opaque over a private triple — a public enum would grant public construction;
`DurationBudget` is the paved road), `ConsumedBudgetEvidence` (persisted spend
at named durable points, never a raw instant — the durable coordinate rides a
typed reference because the coordinate value is the history home's, above this
band), and `LiveMonotonicDeadline` (per-clock-domain-life, structurally
`!Send`/`!Sync` via raw-pointer phantom, unserializable, dead with the clock
domain that produced it). The
narrowing law: remaining = policy − (consumed widened by ±u); a crash-restart
loop monotonically loses budget; adapters receive derived allowances and never
hold the policy. `DeadlineDimension` rides the bounds home's affine budget as
promised at band 05.

## HLC

`SourceHlc` ≠ `AcceptedHlc` ≠ `ChronologySummary` (an envelope, never an HLC
value — its extrema are independent fields so it cannot even look like one; no
morphism leads back). The summary merge is a real seam (`try_merge`),
commutative/associative/idempotent under its stated domain. The stateful
admission clock is a distinct owned object sharing no surface with the merge;
its name (`ChronologyAdmissionClock`) is AUTHORED: the role and its nine-item
roster are law here, the spelling is this home's. Logical counter u32: overflow
refuses, never wraps — the smallest width whose overflow can only mean broken
clock physics.

The two roles are made of the same payload and are no longer cross-constructible
from it. `HlcCoordinate` carries no role, `SourceHlc::observed` is the open end
because an observation genuinely arrives from anywhere, and `AcceptedHlc` has no
public mint at all — "yielded only by the admission clock" is the type's shape
now rather than a sentence a reader has to remember. The crossing between them is
declared as a contract (`ChronologyAdmission`): the observation is consumed,
exactly one admitted position comes out, the clock is mutated by the act, and no
road runs the other way. Its RULE — counter advancement, regression behavior,
excessive-future classification, the overflow refusal — is the clock's machinery
and is deliberately unwritten, because writing a body would mean choosing that
rule in the seat that is supposed to receive it.

## Presentation keys (doc law)

Within one local writer authority: HLC, then the authority order. Across
independent stores: HLC, then store identity, then that store's authority
order — presentation only, proving no shared commit order, atomicity, causal
dependency, or federation progress. HLC ranges are half-open. When admitted
chronology evidence is unavailable, the result says so; an observed wall time
is never silently promoted to HLC.

## Prohibited collapses

A foreign timestamp is not HLC; a foreign sequence is not an authority order;
HLC is never durable order, retry authority, or deadline authority; wall time
is never HLC; correlation and chronology are never causation.
